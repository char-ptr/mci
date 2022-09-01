use std::{sync::{RwLock, Arc}, io::Write, collections::HashMap};

use rand::Rng;

use crate::{maps::{SigMappings, SigMod}, type_sig::{sanitize, parse_sig_f, parse_sig_m}};



pub fn generate_rs<W:Write>(yarn_maps : Arc<RwLock<SigMappings>>,modul:&SigMod,writer:&mut W) -> Result<(),()> {

    // let xyarn_maps = &Arc::clone(&yarn_maps).read().unwrap().sig_to_x;

    match modul {
        SigMod::Mod(x, data) => {
            writer.write_all(format!("pub mod {} {{\n",x).as_bytes()).unwrap();
            for d in data {
                if generate_rs(Arc::clone(&yarn_maps),d,writer).is_err() {
                    // return Err(());
                    break;
                }
            }
            writer.write_all(b"}\n").unwrap();
        },
        SigMod::class(c) => {
        
            if c.to.is_empty() {
                // println!("{} empty",c.srg);
                return Ok(());
            }
            let class_name = sanitize(&c.to);
            writer.write_all(format!("pub struct r#{}<'a> {{\n",class_name).as_bytes()).unwrap();
            writer.write_all(b"pub __map_internal : jni::object::JObject<'a>\n}\n").unwrap();
            // writer.write_all(b"}\n").unwrap();
            writer.write_all(format!("impl From<jni::object::JObject<'_>> for {}<'_> {{{}}}\n",class_name,stringify!{
                fn from(x:jni::object::JObject) -> Self {
                    Self {
                        __map_internal: x
                    }
                }
            }).as_bytes()).unwrap();
            writer.write_all(format!("impl {}<'_> {{\n",sanitize(&c.to)).as_bytes()).unwrap();
            writer.write_all(format!("const __map_sig : &str = \"{}\";\n",c.from).as_bytes()).unwrap();
            for f in &c.fields {
                let (fty,jclass) = parse_sig_f(&f.type_,&Arc::clone(&yarn_maps).read().unwrap().sig_to_x);
                let fty = sanitize(&fty);
                let is_jobj_t = fty.contains("JObject");
                writer.write_all(format!("/* {ty} = {sig} */\npub fn r#get_{}(&self) -> Result<{ty},()> {{\n{fnb}}} \n",sanitize(&f.to),ty=fty,sig=f.type_,
                    fnb= if is_jobj_t {format!("{}::from(self.__map_internal.env.find_class(Self::__map_sig)?)",fty)} else {"self.__map_internal.env.find_class(Self::__map_sig)?".to_string()}).as_bytes()).unwrap();
                writer.write_all(format!("/* {ty} = {sig} */\npub fn r#set_{}(&self,val : {ty}) -> () {{}} \n",sanitize(&f.to),ty=fty,sig=f.type_).as_bytes()).unwrap();
            }
            for m in &c.methods {
                let method_parse_sig = parse_sig_m(&m.type_,&Arc::clone(&yarn_maps).read().unwrap().sig_to_x);
                // println!("{:?} | {}",method_parse_sig,m.sig);
                let mut arg_s = String::new();
                for (i,str) in method_parse_sig.args.iter().enumerate() {
                    let name = sanitize(&m.args.get(i).unwrap_or(&format!("unknwnarg_{}",rand::thread_rng().gen::<u32>())));
                    arg_s.push_str(&format!("r#{} : {},",name,sanitize(str)));
                }
                writer.write_all(format!("pub fn r#{}({}) -> {} {{ }}\n",sanitize(&m.to),arg_s,method_parse_sig.ret).as_bytes()).unwrap();
            }
            writer.write_all(b"}\n").unwrap();
            return Err(());
        },
    }
    Ok(())
}