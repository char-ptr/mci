use std::{sync::{RwLock, Arc}, io::Write, collections::HashMap};

use rand::Rng;

use crate::{maps::{SigMappings, SigMod}, type_sig::{sanitize, parse_sig_f, parse_sig_m}};



pub fn generate_rs<W:Write>(yarn_maps : Arc<RwLock<SigMappings>>,modul:&SigMod,writer:&mut W,stk:&str) -> Result<(),()> {

    // let xyarn_maps = &Arc::clone(&yarn_maps).read().unwrap().sig_to_x;

    match modul {
        SigMod::Mod(x, data) => {
            writer.write_all(format!("pub mod {} {{\n",x).as_bytes()).unwrap();
            for d in data {
                let nstk = format!("{}/{}",stk,x);
                // println!("Generating {nstk}/{}",d);
                if generate_rs(Arc::clone(&yarn_maps),d,writer,&nstk).is_err() {
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
            let class_name = sanitize(&c.to);
            writer.write_all(format!("pub struct r#{}<'a> {{\n",class_name).as_bytes()).unwrap();
            writer.write_all(b"pub __map_internal : Option<jni::object::JObject<'a>>,\n").unwrap();
            writer.write_all(b"pub __map_internal_e : &'a jni::env::Jenv<'a>\n}\n").unwrap();
            // writer.write_all(b"}\n").unwrap();
            writer.write_all(format!("impl From<jni::object::JObject<'_>> for {}<'_> {{{}}}\n",class_name,stringify!{
                fn from(x:jni::object::JObject) -> Self {
                    Self {
                        __map_internal: Some(x),
                        __map_internal_e: x.env 
                    }
                }
            }).as_bytes()).unwrap();
            writer.write_all(format!("impl {}<'_> {{\n",sanitize(&c.to)).as_bytes()).unwrap();
            writer.write_all(format!("const __map_sig : &str = \"{}\";\n",c.from).as_bytes()).unwrap();
            for f in &c.fields {
                let (fty,jty,jclass) = parse_sig_f(&f.type_,&Arc::clone(&yarn_maps).read().unwrap().sig_to_x);
                let fty = sanitize(&fty);
                let is_jobj_t = jty.contains("Object");
                let get_code = format!("if let Some(jobj) = self.__map_internal {{ {} }} else {{{}}}",if is_jobj_t {
                    format!("jobj.get_field_object::<{}>(\"{}\",\"{}\")",fty,f.from,f.type_)
                } else {
                    format!("jobj.get_field_{}(\"{}\",\"{}\")",jty.to_lowercase(),f.from,f.type_)
                },if is_jobj_t {
                    format!("let class = self.__map_internal_e.find_class(Self::__map_sig)?; class.get_static_object_field::<{}>(\"{}\",\"{}\")",
                        fty,f.from,f.type_
                    )
                } else {
                    format!("let class = self.__map_internal_e.find_class(Self::__map_sig)?; class.get_static_{}_field(\"{}\",\"{}\")",jty.to_lowercase(),
                    f.from,f.type_)
                });

                writer.write_all(format!("/* {ty} = {sig} */\npub fn r#get_{}(&self) -> Result<{ty},()> {{\n{fnb}}} \n",
                sanitize(&f.to),ty=fty,sig=f.type_,fnb=get_code).as_bytes()).unwrap();
                // writer.write_all(format!("/* {ty} = {sig} */\npub fn r#set_{}(&self,val : {ty}) -> () {{}} \n",sanitize(&f.to),ty=fty,sig=f.type_).as_bytes()).unwrap();
            }
            for m in &c.methods {
                let method_parse_sig = parse_sig_m(&m.type_,&Arc::clone(&yarn_maps).read().unwrap().sig_to_x);
                // println!("{:?} | {}",method_parse_sig,m.sig);
                let mut arg_s = String::new();
                for (i,str) in method_parse_sig.args.iter().enumerate() {
                    let name = sanitize(&m.args.get(i).unwrap_or(&format!("unknwnarg_{}",rand::thread_rng().gen::<u32>())));
                    arg_s.push_str(&format!("r#{} : {},",name,sanitize(&str.0)));
                }
                // writer.write_all(format!("pub fn r#{}({}) -> {} {{ }}\n",sanitize(&m.to),arg_s,method_parse_sig.ret.0).as_bytes()).unwrap();
            }
            writer.write_all(b"}\n").unwrap();
            // return Err(());
        },
    }
    Ok(())
}