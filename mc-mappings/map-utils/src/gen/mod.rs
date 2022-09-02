use std::{sync::{RwLock, Arc}, io::Write, collections::HashMap};

use rand::Rng;

use crate::{maps::{SigMappings, SigMod}, type_sig::{sanitize, parse_sig_f, parse_sig_m}};



pub fn generate_rs<W:Write>(yarn_maps : Arc<RwLock<SigMappings>>,tiny_maps : Arc<RwLock<SigMappings>>,modul:&SigMod,writer:&mut W,stk:&str) -> Result<(),()> {

    // let xyarn_maps = &Arc::clone(&yarn_maps).read().unwrap().sig_to_x;

    match modul {
        SigMod::Mod(x, data) => {
            writer.write_all(format!("pub mod {} {{\n",x).as_bytes()).unwrap();
            for d in data {
                let nstk = format!("{}/{}",stk,x);
                // println!("Generating {nstk}/{}",d);
                if generate_rs(Arc::clone(&yarn_maps),Arc::clone(&tiny_maps),d,writer,&nstk).is_err() {
                    // return Err(());
                    // break;
                    println!("fuck {:?}",d)
                }
            }
            writer.write_all(b"}\n").unwrap();
        },
        SigMod::class(c) => {
        
            if c.to.is_empty() {
                println!("{} empty",c.from);
                return Ok(());
            }
            let tinm = Arc::clone(&tiny_maps);
            let tinm = tinm.read().unwrap();
            let actual_class = tinm.sig_to_x.get(&c.from).ok_or(())?;
            println!("{:?}",actual_class);
            let class_name = sanitize(&c.to);
            writer.write_all(format!("pub struct r#{}<'a> {{\n",class_name).as_bytes()).unwrap();
            writer.write_all(b"pub inner : jni::object::JObject<'a>,\n").unwrap();
            // writer.write_all(b"}\n").unwrap();
            writer.write_all(format!("impl<'a> From<jni::object::JObject<'a>> for {}<'a> {{{}}}\n",class_name,stringify!{
                fn from(x:jni::object::JObject<'a>) -> Self {
                    Self {
                        inner: x,
                    }
                }
            }).as_bytes()).unwrap();
            writer.write_all(format!("impl<'a> {}<'a> {{\n",sanitize(&c.to)).as_bytes()).unwrap();
            writer.write_all(format!("const __map_sig : &str = \"{}\";\n",actual_class.get_from()).as_bytes()).unwrap();
            for f in &c.fields {
                let (fty,jty,jclass) = parse_sig_f(&f.type_,&Arc::clone(&yarn_maps).read().unwrap().sig_to_x);
                let fty = sanitize(&fty);
                let is_jobj_t = jty.contains("Object");
                if let Some(actual_field) = tinm.sig_to_x.get(&f.from) {
                    
                    let code = if is_jobj_t {
                        let non_static_code = format!("self.inner.get_field_object::<{typ}>(\"{field_name}\",\"{field_sig}\")",
                            typ = fty,
                            field_name = actual_field.get_from(),
                            field_sig = f.type_,
                        );
                        let static_code = format!("let class = env.find_class(Self::__map_sig);class.get_static_object_field::<{typ}>(\"{field_name}\",\"{field_sig}\")",
                            typ = fty,
                            field_name = actual_field.get_from(),
                            field_sig = f.type_,
                        );
                        format!(
                            "
pub fn get_{fn_name}(&self) -> Result<{typ},()> {non_static_code}
pub fn s_get_{fn_name}(env:&'a jni::env::Jenv<'a>) -> Result<{typ},()> {static_code}
                            ",
                            fn_name=sanitize(&f.to),
                            typ=fty,
                        )
                    } else {
                        let non_static_code = format!("self.inner.get_field_{jtype}(\"{field_name}\",\"{field_sig}\")",
                            field_name = actual_field.get_from(),
                            field_sig = f.type_,
                            jtype = jty.to_lowercase(),
                        );
                        let static_code = format!("let class = env.find_class(Self::__map_sig);class.get_static_{jtype}_field(\"{field_name}\",\"{field_sig}\")",
                            jtype = jty.to_lowercase(),
                            field_name = actual_field.get_from(),
                            field_sig = f.type_,
                        );
                        format!(
                            "
pub fn get_{fn_name}(&self) -> Result<{typ},()> {non_static_code}
pub fn s_get_{fn_name}(env:&'a jni::env::Jenv<'a>) -> Result<{typ},()> {static_code}
                            ",
                            fn_name=sanitize(&f.to),
                            typ=fty,
                        )
                    };

                    writer.write_all(code.as_bytes()).unwrap();

                }
                // writer.write_all(format!("/* {ty} = {sig} */\npub fn r#set_{}(&self,val : {ty}) -> () {{}} \n",sanitize(&f.to),ty=fty,sig=f.type_).as_bytes()).unwrap();
            }
            for m in &c.methods {
                let method_parse_sig = parse_sig_m(&m.type_,&Arc::clone(&yarn_maps).read().unwrap().sig_to_x);
                // println!("{:?} | {}",method_parse_sig,m.sig);
                let mut arg_s = Vec::<(String,String)>::new();
                for (i,str) in method_parse_sig.args.iter().enumerate() {
                    let name = sanitize(&m.args.get(i).unwrap_or(&format!("unknwnarg_{}",rand::thread_rng().gen::<u32>())));
                    arg_s.push((format!("r#a{name}"),format!("{}",sanitize(&str.0))));
                }
                let is_jobj_t = method_parse_sig.ret.1.contains("Object");

                if let Some(actual_method) = tinm.sig_to_x.get(&m.from) {
                    let code = if is_jobj_t {
                        let non_static_code = format!("let args = vec![{args}];self.inner.call_object_method::<{typ}>(\"{field_name}\",\"{field_sig}\")",
                            typ = method_parse_sig.ret.0,
                            field_name = actual_method.get_from(),
                            field_sig = m.type_,
                            args=arg_s.iter().map(|(a,b)| format!("{}.into()",a)).collect::<Vec<String>>().join(", "),
                        );
                        let static_code = format!("let args = vec![{args}];let class = env.find_class(Self::__map_sig);class.call_static_object_method::<{typ}>(\"{field_name}\",\"{field_sig}\")",
                            typ = method_parse_sig.ret.0,
                            field_name = actual_method.get_from(),
                            field_sig = m.type_,
                            args=arg_s.iter().map(|(a,b)| format!("{}.into()",a)).collect::<Vec<String>>().join(", "),
                        );
                        format!(
                            "
pub fn call_{fn_name}(&self,{args}) -> Result<{typ},()> {non_static_code}
pub fn s_call_{fn_name}(env:&'a jni::env::Jenv<'a>,{args}) -> Result<{typ},()> {static_code}
                            ",
                            fn_name=sanitize(&m.to),
                            typ=method_parse_sig.ret.0,
                            args=arg_s.iter().map(|(a,b)| format!("{} : {}",a,b)).collect::<Vec<String>>().join(", "),
                        )
                    } else {
                        let non_static_code = format!("let args=vec![{args}];self.inner.call_{jtype}_method(\"{field_name}\",\"{field_sig}\")",
                            field_name = actual_method.get_from(),
                            field_sig = m.type_,
                            jtype = method_parse_sig.ret.1.to_lowercase(),
                            args=arg_s.iter().map(|(a,b)| format!("{}.into()",a)).collect::<Vec<String>>().join(", "),

                        );
                        let static_code = format!("let args=vec![{args}];let class = env.find_class(Self::__map_sig);class.call_static_{jtype}_method(\"{field_name}\",\"{field_sig}\")",
                            jtype = method_parse_sig.ret.1.to_lowercase(),
                            field_name = actual_method.get_from(),
                            field_sig = m.type_,
                            args=arg_s.iter().map(|(a,b)| format!("{}.into()",a)).collect::<Vec<String>>().join(", "),
                        );
                        format!(
                            "
pub fn call_{fn_name}(&self,{args}) -> Result<{typ},()> {non_static_code}
pub fn s_call_{fn_name}(env:&'a jni::env::Jenv<'a>,{args}) -> Result<{typ},()> {static_code}
                            ",
                            fn_name=sanitize(&m.to),
                            typ=method_parse_sig.ret.0,
                            args=arg_s.iter().map(|(a,b)| format!("{} : {}",a,b)).collect::<Vec<String>>().join(", "),

                        )
                    };

                    writer.write_all(code.as_bytes()).unwrap();

                }


                // writer.write_all(format!("pub fn r#{}({}) -> {} {{ }}\n",sanitize(&m.to),arg_s,method_parse_sig.ret.0).as_bytes()).unwrap();
            }
            writer.write_all(b"}\n").unwrap();
            // return Err(());
        },
    }
    Ok(())
}