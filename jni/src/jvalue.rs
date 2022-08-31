use jdk_sys::jvalue;

use crate::object::JObject;

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
}