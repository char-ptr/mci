use std::{path::PathBuf, env, fs};

fn main() {

    println!("cargo:rerun-if-env-changed=JAVA_HOME");
    let mut jdk_location = PathBuf::from(std::env::var("JAVA_HOME").expect("JAVA_HOME not set"));
    
    if jdk_location.display().to_string().contains("homebrew") && !jdk_location.display().to_string().contains("Contents"){
        jdk_location = jdk_location.join("libexec/openjdk.jdk/Contents/Home");
        println!("cargo:rustc-link-search={}",jdk_location.join("lib/server").display());
    }
    println!("cargo:rustc-link-search={}",jdk_location.join("lib").display());
    println!("cargo:rustc-link-lib=jvm");

    { // fix jni jni_md location cuz it's fucked
        let os = match std::env::consts::OS {
            "macos" => "darwin",
            "linux" => "linux",
            "windows" => "win32",
            _ => panic!("unsupported os"),
        };
        fs::copy(
            jdk_location.join("include").join(os).join("jni_md.h"), 
            jdk_location.join("include/jni_md.h")
        );
    }


    let bindings = bindgen::Builder::default()
    // The input header we would like to generate
    // bindings for.
    .header(jdk_location.join("include/jni_md.h").display().to_string())
    .header(jdk_location.join("include/jni.h").display().to_string())
    .header(jdk_location.join("include/jvmti.h").display().to_string())
    // Tell cargo to invalidate the built crate whenever any of the
    // included header files changed. //a
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    // Finish the builder and generate the bindings.
    .generate()
    // Unwrap the Result and panic on failure.Ÿ`
    .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        // .write_to_file("./binds.rs")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    {
        fs::remove_file(jdk_location.join("include/jni_md.h")).expect("Failed to clean up jni_md.h");
    }
}