use std::{path::PathBuf, fs::File, io::Read, sync::{Arc, RwLock}, borrow::Cow};

use rand::Rng;

use super::{SigMappings, SigClass, SigMethod, SigField, SigType, SigMod};

pub fn run_dir<'a>(p:&PathBuf,m:Arc<RwLock<SigMappings<'a>>>, mod_stk : &mut SigMod<'a>, mod_stk_str : &mut Vec<String>) {
    let mut dir = p.read_dir().unwrap();
    while let Some(entry) = dir.next() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            let dir_name = path.file_name().unwrap().to_str().unwrap().to_string();
            let mut modu = SigMod::Mod(dir_name.clone(), vec![]);
            mod_stk_str.push(dir_name.clone());
            run_dir(&path,Arc::clone(&m),&mut modu,mod_stk_str);
            mod_stk_str.pop().unwrap();
            match mod_stk {
                SigMod::Mod(_, vecd) => {
                    vecd.push(modu);
                },
                SigMod::class(_) => {},
            }


        } else {
            if let Ok(mut f) = File::open(path) {
                let mut buf = String::new();
                f.read_to_string(&mut buf);
                let mut word = String::new();
                let mut words: Vec<String> = vec![];

                let mut current_c : Option<SigClass> = None;
                let mut current_m : Option<SigMethod> = None;

                let lines = buf.lines();
                // let mq  = Arc::clone(&m).write().unwrap();

                for line in lines {
                    let mut line_data = line.split_whitespace();
                    let command = line_data.nth(0).unwrap().trim();
                    // println!("{}",command);
                    match command {
                        "CLASS" => {
                            
                            if current_c.is_some() {
                                let cla = current_c.unwrap();
                                let old_from = cla.from.clone();
                                Arc::clone(&m).write().unwrap().classes.push(Cow::Owned(cla));
                                {
                                    let last = m.write().unwrap().classes.last().unwrap().clone();
                                    m.write().unwrap().sig_to_x.insert(last.from.clone(),SigType::Class(last.clone()));
                                    if let SigMod::Mod(s, vecd) = mod_stk {
                                        vecd.push(SigMod::class(last));
                                    }
                                }

                                let new_pog = format!("{}${}",old_from,line_data.nth(0).unwrap().trim().to_string());
                                let new_cla = SigClass{
                                    from:new_pog.clone(),
                                    to:line_data.nth(0).unwrap_or("").trim().to_string(),
                                    fields:vec![],
                                    methods:vec![],
                                    module_stack:mod_stk_str.to_vec(),
                                };
                                current_c = Some(new_cla);
                            } else {

                                let new_cla = SigClass{
                                    from:line_data.nth(0).unwrap_or("").trim().to_string(),
                                    to:line_data.nth(0).unwrap_or("").trim().to_string(),
                                    fields:vec![],
                                    methods:vec![],
                                    module_stack:mod_stk_str.to_vec(),
                                };
                                current_c = Some(new_cla);
                            }
                        }
                        "FIELD" => {
                            if let Some(cla) = &mut current_c {
                                let id = line_data.nth(0).unwrap().trim().to_string();
                                let mut name = line_data.nth(0).unwrap().trim().to_string();
                                let mut typ = line_data.nth(0).unwrap_or("").to_string();
                                if typ.is_empty() {
                                    typ = name.clone();
                                    name = id.clone();
                                    // println!("swp name id = {} typ= {typ}",name);
                                }
                                cla.fields.push(SigField{
                                    to:name,
                                    from:id,
                                    type_: typ,
                                    parent:cla.from.clone(),
                                })
                            }
                        }
                        "METHOD" => {
                            if let Some(cla) = &mut current_c {
                                if let Some(oldm) = current_m {
                                    cla.methods.push(oldm);
                                }
                                let id = line_data.nth(0).unwrap().trim().to_string();
                                if id == "<init>" {
                                    current_m = Some(SigMethod{
                                        to:"!!zINDMINIT".to_string(),
                                        from:id,
                                        type_:line_data.nth(0).unwrap().trim().to_string(),
                                        args:vec![],
                                        parent:cla.from.clone(),
                                    });
                                    continue;
                                }
                                let mut nam = line_data.nth(0).unwrap().trim().to_string();
                                let mut sigg = line_data.nth(0).unwrap_or("").trim().to_string();
                                if nam.starts_with('(') {
                                    sigg = nam;
                                    nam = format!("unknwnfn_{}",rand::thread_rng().gen::<u32>())
                                }
                                current_m = Some(SigMethod{
                                    to:nam,
                                    from:id,
                                    type_:sigg,
                                    parent:cla.from.clone(),
                                    args:vec![],
                                });
                            }
                        }
                        "ARG" => {
                            if let Some(m) = &mut current_m {
                                m.args.push(line_data.nth(1).unwrap().trim().to_string())
                            }
                        }
                        _=>{}

                    }

                }
                if let Some(clas) = current_c {
                    Arc::clone(&m).write().unwrap().classes.push(Cow::Owned(clas));
                    {
                        let last = m.write().unwrap().classes.last().unwrap().clone();
                        m.write().unwrap().sig_to_x.insert(last.from.clone(),SigType::Class(last));
                    }
                }

                // m.1.push(MapOrMod::Map(map));
            }
        }
    }
}