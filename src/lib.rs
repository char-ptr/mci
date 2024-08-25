#![deny(rust_2018_idioms)]
#![feature(new_uninit)]
mod mci;
use std::ffi::CString;

use jni::{jstring::JString, object::JObject};

#[cfg(any(target_os = "macos", target_os = "linux"))]
use ctor::*;
#[cfg(target_os = "macos")]
#[ctor]
fn entry() {
    use std::time::Duration;
    let mut args = std::env::args();

    if args.next().unwrap().contains("java")
        && args
            .filter_map(|a| {
                if a.contains("minecraft") && a.ends_with(".jar") {
                    Some(a)
                } else {
                    None
                }
            })
            .count()
            > 0
    {
        println!("pogging");
        std::thread::spawn(|| {
            std::thread::sleep(Duration::from_secs(2));

            main_thread_wrap();
        });
    }
}
// #[cfg(not(target_os = "macos"))]
#[cfg_attr(not(target_os = "macos"), poggers_derive::create_entry(no_console))]
// #[poggers_derive::create_entry(no_console)]
fn main_thread() -> Result<(), String> {
    use mc_mappings::mappings::net::minecraft::{
        client::MinecraftClient, text::Text, util::math::Vec3d,
    };

    let mut mci = mci::MCI::default();

    mci.load_jvm()?;

    mci.attach_current_thread()?;

    println!("we're chillin");
    {
        let jenv = mci.get_jenv();
        let jenv = jenv.write().unwrap();
        let ver = jenv.get_version();
        println!("version: {}", ver);

        // mc_mappings::m_mc::MinecraftClient::s_call_getInstance(&jenv, &[]).call();

        if let Ok(mcc) = MinecraftClient::ms_getInstance_method_1551(&jenv) {
            println!("mc = ok");
            println!("is 64 bit: {:?}", mcc.is64Bit());
            if let Ok(plr) = mcc.player() {
                // let msg = JString::new("hi from rust",&jenv);
                // let txt = Text { i: JObject::null(&jenv) };
                // std::thread::sleep(std::time::Duration::from_secs(1));
                // let mut c = 0;
                // while c < 3 {
                //     plr.m_sendChatMessage_method_44096(&msg.obj,&txt);
                //     c+=1;
                //     std::thread::sleep(std::time::Duration::from_secs(14));
                // }
                // plr.m_sendChatMessage_method_44096(&msg.obj,&txt);
                let move_vec = Vec3d::m_init_leleleid(&jenv, 3f64, 0f64, 0f64);

                // move_vec.
            } else {
                println!("what the fuck?")
            }
            if let Ok(ver) = mcc.gameVersion() {
                println!("game version: {:?}", JString::from(ver));
            }
        }

        println!("q");
    }

    Ok(())
}

fn main_thread_wrap() {
    use std::panic;

    match panic::catch_unwind(|| main_thread()) {
        Err(e) => {
            println!("`main` has panicked: {:#?}", e);
        }
        Ok(r) => match r {
            Err(e) => {
                eprint!("`main` failed with {:?}", e);
            }
            _ => {}
        },
    }

    println!("done");
}
