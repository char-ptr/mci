#![feature(fs_try_exists)]
use std::io::Seek;
use std::process;
use std::{path::PathBuf, io::{Write, Read, BufRead}, fs, collections::HashMap, sync::RwLock, str::FromStr};
use lazy_static::lazy_static;
use rand::Rng;
lazy_static!{
    static ref sig_to_class : RwLock<HashMap<String,(MClass,Vec<String>)>> =  RwLock::new(HashMap::<String,(MClass,Vec<String>)>::new());
}


#[derive(PartialEq, Eq)]
enum MState {
    kw,
    class,
    field,
    method
}
#[derive(Clone,Debug)]

struct MField {
    name : String,
    type_ : String,
}
#[derive(Clone,Debug)]

struct MMethodArgs {
    name : String,
}
#[derive(Clone,Debug)]

struct MMethod {
    name : String,
    sig : String,
    args : Vec<MMethodArgs>,
}
#[derive(Clone,Debug)]
struct MClass {
    srg:String,
    name:String,
    fields:Vec<MField>,
    methods:Vec<MMethod>,
    classes:Vec<MClass>,

}
#[derive(Debug,Clone)]
struct Mapping {
    
    classes: Vec<MClass>,

}
#[derive(Debug,Clone)]
enum MapOrMod {
    Map(Mapping),
    Mod(Module),
}

#[derive(Clone,Debug)]
struct Module(String,Vec<MapOrMod>);


fn generate_rs<W:Write>(modu:&Module,writer:&mut W) {
    writer.write_all(format!("pub mod {} {{\n",modu.0).as_bytes()).unwrap();
    for m in &modu.1 {
        match m {
            MapOrMod::Map(m) => {
                for c in &m.classes {
                    if c.name.is_empty() {
                        // println!("{} empty",c.srg);
                        continue;
                    }
                    writer.write_all(format!("pub struct r#{} {{\n",sanitize(&c.name)).as_bytes()).unwrap();
                    writer.write_all(b"}\n").unwrap();
                    writer.write_all(format!("impl {} {{\n",sanitize(&c.name)).as_bytes()).unwrap();
                    for f in &c.fields {
                        let fty = sanitize(&parse_sig_f(&f.type_));
                        writer.write_all(format!("/* {ty} = {sig} */\npub fn r#get_{}() -> {ty} {{}} \n",sanitize(&f.name),ty=fty,sig=f.type_).as_bytes()).unwrap();
                        writer.write_all(format!("/* {ty} = {sig} */\npub fn r#set_{}(val : {ty}) -> () {{}} \n",sanitize(&f.name),ty=fty,sig=f.type_).as_bytes()).unwrap();
                    }
                    for m in &c.methods {
                        let method_parse_sig = parse_sig_m(&m.sig);
                        // println!("{:?} | {}",method_parse_sig,m.sig);
                        let mut arg_s = String::new();
                        for (i,str) in method_parse_sig.args.iter().enumerate() {
                            let name = sanitize(&m.args.get(i).unwrap_or(&MMethodArgs{name:format!("unknwnarg_{}",rand::thread_rng().gen::<u32>())}).name);
                            arg_s.push_str(&format!("r#{} : {},",name,sanitize(str)));
                        }
                        writer.write_all(format!("pub fn r#{}({}) -> {} {{ }}\n",sanitize(&m.name),arg_s,method_parse_sig.ret).as_bytes()).unwrap();
                    }
                    writer.write_all(b"}\n").unwrap();
                }
            }
            MapOrMod::Mod(m) => {
                generate_rs(&m,writer);
            }
        }
    }
    writer.write_all(b"}\n").unwrap();
}

fn main() {
    println!("cargo:rerun-if-changed=version.txt");

    let outdir = PathBuf::from_str(".").unwrap();
    
    if std::fs::try_exists(outdir.join("mappings")).is_err() {
        process::Command::new("git").args(vec!["clone","--depth","1","--filter=blob:none","--sparse", "https://github.com/FabricMC/yarn.git", &outdir.join("mappings").display().to_string()]).spawn().unwrap().wait().unwrap();
        process::Command::new("git").args(vec!["-C",&outdir.join("mappings").display().to_string(),"sparse-checkout", "set","mappings"]).spawn().unwrap().wait().unwrap();
    }
    
    
    let start_path = outdir.join("mappings/mappings/net");


    // let outdir = PathBuf::from_str(&std::env::var("OUT_DIR").unwrap()).unwrap();
    
    let mut map_to_rs = std::fs::File::create(outdir.join("maps.rs")).unwrap();
    let mut map_to_rstx = std::fs::File::create("./maps.rs").unwrap();
    let mut root_mod = Module("root".to_string(),vec![]);
    let mut mod_stk = vec!["root".to_string()];
    run_dir(&start_path, &mut root_mod,&mut mod_stk);
    map_to_rs.write_all(b"use std::vec::Vec;\nuse jni::object::JObject;\n");
    generate_rs(&root_mod, &mut map_to_rs);
    // let mut map_to_rs = std::fs::File::create(outdir.join("maps.rs")).unwrap();
    // let mut bf = vec![];
    // map_to_rs.read_to_end(&mut bf);
    // map_to_rstx.write_all(&mut bf);

}