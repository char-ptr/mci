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
#[proc_macro]
pub fn generate_joaf_impls(input: TokenStream) -> TokenStream {
    let JClasser { rust, java } = syn::parse_macro_input!(input as JClasser);

    let setf = quote::format_ident!("set_field_{}", java);
    let getf = quote::format_ident!("get_field_{}", java);
    let setfs = quote::format_ident!("set_static_{}_field", java);
    let getfs = quote::format_ident!("get_static_{}_field", java);

    let mut tokens = quote::quote! {
        impl<'a> AbstractJField<'a,#rust> {
            pub const fn new(parent : &'a JObject<'a>,name : String,sig : String) -> Self {
                Self {
                    parent,
                    name,
                    sig,
                    return_type : PhantomData,
                }
            }
            pub fn get(&self) -> Result<#rust,()> {
                self.parent.#getf(&self.name,&self.sig)
            }
            pub fn set(&self,new_value:#rust) -> Result<(),()> {
                self.parent.#setf(&self.name,&self.sig,new_value)
            }
        }
        impl<'a > AbstractStaticJField<#rust> {
            pub const fn new(class_sig : String,name : String,sig : String) -> Self {
                Self {
                    class_sig,
                    name,
                    sig,
                    return_type : PhantomData,
                }
            }
            pub fn get(&self,env:&'a Jenv) -> Result<#rust,()> {
                env.find_class(&self.class_sig)?.#getfs(&self.name,&self.sig)
            }
            pub fn set(&self,env:&'a Jenv,new_value:#rust) -> Result<(),()> {
                env.find_class(&self.class_sig)?.#setfs(&self.name,&self.sig,new_value)
            }
        }
    };
    tokens.into()
}
#[proc_macro]
pub fn generate_joam_impls(input: TokenStream) -> TokenStream {
    let JClasser { rust, java } = syn::parse_macro_input!(input as JClasser);
    let callm = quote::format_ident!("call_{}_method", java);
    let callms = quote::format_ident!("call_static_{}_method", java);

    let mut tokens = quote::quote! {
        impl<'a> AbstractJMethod<'a,#rust> {
            pub const fn new(parent : &'a JObject<'a>,name : String,sig : String, args: Vec<JValue<'a>>) -> Self {
                Self {
                    parent,
                    name,
                    sig,
                    args,
                    return_type : PhantomData,
                }
            }
            pub fn call(&self) -> Result<#rust,()> {
                self.parent.#callm(&self.name,&self.sig,&self.args)
            }
        }
        impl<'a> AbstractStaticJMethod<'a,#rust> {
            pub const fn new(class_sig : String,name : String,sig : String, args: Vec<JValue<'a>>) -> Self {
                Self {
                    class_sig,
                    name,
                    sig,
                    args,
                    return_type : PhantomData,
                }
            }
            pub fn call(&self,env: &'a Jenv) -> Result<#rust,()> {
                env.find_class(&self.class_sig)?.#callms(&self.name,&self.sig,&self.args)
            }
        }
    };
    tokens.into()
}
#[proc_macro]
pub fn generate_jobject_impls(input: TokenStream) -> TokenStream {
    let JClasser { rust, java } = syn::parse_macro_input!(input as JClasser);

    (quote::quote! {
        jni_proc::generate_joaf_impls!(#rust, #java);
        jni_proc::generate_joam_impls!(#rust, #java);
    }).into()
}