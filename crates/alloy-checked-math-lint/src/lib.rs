use std::vec;

use glob::glob;
use syn::visit::Visit;

fn is_checked_binary_op(op: syn::BinOp) -> bool {
    matches!(op,
        | syn::BinOp::Add(_) | syn::BinOp::AddAssign(_)
        | syn::BinOp::Sub(_) | syn::BinOp::SubAssign(_)
        | syn::BinOp::Mul(_) | syn::BinOp::MulAssign(_)
        | syn::BinOp::Div(_) | syn::BinOp::DivAssign(_)
        | syn::BinOp::Rem(_) | syn::BinOp::RemAssign(_)
    )
}

fn is_checked_unary_op(op: syn::UnOp) -> bool {
    matches!(op,
        | syn::UnOp::Neg(_)
    )
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

impl CheckedVisitor {
    fn push_error(&mut self, unchecked_expr: syn::Expr) {
        self.errors.push(Error {
            current_file: self.current_file.clone(),
            current_fn: self.current_fn.clone(),
            unchecked_expr,
        });
    }
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
        if is_checked_binary_op(node.op) {
            return self.push_error(syn::Expr::Binary(node.clone()));
        }

        self.visit_expr(&node.left);
        self.visit_expr(&node.right);
    }

    fn visit_expr_unary(&mut self, node: &'ast syn::ExprUnary) {
        if is_checked_unary_op(node.op) {
            return self.push_error(syn::Expr::Unary(node.clone()));
        }

        self.visit_expr(&node.expr);
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
    let mut root_path = root_path.as_ref();

    assert!(root_path.exists(), "Root path does not exist");

    let files = if root_path.is_file() {
        let files = vec![root_path.to_path_buf()];
        root_path = root_path.parent().unwrap();
        files
    } else {
        glob(root_path.join("**/*.rs").to_str().unwrap())
            .expect("Failed to read glob pattern")
            .filter_map(Result::ok)
            .collect()
    };

    let mut errors = Vec::new();

    for path in files {
        print!("Checking {} ... ", path.strip_prefix(root_path).unwrap().display());

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
            println!("  - path: {}", error.current_file.strip_prefix(root_path).unwrap().display());
            println!("    function: {}", error.current_fn.as_ref().map(|fn_name| fn_name.to_string()).unwrap_or("unknown".to_string()));
            println!("    expression: {}", pretty_expr(&error.unchecked_expr));
            println!("");
        }
    }

    assert!(errors.is_empty(), "Unchecked arithmetic expressions found in the codebase.");
}

#[macro_export]
macro_rules! assert_checked_subtree {
    () => {
        {
            let mut get_root_cargo_toml_command = std::process::Command::new("cargo");
            get_root_cargo_toml_command.arg("locate-project").args(["--message-format", "plain"]).arg("--workspace");
            let root_cargo_toml = get_root_cargo_toml_command.output().expect("Failed to execute command");
            let root_cargo_toml = std::path::PathBuf::from(String::from_utf8(root_cargo_toml.stdout).unwrap().trim_end());
            let workspace_root = root_cargo_toml.parent().unwrap();

            let current_mod_relative_path = std::path::PathBuf::from(file!());
            let current_mod_root = workspace_root.join(current_mod_relative_path);
            let current_mod_root = current_mod_root.parent().unwrap();

            alloy_checked_math::assert_checked(&current_mod_root);
        }
    };
}

#[macro_export]
macro_rules! assert_checked_mod {
    () => {
        {
            let mut get_root_cargo_toml_command = std::process::Command::new("cargo");
            get_root_cargo_toml_command.arg("locate-project").args(["--message-format", "plain"]).arg("--workspace");
            let root_cargo_toml = get_root_cargo_toml_command.output().expect("Failed to execute command");
            let root_cargo_toml = std::path::PathBuf::from(String::from_utf8(root_cargo_toml.stdout).unwrap().trim_end());
            let workspace_root = root_cargo_toml.parent().unwrap();

            let current_mod_relative_path = std::path::PathBuf::from(file!());
            let current_mod_root = workspace_root.join(current_mod_relative_path);

            alloy_checked_math::assert_checked(&current_mod_root);
        }
    };
}
