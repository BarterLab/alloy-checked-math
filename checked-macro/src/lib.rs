use quote::ToTokens as _;
use syn::fold::Fold;

struct CheckedTransformer;

fn checked_bin_op(op: syn::BinOp) -> bool {
    use syn::BinOp::*;
    matches!(op, Add(_) | Sub(_) | Mul(_) | Div(_) | Rem(_))
}

fn checked_un_op(op: syn::UnOp) -> bool {
    use syn::UnOp::*;
    matches!(op, Neg(_))
}

impl Fold for CheckedTransformer {
    fn fold_expr(&mut self, e: syn::Expr) -> syn::Expr {
        match e {
            syn::Expr::Binary(mut binary) => {
                *binary.left = self.fold_expr(*binary.left);
                *binary.right = self.fold_expr(*binary.right);

                if !checked_bin_op(binary.op) {
                    return syn::Expr::Binary(binary);
                }

                *binary.left = { let left = *binary.left; syn::parse_quote! { Checked::Ok(#left) } };
                *binary.right = { let right = *binary.right; syn::parse_quote! { Checked::Ok(#right) } };
                return syn::parse_quote! { (#binary)? };
            },

            syn::Expr::Unary(mut unary) => {
                *unary.expr = self.fold_expr(*unary.expr);

                if !checked_un_op(unary.op) {
                    return syn::Expr::Unary(unary);
                }

                *unary.expr = { let expr = *unary.expr; syn::parse_quote! { Checked::Ok(#expr) } };
                return syn::parse_quote! { (#unary)? };
            },

            syn::Expr::Array(e) => syn::Expr::Array(self.fold_expr_array(e)),
            syn::Expr::Assign(e) => syn::Expr::Assign(self.fold_expr_assign(e)),
            syn::Expr::Async(e) => syn::Expr::Async(self.fold_expr_async(e)),
            syn::Expr::Await(e) => syn::Expr::Await(self.fold_expr_await(e)),
            syn::Expr::Block(e) => syn::Expr::Block(self.fold_expr_block(e)),
            syn::Expr::Break(e) => syn::Expr::Break(self.fold_expr_break(e)),
            syn::Expr::Call(e) => syn::Expr::Call(self.fold_expr_call(e)),
            syn::Expr::Cast(e) => syn::Expr::Cast(self.fold_expr_cast(e)),
            syn::Expr::Closure(e) => syn::Expr::Closure(self.fold_expr_closure(e)),
            syn::Expr::Const(e) => syn::Expr::Const(self.fold_expr_const(e)),
            syn::Expr::Continue(e) => syn::Expr::Continue(self.fold_expr_continue(e)),
            syn::Expr::Field(e) => syn::Expr::Field(self.fold_expr_field(e)),
            syn::Expr::ForLoop(e) => syn::Expr::ForLoop(self.fold_expr_for_loop(e)),
            syn::Expr::Group(e) => syn::Expr::Group(self.fold_expr_group(e)),
            syn::Expr::If(e) => syn::Expr::If(self.fold_expr_if(e)),
            syn::Expr::Index(e) => syn::Expr::Index(self.fold_expr_index(e)),
            syn::Expr::Infer(e) => syn::Expr::Infer(self.fold_expr_infer(e)),
            syn::Expr::Let(e) => syn::Expr::Let(self.fold_expr_let(e)),
            syn::Expr::Lit(e) => syn::Expr::Lit(self.fold_expr_lit(e)),
            syn::Expr::Loop(e) => syn::Expr::Loop(self.fold_expr_loop(e)),
            syn::Expr::Macro(e) => syn::Expr::Macro(self.fold_expr_macro(e)),
            syn::Expr::Match(e) => syn::Expr::Match(self.fold_expr_match(e)),
            syn::Expr::MethodCall(e) => syn::Expr::MethodCall(self.fold_expr_method_call(e)),
            syn::Expr::Paren(e) => syn::Expr::Paren(self.fold_expr_paren(e)),
            syn::Expr::Path(e) => syn::Expr::Path(self.fold_expr_path(e)),
            syn::Expr::Range(e) => syn::Expr::Range(self.fold_expr_range(e)),
            syn::Expr::RawAddr(e) => syn::Expr::RawAddr(self.fold_expr_raw_addr(e)),
            syn::Expr::Reference(e) => syn::Expr::Reference(self.fold_expr_reference(e)),
            syn::Expr::Repeat(e) => syn::Expr::Repeat(self.fold_expr_repeat(e)),
            syn::Expr::Return(e) => syn::Expr::Return(self.fold_expr_return(e)),
            syn::Expr::Struct(e) => syn::Expr::Struct(self.fold_expr_struct(e)),
            syn::Expr::Try(e) => syn::Expr::Try(self.fold_expr_try(e)),
            syn::Expr::TryBlock(e) => syn::Expr::TryBlock(self.fold_expr_try_block(e)),
            syn::Expr::Tuple(e) => syn::Expr::Tuple(self.fold_expr_tuple(e)),
            syn::Expr::Unsafe(e) => syn::Expr::Unsafe(self.fold_expr_unsafe(e)),
            syn::Expr::While(e) => syn::Expr::While(self.fold_expr_while(e)),
            syn::Expr::Yield(e) => syn::Expr::Yield(self.fold_expr_yield(e)),
            syn::Expr::Verbatim(e) => syn::Expr::Verbatim(e),
            e => unimplemented!("Expression type not implemented: {e:?}"),
        }
    }
}

#[proc_macro]
pub fn checked(source: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let expr = syn::parse_macro_input!(source as syn::Expr);
    let expr = CheckedTransformer.fold_expr(expr);
    return expr.to_token_stream().into();
}
