#![feature(new_uninit)]
mod mci;
use std::ffi::CString;

use jni::object::JObject;

#[cfg(any(target_os="macos",target_os="linux"))]
use ctor::*;
#[cfg(any(target_os="macos",target_os="linux"))]
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


#[cfg(windows)]
toy_arms::create_entrypoint!(main_thread_wrap);

fn main_thread() -> Result<(), String> {

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
            println!("is_64bit = {:?}",is_64bit);
        }
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