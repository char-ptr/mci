#![feature(new_uninit)]
mod mci;
use std::ffi::CString;

use jni::{object::JObject, jstring::JString};

#[cfg(any(target_os="macos",target_os="linux"))]
use ctor::*;
#[cfg(target_os="macos")]
#[ctor]
fn entry() {
    use std::time::Duration;
    let mut args = std::env::args();

    if args.next().unwrap().contains("java") && args.filter_map(|a| if a.contains("minecraft") && a.ends_with(".jar") {Some(a)} else {None}).count() > 0  {
        println!("pogging");
        std::thread::spawn(|| {
            std::thread::sleep(Duration::from_secs(2));

            main_thread_wrap();
        });
    }
}
#[cfg(not(target_os="macos"))]
#[poggers_derive::create_entry]
fn main_thread() -> Result<(), String> {
    use jni::jvalue::JValue;


    let mut mci = mci::MCI::default();

    mci.load_jvm()?;

    mci.attach_current_thread()?;


    println!("we're chillin");

    {
        let jenv = mci.get_jenv();
        let jenv = jenv.write().unwrap();
        let ver = jenv.get_version();
        println!("version: {}", ver);
        let minecraft_client = jenv.find_class("eev").unwrap();
        println!("mc = {:?}",minecraft_client.ptr);
        if let Ok(obj) = minecraft_client.call_static_object_method::<JObject>("G", "()Leev;",&vec![]) {
            println!("oke {:?}",obj.ptr);

            let is_64bit = obj.get_field_boolean("ac", "Z");
            let game_ver = obj.get_field_object::<JString>("Y", "Ljava/lang/String;");
            println!("is_64bit = {:?}",is_64bit);
            if let Ok(gamev) = game_ver {
                println!("minecraft version = {:?}",gamev.to_string());
            }
        }
        
        // MinecraftVersion class

        // let minecraft_version = jenv.find_class("v").unwrap();
        // if let Ok(GameVersionObj) = minecraft_version.get_static_object_field::<JObject>("a", "Lae;") {
        //     let str = GameVersionObj.get_field_object::<JString>("d", "Ljava/lang/String;");
        //     if let Ok(str) = str {
        //         println!("game version = {:?}",str);
        //     }
        // }
        println!("q");

        

    }


    Ok(())
}


fn main_thread_wrap() {
    use std::panic;

    match panic::catch_unwind(||main_thread()) {
        Err(e) => {
            println!("`main` has panicked: {:#?}", e);
        }
        Ok(r) => match r {
            Err(e) => {
                eprint!("`main` failed with {:?}", e);
            }
            _ => {},
        },
    }

    println!("done");
}