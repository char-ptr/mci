use std::sync::Arc;

use parking_lot::RwLock;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote, format_ident};

use super::map::{Yarn, Module, ModuleOrClass, Class, Method, Field};

pub trait YarnGen {
    fn gen(&self, yarn: &Yarn) -> TokenStream;
}


impl<'a> Yarn<'a> {
    pub fn generate_code(&self) -> String {
        let mut code = String::new();
        for module in &self.modules {
            code.push_str(&module.clone().read().gen(&self).to_string());
        }
        code
    }
}
impl YarnGen for  ModuleOrClass {
    fn gen(&self,y:&Yarn) -> TokenStream {
        match self {
            ModuleOrClass::Module(module) => module.clone().read().gen(y),
            ModuleOrClass::Class(class) => class.clone().read().gen(y),
        }
    }
}
impl YarnGen for Module {
    fn gen(&self, yarn: &Yarn) -> TokenStream {
        let name = format_ident!("{}",&self.name);
        println!("Generating module {:?}", name);
        let code = self.scope.iter().fold(TokenStream::new(), |mut j,m| {m.gen(yarn).to_tokens(&mut j); j} );
        quote! {
            mod #name {
                #code
            }
        }
    }
} 
impl YarnGen for Class {
    fn gen(&self, yarn: &Yarn) -> TokenStream {
        let name = format_ident!("{}",&self.map_data.get_safe_name());
        let methods = self.methods.iter().fold(TokenStream::new(), |mut j,m| {m.gen(yarn).to_tokens(&mut j); j} );
        let fields = self.fields.iter().fold(TokenStream::new(), |mut j,m| {m.gen(yarn).to_tokens(&mut j); j} );

        quote! {
            struct #name {
            }
            impl #name {
                #fields
                #methods
            }
        }
    }
}
impl YarnGen for Method {
    fn gen(&self, yarn: &Yarn) -> TokenStream {
        let name = &self.map_data.to;
        let mut code = TokenStream::new();
        quote! {
            fn #name() {
                #code
            }
        }
    }
}
impl YarnGen for Field {
    fn gen(&self, yarn: &Yarn) -> TokenStream {
        let name = &self.map_data.to;
        let mut code = TokenStream::new();
        quote! {
            #name: #code
        }
    }
}

mod tests {
    use std::path::PathBuf;

    use super::Yarn;
    #[test]
    fn test_all() {
        let mut yarn_instance = Yarn::new();

        println!("env = {}",std::env::current_dir().unwrap().to_str().unwrap());

        yarn_instance.run_directory(PathBuf::from("../mc-mappings/mappings/mappings/net/minecraft/advancement"), None);

        let code = yarn_instance.generate_code();

        println!("{}",code);
    }
}