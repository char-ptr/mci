use std::{ffi::CString, ptr};

use jdk_sys::{jfieldID, jmethodID, jvalue, JNI_TRUE};
use crate::{unchecked_jnic, unchecked_jnice, jvalue::JValue};
use super::env::Jenv;

pub struct JObject<'a> {
    pub ptr : jdk_sys::jobject,
    pub env : &'a Jenv<'a>,
    pub is_instance: bool
}

impl<'a> JObject<'a> {
    pub fn new(ptr : jdk_sys::jobject,env : &'a Jenv,is_instance:bool) -> Self {
        JObject {
            ptr,
            env,
            is_instance
        }
    }
    pub fn get_class(&self) -> JObject<'a> {
        if self.is_instance {
            JObject::new(unchecked_jnic!(self.env.ptr, GetObjectClass, self.ptr),self.env,false)
        } else {
            JObject::new(self.ptr,self.env,false)
        }
    }

    pub fn get_field_id(&self, name:&str, sig:&str) -> Result<jfieldID,()> {
        if !self.is_instance {
            println!("attempt to get field id from static");
            return Err(());
        }
        let name = CString::new(name).unwrap();
        let sig = CString::new(sig).unwrap();
        let class = self.get_class();
        Ok(unchecked_jnic!(self.env.ptr,GetFieldID, class.ptr, name.as_ptr(), sig.as_ptr()))

    }
    pub fn get_static_field_id(&self, name:&str, sig:&str) -> Result<jfieldID,()> {
        if self.is_instance {
            println!("attempt to get static field id from instance");
            return Err(());
        }
        let name = CString::new(name).unwrap();
        let sig = CString::new(sig).unwrap();
        unchecked_jnice!(self.env.ptr,GetStaticFieldID, self.ptr, name.as_ptr(), sig.as_ptr())
        
    }
    pub fn get_method_id(&self, name:&str, sig:&str) -> Result<jmethodID,()> {
        if !self.is_instance {
            println!("attempt to get method id from static");
            return Err(());
        }
        let name = CString::new(name).unwrap();
        let sig = CString::new(sig).unwrap();
        let class = self.get_class();
        unchecked_jnice!(self.env.ptr,GetMethodID, class.ptr, name.as_ptr(), sig.as_ptr())

    }
    pub fn get_static_method_id(&self, name:&str, sig:&str) -> Result<jmethodID,()> {
        if self.is_instance {
            println!("attempt to get static method id from instance");
            return Err(());
        }
        let name = CString::new(name).unwrap();
        let sig = CString::new(sig).unwrap();
        unchecked_jnice!(self.env.ptr,GetStaticMethodID, self.ptr, name.as_ptr(), sig.as_ptr())
        
    }


    // methods


    pub fn call_object_method<T:From<JObject<'a>>>(&self,name:&str,sig:&str,args:Vec<JValue>) -> Result<T,()> {
        let mut obj = ptr::null_mut();
        let args = args.iter().map(|f|f.get_c_style()).collect::<Vec<jvalue>>();
        if let Ok(fid) = self.get_method_id(name,sig) {
            println!("{:?}",fid);
            let obj_ = unchecked_jnice!(self.env.ptr,CallObjectMethodA, self.ptr, fid,args.as_ptr());
            if let Ok(obj_) = obj_ {
                obj = obj_;
            }
        } 
        else if obj.is_null() {
            if let Ok(fid) = self.get_static_method_id(name,sig) {
                obj = unchecked_jnice!(self.env.ptr,CallStaticObjectMethodA, self.ptr, fid,args.as_ptr())?;
            }
        }
        if obj.is_null() {
            return Err(());
        }

        Ok(T::from(JObject::new(obj,self.env,true)))
    }


    // get fields

    pub fn get_field_object<T:From<JObject<'a>>>(&self,name:&str,sig:&str) -> Result<T,()> {
        let mut obj = ptr::null_mut();
        if let Ok(fid) = self.get_static_field_id(name,sig) {
            println!("{:?}",fid);
            let obj_ = unchecked_jnice!(self.env.ptr,GetStaticObjectField, self.ptr, fid);
            if let Ok(obj_) = obj_ {
                obj = obj_;
            }
        } 
        else if obj.is_null() {
            if let Ok(fid) = self.get_field_id(name,sig) {
                obj = unchecked_jnice!(self.env.ptr,GetObjectField, self.ptr, fid)?;
            }
        }
        if obj.is_null() {
            return Err(());
        }

        Ok(T::from(JObject::new(obj,self.env,true)))
    }
    pub fn get_field_bool(&self,name:&str,sig:&str) -> Result<bool,()> {
        let mut changed = false;
        let mut iret = false;
        if let Ok(fid) = self.get_static_field_id(name,sig) {
            println!("{:?}",fid);
            let obj_ = unchecked_jnice!(self.env.ptr,GetStaticBooleanField, self.ptr, fid);
            if let Ok(obj_) = obj_ {
                iret = obj_ == 1;
                changed = true;
            }
        } 
        else if iret == false {
            println!("here");
            if let Ok(fid) = self.get_field_id(name,sig) {
                println!("{:?}",fid);
                iret = unchecked_jnice!(self.env.ptr,GetBooleanField, self.ptr, fid)? == JNI_TRUE as u8;
                changed = true;
            }
        }
        if !changed {
            return Err(());
        }

        Ok(iret)
    }
    pub fn get_field_byte(&self,name:&str,sig:&str) -> Result<i8,()> {
        let mut iret = 0i8;
        if let Ok(fid) = self.get_static_field_id(name,sig) {
            println!("{:?}",fid);
            let obj_ = unchecked_jnice!(self.env.ptr,GetStaticByteField, self.ptr, fid);
            if let Ok(obj_) = obj_ {
                iret = obj_;
            }
        } 
        else if iret == 0i8{
            if let Ok(fid) = self.get_field_id(name,sig) {
                iret = unchecked_jnice!(self.env.ptr,GetByteField, self.ptr, fid)?;
            }
        }

        Ok(iret)
    }
    pub fn get_field_char(&self,name:&str,sig:&str) -> Result<char,()> {
        let mut iret = '\0';
        if let Ok(fid) = self.get_static_field_id(name,sig) {
            println!("{:?}",fid);
            let obj_ = unchecked_jnice!(self.env.ptr,GetStaticCharField, self.ptr, fid);
            if let Ok(obj_) = obj_ {
                iret = obj_ as u8 as char;
            }
        } 
        else if iret == '\0' {
            if let Ok(fid) = self.get_field_id(name,sig) {
                iret = unchecked_jnice!(self.env.ptr,GetCharField, self.ptr, fid)? as u8 as char;
            }
        }
        if iret == '\0' {
            return Err(());
        }

        Ok(iret)
    }
    pub fn get_field_short(&self,name:&str,sig:&str) -> Result<i16,()> {
        let mut iret = -9917i16;
        if let Ok(fid) = self.get_static_field_id(name,sig) {
            println!("{:?}",fid);
            let obj_ = unchecked_jnice!(self.env.ptr,GetStaticShortField, self.ptr, fid);
            if let Ok(obj_) = obj_ {
                iret = obj_;
            }
        } 
        else if iret == -9917i16 {
            if let Ok(fid) = self.get_field_id(name,sig) {
                iret = unchecked_jnice!(self.env.ptr,GetShortField, self.ptr, fid)?;
            }
        }
        if iret == -9917i16 {
            return Err(());
        }

        Ok(iret)
    }
    pub fn get_field_int(&self,name:&str,sig:&str) -> Result<i32,()> {
        let mut iret = -99175165;
        if let Ok(fid) = self.get_static_field_id(name,sig) {
            println!("{:?}",fid);
            let obj_ = unchecked_jnice!(self.env.ptr,GetStaticIntField, self.ptr, fid);
            if let Ok(obj_) = obj_ {
                iret = obj_;
            }
        } 
        else if iret == -99175165 {
            if let Ok(fid) = self.get_field_id(name,sig) {
                iret = unchecked_jnice!(self.env.ptr,GetIntField, self.ptr, fid)?;
            }
        }
        if iret == -99175165 {
            return Err(());
        }

        Ok(iret)
    }
    pub fn get_field_long(&self,name:&str,sig:&str) -> Result<i64,()> {
        let mut iret = -99175165i64;
        if let Ok(fid) = self.get_static_field_id(name,sig) {
            println!("{:?}",fid);
            let obj_ = unchecked_jnice!(self.env.ptr,GetStaticLongField, self.ptr, fid);
            if let Ok(obj_) = obj_ {
                iret = obj_;
            }
        } 
        else if iret == -99175165i64 {
            if let Ok(fid) = self.get_field_id(name,sig) {
                iret = unchecked_jnice!(self.env.ptr,GetLongField, self.ptr, fid)?;
            }
        }
        if iret == -99175165i64 {
            return Err(());
        }

        Ok(iret)
    }
    pub fn get_field_float(&self,name:&str,sig:&str) -> Result<f32,()> {
        let mut iret = -99175165f32;
        if let Ok(fid) = self.get_static_field_id(name,sig) {
            println!("{:?}",fid);
            let obj_ = unchecked_jnice!(self.env.ptr,GetStaticFloatField, self.ptr, fid);
            if let Ok(obj_) = obj_ {
                iret = obj_;
            }
        } 
        else if iret == -99175165f32 {
            if let Ok(fid) = self.get_field_id(name,sig) {
                iret = unchecked_jnice!(self.env.ptr,GetFloatField, self.ptr, fid)?;
            }
        }
        if iret == -99175165f32 {
            return Err(());
        }

        Ok(iret)
    }
    pub fn get_field_double(&self,name:&str,sig:&str) -> Result<f64,()> {
        let mut iret = -99175165f64;
        if let Ok(fid) = self.get_static_field_id(name,sig) {
            println!("{:?}",fid);
            let obj_ = unchecked_jnice!(self.env.ptr,GetStaticDoubleField, self.ptr, fid);
            if let Ok(obj_) = obj_ {
                iret = obj_;
            }
        } 
        else if iret == -99175165f64 {
            if let Ok(fid) = self.get_field_id(name,sig) {
                iret = unchecked_jnice!(self.env.ptr,GetDoubleField, self.ptr, fid)?;
            }
        }
        if iret == -99175165f64 {
            return Err(());
        }

        Ok(iret)
    }
}
impl<'a> From<&JObject<'a>> for JObject<'a> {
    fn from(x: &JObject<'a>) -> Self {
        Self {
            ptr: x.ptr,
            env: x.env,
            is_instance: x.is_instance
        }
    }
}