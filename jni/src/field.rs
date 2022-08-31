use jdk_sys::jfieldID;

use crate::unchecked_jnic;

use super::{object::JObject, env::Jenv};


pub enum JFieldTypes {
    Object,
    Boolean,
    Byte,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
}
pub enum JFieldTypesRet<'a> {
    Object(JObject<'a>),
    Boolean(bool),
    Byte(i8),
    Char(char),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
}
impl<'a> JFieldTypes {
    pub fn get_field(&self,env : &'a Jenv<'a>, obj : &JObject, field: jfieldID) -> JFieldTypesRet<'a> {
        match self {
            JFieldTypes::Object => JFieldTypesRet::Object(JObject::new(unchecked_jnic!(env.ptr,GetObjectField, obj.ptr, field),env)),
            JFieldTypes::Boolean => JFieldTypesRet::Boolean(unchecked_jnic!(env.ptr,GetBooleanField, obj.ptr, field) == 1),
            JFieldTypes::Byte => JFieldTypesRet::Byte(unchecked_jnic!(env.ptr,GetByteField, obj.ptr, field)),
            JFieldTypes::Char => JFieldTypesRet::Char(unchecked_jnic!(env.ptr,GetCharField, obj.ptr, field) as u8 as char),
            JFieldTypes::Short => JFieldTypesRet::Short(unchecked_jnic!(env.ptr,GetShortField, obj.ptr, field)),
            JFieldTypes::Int => JFieldTypesRet::Int(unchecked_jnic!(env.ptr,GetIntField, obj.ptr, field)),
            JFieldTypes::Long => JFieldTypesRet::Long(unchecked_jnic!(env.ptr,GetLongField, obj.ptr, field)),
            JFieldTypes::Float => JFieldTypesRet::Float(unchecked_jnic!(env.ptr,GetFloatField, obj.ptr, field)),
            JFieldTypes::Double => JFieldTypesRet::Double(unchecked_jnic!(env.ptr,GetDoubleField, obj.ptr, field)),
        }
    }
    pub fn get_static_field(&self,env : &'a Jenv<'a>, obj : &JObject, field: jfieldID) -> JFieldTypesRet<'a> {
        match self {
            JFieldTypes::Object => JFieldTypesRet::Object(JObject::new(unchecked_jnic!(env.ptr,GetStaticObjectField, obj.ptr, field),env)),
            JFieldTypes::Boolean => JFieldTypesRet::Boolean(unchecked_jnic!(env.ptr,GetStaticBooleanField, obj.ptr, field) == 1),
            JFieldTypes::Byte => JFieldTypesRet::Byte(unchecked_jnic!(env.ptr,GetStaticByteField, obj.ptr, field)),
            JFieldTypes::Char => JFieldTypesRet::Char(unchecked_jnic!(env.ptr,GetStaticCharField, obj.ptr, field) as u8 as char),
            JFieldTypes::Short => JFieldTypesRet::Short(unchecked_jnic!(env.ptr,GetStaticShortField, obj.ptr, field)),
            JFieldTypes::Int => JFieldTypesRet::Int(unchecked_jnic!(env.ptr,GetStaticIntField, obj.ptr, field)),
            JFieldTypes::Long => JFieldTypesRet::Long(unchecked_jnic!(env.ptr,GetStaticLongField, obj.ptr, field)),
            JFieldTypes::Float => JFieldTypesRet::Float(unchecked_jnic!(env.ptr,GetStaticFloatField, obj.ptr, field)),
            JFieldTypes::Double => JFieldTypesRet::Double(unchecked_jnic!(env.ptr,GetStaticDoubleField, obj.ptr, field)),
        }
    }
}
impl JFieldTypesRet<'_> {
    pub fn set_field(&self,env : &Jenv, obj : &JObject, field: jfieldID) {
        match self {
            JFieldTypesRet::Object(v) => unchecked_jnic!(env.ptr,SetObjectField, obj.ptr, field,v.ptr),
            JFieldTypesRet::Boolean(v) => unchecked_jnic!(env.ptr,SetBooleanField, obj.ptr, field,*v as u8),
            JFieldTypesRet::Byte(v) => unchecked_jnic!(env.ptr,SetByteField, obj.ptr, field,*v),
            JFieldTypesRet::Char(v) => unchecked_jnic!(env.ptr,SetCharField, obj.ptr, field,*v as u16) ,
            JFieldTypesRet::Short(v) => unchecked_jnic!(env.ptr,SetShortField, obj.ptr, field,*v),
            JFieldTypesRet::Int(v) => unchecked_jnic!(env.ptr,SetIntField, obj.ptr, field,*v),
            JFieldTypesRet::Long(v) => unchecked_jnic!(env.ptr,SetLongField, obj.ptr, field,*v),
            JFieldTypesRet::Float(v) => unchecked_jnic!(env.ptr,SetFloatField, obj.ptr, field,*v),
            JFieldTypesRet::Double(v) => unchecked_jnic!(env.ptr,SetDoubleField, obj.ptr, field,*v),
        };
    }
    pub fn set_static_field(&self,env : &Jenv, obj : &JObject, field: jfieldID) {
        match self {
            JFieldTypesRet::Object(v) => unchecked_jnic!(env.ptr,SetStaticObjectField, obj.ptr, field,v.ptr),
            JFieldTypesRet::Boolean(v) => unchecked_jnic!(env.ptr,SetStaticBooleanField, obj.ptr, field,*v as u8),
            JFieldTypesRet::Byte(v) => unchecked_jnic!(env.ptr,SetStaticByteField, obj.ptr, field,*v),
            JFieldTypesRet::Char(v) => unchecked_jnic!(env.ptr,SetStaticCharField, obj.ptr, field,*v as u16) ,
            JFieldTypesRet::Short(v) => unchecked_jnic!(env.ptr,SetStaticShortField, obj.ptr, field,*v),
            JFieldTypesRet::Int(v) => unchecked_jnic!(env.ptr,SetStaticIntField, obj.ptr, field,*v),
            JFieldTypesRet::Long(v) => unchecked_jnic!(env.ptr,SetStaticLongField, obj.ptr, field,*v),
            JFieldTypesRet::Float(v) => unchecked_jnic!(env.ptr,SetStaticFloatField, obj.ptr, field,*v),
            JFieldTypesRet::Double(v) => unchecked_jnic!(env.ptr,SetStaticDoubleField, obj.ptr, field,*v),
        };
    }
}
