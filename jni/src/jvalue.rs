use jdk_sys::jvalue;

use crate::{object::JObject, jarray::JArray};

pub enum JValue<'a> {
    JBoolean(bool),
    JByte(i8),
    JChar(char),
    JShort(i16),
    JInt(i32),
    JLong(i64),
    JFloat(f32),
    JDouble(f64),
    JObject(JObject<'a>),
}

impl JValue<'_> {
    pub fn get_c_style(&self) -> jvalue {
        match self {
            JValue::JBoolean(x) => jvalue{z:x.clone() as u8},
            JValue::JByte(x) => jvalue{b:x.clone()},
            JValue::JChar(x) => jvalue{c:x.clone() as u16},
            JValue::JShort(x) => jvalue{s:x.clone()},
            JValue::JInt(x) => jvalue{i:x.clone()},
            JValue::JLong(x) => jvalue{j:x.clone()},
            JValue::JFloat(x) => jvalue{f:x.clone()},
            JValue::JDouble(x) => jvalue{d:x.clone()},
            JValue::JObject(x) => jvalue{l:x.ptr},
        }
    }
    pub fn as_bool(&self) -> Option<&bool> {
        match self {
            JValue::JBoolean(x) => Some(x),
            _ => None,
        }
    }
    pub fn as_byte(&self) -> Option<&i8> {
        match self {
            JValue::JByte(x) => Some(x),
            _ => None,
        }
    }
    pub fn as_char(&self) -> Option<&char> {
        match self {
            JValue::JChar(x) => Some(x),
            _ => None,
        }
    }
    pub fn as_short(&self) -> Option<&i16> {
        match self {
            JValue::JShort(x) => Some(x),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<&i32> {
        match self {
            JValue::JInt(x) => Some(x),
            _ => None,
        }
    }
    pub fn as_long(&self) -> Option<&i64> {
        match self {
            JValue::JLong(x) => Some(x),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<&f32> {
        match self {
            JValue::JFloat(x) => Some(x),
            _ => None,
        }
    }
    pub fn as_double(&self) -> Option<&f64> {
        match self {
            JValue::JDouble(x) => Some(x),
            _ => None,
        }
    }
    pub fn as_object(&self) -> Option<&JObject> {
        match self {
            JValue::JObject(x) => Some(x),
            _ => None,
        }
    }
}

impl<'a,T> From<JArray<'a,T>> for JValue<'a> {
    fn from(obj: JArray<'a,T>) -> Self {
        Self::JObject(obj.ptr)
    }
}


impl<'a> From<JObject<'a>> for JValue<'a> {
    fn from(obj: JObject<'a>) -> Self {
        Self::JObject(obj)
    }
}

impl From<bool> for JValue<'_> {
    fn from(x: bool) -> Self {
        Self::JBoolean(x)
    }
}
impl From<i8> for JValue<'_> {
    fn from(x: i8) -> Self {
        Self::JByte(x)
    }
}
impl From<char> for JValue<'_> {
    fn from(x: char) -> Self {
        Self::JChar(x)
    }
}
impl From<i16> for JValue<'_> {
    fn from(x: i16) -> Self {
        Self::JShort(x)
    }
}
impl From<i32> for JValue<'_> {
    fn from(x: i32) -> Self {
        Self::JInt(x)
    }
}
impl From<i64> for JValue<'_> {
    fn from(x: i64) -> Self {
        Self::JLong(x)
    }
}
impl From<f32> for JValue<'_> {
    fn from(x: f32) -> Self {
        Self::JFloat(x)
    }
}
impl From<f64> for JValue<'_> {
    fn from(x: f64) -> Self {
        Self::JDouble(x)
    }
}