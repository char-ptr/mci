#![feature(fs_try_exists)]

use std::{path::PathBuf, process, str::FromStr};


fn main() {
    println!("cargo:rerun-if-changed=version.txt");

    let outdir = PathBuf::from_str(".").unwrap();
    
    if std::fs::try_exists(outdir.join("mappings")).is_err() {
        process::Command::new("git").args(vec!["clone","--depth","1","--filter=blob:none","--sparse", "https://github.com/FabricMC/yarn.git", &outdir.join("mappings").display().to_string()]).spawn().unwrap().wait().unwrap();
        process::Command::new("git").args(vec!["-C",&outdir.join("mappings").display().to_string(),"sparse-checkout", "set","mappings"]).spawn().unwrap().wait().unwrap();
    }
    
    
    let start_path = outdir.join("mappings/mappings/net");


    // let outdir = PathBuf::from_str(&std::env::var("OUT_DIR").unwrap()).unwrap();
    
    // let mut map_to_rs = std::fs::File::create(outdir.join("maps.rs")).unwrap();
    // let mut map_to_rstx = std::fs::File::create("./maps.rs").unwrap();
    // let mut root_mod = Module("root".to_string(),vec![]);
    // let mut mod_stk = vec!["root".to_string()];
    // run_dir(&start_path, &mut root_mod,&mut mod_stk);
    // map_to_rs.write_all(b"use std::vec::Vec;\nuse jni::object::JObject;\n");
    // generate_rs(&root_mod, &mut map_to_rs);
    // let mut map_to_rs = std::fs::File::create(outdir.join("maps.rs")).unwrap();
    // let mut bf = vec![];
    // map_to_rs.read_to_end(&mut bf);
    // map_to_rstx.write_all(&mut bf);

}