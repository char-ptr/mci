use std::{borrow::Cow, fmt::{Formatter, Display, Debug}, ops::{Deref, DerefMut}, ffi::CString};

use crate::{object::JObject, unchecked_jnic, jvalue::JValue, env::Jenv};

pub struct JString<'a,> {
    pub obj : JObject<'a>,
}
impl<'a> JString<'a> {
    pub fn new_from_obj(obj:JObject<'a>) -> JString<'a> {
        JString {
            obj,
        }
    }
    pub fn new(string_ref : &str, env : &'a Jenv) -> Self {
        let mut added_nul = string_ref.to_string();
        added_nul.push('\0');
        let ptr = unchecked_jnic!(env.ptr,NewStringUTF, added_nul.as_ptr() as *const i8);
        Self {
            obj : JObject::new(ptr, env),
        }
    }

    pub fn length(&self) -> usize {
        unchecked_jnic!(self.obj.env.ptr,GetStringUTFLength, self.obj.ptr) as usize
    }
    pub fn to_string(&self) -> ReturnedJString {
        let chr_ptr = unchecked_jnic!(self.obj.env.ptr,GetStringUTFChars, self.obj.ptr, &mut 0u8 as *mut u8);
        ReturnedJString {
            ptr: chr_ptr,
            inner: String::from_utf8_lossy(unsafe { std::slice::from_raw_parts(chr_ptr as *const u8, self.length()) }),
            from: self
        }
    }
    /// Will only error when `start` + `length` is greater than the [`Self::length`] of the string
    pub fn get_substring(&self, start:usize, length:usize) -> Result<String,()> {
        if start + length > self.length() {
            return Err(());
        }

        let mut buf = vec![1u8; length + 1];
        buf.push('\0' as u8);
        let mut cstr = CString::from_vec_with_nul(buf).unwrap();
        let chr_ptr = cstr.as_ptr() as *mut i8;
        unchecked_jnic!(self.obj.env.ptr,GetStringUTFRegion, self.obj.ptr, start as i32, length as i32, chr_ptr);
        Ok(cstr.to_string_lossy().to_string())
    } 
}
impl Display for JString<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.to_string().inner, f)
    }
}
impl Debug for JString<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.to_string().inner, f)
    }
}
impl<'a> From<JObject<'a>> for JString<'a> {
    fn from(obj : JObject<'a>) -> Self {
        Self::new_from_obj(obj)
    }
}

pub struct ReturnedJString<'a> {
    ptr: *const i8,
    pub inner : Cow<'a,str>,
    from: &'a JString<'a>,
} 
impl Drop for ReturnedJString<'_> {
    fn drop(&mut self) {
        unchecked_jnic!(self.from.obj.env.ptr,ReleaseStringUTFChars, self.from.obj.ptr, self.ptr);
    }
}
impl Deref for ReturnedJString<'_> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Display for ReturnedJString<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.inner, f)
    }
}
impl Debug for ReturnedJString<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.inner, f)
    }
}