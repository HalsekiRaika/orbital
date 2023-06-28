use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemTrait, Ident};

#[proc_macro_attribute]
pub fn export_service(attrs: TokenStream, input: TokenStream) -> TokenStream {
    export(attrs, input)
}

fn export(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let ast_input = parse_macro_input!(input as ItemTrait);
    let base = &ast_input.ident;

    let t_ident = Ident::new(&format!("DependOn{}", base), base.span());
    let f_ident = Ident::new(&to_snake_case(base.to_string()), base.span());

    quote::quote! {
        #ast_input

        /// Auto-Generated by `orbital::export_service`
        pub trait #t_ident: 'static + Sync + Send {
            type #base: #base;
            fn #f_ident(&self) -> &Self::#base;
        }
    }.into()
}

fn to_snake_case(input: impl AsRef<str>) -> String {
    let input: &str = input.as_ref();
    input
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if c.is_uppercase() {
                let mut s = String::new();
                if i != 0 && !input.is_empty() && input.chars().next().unwrap().is_uppercase() {
                    s.push('_');
                }
                s.push_str(&c.to_lowercase().to_string());
                std::iter::once(s)
            } else {
                std::iter::once(c.to_string())
            }
        })
        .collect::<Vec<String>>()
        .join("")
}