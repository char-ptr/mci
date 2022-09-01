use std::ffi::CString;

use jdk_sys::{jfieldID, jmethodID};

use crate::{env::Jenv, object::JObject, unchecked_jnic, unchecked_jnice};

pub struct JClass<'a> {
    pub ptr : jdk_sys::jclass,
    pub env : &'a Jenv<'a>,
}
impl<'a> JClass<'a> {
    pub fn new(ptr:jdk_sys::jclass,env : &'a Jenv) -> Self {
        JClass {
            ptr,
            env,
        }
    }
    pub fn get_field_id(&self, name:&str, sig:&str) -> Result<jfieldID,()> {
        let name = CString::new(name).unwrap();
        let sig = CString::new(sig).unwrap();
        unchecked_jnice!(self.env.ptr,GetFieldID, self.ptr, name.as_ptr(), sig.as_ptr())

    }
    pub fn get_static_field_id(&self, name:&str, sig:&str) -> Result<jfieldID,()> {

        let name = CString::new(name).unwrap();
        let sig = CString::new(sig).unwrap();
        unchecked_jnice!(self.env.ptr,GetStaticFieldID, self.ptr, name.as_ptr(), sig.as_ptr())
        
    }
    pub fn get_method_id(&self, name:&str, sig:&str) -> Result<jmethodID,()> {
        let name = CString::new(name).unwrap();

        let sig = CString::new(sig).unwrap();
        unchecked_jnice!(self.env.ptr,GetMethodID, self.ptr, name.as_ptr(), sig.as_ptr())

    }
    pub fn get_static_method_id(&self, name:&str, sig:&str) -> Result<jmethodID,()> {
        let name = CString::new(name).unwrap();
        let sig = CString::new(sig).unwrap();
        unchecked_jnice!(self.env.ptr,GetStaticMethodID, self.ptr, name.as_ptr(), sig.as_ptr())
        
    }
}

impl<'a> From<JObject<'a>> for JClass<'a> {
    fn from(x: JObject<'a>) -> Self {
        Self::new(x.ptr, x.env)
    }
}