use std::{ffi::CString, ptr};

use jdk_sys::{jfieldID, jmethodID, jvalue, JNI_TRUE};

use crate::{env::Jenv, object::JObject, unchecked_jnic, unchecked_jnice, jvalue::JValue};

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

    pub fn call_static_object_method<T:From<JObject<'a>>>(&self,name:&str,sig:&str,args:Vec<JValue>) -> Result<T,()> {
        let mut obj = ptr::null_mut();
        let args = args.iter().map(|f|f.get_c_style()).collect::<Vec<jvalue>>();

        if let Ok(fid) = self.get_static_method_id(name,sig) {
            obj = unchecked_jnice!(self.env.ptr,CallStaticObjectMethodA, self.ptr, fid,args.as_ptr())?;
        }
    
        if obj.is_null() {
            return Err(());
        }

        Ok(T::from(JObject::new(obj,self.env,true)))
    }

    pub fn get_static_obj_field<T: From<JObject<'a>>>(&self,name:&str,sig:&str) -> Result<T,()> {
        let mut obj = ptr::null_mut();
        let fid = self.get_static_field_id(name,sig)?;
        obj = unchecked_jnice!(self.env.ptr,GetStaticObjectField, self.ptr, fid)?;
    
        if obj.is_null() {
            return Err(());
        }
        Ok(T::from(JObject::new(obj,self.env,true)))
    }

    pub fn get_static_bool_field(&self,name:&str,sig:&str) -> Result<bool,()> {
        let fid = self.get_static_field_id(name,sig)?;
        Ok(unchecked_jnice!(self.env.ptr,GetStaticBooleanField, self.ptr, fid)? == JNI_TRUE as u8)
    }
    pub fn get_static_byte_field(&self,name:&str,sig:&str) -> Result<i8,()> {
        let fid = self.get_static_field_id(name,sig)?;
        Ok(unchecked_jnice!(self.env.ptr,GetStaticByteField, self.ptr, fid)? as i8)
    }
    pub fn get_static_char_field(&self,name:&str,sig:&str) -> Result<char,()> {
        let fid = self.get_static_field_id(name,sig)?;
        Ok(unchecked_jnice!(self.env.ptr,GetStaticCharField, self.ptr, fid)? as u8 as char)
    }
    pub fn get_static_short_field(&self,name:&str,sig:&str) -> Result<i16,()> {
        let fid = self.get_static_field_id(name,sig)?;
        Ok(unchecked_jnice!(self.env.ptr,GetStaticShortField, self.ptr, fid)?)
    }
    pub fn get_static_int_field(&self,name:&str,sig:&str) -> Result<i32,()> {
        let fid = self.get_static_field_id(name,sig)?;
        Ok(unchecked_jnice!(self.env.ptr,GetStaticIntField, self.ptr, fid)?)
    }
    pub fn get_static_long_field(&self,name:&str,sig:&str) -> Result<i64,()> {
        let fid = self.get_static_field_id(name,sig)?;
        Ok(unchecked_jnice!(self.env.ptr,GetStaticLongField, self.ptr, fid)?)
    }
    pub fn get_static_float_field(&self,name:&str,sig:&str) -> Result<f32,()> {
        let fid = self.get_static_field_id(name,sig)?;
        Ok(unchecked_jnice!(self.env.ptr,GetStaticFloatField, self.ptr, fid)?)
    }
    pub fn get_static_double_field(&self,name:&str,sig:&str) -> Result<f64,()> {
        let fid = self.get_static_field_id(name,sig)?;
        Ok(unchecked_jnice!(self.env.ptr,GetStaticDoubleField, self.ptr, fid)?)
    }

}

impl<'a> From<JObject<'a>> for JClass<'a> {
    fn from(x: JObject<'a>) -> Self {
        Self::new(x.ptr, x.env)
    }
}