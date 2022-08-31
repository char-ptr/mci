use std::ffi::CString;

use jdk_sys::jfieldID;
use crate::unchecked_jnic;
use super::env::Jenv;

pub struct JObject<'a> {
    pub ptr : jdk_sys::jobject,
    pub env : &'a Jenv<'a>,
}

impl<'a> JObject<'a> {
    pub fn new(ptr : jdk_sys::jobject,env : &'a Jenv) -> Self {
        JObject {
            ptr,
            env,
        }
    }
    pub fn get_field_id(&self, name:&str, sig:&str) -> jfieldID {
        let name = CString::new(name).unwrap().as_ptr();
        let sig = CString::new(sig).unwrap().as_ptr();
        unchecked_jnic!(self.env.ptr,GetFieldID, self.ptr, name, sig)
    }
    pub fn get_static_field_id(&self, name:&str, sig:&str) -> jfieldID {
        let name = CString::new(name).unwrap().as_ptr();
        let sig = CString::new(sig).unwrap().as_ptr();
        unchecked_jnic!(self.env.ptr,GetStaticFieldID, self.ptr, name, sig)
    }
}