use std::{sync::{RwLock, Arc}, io::Write, collections::HashMap};

use rand::Rng;

use crate::{maps::{SigMappings, SigMod}, type_sig::{sanitize, parse_sig_f, parse_sig_m}};


pub fn generate_rs<W:Write>(yarn_maps : Arc<RwLock<SigMappings>>,modul:&SigMod,writer:&mut W) {

    // let xyarn_maps = &Arc::clone(&yarn_maps).read().unwrap().sig_to_x;

    match modul {
        SigMod::Mod(x, data) => {
            writer.write_all(format!("pub mod {} {{\n",x).as_bytes()).unwrap();
            for d in data {
                generate_rs(Arc::clone(&yarn_maps),d,writer);
            }
            writer.write_all(b"}\n").unwrap();
        },
        SigMod::class(c) => {
        
            if c.to.is_empty() {
                // println!("{} empty",c.srg);
                return;
            }
            writer.write_all(format!("pub struct r#{} {{\n",sanitize(&c.to)).as_bytes()).unwrap();
            writer.write_all(b"}\n").unwrap();
            writer.write_all(format!("impl {} {{\n",sanitize(&c.to)).as_bytes()).unwrap();
            for f in &c.fields {
                let fty = sanitize(&parse_sig_f(&f.type_,&Arc::clone(&yarn_maps).read().unwrap().sig_to_x));
                writer.write_all(format!("/* {ty} = {sig} */\npub fn r#get_{}() -> {ty} {{}} \n",sanitize(&f.to),ty=fty,sig=f.type_).as_bytes()).unwrap();
                writer.write_all(format!("/* {ty} = {sig} */\npub fn r#set_{}(val : {ty}) -> () {{}} \n",sanitize(&f.to),ty=fty,sig=f.type_).as_bytes()).unwrap();
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
        },
    }
}