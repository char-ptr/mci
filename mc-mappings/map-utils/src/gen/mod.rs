use std::{sync::{RwLock, Arc}, io::Write, collections::{HashMap, HashSet}};

use rand::Rng;

use crate::{maps::{SigMappings, SigMod, SigField, SigMethod}, type_sig::{sanitize, parse_sig_f, parse_sig_m}};

#[inline]
pub fn get_class_code(name:&str,sig:&str,body:&str) -> String {
    format!(
        "pub struct {class_name}<'a> {{
            pub inner: jni::object::JObject<'a>
        }}
        impl<'a> {class_name}<'a> {{
            const class_sig: &str = \"{class_sig}\";
    
            {class_body}
        }}",
        class_name = name,
        class_sig = sig,
        class_body = body
    )
}
#[inline]
pub fn get_field_code(f:&SigField,field_name:&str,from:&str,rust_type:&str,java_type:&str) -> String {
    format!(
        "pub fn get_{field_name}(&self) -> Result<{rust_type},()> {{
            self.inner.get_field_{java_type}::<{rust_type}>(\"{from}\",\"{field_sig}\");
        }}
        pub fn s_get_{field_name}(env:&'a jni::env::Jenv<'a>) -> Result<{rust_type},()> {{
            let class = env.find_class(Self::class_sig);
            class.get_static_{java_type}_field::<{rust_type}>(\"{from}\",\"{field_sig}\")
        }}",
        field_sig = f.type_,
    )
}
#[inline]
pub fn get_method_code(f:&SigMethod,method_name:&str,from:&str,rust_type:&str,java_type:&str,args:Vec<(String,String)>) -> String {
    format!(
        "pub fn call_{method_name}(&self,{args}) -> Result<{rust_type},()> {{
            let args = vec![{args_name}];
            self.inner.call_{java_type}_method::<{rust_type}>(\"{from}\",\"{method_sig}\")
        }}
        pub fn s_call_{method_name}(env:&'a jni::env::Jenv<'a>,{args}) -> Result<{rust_type},()> {{
            let args=vec![{args_name}];
            let class = env.find_class(Self::__map_sig);
            class.call_static_{java_type}_method(\"{from}\",\"{method_sig}\")
        }}",
        method_sig = f.type_,
        args = args.iter().map(|(name,type_)| format!("{}:{}",name,type_)).collect::<Vec<String>>().join(", "),
        args_name = args.iter().map(|(name,_)| name.clone()).collect::<Vec<String>>().join(", "),
    )
}

pub fn generate_rs<W:Write>(yarn_maps : Arc<RwLock<SigMappings>>,tiny_maps : Arc<RwLock<SigMappings>>,modul:&SigMod,writer:&mut W,stk:&str,names:&mut Vec<String>) -> Result<(),()> {

    // let xyarn_maps = &Arc::clone(&yarn_maps).read().unwrap().sig_to_x;

    match modul {
        SigMod::Mod(x, data) => {
            writer.write_all(format!("pub mod {} {{\n",x).as_bytes()).unwrap();
            for d in data {
                let nstk = format!("{}/{}",stk,x);
                // println!("Generating {nstk}/{}",d);
                if generate_rs(Arc::clone(&yarn_maps),Arc::clone(&tiny_maps),d,writer,&nstk,names).is_err() {
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

            let class_name = if names.contains(&format!("c:{}",c.to)) {
                sanitize(&format!("{}{}",c.to,rand::thread_rng().gen::<u32>()))
            } else {
                names.push(format!("c:{}",c.to));
                sanitize(&c.to)
            };


            let mut fnm : Vec<String> = vec![];


            let mut body = String::new();

            // fields
            for field in &c.fields {
                if let Some(tinyf) = tinm.sig_to_x.get(&field.from) {
                    let (rust_type,java_type,is_jclass) = parse_sig_f(&field.type_,&Arc::clone(&yarn_maps).read().unwrap().sig_to_x);
                    let field_name = if fnm.contains(&field.to) {
                        format!("{}{}",field.to,rand::thread_rng().gen::<u32>())
                    } else {
                        fnm.push(field.to.clone());
                        field.to.clone()
                    };

                    body.push_str(&get_field_code(field,&field_name, tinyf.get_from(), &sanitize(&rust_type), &java_type))

                }
            }
            // methods
            for method in &c.methods {
                if let Some(tinyf) = tinm.sig_to_x.get(&method.from) {
                    let method_data = parse_sig_m(&method.type_,&Arc::clone(&yarn_maps).read().unwrap().sig_to_x);

                    let mut arg_s = Vec::<(String,String)>::new();
                    for (i,str) in method_data.args.iter().enumerate() {
                        let name = sanitize(&method.args.get(i).unwrap_or(&format!("unknwnarg_{}",rand::thread_rng().gen::<u32>())));
                        arg_s.push((format!("r#a{name}"),format!("{}",sanitize(&str.0))));
                    }

                    let method_name = if fnm.contains(&method.to) {
                        format!("{}{}",method.to,rand::thread_rng().gen::<u32>())
                    } else {
                        fnm.push(method.to.clone());
                        method.to.clone()
                    };

                    body.push_str(&get_method_code(method,&method_name, tinyf.get_from(), &sanitize(&method_data.ret.0), &method_data.ret.1,arg_s))

                }
            }

            let class_code = get_class_code(&class_name, &sanitize( &actual_class.get_from()),&body);

            writer.write_all(class_code.as_bytes()).unwrap();
        },
    }
    Ok(())
}