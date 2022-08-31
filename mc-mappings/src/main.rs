use std::{path::PathBuf, sync::{Arc, RwLock}, io::Write};

use map_utils::{maps::{yarn::run_dir, SigMappings}, gen::generate_rs};

fn main() {
    let mut map_to_rs = std::fs::File::create("./src/maps.rs").unwrap();
    let mut map_to_rstx = std::fs::File::create("./maps.rs.txt").unwrap();
    let start_path = PathBuf::from("./mappings/mappings/net");
    let mut mod_stk = vec!["root".to_string()];

    let sigs = Arc::new(RwLock::new(SigMappings::default()));
    let sig = Arc::clone(&sigs);
    let mut mdstks = vec!["root".to_string()];
    let mut mods = {sig.write().unwrap().mods.clone()};

    run_dir(&start_path, Arc::clone(&sigs),&mut mods, &mut mdstks);
    println!("done parsing");
    {
        let sigs = Arc::clone(&sigs);

        generate_rs(Arc::clone(&sigs), &mods, &mut map_to_rs);

        println!("done")

        // let sigs = sigs.read().unwrap();
        // // println!("{:?}",sigs);
        // map_to_rstx.write_all(format!("{:#?}",sigs).as_bytes());
    }

    // map_to_rs.write_all(b"use std::vec::Vec;\nuse jni::object::JObject;\n");

}