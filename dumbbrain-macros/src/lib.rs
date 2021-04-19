use inflections::Inflect;
use proc_macro::TokenStream;
use quote::format_ident;
use quote::quote;
use quote::ToTokens;
use syn::braced;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse_macro_input;
use syn::punctuated::Punctuated;
use syn::token::Brace;
use syn::Expr;
use syn::Fields;
use syn::Ident;
use syn::ItemEnum;
use syn::LitChar;
use syn::Token;
use syn::Type;

struct TInput {
    components: Punctuated<Expr, Token![,]>,
}

impl Parse for TInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            components: Punctuated::parse_separated_nonempty(input)?,
        })
    }
}

#[proc_macro]
pub fn t(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as TInput);
    let components = input.components;

    (quote! {Test::new(#components)}).into()
}

struct TestStructComponent {
    name: Ident,
    _colon_tok: Token![:],
    ty: Type,
}

impl Parse for TestStructComponent {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let _colon_tok = input.parse()?;
        let ty = input.parse()?;

        Ok(Self {
            name,
            _colon_tok,
            ty,
        })
    }
}

impl ToTokens for TestStructComponent {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let ty = &self.ty;
        tokens.extend(quote! { #name: #ty });
    }
}

struct Test {
    _brace_tok: Brace,
    contents: Punctuated<Expr, Token![,]>,
}

impl Parse for Test {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let contents;
        let _brace_tok = braced!(contents in input);
        let contents = contents.parse_terminated(Expr::parse)?;

        Ok(Test {
            _brace_tok,
            contents,
        })
    }
}

impl ToTokens for Test {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let contents = self.contents.iter().collect::<Vec<_>>();

        tokens.extend(quote! {Test::new(#(#contents),*)});
    }
}

struct TestStructInput {
    _struct_kw: Token![struct],
    _brace_tok: Brace,
    struct_components: Punctuated<TestStructComponent, Token![,]>,
    _brace_tok2: Brace,
    tests: Punctuated<Test, Token![,]>,
}

impl Parse for TestStructInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let struct_contents;
        let tests;

        let _struct_kw = input.parse()?;
        let _brace_tok = braced!(struct_contents in input);
        let struct_components = struct_contents.parse_terminated(TestStructComponent::parse)?;
        let _brace_tok2 = braced!(tests in input);
        let tests = tests.parse_terminated(Test::parse)?;

        Ok(TestStructInput {
            _struct_kw,
            _brace_tok,
            struct_components,
            _brace_tok2,
            tests,
        })
    }
}

#[proc_macro]
pub fn test_struct(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as TestStructInput);

    let test_struct_components = input.struct_components.into_iter().collect::<Vec<_>>();

    let test_struct = quote! {
        struct Test {
            #(#test_struct_components),*
        }
    };

    let components = test_struct_components
        .iter()
        .map(|c| &c.name)
        .collect::<Vec<_>>();

    let test_impl = quote! {
        impl Test {
            fn new(#(#test_struct_components),*) -> Self {
                Self {
                    #(#components),*
                }
            }
        }
    };

    let tests = input.tests.into_iter().collect::<Vec<_>>();

    let tests = quote! {
        let tests: &[Test] = &[
            #(#tests),*
        ];
    };

    let output = quote! {
        #test_struct
        #test_impl
        #tests
    };

    output.into()
}

struct SimpleCharsInput {
    matches: Vec<LitChar>,
    kinds: Vec<syn::Path>,
}

impl Parse for SimpleCharsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut matches = vec![];
        let mut kinds = vec![];

        while let Ok(mat) = input.parse::<LitChar>() {
            input.parse::<Token![=>]>()?;
            let kind = input.parse::<syn::Path>()?;
            input.parse::<Token![,]>()?;

            matches.push(mat);
            kinds.push(kind);
        }

        Ok(Self { matches, kinds })
    }
}

#[proc_macro]
pub fn lexer_simple_chars(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as SimpleCharsInput);

    let mut match_arms = vec![];
    for (mat, kind) in input.matches.iter().zip(input.kinds.iter()) {
        let match_arm = quote! {
            #mat => {
                tok = Token {
                    kind: #kind,
                    literal: #kind.to_string(),
                };
            }
        };
        match_arms.push(match_arm);
    }

    let output = quote! {
        match self.ch {
            #(#match_arms)*
            _ => {}
        }
    };
    output.into()
}

#[proc_macro_derive(IsAs)]
pub fn derive_is_as(input: TokenStream) -> TokenStream {
    let e = parse_macro_input!(input as ItemEnum);
    let e_name = &e.ident;
    let e_vis = &e.vis;

    let mut defs = vec![];
    for item in &e.variants {
        if item.fields.len() != 1 {
            continue;
        }

        let fields = if let Fields::Unnamed(f) = &item.fields {
            f
        } else {
            continue;
        };

        let item_name = &item.ident;
        let field_type = &fields.unnamed[0].ty;

        let is_ = format_ident!("is_{}", item_name.to_string().to_snake_case());
        let as_ = format_ident!("as_{}", item_name.to_string().to_snake_case());
        let try_into_ = format_ident!("try_into_{}", item_name.to_string().to_snake_case());

        let err_msg = format!(
            "enum {} did not match the expected variant {}::{}",
            e_name, e_name, item_name
        );

        defs.push(quote! {
            impl #e_name {
                #e_vis fn #is_(&self) -> bool {
                    match self {
                        Self::#item_name(_) => true,
                        _ => false,
                    }
                }

                #e_vis fn #as_(&self) -> Option<&#field_type> {
                    match self {
                        Self::#item_name(value) => Some(value),
                        _ => None,
                    }
                }

                #e_vis fn #try_into_(self) -> Result<#field_type, &'static str> {
                    match self {
                        Self::#item_name(value) => Ok(value),
                        _ => Err(#err_msg),
                    }
                }
            }
        });
    }

    let output = quote! {
        #(#defs)*
    };
    output.into()
}
