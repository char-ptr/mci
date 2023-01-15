use std::{mem, sync::{Arc, RwLock}, ptr};

use jdk_sys::{JNIEnv, JavaVM};
// use toy_arms::internal::get_module_function_address;

use jni::env::Jenv;


type GetJvms = unsafe extern "C" fn (vmBuf : *mut *mut JavaVM,BufLen: isize, nVMs : *mut i32) -> jdk_sys::jint;

#[derive(Debug)]
pub struct MCI<'a> {
    jvm : Arc<RwLock<*mut JavaVM>>,
    jenv : Arc<RwLock<Jenv<'a>>>,
}
impl Default for MCI<'_> {
    fn default() -> Self {
        Self { jvm: Arc::new(RwLock::new(ptr::null_mut())), jenv: Arc::new(RwLock::new(Jenv::default())) }
    }
}

impl<'a> MCI<'a> {
    pub fn get_jvm(&self) -> Arc<RwLock<*mut JavaVM>> {
        Arc::clone(&self.jvm)
    }
    pub fn get_jenv(&self) -> Arc<RwLock<Jenv<'a>>> {
        Arc::clone(&self.jenv)
    }

    pub fn load_jvm(&mut self) -> Result<(), String> {
        let get_jvms = jdk_sys::JNI_GetCreatedJavaVMs;
        let i32_null_ptr : *mut i32 = std::ptr::null_mut();
        let mut jvm_ptr: *mut *mut JavaVM = &mut *self.jvm.write().or(Err("unable to write to jvm".to_string()))?;
        
        unsafe {get_jvms(jvm_ptr,1, i32_null_ptr);};

        Ok(())
    }

    pub fn attach_current_thread(&mut self) -> Result<(),String> {
        let mut jvm = *self.jvm.write().or(Err("unable to write to jvm".to_string()))?;
        let mut jenv2 :*mut *mut JNIEnv = &mut self.jenv.write().or(Err("unable to write to jvm".to_string()))?.ptr;
        unsafe {(*(*jvm)).AttachCurrentThread.unwrap()(jvm,std::mem::transmute(jenv2),ptr::null_mut());};

        if jenv2.is_null() {
            Err("AttachCurrentThread failed".to_string())
        } else {
            unsafe {
                if (*jenv2).is_null() {
                    return Err("AttachCurrentThread failed".to_string());
                }
            }
            Ok(())
        }
    }
}
impl Drop for MCI<'_> {
    fn drop(&mut self) {
        let mut jvm = *self.jvm.write().unwrap();
        let mut jenv = *self.jenv.write().unwrap();

        if jenv.ptr.is_null() || jvm.is_null() {
            return;
        } else {
            println!("detaching");
            unsafe {(*(*jvm)).DetachCurrentThread.unwrap()(jvm);};
        }

    }
}