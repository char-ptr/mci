use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::format_ident;
use quote::quote;

use crate::yarn::map::{*};
use crate::tiny::map::Tiny;


#[derive(Debug)]
pub struct Generator<'a> {

    pub(crate) Yarn: Yarn<'a>,
    pub(crate) Tiny: Tiny,

}

impl<'a> Generator<'a> {
    pub fn new() -> Self {
        Self {
            Yarn: Yarn::new(),
            Tiny: Tiny::new(),
        }
    }
}


pub trait CodeGenerator {
    fn gen(&self, gen: &Generator) -> TokenStream;
}


impl<'a> CodeGenerator for Yarn<'a> {
    fn gen(&self,y:&Generator) -> TokenStream {
        let mut code = TokenStream::new();
        for module in &self.modules {
            code.extend((&module).clone().read().gen(y));
        }
        code
    }
}
impl CodeGenerator for ModuleOrClass {
    fn gen(&self,g:&Generator) -> TokenStream {
        match self {
            ModuleOrClass::Module(module) => module.clone().read().gen(g),
            ModuleOrClass::Class(class) => class.clone().read().gen(g),
        }
    }
}
impl CodeGenerator for Module {
    fn gen(&self, gen: &Generator) -> TokenStream {
        let name = format_ident!("{}",&self.name);
        println!("Generating module {:?}", name);
        let code = self.scope.iter().fold(TokenStream::new(), |mut j,m| {m.gen(gen).to_tokens(&mut j); j} );
        quote! {
            mod #name {
                #code
            }
        }
    }
} 
impl CodeGenerator for Class {
    fn gen(&self, gen: &Generator) -> TokenStream {
        let name = format_ident!("{}",&self.map_data.get_safe_name());
        let methods = self.methods.iter().fold(TokenStream::new(), |mut j,m| {m.gen(gen).to_tokens(&mut j); j} );
        let fields = self.fields.iter().fold(TokenStream::new(), |mut j,m| {m.gen(gen).to_tokens(&mut j); j} );
        let sig = gen.Tiny.lookup.clone().read().get(&format!("{}_c",self.map_data.from)).unwrap().get_obfuscated();
        println!("{}",sig);
        quote! {
            struct #name<'a> {
                inner : JObject<'a>
            }
            impl<'a> From<JObject<'a>> for #name<'a> {
                fn from(obj: JObject<'a>) -> Self {
                    Self { inner: obj }
                }
            }
            impl<'a> #name<'a> {
                const class_sig : &'static str = #sig;
                #fields
                #methods
            }
        }
    }
}
impl CodeGenerator for Method {
    fn gen(&self, gen: &Generator) -> TokenStream {
        let using = if (&self).map_data.to.is_empty() { &self.map_data.from } else { &self.map_data.to};
        let conv_to : String = using.chars().map(|c| if c.is_alphanumeric() {c} else {'_'}).collect();
        println!("[fn] {} -> {}",self.map_data.from, conv_to);
        let name = format_ident!("fn_{}",conv_to);
        let mut code = TokenStream::new();
        quote! {
            fn #name() -> JObject<'a> {
                #code
            }
        }
    }
}
impl CodeGenerator for Field {
    fn gen(&self, yarn: &Generator) -> TokenStream {
        let name = &self.map_data.to;
        let ident = format_ident!("m_{}",name);
        let static_ident = format_ident!("s_{}",name);
        let return_type = quote!{};
        quote! {
            pub fn #ident(&self) -> #return_type {

            }
            pub fn #static_ident() -> #return_type {

            }
        }
    }
}

mod tests {
    use std::{path::PathBuf, io::BufReader, fs::{File, self}, str::FromStr};
    use super::*;

    #[test]
    fn test_all() {
        let mut gen = Generator::new();

        gen.Yarn.run_directory(PathBuf::from("../mc-mappings/mappings/mappings/net/minecraft/advancement"), None).expect("bnruh");
        let mut tiny = BufReader::new(File::open("../mc-mappings/mappings/tiny/1.19.tiny").expect("fw"));
        gen.Tiny.populate_from_reader(&mut tiny);

        println!("yarn : {:#?}",gen.Yarn);

        // println!("{:?}",gen.Tiny);

        let code = gen.Yarn.gen(&gen);

        fs::write("../gen_src.rs", code.to_string()).expect("fw");

        println!("{}",code);
    }
}