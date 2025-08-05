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
    pub current_file: Option<std::path::PathBuf>,
    pub current_fn: Option<syn::Ident>,
    pub errors: Vec<Error>,
}

impl<'ast> Visit<'ast> for CheckedVisitor {
    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        self.current_fn = Some(i.sig.ident.clone());
        syn::visit::visit_block(self, &i.block);
        self.current_fn = None;
    }

    fn visit_impl_item_fn(&mut self, i: &'ast syn::ImplItemFn) {
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
            current_file: self.current_file.clone().unwrap(),
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
            current_file: self.current_file.clone().unwrap(),
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

pub fn assert_checked(root_path: &std::path::Path) {
    let mut visitor = CheckedVisitor { current_file: None, current_fn: None, errors: Vec::new() };

    glob(root_path.join("**/*.rs").to_str().unwrap())
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok)
        .for_each(|path| {
            let content = std::fs::read_to_string(&path).unwrap();
            let source = syn::parse_str(&content).unwrap();
            visitor.current_file = Some(path.clone());
            visitor.visit_file(&source);
        });

    if !visitor.errors.is_empty() {
        panic!("Unchecked arithmetic found:\n{}",
            visitor.errors.iter()
                .map(|e| {
                    let path = e.current_file.to_str().unwrap();
                    let fn_ident = e.current_fn.as_ref().map(|fn_name| fn_name.to_string()).unwrap_or("unknown".to_string());
                    let expr = pretty_expr(&e.unchecked_expr);

                    format!("- Path:       {path}\n  Function:   {fn_ident}\n  Expression: {expr}")
                })
                .collect::<Vec<_>>().join("\n")
        );
    }
}
