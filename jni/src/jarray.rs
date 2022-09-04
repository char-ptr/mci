use std::{ops::{Index, Deref, DerefMut}, default};

use crate::{unchecked_jnic, object::JObject, jvalue::JValue};
use super::env::Jenv;
pub struct JArray<'a> {
    pub ptr : JObject<'a>,
    pub length : usize,
    pub inner:  Vec<JValue<'a>>,
}

impl<'a,T: From<JObject<'a>>> From<JObject<'a>> for JArray<'a> {
    fn from(obj : JObject<'a>) -> Self {
        let length = unchecked_jnic!(obj.env.ptr,GetArrayLength, obj.ptr) as usize;

        JArray {
            ptr : obj,
            length,
            inner : Vec::with_capacity(length),
        }
    }

}
impl<'a> Deref for JArray<'a> {
    type Target = Vec<JValue<'a>>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a,T> DerefMut for JArray<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}


impl<'a> JArray<'a> {
    fn len(&self) -> usize {
        self.length
    }

}