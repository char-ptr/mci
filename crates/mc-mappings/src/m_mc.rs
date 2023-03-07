// this file is just a template / example so that i know what i should generate.

use jni::{object::JObject, env::Jenv, jvalue::JValue};
use jni::object::{AbstractJField, AbstractJMethod, AbstractStaticJField, AbstractStaticJMethod, JClassInstance};

pub struct MinecraftClient<'a> {
    inner : JObject<'a>,
}
impl<'a> MinecraftClient<'a> {
    const MAP_SIG: &'static str = "eev";


    pub fn get_instance(&self) -> AbstractJField<Self> {
        AbstractJField::<Self>::new(&self.inner,"E".to_string(), "Leev;".to_string())
    }
    pub fn s_get_instance(env:&'a Jenv<'a>) -> AbstractStaticJField<Self> {
        AbstractStaticJField::<Self>::new(Self::MAP_SIG.to_string(),"E".to_string(), "Leev;".to_string())
    }

    pub fn s_call_getInstance(env:&'a Jenv<'a>,args: Vec<JValue>) -> AbstractStaticJMethod<'a,Self> {
        AbstractStaticJMethod::<Self>::new(Self::MAP_SIG.to_string(),"G".to_string(), "()Leev;".to_string(),args)
    }
    pub fn call_getInstance(&self,args: Vec<JValue>) -> AbstractJMethod<Self> {
        AbstractJMethod::<Self>::new(&self.inner,"G".to_string(), "()Leev;".to_string(),args)
    }

}
impl<'a> JClassInstance for MinecraftClient<'a> {
    fn get_jobject(&self) -> JObject {
        self.inner.clone()
    }
}
impl<'a> From<JObject<'a>> for MinecraftClient<'a> {
    fn from(obj: JObject<'a>) -> Self {
        Self { inner: obj }
    }
}
impl<'a> Into<JObject<'a>> for MinecraftClient<'a> {
    fn into(self) -> JObject<'a> {
        self.inner
    }
}