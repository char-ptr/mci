#![feature(new_uninit)]
mod mci;
use toy_arms::{VirtualKeyCode};


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

        // let MinecraftClient = jenv.find_class("net/minecraft/class_310").unwrap();
        // let class_loader_field_id = Launch.get_static_field_id("classLoader", "Lnet/minecraft/launchwrapper/LaunchClassLoader;");
        // if let JFieldTypesRet::Object(class_loader) = JFieldTypes::Object.get_field(&jenv, &Launch, class_loader_field_id) {
        //     println!("class_loader: {:?}", class_loader.ptr);
        // }
        // class_loader.

    }

    loop {
        if toy_arms::detect_keydown!(VirtualKeyCode::VK_INSERT) {
            break;
        }
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