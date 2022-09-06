
extern crate proc_macro;

mod ast;
mod kw;

use syn::{ parse_macro_input, braced,
    Attribute, Error, Path, Field, Ident, Lit, LitStr, Result,
    Token, token
};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

use proc_macro2::{Group, Span};
use proc_macro::TokenStream;
use quote::{format_ident, quote};

use rtl_dsl::*;
use crate::ast::Body;

#[proc_macro]
pub fn rtl(body: TokenStream) -> TokenStream {
    let input = parse_macro_input!(body as Body).0;
    quote::quote!( #(#input)* ).into()
}


//#[cfg(test)]
//mod tests {
//    #[test]
//    fn it_works() {
//        let result = 2 + 2;
//        assert_eq!(result, 4);
//    }
//}
