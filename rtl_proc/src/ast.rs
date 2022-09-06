
use syn::{ parse_macro_input, braced,
    Attribute, Error, Path, Field, Ident, Lit, LitStr, Result,
    Expr,
    Token, token, Type,
    PathArguments
};
use syn::parse::{Parse, ParseBuffer, ParseStream};
use syn::punctuated::Punctuated;
use proc_macro2::TokenStream;
use quote::{quote, format_ident, ToTokens};

use crate::kw;


pub struct Body(pub Vec<AstDefNode>);
impl Parse for Body {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut items = Vec::new();
        while !input.is_empty() {
            items.push(input.parse()?);
        }
        Ok(Self(items))
    }
}
impl ToTokens for Body {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for node in &self.0 {
            node.to_tokens(tokens);
        }
    }
}

pub enum AstDefNode {
    BundleDef(BundleDef),
    ModuleDef(ModuleDef),
}
impl Parse for AstDefNode {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::Bundle) {
            Ok(Self::BundleDef(input.parse()?))
        } 
        else if lookahead.peek(kw::Module) {
            Ok(Self::ModuleDef(input.parse()?))
        }
        else {
            Err(lookahead.error())
        }
    }
}
impl ToTokens for AstDefNode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::BundleDef(x) => x.to_tokens(tokens),
            Self::ModuleDef(x) => x.to_tokens(tokens),
        }
    }
}

pub enum PortDef {
    Input(InputPortDef),
    Output(OutputPortDef),
    InOut(InOutPortDef),
}
impl Parse for PortDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::input) {
            Ok(Self::Input(input.parse()?))
        } 
        else if lookahead.peek(kw::output) {
            Ok(Self::Output(input.parse()?))
        } 
        else if lookahead.peek(kw::io) {
            Ok(Self::InOut(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

pub fn parse_port_type(ty: &Type) {
    if let Type::Path(tp) = ty {
        let ident = tp.path.segments[0].ident.to_string();
        let a = &tp.path.segments[0].arguments;
        if let PathArguments::AngleBracketed(args) = a {

        }
 
        println!("{}\n", ident);
    }
}

pub struct InputPortDef {
    kw: Ident,
    name: Ident,
    colon: token::Colon,
    ty: Type,
}
impl Parse for InputPortDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let kw = input.parse()?;
        let name = input.parse()?;
        let colon = input.parse()?;
        let ty = input.parse()?;
        parse_port_type(&ty);
        Ok(Self { kw, name, colon, ty })
    }
}
pub struct OutputPortDef {
    kw: Ident,
    name: Ident,
    colon: token::Colon,
    ty: Type,
}
impl Parse for OutputPortDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let kw = input.parse()?;
        let name = input.parse()?;
        let colon = input.parse()?;
        let ty = input.parse()?;
        parse_port_type(&ty);
        Ok(Self { kw, name, colon, ty })
    }
}
pub struct InOutPortDef {
    kw: Ident,
    name: Ident,
    colon: token::Colon,
    ty: Type,
}
impl Parse for InOutPortDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let kw = input.parse()?;
        let name = input.parse()?;
        let colon = input.parse()?;
        let ty = input.parse()?;
        parse_port_type(&ty);
        Ok(Self { kw, name, colon, ty })
    }
}


