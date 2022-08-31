use std::{ffi::CString, marker::PhantomData, ptr};

use jdk_sys::JNIEnv;

use crate::{unchecked_jnic, unchecked_jnice};

use super::object::JObject;


#[repr(transparent)]
#[derive(Copy, Clone,Debug)]
pub struct Jenv<'a>{
    pub ptr : *mut JNIEnv,
    lifetime : PhantomData<&'a ()>,
}
impl Jenv<'_> {
    fn get_lowest(&self) -> jdk_sys::jniNativeInterface {
        if self.ptr.is_null() {
            panic!("JNIEnv is null");
        }
        unsafe {
            if (*self.ptr).is_null() {
                panic!("JNIEnv is null");
            }
        }
        unsafe {**self.ptr}
    }
    pub fn get_version(&self) -> String {
        use jdk_sys;
        unsafe {(*(*self.ptr)).GetVersion.unwrap()(self.ptr);}

        match unchecked_jnic!(self.ptr,GetVersion) as u32 {
            JNI_VERSION_10 => "10+",
            JNI_VERSION_1_1 => "1.1",
            JNI_VERSION_1_2 => "1.2 | 1.3",
            JNI_VERSION_1_4 => "1.4 | 5.0",
            JNI_VERSION_1_6 => "6 | 7",
            JNI_VERSION_1_8 => "8 | 9",
            _=> "unknown",
        }.to_string()
    }
    pub fn find_class(&self, name:&str) -> Result<JObject,()> {
        let name = CString::new(name).unwrap();
        let jobj = unchecked_jnice!(self.ptr,FindClass, name.as_ptr())?;
        if jobj.is_null(){
            return Err(())
        }
        // if unchecked_jnic!(self.ptr,ExceptionCheck) == jdk_sys::JNI_TRUE as u8 {
        //     unchecked_jnic!(self.ptr,ExceptionDescribe);
        //     return Err(());
        // }
        let jobj_custom = JObject::new(jobj,&self);
        Ok(jobj_custom)
    }
}
impl Default for Jenv<'_> {
    fn default() -> Self {
        Jenv {
            ptr : ptr::null_mut(),
            lifetime : PhantomData,
        }
    }
}