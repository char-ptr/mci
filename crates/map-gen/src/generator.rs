use std::sync::Arc;

use codegen::Block;
use parking_lot::RwLock;
use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::format_ident;
use quote::quote;

use crate::yarn::map::{*};
use crate::tiny::map::Tiny;


#[derive(Debug)]
pub struct Generator {

    pub Yarn: Yarn,
    pub Tiny: Tiny,

}

impl Generator {
    pub fn new() -> Self {
        Self {
            Yarn: Yarn::new(),
            Tiny: Tiny::new(),
        }
    }
    pub fn generate(&self) -> String {
        let mut scp = codegen::Scope::new();
        for module in &self.Yarn.modules {
            let clmod = module.clone();
            let clmod = clmod.read();
            
            let cmod = scp.get_or_new_module(&clmod.name).vis("pub").import("jni::prelude", "*");
            for clzz_mod in &clmod.scope {
                self.generate_mod_or_class(cmod, clzz_mod);
            }
        };
        scp.to_string()
    }
    pub fn generate_mod_or_class(&self,up_mod : &mut codegen::Module, mod_or_class: &ModuleOrClass) -> () {
        match mod_or_class {
            ModuleOrClass::Module(module) => self.generate_module(up_mod,module),
            ModuleOrClass::Class(class) => self.generate_class(up_mod,class),
        }
    }
    pub fn generate_module(&self,gen_on : &mut codegen::Module, module: &Arc<RwLock<Module>>) -> () {
        let moder = module.clone();
        let moder = moder.read();
        let mut cmod = gen_on.new_module(&moder.name).vis("pub").import("jni::prelude", "*");
        for clzz_mod in &moder.scope {
            self.generate_mod_or_class(&mut cmod, clzz_mod);
        };
    }
    pub fn generate_class(&self,gen_on : &mut codegen::Module, class: &Arc<RwLock<Class>>) -> () {
        let clz = class.clone();
        let clz = clz.read();
        let cstruct = gen_on.new_struct(&clz.map_data.get_safe_name()).vis("pub");
        cstruct.generic("'a");
        cstruct.field("inner", "JObject<'a>");

        gen_on.new_impl(&clz.map_data.get_safe_name()).generic("'a").target_generic("'a").impl_trait("From<JObject<'a>>")
        .new_fn("from").arg("obj", "JObject<'a>").ret("Self").line("Self { inner: obj }");
        gen_on.new_impl(&clz.map_data.get_safe_name()).generic("'a").target_generic("'a").associate_const("MAP_SIG", "&'static str", format!(r#""{}""#,clz.map_data.from), "pub");
        let cimpl = gen_on.new_impl(&clz.map_data.get_safe_name()).generic("'a").target_generic("'a");
        for fiel in &clz.fields { 
            let pre_lookup = fiel.type_signature.clone().unwrap_field();
            let is_clz = pre_lookup.is_class();
            let looked_up = if is_clz {
                if let Some(lkup) = self.Yarn.lookup.get(&pre_lookup.to_rust().to_uppercase()) {
                    format!("crate::{}",lkup)
                } else {
                    // println!("lookup failure for {} in {:?}",pre_lookup.to_rust(),clz.get_namespaced());
                    "JObject<'a>".to_string()
                }
            } else {
                pre_lookup.to_rust()
            };

            if let Some(ti_name) = self.Tiny.lookup.read().get(&format!("{}_f",fiel.map_data.from)) {
                let ret = format!("Result<{},()>",looked_up);

                let sig_type = fiel.type_signature.to_java();
                let is_clz = is_clz || fiel.type_signature.clone().unwrap_field().is_array();
                let primative = if is_clz {"".to_string()} else {fiel.type_signature.clone().unwrap_field().get_content().to_java_name()};

                let static_mname = if is_clz {format!("get_static_object_field::<{looked_up}>")} else {format!("get_static_{}_field",primative)};
                let n_mname = if is_clz {format!("get_field_object::<{looked_up}>")} else {format!("get_field_{}",primative)};

                cimpl.new_fn(&format!("s_{}",fiel.map_data.get_safe_name())).ret(&ret).arg("env", "&'a Jenv<'a>").vis("pub")
                    .line(format!(r#"let class = env.find_class(Self::MAP_SIG)?;class.{static_mname}("{}", "{}")"#,ti_name.get_obfuscated(),sig_type));
                cimpl.new_fn(&format!("r#{}",fiel.map_data.get_safe_name())).ret(&ret).arg_ref_self().vis("pub")
                    .line(format!(r#"self.inner.{n_mname}("{}", "{}")"#,ti_name.get_obfuscated(),sig_type));

            }
            else {
                println!("No tiny lookup for {:?} in {}",fiel,clz.get_namespaced());
            }
            
        }
        // let sig = self.Tiny.lookup.clone().read().get(&format!("{}_c",clz.map_data.from)).unwrap().get_obfuscated();
        // println!("{}",sig);
    }
}

pub trait CodeGenerator {
    fn gen(&self, gen: &Generator) -> TokenStream;
}


impl CodeGenerator for Yarn {
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
        let return_type = quote!{()};
        quote! {
            pub fn #ident(&self) -> #return_type {

            }
            pub fn #static_ident() -> #return_type {

            }
        }
    }
}

mod tests {
    use std::{path::PathBuf, io::{BufReader, Write}, fs::{File, self}, str::FromStr};
    use super::*;

    #[test]
    fn test_all() {
        let mut gen = Generator::new();

        gen.Yarn.run_directory(PathBuf::from("../mc-mappings/mappings/mappings/net/minecraft/"), None).expect("bnruh");
        let mut tiny = BufReader::new(File::open("../mc-mappings/mappings/tiny/1.19.tiny").expect("fw"));
        gen.Tiny.populate_from_reader(&mut tiny);

        // println!("yarn : {:#?}",gen.Yarn);

        // println!("{:?}",gen.Tiny);

        // let code = gen.Yarn.gen(&gen);
        let code = gen.generate();
        let f = fs::File::create("../mc-mappings/src/gen_src.rs");
        f.unwrap().write_all(code.to_string().as_bytes());

        println!("writing lookup table");
        let f = fs::File::create("./lookup.ron");
        f.unwrap().write_all(format!("{:#?}",gen.Yarn.lookup).as_bytes());

        // println!("{}",code);
    }
}