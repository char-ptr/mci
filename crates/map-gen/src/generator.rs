use std::sync::Arc;

use codegen::Block;
use parking_lot::RwLock;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use quote::ToTokens;
use rand::rngs::ThreadRng;
use rand::Rng;

use crate::tiny;
use crate::tiny::map::LookupType;
use crate::tiny::map::Tiny;
use crate::yarn::map::*;

#[derive(Debug)]
pub struct Generator {
    pub yarn: Yarn,
    pub tiny: Tiny,
}

impl Generator {
    pub fn new() -> Self {
        Self {
            yarn: Yarn::new(),
            tiny: Tiny::new(),
        }
    }
    pub fn generate(&self) -> String {
        let mut scp = codegen::Scope::new();
        for clmod in &self.yarn.modules {
            let cmod = scp
                .get_or_new_module(&clmod.name)
                .vis("pub")
                .import("jni::prelude", "*");
            for clzz_mod in &clmod.scope {
                self.generate_mod_or_class(cmod, clzz_mod);
            }
        }
        scp.to_string()
    }
    pub fn generate_mod_or_class(
        &self,
        up_mod: &mut codegen::Module,
        mod_or_class: &ModuleOrClass,
    ) -> () {
        match mod_or_class {
            ModuleOrClass::Module(module) => self.generate_module(up_mod, module),
            ModuleOrClass::Class(class) => self.generate_class(up_mod, class),
        }
    }
    pub fn generate_module(&self, gen_on: &mut codegen::Module, module: &Module) {
        let mut cmod = gen_on
            .new_module(&module.name)
            .vis("pub")
            .import("jni::prelude", "*");
        for clzz_mod in &module.scope {
            self.generate_mod_or_class(&mut cmod, clzz_mod);
        }
    }
    pub fn generate_class(&self, gen_on: &mut codegen::Module, class: &Class) -> () {
        let cstruct = gen_on
            .new_struct(&class.map_data.get_safe_name())
            .vis("pub");
        cstruct.generic("'a");
        cstruct.field("pub i", "JObject<'a>");

        gen_on
            .new_impl(&class.map_data.get_safe_name())
            .generic("'a")
            .target_generic("'a")
            .impl_trait("From<JObject<'a>>")
            .new_fn("from")
            .arg("obj", "JObject<'a>")
            .ret("Self")
            .line("Self { i: obj }");
        // gen_on.new_impl(&clz.map_data.get_safe_name()).generic("'a").target_generic("'a").impl_trait("Into<JObject<'a>>")
        //     .new_fn("into").ret("JObject<'a>").line("self.i").arg_self();
        gen_on
            .new_impl(&class.map_data.get_safe_name())
            .impl_trait("jni::object::JClassInstance")
            .generic("'a")
            .target_generic("'a")
            .new_fn("get_jobject")
            .ret("JObject<'a>")
            .arg_ref_self()
            .line("self.i.clone()");
        if let Some(ti_name) = self
            .tiny
            .lookup
            .clone()
            .read()
            .get(&format!("{}_c", class.map_data.from))
        {
            gen_on
                .new_impl(&class.map_data.get_safe_name())
                .generic("'a")
                .target_generic("'a")
                .associate_const(
                    "M_S",
                    "&'static str",
                    format!(r#""{}""#, ti_name.get_obfuscated()),
                    "pub",
                );
        } else {
            return ();
        }
        let cimpl = gen_on
            .new_impl(&class.map_data.get_safe_name())
            .generic("'a")
            .target_generic("'a");
        for fiel in &class.fields {
            self.generate_field(fiel, class, cimpl)
        }
        let mut thr_rand = rand::thread_rng();
        for meth in &class.methods {
            self.generate_method(meth, class, cimpl, &mut thr_rand)
        }
        // let sig = self.Tiny.lookup.clone().read().get(&format!("{}_c",clz.map_data.from)).unwrap().get_obfuscated();
        // println!("{}",sig);
    }
    //@todo refactor
    fn generate_method(
        &self,
        meth: &Method,
        class: &Arc<RwLock<Class>>,
        cimpl: &mut codegen::Impl,
        thr_rand: &mut ThreadRng,
    ) -> () {
        let clz = class.clone();
        let clz = clz.read();
        let (pre_lookup_a, pre_lookup_ret) = meth.type_signature.clone().unwrap_method();
        let is_ret_clz = pre_lookup_ret.is_class();
        let ret_looked_up = if is_ret_clz {
            if let Some(lkup) = self
                .yarn
                .lookup
                .get(&pre_lookup_ret.to_rust().to_uppercase())
            {
                pre_lookup_ret.to_rust_custom_life(&format!("crate::{}<'a>", lkup))
            } else {
                pre_lookup_ret.to_rust_custom_life("JObject<'a>")
            }
        } else {
            pre_lookup_ret.to_rust_life()
        };
        let temp_lt = LookupType::Method(tiny::map::Method {
            Obfuscated: meth.map_data.from.clone(),
            Signature: meth.type_signature.to_java(),
            Yarn: format!(
                "{}_{}",
                meth.map_data.get_safe_name(),
                meth.type_signature.to_javas()
            ),
            parent: meth.map_data.from.clone(),
        });
        let mut use_temp = false;
        if let Some(ti_name) = self
            .tiny
            .lookup
            .clone()
            .read()
            .get(&format!("{}_m", meth.map_data.from))
            .or_else(|| {
                println!("i got you = {temp_lt:?}");
                if !meth.map_data.from.is_empty() && meth.map_data.to.is_empty() {
                    use_temp = true;
                    Some(&temp_lt)
                } else {
                    None
                }
            })
        {
            let mut ret = format!("Result<{},()>", ret_looked_up);

            let is_ret_clz = is_ret_clz || pre_lookup_ret.is_array();
            let primative = if is_ret_clz {
                "".to_string()
            } else {
                pre_lookup_ret.get_content().to_java_name()
            };

            let mut n_mname = if is_ret_clz {
                format!("call_object_method::<{ret_looked_up}>")
            } else {
                format!("call_{}_method", primative)
            };
            let s_mname = if is_ret_clz {
                format!("call_static_object_method::<{ret_looked_up}>")
            } else {
                format!("call_static_{}_method", primative)
            };
            let nst = if use_temp {
                format!("m_{}", temp_lt.get_yarn())
            } else {
                format!(
                    "m_{}_{}",
                    &meth.map_data.get_safe_name(),
                    &meth.map_data.from
                )
            };
            let sst = if use_temp {
                format!("ms_{}", temp_lt.get_yarn())
            } else {
                format!(
                    "ms_{}_{}",
                    &meth.map_data.get_safe_name(),
                    &meth.map_data.from
                )
            };
            let is_constructor = meth.map_data.from.contains("<init>");
            let mut mcf = if is_constructor {
                let clzn = clz.map_data.get_safe_name();
                n_mname = format!("new_object::<{}>", &clzn);
                ret = format!("Result<{}<'a>,()>", clzn);
                let mut mcf = codegen::Function::new(&nst);
                mcf.ret(&ret).vis("pub").arg("e", "&'a Jenv<'a>");
                mcf
            } else {
                let mut mcf = codegen::Function::new(&nst);
                mcf.ret(&ret).vis("pub").arg_ref_self();
                mcf
            };
            let mut mcfs = codegen::Function::new(&sst);
            mcfs.ret(ret).vis("pub").arg("e", "&'a Jenv<'a>");

            let mut argument_jobjects = vec![];
            let mut argument_names = vec![];
            for (idx, marg) in meth.arguments.iter().enumerate() {
                let mut a_name = marg.get_safe_name();
                let a_sig = &pre_lookup_a[idx];

                let is_a_clz = a_sig.is_class();

                // now lookup the type.

                let arg_looked_up = if is_a_clz {
                    format!(
                        "&'a {}",
                        if let Some(lkup) = self
                            .yarn
                            .lookup
                            .get(&a_sig.to_rust_no_array().to_uppercase())
                        {
                            a_sig.to_rust_custom_life(&format!("crate::{}", lkup))
                        } else {
                            a_sig.to_rust_custom_life("JObject<'a>")
                        }
                    )
                } else {
                    a_sig.to_rust_life()
                };

                if argument_names.contains(&a_name) {
                    a_name = format!("{}{}", a_name, thr_rand.gen::<u32>());
                }

                argument_names.push(a_name.clone());

                let fa_name = format!("a_{a_name}");

                let jobj_n = format!(
                    "JValue::from({})",
                    if is_a_clz {
                        format!("{}.get_jobject()", fa_name)
                    } else {
                        fa_name.clone()
                    }
                );
                argument_jobjects.push(jobj_n);

                mcf.arg(&fa_name, &arg_looked_up);
                mcfs.arg(&fa_name, &arg_looked_up);
            }
            let code_args = argument_jobjects.join(",");
            let args = format!(r#"vec![{code_args}]"#);
            let args_post = format!(
                r#"("{}","{}",&{args})"#,
                ti_name.get_obfuscated(),
                ti_name.get_signature()
            );
            if is_constructor {
                mcf.line(format!(r#"e.find_class(Self::M_S)?.{n_mname}{args_post}"#));
            } else {
                mcf.line(format!(r#"self.i.{n_mname}{args_post}"#));
            }
            mcfs.line(format!(r#"e.find_class(Self::M_S)?.{s_mname}{args_post}"#));

            cimpl.push_fn(mcf);
            if !is_constructor {
                cimpl.push_fn(mcfs);
            }
        }
    }

    fn generate_field(&self, fiel: &Field, class: &Class, cimpl: &mut codegen::Impl) -> () {
        let clz = class.clone();
        let pre_lookup = fiel.type_signature.clone().unwrap_field();
        let is_clz = pre_lookup.is_class();
        let looked_up = if is_clz {
            if let Some(lkup) = self
                .yarn
                .lookup
                .get(&pre_lookup.to_rust_no_array().to_uppercase())
            {
                pre_lookup.to_rust_custom(&format!("crate::{}", lkup))
            } else {
                pre_lookup.to_rust_custom("JObject<'a>")
            }
        } else {
            pre_lookup.to_rust()
        };

        if let Some(ti_name) = self
            .tiny
            .lookup
            .read()
            .get(&format!("{}_f", fiel.map_data.from))
        {
            let ret = format!("Result<{},()>", looked_up);

            let sig_type = fiel.type_signature.to_java();
            let is_clz = is_clz || fiel.type_signature.clone().unwrap_field().is_array();
            let primative = if is_clz {
                "".to_string()
            } else {
                fiel.type_signature
                    .clone()
                    .unwrap_field()
                    .get_content()
                    .to_java_name()
            };

            let static_mname = if is_clz {
                format!("get_static_object_field::<{looked_up}>")
            } else {
                format!("get_static_{}_field", primative)
            };
            let n_mname = if is_clz {
                format!("get_field_object::<{looked_up}>")
            } else {
                format!("get_field_{}", primative)
            };

            cimpl
                .new_fn(&format!("s_{}", fiel.map_data.get_safe_name()))
                .ret(&ret)
                .arg("e", "&'a Jenv<'a>")
                .vis("pub")
                .line(format!(
                    r#"e.find_class(Self::M_S)?.{static_mname}("{}", "{}")"#,
                    ti_name.get_obfuscated(),
                    ti_name.get_signature()
                ));
            cimpl
                .new_fn(&format!("r#{}", fiel.map_data.get_safe_name()))
                .ret(&ret)
                .arg_ref_self()
                .vis("pub")
                .line(format!(
                    r#"self.i.{n_mname}("{}", "{}")"#,
                    ti_name.get_obfuscated(),
                    ti_name.get_signature()
                ));
        } else {
            // println!("No tiny lookup for {:?} in {}", fiel, clz.get_namespaced());
        }
    }
}

mod tests {
    use super::*;
    use std::{
        fs::{self, File},
        io::{BufReader, Write},
        path::PathBuf,
        str::FromStr,
    };

    #[test]
    fn test_all() {
        let mut gen = Generator::new();
        // println!("@ => {}", std::e::current_dir().unwrap().display());
        gen.yarn
            .run_directory(PathBuf::from(
                "../../mappings/yarn-maps/mappings/net/minecraft/",
            ))
            .expect("bnruh");
        let mut tiny = BufReader::new(File::open("../../mappings/maps.tiny").expect("fw"));
        gen.tiny.populate_from_reader(&mut tiny);

        // println!("yarn : {:#?}",gen.Yarn);

        // println!("{:?}",gen.Tiny);

        // let code = gen.Yarn.gen(&gen);
        let code = gen.generate();
        let f = fs::File::create("../mc-mappings/gen_src.rs");
        f.unwrap().write_all(code.to_string().as_bytes());

        println!("writing lookup table");
        let f = fs::File::create("./lookup.ron");
        f.unwrap()
            .write_all(format!("{:#?}", gen.yarn.lookup).as_bytes());

        // println!("{}",code);
    }
}
