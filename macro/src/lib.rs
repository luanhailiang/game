extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(MyName)]
pub fn myname_derive(input: TokenStream) -> TokenStream {
    // 基于 input 构建 AST 语法树
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    // 构建特征实现代码
    let name = ast.ident;
    let gen = quote! {
        impl MyName for #name {
            fn name(&self)->String{
                String::from(stringify!(#name))
            }
        }
    };
    gen.into()
}
