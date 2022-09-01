use std::{ffi::CString, ptr};

use jdk_sys::{jfieldID, jmethodID, jvalue, JNI_TRUE};
use crate::{unchecked_jnic, unchecked_jnice, jvalue::JValue, class::JClass};
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
    pub fn get_class(&self) -> JClass<'a> {
        if self.is_instance {
            JClass::new(unchecked_jnic!(self.env.ptr, GetObjectClass, self.ptr),self.env)
        } else {
            JClass::new(self.ptr,self.env)
        }
    }

    // get field

    
    pub fn _get_obj_field<T: From<JObject<'a>>>(&self,name:&str,sig:&str) -> Result<T,()> {
        let mut obj = ptr::null_mut();
        let fid = self.get_class().get_field_id(name,sig)?;
        obj = unchecked_jnice!(self.env.ptr,GetObjectField, self.ptr, fid)?;
    
        if obj.is_null() {
            return Err(());
        }
        Ok(T::from(JObject::new(obj,self.env,true)))
    }

    fn _get_bool_field(&self,name:&str,sig:&str) -> Result<bool,()> {
        let fid = self.get_class().get_field_id(name,sig)?;
        Ok(unchecked_jnice!(self.env.ptr,GetBooleanField, self.ptr, fid)? == JNI_TRUE as u8)
    }
    fn _get_byte_field(&self,name:&str,sig:&str) -> Result<i8,()> {
        let fid = self.get_class().get_field_id(name,sig)?;
        Ok(unchecked_jnice!(self.env.ptr,GetByteField, self.ptr, fid)? as i8)
    }
    fn _get_char_field(&self,name:&str,sig:&str) -> Result<char,()> {
        let fid = self.get_class().get_field_id(name,sig)?;
        Ok(unchecked_jnice!(self.env.ptr,GetCharField, self.ptr, fid)? as u8 as char)
    }
    fn _get_short_field(&self,name:&str,sig:&str) -> Result<i16,()> {
        let fid = self.get_class().get_field_id(name,sig)?;
        Ok(unchecked_jnice!(self.env.ptr,GetShortField, self.ptr, fid)?)
    }
    fn _get_int_field(&self,name:&str,sig:&str) -> Result<i32,()> {
        let fid = self.get_class().get_field_id(name,sig)?;
        Ok(unchecked_jnice!(self.env.ptr,GetIntField, self.ptr, fid)?)
    }
    fn _get_long_field(&self,name:&str,sig:&str) -> Result<i64,()> {
        let fid = self.get_class().get_field_id(name,sig)?;
        Ok(unchecked_jnice!(self.env.ptr,GetLongField, self.ptr, fid)?)
    }
    fn _get_float_field(&self,name:&str,sig:&str) -> Result<f32,()> {
        let fid = self.get_class().get_field_id(name,sig)?;
        Ok(unchecked_jnice!(self.env.ptr,GetFloatField, self.ptr, fid)?)
    }
    fn _get_double_field(&self,name:&str,sig:&str) -> Result<f64,()> {
        let fid = self.get_class().get_field_id(name,sig)?;
        Ok(unchecked_jnice!(self.env.ptr,GetDoubleField, self.ptr, fid)?)
    }

    // methods


    pub fn call_object_method<T:From<JObject<'a>>>(&self,name:&str,sig:&str,args:Vec<JValue>) -> Result<T,()> {
        let mut obj = ptr::null_mut();
        let args_ = args.iter().map(|f|f.get_c_style()).collect::<Vec<jvalue>>();
        if let Ok(fid) = self.get_class().get_method_id(name,sig) {
            println!("{:?}",fid);
            let obj_ = unchecked_jnice!(self.env.ptr,CallObjectMethodA, self.ptr, fid,args_.as_ptr());
            if let Ok(obj_) = obj_ {
                obj = obj_;
            }
        } 
        else if obj.is_null() {
            return self.get_class().call_static_object_method::<T>(name, sig, args)
        }
        if obj.is_null() {
            return Err(());
        }

        Ok(T::from(JObject::new(obj,self.env,true)))
    }


    // get fields

    pub fn get_field_object<T:From<JObject<'a>>>(&self,name:&str,sig:&str) -> Result<T,()> {
        self._get_obj_field(name, sig).or(self.get_class().get_static_obj_field(name, sig))
    }
    pub fn get_field_bool(&self,name:&str,sig:&str) -> Result<bool,()> {
        self._get_bool_field(name, sig).or(self.get_class().get_static_bool_field(name, sig))
    }
    pub fn get_field_byte(&self,name:&str,sig:&str) -> Result<i8,()> {
        self._get_byte_field(name, sig).or(self.get_class().get_static_byte_field(name, sig))
    }
    pub fn get_field_char(&self,name:&str,sig:&str) -> Result<char,()> {
        self._get_char_field(name, sig).or(self.get_class().get_static_char_field(name, sig))
    }
    pub fn get_field_short(&self,name:&str,sig:&str) -> Result<i16,()> {
        self._get_short_field(name, sig).or(self.get_class().get_static_short_field(name, sig))
    }
    pub fn get_field_int(&self,name:&str,sig:&str) -> Result<i32,()> {
        self._get_int_field(name, sig).or(self.get_class().get_static_int_field(name, sig))
    }
    pub fn get_field_long(&self,name:&str,sig:&str) -> Result<i64,()> {
        self._get_long_field(name, sig).or(self.get_class().get_static_long_field(name, sig))
    }
    pub fn get_field_float(&self,name:&str,sig:&str) -> Result<f32,()> {
        self._get_float_field(name, sig).or(self.get_class().get_static_float_field(name, sig))
    }
    pub fn get_field_double(&self,name:&str,sig:&str) -> Result<f64,()> {
        self._get_double_field(name, sig).or(self.get_class().get_static_double_field(name, sig))
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