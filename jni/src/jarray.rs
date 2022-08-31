use std::{ops::{Index, Deref, DerefMut}, default};

use crate::{unchecked_jnic, object::JObject, field::JFieldTypes};
use super::env::Jenv;
pub struct JArray<'a,T> {
    pub ptr : JObject<'a>,
    pub length : usize,
    pub inner:  Vec<T>,
}

impl<'a,T: From<JObject<'a>>> From<JObject<'a>> for JArray<'a,T> {
    fn from(obj : JObject<'a>) -> Self {
        let length = unchecked_jnic!(obj.env.ptr,GetArrayLength, obj.ptr) as usize;

        JArray {
            ptr : obj,
            length,
            inner : Vec::with_capacity(length),
        }
    }

}
impl<'a,T> Deref for JArray<'a,T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a,T> DerefMut for JArray<'a,T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}


impl<'a,T> JArray<'a,T> {
    fn len(&self) -> usize {
        self.length
    }

}