pub struct BundleDef {
    kw: Ident,
    name: Ident,
    brace: token::Brace,
    fields: Vec<PortDef>,
    semi: Token![;],
}
impl Parse for BundleDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let kw = input.parse()?;
        let name = input.parse()?;
        let brace = braced!(content in input);
        let fields = {
            Punctuated::<PortDef, token::Semi>::parse_terminated_with(
                &content, |inner| { Ok(inner.parse()?) },
            )?.into_iter().collect()
        };
        let semi = input.parse()?;
        Ok( Self { kw, name, brace, fields, semi } )
    }
}
impl ToTokens for BundleDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let BundleDef { kw, name, brace, fields, semi } = self;

        let mut field_stream = TokenStream::new();
        for portdef in fields {
            match portdef {
                PortDef::Input(x) => {
                    let InputPortDef { kw, name, colon, ty } = x;
                    field_stream.extend(quote! {
                        pub #name: Input<#ty>,
                    })
                },
                PortDef::Output(x) => {
                    let OutputPortDef { kw, name, colon, ty } = x;
                    field_stream.extend(quote! {
                        pub #name: Output<#ty>,
                    })
                },
                PortDef::InOut(x) => {
                    let InOutPortDef { kw, name, colon, ty } = x;
                    field_stream.extend(quote! {
                        pub #name: InOut<#ty>,
                    })

                },
            }
        }
        //let bundle_name = format_ident!("__bundle_{}", name);
        tokens.extend(quote! {
            impl rtl_dsl::DslBundleDef for #name {}
            pub struct #name {
                #field_stream
            }
        });
    }
}

pub struct CombExpr {
    dst: Ident,
    expr: Expr,
}
impl Parse for CombExpr {
    fn parse(input: ParseStream) -> Result<Self> {
        let dst: Ident = input.parse()?;
        let _: Token![:] = input.parse()?;
        let _: Token![=] = input.parse()?;
        let expr: Expr = input.parse()?;
        Ok( Self { dst, expr })
    }
}


pub struct ModuleDef {
    kw: Ident,
    name: Ident,
    brace: token::Brace,
    ports: Vec<PortDef>,
    combs: Vec<CombExpr>,
    semi: Token![;],
}
impl Parse for ModuleDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut ports: Vec<PortDef> = Vec::new();
        let mut combs: Vec<CombExpr> = Vec::new();
        let body;

        let kw = input.parse()?;
        let name = input.parse()?;
        let brace = braced!(body in input);

        // Parse the set of port definitions for this module
        let lookahead = body.lookahead1();
        if lookahead.peek(kw::ports) {
            let ports_body;
            let _: Ident = body.parse()?;
            let brace = braced!(ports_body in body);
            ports.extend(
                Punctuated::<PortDef, token::Semi>::parse_terminated_with(
                    &ports_body, |inner| { Ok(inner.parse()?) },
                )?.into_iter()
            );
            let _: Token![;] = body.parse()?;
        } else {
            return Err(lookahead.error());
        }

        let lookahead = body.lookahead1();
        if lookahead.peek(kw::comb) {
            let combs_body;
            let _: Ident = body.parse()?;
            let brace = braced!(combs_body in body);
            combs.extend(
                Punctuated::<CombExpr, token::Semi>::parse_terminated_with(
                    &combs_body, |inner| { Ok(inner.parse()?) },
                )?.into_iter()
            );
            let _: Token![;] = body.parse()?;

        } else {
            return Err(lookahead.error());
        }

        let semi: Token![;] = input.parse()?;
        Ok( Self { kw, name, brace, ports, combs, semi } )
    }
}
impl ToTokens for ModuleDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ModuleDef { kw, name, brace, ports, combs, semi } = self;

        let mut field_stream = TokenStream::new();
        for port in ports {
            match port {
                PortDef::Input(x) => {
                    let InputPortDef { kw, name, colon, ty } = x;
                    field_stream.extend(quote! {
                        pub #name: Input<#ty>,
                    })
                },
                PortDef::Output(x) => {
                    let OutputPortDef { kw, name, colon, ty } = x;
                    field_stream.extend(quote! {
                        pub #name: Output<#ty>,
                    })
                },
                PortDef::InOut(x) => {
                    let InOutPortDef { kw, name, colon, ty } = x;
                    field_stream.extend(quote! {
                        pub #name: Output<#ty>,
                    })
                },
            }
        }

        //let mut comb_stream = TokenStream::new();

        tokens.extend(quote! {
            impl rtl_dsl::DslModuleDef for #name {}
        });
        for comb in combs {
            let CombExpr { dst, expr } = comb;
            let s = format!("{}", dst);
            tokens.extend(quote! {
                #[doc = #s]
            });
        }
        tokens.extend(quote! {
            pub struct #name {
                #field_stream
            }
        });
    }
}



