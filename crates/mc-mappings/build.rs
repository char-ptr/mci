use map_gen::generator::Generator;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    // let mc_ver = std::env::var("MCI_GAME_VER").unwrap_or("1.19".to_string());
    // let tiny = reqwest::blocking::get(format!("https://raw.githubusercontent.com/FabricMC/intermediary/master/mappings/{mc_ver}.tiny"))
    //     .expect("unable to download tiny mappings").bytes().expect("unable to get bytes");

    let MCM_DIR: String = std::env::var("MCMAPS").unwrap();
    let MCM_DIRP: PathBuf = PathBuf::from(&MCM_DIR);
    println!("cargo:rerun-if-env-changed=MCMAPS");
    let OUT_DIRP: PathBuf = PathBuf::from(&std::env::var("OUT_DIR").unwrap());
    // let mut tinyf = File::create(format!("{OUT_DIR}/{mc_ver}.tiny",)).expect("unable to create tiny mappings file");
    // tinyf.write_all(&*tiny);

    // if let Ok(mut ch) = Command::new("git").arg("clone").arg("--depth").arg("1").arg("--sparse").arg("-b").arg(&mc_ver).arg("https://github.com/FabricMC/yarn").arg(format!("{OUT_DIR}/yarn-maps{mc_ver}")).spawn() {
    //     ch.wait();
    //     std::env::set_current_dir(format!("{OUT_DIR}/yarn-maps{mc_ver}"));
    //     println!("hi {:?}",std::env::current_dir());
    //     if let Ok(mut ch2) = Command::new("git").arg("sparse-checkout").arg("set").arg("mappings").spawn() {
    //         ch2.wait();
    //     } else {
    //         eprintln!("unable to run sparse");
    //     }

    // generator code

    let mut gen = Generator::new();

    gen.Yarn
        .run_directory(MCM_DIRP.join(format!("{MCM_DIR}/yarn-maps/mappings")), None)
        .expect("unable to parse yarn mappings");
    let mut tiny = BufReader::new(
        File::open(MCM_DIRP.join(format!("maps.tiny"))).expect("unable to parse tiny mappings"),
    );
    gen.Tiny.populate_from_reader(&mut tiny);

    let code = gen.generate();
    let min_code = rust_minify::minify(&code).expect("unable to minify generated code");
    let f = File::create(OUT_DIRP.join("gen.rs"));
    f.unwrap().write_all(min_code.to_string().as_bytes());

    // } else {
    //     eprintln!("unable to clone yarn maps");
    // }
}

