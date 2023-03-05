use proc_macro::TokenStream;
use syn::{parse::Parse, Token};

struct JClasser {
    rust: syn::Type,
    java: syn::Ident
}
impl Parse for JClasser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let rs = input.parse::<syn::Type>()?;
        input.parse::<Token![,]>()?;
        let java = input.parse::<syn::Ident>()?;
        Ok(JClasser {
            rust: rs,
            java
        })
    }
}
#[proc_macro]
pub fn generate_jclass_impls(input: TokenStream) -> TokenStream {
    let JClasser { rust, java } = syn::parse_macro_input!(input as JClasser);

    let setf = quote::format_ident!("set_static_{}_field", java);
    let getf = quote::format_ident!("get_static_{}_field", java);
    let callm = quote::format_ident!("call_static_{}_method", java);

    let mut tokens = quote::quote! {
        impl<'a> crate::class::JClassExt<#rust> for crate::class::JClass<'a> {
            fn set_static_field(cls:&crate::class::JClass, name: &str, sig: &str, new_value: #rust) -> Result<(), ()> {
                cls.#setf(name, sig, new_value)
            }
            fn get_static_field(cls:&crate::class::JClass, name: &str, sig: &str) -> Result<#rust, ()> {
                cls.#getf(name, sig)
            }
            fn call_static_method(cls:&crate::class::JClass, name: &str, sig: &str, args:&Vec<crate::jvalue::JValue>) -> Result<#rust, ()> {
                cls.#callm(name, sig, args)
            }
        }
    };
    tokens.into()
}