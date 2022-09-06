use std::{path::PathBuf, process, str::FromStr, sync::{Arc, RwLock}};

use map_utils::{maps::{SigMappings, yarn::run_dir}, gen::generate_rs};


fn main() {
    println!("cargo:rerun-if-changed=version.txt");

    let outdir = PathBuf::from_str(".").unwrap();
    
    if !outdir.join("mappings").exists() {
        process::Command::new("git").args(vec!["clone","--depth","1","--filter=blob:none","--sparse", "https://github.com/FabricMC/yarn.git", &outdir.join("mappings").display().to_string()]).spawn().unwrap().wait().unwrap();
        process::Command::new("git").args(vec!["-C",&outdir.join("mappings").display().to_string(),"sparse-checkout", "set","mappings"]).spawn().unwrap().wait().unwrap();
    }
    
    // return;

    let start_path = outdir.join("mappings/mappings/net");


    let outdir = PathBuf::from_str(&std::env::var("OUT_DIR").unwrap()).unwrap();
    
    let mut map_to_rs = std::fs::File::create(outdir.join("maps.rs")).unwrap();
    let mut map_to_rstx = std::fs::File::create("./maps.rs.txt").unwrap();
    let start_path = PathBuf::from("./mappings/mappings/net");
    let mut mod_stk = vec!["root".to_string()];

    let sigs = Arc::new(RwLock::new(SigMappings::default()));
    let tiny_sigs = Arc::new(RwLock::new(SigMappings::default()));
    let sig = Arc::clone(&sigs);
    let tiny_sig = Arc::clone(&tiny_sigs);
    let mut mdstks = vec!["root".to_string()];
    let mut mods = {sig.write().unwrap().mods.clone()};

    let mut tiny_file = std::fs::File::open(concat!("./mappings/tiny/", include!("./version.txt"), ".tiny")).expect("unable to get tf");

    map_utils::maps::tiny::parse_tiny_mappings(&mut tiny_file, Arc::clone(&tiny_sig));

    run_dir(&start_path, Arc::clone(&sigs),&mut mods, &mut mdstks);
    println!("done parsing");
    {
        let sigs = Arc::clone(&sigs);

        {
            (*sigs.write().unwrap()).mods = mods.clone();
        }

        generate_rs(Arc::clone(&sigs),Arc::clone(&tiny_sig), &mods, &mut map_to_rs, "",&mut vec![]);

        println!("done");

        // let sigs = sigs.read().unwrap();
        // // println!("{:?}",sigs);
        // map_to_rstx.write_all(format!("{:#?}",sigs).as_bytes());
    }
    // map_to_rstx.write_all(&mut bf);

}