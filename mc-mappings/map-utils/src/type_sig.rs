use std::{str::FromStr, collections::HashMap};

use crate::maps::SigType;
static ok_chars: [char;3] = [':','<','>'];

pub fn sanitize(mut s:&str) -> String {
    let mut sn = String::from_str(s).unwrap();
    if s.contains('/') {
        if let Some(last_nams) = s.rfind(':') {
            let mut pog = s[..last_nams+1].to_string();
            pog.push_str(s.rsplit('/').nth(0).unwrap());
            sn = pog;
            if sn.ends_with("::") {
                // println!("new {} old {}",sn,s);
            }
        } else {
            sn = s.rsplit('/').nth(0).unwrap().to_string();
        }
    }
    sn.chars().map(|f|if f.is_ascii_alphanumeric() || ok_chars.contains(&f) {f} else {'_'} ).collect()
}
#[derive(Debug)]
pub struct SigParseM {
    pub args:Vec<String>,
    pub ret:String,
}
pub fn parse_sig_basic(ch:char) -> String {
    match ch {
        'Z' => "bool",
        'B' => "u8",
        'C' => "char",
        'S' => "i16",
        'I' => "i32",
        'J' => "i64",
        'F' => "f32",
        'D' => "f64",
        'V' => "()",
        _=>""
    }.to_string()
}

pub fn resolve_srg(srg:&str,sig2class:&HashMap<String, SigType>) -> String {
    // println!("srg = {}",srg);
    match sig2class.get(srg) {
        Some(cla) => {
            match cla {
                SigType::Class(cla) => {

                    let mut base = cla.module_stack.join("::");
                    if cla.to.is_empty() {
                        // println!("{} empty",srg);
                        return "".to_string();
                    }
                    base.push_str(&format!("::{}",cla.to));
                    base
                },
                SigType::Field(_) => "".to_string(),
                SigType::Method(_) => "".to_string(),
            }
            // println!("{:?}",cla.1);
        },
        None => "".to_string()
    }
}
pub fn parse_sig_f(sig:&str,sig2class:&HashMap<String, SigType>) -> String {
    let mut begin = true;
    let mut ret = String::new();
    let mut j_class_search = false;
    let mut buf = String::new();
    let mut vec_count = 0;
    let mut vec_valid = true;
    for char in sig.chars() {
        match char {
            'L' => {
                vec_valid = false;
                if begin {
                    j_class_search = true;
                    begin = false;
                }
            }
            '[' => {
                if vec_valid {
                    vec_count += 1;
                    begin=true;
                }
            }
            ';' => {
                vec_valid = false;
                if j_class_search {
                    begin = true;
                    break;
                }
            }
            _=> {buf.push(char);vec_valid = false; begin=false;}
        };
        
    };
    if buf.len() > 0 {
        if buf.trim().len() == 1 {
            ret.push_str(&parse_sig_basic(buf.chars().nth(0).unwrap()));
        } 
        else if j_class_search {
            let resolved = resolve_srg(&buf,sig2class);
            // println!("jcs {} {}",buf,resolved);
            if resolved.is_empty() {
                ret.push_str("JObject");
            } else {

                ret.push_str(&resolve_srg(&buf,sig2class));
            }
        }
            
        else {
            // println!("oh fuck {}",buf);
        }
    }
    for _ in 0..vec_count {
        let inside = ret;
        ret = format!("std::vec::Vec<{}>",inside);
    }
    if ret.is_empty() {
        // println!("empty {}",sig);
    }
    ret
}
pub fn parse_sig_m(sig:&str,sig2class:&HashMap<String, SigType>) -> SigParseM {
    let mut parsed = SigParseM{
        args:vec![],
        ret:String::new(),
    };
    let mut in_args = false;
    let mut begin = false;
    let mut jclass_search = false;
    let mut arr_start = false;
    let mut current_arg = String::new();
    for char in sig.chars() {
        match char {
            '(' => {
                in_args = true;
                begin = true;
            }
            ';' => {
                begin = true;
                if jclass_search {
                    let curr = current_arg.clone();
                    let res = resolve_srg(&curr,sig2class);
                    if !res.is_empty() {
                        current_arg = res;
                    } else {
                        current_arg = "JObject".to_string();
                    }
                }
                if in_args {
                    parsed.args.push(current_arg.clone());
                    current_arg.clear();
                } else {
                    if current_arg != "()" {
                        current_arg = sanitize(&current_arg)
                    }
                    parsed.ret = current_arg.clone();
                    current_arg.clear();
                }
                jclass_search = false;
            }
            ')' => {
                in_args = false;
                jclass_search =false;
                begin = true;
                current_arg.clear();
            }
            'L' => {
               if begin {
                    jclass_search = true;
                    begin = false;
    
               } else {
                    current_arg.push(char);
               }
            }
            '[' => {arr_start=true}
            ']' => {
                if arr_start {
                    arr_start = false;
                    let ca = current_arg.clone();
                    current_arg = format!("Vec<{}>",ca);
                }
            }
            _=>{
                if in_args {
                    if jclass_search {
                        current_arg.push(char);
                    } else {
                        let sig_parse = parse_sig_basic(char);
                        if sig_parse.is_empty() {

                        } else {
                            parsed.args.push(sig_parse);
                            current_arg.clear();
                        }
                    }

                } else {
                    current_arg.push(char);
                }

            }
        }
    }
    if jclass_search {
        // println!("JCSEARRRR!{}",current_arg);
    } else if current_arg.trim().len() == 1 {
        let sigparse = parse_sig_basic(current_arg.trim().chars().next().unwrap());
        if sigparse.is_empty() {
            // println!("QQQAAARR!{} | parse = {}",current_arg,sigparse)
        } else {
            parsed.ret = sigparse;
        }
    }
    parsed
}