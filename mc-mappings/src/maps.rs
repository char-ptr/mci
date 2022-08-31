pub mod root {
pub mod minecraft {
pub mod advancement {
pub struct r#Advancement<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for Advancement<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl Advancement<'_> {
const __map_sig : &str = "net/minecraft/class_161";
/* jni::object::JObject = Ljava/util/Map; */
pub fn r#get_criteria(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/Map; */
pub fn r#set_criteria(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/util/Set; */
pub fn r#get_children(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/Set; */
pub fn r#set_children(&self,val : jni::object::JObject) -> () {} 
/* root::minecraft::text::Text = Lnet/minecraft/class_2561; */
pub fn r#get_text(&self) -> Result<root::minecraft::text::Text,()> {
root::minecraft::text::Text::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* root::minecraft::text::Text = Lnet/minecraft/class_2561; */
pub fn r#set_text(&self,val : root::minecraft::text::Text) -> () {} 
/* std::vec::Vec<std::vec::Vec<jni::object::JObject>> = [[Ljava/lang/String; */
pub fn r#get_requirements(&self) -> Result<std::vec::Vec<std::vec::Vec<jni::object::JObject>>,()> {
std::vec::Vec<std::vec::Vec<jni::object::JObject>>::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* std::vec::Vec<std::vec::Vec<jni::object::JObject>> = [[Ljava/lang/String; */
pub fn r#set_requirements(&self,val : std::vec::Vec<std::vec::Vec<jni::object::JObject>>) -> () {} 
/* root::minecraft::advancement::Advancement = Lnet/minecraft/class_161; */
pub fn r#get_parent(&self) -> Result<root::minecraft::advancement::Advancement,()> {
root::minecraft::advancement::Advancement::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* root::minecraft::advancement::Advancement = Lnet/minecraft/class_161; */
pub fn r#set_parent(&self,val : root::minecraft::advancement::Advancement) -> () {} 
/* root::minecraft::util::Identifier = Lnet/minecraft/class_2960; */
pub fn r#get_id(&self) -> Result<root::minecraft::util::Identifier,()> {
root::minecraft::util::Identifier::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* root::minecraft::util::Identifier = Lnet/minecraft/class_2960; */
pub fn r#set_id(&self,val : root::minecraft::util::Identifier) -> () {} 
/* root::minecraft::advancement::AdvancementRewards = Lnet/minecraft/class_170; */
pub fn r#get_rewards(&self) -> Result<root::minecraft::advancement::AdvancementRewards,()> {
root::minecraft::advancement::AdvancementRewards::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* root::minecraft::advancement::AdvancementRewards = Lnet/minecraft/class_170; */
pub fn r#set_rewards(&self,val : root::minecraft::advancement::AdvancementRewards) -> () {} 
/* root::minecraft::advancement::AdvancementDisplay = Lnet/minecraft/class_185; */
pub fn r#get_display(&self) -> Result<root::minecraft::advancement::AdvancementDisplay,()> {
root::minecraft::advancement::AdvancementDisplay::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* root::minecraft::advancement::AdvancementDisplay = Lnet/minecraft/class_185; */
pub fn r#set_display(&self,val : root::minecraft::advancement::AdvancementDisplay) -> () {} 
pub fn r#__zINDMINIT(r#id : root::minecraft::util::Identifier,r#parent : root::minecraft::advancement::Advancement,r#display : root::minecraft::advancement::AdvancementDisplay,r#rewards : root::minecraft::advancement::AdvancementRewards,r#criteria : jni::object::JObject,r#requirements : jni::object::JObject,) -> () { }
pub fn r#unknwnfn_60287195(r#o : jni::object::JObject,) -> bool { }
pub fn r#getRequirements() -> jni::object::JObject { }
pub fn r#getChildren() -> jni::object::JObject { }
pub fn r#getCriteria() -> jni::object::JObject { }
pub fn r#getRequirementCount() -> i32 { }
pub fn r#toHoverableText() -> root::minecraft::text::Text { }
pub fn r#unknwnfn_218005489(r#style : root::minecraft::text::Text,r#unknwnarg_1494978775 : root::minecraft::text::Style,) -> root::minecraft::text::Style { }
pub fn r#getDisplay() -> root::minecraft::advancement::AdvancementDisplay { }
pub fn r#getParent() -> root::minecraft::advancement::Advancement { }
pub fn r#getId() -> root::minecraft::util::Identifier { }
pub fn r#createTask() -> root::minecraft::advancement::Builder { }
pub fn r#addChild(r#child : root::minecraft::advancement::Advancement,) -> () { }
}
