mod parser;
extern crate proc_macro;


use proc_macro::TokenStream;
use quote::{format_ident, quote};

type TokenStream2 = quote::__private::TokenStream;

#[proc_macro]
pub fn ast_gen(_input: TokenStream) -> TokenStream {
    let item1 = ast_gen1();
    let item2 = ast_gen2();
    let ident = format_ident!("{}", "Asdf");
    quote! {
        enum #ident {
            #item1
            #item2
        }
    }
    .into()
}

mod parser_generation{
    use quote::{quote, format_ident};

    use crate::parser::parser_ast::{Rule, Alternation, Rhs};

    use super::parser::parser_ast::Specification;
    use super::TokenStream2;

    pub fn parser_gen(Specification(rules): Specification) -> TokenStream2 {
        quote!{
            use nom::IResult;
        }
    }

    fn rule(Rule{lhs, rhs: Rhs(alternations)}: Rule) -> TokenStream2 {
        let alts: Vec<TokenStream2> = alternations.into_iter().map(alternation).collect();
        let type_name = format_ident!("{}", lhs);
        let function_name = format_ident!("{}", lhs.to_lowercase());

        if alts.len() > 1 {
            quote!{
                use nom::branch::alt;
    
                fn #function_name (input: &str) -> IResult<&str, #type_name>{
                    alt((
                        #(#alts), *
                    ))(input)
                }
            }
        } else {
            let parsr = alts.get(0).unwrap();
            quote!{


                fn #function_name (input: &str) -> IResult<&str, #type_name>{
                    #parsr (input)
                }
            }
        }

        // todo!()
    }

    fn alternation(Alternation { concatenation, identifier }: Alternation) -> TokenStream2 {

        todo!()
    }

}

fn ast_gen1() -> TokenStream2 {
    let item1 = format_ident!("{}", "Asdf1");
    quote! {
        #item1(i32),
    }
}

fn ast_gen2() -> TokenStream2 {
    quote! {
        Asdf2(i32),
    }
    .into()
}

// #[proc_macro]
// pub fn ast_gen(input: TokenStream) -> TokenStream {
//     let gen = quote!{};

//     gen.into()
// }

// mod parser;
// mod builder;
