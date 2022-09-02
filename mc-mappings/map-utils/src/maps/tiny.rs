use std::{io::{Read, BufReader, BufRead}, sync::{Arc, RwLock}, collections::HashMap, borrow::Cow};

use super::{SigField, SigMethod, SigClass, SigMappings, SigType};

pub fn parse_tiny_mappings<'a,R:Read>(reader:&mut R,m:Arc<RwLock<SigMappings<'a>>>) {
    let bufr = BufReader::new(reader);

    let sig_maps = Arc::clone(&m);

    let mut currc = None;

    for l in bufr.lines() {
        if let Ok(l) = l {

            let mut data = l.split_whitespace();
            let cmd = data.nth(0).unwrap_or("");

            match cmd {

                "CLASS" => {
                    let obf = data.nth(0).unwrap_or("");
                    let deobf = data.nth(0).unwrap_or("");
                    let sc = SigClass {
                        from: obf.to_string(),
                        to: deobf.to_string(),
                        fields: vec![],
                        methods: vec![],
                        module_stack: vec![],
                    };
                    if currc.is_some() {
                        let oldc = currc.unwrap();
                        sig_maps.write().unwrap().sig_to_x.insert(deobf.to_string(),SigType::Class(Cow::Owned(sc.clone())));
                        
                    }
                    currc = Some(sc);
                }
                "METHOD" => {
                    let class_obf = data.nth(0).unwrap_or("");
                    let sig = data.nth(0).unwrap_or("");
                    let obf = data.nth(0).unwrap_or("");
                    let id = data.nth(0).unwrap_or("");
                    let sm = SigMethod{
                        from: obf.to_string(),
                        to: id.to_string(),
                        type_: sig.to_string(),
                        parent: class_obf.to_string(),
                        args:vec![]
                    };
                    if let Some(sc) = &mut currc {
                        sig_maps.write().unwrap().sig_to_x.insert(id.to_string(),SigType::Method(Cow::Owned(sm.clone())));
                        sc.methods.push(sm);
                    }
                }
                "FIELD" => {
                    let class_obf = data.nth(0).unwrap_or("");
                    let sig = data.nth(0).unwrap_or("");
                    let obf = data.nth(0).unwrap_or("");
                    let id = data.nth(0).unwrap_or("");

                    let sf = SigField{
                        from: obf.to_string(),
                        to: id.to_string(),
                        type_: sig.to_string(),
                        parent: class_obf.to_string(),
                    };
                    if let Some(sc) = &mut currc {
                        sig_maps.write().unwrap().sig_to_x.insert(id.to_string(),SigType::Field(Cow::Owned(sf.clone())));
                        sc.fields.push(sf);
                    }
                }

                _=>{}
            }

        }
    }

    println!("tiny mapping file done, test:");
    {
        let data = Arc::clone(&m);
        // let test_va = data.write().unwrap().sig_to_x.get("field_21658");
        println!("field_21658 = {:?}",data.write().unwrap().sig_to_x.get("field_21658"));
    }

}
