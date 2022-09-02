use std::{ffi::CString, ptr, sync::Arc};

use jdk_sys::{jfieldID, jmethodID, jvalue, JNI_TRUE};
use crate::{unchecked_jnic, unchecked_jnice, jvalue::JValue, class::JClass};
use super::env::Jenv;

#[derive(Debug, Clone)]
pub struct JObject<'a> {
    pub ptr : jdk_sys::jobject,
    pub env : &'a Jenv<'a>,
    class : Arc<JClass<'a>>,
}


impl<'a> JObject<'a> {
    pub fn new(ptr : jdk_sys::jobject,env : &'a Jenv) -> Self {
        let class = Arc::new(JClass::new(unchecked_jnic!(env.ptr,GetObjectClass, ptr),env));
        JObject {
            ptr,
            env,
            class
        }
    }
    pub fn get_class(&self) -> Arc<JClass<'a>> {
        Arc::clone(&self.class)
    }

    // get field

    
    pub fn _get_object_field<T: From<JObject<'a>>>(&self,name:&str,sig:&str) -> Result<T,()> {
        let mut obj = ptr::null_mut();
        let fid = self.get_class().get_field_id(name,sig)?;
        obj = unchecked_jnice!(self.env.ptr,GetObjectField, self.ptr, fid)?;
    
        if obj.is_null() {
            return Err(());
        }
        Ok(T::from(JObject::new(obj,self.env)))
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

    // set

    pub fn _set_object_field(&self,name:&str,sig:&str,new_value:&'a JObject<'a>) -> Result<(),()> {
        let fid = self.get_class().get_field_id(name,sig)?;
        unchecked_jnice!(self.env.ptr,SetObjectField, self.ptr, fid,new_value.ptr)
    }

    pub fn _set_bool_field(&self,name:&str,sig:&str, new_value:bool) -> Result<(),()> {
        let fid = self.get_class().get_field_id(name,sig)?;
        unchecked_jnice!(self.env.ptr,SetBooleanField, self.ptr, fid,new_value as u8)
    }
    pub fn _set_byte_field(&self,name:&str,sig:&str,new_value:i8) -> Result<(),()> {
        let fid = self.get_class().get_field_id(name,sig)?;
        unchecked_jnice!(self.env.ptr,SetByteField, self.ptr, fid,new_value as i8)
    }
    pub fn _set_char_field(&self,name:&str,sig:&str,new_value:char) -> Result<(),()> {
        let fid = self.get_class().get_field_id(name,sig)?;
        unchecked_jnice!(self.env.ptr,SetCharField, self.ptr, fid, new_value as u16)
    }
    pub fn _set_short_field(&self,name:&str,sig:&str,new_value:i16) -> Result<(),()> {
        let fid = self.get_class().get_field_id(name,sig)?;
        unchecked_jnice!(self.env.ptr,SetShortField, self.ptr, fid,new_value)
    }
    pub fn _set_int_field(&self,name:&str,sig:&str,new_value:i32) -> Result<(),()> {
        let fid = self.get_class().get_field_id(name,sig)?;
        unchecked_jnice!(self.env.ptr,SetIntField, self.ptr, fid,new_value)
    }
    pub fn _set_long_field(&self,name:&str,sig:&str,new_value:i64) -> Result<(),()> {
        let fid = self.get_class().get_field_id(name,sig)?;
        unchecked_jnice!(self.env.ptr,SetLongField, self.ptr, fid,new_value)
    }
    pub fn _set_float_field(&self,name:&str,sig:&str,new_value:f32) -> Result<(),()> {
        let fid = self.get_class().get_field_id(name,sig)?;
        unchecked_jnice!(self.env.ptr,SetFloatField, self.ptr, fid,new_value)
    }
    pub fn _set_double_field(&self,name:&str,sig:&str,new_value:f64) -> Result<(),()> {
        let fid = self.get_class().get_field_id(name,sig)?;
        unchecked_jnice!(self.env.ptr,SetDoubleField, self.ptr, fid,new_value)
    }

    // methods


    fn _call_object_method<T:From<JObject<'a>>>(&self,name:&str,sig:&str,args:&Vec<JValue>) -> Result<T,()> {
        let mut obj = ptr::null_mut();
        let args = args.iter().map(|f|f.get_c_style()).collect::<Vec<jvalue>>();

        let mid=  self.get_class().get_method_id(name,sig)?;
        obj = unchecked_jnice!(self.env.ptr,CallObjectMethodA, self.ptr, mid,args.as_ptr())?;
        
        if obj.is_null() {
            return Err(());
        }

        Ok(T::from(JObject::new(obj,self.env)))
    }
    fn _call_bool_method(&self,name:&str,sig:&str,args:&Vec<JValue>) -> Result<bool,()> {
        let args = args.iter().map(|f|f.get_c_style()).collect::<Vec<jvalue>>();
        let mid=  self.get_class().get_method_id(name,sig)?;

        Ok(unchecked_jnice!(self.env.ptr,CallBooleanMethodA, self.ptr, mid,args.as_ptr() )? == JNI_TRUE as u8)
    }
    fn _call_byte_method(&self,name:&str,sig:&str,args:&Vec<JValue>) -> Result<i8,()> {
        let args = args.iter().map(|f|f.get_c_style()).collect::<Vec<jvalue>>();
        let mid=  self.get_class().get_method_id(name,sig)?;

        unchecked_jnice!(self.env.ptr,CallByteMethodA, self.ptr, mid,args.as_ptr() )
    }
    fn _call_char_method(&self,name:&str,sig:&str,args:&Vec<JValue>) -> Result<char,()> {
        let args = args.iter().map(|f|f.get_c_style()).collect::<Vec<jvalue>>();
        let mid=  self.get_class().get_method_id(name,sig)?;

        Ok(unchecked_jnice!(self.env.ptr,CallCharMethodA, self.ptr, mid,args.as_ptr())? as u8 as char)
    }
    fn _call_short_method(&self,name:&str,sig:&str,args:&Vec<JValue>) -> Result<i16,()> {
        let args = args.iter().map(|f|f.get_c_style()).collect::<Vec<jvalue>>();
        let mid=  self.get_class().get_method_id(name,sig)?;

        unchecked_jnice!(self.env.ptr,CallShortMethodA, self.ptr, mid,args.as_ptr() )
    }
    fn _call_int_method(&self,name:&str,sig:&str,args:&Vec<JValue>) -> Result<i32,()> {
        let args = args.iter().map(|f|f.get_c_style()).collect::<Vec<jvalue>>();
        let mid=  self.get_class().get_method_id(name,sig)?;

        unchecked_jnice!(self.env.ptr,CallIntMethodA, self.ptr, mid,args.as_ptr() )
    }
    fn _call_long_method(&self,name:&str,sig:&str,args:&Vec<JValue>) -> Result<i64,()> {
        let args = args.iter().map(|f|f.get_c_style()).collect::<Vec<jvalue>>();
        let mid=  self.get_class().get_method_id(name,sig)?;

        unchecked_jnice!(self.env.ptr,CallLongMethodA, self.ptr, mid,args.as_ptr() )
    }
    fn _call_float_method(&self,name:&str,sig:&str,args:&Vec<JValue>) -> Result<f32,()> {
        let args = args.iter().map(|f|f.get_c_style()).collect::<Vec<jvalue>>();
        let mid=  self.get_class().get_method_id(name,sig)?;

        unchecked_jnice!(self.env.ptr,CallFloatMethodA, self.ptr, mid,args.as_ptr() )
    }
    fn _call_double_method(&self,name:&str,sig:&str,args:&Vec<JValue>) -> Result<f64,()> {
        let args = args.iter().map(|f|f.get_c_style()).collect::<Vec<jvalue>>();
        let mid=  self.get_class().get_method_id(name,sig)?;

        unchecked_jnice!(self.env.ptr,CallDoubleMethodA, self.ptr, mid,args.as_ptr() )
    }

    // get fields

    pub fn get_field_object<T:From<JObject<'a>>>(&self,name:&str,sig:&str) -> Result<T,()> {
        self._get_object_field(name, sig).or(self.get_class().get_static_object_field(name, sig))
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

        // set fields

    pub fn set_field_object(&self,name:&str,sig:&str,new_value:&'a JObject<'a>) -> Result<(),()> {
        self._set_object_field(name, sig, new_value).or(self.get_class().set_static_object_field(name, sig, new_value))
    }
    pub fn set_field_bool(&self,name:&str,sig:&str,new_value:bool) -> Result<(),()> {
        self._set_bool_field(name, sig, new_value).or(self.get_class().set_static_bool_field(name, sig, new_value))
    }
    pub fn set_field_byte(&self,name:&str,sig:&str,new_value:i8) -> Result<(),()> {
        self._set_byte_field(name, sig, new_value).or(self.get_class().set_static_byte_field(name, sig, new_value))
    }
    pub fn set_field_char(&self,name:&str,sig:&str,new_value:char) -> Result<(),()> {
        self._set_char_field(name, sig, new_value).or(self.get_class().set_static_char_field(name, sig, new_value))
    }
    pub fn set_field_short(&self,name:&str,sig:&str,new_value:i16) -> Result<(),()> {
        self._set_short_field(name, sig, new_value).or(self.get_class().set_static_short_field(name, sig, new_value))
    }
    pub fn set_field_int(&self,name:&str,sig:&str,new_value:i32) -> Result<(),()> {
        self._set_int_field(name, sig, new_value).or(self.get_class().set_static_int_field(name, sig, new_value))
    }
    pub fn set_field_long(&self,name:&str,sig:&str,new_value:i64) -> Result<(),()> {
        self._set_long_field(name, sig, new_value).or(self.get_class().set_static_long_field(name, sig, new_value))
    }
    pub fn set_field_float(&self,name:&str,sig:&str,new_value:f32) -> Result<(),()> {
        self._set_float_field(name, sig, new_value).or(self.get_class().set_static_float_field(name, sig, new_value))
    }
    pub fn set_field_double(&self,name:&str,sig:&str,new_value:f64) -> Result<(),()> {
        self._set_double_field(name, sig, new_value).or(self.get_class().set_static_double_field(name, sig, new_value))
    }

    // methods

    pub fn call_object_method<T:From<JObject<'a>>>(&self,name:&str,sig:&str,args:&Vec<JValue>) -> Result<T,()> {
        self._call_object_method(name, sig, args).or(self.get_class().call_static_object_method(name, sig, args))
    }
    pub fn call_bool_method(&self,name:&str,sig:&str,args:&Vec<JValue>) -> Result<bool,()> {
        self._call_bool_method(name, sig, args).or(self.get_class().call_static_bool_method(name, sig, args))
    }
    pub fn call_char_method(&self,name:&str,sig:&str,args:&Vec<JValue>) -> Result<char,()> {
        self._call_char_method(name, sig, args).or(self.get_class().call_static_char_method(name, sig, args))
    }
    pub fn call_short_method(&self,name:&str,sig:&str,args:&Vec<JValue>) -> Result<i16,()> {
        self._call_short_method(name, sig, args).or(self.get_class().call_static_short_method(name, sig, args))
    }
    pub fn call_int_method(&self,name:&str,sig:&str,args:&Vec<JValue>) -> Result<i32,()> {
        self._call_int_method(name, sig, args).or(self.get_class().call_static_int_method(name, sig, args))
    }
    pub fn call_long_method(&self,name:&str,sig:&str,args:&Vec<JValue>) -> Result<i64,()> {
        self._call_long_method(name, sig, args).or(self.get_class().call_static_long_method(name, sig, args))
    }
    pub fn call_float_method(&self,name:&str,sig:&str,args:&Vec<JValue>) -> Result<f32,()> {
        self._call_float_method(name, sig, args).or(self.get_class().call_static_float_method(name, sig, args))
    }
    pub fn call_double_method(&self,name:&str,sig:&str,args:&Vec<JValue>) -> Result<f64,()> {
        self._call_double_method(name, sig, args).or(self.get_class().call_static_double_method(name, sig, args))
    }
    

}
impl<'a> From<&JObject<'a>> for JObject<'a> {
    fn from(x: &JObject<'a>) -> Self {
        Self::new(x.ptr, x.env)
    }
}