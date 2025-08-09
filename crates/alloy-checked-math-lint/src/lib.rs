use glob::glob;
use syn::visit::Visit;

pub fn checked_bin_op(op: syn::BinOp) -> bool {
    use syn::BinOp::*;
    matches!(op, Add(_) | Sub(_) | Mul(_) | Div(_) | Rem(_))
}

pub fn checked_un_op(op: syn::UnOp) -> bool {
    use syn::UnOp::*;
    matches!(op, Neg(_))
}

struct Error {
    pub current_file: std::path::PathBuf,
    pub current_fn: Option<syn::Ident>,
    pub unchecked_expr: syn::Expr,
}

struct CheckedVisitor {
    pub current_file: std::path::PathBuf,
    pub current_fn: Option<syn::Ident>,
    pub errors: Vec<Error>,
}

fn has_checked_fn_attr(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| false
        || attr.path().is_ident("checked_fn")
        || attr.path().is_ident("unchecked_fn")
    )
}

impl<'ast> Visit<'ast> for CheckedVisitor {
    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        if has_checked_fn_attr(&i.attrs) {
            return;
        }

        self.current_fn = Some(i.sig.ident.clone());
        syn::visit::visit_block(self, &i.block);
        self.current_fn = None;
    }

    fn visit_impl_item_fn(&mut self, i: &'ast syn::ImplItemFn) {
        if has_checked_fn_attr(&i.attrs) {
            return;
        }

        self.current_fn = Some(i.sig.ident.clone());
        syn::visit::visit_block(self, &i.block);
        self.current_fn = None;
    }

    fn visit_expr_binary(&mut self, node: &'ast syn::ExprBinary) {
        if !checked_bin_op(node.op) {
            self.visit_expr(&node.left);
            self.visit_expr(&node.right);
            return;
        }

        self.errors.push(Error {
            current_file: self.current_file.clone(),
            current_fn: self.current_fn.clone(),
            unchecked_expr: syn::Expr::Binary(node.clone()),
        });
    }

    fn visit_expr_unary(&mut self, node: &'ast syn::ExprUnary) {
        if !checked_un_op(node.op) {
            self.visit_expr(&node.expr);
            return;
        }

        self.errors.push(Error {
            current_file: self.current_file.clone(),
            current_fn: self.current_fn.clone(),
            unchecked_expr: syn::Expr::Unary(node.clone()),
        });
    }
}

fn pretty_expr(expr: &syn::Expr) -> String {
    let file = syn::File {
        shebang: None,
        attrs: vec![],
        items: vec![syn::parse_quote! {
            fn __checked_expr() {#expr}
        }],
    };

    let unparsed = prettyplease::unparse(&file);

    return unparsed
        .trim()
        .strip_prefix("fn __checked_expr() {").unwrap()
        .strip_suffix("}").unwrap()
        .trim()
        .to_string();
}

pub fn assert_checked<P: AsRef<std::path::Path>>(root_path: P) {
    let files = glob(root_path.as_ref().join("**/*.rs").to_str().unwrap())
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok);

    let mut errors = Vec::new();

    for path in files {
        print!("Checking {} ... ", path.strip_prefix(root_path.as_ref()).unwrap().display());

        let mut visitor = CheckedVisitor { current_file: path.clone(), current_fn: None, errors: Vec::new() };
        let content = std::fs::read_to_string(&path).unwrap();
        let source = syn::parse_str(&content).unwrap();

        visitor.visit_file(&source);

        if visitor.errors.is_empty() {
            println!("done");
        } else {
            println!("found {} unchecked arithmetic expressions", visitor.errors.len());
        }

        errors.extend(visitor.errors);
    }

    if errors.is_empty() {
        println!("No unchecked arithmetic expressions found in the codebase.");
    } else {
        println!("Found total {} unchecked arithmetic expressions", errors.len());
        println!("");
        for error in &errors {
            println!("  - path: {}", error.current_file.strip_prefix(root_path.as_ref()).unwrap().display());
            println!("    function: {}", error.current_fn.as_ref().map(|fn_name| fn_name.to_string()).unwrap_or("unknown".to_string()));
            println!("    expression: {}", pretty_expr(&error.unchecked_expr));
            println!("");
        }
    }

    assert!(errors.is_empty(), "Unchecked arithmetic expressions found in the codebase.");
}
