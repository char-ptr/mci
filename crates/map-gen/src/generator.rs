use std::sync::Arc;

use codegen::Block;
use parking_lot::RwLock;
use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::format_ident;
use quote::quote;
use rand::Rng;

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
        cstruct.field("pub inner", "JObject<'a>");

        gen_on.new_impl(&clz.map_data.get_safe_name()).generic("'a").target_generic("'a").impl_trait("From<JObject<'a>>")
            .new_fn("from").arg("obj", "JObject<'a>").ret("Self").line("Self { inner: obj }");
        // gen_on.new_impl(&clz.map_data.get_safe_name()).generic("'a").target_generic("'a").impl_trait("Into<JObject<'a>>")
        //     .new_fn("into").ret("JObject<'a>").line("self.inner").arg_self();
        gen_on.new_impl(&clz.map_data.get_safe_name()).impl_trait("jni::object::JClassInstance").generic("'a").target_generic("'a")
            .new_fn("get_jobject").ret("JObject<'a>").arg_ref_self().line("self.inner.clone()");
        if let Some(ti_name) = self.Tiny.lookup.clone().read().get(&format!("{}_c",clz.map_data.from)) {
            gen_on.new_impl(&clz.map_data.get_safe_name()).generic("'a").target_generic("'a").associate_const("MAP_SIG", "&'static str", format!(r#""{}""#,ti_name.get_obfuscated()), "pub");
        } else {
            return ();
        }
        let cimpl = gen_on.new_impl(&clz.map_data.get_safe_name()).generic("'a").target_generic("'a");
        for fiel in &clz.fields { 
            self.generate_field(fiel, class, cimpl)
        }
        let mut thr_rand = rand::thread_rng();
        for meth in &clz.methods {
            
            
            // step 1: get return ready.
            let (pre_lookup_a,pre_lookup_ret) = meth.type_signature.clone().unwrap_method();
            let is_ret_clz = pre_lookup_ret.is_class();
            let ret_looked_up = if is_ret_clz {
                if let Some(lkup) = self.Yarn.lookup.get(&pre_lookup_ret.to_rust().to_uppercase()) {
                    pre_lookup_ret.to_rust_custom_life(&format!("crate::{}<'a>",lkup))
                } else {
                    pre_lookup_ret.to_rust_custom_life("JObject<'a>")
                }
            } else {
                pre_lookup_ret.to_rust_life()
            };
            if let Some(ti_name) = self.Tiny.lookup.clone().read().get(&format!("{}_m",meth.map_data.from)) {
                let ret = format!("Result<{},()>",ret_looked_up);

                let is_ret_clz = is_ret_clz || pre_lookup_ret.is_array();
                let primative = if is_ret_clz {"".to_string()} else {pre_lookup_ret.get_content().to_java_name()};
    

                let n_mname = if is_ret_clz {format!("call_object_method::<{ret_looked_up}>")} else {format!("call_{}_method",primative)};
                let s_mname = if is_ret_clz {format!("call_static_object_method::<{ret_looked_up}>")} else {format!("call_static_{}_method",primative)};
                let mid = thr_rand.gen::<u32>();
                let mut mcf = codegen::Function::new(&format!("m_{}_{}",&meth.map_data.get_safe_name(),&meth.map_data.from));
                mcf.ret(&ret).vis("pub").arg_ref_self();
                let mut mcfs = codegen::Function::new(&format!("ms_{}_{}",&meth.map_data.get_safe_name(),&meth.map_data.from));
                mcfs.ret(ret).vis("pub").arg("env", "&'a Jenv<'a>");

                let mut argument_jobjects = vec![];
                let mut argument_names = vec![];
                for (idx,marg) in meth.arguments.iter().enumerate() {
                    let mut a_name = marg.get_safe_name();
                    let a_sig = &pre_lookup_a[idx];

                    let is_a_clz = a_sig.is_class();

                    // now lookup the type.

                    let arg_looked_up = if is_a_clz {
                        format!("&'a {}",if let Some(lkup) = self.Yarn.lookup.get(&a_sig.to_rust_no_array().to_uppercase()) {
                            a_sig.to_rust_custom_life(&format!("crate::{}",lkup))
                        } else {
                            a_sig.to_rust_custom_life("JObject<'a>")
                        })
                    } else {
                        a_sig.to_rust_life()
                    };

                    if argument_names.contains(&a_name) {
                        a_name = format!("{}{}",a_name,thr_rand.gen::<u32>());
                    }

                    argument_names.push(a_name.clone());

                    let fa_name = format!("a_{a_name}");

                    let jobj_n = format!("JValue::from({})",if is_a_clz {
                        format!("{}.get_jobject()",fa_name)
                    } else {
                        fa_name.clone()
                    });
                    argument_jobjects.push(jobj_n);



                    mcf.arg(&fa_name,&arg_looked_up);
                    mcfs.arg(&fa_name,&arg_looked_up);
                }
                let code_args = argument_jobjects.join(",");
                let code_setup = format!(r#"let h_args = vec![{code_args}];"#);
                let args_post = format!(r#"("{}","{}",&h_args)"#,ti_name.get_obfuscated(),ti_name.get_signature());
                mcf.line(format!(r#"{code_setup}self.inner.{n_mname}{args_post}"#));
                mcfs.line(format!(r#"{code_setup}let class = env.find_class(Self::MAP_SIG)?;class.{s_mname}{args_post}"#));

                cimpl.push_fn(mcf);
                cimpl.push_fn(mcfs);
            }
        }
        // let sig = self.Tiny.lookup.clone().read().get(&format!("{}_c",clz.map_data.from)).unwrap().get_obfuscated();
        // println!("{}",sig);
    }
    fn generate_field(&self,fiel:&Field, class: &Arc<RwLock<Class>>, cimpl: &mut codegen::Impl) -> () {
        let clz = class.clone();
        let clz = clz.read();
        let pre_lookup = fiel.type_signature.clone().unwrap_field();
        let is_clz = pre_lookup.is_class();
        let looked_up = if is_clz {
            if let Some(lkup) = self.Yarn.lookup.get(&pre_lookup.to_rust_no_array().to_uppercase()) {
                pre_lookup.to_rust_custom(&format!("crate::{}",lkup))
            } else {
                pre_lookup.to_rust_custom("JObject<'a>")
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
                .line(format!(r#"let class = env.find_class(Self::MAP_SIG)?;class.{static_mname}("{}", "{}")"#,ti_name.get_obfuscated(),ti_name.get_signature()));
            cimpl.new_fn(&format!("r#{}",fiel.map_data.get_safe_name())).ret(&ret).arg_ref_self().vis("pub")
                .line(format!(r#"self.inner.{n_mname}("{}", "{}")"#,ti_name.get_obfuscated(),ti_name.get_signature()));

        }
        else {
            println!("No tiny lookup for {:?} in {}",fiel,clz.get_namespaced());
        }
    }

}

mod tests {
    use std::{path::PathBuf, io::{BufReader, Write}, fs::{File, self}, str::FromStr};
    use super::*;

    #[test]
    fn test_all() {
        let mut gen = Generator::new();
        // println!("@ => {}", std::env::current_dir().unwrap().display());
        gen.Yarn.run_directory(PathBuf::from("../../mappings/yarn-maps/mappings/net/minecraft/"), None).expect("bnruh");
        let mut tiny = BufReader::new(File::open("../../mappings/maps.tiny").expect("fw"));
        gen.Tiny.populate_from_reader(&mut tiny);

        // println!("yarn : {:#?}",gen.Yarn);

        // println!("{:?}",gen.Tiny);

        // let code = gen.Yarn.gen(&gen);
        let code = gen.generate();
        let f = fs::File::create("../mc-mappings/gen_src.rs");
        f.unwrap().write_all(code.to_string().as_bytes());

        println!("writing lookup table");
        let f = fs::File::create("./lookup.ron");
        f.unwrap().write_all(format!("{:#?}",gen.Yarn.lookup).as_bytes());

        // println!("{}",code);
    }
}