use std::{ops::{Index, Deref, DerefMut}, default, marker::PhantomData};

use crate::{unchecked_jnic, object::JObject, jvalue::JValue};
use super::env::Jenv;
pub struct JArray<'a,T> {
    pub ptr : JObject<'a>,
    pub length : usize,
    phantom_type : PhantomData<[T]>
}

impl<'a,T> JArray<'a,T> {
    pub fn new(obj:JObject<'a>) -> JArray<'a,T> {
        let length = unchecked_jnic!(obj.env.ptr,GetArrayLength, obj.ptr) as usize;

        JArray {
            ptr : obj,
            length,
            phantom_type : PhantomData
        }
    }
    pub fn len(&self) -> usize {
        self.length
    }
}

impl<'a,T> From<JObject<'a>> for JArray<'a,T> {
    fn from(obj : JObject<'a>) -> Self {
        Self::new(obj)
    }
}
impl<'a> JArray<'a,i32> {
    pub fn get_all(&self) -> Vec<i32> {
        unsafe {
            Vec::from_raw_parts(unchecked_jnic!(self.ptr.env.ptr,GetIntArrayElements, self.ptr.ptr, std::ptr::null_mut()), self.length, self.length)
        }
    }
}