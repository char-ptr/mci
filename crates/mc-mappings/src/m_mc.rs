// this file is just a template / example so that i know what i should generate.

use jni::{object::JObject, env::Jenv, jvalue::JValue};

struct MinecraftClient<'a> {
    inner : JObject<'a>
}
impl<'a> MinecraftClient<'a> {
    const map_sig: &str = "eev";


    fn get_instance(&self) -> Result<Self,()> {
        self.inner.get_field_object::<Self>("E", "Leev;")
    }
    fn s_get_instance(env:&'a Jenv<'a>) -> Result<Self,()> {
        let class = env.find_class(Self::map_sig)?;
        class.get_static_object_field::<Self>("E", "Leev;")
    }

    fn s_call_getInstance(env:&'a Jenv<'a>,args: Vec<JValue>) -> Result<Self,()> {
        let class = env.find_class(Self::map_sig)?;
        class.call_static_object_method::<Self>("G", "()Leev;",&args)
    }
    fn call_getInstance(&self,args: Vec<JValue>) -> Result<Self,()> {
        self.inner.call_object_method::<Self>("G", "()Leev;",&args)
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