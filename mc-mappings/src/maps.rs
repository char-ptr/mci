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
/* crate::root::minecraft::text::Text = Lnet/minecraft/class_2561; */
pub fn r#get_text(&self) -> Result<crate::root::minecraft::text::Text,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::text::Text = Lnet/minecraft/class_2561; */
pub fn r#set_text(&self,val : crate::root::minecraft::text::Text) -> () {} 
/* std::vec::Vec<std::vec::Vec<jni::object::JObject>> = [[Ljava/lang/String; */
pub fn r#get_requirements(&self) -> Result<std::vec::Vec<std::vec::Vec<jni::object::JObject>>,()> {
std::vec::Vec<std::vec::Vec<jni::object::JObject>>::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* std::vec::Vec<std::vec::Vec<jni::object::JObject>> = [[Ljava/lang/String; */
pub fn r#set_requirements(&self,val : std::vec::Vec<std::vec::Vec<jni::object::JObject>>) -> () {} 
/* crate::root::minecraft::advancement::Advancement = Lnet/minecraft/class_161; */
pub fn r#get_parent(&self) -> Result<crate::root::minecraft::advancement::Advancement,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::advancement::Advancement = Lnet/minecraft/class_161; */
pub fn r#set_parent(&self,val : crate::root::minecraft::advancement::Advancement) -> () {} 
/* crate::root::minecraft::util::Identifier = Lnet/minecraft/class_2960; */
pub fn r#get_id(&self) -> Result<crate::root::minecraft::util::Identifier,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::util::Identifier = Lnet/minecraft/class_2960; */
pub fn r#set_id(&self,val : crate::root::minecraft::util::Identifier) -> () {} 
/* crate::root::minecraft::advancement::AdvancementRewards = Lnet/minecraft/class_170; */
pub fn r#get_rewards(&self) -> Result<crate::root::minecraft::advancement::AdvancementRewards,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::advancement::AdvancementRewards = Lnet/minecraft/class_170; */
pub fn r#set_rewards(&self,val : crate::root::minecraft::advancement::AdvancementRewards) -> () {} 
/* crate::root::minecraft::advancement::AdvancementDisplay = Lnet/minecraft/class_185; */
pub fn r#get_display(&self) -> Result<crate::root::minecraft::advancement::AdvancementDisplay,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::advancement::AdvancementDisplay = Lnet/minecraft/class_185; */
pub fn r#set_display(&self,val : crate::root::minecraft::advancement::AdvancementDisplay) -> () {} 
pub fn r#__zINDMINIT(r#id : crate::root::minecraft::util::Identifier,r#parent : crate::root::minecraft::advancement::Advancement,r#display : crate::root::minecraft::advancement::AdvancementDisplay,r#rewards : crate::root::minecraft::advancement::AdvancementRewards,r#criteria : jni::object::JObject,r#requirements : jni::object::JObject,) -> () { }
pub fn r#unknwnfn_4275627281(r#o : jni::object::JObject,) -> bool { }
pub fn r#getRequirements() -> jni::object::JObject { }
pub fn r#getChildren() -> jni::object::JObject { }
pub fn r#getCriteria() -> jni::object::JObject { }
pub fn r#getRequirementCount() -> i32 { }
pub fn r#toHoverableText() -> crate::root::minecraft::text::Text { }
pub fn r#unknwnfn_718570301(r#style : crate::root::minecraft::text::Text,r#unknwnarg_47908815 : crate::root::minecraft::text::Style,) -> crate::root::minecraft::text::Style { }
pub fn r#getDisplay() -> crate::root::minecraft::advancement::AdvancementDisplay { }
pub fn r#getParent() -> crate::root::minecraft::advancement::Advancement { }
pub fn r#getId() -> crate::root::minecraft::util::Identifier { }
pub fn r#createTask() -> crate::root::minecraft::advancement::Builder { }
pub fn r#addChild(r#child : crate::root::minecraft::advancement::Advancement,) -> () { }
}
}
pub mod block {
pub struct r#AbstractBlock<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for AbstractBlock<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl AbstractBlock<'_> {
const __map_sig : &str = "net/minecraft/class_4970";
/* bool = Z */
pub fn r#get_dynamicBounds(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_dynamicBounds(&self,val : bool) -> () {} 
/* jni::object::JObject = Lnet/minecraft/class_4970$class_2251; */
pub fn r#get_settings(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lnet/minecraft/class_4970$class_2251; */
pub fn r#set_settings(&self,val : jni::object::JObject) -> () {} 
/* crate::root::minecraft::util::Identifier = Lnet/minecraft/class_2960; */
pub fn r#get_lootTableId(&self) -> Result<crate::root::minecraft::util::Identifier,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::util::Identifier = Lnet/minecraft/class_2960; */
pub fn r#set_lootTableId(&self,val : crate::root::minecraft::util::Identifier) -> () {} 
/* std::vec::Vec<crate::root::minecraft::util::math::Direction> = [Lnet/minecraft/class_2350; */
pub fn r#get_DIRECTIONS(&self) -> Result<std::vec::Vec<crate::root::minecraft::util::math::Direction>,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* std::vec::Vec<crate::root::minecraft::util::math::Direction> = [Lnet/minecraft/class_2350; */
pub fn r#set_DIRECTIONS(&self,val : std::vec::Vec<crate::root::minecraft::util::math::Direction>) -> () {} 
/* crate::root::minecraft::block::Material = Lnet/minecraft/class_3614; */
pub fn r#get_material(&self) -> Result<crate::root::minecraft::block::Material,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::block::Material = Lnet/minecraft/class_3614; */
pub fn r#set_material(&self,val : crate::root::minecraft::block::Material) -> () {} 
/* bool = Z */
pub fn r#get_collidable(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_collidable(&self,val : bool) -> () {} 
/* f32 = F */
pub fn r#get_resistance(&self) -> Result<f32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f32 = F */
pub fn r#set_resistance(&self,val : f32) -> () {} 
/* bool = Z */
pub fn r#get_randomTicks(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_randomTicks(&self,val : bool) -> () {} 
/* crate::root::minecraft::sound::BlockSoundGroup = Lnet/minecraft/class_2498; */
pub fn r#get_soundGroup(&self) -> Result<crate::root::minecraft::sound::BlockSoundGroup,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::sound::BlockSoundGroup = Lnet/minecraft/class_2498; */
pub fn r#set_soundGroup(&self,val : crate::root::minecraft::sound::BlockSoundGroup) -> () {} 
/* f32 = F */
pub fn r#get_slipperiness(&self) -> Result<f32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f32 = F */
pub fn r#set_slipperiness(&self,val : f32) -> () {} 
/* f32 = F */
pub fn r#get_velocityMultiplier(&self) -> Result<f32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f32 = F */
pub fn r#set_velocityMultiplier(&self,val : f32) -> () {} 
/* f32 = F */
pub fn r#get_jumpVelocityMultiplier(&self) -> Result<f32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f32 = F */
pub fn r#set_jumpVelocityMultiplier(&self,val : f32) -> () {} 
pub fn r#__zINDMINIT(r#settings : jni::object::JObject,) -> () { }
pub fn r#createScreenHandlerFactory(r#state : crate::root::minecraft::block::BlockState,r#world : crate::root::minecraft::world::World,r#pos : crate::root::minecraft::util::math::BlockPos,) -> crate::root::minecraft::screen::NamedScreenHandlerFactory { }
pub fn r#onProjectileHit(r#world : crate::root::minecraft::world::World,r#state : crate::root::minecraft::block::BlockState,r#hit : crate::root::minecraft::util::hit::BlockHitResult,r#projectile : crate::root::minecraft::entity::projectile::ProjectileEntity,) -> () { }
pub fn r#canBucketPlace(r#state : crate::root::minecraft::block::BlockState,r#fluid : crate::root::minecraft::fluid::Fluid,) -> bool { }
pub fn r#getSidesShape(r#state : crate::root::minecraft::block::BlockState,r#world : crate::root::minecraft::world::BlockView,r#pos : crate::root::minecraft::util::math::BlockPos,) -> crate::root::minecraft::util::shape::VoxelShape { }
pub fn r#getCameraCollisionShape(r#state : crate::root::minecraft::block::BlockState,r#world : crate::root::minecraft::world::BlockView,r#pos : crate::root::minecraft::util::math::BlockPos,r#context : crate::root::minecraft::block::ShapeContext,) -> crate::root::minecraft::util::shape::VoxelShape { }
pub fn r#asBlock() -> crate::root::minecraft::block::Block { }
pub fn r#getLootTableId() -> crate::root::minecraft::util::Identifier { }
pub fn r#getDefaultMapColor() -> crate::root::minecraft::block::MapColor { }
pub fn r#getMaxHorizontalModelOffset() -> f32 { }
pub fn r#getHardness() -> f32 { }
pub fn r#getVerticalModelOffsetMultiplier() -> f32 { }
pub fn r#isShapeFullCube(r#state : crate::root::minecraft::block::BlockState,r#world : crate::root::minecraft::world::BlockView,r#pos : crate::root::minecraft::util::math::BlockPos,) -> bool { }
pub fn r#isCullingShapeFullCube(r#state : crate::root::minecraft::block::BlockState,r#world : crate::root::minecraft::world::BlockView,r#pos : crate::root::minecraft::util::math::BlockPos,) -> bool { }
pub fn r#asItem() -> crate::root::minecraft::item::Item { }
pub fn r#hasComparatorOutput(r#state : crate::root::minecraft::block::BlockState,) -> bool { }
pub fn r#getOpacity(r#state : crate::root::minecraft::block::BlockState,r#world : crate::root::minecraft::world::BlockView,r#pos : crate::root::minecraft::util::math::BlockPos,) -> i32 { }
pub fn r#emitsRedstonePower(r#state : crate::root::minecraft::block::BlockState,) -> bool { }
pub fn r#randomTick(r#state : crate::root::minecraft::block::BlockState,r#world : crate::root::minecraft::server::world::ServerWorld,r#pos : crate::root::minecraft::util::math::BlockPos,r#random : crate::root::minecraft::util::math::random::Random,) -> () { }
pub fn r#canPathfindThrough(r#state : crate::root::minecraft::block::BlockState,r#world : crate::root::minecraft::world::BlockView,r#pos : crate::root::minecraft::util::math::BlockPos,r#type : crate::root::minecraft::entity::ai::pathing::NavigationType,) -> bool { }
pub fn r#prepare(r#state : crate::root::minecraft::block::BlockState,r#world : crate::root::minecraft::world::WorldAccess,r#pos : crate::root::minecraft::util::math::BlockPos,r#flags : i32,r#maxUpdateDepth : i32,) -> () { }
pub fn r#isSideInvisible(r#state : crate::root::minecraft::block::BlockState,r#stateFrom : crate::root::minecraft::block::BlockState,r#direction : crate::root::minecraft::util::math::Direction,) -> bool { }
pub fn r#getWeakRedstonePower(r#state : crate::root::minecraft::block::BlockState,r#world : crate::root::minecraft::world::BlockView,r#pos : crate::root::minecraft::util::math::BlockPos,r#direction : crate::root::minecraft::util::math::Direction,) -> i32 { }
pub fn r#hasSidedTransparency(r#state : crate::root::minecraft::block::BlockState,) -> bool { }
pub fn r#getPistonBehavior(r#state : crate::root::minecraft::block::BlockState,) -> crate::root::minecraft::block::piston::PistonBehavior { }
pub fn r#getOutlineShape(r#state : crate::root::minecraft::block::BlockState,r#world : crate::root::minecraft::world::BlockView,r#pos : crate::root::minecraft::util::math::BlockPos,r#context : crate::root::minecraft::block::ShapeContext,) -> crate::root::minecraft::util::shape::VoxelShape { }
pub fn r#onUse(r#state : crate::root::minecraft::block::BlockState,r#world : crate::root::minecraft::world::World,r#pos : crate::root::minecraft::util::math::BlockPos,r#player : crate::root::minecraft::entity::player::PlayerEntity,r#hand : crate::root::minecraft::util::Hand,r#hit : crate::root::minecraft::util::hit::BlockHitResult,) -> crate::root::minecraft::util::ActionResult { }
pub fn r#getRenderingSeed(r#state : crate::root::minecraft::block::BlockState,r#pos : crate::root::minecraft::util::math::BlockPos,) -> i64 { }
pub fn r#onStateReplaced(r#state : crate::root::minecraft::block::BlockState,r#world : crate::root::minecraft::world::World,r#pos : crate::root::minecraft::util::math::BlockPos,r#newState : crate::root::minecraft::block::BlockState,r#moved : bool,) -> () { }
pub fn r#getFluidState(r#state : crate::root::minecraft::block::BlockState,) -> crate::root::minecraft::fluid::FluidState { }
pub fn r#onEntityCollision(r#state : crate::root::minecraft::block::BlockState,r#world : crate::root::minecraft::world::World,r#pos : crate::root::minecraft::util::math::BlockPos,r#entity : crate::root::minecraft::entity::Entity,) -> () { }
pub fn r#getCollisionShape(r#state : crate::root::minecraft::block::BlockState,r#world : crate::root::minecraft::world::BlockView,r#pos : crate::root::minecraft::util::math::BlockPos,r#context : crate::root::minecraft::block::ShapeContext,) -> crate::root::minecraft::util::shape::VoxelShape { }
pub fn r#canPlaceAt(r#state : crate::root::minecraft::block::BlockState,r#world : crate::root::minecraft::world::WorldView,r#pos : crate::root::minecraft::util::math::BlockPos,) -> bool { }
pub fn r#getStateForNeighborUpdate(r#state : crate::root::minecraft::block::BlockState,r#direction : crate::root::minecraft::util::math::Direction,r#neighborState : crate::root::minecraft::block::BlockState,r#world : crate::root::minecraft::world::WorldAccess,r#pos : crate::root::minecraft::util::math::BlockPos,r#neighborPos : crate::root::minecraft::util::math::BlockPos,) -> crate::root::minecraft::block::BlockState { }
pub fn r#getDroppedStacks(r#state : crate::root::minecraft::block::BlockState,r#builder : crate::root::minecraft::loot::context::Builder,) -> jni::object::JObject { }
pub fn r#onStacksDropped(r#state : crate::root::minecraft::block::BlockState,r#world : crate::root::minecraft::server::world::ServerWorld,r#pos : crate::root::minecraft::util::math::BlockPos,r#stack : crate::root::minecraft::item::ItemStack,r#dropExperience : bool,) -> () { }
pub fn r#mirror(r#state : crate::root::minecraft::block::BlockState,r#mirror : crate::root::minecraft::util::BlockMirror,) -> crate::root::minecraft::block::BlockState { }
pub fn r#getCullingShape(r#state : crate::root::minecraft::block::BlockState,r#world : crate::root::minecraft::world::BlockView,r#pos : crate::root::minecraft::util::math::BlockPos,) -> crate::root::minecraft::util::shape::VoxelShape { }
pub fn r#getComparatorOutput(r#state : crate::root::minecraft::block::BlockState,r#world : crate::root::minecraft::world::World,r#pos : crate::root::minecraft::util::math::BlockPos,) -> i32 { }
pub fn r#getAmbientOcclusionLightLevel(r#state : crate::root::minecraft::block::BlockState,r#world : crate::root::minecraft::world::BlockView,r#pos : crate::root::minecraft::util::math::BlockPos,) -> f32 { }
pub fn r#getRaycastShape(r#state : crate::root::minecraft::block::BlockState,r#world : crate::root::minecraft::world::BlockView,r#pos : crate::root::minecraft::util::math::BlockPos,) -> crate::root::minecraft::util::shape::VoxelShape { }
pub fn r#scheduledTick(r#state : crate::root::minecraft::block::BlockState,r#world : crate::root::minecraft::server::world::ServerWorld,r#pos : crate::root::minecraft::util::math::BlockPos,r#random : crate::root::minecraft::util::math::random::Random,) -> () { }
pub fn r#onSyncedBlockEvent(r#state : crate::root::minecraft::block::BlockState,r#world : crate::root::minecraft::world::World,r#pos : crate::root::minecraft::util::math::BlockPos,r#type : i32,r#data : i32,) -> bool { }
pub fn r#calcBlockBreakingDelta(r#state : crate::root::minecraft::block::BlockState,r#player : crate::root::minecraft::entity::player::PlayerEntity,r#world : crate::root::minecraft::world::BlockView,r#pos : crate::root::minecraft::util::math::BlockPos,) -> f32 { }
pub fn r#rotate(r#state : crate::root::minecraft::block::BlockState,r#rotation : crate::root::minecraft::util::BlockRotation,) -> crate::root::minecraft::block::BlockState { }
pub fn r#getStrongRedstonePower(r#state : crate::root::minecraft::block::BlockState,r#world : crate::root::minecraft::world::BlockView,r#pos : crate::root::minecraft::util::math::BlockPos,r#direction : crate::root::minecraft::util::math::Direction,) -> i32 { }
pub fn r#getRenderType(r#state : crate::root::minecraft::block::BlockState,) -> crate::root::minecraft::block::BlockRenderType { }
pub fn r#onBlockBreakStart(r#state : crate::root::minecraft::block::BlockState,r#world : crate::root::minecraft::world::World,r#pos : crate::root::minecraft::util::math::BlockPos,r#player : crate::root::minecraft::entity::player::PlayerEntity,) -> () { }
pub fn r#neighborUpdate(r#state : crate::root::minecraft::block::BlockState,r#world : crate::root::minecraft::world::World,r#pos : crate::root::minecraft::util::math::BlockPos,r#sourceBlock : crate::root::minecraft::block::Block,r#sourcePos : crate::root::minecraft::util::math::BlockPos,r#notify : bool,) -> () { }
pub fn r#onBlockAdded(r#state : crate::root::minecraft::block::BlockState,r#world : crate::root::minecraft::world::World,r#pos : crate::root::minecraft::util::math::BlockPos,r#oldState : crate::root::minecraft::block::BlockState,r#notify : bool,) -> () { }
}
}
pub mod client {
pub mod color {
pub mod block {
}
pub mod item {
}
pub mod world {
}
}
pub mod font {
pub struct r#BitmapFont<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for BitmapFont<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl BitmapFont<'_> {
const __map_sig : &str = "net/minecraft/class_386";
/* jni::object::JObject = Lit/unimi/dsi/fastutil/ints/Int2ObjectMap; */
pub fn r#get_glyphs(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lit/unimi/dsi/fastutil/ints/Int2ObjectMap; */
pub fn r#set_glyphs(&self,val : jni::object::JObject) -> () {} 
/* crate::root::minecraft::client::texture::NativeImage = Lnet/minecraft/class_1011; */
pub fn r#get_image(&self) -> Result<crate::root::minecraft::client::texture::NativeImage,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::texture::NativeImage = Lnet/minecraft/class_1011; */
pub fn r#set_image(&self,val : crate::root::minecraft::client::texture::NativeImage) -> () {} 
/* jni::object::JObject = Lorg/slf4j/Logger; */
pub fn r#get_LOGGER(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lorg/slf4j/Logger; */
pub fn r#set_LOGGER(&self,val : jni::object::JObject) -> () {} 
}
}
pub mod gl {
pub struct r#GlDebug<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for GlDebug<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl GlDebug<'_> {
const __map_sig : &str = "net/minecraft/class_1008";
/* i32 = I */
pub fn r#get_DEBUG_MESSAGE_QUEUE_SIZE(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_DEBUG_MESSAGE_QUEUE_SIZE(&self,val : i32) -> () {} 
/* jni::object::JObject = Ljava/util/Queue; */
pub fn r#get_DEBUG_MESSAGES(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/Queue; */
pub fn r#set_DEBUG_MESSAGES(&self,val : jni::object::JObject) -> () {} 
/* crate::root::minecraft::client::gl::DebugMessage = Lnet/minecraft/class_1008$class_6359; */
pub fn r#get_lastDebugMessage(&self) -> Result<crate::root::minecraft::client::gl::DebugMessage,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::gl::DebugMessage = Lnet/minecraft/class_1008$class_6359; */
pub fn r#set_lastDebugMessage(&self,val : crate::root::minecraft::client::gl::DebugMessage) -> () {} 
/* bool = Z */
pub fn r#get_debugMessageEnabled(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_debugMessageEnabled(&self,val : bool) -> () {} 
/* jni::object::JObject = Ljava/util/List; */
pub fn r#get_KHR_VERBOSITY_LEVELS(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/List; */
pub fn r#set_KHR_VERBOSITY_LEVELS(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/util/List; */
pub fn r#get_ARB_VERBOSITY_LEVELS(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/List; */
pub fn r#set_ARB_VERBOSITY_LEVELS(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Lorg/slf4j/Logger; */
pub fn r#get_LOGGER(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lorg/slf4j/Logger; */
pub fn r#set_LOGGER(&self,val : jni::object::JObject) -> () {} 
pub fn r#collectDebugMessages() -> jni::object::JObject { }
pub fn r#isDebugMessageEnabled() -> bool { }
pub fn r#getSource(r#opcode : i32,) -> jni::object::JObject { }
pub fn r#info(r#source : i32,r#type : i32,r#id : i32,r#severity : i32,r#messageLength : i32,r#message : i64,r#unknwnarg_1298017922 : i64,) -> () { }
pub fn r#unknown(r#opcode : i32,) -> jni::object::JObject { }
pub fn r#getSeverity(r#opcode : i32,) -> jni::object::JObject { }
pub fn r#enableDebug(r#verbosity : i32,r#sync : bool,) -> () { }
}
}
pub mod gui {
pub struct r#EditBox<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for EditBox<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl EditBox<'_> {
const __map_sig : &str = "net/minecraft/class_7530";
/* i32 = I */
pub fn r#get_UNLIMITED_LENGTH(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_UNLIMITED_LENGTH(&self,val : i32) -> () {} 
/* i32 = I */
pub fn r#get_CURSOR_WIDTH(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_CURSOR_WIDTH(&self,val : i32) -> () {} 
/* crate::root::minecraft::client::font::TextRenderer = Lnet/minecraft/class_327; */
pub fn r#get_textRenderer(&self) -> Result<crate::root::minecraft::client::font::TextRenderer,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::font::TextRenderer = Lnet/minecraft/class_327; */
pub fn r#set_textRenderer(&self,val : crate::root::minecraft::client::font::TextRenderer) -> () {} 
/* jni::object::JObject = Ljava/util/List; */
pub fn r#get_lines(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/List; */
pub fn r#set_lines(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#get_text(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#set_text(&self,val : jni::object::JObject) -> () {} 
/* i32 = I */
pub fn r#get_cursor(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_cursor(&self,val : i32) -> () {} 
/* i32 = I */
pub fn r#get_selectionEnd(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_selectionEnd(&self,val : i32) -> () {} 
/* bool = Z */
pub fn r#get_selecting(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_selecting(&self,val : bool) -> () {} 
/* i32 = I */
pub fn r#get_maxLength(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_maxLength(&self,val : i32) -> () {} 
/* i32 = I */
pub fn r#get_width(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_width(&self,val : i32) -> () {} 
/* jni::object::JObject = Ljava/util/function/Consumer; */
pub fn r#get_changeListener(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/function/Consumer; */
pub fn r#set_changeListener(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/lang/Runnable; */
pub fn r#get_cursorChangeListener(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/Runnable; */
pub fn r#set_cursorChangeListener(&self,val : jni::object::JObject) -> () {} 
pub fn r#__zINDMINIT(r#textRenderer : crate::root::minecraft::client::font::TextRenderer,r#width : i32,) -> () { }
pub fn r#getMaxLength() -> i32 { }
pub fn r#moveCursor(r#x : f64,r#y : f64,) -> () { }
pub fn r#setMaxLength(r#maxLength : i32,) -> () { }
pub fn r#moveCursor(r#movement : crate::root::minecraft::client::input::CursorMovement,r#amount : i32,) -> () { }
pub fn r#setCursorChangeListener(r#cursorChangeListener : jni::object::JObject,) -> () { }
pub fn r#setText(r#text : jni::object::JObject,) -> () { }
pub fn r#setChangeListener(r#changeListener : jni::object::JObject,) -> () { }
pub fn r#unknwnfn_3354839097(r#style : crate::root::minecraft::text::Style,r#start : i32,r#end : i32,) -> () { }
pub fn r#setSelecting(r#selecting : bool,) -> () { }
pub fn r#hasMaxLength() -> bool { }
pub fn r#delete(r#offset : i32,) -> () { }
pub fn r#replaceSelection(r#string : jni::object::JObject,) -> () { }
pub fn r#getText() -> jni::object::JObject { }
pub fn r#getLine(r#index : i32,) -> crate::root::minecraft::client::gui::Substring { }
pub fn r#truncateForReplacement(r#value : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#getCursor() -> i32 { }
pub fn r#moveCursorLine(r#offset : i32,) -> () { }
pub fn r#truncate(r#value : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#getSelection() -> crate::root::minecraft::client::gui::Substring { }
pub fn r#handleSpecialKey(r#keyCode : i32,) -> bool { }
pub fn r#unknwnfn_1485706227(r#text : jni::object::JObject,) -> () { }
pub fn r#getLineCount() -> i32 { }
pub fn r#getOffsetLine(r#offsetFromCurrent : i32,) -> crate::root::minecraft::client::gui::Substring { }
pub fn r#getCurrentLineIndex() -> i32 { }
pub fn r#getWordEndIndex(r#startIndex : i32,) -> i32 { }
pub fn r#getLines() -> jni::object::JObject { }
pub fn r#hasSelection() -> bool { }
pub fn r#getSelectedText() -> jni::object::JObject { }
pub fn r#getPreviousWordAtCursor() -> crate::root::minecraft::client::gui::Substring { }
pub fn r#getNextWordAtCursor() -> crate::root::minecraft::client::gui::Substring { }
pub fn r#getCurrentLine() -> crate::root::minecraft::client::gui::Substring { }
pub fn r#onChange() -> () { }
}
}
pub mod input {
}
pub mod item {
pub struct r#CompassAnglePredicateProvider<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for CompassAnglePredicateProvider<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl CompassAnglePredicateProvider<'_> {
const __map_sig : &str = "net/minecraft/class_7391";
/* crate::root::minecraft::client::item::AngleInterpolator = Lnet/minecraft/class_7391$class_5171; */
pub fn r#get_aimedInterpolator(&self) -> Result<crate::root::minecraft::client::item::AngleInterpolator,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::item::AngleInterpolator = Lnet/minecraft/class_7391$class_5171; */
pub fn r#set_aimedInterpolator(&self,val : crate::root::minecraft::client::item::AngleInterpolator) -> () {} 
/* crate::root::minecraft::client::item::AngleInterpolator = Lnet/minecraft/class_7391$class_5171; */
pub fn r#get_aimlessInterpolator(&self) -> Result<crate::root::minecraft::client::item::AngleInterpolator,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::item::AngleInterpolator = Lnet/minecraft/class_7391$class_5171; */
pub fn r#set_aimlessInterpolator(&self,val : crate::root::minecraft::client::item::AngleInterpolator) -> () {} 
/* jni::object::JObject = Lnet/minecraft/class_7391$class_7392; */
pub fn r#get_compassTarget(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lnet/minecraft/class_7391$class_7392; */
pub fn r#set_compassTarget(&self,val : jni::object::JObject) -> () {} 
pub fn r#__zINDMINIT(r#compassTarget : jni::object::JObject,) -> () { }
pub fn r#scatter(r#seed : i32,) -> i32 { }
pub fn r#getAimlessAngle(r#seed : i32,r#time : i64,) -> f32 { }
pub fn r#getBodyYaw(r#entity : crate::root::minecraft::entity::Entity,) -> f64 { }
pub fn r#getAngleTo(r#entity : crate::root::minecraft::entity::Entity,r#time : i64,r#pos : crate::root::minecraft::util::math::BlockPos,) -> f32 { }
pub fn r#getClientWorld(r#entity : crate::root::minecraft::entity::Entity,r#world : crate::root::minecraft::client::world::ClientWorld,) -> crate::root::minecraft::client::world::ClientWorld { }
pub fn r#getAngleTo(r#entity : crate::root::minecraft::entity::Entity,r#pos : crate::root::minecraft::util::math::BlockPos,) -> f64 { }
pub fn r#canPointTo(r#entity : crate::root::minecraft::entity::Entity,r#pos : crate::root::minecraft::util::math::GlobalPos,) -> bool { }
}
}
pub mod main {
}
pub struct r#MinecraftClient<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for MinecraftClient<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl MinecraftClient<'_> {
const __map_sig : &str = "net/minecraft/class_310";
/* crate::root::minecraft::util::profiler::Profiler = Lnet/minecraft/class_3695; */
pub fn r#get_profiler(&self) -> Result<crate::root::minecraft::util::profiler::Profiler,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::util::profiler::Profiler = Lnet/minecraft/class_3695; */
pub fn r#set_profiler(&self,val : crate::root::minecraft::util::profiler::Profiler) -> () {} 
/* crate::root::minecraft::client::MinecraftClientGame = Lnet/minecraft/class_3799; */
pub fn r#get_game(&self) -> Result<crate::root::minecraft::client::MinecraftClientGame,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::MinecraftClientGame = Lnet/minecraft/class_3799; */
pub fn r#set_game(&self,val : crate::root::minecraft::client::MinecraftClientGame) -> () {} 
/* crate::root::minecraft::client::util::WindowProvider = Lnet/minecraft/class_3682; */
pub fn r#get_windowProvider(&self) -> Result<crate::root::minecraft::client::util::WindowProvider,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::util::WindowProvider = Lnet/minecraft/class_3682; */
pub fn r#set_windowProvider(&self,val : crate::root::minecraft::client::util::WindowProvider) -> () {} 
/* crate::root::minecraft::client::world::ClientWorld = Lnet/minecraft/class_638; */
pub fn r#get_world(&self) -> Result<crate::root::minecraft::client::world::ClientWorld,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::world::ClientWorld = Lnet/minecraft/class_638; */
pub fn r#set_world(&self,val : crate::root::minecraft::client::world::ClientWorld) -> () {} 
/* crate::root::minecraft::util::MetricsData = Lnet/minecraft/class_3517; */
pub fn r#get_metricsData(&self) -> Result<crate::root::minecraft::util::MetricsData,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::util::MetricsData = Lnet/minecraft/class_3517; */
pub fn r#set_metricsData(&self,val : crate::root::minecraft::util::MetricsData) -> () {} 
/* crate::root::minecraft::client::gl::Framebuffer = Lnet/minecraft/class_276; */
pub fn r#get_framebuffer(&self) -> Result<crate::root::minecraft::client::gl::Framebuffer,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::gl::Framebuffer = Lnet/minecraft/class_276; */
pub fn r#set_framebuffer(&self,val : crate::root::minecraft::client::gl::Framebuffer) -> () {} 
/* crate::root::minecraft::client::option::GameOptions = Lnet/minecraft/class_315; */
pub fn r#get_options(&self) -> Result<crate::root::minecraft::client::option::GameOptions,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::option::GameOptions = Lnet/minecraft/class_315; */
pub fn r#set_options(&self,val : crate::root::minecraft::client::option::GameOptions) -> () {} 
/* crate::root::minecraft::entity::Entity = Lnet/minecraft/class_1297; */
pub fn r#get_targetedEntity(&self) -> Result<crate::root::minecraft::entity::Entity,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::entity::Entity = Lnet/minecraft/class_1297; */
pub fn r#set_targetedEntity(&self,val : crate::root::minecraft::entity::Entity) -> () {} 
/* bool = Z */
pub fn r#get_is64Bit(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_is64Bit(&self,val : bool) -> () {} 
/* jni::object::JObject = Lcom/mojang/authlib/properties/PropertyMap; */
pub fn r#get_sessionPropertyMap(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lcom/mojang/authlib/properties/PropertyMap; */
pub fn r#set_sessionPropertyMap(&self,val : jni::object::JObject) -> () {} 
/* bool = Z */
pub fn r#get_windowFocused(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_windowFocused(&self,val : bool) -> () {} 
/* jni::object::JObject = Ljava/lang/Thread; */
pub fn r#get_thread(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/Thread; */
pub fn r#set_thread(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/io/File; */
pub fn r#get_runDirectory(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/io/File; */
pub fn r#set_runDirectory(&self,val : jni::object::JObject) -> () {} 
/* bool = Z */
pub fn r#get_running(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_running(&self,val : bool) -> () {} 
/* crate::root::minecraft::client::network::ServerInfo = Lnet/minecraft/class_642; */
pub fn r#get_currentServerEntry(&self) -> Result<crate::root::minecraft::client::network::ServerInfo,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::network::ServerInfo = Lnet/minecraft/class_642; */
pub fn r#set_currentServerEntry(&self,val : crate::root::minecraft::client::network::ServerInfo) -> () {} 
/* crate::root::minecraft::client::MinecraftClient = Lnet/minecraft/class_310; */
pub fn r#get_instance(&self) -> Result<crate::root::minecraft::client::MinecraftClient,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::MinecraftClient = Lnet/minecraft/class_310; */
pub fn r#set_instance(&self,val : crate::root::minecraft::client::MinecraftClient) -> () {} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#get_openProfilerSection(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#set_openProfilerSection(&self,val : jni::object::JObject) -> () {} 
/* crate::root::minecraft::client::toast::ToastManager = Lnet/minecraft/class_374; */
pub fn r#get_toastManager(&self) -> Result<crate::root::minecraft::client::toast::ToastManager,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::toast::ToastManager = Lnet/minecraft/class_374; */
pub fn r#set_toastManager(&self,val : crate::root::minecraft::client::toast::ToastManager) -> () {} 
/* bool = Z */
pub fn r#get_IS_SYSTEM_MAC(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_IS_SYSTEM_MAC(&self,val : bool) -> () {} 
/* crate::root::minecraft::client::util::Window = Lnet/minecraft/class_1041; */
pub fn r#get_window(&self) -> Result<crate::root::minecraft::client::util::Window,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::util::Window = Lnet/minecraft/class_1041; */
pub fn r#set_window(&self,val : crate::root::minecraft::client::util::Window) -> () {} 
/* crate::root::minecraft::client::gui::hud::InGameHud = Lnet/minecraft/class_329; */
pub fn r#get_inGameHud(&self) -> Result<crate::root::minecraft::client::gui::hud::InGameHud,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::gui::hud::InGameHud = Lnet/minecraft/class_329; */
pub fn r#set_inGameHud(&self,val : crate::root::minecraft::client::gui::hud::InGameHud) -> () {} 
/* crate::root::minecraft::client::texture::PlayerSkinProvider = Lnet/minecraft/class_1071; */
pub fn r#get_skinProvider(&self) -> Result<crate::root::minecraft::client::texture::PlayerSkinProvider,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::texture::PlayerSkinProvider = Lnet/minecraft/class_1071; */
pub fn r#set_skinProvider(&self,val : crate::root::minecraft::client::texture::PlayerSkinProvider) -> () {} 
/* crate::root::minecraft::client::font::FontManager = Lnet/minecraft/class_378; */
pub fn r#get_fontManager(&self) -> Result<crate::root::minecraft::client::font::FontManager,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::font::FontManager = Lnet/minecraft/class_378; */
pub fn r#set_fontManager(&self,val : crate::root::minecraft::client::font::FontManager) -> () {} 
/* crate::root::minecraft::client::render::debug::DebugRenderer = Lnet/minecraft/class_863; */
pub fn r#get_debugRenderer(&self) -> Result<crate::root::minecraft::client::render::debug::DebugRenderer,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::render::debug::DebugRenderer = Lnet/minecraft/class_863; */
pub fn r#set_debugRenderer(&self,val : crate::root::minecraft::client::render::debug::DebugRenderer) -> () {} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#get_gameVersion(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#set_gameVersion(&self,val : jni::object::JObject) -> () {} 
/* i64 = J */
pub fn r#get_nextDebugInfoUpdateTime(&self) -> Result<i64,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i64 = J */
pub fn r#set_nextDebugInfoUpdateTime(&self,val : i64) -> () {} 
/* crate::root::minecraft::client::particle::ParticleManager = Lnet/minecraft/class_702; */
pub fn r#get_particleManager(&self) -> Result<crate::root::minecraft::client::particle::ParticleManager,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::particle::ParticleManager = Lnet/minecraft/class_702; */
pub fn r#set_particleManager(&self,val : crate::root::minecraft::client::particle::ParticleManager) -> () {} 
/* crate::root::minecraft::client::sound::MusicTracker = Lnet/minecraft/class_1142; */
pub fn r#get_musicTracker(&self) -> Result<crate::root::minecraft::client::sound::MusicTracker,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::sound::MusicTracker = Lnet/minecraft/class_1142; */
pub fn r#set_musicTracker(&self,val : crate::root::minecraft::client::sound::MusicTracker) -> () {} 
/* crate::root::minecraft::resource::ResourcePackManager = Lnet/minecraft/class_3283; */
pub fn r#get_resourcePackManager(&self) -> Result<crate::root::minecraft::resource::ResourcePackManager,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::resource::ResourcePackManager = Lnet/minecraft/class_3283; */
pub fn r#set_resourcePackManager(&self,val : crate::root::minecraft::resource::ResourcePackManager) -> () {} 
/* crate::root::minecraft::client::resource::language::LanguageManager = Lnet/minecraft/class_1076; */
pub fn r#get_languageManager(&self) -> Result<crate::root::minecraft::client::resource::language::LanguageManager,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::resource::language::LanguageManager = Lnet/minecraft/class_1076; */
pub fn r#set_languageManager(&self,val : crate::root::minecraft::client::resource::language::LanguageManager) -> () {} 
/* crate::root::minecraft::entity::Entity = Lnet/minecraft/class_1297; */
pub fn r#get_cameraEntity(&self) -> Result<crate::root::minecraft::entity::Entity,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::entity::Entity = Lnet/minecraft/class_1297; */
pub fn r#set_cameraEntity(&self,val : crate::root::minecraft::entity::Entity) -> () {} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#get_versionType(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#set_versionType(&self,val : jni::object::JObject) -> () {} 
/* bool = Z */
pub fn r#get_isDemo(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_isDemo(&self,val : bool) -> () {} 
/* crate::root::minecraft::client::resource::ClientBuiltinResourcePackProvider = Lnet/minecraft/class_1066; */
pub fn r#get_builtinPackProvider(&self) -> Result<crate::root::minecraft::client::resource::ClientBuiltinResourcePackProvider,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::resource::ClientBuiltinResourcePackProvider = Lnet/minecraft/class_1066; */
pub fn r#set_builtinPackProvider(&self,val : crate::root::minecraft::client::resource::ClientBuiltinResourcePackProvider) -> () {} 
/* jni::object::JObject = Lcom/mojang/authlib/minecraft/MinecraftSessionService; */
pub fn r#get_sessionService(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lcom/mojang/authlib/minecraft/MinecraftSessionService; */
pub fn r#set_sessionService(&self,val : jni::object::JObject) -> () {} 
/* crate::root::minecraft::client::network::ClientPlayerEntity = Lnet/minecraft/class_746; */
pub fn r#get_player(&self) -> Result<crate::root::minecraft::client::network::ClientPlayerEntity,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::network::ClientPlayerEntity = Lnet/minecraft/class_746; */
pub fn r#set_player(&self,val : crate::root::minecraft::client::network::ClientPlayerEntity) -> () {} 
/* crate::root::minecraft::client::util::Session = Lnet/minecraft/class_320; */
pub fn r#get_session(&self) -> Result<crate::root::minecraft::client::util::Session,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::util::Session = Lnet/minecraft/class_320; */
pub fn r#set_session(&self,val : crate::root::minecraft::client::util::Session) -> () {} 
/* crate::root::minecraft::client::sound::SoundManager = Lnet/minecraft/class_1144; */
pub fn r#get_soundManager(&self) -> Result<crate::root::minecraft::client::sound::SoundManager,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::sound::SoundManager = Lnet/minecraft/class_1144; */
pub fn r#set_soundManager(&self,val : crate::root::minecraft::client::sound::SoundManager) -> () {} 
/* crate::root::minecraft::client::render::RenderTickCounter = Lnet/minecraft/class_317; */
pub fn r#get_renderTickCounter(&self) -> Result<crate::root::minecraft::client::render::RenderTickCounter,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::render::RenderTickCounter = Lnet/minecraft/class_317; */
pub fn r#set_renderTickCounter(&self,val : crate::root::minecraft::client::render::RenderTickCounter) -> () {} 
/* crate::root::minecraft::client::Mouse = Lnet/minecraft/class_312; */
pub fn r#get_mouse(&self) -> Result<crate::root::minecraft::client::Mouse,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::Mouse = Lnet/minecraft/class_312; */
pub fn r#set_mouse(&self,val : crate::root::minecraft::client::Mouse) -> () {} 
/* bool = Z */
pub fn r#get_chunkCullingEnabled(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_chunkCullingEnabled(&self,val : bool) -> () {} 
/* crate::root::minecraft::client::render::entity::EntityRenderDispatcher = Lnet/minecraft/class_898; */
pub fn r#get_entityRenderDispatcher(&self) -> Result<crate::root::minecraft::client::render::entity::EntityRenderDispatcher,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::render::entity::EntityRenderDispatcher = Lnet/minecraft/class_898; */
pub fn r#set_entityRenderDispatcher(&self,val : crate::root::minecraft::client::render::entity::EntityRenderDispatcher) -> () {} 
/* crate::root::minecraft::client::option::HotbarStorage = Lnet/minecraft/class_302; */
pub fn r#get_creativeHotbarStorage(&self) -> Result<crate::root::minecraft::client::option::HotbarStorage,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::option::HotbarStorage = Lnet/minecraft/class_302; */
pub fn r#set_creativeHotbarStorage(&self,val : crate::root::minecraft::client::option::HotbarStorage) -> () {} 
/* crate::root::minecraft::client::search::SearchManager = Lnet/minecraft/class_1124; */
pub fn r#get_searchManager(&self) -> Result<crate::root::minecraft::client::search::SearchManager,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::search::SearchManager = Lnet/minecraft/class_1124; */
pub fn r#set_searchManager(&self,val : crate::root::minecraft::client::search::SearchManager) -> () {} 
/* bool = Z */
pub fn r#get_paused(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_paused(&self,val : bool) -> () {} 
/* i32 = I */
pub fn r#get_fpsCounter(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_fpsCounter(&self,val : i32) -> () {} 
/* i32 = I */
pub fn r#get_currentFps(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_currentFps(&self,val : i32) -> () {} 
/* jni::object::JObject = Ljava/net/Proxy; */
pub fn r#get_networkProxy(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/net/Proxy; */
pub fn r#set_networkProxy(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/util/Queue; */
pub fn r#get_renderTaskQueue(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/Queue; */
pub fn r#set_renderTaskQueue(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/util/concurrent/atomic/AtomicReference; */
pub fn r#get_worldGenProgressTracker(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/concurrent/atomic/AtomicReference; */
pub fn r#set_worldGenProgressTracker(&self,val : jni::object::JObject) -> () {} 
/* crate::root::minecraft::util::Identifier = Lnet/minecraft/class_2960; */
pub fn r#get_DEFAULT_FONT_ID(&self) -> Result<crate::root::minecraft::util::Identifier,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::util::Identifier = Lnet/minecraft/class_2960; */
pub fn r#set_DEFAULT_FONT_ID(&self,val : crate::root::minecraft::util::Identifier) -> () {} 
/* f32 = F */
pub fn r#get_pausedTickDelta(&self) -> Result<f32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f32 = F */
pub fn r#set_pausedTickDelta(&self,val : f32) -> () {} 
/* crate::root::minecraft::client::render::item::ItemRenderer = Lnet/minecraft/class_918; */
pub fn r#get_itemRenderer(&self) -> Result<crate::root::minecraft::client::render::item::ItemRenderer,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::render::item::ItemRenderer = Lnet/minecraft/class_918; */
pub fn r#set_itemRenderer(&self,val : crate::root::minecraft::client::render::item::ItemRenderer) -> () {} 
/* bool = Z */
pub fn r#get_skipGameRender(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_skipGameRender(&self,val : bool) -> () {} 
/* bool = Z */
pub fn r#get_connectedToRealms(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_connectedToRealms(&self,val : bool) -> () {} 
/* crate::root::minecraft::resource::ReloadableResourceManagerImpl = Lnet/minecraft/class_3304; */
pub fn r#get_resourceManager(&self) -> Result<crate::root::minecraft::resource::ReloadableResourceManagerImpl,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::resource::ReloadableResourceManagerImpl = Lnet/minecraft/class_3304; */
pub fn r#set_resourceManager(&self,val : crate::root::minecraft::resource::ReloadableResourceManagerImpl) -> () {} 
/* crate::root::minecraft::network::ClientConnection = Lnet/minecraft/class_2535; */
pub fn r#get_integratedServerConnection(&self) -> Result<crate::root::minecraft::network::ClientConnection,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::network::ClientConnection = Lnet/minecraft/class_2535; */
pub fn r#set_integratedServerConnection(&self,val : crate::root::minecraft::network::ClientConnection) -> () {} 
/* jni::object::JObject = Ljava/util/function/Supplier; */
pub fn r#get_crashReportSupplier(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/function/Supplier; */
pub fn r#set_crashReportSupplier(&self,val : jni::object::JObject) -> () {} 
/* crate::root::minecraft::world::level::storage::LevelStorage = Lnet/minecraft/class_32; */
pub fn r#get_levelStorage(&self) -> Result<crate::root::minecraft::world::level::storage::LevelStorage,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::world::level::storage::LevelStorage = Lnet/minecraft/class_32; */
pub fn r#set_levelStorage(&self,val : crate::root::minecraft::world::level::storage::LevelStorage) -> () {} 
/* crate::root::minecraft::util::Identifier = Lnet/minecraft/class_2960; */
pub fn r#get_ALT_TEXT_RENDERER_ID(&self) -> Result<crate::root::minecraft::util::Identifier,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::util::Identifier = Lnet/minecraft/class_2960; */
pub fn r#set_ALT_TEXT_RENDERER_ID(&self,val : crate::root::minecraft::util::Identifier) -> () {} 
/* i64 = J */
pub fn r#get_lastMetricsSampleTime(&self) -> Result<i64,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i64 = J */
pub fn r#set_lastMetricsSampleTime(&self,val : i64) -> () {} 
/* crate::root::minecraft::client::color::block::BlockColors = Lnet/minecraft/class_324; */
pub fn r#get_blockColors(&self) -> Result<crate::root::minecraft::client::color::block::BlockColors,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::color::block::BlockColors = Lnet/minecraft/class_324; */
pub fn r#set_blockColors(&self,val : crate::root::minecraft::client::color::block::BlockColors) -> () {} 
/* i32 = I */
pub fn r#get_itemUseCooldown(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_itemUseCooldown(&self,val : i32) -> () {} 
/* crate::root::minecraft::client::gui::screen::Screen = Lnet/minecraft/class_437; */
pub fn r#get_currentScreen(&self) -> Result<crate::root::minecraft::client::gui::screen::Screen,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::gui::screen::Screen = Lnet/minecraft/class_437; */
pub fn r#set_currentScreen(&self,val : crate::root::minecraft::client::gui::screen::Screen) -> () {} 
/* crate::root::minecraft::client::render::block::BlockRenderManager = Lnet/minecraft/class_776; */
pub fn r#get_blockRenderManager(&self) -> Result<crate::root::minecraft::client::render::block::BlockRenderManager,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::render::block::BlockRenderManager = Lnet/minecraft/class_776; */
pub fn r#set_blockRenderManager(&self,val : crate::root::minecraft::client::render::block::BlockRenderManager) -> () {} 
/* jni::object::JObject = Ljava/io/File; */
pub fn r#get_resourcePackDir(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/io/File; */
pub fn r#set_resourcePackDir(&self,val : jni::object::JObject) -> () {} 
/* crate::root::minecraft::client::tutorial::TutorialManager = Lnet/minecraft/class_1156; */
pub fn r#get_tutorialManager(&self) -> Result<crate::root::minecraft::client::tutorial::TutorialManager,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::tutorial::TutorialManager = Lnet/minecraft/class_1156; */
pub fn r#set_tutorialManager(&self,val : crate::root::minecraft::client::tutorial::TutorialManager) -> () {} 
/* bool = Z */
pub fn r#get_integratedServerRunning(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_integratedServerRunning(&self,val : bool) -> () {} 
/* crate::root::minecraft::client::color::item::ItemColors = Lnet/minecraft/class_325; */
pub fn r#get_itemColors(&self) -> Result<crate::root::minecraft::client::color::item::ItemColors,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::color::item::ItemColors = Lnet/minecraft/class_325; */
pub fn r#set_itemColors(&self,val : crate::root::minecraft::client::color::item::ItemColors) -> () {} 
/* crate::root::minecraft::client::network::ClientPlayerInteractionManager = Lnet/minecraft/class_636; */
pub fn r#get_interactionManager(&self) -> Result<crate::root::minecraft::client::network::ClientPlayerInteractionManager,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::network::ClientPlayerInteractionManager = Lnet/minecraft/class_636; */
pub fn r#set_interactionManager(&self,val : crate::root::minecraft::client::network::ClientPlayerInteractionManager) -> () {} 
/* jni::object::JObject = Lorg/slf4j/Logger; */
pub fn r#get_LOGGER(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lorg/slf4j/Logger; */
pub fn r#set_LOGGER(&self,val : jni::object::JObject) -> () {} 
/* crate::root::minecraft::client::render::model::BakedModelManager = Lnet/minecraft/class_1092; */
pub fn r#get_bakedModelManager(&self) -> Result<crate::root::minecraft::client::render::model::BakedModelManager,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::render::model::BakedModelManager = Lnet/minecraft/class_1092; */
pub fn r#set_bakedModelManager(&self,val : crate::root::minecraft::client::render::model::BakedModelManager) -> () {} 
/* crate::root::minecraft::client::texture::TextureManager = Lnet/minecraft/class_1060; */
pub fn r#get_textureManager(&self) -> Result<crate::root::minecraft::client::texture::TextureManager,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::texture::TextureManager = Lnet/minecraft/class_1060; */
pub fn r#set_textureManager(&self,val : crate::root::minecraft::client::texture::TextureManager) -> () {} 
/* crate::root::minecraft::util::hit::HitResult = Lnet/minecraft/class_239; */
pub fn r#get_crosshairTarget(&self) -> Result<crate::root::minecraft::util::hit::HitResult,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::util::hit::HitResult = Lnet/minecraft/class_239; */
pub fn r#set_crosshairTarget(&self,val : crate::root::minecraft::util::hit::HitResult) -> () {} 
/* crate::root::minecraft::server::integrated::IntegratedServer = Lnet/minecraft/class_1132; */
pub fn r#get_server(&self) -> Result<crate::root::minecraft::server::integrated::IntegratedServer,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::server::integrated::IntegratedServer = Lnet/minecraft/class_1132; */
pub fn r#set_server(&self,val : crate::root::minecraft::server::integrated::IntegratedServer) -> () {} 
/* jni::object::JObject = Lcom/mojang/datafixers/DataFixer; */
pub fn r#get_dataFixer(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lcom/mojang/datafixers/DataFixer; */
pub fn r#set_dataFixer(&self,val : jni::object::JObject) -> () {} 
/* crate::root::minecraft::client::render::WorldRenderer = Lnet/minecraft/class_761; */
pub fn r#get_worldRenderer(&self) -> Result<crate::root::minecraft::client::render::WorldRenderer,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::render::WorldRenderer = Lnet/minecraft/class_761; */
pub fn r#set_worldRenderer(&self,val : crate::root::minecraft::client::render::WorldRenderer) -> () {} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#get_fpsDebugString(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#set_fpsDebugString(&self,val : jni::object::JObject) -> () {} 
/* i32 = I */
pub fn r#get_attackCooldown(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_attackCooldown(&self,val : i32) -> () {} 
/* crate::root::minecraft::client::font::TextRenderer = Lnet/minecraft/class_327; */
pub fn r#get_textRenderer(&self) -> Result<crate::root::minecraft::client::font::TextRenderer,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::font::TextRenderer = Lnet/minecraft/class_327; */
pub fn r#set_textRenderer(&self,val : crate::root::minecraft::client::font::TextRenderer) -> () {} 
/* crate::root::minecraft::client::render::GameRenderer = Lnet/minecraft/class_757; */
pub fn r#get_gameRenderer(&self) -> Result<crate::root::minecraft::client::render::GameRenderer,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::render::GameRenderer = Lnet/minecraft/class_757; */
pub fn r#set_gameRenderer(&self,val : crate::root::minecraft::client::render::GameRenderer) -> () {} 
/* crate::root::minecraft::client::Keyboard = Lnet/minecraft/class_309; */
pub fn r#get_keyboard(&self) -> Result<crate::root::minecraft::client::Keyboard,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::Keyboard = Lnet/minecraft/class_309; */
pub fn r#set_keyboard(&self,val : crate::root::minecraft::client::Keyboard) -> () {} 
/* crate::root::minecraft::client::resource::SplashTextResourceSupplier = Lnet/minecraft/class_4008; */
pub fn r#get_splashTextLoader(&self) -> Result<crate::root::minecraft::client::resource::SplashTextResourceSupplier,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::resource::SplashTextResourceSupplier = Lnet/minecraft/class_4008; */
pub fn r#set_splashTextLoader(&self,val : crate::root::minecraft::client::resource::SplashTextResourceSupplier) -> () {} 
/* crate::root::minecraft::client::texture::PaintingManager = Lnet/minecraft/class_4044; */
pub fn r#get_paintingManager(&self) -> Result<crate::root::minecraft::client::texture::PaintingManager,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::texture::PaintingManager = Lnet/minecraft/class_4044; */
pub fn r#set_paintingManager(&self,val : crate::root::minecraft::client::texture::PaintingManager) -> () {} 
/* jni::object::JObject = Ljava/util/concurrent/CompletableFuture; */
pub fn r#get_COMPLETED_UNIT_FUTURE(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/concurrent/CompletableFuture; */
pub fn r#set_COMPLETED_UNIT_FUTURE(&self,val : jni::object::JObject) -> () {} 
/* crate::root::minecraft::client::texture::StatusEffectSpriteManager = Lnet/minecraft/class_4074; */
pub fn r#get_statusEffectSpriteManager(&self) -> Result<crate::root::minecraft::client::texture::StatusEffectSpriteManager,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::texture::StatusEffectSpriteManager = Lnet/minecraft/class_4074; */
pub fn r#set_statusEffectSpriteManager(&self,val : crate::root::minecraft::client::texture::StatusEffectSpriteManager) -> () {} 
/* jni::object::JObject = Ljava/util/concurrent/CompletableFuture; */
pub fn r#get_resourceReloadFuture(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/concurrent/CompletableFuture; */
pub fn r#set_resourceReloadFuture(&self,val : jni::object::JObject) -> () {} 
/* crate::root::minecraft::client::gui::screen::Overlay = Lnet/minecraft/class_4071; */
pub fn r#get_overlay(&self) -> Result<crate::root::minecraft::client::gui::screen::Overlay,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::gui::screen::Overlay = Lnet/minecraft/class_4071; */
pub fn r#set_overlay(&self,val : crate::root::minecraft::client::gui::screen::Overlay) -> () {} 
/* bool = Z */
pub fn r#get_debugChunkInfo(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_debugChunkInfo(&self,val : bool) -> () {} 
/* bool = Z */
pub fn r#get_debugChunkOcclusion(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_debugChunkOcclusion(&self,val : bool) -> () {} 
/* crate::root::minecraft::client::render::BufferBuilderStorage = Lnet/minecraft/class_4599; */
pub fn r#get_bufferBuilders(&self) -> Result<crate::root::minecraft::client::render::BufferBuilderStorage,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::render::BufferBuilderStorage = Lnet/minecraft/class_4599; */
pub fn r#set_bufferBuilders(&self,val : crate::root::minecraft::client::render::BufferBuilderStorage) -> () {} 
/* i32 = I */
pub fn r#get_trackingTick(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_trackingTick(&self,val : i32) -> () {} 
/* crate::root::minecraft::util::profiler::TickTimeTracker = Lnet/minecraft/class_4757; */
pub fn r#get_tickTimeTracker(&self) -> Result<crate::root::minecraft::util::profiler::TickTimeTracker,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::util::profiler::TickTimeTracker = Lnet/minecraft/class_4757; */
pub fn r#set_tickTimeTracker(&self,val : crate::root::minecraft::util::profiler::TickTimeTracker) -> () {} 
/* crate::root::minecraft::util::profiler::ProfileResult = Lnet/minecraft/class_3696; */
pub fn r#get_tickProfilerResult(&self) -> Result<crate::root::minecraft::util::profiler::ProfileResult,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::util::profiler::ProfileResult = Lnet/minecraft/class_3696; */
pub fn r#set_tickProfilerResult(&self,val : crate::root::minecraft::util::profiler::ProfileResult) -> () {} 
/* crate::root::minecraft::util::Identifier = Lnet/minecraft/class_2960; */
pub fn r#get_UNICODE_FONT_ID(&self) -> Result<crate::root::minecraft::util::Identifier,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::util::Identifier = Lnet/minecraft/class_2960; */
pub fn r#set_UNICODE_FONT_ID(&self,val : crate::root::minecraft::util::Identifier) -> () {} 
/* bool = Z */
pub fn r#get_multiplayerEnabled(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_multiplayerEnabled(&self,val : bool) -> () {} 
/* bool = Z */
pub fn r#get_onlineChatEnabled(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_onlineChatEnabled(&self,val : bool) -> () {} 
/* crate::root::minecraft::client::resource::VideoWarningManager = Lnet/minecraft/class_5407; */
pub fn r#get_videoWarningManager(&self) -> Result<crate::root::minecraft::client::resource::VideoWarningManager,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::resource::VideoWarningManager = Lnet/minecraft/class_5407; */
pub fn r#set_videoWarningManager(&self,val : crate::root::minecraft::client::resource::VideoWarningManager) -> () {} 
/* crate::root::minecraft::text::Text = Lnet/minecraft/class_2561; */
pub fn r#get_SOCIAL_INTERACTIONS_NOT_AVAILABLE(&self) -> Result<crate::root::minecraft::text::Text,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::text::Text = Lnet/minecraft/class_2561; */
pub fn r#set_SOCIAL_INTERACTIONS_NOT_AVAILABLE(&self,val : crate::root::minecraft::text::Text) -> () {} 
/* crate::root::minecraft::client::network::SocialInteractionsManager = Lnet/minecraft/class_5520; */
pub fn r#get_socialInteractionsManager(&self) -> Result<crate::root::minecraft::client::network::SocialInteractionsManager,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::network::SocialInteractionsManager = Lnet/minecraft/class_5520; */
pub fn r#set_socialInteractionsManager(&self,val : crate::root::minecraft::client::network::SocialInteractionsManager) -> () {} 
/* crate::root::minecraft::client::toast::TutorialToast = Lnet/minecraft/class_372; */
pub fn r#get_socialInteractionsToast(&self) -> Result<crate::root::minecraft::client::toast::TutorialToast,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::toast::TutorialToast = Lnet/minecraft/class_372; */
pub fn r#set_socialInteractionsToast(&self,val : crate::root::minecraft::client::toast::TutorialToast) -> () {} 
/* jni::object::JObject = Lcom/mojang/authlib/minecraft/UserApiService; */
pub fn r#get_userApiService(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lcom/mojang/authlib/minecraft/UserApiService; */
pub fn r#set_userApiService(&self,val : jni::object::JObject) -> () {} 
/* crate::root::minecraft::client::render::entity::model::EntityModelLoader = Lnet/minecraft/class_5599; */
pub fn r#get_entityModelLoader(&self) -> Result<crate::root::minecraft::client::render::entity::model::EntityModelLoader,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::render::entity::model::EntityModelLoader = Lnet/minecraft/class_5599; */
pub fn r#set_entityModelLoader(&self,val : crate::root::minecraft::client::render::entity::model::EntityModelLoader) -> () {} 
/* crate::root::minecraft::client::render::block::entity::BlockEntityRenderDispatcher = Lnet/minecraft/class_824; */
pub fn r#get_blockEntityRenderDispatcher(&self) -> Result<crate::root::minecraft::client::render::block::entity::BlockEntityRenderDispatcher,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::render::block::entity::BlockEntityRenderDispatcher = Lnet/minecraft/class_824; */
pub fn r#set_blockEntityRenderDispatcher(&self,val : crate::root::minecraft::client::render::block::entity::BlockEntityRenderDispatcher) -> () {} 
/* crate::root::minecraft::util::profiler::Recorder = Lnet/minecraft/class_5962; */
pub fn r#get_recorder(&self) -> Result<crate::root::minecraft::util::profiler::Recorder,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::util::profiler::Recorder = Lnet/minecraft/class_5962; */
pub fn r#set_recorder(&self,val : crate::root::minecraft::util::profiler::Recorder) -> () {} 
/* bool = Z */
pub fn r#get_wireFrame(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_wireFrame(&self,val : bool) -> () {} 
/* crate::root::minecraft::client::resource::ResourceReloadLogger = Lnet/minecraft/class_6360; */
pub fn r#get_resourceReloadLogger(&self) -> Result<crate::root::minecraft::client::resource::ResourceReloadLogger,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::resource::ResourceReloadLogger = Lnet/minecraft/class_6360; */
pub fn r#set_resourceReloadLogger(&self,val : crate::root::minecraft::client::resource::ResourceReloadLogger) -> () {} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#get_GL_ERROR_DIALOGUE(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#set_GL_ERROR_DIALOGUE(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/util/UUID; */
pub fn r#get_deviceSessionId(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/UUID; */
pub fn r#set_deviceSessionId(&self,val : jni::object::JObject) -> () {} 
/* crate::root::minecraft::util::Identifier = Lnet/minecraft/class_2960; */
pub fn r#get_REGIONAL_COMPLIANCIES_ID(&self) -> Result<crate::root::minecraft::util::Identifier,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::util::Identifier = Lnet/minecraft/class_2960; */
pub fn r#set_REGIONAL_COMPLIANCIES_ID(&self,val : crate::root::minecraft::util::Identifier) -> () {} 
/* crate::root::minecraft::client::resource::PeriodicNotificationManager = Lnet/minecraft/class_6877; */
pub fn r#get_regionalComplianciesManager(&self) -> Result<crate::root::minecraft::client::resource::PeriodicNotificationManager,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::resource::PeriodicNotificationManager = Lnet/minecraft/class_6877; */
pub fn r#set_regionalComplianciesManager(&self,val : crate::root::minecraft::client::resource::PeriodicNotificationManager) -> () {} 
/* i64 = J */
pub fn r#get_metricsSampleDuration(&self) -> Result<i64,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i64 = J */
pub fn r#set_metricsSampleDuration(&self,val : i64) -> () {} 
/* f64 = D */
pub fn r#get_gpuUtilizationPercentage(&self) -> Result<f64,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f64 = D */
pub fn r#set_gpuUtilizationPercentage(&self,val : f64) -> () {} 
/* crate::root::minecraft::client::gl::Query = Lnet/minecraft/class_7168$class_7169; */
pub fn r#get_currentGlTimerQuery(&self) -> Result<crate::root::minecraft::client::gl::Query,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::gl::Query = Lnet/minecraft/class_7168$class_7169; */
pub fn r#set_currentGlTimerQuery(&self,val : crate::root::minecraft::client::gl::Query) -> () {} 
/* crate::root::minecraft::client::util::ProfileKeys = Lnet/minecraft/class_7434; */
pub fn r#get_profileKeys(&self) -> Result<crate::root::minecraft::client::util::ProfileKeys,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::util::ProfileKeys = Lnet/minecraft/class_7434; */
pub fn r#set_profileKeys(&self,val : crate::root::minecraft::client::util::ProfileKeys) -> () {} 
/* crate::root::minecraft::client::realms::util::Realms32BitWarningChecker = Lnet/minecraft/class_7478; */
pub fn r#get_realms32BitWarningChecker(&self) -> Result<crate::root::minecraft::client::realms::util::Realms32BitWarningChecker,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::realms::util::Realms32BitWarningChecker = Lnet/minecraft/class_7478; */
pub fn r#set_realms32BitWarningChecker(&self,val : crate::root::minecraft::client::realms::util::Realms32BitWarningChecker) -> () {} 
/* jni::object::JObject = Lcom/mojang/authlib/yggdrasil/YggdrasilAuthenticationService; */
pub fn r#get_authenticationService(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lcom/mojang/authlib/yggdrasil/YggdrasilAuthenticationService; */
pub fn r#set_authenticationService(&self,val : jni::object::JObject) -> () {} 
/* crate::root::minecraft::network::encryption::SignatureVerifier = Lnet/minecraft/class_7500; */
pub fn r#get_servicesSignatureVerifier(&self) -> Result<crate::root::minecraft::network::encryption::SignatureVerifier,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::network::encryption::SignatureVerifier = Lnet/minecraft/class_7500; */
pub fn r#set_servicesSignatureVerifier(&self,val : crate::root::minecraft::network::encryption::SignatureVerifier) -> () {} 
/* crate::root::minecraft::client::report::AbuseReportContext = Lnet/minecraft/class_7574; */
pub fn r#get_abuseReportContext(&self) -> Result<crate::root::minecraft::client::report::AbuseReportContext,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::report::AbuseReportContext = Lnet/minecraft/class_7574; */
pub fn r#set_abuseReportContext(&self,val : crate::root::minecraft::client::report::AbuseReportContext) -> () {} 
/* crate::root::minecraft::client::realms::RealmsPeriodicCheckers = Lnet/minecraft/class_7578; */
pub fn r#get_realmsPeriodicCheckers(&self) -> Result<crate::root::minecraft::client::realms::RealmsPeriodicCheckers,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::realms::RealmsPeriodicCheckers = Lnet/minecraft/class_7578; */
pub fn r#set_realmsPeriodicCheckers(&self,val : crate::root::minecraft::client::realms::RealmsPeriodicCheckers) -> () {} 
/* crate::root::minecraft::client::util::NarratorManager = Lnet/minecraft/class_333; */
pub fn r#get_narratorManager(&self) -> Result<crate::root::minecraft::client::util::NarratorManager,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::util::NarratorManager = Lnet/minecraft/class_333; */
pub fn r#set_narratorManager(&self,val : crate::root::minecraft::client::util::NarratorManager) -> () {} 
/* crate::root::minecraft::client::network::message::MessageHandler = Lnet/minecraft/class_7594; */
pub fn r#get_messageHandler(&self) -> Result<crate::root::minecraft::client::network::message::MessageHandler,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::network::message::MessageHandler = Lnet/minecraft/class_7594; */
pub fn r#set_messageHandler(&self,val : crate::root::minecraft::client::network::message::MessageHandler) -> () {} 
/* crate::root::minecraft::client::font::TextRenderer = Lnet/minecraft/class_327; */
pub fn r#get_advanceValidatingTextRenderer(&self) -> Result<crate::root::minecraft::client::font::TextRenderer,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::client::font::TextRenderer = Lnet/minecraft/class_327; */
pub fn r#set_advanceValidatingTextRenderer(&self,val : crate::root::minecraft::client::font::TextRenderer) -> () {} 
pub fn r#__zINDMINIT(r#args : crate::root::minecraft::client::RunArgs,) -> () { }
pub fn r#checkIs64Bit() -> bool { }
pub fn r#getResourceManager() -> crate::root::minecraft::resource::ResourceManager { }
pub fn r#getResourcePackDir() -> jni::object::JObject { }
pub fn r#getItemRenderer() -> crate::root::minecraft::client::render::item::ItemRenderer { }
pub fn r#joinWorld(r#world : crate::root::minecraft::client::world::ClientWorld,) -> () { }
pub fn r#getSoundManager() -> crate::root::minecraft::client::sound::SoundManager { }
pub fn r#getSearchProvider(r#key : crate::root::minecraft::client::search::Key,) -> crate::root::minecraft::client::search::SearchProvider { }
pub fn r#unknwnfn_291989438(r#stack : crate::root::minecraft::item::ItemStack,) -> jni::object::JObject { }
pub fn r#getNetworkProxy() -> jni::object::JObject { }
pub fn r#getTickDelta() -> f32 { }
pub fn r#stop() -> () { }
pub fn r#drawProfilerResults(r#matrices : crate::root::minecraft::client::util::math::MatrixStack,r#profileResult : crate::root::minecraft::util::profiler::ProfileResult,) -> () { }
pub fn r#isPaused() -> bool { }
pub fn r#setCrashReportSupplierAndAddDetails(r#crashReport : crate::root::minecraft::util::crash::CrashReport,) -> () { }
pub fn r#getSessionService() -> jni::object::JObject { }
pub fn r#isIntegratedServerRunning() -> bool { }
pub fn r#isHudEnabled() -> bool { }
pub fn r#addBlockEntityNbt(r#stack : crate::root::minecraft::item::ItemStack,r#blockEntity : crate::root::minecraft::block::entity::BlockEntity,) -> crate::root::minecraft::item::ItemStack { }
pub fn r#unknwnfn_1562372525(r#stack : crate::root::minecraft::item::ItemStack,) -> jni::object::JObject { }
pub fn r#setCameraEntity(r#entity : crate::root::minecraft::entity::Entity,) -> () { }
pub fn r#getBlockColors() -> crate::root::minecraft::client::color::block::BlockColors { }
pub fn r#handleGlErrorByDisableVsync(r#error : i32,r#description : i64,) -> () { }
pub fn r#setScreen(r#screen : crate::root::minecraft::client::gui::screen::Screen,) -> () { }
pub fn r#handleInputEvents() -> () { }
pub fn r#unknwnfn_3015602188(r#status : crate::root::minecraft::text::Text,) -> () { }
pub fn r#doItemPick() -> () { }
pub fn r#reloadResourcesConcurrently() -> jni::object::JObject { }
pub fn r#run() -> () { }
pub fn r#getGameVersion() -> jni::object::JObject { }
pub fn r#getResourcePackProvider() -> crate::root::minecraft::client::resource::ClientBuiltinResourcePackProvider { }
pub fn r#isFancyGraphicsOrBetter() -> bool { }
pub fn r#cleanUpAfterCrash() -> () { }
pub fn r#getResourcePackManager() -> crate::root::minecraft::resource::ResourcePackManager { }
pub fn r#reloadResources() -> jni::object::JObject { }
pub fn r#getFramebuffer() -> crate::root::minecraft::client::gl::Framebuffer { }
pub fn r#render(r#tick : bool,) -> () { }
pub fn r#handleProfilerKeyPress(r#digit : i32,) -> () { }
pub fn r#unknwnfn_1554112547(r#stack : crate::root::minecraft::item::ItemStack,) -> jni::object::JObject { }
pub fn r#getLanguageManager() -> crate::root::minecraft::client::resource::language::LanguageManager { }
pub fn r#isDemo() -> bool { }
pub fn r#getTextureManager() -> crate::root::minecraft::client::texture::TextureManager { }
pub fn r#getLastFrameDuration() -> f32 { }
pub fn r#doAttack() -> bool { }
pub fn r#setConnectedToRealms(r#connectedToRealms : bool,) -> () { }
pub fn r#getMusicTracker() -> crate::root::minecraft::client::sound::MusicTracker { }
pub fn r#getSessionProperties() -> jni::object::JObject { }
pub fn r#is64Bit() -> bool { }
pub fn r#getBlockRenderManager() -> crate::root::minecraft::client::render::block::BlockRenderManager { }
pub fn r#isInSingleplayer() -> bool { }
pub fn r#getDataFixer() -> jni::object::JObject { }
pub fn r#getMusicType() -> crate::root::minecraft::sound::MusicSound { }
pub fn r#initializeSearchProviders() -> () { }
pub fn r#getVersionType() -> jni::object::JObject { }
pub fn r#getSession() -> crate::root::minecraft::client::util::Session { }
pub fn r#getSpriteAtlas(r#id : crate::root::minecraft::util::Identifier,) -> jni::object::JObject { }
pub fn r#getInstance() -> crate::root::minecraft::client::MinecraftClient { }
pub fn r#getBakedModelManager() -> crate::root::minecraft::client::render::model::BakedModelManager { }
pub fn r#hasReducedDebugInfo() -> bool { }
pub fn r#unknwnfn_3084091126(r#recipe : crate::root::minecraft::recipe::Recipe,) -> crate::root::minecraft::util::Identifier { }
pub fn r#getCurrentServerEntry() -> crate::root::minecraft::client::network::ServerInfo { }
pub fn r#getCameraEntity() -> crate::root::minecraft::entity::Entity { }
pub fn r#getEntityRenderDispatcher() -> crate::root::minecraft::client::render::entity::EntityRenderDispatcher { }
pub fn r#getNetworkHandler() -> crate::root::minecraft::client::network::ClientPlayNetworkHandler { }
pub fn r#printCrashReport(r#report : crate::root::minecraft::util::crash::CrashReport,) -> () { }
pub fn r#getToastManager() -> crate::root::minecraft::client::toast::ToastManager { }
pub fn r#isWindowFocused() -> bool { }
pub fn r#getMetricsData() -> crate::root::minecraft::util::MetricsData { }
pub fn r#getCreativeHotbarStorage() -> crate::root::minecraft::client::option::HotbarStorage { }
pub fn r#forcesUnicodeFont() -> bool { }
pub fn r#tick() -> () { }
pub fn r#getServer() -> crate::root::minecraft::server::integrated::IntegratedServer { }
pub fn r#getTutorialManager() -> crate::root::minecraft::client::tutorial::TutorialManager { }
pub fn r#unknwnfn_504520648(r#string : jni::object::JObject,) -> bool { }
pub fn r#unknwnfn_705469844(r#tooltip : crate::root::minecraft::text::Text,) -> jni::object::JObject { }
pub fn r#getSkinProvider() -> crate::root::minecraft::client::texture::PlayerSkinProvider { }
pub fn r#doItemUse() -> () { }
pub fn r#setCurrentServerEntry(r#serverEntry : crate::root::minecraft::client::network::ServerInfo,) -> () { }
pub fn r#getLevelStorage() -> crate::root::minecraft::world::level::storage::LevelStorage { }
pub fn r#addDetailsToCrashReport(r#report : crate::root::minecraft::util::crash::CrashReport,) -> crate::root::minecraft::util::crash::CrashReport { }
pub fn r#isAmbientOcclusionEnabled() -> bool { }
pub fn r#isConnectedToRealms() -> bool { }
pub fn r#handleBlockBreaking(r#breaking : bool,) -> () { }
pub fn r#unknwnfn_768345516(r#recipe : crate::root::minecraft::recipe::Recipe,) -> jni::object::JObject { }
pub fn r#scheduleStop() -> () { }
pub fn r#getFramerateLimit() -> i32 { }
pub fn r#getProfiler() -> crate::root::minecraft::util::profiler::Profiler { }
pub fn r#getGame() -> crate::root::minecraft::client::MinecraftClientGame { }
pub fn r#checkGameData() -> () { }
pub fn r#unknwnfn_3254627728(r#spawnChunkRadius : i32,) -> crate::root::minecraft::server::WorldGenerationProgressListener { }
pub fn r#getSplashTextLoader() -> crate::root::minecraft::client::resource::SplashTextResourceSupplier { }
pub fn r#disconnect(r#screen : crate::root::minecraft::client::gui::screen::Screen,) -> () { }
pub fn r#setWorld(r#world : crate::root::minecraft::client::world::ClientWorld,) -> () { }
pub fn r#reset(r#screen : crate::root::minecraft::client::gui::screen::Screen,) -> () { }
pub fn r#disconnect() -> () { }
pub fn r#getPaintingManager() -> crate::root::minecraft::client::texture::PaintingManager { }
pub fn r#setOverlay(r#overlay : crate::root::minecraft::client::gui::screen::Overlay,) -> () { }
pub fn r#getStatusEffectSpriteManager() -> crate::root::minecraft::client::texture::StatusEffectSpriteManager { }
pub fn r#getOverlay() -> crate::root::minecraft::client::gui::screen::Overlay { }
pub fn r#unknwnfn_3948731855(r#future : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#openPauseMenu(r#pause : bool,) -> () { }
pub fn r#shouldRenderAsync() -> bool { }
pub fn r#isRunning() -> bool { }
pub fn r#addSystemDetailsToCrashReport(r#client : crate::root::minecraft::client::MinecraftClient,r#languageManager : crate::root::minecraft::client::resource::language::LanguageManager,r#version : jni::object::JObject,r#options : crate::root::minecraft::client::option::GameOptions,r#report : crate::root::minecraft::util::crash::CrashReport,) -> () { }
pub fn r#getWindow() -> crate::root::minecraft::client::util::Window { }
pub fn r#getBufferBuilders() -> crate::root::minecraft::client::render::BufferBuilderStorage { }
pub fn r#createResourcePackProfile(r#name : jni::object::JObject,r#displayName : crate::root::minecraft::text::Text,r#alwaysEnabled : bool,r#packFactory : jni::object::JObject,r#metadata : crate::root::minecraft::resource::metadata::PackResourceMetadata,r#insertionPosition : crate::root::minecraft::resource::InsertionPosition,r#source : crate::root::minecraft::resource::ResourcePackSource,) -> crate::root::minecraft::resource::ResourcePackProfile { }
pub fn r#unknwnfn_3587117816(r#throwable : jni::object::JObject,) -> () { }
pub fn r#setMipmapLevels(r#mipmapLevels : i32,) -> () { }
pub fn r#createV3ResourcePackFactory(r#packFactory : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#createV4ResourcePackFactory(r#packFactory : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#handleResourceReloadException(r#throwable : jni::object::JObject,) -> () { }
pub fn r#unknwnfn_2372286167(r#throwable : jni::object::JObject,r#unknwnarg_4154362631 : jni::object::JObject,) -> () { }
pub fn r#getWindowTitle() -> jni::object::JObject { }
pub fn r#updateWindowTitle() -> () { }
pub fn r#getModStatus() -> crate::root::minecraft::util::ModStatus { }
pub fn r#startMonitor(r#active : bool,r#monitor : crate::root::minecraft::util::TickDurationMonitor,) -> crate::root::minecraft::util::profiler::Profiler { }
pub fn r#shouldMonitorTickDuration() -> bool { }
pub fn r#endMonitor(r#active : bool,r#monitor : crate::root::minecraft::util::TickDurationMonitor,) -> () { }
pub fn r#hasOutline(r#entity : crate::root::minecraft::entity::Entity,) -> bool { }
pub fn r#initFont(r#forcesUnicode : bool,) -> () { }
pub fn r#openChatScreen(r#text : jni::object::JObject,) -> () { }
pub fn r#shouldBlockMessages(r#sender : jni::object::JObject,) -> bool { }
pub fn r#isMultiplayerEnabled() -> bool { }
pub fn r#unknwnfn_2300802965(r#thread : crate::root::minecraft::world::level::storage::Session,r#unknwnarg_1875306111 : crate::root::minecraft::resource::ResourcePackManager,r#unknwnarg_1113206184 : crate::root::minecraft::server::SaveLoader,r#unknwnarg_1916015486 : crate::root::minecraft::util::ApiServices,r#unknwnarg_461552967 : jni::object::JObject,) -> crate::root::minecraft::server::integrated::IntegratedServer { }
pub fn r#startIntegratedServer(r#levelName : jni::object::JObject,r#session : crate::root::minecraft::world::level::storage::Session,r#dataPackManager : crate::root::minecraft::resource::ResourcePackManager,r#saveLoader : crate::root::minecraft::server::SaveLoader,) -> () { }
pub fn r#isFabulousGraphicsOrBetter() -> bool { }
pub fn r#setScreenAndRender(r#screen : crate::root::minecraft::client::gui::screen::Screen,) -> () { }
pub fn r#getVideoWarningManager() -> crate::root::minecraft::client::resource::VideoWarningManager { }
pub fn r#onResourceReloadFailure(r#exception : jni::object::JObject,r#resourceName : crate::root::minecraft::text::Text,) -> () { }
pub fn r#getSocialInteractionsManager() -> crate::root::minecraft::client::network::SocialInteractionsManager { }
pub fn r#isConnectedToServer() -> bool { }
pub fn r#createUserApiService(r#authService : jni::object::JObject,r#runArgs : crate::root::minecraft::client::RunArgs,) -> jni::object::JObject { }
pub fn r#getEntityModelLoader() -> crate::root::minecraft::client::render::entity::model::EntityModelLoader { }
pub fn r#getBlockEntityRenderDispatcher() -> crate::root::minecraft::client::render::block::entity::BlockEntityRenderDispatcher { }
pub fn r#shouldFilterText() -> bool { }
pub fn r#getChatRestriction() -> crate::root::minecraft::client::ChatRestriction { }
pub fn r#toggleDebugProfiler(r#chatMessageSender : jni::object::JObject,) -> bool { }
pub fn r#unknwnfn_1709583790(r#result : jni::object::JObject,r#unknwnarg_3297853342 : crate::root::minecraft::util::profiler::ProfileResult,) -> () { }
pub fn r#takePanorama(r#directory : jni::object::JObject,r#width : i32,r#height : i32,) -> crate::root::minecraft::text::Text { }
pub fn r#takeHugeScreenshot(r#gameDirectory : jni::object::JObject,r#unitWidth : i32,r#unitHeight : i32,r#width : i32,r#height : i32,) -> crate::root::minecraft::text::Text { }
pub fn r#unknwnfn_2298422127(r#style : jni::object::JObject,r#unknwnarg_2177933025 : crate::root::minecraft::text::Style,) -> crate::root::minecraft::text::Style { }
pub fn r#unknwnfn_3417872234(r#message : crate::root::minecraft::text::Text,) -> () { }
pub fn r#getWorldGenerationProgressTracker() -> crate::root::minecraft::client::gui::WorldGenerationProgressTracker { }
pub fn r#unknwnfn_3947477055(r#style : jni::object::JObject,r#unknwnarg_2485542753 : crate::root::minecraft::text::Style,) -> crate::root::minecraft::text::Style { }
pub fn r#isRealmsEnabled() -> bool { }
pub fn r#reloadResources(r#force : bool,) -> jni::object::JObject { }
pub fn r#addSystemDetailsToCrashReport(r#systemDetails : crate::root::minecraft::util::SystemDetails,r#client : crate::root::minecraft::client::MinecraftClient,r#languageManager : crate::root::minecraft::client::resource::language::LanguageManager,r#version : jni::object::JObject,r#options : crate::root::minecraft::client::option::GameOptions,) -> crate::root::minecraft::util::SystemDetails { }
pub fn r#saveProfilingResult(r#details : crate::root::minecraft::util::SystemDetails,r#files : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#unknwnfn_2656750743(r#files : crate::root::minecraft::util::SystemDetails,r#unknwnarg_2039466211 : jni::object::JObject,r#unknwnarg_3035911038 : jni::object::JObject,) -> () { }
pub fn r#unknwnfn_3012554936(r#result : crate::root::minecraft::util::profiler::ProfileResult,) -> () { }
pub fn r#unknwnfn_817939887(r#style : jni::object::JObject,r#unknwnarg_3013863238 : crate::root::minecraft::text::Style,) -> crate::root::minecraft::text::Style { }
pub fn r#unknwnfn_3039483169(r#path : jni::object::JObject,r#unknwnarg_1243366556 : jni::object::JObject,) -> () { }
pub fn r#stopRecorder() -> () { }
pub fn r#unknwnfn_718754336(r#result : jni::object::JObject,r#unknwnarg_3776413280 : crate::root::minecraft::util::profiler::ProfileResult,) -> () { }
pub fn r#unknwnfn_66635863(r#path : jni::object::JObject,r#unknwnarg_497345813 : jni::object::JObject,) -> () { }
pub fn r#createTelemetrySender() -> crate::root::minecraft::client::util::telemetry::TelemetrySender { }
pub fn r#loadBlockList() -> () { }
pub fn r#isCountrySetTo(r#country : jni::object::JObject,) -> bool { }
pub fn r#unknwnfn_100102479(r#glTimer : crate::root::minecraft::client::gl::GlTimer,) -> () { }
pub fn r#forceStopRecorder() -> () { }
pub fn r#getGpuUtilizationPercentage() -> f64 { }
pub fn r#createIntegratedServerLoader() -> crate::root::minecraft::server::integrated::IntegratedServerLoader { }
pub fn r#setCrashReportSupplier(r#crashReport : crate::root::minecraft::util::crash::CrashReport,) -> () { }
pub fn r#getProfileKeys() -> crate::root::minecraft::client::util::ProfileKeys { }
pub fn r#unknwnfn_1200383080(r#resultCollection : crate::root::minecraft::client::gui::screen::recipebook::RecipeResultCollection,) -> jni::object::JObject { }
pub fn r#reloadSearchProvider(r#key : crate::root::minecraft::client::search::Key,r#values : jni::object::JObject,) -> () { }
pub fn r#unknwnfn_2299752674(r#resultCollections : jni::object::JObject,) -> crate::root::minecraft::client::search::ReloadableSearchProvider { }
pub fn r#unknwnfn_4009845884(r#resultCollection : crate::root::minecraft::client::gui::screen::recipebook::RecipeResultCollection,) -> jni::object::JObject { }
pub fn r#unknwnfn_1104170969(r#stacks : jni::object::JObject,) -> crate::root::minecraft::client::search::ReloadableSearchProvider { }
pub fn r#unknwnfn_1925938909(r#text : jni::object::JObject,) -> bool { }
pub fn r#unknwnfn_700122638(r#stacks : jni::object::JObject,) -> crate::root::minecraft::client::search::ReloadableSearchProvider { }
pub fn r#unknwnfn_3247550999(r#text : crate::root::minecraft::text::Text,) -> jni::object::JObject { }
pub fn r#getRealms32BitWarningChecker() -> crate::root::minecraft::client::realms::util::Realms32BitWarningChecker { }
pub fn r#getServicesSignatureVerifier() -> crate::root::minecraft::network::encryption::SignatureVerifier { }
pub fn r#unknwnfn_997261011(r#confirmed : bool,) -> () { }
pub fn r#setCurrentServerEntry(r#server : crate::root::minecraft::client::realms::dto::RealmsServer,r#address : jni::object::JObject,) -> () { }
pub fn r#ensureAbuseReportContext(r#environment : crate::root::minecraft::client::report::ReporterEnvironment,) -> () { }
pub fn r#getAbuseReportContext() -> crate::root::minecraft::client::report::AbuseReportContext { }
pub fn r#getRealmsPeriodicCheckers() -> crate::root::minecraft::client::realms::RealmsPeriodicCheckers { }
pub fn r#getMultiplayerBanDetails() -> jni::object::JObject { }
pub fn r#unknwnfn_698910850(r#confirmed : bool,) -> () { }
pub fn r#isMultiplayerBanned() -> bool { }
pub fn r#getNarratorManager() -> crate::root::minecraft::client::util::NarratorManager { }
}
}
pub mod command {
pub mod argument {
pub struct r#AngleArgumentType<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for AngleArgumentType<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl AngleArgumentType<'_> {
const __map_sig : &str = "net/minecraft/class_5473";
/* jni::object::JObject = Lcom/mojang/brigadier/exceptions/SimpleCommandExceptionType; */
pub fn r#get_INCOMPLETE_ANGLE_EXCEPTION(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lcom/mojang/brigadier/exceptions/SimpleCommandExceptionType; */
pub fn r#set_INCOMPLETE_ANGLE_EXCEPTION(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/util/Collection; */
pub fn r#get_EXAMPLES(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/Collection; */
pub fn r#set_EXAMPLES(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Lcom/mojang/brigadier/exceptions/SimpleCommandExceptionType; */
pub fn r#get_INVALID_ANGLE_EXCEPTION(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lcom/mojang/brigadier/exceptions/SimpleCommandExceptionType; */
pub fn r#set_INVALID_ANGLE_EXCEPTION(&self,val : jni::object::JObject) -> () {} 
pub fn r#angle() -> crate::root::minecraft::command::argument::AngleArgumentType { }
pub fn r#getAngle(r#context : jni::object::JObject,r#name : jni::object::JObject,) -> f32 { }
}
}
pub struct r#BlockDataObject<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for BlockDataObject<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl BlockDataObject<'_> {
const __map_sig : &str = "net/minecraft/class_3161";
/* crate::root::minecraft::util::math::BlockPos = Lnet/minecraft/class_2338; */
pub fn r#get_pos(&self) -> Result<crate::root::minecraft::util::math::BlockPos,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::util::math::BlockPos = Lnet/minecraft/class_2338; */
pub fn r#set_pos(&self,val : crate::root::minecraft::util::math::BlockPos) -> () {} 
/* crate::root::minecraft::block::entity::BlockEntity = Lnet/minecraft/class_2586; */
pub fn r#get_blockEntity(&self) -> Result<crate::root::minecraft::block::entity::BlockEntity,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::block::entity::BlockEntity = Lnet/minecraft/class_2586; */
pub fn r#set_blockEntity(&self,val : crate::root::minecraft::block::entity::BlockEntity) -> () {} 
/* jni::object::JObject = Lcom/mojang/brigadier/exceptions/SimpleCommandExceptionType; */
pub fn r#get_INVALID_BLOCK_EXCEPTION(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lcom/mojang/brigadier/exceptions/SimpleCommandExceptionType; */
pub fn r#set_INVALID_BLOCK_EXCEPTION(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/util/function/Function; */
pub fn r#get_TYPE_FACTORY(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/function/Function; */
pub fn r#set_TYPE_FACTORY(&self,val : jni::object::JObject) -> () {} 
pub fn r#__zINDMINIT(r#blockEntity : crate::root::minecraft::block::entity::BlockEntity,r#pos : crate::root::minecraft::util::math::BlockPos,) -> () { }
}
}
pub mod data {
pub mod client {
pub struct r#BlockStateModelGenerator<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for BlockStateModelGenerator<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl BlockStateModelGenerator<'_> {
const __map_sig : &str = "net/minecraft/class_4910";
/* jni::object::JObject = Ljava/util/function/Consumer; */
pub fn r#get_blockStateCollector(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/function/Consumer; */
pub fn r#set_blockStateCollector(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/util/function/BiConsumer; */
pub fn r#get_modelCollector(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/function/BiConsumer; */
pub fn r#set_modelCollector(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/util/function/Consumer; */
pub fn r#get_simpleItemModelExemptionCollector(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/function/Consumer; */
pub fn r#set_simpleItemModelExemptionCollector(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/util/List; */
pub fn r#get_CONNECTION_VARIANT_FUNCTIONS(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/List; */
pub fn r#set_CONNECTION_VARIANT_FUNCTIONS(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/util/List; */
pub fn r#get_nonOrientableTrapdoors(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/List; */
pub fn r#set_nonOrientableTrapdoors(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/util/Map; */
pub fn r#get_sandstoneModels(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/Map; */
pub fn r#set_sandstoneModels(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/util/Map; */
pub fn r#get_VARIANT_POOL_FUNCTIONS(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/Map; */
pub fn r#set_VARIANT_POOL_FUNCTIONS(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/util/Map; */
pub fn r#get_stoneStateFactories(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/Map; */
pub fn r#set_stoneStateFactories(&self,val : jni::object::JObject) -> () {} 
pub fn r#__zINDMINIT(r#blockStateCollector : jni::object::JObject,r#modelCollector : jni::object::JObject,r#simpleItemModelExemptionCollector : jni::object::JObject,) -> () { }
pub fn r#registerChorusPlant() -> () { }
pub fn r#registerComposter() -> () { }
pub fn r#registerDaylightDetector() -> () { }
pub fn r#registerFarmland() -> () { }
pub fn r#registerFire() -> () { }
pub fn r#registerSoulFire() -> () { }
pub fn r#registerFrostedIce() -> () { }
pub fn r#registerTopSoils() -> () { }
pub fn r#registerCocoa() -> () { }
pub fn r#registerGrassPath() -> () { }
pub fn r#registerHopper() -> () { }
pub fn r#registerIronBars() -> () { }
pub fn r#registerLever() -> () { }
pub fn r#registerLilyPad() -> () { }
pub fn r#registerNetherPortal() -> () { }
pub fn r#registerNetherrack() -> () { }
pub fn r#registerObserver() -> () { }
pub fn r#registerPistons() -> () { }
pub fn r#registerPistonHead() -> () { }
pub fn r#registerScaffolding() -> () { }
pub fn r#registerRedstoneLamp() -> () { }
pub fn r#registerRedstoneTorch() -> () { }
pub fn r#registerRepeater() -> () { }
pub fn r#registerSeaPickle() -> () { }
pub fn r#registerSnows() -> () { }
pub fn r#register() -> () { }
pub fn r#getBambooBlockStateVariants(r#age : i32,) -> jni::object::JObject { }
pub fn r#getTurtleEggModel(r#eggs : i32,r#prefix : jni::object::JObject,r#textures : crate::root::minecraft::data::client::TextureMap,) -> crate::root::minecraft::util::Identifier { }
pub fn r#registerItemModel(r#item : crate::root::minecraft::item::Item,) -> () { }
pub fn r#registerParentedItemModel(r#item : crate::root::minecraft::item::Item,r#parentModelId : crate::root::minecraft::util::Identifier,) -> () { }
pub fn r#excludeFromSimpleItemModelGeneration(r#block : crate::root::minecraft::block::Block,) -> () { }
pub fn r#registerBuiltinWithParticle(r#block : crate::root::minecraft::block::Block,r#particleSource : crate::root::minecraft::item::Item,) -> () { }
pub fn r#registerStateWithModelReference(r#block : crate::root::minecraft::block::Block,r#modelReference : crate::root::minecraft::block::Block,) -> () { }
pub fn r#registerCoral(r#coral : crate::root::minecraft::block::Block,r#deadCoral : crate::root::minecraft::block::Block,r#coralBlock : crate::root::minecraft::block::Block,r#deadCoralBlock : crate::root::minecraft::block::Block,r#coralFan : crate::root::minecraft::block::Block,r#deadCoralFan : crate::root::minecraft::block::Block,r#coralWallFan : crate::root::minecraft::block::Block,r#deadCoralWallFan : crate::root::minecraft::block::Block,) -> () { }
pub fn r#registerFlowerPotPlant(r#plantBlock : crate::root::minecraft::block::Block,r#flowerPotBlock : crate::root::minecraft::block::Block,r#tintType : jni::object::JObject,) -> () { }
pub fn r#registerCubeWithCustomTextures(r#block : crate::root::minecraft::block::Block,r#otherTextureSource : crate::root::minecraft::block::Block,r#texturesFactory : jni::object::JObject,) -> () { }
pub fn r#registerCrop(r#crop : crate::root::minecraft::block::Block,r#ageProperty : crate::root::minecraft::state::property::Property,r#ageTextureIndices : i32,) -> () { }
pub fn r#registerTintableCross(r#block : crate::root::minecraft::block::Block,r#tintType : jni::object::JObject,) -> () { }
pub fn r#registerTintableCross(r#block : crate::root::minecraft::block::Block,r#tintType : jni::object::JObject,r#texture : crate::root::minecraft::data::client::TextureMap,) -> () { }
pub fn r#registerNorthDefaultHorizontalRotatable(r#block : crate::root::minecraft::block::Block,r#texture : crate::root::minecraft::data::client::TextureMap,) -> () { }
pub fn r#registerAxisRotated(r#block : crate::root::minecraft::block::Block,r#modelFactory : crate::root::minecraft::data::client::Factory,) -> () { }
pub fn r#registerAxisRotated(r#block : crate::root::minecraft::block::Block,r#verticalModelFactory : crate::root::minecraft::data::client::Factory,r#horizontalModelFactory : crate::root::minecraft::data::client::Factory,) -> () { }
pub fn r#registerItemModel(r#block : crate::root::minecraft::block::Block,r#textureSuffix : jni::object::JObject,) -> () { }
pub fn r#createSubModel(r#block : crate::root::minecraft::block::Block,r#suffix : jni::object::JObject,r#model : crate::root::minecraft::data::client::Model,r#texturesFactory : jni::object::JObject,) -> crate::root::minecraft::util::Identifier { }
pub fn r#registerBeehive(r#beehive : crate::root::minecraft::block::Block,r#texturesFactory : jni::object::JObject,) -> () { }
pub fn r#registerTopSoil(r#topSoil : crate::root::minecraft::block::Block,r#modelId : crate::root::minecraft::util::Identifier,r#snowyVariant : crate::root::minecraft::data::client::BlockStateVariant,) -> () { }
pub fn r#registerPiston(r#piston : crate::root::minecraft::block::Block,r#extendedModelId : crate::root::minecraft::util::Identifier,r#textures : crate::root::minecraft::data::client::TextureMap,) -> () { }
pub fn r#createBooleanModelMap(r#property : crate::root::minecraft::state::property::BooleanProperty,r#trueModel : crate::root::minecraft::util::Identifier,r#falseModel : crate::root::minecraft::util::Identifier,) -> crate::root::minecraft::data::client::BlockStateVariantMap { }
pub fn r#createValueFencedModelMap(r#property : crate::root::minecraft::state::property::Property,r#fence : jni::object::JObject,r#higherOrEqualModelId : crate::root::minecraft::util::Identifier,r#lowerModelId : crate::root::minecraft::util::Identifier,) -> crate::root::minecraft::data::client::BlockStateVariantMap { }
pub fn r#fillDoorVariantMap(r#variantMap : jni::object::JObject,r#targetHalf : crate::root::minecraft::block::enums::DoubleBlockHalf,r#unknwnarg_2920684523 : crate::root::minecraft::util::Identifier,r#unknwnarg_3504898755 : crate::root::minecraft::util::Identifier,r#unknwnarg_864267611 : crate::root::minecraft::util::Identifier,r#unknwnarg_2247530962 : crate::root::minecraft::util::Identifier,) -> jni::object::JObject { }
pub fn r#registerRandomHorizontalRotations(r#modelFactory : crate::root::minecraft::data::client::Factory,r#blocks : crate::root::minecraft::block::Block,) -> () { }
pub fn r#getTurtleEggModel(r#eggs : jni::object::JObject,r#hatch : jni::object::JObject,) -> crate::root::minecraft::util::Identifier { }
pub fn r#buildBlockStateVariants(r#modelIds : jni::object::JObject,r#processor : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#createModelVariantWithRandomHorizontalRotations(r#modelId : crate::root::minecraft::util::Identifier,) -> crate::root::minecraft::data::client::BlockStateVariant { }
pub fn r#registerBuiltin(r#modelId : crate::root::minecraft::util::Identifier,r#particleBlock : crate::root::minecraft::block::Block,) -> crate::root::minecraft::data::client::BuiltinModelPool { }
pub fn r#registerStonecutter() -> () { }
pub fn r#registerStructureBlock() -> () { }
pub fn r#registerSweetBerryBush() -> () { }
pub fn r#registerTripwire() -> () { }
pub fn r#registerTripwireHook() -> () { }
pub fn r#registerTurtleEgg() -> () { }
pub fn r#registerMagmaBlock() -> () { }
pub fn r#registerInfestedStone() -> () { }
pub fn r#createNorthDefaultHorizontalRotationStates() -> crate::root::minecraft::data::client::BlockStateVariantMap { }
pub fn r#registerItemModel(r#block : crate::root::minecraft::block::Block,) -> () { }
pub fn r#registerCoralFan(r#coralFanBlock : crate::root::minecraft::block::Block,r#coralWallFanBlock : crate::root::minecraft::block::Block,) -> () { }
pub fn r#registerPlantPart(r#plant : crate::root::minecraft::block::Block,r#plantStem : crate::root::minecraft::block::Block,r#tintType : jni::object::JObject,) -> () { }
pub fn r#registerTintableCrossBlockState(r#block : crate::root::minecraft::block::Block,r#tintType : jni::object::JObject,) -> () { }
pub fn r#registerTintableCrossBlockState(r#block : crate::root::minecraft::block::Block,r#tintType : jni::object::JObject,r#crossTexture : crate::root::minecraft::data::client::TextureMap,) -> () { }
pub fn r#registerNorthDefaultHorizontalRotated(r#block : crate::root::minecraft::block::Block,r#modelFactory : crate::root::minecraft::data::client::Factory,) -> () { }
pub fn r#createDoorBlockState(r#doorBlock : crate::root::minecraft::block::Block,r#bottomModelId : crate::root::minecraft::util::Identifier,r#bottomHingeModelId : crate::root::minecraft::util::Identifier,r#unknwnarg_1018490414 : crate::root::minecraft::util::Identifier,r#unknwnarg_3390517524 : crate::root::minecraft::util::Identifier,r#unknwnarg_3808154981 : crate::root::minecraft::util::Identifier,r#unknwnarg_483979231 : crate::root::minecraft::util::Identifier,r#unknwnarg_1048968885 : crate::root::minecraft::util::Identifier,r#unknwnarg_915623502 : crate::root::minecraft::util::Identifier,) -> crate::root::minecraft::data::client::BlockStateSupplier { }
pub fn r#registerSouthDefaultHorizontalFacing(r#modelFactory : crate::root::minecraft::data::client::Factory,r#blocks : crate::root::minecraft::block::Block,) -> () { }
pub fn r#createSouthDefaultHorizontalRotationStates() -> crate::root::minecraft::data::client::BlockStateVariantMap { }
pub fn r#registerMirrorable(r#block : crate::root::minecraft::block::Block,) -> () { }
pub fn r#registerGourd(r#stemBlock : crate::root::minecraft::block::Block,r#attachedStemBlock : crate::root::minecraft::block::Block,) -> () { }
pub fn r#registerDoubleBlock(r#doubleBlock : crate::root::minecraft::block::Block,r#tintType : jni::object::JObject,) -> () { }
pub fn r#registerSingleton(r#block : crate::root::minecraft::block::Block,r#modelFactory : crate::root::minecraft::data::client::Factory,) -> () { }
pub fn r#registerParentedItemModel(r#block : crate::root::minecraft::block::Block,r#parentModelId : crate::root::minecraft::util::Identifier,) -> () { }
pub fn r#createFenceGateBlockState(r#fenceGateBlock : crate::root::minecraft::block::Block,r#openModelId : crate::root::minecraft::util::Identifier,r#closedModelId : crate::root::minecraft::util::Identifier,r#openWallModelId : crate::root::minecraft::util::Identifier,r#closedWallModelId : crate::root::minecraft::util::Identifier,) -> crate::root::minecraft::data::client::BlockStateSupplier { }
pub fn r#createEastDefaultHorizontalRotationStates() -> crate::root::minecraft::data::client::BlockStateVariantMap { }
pub fn r#registerRotatable(r#block : crate::root::minecraft::block::Block,) -> () { }
pub fn r#registerBuiltin(r#block : crate::root::minecraft::block::Block,r#particleBlock : crate::root::minecraft::block::Block,) -> crate::root::minecraft::data::client::BuiltinModelPool { }
pub fn r#createBlockStateWithRandomHorizontalRotations(r#block : crate::root::minecraft::block::Block,r#modelId : crate::root::minecraft::util::Identifier,) -> crate::root::minecraft::data::client::VariantsBlockStateSupplier { }
pub fn r#createWallBlockState(r#wallBlock : crate::root::minecraft::block::Block,r#postModelId : crate::root::minecraft::util::Identifier,r#lowSideModelId : crate::root::minecraft::util::Identifier,r#tallSideModelId : crate::root::minecraft::util::Identifier,) -> crate::root::minecraft::data::client::BlockStateSupplier { }
pub fn r#createNorthDefaultRotationStates() -> crate::root::minecraft::data::client::BlockStateVariantMap { }
pub fn r#registerSimpleCubeAll(r#block : crate::root::minecraft::block::Block,) -> () { }
pub fn r#registerWoolAndCarpet(r#wool : crate::root::minecraft::block::Block,r#carpet : crate::root::minecraft::block::Block,) -> () { }
pub fn r#createSingletonBlockState(r#block : crate::root::minecraft::block::Block,r#modelId : crate::root::minecraft::util::Identifier,) -> crate::root::minecraft::data::client::VariantsBlockStateSupplier { }
pub fn r#createBlockStateWithTwoModelAndRandomInversion(r#block : crate::root::minecraft::block::Block,r#firstModelId : crate::root::minecraft::util::Identifier,r#secondModelId : crate::root::minecraft::util::Identifier,) -> crate::root::minecraft::data::client::VariantsBlockStateSupplier { }
pub fn r#createStairsBlockState(r#stairsBlock : crate::root::minecraft::block::Block,r#innerModelId : crate::root::minecraft::util::Identifier,r#regularModelId : crate::root::minecraft::util::Identifier,r#outerModelId : crate::root::minecraft::util::Identifier,) -> crate::root::minecraft::data::client::BlockStateSupplier { }
pub fn r#createAxisRotatedVariantMap() -> crate::root::minecraft::data::client::BlockStateVariantMap { }
pub fn r#registerCubeAllModelTexturePool(r#block : crate::root::minecraft::block::Block,) -> jni::object::JObject { }
pub fn r#registerGlassPane(r#glass : crate::root::minecraft::block::Block,r#glassPane : crate::root::minecraft::block::Block,) -> () { }
pub fn r#registerCooker(r#cooker : crate::root::minecraft::block::Block,r#modelFactory : crate::root::minecraft::data::client::Factory,) -> () { }
pub fn r#createAxisRotatedBlockState(r#block : crate::root::minecraft::block::Block,r#modelId : crate::root::minecraft::util::Identifier,) -> crate::root::minecraft::data::client::BlockStateSupplier { }
pub fn r#createButtonBlockState(r#buttonBlock : crate::root::minecraft::block::Block,r#regularModelId : crate::root::minecraft::util::Identifier,r#pressedModelId : crate::root::minecraft::util::Identifier,) -> crate::root::minecraft::data::client::BlockStateSupplier { }
pub fn r#createOrientableTrapdoorBlockState(r#trapdoorBlock : crate::root::minecraft::block::Block,r#topModelId : crate::root::minecraft::util::Identifier,r#bottomModelId : crate::root::minecraft::util::Identifier,r#openModelId : crate::root::minecraft::util::Identifier,) -> crate::root::minecraft::data::client::BlockStateSupplier { }
pub fn r#registerSunflower() -> () { }
pub fn r#registerDoor(r#doorBlock : crate::root::minecraft::block::Block,) -> () { }
pub fn r#registerBuiltinWithParticle(r#block : crate::root::minecraft::block::Block,r#particleSource : crate::root::minecraft::util::Identifier,) -> () { }
pub fn r#createFenceBlockState(r#fenceBlock : crate::root::minecraft::block::Block,r#postModelId : crate::root::minecraft::util::Identifier,r#sideModelId : crate::root::minecraft::util::Identifier,) -> crate::root::minecraft::data::client::BlockStateSupplier { }
pub fn r#createTrapdoorBlockState(r#trapdoorBlock : crate::root::minecraft::block::Block,r#topModelId : crate::root::minecraft::util::Identifier,r#bottomModelId : crate::root::minecraft::util::Identifier,r#openModelId : crate::root::minecraft::util::Identifier,) -> crate::root::minecraft::data::client::BlockStateSupplier { }
pub fn r#registerTallSeagrass() -> () { }
pub fn r#registerOrientableTrapdoor(r#trapdoorBlock : crate::root::minecraft::block::Block,) -> () { }
pub fn r#registerPressurePlate(r#pressurePlate : crate::root::minecraft::block::Block,r#textureSource : crate::root::minecraft::block::Block,) -> () { }
pub fn r#createAxisRotatedBlockState(r#block : crate::root::minecraft::block::Block,r#verticalModelId : crate::root::minecraft::util::Identifier,r#horizontalModelId : crate::root::minecraft::util::Identifier,) -> crate::root::minecraft::data::client::BlockStateSupplier { }
pub fn r#createSlabBlockState(r#slabBlock : crate::root::minecraft::block::Block,r#bottomModelId : crate::root::minecraft::util::Identifier,r#topModelId : crate::root::minecraft::util::Identifier,r#fullModelId : crate::root::minecraft::util::Identifier,) -> crate::root::minecraft::data::client::BlockStateSupplier { }
pub fn r#registerBamboo() -> () { }
pub fn r#registerTrapdoor(r#trapdoorBlock : crate::root::minecraft::block::Block,) -> () { }
pub fn r#registerParented(r#modelSource : crate::root::minecraft::block::Block,r#child : crate::root::minecraft::block::Block,) -> () { }
pub fn r#createPressurePlateBlockState(r#pressurePlateBlock : crate::root::minecraft::block::Block,r#upModelId : crate::root::minecraft::util::Identifier,r#downModelId : crate::root::minecraft::util::Identifier,) -> crate::root::minecraft::data::client::BlockStateSupplier { }
pub fn r#createUpDefaultFacingVariantMap() -> crate::root::minecraft::data::client::BlockStateVariantMap { }
pub fn r#registerLog(r#logBlock : crate::root::minecraft::block::Block,) -> jni::object::JObject { }
pub fn r#registerTorch(r#torch : crate::root::minecraft::block::Block,r#wallTorch : crate::root::minecraft::block::Block,) -> () { }
pub fn r#registerDoubleBlock(r#block : crate::root::minecraft::block::Block,r#upperHalfModelId : crate::root::minecraft::util::Identifier,r#lowerHalfModelId : crate::root::minecraft::util::Identifier,) -> () { }
pub fn r#registerBarrel() -> () { }
pub fn r#registerSimpleState(r#block : crate::root::minecraft::block::Block,) -> () { }
pub fn r#registerBed(r#bed : crate::root::minecraft::block::Block,r#particleSource : crate::root::minecraft::block::Block,) -> () { }
pub fn r#registerBell() -> () { }
pub fn r#registerTurnableRail(r#rail : crate::root::minecraft::block::Block,) -> () { }
pub fn r#registerRoots(r#root : crate::root::minecraft::block::Block,r#pottedRoot : crate::root::minecraft::block::Block,) -> () { }
pub fn r#registerGrindstone() -> () { }
pub fn r#registerStraightRail(r#rail : crate::root::minecraft::block::Block,) -> () { }
pub fn r#registerCommandBlock(r#commandBlock : crate::root::minecraft::block::Block,) -> () { }
pub fn r#registerBookshelf() -> () { }
pub fn r#registerAnvil(r#anvil : crate::root::minecraft::block::Block,) -> () { }
pub fn r#registerRedstone() -> () { }
pub fn r#registerMushroomBlock(r#mushroomBlock : crate::root::minecraft::block::Block,) -> () { }
pub fn r#registerComparator() -> () { }
pub fn r#registerDispenserLikeOrientable(r#block : crate::root::minecraft::block::Block,) -> () { }
pub fn r#registerSmoothStone() -> () { }
pub fn r#registerNetherrackBottomCustomTop(r#block : crate::root::minecraft::block::Block,) -> () { }
pub fn r#registerBrewingStand() -> () { }
pub fn r#getFireFloorModels(r#texture : crate::root::minecraft::block::Block,) -> jni::object::JObject { }
pub fn r#registerCake() -> () { }
pub fn r#getFireSideModels(r#texture : crate::root::minecraft::block::Block,) -> jni::object::JObject { }
pub fn r#registerCartographyTable() -> () { }
pub fn r#getFireUpModels(r#texture : crate::root::minecraft::block::Block,) -> jni::object::JObject { }
pub fn r#registerSmithingTable() -> () { }
pub fn r#registerLantern(r#lantern : crate::root::minecraft::block::Block,) -> () { }
pub fn r#registerPumpkins() -> () { }
pub fn r#registerNorthDefaultHorizontalRotation(r#block : crate::root::minecraft::block::Block,) -> () { }
pub fn r#registerCauldrons() -> () { }
pub fn r#registerShulkerBox(r#shulkerBox : crate::root::minecraft::block::Block,) -> () { }
pub fn r#registerChorusFlower() -> () { }
pub fn r#registerEndPortalFrame() -> () { }
pub fn r#registerRespawnAnchor() -> () { }
pub fn r#addJigsawOrientationToVariant(r#orientation : crate::root::minecraft::block::enums::JigsawOrientation,r#variant : crate::root::minecraft::data::client::BlockStateVariant,) -> crate::root::minecraft::data::client::BlockStateVariant { }
pub fn r#registerJigsaw() -> () { }
pub fn r#registerCampfire(r#blocks : crate::root::minecraft::block::Block,) -> () { }
pub fn r#registerAxisRotated(r#block : crate::root::minecraft::block::Block,r#modelId : crate::root::minecraft::util::Identifier,) -> () { }
pub fn r#registerRod(r#block : crate::root::minecraft::block::Block,) -> () { }
pub fn r#registerAmethysts() -> () { }
pub fn r#registerCandle(r#candle : crate::root::minecraft::block::Block,r#cake : crate::root::minecraft::block::Block,) -> () { }
pub fn r#registerAmethyst(r#block : crate::root::minecraft::block::Block,) -> () { }
pub fn r#registerPointedDripstone() -> () { }
pub fn r#getDripstoneVariant(r#direction : crate::root::minecraft::util::math::Direction,r#thickness : crate::root::minecraft::block::enums::Thickness,) -> crate::root::minecraft::data::client::BlockStateVariant { }
pub fn r#registerSculkSensor() -> () { }
pub fn r#registerPetrifiedOakSlab() -> () { }
pub fn r#registerWallPlant(r#block : crate::root::minecraft::block::Block,) -> () { }
pub fn r#registerCaveVines() -> () { }
pub fn r#registerBigDripleaf() -> () { }
pub fn r#registerSmallDripleaf() -> () { }
pub fn r#registerAzalea(r#block : crate::root::minecraft::block::Block,) -> () { }
pub fn r#unknwnfn_3545228858(r#item : crate::root::minecraft::item::SpawnEggItem,) -> () { }
pub fn r#unknwnfn_2734838572(r#property : crate::root::minecraft::block::Block,r#unknwnarg_4293075483 : jni::object::JObject,r#unknwnarg_1068987616 : crate::root::minecraft::state::property::BooleanProperty,) -> () { }
pub fn r#createStoneState(r#block : crate::root::minecraft::block::Block,r#modelId : crate::root::minecraft::util::Identifier,r#textures : crate::root::minecraft::data::client::TextureMap,r#modelCollector : jni::object::JObject,) -> crate::root::minecraft::data::client::BlockStateSupplier { }
pub fn r#unknwnfn_4263910597(r#mode : crate::root::minecraft::block::enums::StructureBlockMode,) -> crate::root::minecraft::data::client::BlockStateVariant { }
pub fn r#unknwnfn_1205769346(r#family : crate::root::minecraft::data::family::BlockFamily,) -> () { }
pub fn r#unknwnfn_3614093286(r#stage : jni::object::JObject,) -> crate::root::minecraft::data::client::BlockStateVariant { }
pub fn r#unknwnfn_2591566589(r#tick : jni::object::JObject,r#locked : jni::object::JObject,r#on : jni::object::JObject,) -> crate::root::minecraft::data::client::BlockStateVariant { }
pub fn r#unknwnfn_1014246117(r#textures : crate::root::minecraft::data::client::TextureMap,) -> () { }
pub fn r#unknwnfn_2553349798(r#id : crate::root::minecraft::data::client::TextureMap,r#unknwnarg_1665245928 : crate::root::minecraft::util::Identifier,) -> crate::root::minecraft::data::client::TextureMap { }
pub fn r#unknwnfn_3939605808(r#height : crate::root::minecraft::util::Identifier,r#unknwnarg_96858252 : jni::object::JObject,) -> crate::root::minecraft::data::client::BlockStateVariant { }
pub fn r#unknwnfn_2770461798(r#textures : crate::root::minecraft::util::Identifier,r#unknwnarg_1091921495 : crate::root::minecraft::data::client::TextureMap,) -> () { }
pub fn r#unknwnfn_3943069677(r#on : crate::root::minecraft::util::Identifier,r#shape : crate::root::minecraft::util::Identifier,r#unknwnarg_804862456 : crate::root::minecraft::util::Identifier,r#unknwnarg_206974026 : crate::root::minecraft::util::Identifier,r#unknwnarg_2819237262 : crate::root::minecraft::util::Identifier,r#unknwnarg_1933226756 : crate::root::minecraft::util::Identifier,r#unknwnarg_3437641966 : jni::object::JObject,r#unknwnarg_2393640965 : crate::root::minecraft::block::enums::RailShape,) -> crate::root::minecraft::data::client::BlockStateVariant { }
pub fn r#createDeepslateState(r#block : crate::root::minecraft::block::Block,r#modelId : crate::root::minecraft::util::Identifier,r#textures : crate::root::minecraft::data::client::TextureMap,r#modelCollector : jni::object::JObject,) -> crate::root::minecraft::data::client::BlockStateSupplier { }
pub fn r#unknwnfn_4086582013(r#textures : crate::root::minecraft::data::client::TextureMap,) -> () { }
pub fn r#unknwnfn_2744660880(r#id : crate::root::minecraft::data::client::TextureMap,r#unknwnarg_2099528525 : crate::root::minecraft::util::Identifier,) -> crate::root::minecraft::data::client::TextureMap { }
pub fn r#unknwnfn_843798168(r#textures : crate::root::minecraft::util::Identifier,r#unknwnarg_4252494136 : crate::root::minecraft::data::client::TextureMap,) -> () { }
pub fn r#unknwnfn_925480896(r#textures : crate::root::minecraft::util::Identifier,r#unknwnarg_3130472702 : crate::root::minecraft::data::client::TextureMap,) -> () { }
pub fn r#unknwnfn_1563627671(r#modelId : crate::root::minecraft::util::Identifier,) -> crate::root::minecraft::data::client::BlockStateVariant { }
pub fn r#registerLightningRod() -> () { }
pub fn r#registerSingleton(r#block : crate::root::minecraft::block::Block,r#textures : crate::root::minecraft::data::client::TextureMap,r#model : crate::root::minecraft::data::client::Model,) -> () { }
pub fn r#registerInfestedDeepslate() -> () { }
pub fn r#registerPottedAzaleaBush(r#block : crate::root::minecraft::block::Block,) -> () { }
pub fn r#registerLightBlock() -> () { }
pub fn r#registerMuddyMangroveRoots() -> () { }
pub fn r#registerMangrovePropagule() -> () { }
pub fn r#registerFrogspawn() -> () { }
pub fn r#registerSculkShrieker() -> () { }
pub fn r#registerSculkCatalyst() -> () { }
}
}
pub struct r#DataCache<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for DataCache<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl DataCache<'_> {
const __map_sig : &str = "net/minecraft/class_2408";
/* jni::object::JObject = Ljava/nio/file/Path; */
pub fn r#get_root(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/nio/file/Path; */
pub fn r#set_root(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Lorg/slf4j/Logger; */
pub fn r#get_LOGGER(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lorg/slf4j/Logger; */
pub fn r#set_LOGGER(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#get_HEADER(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#set_HEADER(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/nio/file/Path; */
pub fn r#get_cachePath(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/nio/file/Path; */
pub fn r#set_cachePath(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#get_versionName(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#set_versionName(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/util/Map; */
pub fn r#get_cachedDatas(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/Map; */
pub fn r#set_cachedDatas(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/util/Map; */
pub fn r#get_dataWriters(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/Map; */
pub fn r#set_dataWriters(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/util/Set; */
pub fn r#get_paths(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/Set; */
pub fn r#set_paths(&self,val : jni::object::JObject) -> () {} 
/* i32 = I */
pub fn r#get_totalSize(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_totalSize(&self,val : i32) -> () {} 
pub fn r#__zINDMINIT(r#root : jni::object::JObject,r#dataProviders : jni::object::JObject,r#gameVersion : crate::root::minecraft::GameVersion,) -> () { }
pub fn r#write() -> () { }
pub fn r#isVersionDifferent(r#dataProvider : crate::root::minecraft::data::DataProvider,) -> bool { }
pub fn r#parseOrCreateCache(r#root : jni::object::JObject,r#dataProviderPath : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#unknwnfn_553405271(r#cachedData : jni::object::JObject,r#unknwnarg_53193948 : jni::object::JObject,) -> () { }
pub fn r#unknwnfn_4082769789(r#dataProvider : jni::object::JObject,r#writer : crate::root::minecraft::data::DataProvider,r#unknwnarg_2850666268 : crate::root::minecraft::data::CachedDataWriter,) -> () { }
pub fn r#unknwnfn_1769719062(r#path : jni::object::JObject,r#unknwnarg_3060603874 : jni::object::JObject,r#unknwnarg_861961476 : jni::object::JObject,r#unknwnarg_758232056 : jni::object::JObject,) -> () { }
pub fn r#getOrCreateWriter(r#dataProvider : crate::root::minecraft::data::DataProvider,) -> crate::root::minecraft::data::DataWriter { }
pub fn r#getPath(r#dataProvider : crate::root::minecraft::data::DataProvider,) -> jni::object::JObject { }
}
}
pub mod datafixer {
pub mod fix {
pub struct r#AddTrappedChestFix<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for AddTrappedChestFix<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl AddTrappedChestFix<'_> {
const __map_sig : &str = "net/minecraft/class_1215";
/* jni::object::JObject = Lorg/slf4j/Logger; */
pub fn r#get_LOGGER(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lorg/slf4j/Logger; */
pub fn r#set_LOGGER(&self,val : jni::object::JObject) -> () {} 
}
}
pub mod mapping {
}
pub mod schema {
}
}
pub mod enchantment {
pub struct r#Enchantment<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for Enchantment<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl Enchantment<'_> {
const __map_sig : &str = "net/minecraft/class_1887";
/* crate::root::minecraft::enchantment::EnchantmentTarget = Lnet/minecraft/class_1886; */
pub fn r#get_type(&self) -> Result<crate::root::minecraft::enchantment::EnchantmentTarget,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::enchantment::EnchantmentTarget = Lnet/minecraft/class_1886; */
pub fn r#set_type(&self,val : crate::root::minecraft::enchantment::EnchantmentTarget) -> () {} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#get_translationKey(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#set_translationKey(&self,val : jni::object::JObject) -> () {} 
/* crate::root::minecraft::enchantment::Rarity = Lnet/minecraft/class_1887$class_1888; */
pub fn r#get_rarity(&self) -> Result<crate::root::minecraft::enchantment::Rarity,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::enchantment::Rarity = Lnet/minecraft/class_1887$class_1888; */
pub fn r#set_rarity(&self,val : crate::root::minecraft::enchantment::Rarity) -> () {} 
/* std::vec::Vec<crate::root::minecraft::entity::EquipmentSlot> = [Lnet/minecraft/class_1304; */
pub fn r#get_slotTypes(&self) -> Result<std::vec::Vec<crate::root::minecraft::entity::EquipmentSlot>,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* std::vec::Vec<crate::root::minecraft::entity::EquipmentSlot> = [Lnet/minecraft/class_1304; */
pub fn r#set_slotTypes(&self,val : std::vec::Vec<crate::root::minecraft::entity::EquipmentSlot>) -> () {} 
pub fn r#__zINDMINIT(r#weight : crate::root::minecraft::enchantment::Rarity,r#type : crate::root::minecraft::enchantment::EnchantmentTarget,r#slotTypes : crate::root::minecraft::entity::EquipmentSlot,) -> () { }
pub fn r#getMaxPower(r#level : i32,) -> i32 { }
pub fn r#isAvailableForEnchantedBookOffer() -> bool { }
pub fn r#isAvailableForRandomSelection() -> bool { }
pub fn r#onUserDamaged(r#user : crate::root::minecraft::entity::LivingEntity,r#attacker : crate::root::minecraft::entity::Entity,r#level : i32,) -> () { }
pub fn r#getName(r#level : i32,) -> crate::root::minecraft::text::Text { }
pub fn r#canAccept(r#other : crate::root::minecraft::enchantment::Enchantment,) -> bool { }
pub fn r#getProtectionAmount(r#level : i32,r#source : crate::root::minecraft::entity::damage::DamageSource,) -> i32 { }
pub fn r#getMinPower(r#level : i32,) -> i32 { }
pub fn r#getMaxLevel() -> i32 { }
pub fn r#getTranslationKey() -> jni::object::JObject { }
pub fn r#getEquipment(r#entity : crate::root::minecraft::entity::LivingEntity,) -> jni::object::JObject { }
pub fn r#getRarity() -> crate::root::minecraft::enchantment::Rarity { }
pub fn r#getMinLevel() -> i32 { }
pub fn r#canCombine(r#other : crate::root::minecraft::enchantment::Enchantment,) -> bool { }
pub fn r#onTargetDamaged(r#user : crate::root::minecraft::entity::LivingEntity,r#target : crate::root::minecraft::entity::Entity,r#level : i32,) -> () { }
pub fn r#getOrCreateTranslationKey() -> jni::object::JObject { }
pub fn r#byRawId(r#id : i32,) -> crate::root::minecraft::enchantment::Enchantment { }
pub fn r#isAcceptableItem(r#stack : crate::root::minecraft::item::ItemStack,) -> bool { }
pub fn r#isTreasure() -> bool { }
pub fn r#isCursed() -> bool { }
}
}
pub mod entity {
pub mod ai {
pub mod brain {
pub struct r#Brain<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for Brain<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl Brain<'_> {
const __map_sig : &str = "net/minecraft/class_4095";
/* jni::object::JObject = Ljava/util/Map; */
pub fn r#get_memories(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/Map; */
pub fn r#set_memories(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/util/Map; */
pub fn r#get_sensors(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/Map; */
pub fn r#set_sensors(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/util/Map; */
pub fn r#get_tasks(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/Map; */
pub fn r#set_tasks(&self,val : jni::object::JObject) -> () {} 
/* crate::root::minecraft::entity::ai::brain::Schedule = Lnet/minecraft/class_4170; */
pub fn r#get_schedule(&self) -> Result<crate::root::minecraft::entity::ai::brain::Schedule,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::entity::ai::brain::Schedule = Lnet/minecraft/class_4170; */
pub fn r#set_schedule(&self,val : crate::root::minecraft::entity::ai::brain::Schedule) -> () {} 
/* jni::object::JObject = Ljava/util/Map; */
pub fn r#get_requiredActivityMemories(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/Map; */
pub fn r#set_requiredActivityMemories(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/util/Set; */
pub fn r#get_coreActivities(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/Set; */
pub fn r#set_coreActivities(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/util/Set; */
pub fn r#get_possibleActivities(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/Set; */
pub fn r#set_possibleActivities(&self,val : jni::object::JObject) -> () {} 
/* crate::root::minecraft::entity::ai::brain::Activity = Lnet/minecraft/class_4168; */
pub fn r#get_defaultActivity(&self) -> Result<crate::root::minecraft::entity::ai::brain::Activity,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::entity::ai::brain::Activity = Lnet/minecraft/class_4168; */
pub fn r#set_defaultActivity(&self,val : crate::root::minecraft::entity::ai::brain::Activity) -> () {} 
/* i64 = J */
pub fn r#get_activityStartTime(&self) -> Result<i64,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i64 = J */
pub fn r#set_activityStartTime(&self,val : i64) -> () {} 
/* jni::object::JObject = Ljava/util/Map; */
pub fn r#get_forgettingActivityMemories(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/Map; */
pub fn r#set_forgettingActivityMemories(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Lorg/slf4j/Logger; */
pub fn r#get_LOGGER(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lorg/slf4j/Logger; */
pub fn r#set_LOGGER(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/util/function/Supplier; */
pub fn r#get_codecSupplier(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/function/Supplier; */
pub fn r#set_codecSupplier(&self,val : jni::object::JObject) -> () {} 
/* i32 = I */
pub fn r#get_ACTIVITY_REFRESH_COOLDOWN(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_ACTIVITY_REFRESH_COOLDOWN(&self,val : i32) -> () {} 
pub fn r#__zINDMINIT(r#memories : jni::object::JObject,r#sensors : jni::object::JObject,r#memoryEntries : jni::object::JObject,r#codecSupplier : jni::object::JObject,) -> () { }
pub fn r#refreshActivities(r#timeOfDay : i64,r#time : i64,) -> () { }
pub fn r#canDoActivity(r#activity : crate::root::minecraft::entity::ai::brain::Activity,) -> bool { }
pub fn r#forget(r#type : crate::root::minecraft::entity::ai::brain::MemoryModuleType,) -> () { }
pub fn r#isMemoryInState(r#type : crate::root::minecraft::entity::ai::brain::MemoryModuleType,r#state : crate::root::minecraft::entity::ai::brain::MemoryModuleState,) -> bool { }
pub fn r#remember(r#type : crate::root::minecraft::entity::ai::brain::MemoryModuleType,r#value : jni::object::JObject,) -> () { }
pub fn r#remember(r#type : crate::root::minecraft::entity::ai::brain::MemoryModuleType,r#value : jni::object::JObject,) -> () { }
pub fn r#resetPossibleActivities(r#except : crate::root::minecraft::entity::ai::brain::Activity,) -> () { }
pub fn r#setTaskList(r#activity : crate::root::minecraft::entity::ai::brain::Activity,r#indexedTasks : jni::object::JObject,) -> () { }
pub fn r#setTaskList(r#activity : crate::root::minecraft::entity::ai::brain::Activity,r#begin : i32,r#list : jni::object::JObject,) -> () { }
pub fn r#setSchedule(r#schedule : crate::root::minecraft::entity::ai::brain::Schedule,) -> () { }
pub fn r#setCoreActivities(r#coreActivities : jni::object::JObject,) -> () { }
pub fn r#startTasks(r#world : crate::root::minecraft::server::world::ServerWorld,r#entity : crate::root::minecraft::entity::LivingEntity,) -> () { }
pub fn r#getSchedule() -> crate::root::minecraft::entity::ai::brain::Schedule { }
pub fn r#hasMemoryModule(r#type : crate::root::minecraft::entity::ai::brain::MemoryModuleType,) -> bool { }
pub fn r#setDefaultActivity(r#activity : crate::root::minecraft::entity::ai::brain::Activity,) -> () { }
pub fn r#stopAllTasks(r#world : crate::root::minecraft::server::world::ServerWorld,r#entity : crate::root::minecraft::entity::LivingEntity,) -> () { }
pub fn r#getOptionalMemory(r#type : crate::root::minecraft::entity::ai::brain::MemoryModuleType,) -> jni::object::JObject { }
pub fn r#hasActivity(r#activity : crate::root::minecraft::entity::ai::brain::Activity,) -> bool { }
pub fn r#copy() -> crate::root::minecraft::entity::ai::brain::Brain { }
pub fn r#tick(r#world : crate::root::minecraft::server::world::ServerWorld,r#entity : crate::root::minecraft::entity::LivingEntity,) -> () { }
pub fn r#updateTasks(r#world : crate::root::minecraft::server::world::ServerWorld,r#entity : crate::root::minecraft::entity::LivingEntity,) -> () { }
pub fn r#isEmptyCollection(r#value : jni::object::JObject,) -> bool { }
pub fn r#indexTaskList(r#begin : i32,r#tasks : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#remember(r#type : crate::root::minecraft::entity::ai::brain::MemoryModuleType,r#value : jni::object::JObject,r#startTime : i64,) -> () { }
pub fn r#doExclusively(r#activity : crate::root::minecraft::entity::ai::brain::Activity,) -> () { }
pub fn r#setTaskList(r#activity : crate::root::minecraft::entity::ai::brain::Activity,r#begin : i32,r#tasks : jni::object::JObject,r#memoryType : crate::root::minecraft::entity::ai::brain::MemoryModuleType,) -> () { }
pub fn r#setTaskList(r#activity : crate::root::minecraft::entity::ai::brain::Activity,r#indexedTasks : jni::object::JObject,r#requiredMemories : jni::object::JObject,) -> () { }
pub fn r#setTaskList(r#activity : crate::root::minecraft::entity::ai::brain::Activity,r#indexedTasks : jni::object::JObject,r#requiredMemories : jni::object::JObject,r#forgettingMemories : jni::object::JObject,) -> () { }
pub fn r#resetPossibleActivities(r#activities : jni::object::JObject,) -> () { }
pub fn r#setMemory(r#type : crate::root::minecraft::entity::ai::brain::MemoryModuleType,r#memory : jni::object::JObject,) -> () { }
pub fn r#resetPossibleActivities() -> () { }
pub fn r#forgetIrrelevantMemories(r#except : crate::root::minecraft::entity::ai::brain::Activity,) -> () { }
pub fn r#getFirstPossibleNonCoreActivity() -> jni::object::JObject { }
pub fn r#tickSensors(r#world : crate::root::minecraft::server::world::ServerWorld,r#entity : crate::root::minecraft::entity::LivingEntity,) -> () { }
pub fn r#getRunningTasks() -> jni::object::JObject { }
pub fn r#tickMemories() -> () { }
pub fn r#encode(r#ops : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#createProfile(r#memoryModules : jni::object::JObject,r#sensors : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#createBrainCodec(r#memoryModules : jni::object::JObject,r#sensors : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#streamMemories() -> jni::object::JObject { }
pub fn r#hasMemoryModuleWithValue(r#type : crate::root::minecraft::entity::ai::brain::MemoryModuleType,r#value : jni::object::JObject,) -> bool { }
pub fn r#getMemories() -> jni::object::JObject { }
pub fn r#getPossibleActivities() -> jni::object::JObject { }
pub fn r#clear() -> () { }
}
}
pub mod control {
pub struct r#MoveControl<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for MoveControl<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl MoveControl<'_> {
const __map_sig : &str = "net/minecraft/class_1335";
/* f32 = F */
pub fn r#get_REACHED_DESTINATION_DISTANCE_SQUARED(&self) -> Result<f32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f32 = F */
pub fn r#set_REACHED_DESTINATION_DISTANCE_SQUARED(&self,val : f32) -> () {} 
/* f64 = D */
pub fn r#get_targetZ(&self) -> Result<f64,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f64 = D */
pub fn r#set_targetZ(&self,val : f64) -> () {} 
/* f32 = F */
pub fn r#get_forwardMovement(&self) -> Result<f32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f32 = F */
pub fn r#set_forwardMovement(&self,val : f32) -> () {} 
/* f64 = D */
pub fn r#get_targetY(&self) -> Result<f64,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f64 = D */
pub fn r#set_targetY(&self,val : f64) -> () {} 
/* f64 = D */
pub fn r#get_targetX(&self) -> Result<f64,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f64 = D */
pub fn r#set_targetX(&self,val : f64) -> () {} 
/* crate::root::minecraft::entity::mob::MobEntity = Lnet/minecraft/class_1308; */
pub fn r#get_entity(&self) -> Result<crate::root::minecraft::entity::mob::MobEntity,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::entity::mob::MobEntity = Lnet/minecraft/class_1308; */
pub fn r#set_entity(&self,val : crate::root::minecraft::entity::mob::MobEntity) -> () {} 
/* f64 = D */
pub fn r#get_speed(&self) -> Result<f64,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f64 = D */
pub fn r#set_speed(&self,val : f64) -> () {} 
/* f32 = F */
pub fn r#get_sidewaysMovement(&self) -> Result<f32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f32 = F */
pub fn r#set_sidewaysMovement(&self,val : f32) -> () {} 
/* crate::root::minecraft::entity::ai::control::State = Lnet/minecraft/class_1335$class_1336; */
pub fn r#get_state(&self) -> Result<crate::root::minecraft::entity::ai::control::State,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::entity::ai::control::State = Lnet/minecraft/class_1335$class_1336; */
pub fn r#set_state(&self,val : crate::root::minecraft::entity::ai::control::State) -> () {} 
pub fn r#__zINDMINIT(r#entity : crate::root::minecraft::entity::mob::MobEntity,) -> () { }
pub fn r#isPosWalkable(r#x : f32,r#z : f32,) -> bool { }
pub fn r#getTargetY() -> f64 { }
pub fn r#getTargetX() -> f64 { }
pub fn r#getTargetZ() -> f64 { }
pub fn r#wrapDegrees(r#from : f32,r#to : f32,r#max : f32,) -> f32 { }
pub fn r#moveTo(r#x : f64,r#y : f64,r#z : f64,r#speed : f64,) -> () { }
pub fn r#tick() -> () { }
pub fn r#isMoving() -> bool { }
pub fn r#getSpeed() -> f64 { }
}
}
pub mod goal {
pub struct r#CrossbowAttackGoal<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for CrossbowAttackGoal<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl CrossbowAttackGoal<'_> {
const __map_sig : &str = "net/minecraft/class_1383";
/* crate::root::minecraft::entity::ai::goal::Stage = Lnet/minecraft/class_1383$class_3744; */
pub fn r#get_stage(&self) -> Result<crate::root::minecraft::entity::ai::goal::Stage,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::entity::ai::goal::Stage = Lnet/minecraft/class_1383$class_3744; */
pub fn r#set_stage(&self,val : crate::root::minecraft::entity::ai::goal::Stage) -> () {} 
/* i32 = I */
pub fn r#get_chargedTicksLeft(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_chargedTicksLeft(&self,val : i32) -> () {} 
/* crate::root::minecraft::util::math::intprovider::UniformIntProvider = Lnet/minecraft/class_6019; */
pub fn r#get_COOLDOWN_RANGE(&self) -> Result<crate::root::minecraft::util::math::intprovider::UniformIntProvider,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::util::math::intprovider::UniformIntProvider = Lnet/minecraft/class_6019; */
pub fn r#set_COOLDOWN_RANGE(&self,val : crate::root::minecraft::util::math::intprovider::UniformIntProvider) -> () {} 
/* i32 = I */
pub fn r#get_cooldown(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_cooldown(&self,val : i32) -> () {} 
/* f64 = D */
pub fn r#get_speed(&self) -> Result<f64,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f64 = D */
pub fn r#set_speed(&self,val : f64) -> () {} 
/* f32 = F */
pub fn r#get_squaredRange(&self) -> Result<f32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f32 = F */
pub fn r#set_squaredRange(&self,val : f32) -> () {} 
/* i32 = I */
pub fn r#get_seeingTargetTicker(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_seeingTargetTicker(&self,val : i32) -> () {} 
/* crate::root::minecraft::entity::mob::HostileEntity = Lnet/minecraft/class_1588; */
pub fn r#get_actor(&self) -> Result<crate::root::minecraft::entity::mob::HostileEntity,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::entity::mob::HostileEntity = Lnet/minecraft/class_1588; */
pub fn r#set_actor(&self,val : crate::root::minecraft::entity::mob::HostileEntity) -> () {} 
pub fn r#__zINDMINIT(r#actor : crate::root::minecraft::entity::mob::HostileEntity,r#speed : f64,r#range : f32,) -> () { }
pub fn r#isUncharged() -> bool { }
pub fn r#hasAliveTarget() -> bool { }
}
}
pub mod pathing {
}
pub struct r#WardenAngerManager<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for WardenAngerManager<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl WardenAngerManager<'_> {
const __map_sig : &str = "net/minecraft/class_7254";
/* i32 = I */
pub fn r#get_maxAnger(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_maxAnger(&self,val : i32) -> () {} 
/* i32 = I */
pub fn r#get_angerDecreasePerTick(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_angerDecreasePerTick(&self,val : i32) -> () {} 
/* jni::object::JObject = Lit/unimi/dsi/fastutil/objects/Object2IntMap; */
pub fn r#get_suspectsToAngerLevel(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lit/unimi/dsi/fastutil/objects/Object2IntMap; */
pub fn r#set_suspectsToAngerLevel(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/util/ArrayList; */
pub fn r#get_suspects(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/ArrayList; */
pub fn r#set_suspects(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Lit/unimi/dsi/fastutil/objects/Object2IntMap; */
pub fn r#get_suspectUuidsToAngerLevel(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lit/unimi/dsi/fastutil/objects/Object2IntMap; */
pub fn r#set_suspectUuidsToAngerLevel(&self,val : jni::object::JObject) -> () {} 
/* i32 = I */
pub fn r#get_updateTimer(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_updateTimer(&self,val : i32) -> () {} 
/* jni::object::JObject = Lcom/mojang/serialization/Codec; */
pub fn r#get_SUSPECT_CODEC(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lcom/mojang/serialization/Codec; */
pub fn r#set_SUSPECT_CODEC(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/util/function/Predicate; */
pub fn r#get_suspectPredicate(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/function/Predicate; */
pub fn r#set_suspectPredicate(&self,val : jni::object::JObject) -> () {} 
/* crate::root::minecraft::entity::ai::SuspectComparator = Lnet/minecraft/class_7254$class_7379; */
pub fn r#get_suspectComparator(&self) -> Result<crate::root::minecraft::entity::ai::SuspectComparator,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::entity::ai::SuspectComparator = Lnet/minecraft/class_7254$class_7379; */
pub fn r#set_suspectComparator(&self,val : crate::root::minecraft::entity::ai::SuspectComparator) -> () {} 
/* i32 = I */
pub fn r#get_primeAnger(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_primeAnger(&self,val : i32) -> () {} 
pub fn r#__zINDMINIT(r#suspectPredicate : jni::object::JObject,r#suspectUuidsToAngerLevel : jni::object::JObject,) -> () { }
pub fn r#tick(r#world : crate::root::minecraft::server::world::ServerWorld,r#suspectPredicate : jni::object::JObject,) -> () { }
pub fn r#unknwnfn_2730942606(r#suspect : i32,r#anger : crate::root::minecraft::entity::Entity,r#unknwnarg_3864863768 : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#removeSuspect(r#entity : crate::root::minecraft::entity::Entity,) -> () { }
pub fn r#increaseAngerAt(r#entity : crate::root::minecraft::entity::Entity,r#amount : i32,) -> i32 { }
pub fn r#getPrimeSuspect() -> jni::object::JObject { }
pub fn r#getAngerFor(r#entity : crate::root::minecraft::entity::Entity,) -> i32 { }
pub fn r#unknwnfn_1215290412(r#suspect : crate::root::minecraft::entity::Entity,) -> crate::root::minecraft::entity::LivingEntity { }
pub fn r#getPrimeSuspectInternal() -> crate::root::minecraft::entity::Entity { }
pub fn r#unknwnfn_1866541081(r#suspect : crate::root::minecraft::entity::Entity,) -> bool { }
pub fn r#updateSuspectsMap(r#world : crate::root::minecraft::server::world::ServerWorld,) -> () { }
pub fn r#unknwnfn_267519177(r#suspect : jni::object::JObject,) -> () { }
pub fn r#unknwnfn_3802789099(r#suspect : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#unknwnfn_3453408000(r#instance : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#getSuspects() -> jni::object::JObject { }
pub fn r#unknwnfn_3211117913(r#suspect : crate::root::minecraft::entity::Entity,) -> jni::object::JObject { }
pub fn r#createCodec(r#suspectPredicate : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#unknwnfn_3975818488(r#instance : jni::object::JObject,r#unknwnarg_565691850 : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#unknwnfn_1574510057(r#suspectUuidsToAngerLevel : jni::object::JObject,r#unknwnarg_2712567592 : jni::object::JObject,) -> crate::root::minecraft::entity::ai::WardenAngerManager { }
}
}
pub mod attribute {
pub struct r#DefaultAttributeContainer<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for DefaultAttributeContainer<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl DefaultAttributeContainer<'_> {
const __map_sig : &str = "net/minecraft/class_5132";
/* jni::object::JObject = Ljava/util/Map; */
pub fn r#get_instances(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/Map; */
pub fn r#set_instances(&self,val : jni::object::JObject) -> () {} 
pub fn r#__zINDMINIT(r#instances : jni::object::JObject,) -> () { }
pub fn r#builder() -> crate::root::minecraft::entity::attribute::Builder { }
pub fn r#getValue(r#attribute : crate::root::minecraft::entity::attribute::EntityAttribute,) -> f64 { }
pub fn r#createOverride(r#updateCallback : jni::object::JObject,r#attribute : crate::root::minecraft::entity::attribute::EntityAttribute,) -> crate::root::minecraft::entity::attribute::EntityAttributeInstance { }
pub fn r#getBaseValue(r#attribute : crate::root::minecraft::entity::attribute::EntityAttribute,) -> f64 { }
pub fn r#require(r#attribute : crate::root::minecraft::entity::attribute::EntityAttribute,) -> crate::root::minecraft::entity::attribute::EntityAttributeInstance { }
pub fn r#getModifierValue(r#attribute : crate::root::minecraft::entity::attribute::EntityAttribute,r#uuid : jni::object::JObject,) -> f64 { }
pub fn r#hasModifier(r#type : crate::root::minecraft::entity::attribute::EntityAttribute,r#uuid : jni::object::JObject,) -> bool { }
}
}
pub mod boss {
pub struct r#BossBar<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for BossBar<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl BossBar<'_> {
const __map_sig : &str = "net/minecraft/class_1259";
/* jni::object::JObject = Ljava/util/UUID; */
pub fn r#get_uuid(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/UUID; */
pub fn r#set_uuid(&self,val : jni::object::JObject) -> () {} 
/* bool = Z */
pub fn r#get_thickenFog(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_thickenFog(&self,val : bool) -> () {} 
/* f32 = F */
pub fn r#get_percent(&self) -> Result<f32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f32 = F */
pub fn r#set_percent(&self,val : f32) -> () {} 
/* bool = Z */
pub fn r#get_dragonMusic(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_dragonMusic(&self,val : bool) -> () {} 
/* bool = Z */
pub fn r#get_darkenSky(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_darkenSky(&self,val : bool) -> () {} 
/* crate::root::minecraft::text::Text = Lnet/minecraft/class_2561; */
pub fn r#get_name(&self) -> Result<crate::root::minecraft::text::Text,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::text::Text = Lnet/minecraft/class_2561; */
pub fn r#set_name(&self,val : crate::root::minecraft::text::Text) -> () {} 
/* crate::root::minecraft::entity::boss::Color = Lnet/minecraft/class_1259$class_1260; */
pub fn r#get_color(&self) -> Result<crate::root::minecraft::entity::boss::Color,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::entity::boss::Color = Lnet/minecraft/class_1259$class_1260; */
pub fn r#set_color(&self,val : crate::root::minecraft::entity::boss::Color) -> () {} 
/* jni::object::JObject = Lnet/minecraft/class_1259$class_1261; */
pub fn r#get_style(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lnet/minecraft/class_1259$class_1261; */
pub fn r#set_style(&self,val : jni::object::JObject) -> () {} 
pub fn r#__zINDMINIT(r#uuid : jni::object::JObject,r#name : crate::root::minecraft::text::Text,r#color : crate::root::minecraft::entity::boss::Color,r#style : jni::object::JObject,) -> () { }
pub fn r#setDarkenSky(r#darkenSky : bool,) -> crate::root::minecraft::entity::boss::BossBar { }
pub fn r#getUuid() -> jni::object::JObject { }
pub fn r#setPercent(r#percent : f32,) -> () { }
pub fn r#setStyle(r#style : jni::object::JObject,) -> () { }
pub fn r#setDragonMusic(r#dragonMusic : bool,) -> crate::root::minecraft::entity::boss::BossBar { }
pub fn r#setThickenFog(r#thickenFog : bool,) -> crate::root::minecraft::entity::boss::BossBar { }
pub fn r#getPercent() -> f32 { }
pub fn r#setName(r#name : crate::root::minecraft::text::Text,) -> () { }
pub fn r#getName() -> crate::root::minecraft::text::Text { }
pub fn r#getStyle() -> jni::object::JObject { }
pub fn r#setColor(r#color : crate::root::minecraft::entity::boss::Color,) -> () { }
pub fn r#shouldDarkenSky() -> bool { }
pub fn r#hasDragonMusic() -> bool { }
pub fn r#shouldThickenFog() -> bool { }
}
}
pub mod damage {
}
pub mod data {
pub struct r#DataTracker<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for DataTracker<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl DataTracker<'_> {
const __map_sig : &str = "net/minecraft/class_2945";
/* bool = Z */
pub fn r#get_dirty(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_dirty(&self,val : bool) -> () {} 
/* bool = Z */
pub fn r#get_empty(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_empty(&self,val : bool) -> () {} 
/* jni::object::JObject = Lit/unimi/dsi/fastutil/ints/Int2ObjectMap; */
pub fn r#get_entries(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lit/unimi/dsi/fastutil/ints/Int2ObjectMap; */
pub fn r#set_entries(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Lit/unimi/dsi/fastutil/objects/Object2IntMap; */
pub fn r#get_TRACKED_ENTITIES(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lit/unimi/dsi/fastutil/objects/Object2IntMap; */
pub fn r#set_TRACKED_ENTITIES(&self,val : jni::object::JObject) -> () {} 
/* crate::root::minecraft::entity::Entity = Lnet/minecraft/class_1297; */
pub fn r#get_trackedEntity(&self) -> Result<crate::root::minecraft::entity::Entity,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::entity::Entity = Lnet/minecraft/class_1297; */
pub fn r#set_trackedEntity(&self,val : crate::root::minecraft::entity::Entity) -> () {} 
/* jni::object::JObject = Lorg/slf4j/Logger; */
pub fn r#get_LOGGER(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lorg/slf4j/Logger; */
pub fn r#set_LOGGER(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/util/concurrent/locks/ReadWriteLock; */
pub fn r#get_lock(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/concurrent/locks/ReadWriteLock; */
pub fn r#set_lock(&self,val : jni::object::JObject) -> () {} 
/* i32 = I */
pub fn r#get_END_PACKET_WRITE(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_END_PACKET_WRITE(&self,val : i32) -> () {} 
/* i32 = I */
pub fn r#get_MAX_DATA_VALUE_ID(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_MAX_DATA_VALUE_ID(&self,val : i32) -> () {} 
pub fn r#__zINDMINIT(r#trackedEntity : crate::root::minecraft::entity::Entity,) -> () { }
pub fn r#addTrackedData(r#key : crate::root::minecraft::entity::data::TrackedData,r#value : jni::object::JObject,) -> () { }
pub fn r#entryFromPacket(r#buf : crate::root::minecraft::network::PacketByteBuf,r#id : i32,r#handler : crate::root::minecraft::entity::data::TrackedDataHandler,) -> crate::root::minecraft::entity::data::Entry { }
pub fn r#set(r#key : crate::root::minecraft::entity::data::TrackedData,r#value : jni::object::JObject,) -> () { }
pub fn r#writeUpdatedEntries(r#entries : jni::object::JObject,) -> () { }
pub fn r#getDirtyEntries() -> jni::object::JObject { }
pub fn r#writeEntryToPacket(r#buf : crate::root::minecraft::network::PacketByteBuf,r#entry : crate::root::minecraft::entity::data::Entry,) -> () { }
pub fn r#getEntry(r#key : crate::root::minecraft::entity::data::TrackedData,) -> crate::root::minecraft::entity::data::Entry { }
pub fn r#startTracking(r#key : crate::root::minecraft::entity::data::TrackedData,r#initialValue : jni::object::JObject,) -> () { }
pub fn r#copyToFrom(r#to : crate::root::minecraft::entity::data::Entry,r#from : crate::root::minecraft::entity::data::Entry,) -> () { }
pub fn r#isDirty() -> bool { }
pub fn r#entriesToPacket(r#entries : jni::object::JObject,r#buf : crate::root::minecraft::network::PacketByteBuf,) -> () { }
pub fn r#deserializePacket(r#buf : crate::root::minecraft::network::PacketByteBuf,) -> jni::object::JObject { }
pub fn r#get(r#data : crate::root::minecraft::entity::data::TrackedData,) -> jni::object::JObject { }
pub fn r#isEmpty() -> bool { }
pub fn r#registerData(r#entityClass : jni::object::JObject,r#dataHandler : crate::root::minecraft::entity::data::TrackedDataHandler,) -> crate::root::minecraft::entity::data::TrackedData { }
pub fn r#clearDirty() -> () { }
}
}
pub mod decoration {
pub mod painting {
}
}
pub mod effect {
pub struct r#StatusEffectInstance<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for StatusEffectInstance<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl StatusEffectInstance<'_> {
const __map_sig : &str = "net/minecraft/class_1293";
/* crate::root::minecraft::entity::effect::StatusEffectInstance = Lnet/minecraft/class_1293; */
pub fn r#get_hiddenEffect(&self) -> Result<crate::root::minecraft::entity::effect::StatusEffectInstance,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::entity::effect::StatusEffectInstance = Lnet/minecraft/class_1293; */
pub fn r#set_hiddenEffect(&self,val : crate::root::minecraft::entity::effect::StatusEffectInstance) -> () {} 
/* jni::object::JObject = Ljava/util/Optional; */
pub fn r#get_factorCalculationData(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/Optional; */
pub fn r#set_factorCalculationData(&self,val : jni::object::JObject) -> () {} 
/* bool = Z */
pub fn r#get_showIcon(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_showIcon(&self,val : bool) -> () {} 
/* bool = Z */
pub fn r#get_showParticles(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_showParticles(&self,val : bool) -> () {} 
/* bool = Z */
pub fn r#get_permanent(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_permanent(&self,val : bool) -> () {} 
/* bool = Z */
pub fn r#get_ambient(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_ambient(&self,val : bool) -> () {} 
/* i32 = I */
pub fn r#get_amplifier(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_amplifier(&self,val : i32) -> () {} 
/* i32 = I */
pub fn r#get_duration(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_duration(&self,val : i32) -> () {} 
/* crate::root::minecraft::entity::effect::StatusEffect = Lnet/minecraft/class_1291; */
pub fn r#get_type(&self) -> Result<crate::root::minecraft::entity::effect::StatusEffect,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::entity::effect::StatusEffect = Lnet/minecraft/class_1291; */
pub fn r#set_type(&self,val : crate::root::minecraft::entity::effect::StatusEffect) -> () {} 
/* jni::object::JObject = Lorg/slf4j/Logger; */
pub fn r#get_LOGGER(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lorg/slf4j/Logger; */
pub fn r#set_LOGGER(&self,val : jni::object::JObject) -> () {} 
pub fn r#__zINDMINIT(r#type : crate::root::minecraft::entity::effect::StatusEffect,) -> () { }
pub fn r#__zINDMINIT(r#type : crate::root::minecraft::entity::effect::StatusEffect,r#duration : i32,) -> () { }
pub fn r#__zINDMINIT(r#type : crate::root::minecraft::entity::effect::StatusEffect,r#duration : i32,r#amplifier : i32,) -> () { }
pub fn r#__zINDMINIT(r#type : crate::root::minecraft::entity::effect::StatusEffect,r#duration : i32,r#amplifier : i32,r#ambient : bool,r#visible : bool,) -> () { }
pub fn r#__zINDMINIT(r#type : crate::root::minecraft::entity::effect::StatusEffect,r#duration : i32,r#amplifier : i32,r#ambient : bool,r#showParticles : bool,r#showIcon : bool,) -> () { }
pub fn r#__zINDMINIT(r#type : crate::root::minecraft::entity::effect::StatusEffect,r#duration : i32,r#amplifier : i32,r#ambient : bool,r#showParticles : bool,r#showIcon : bool,r#hiddenEffect : crate::root::minecraft::entity::effect::StatusEffectInstance,r#factorCalculationData : jni::object::JObject,) -> () { }
pub fn r#__zINDMINIT(r#instance : crate::root::minecraft::entity::effect::StatusEffectInstance,) -> () { }
pub fn r#unknwnfn_2515584076(r#that : jni::object::JObject,) -> i32 { }
pub fn r#unknwnfn_2777883438(r#o : jni::object::JObject,) -> bool { }
pub fn r#fromNbt(r#type : crate::root::minecraft::entity::effect::StatusEffect,r#nbt : crate::root::minecraft::nbt::NbtCompound,) -> crate::root::minecraft::entity::effect::StatusEffectInstance { }
pub fn r#copyFrom(r#that : crate::root::minecraft::entity::effect::StatusEffectInstance,) -> () { }
pub fn r#writeTypelessNbt(r#nbt : crate::root::minecraft::nbt::NbtCompound,) -> () { }
pub fn r#getFactorCalculationData() -> jni::object::JObject { }
pub fn r#unknwnfn_863667721(r#factorCalculationData : i32,r#unknwnarg_2691976429 : crate::root::minecraft::entity::effect::FactorCalculationData,) -> () { }
pub fn r#unknwnfn_1800670700(r#factorCalculationData : crate::root::minecraft::entity::effect::FactorCalculationData,) -> () { }
pub fn r#unknwnfn_3947395847(r#factorCalculationData : crate::root::minecraft::nbt::NbtCompound,r#unknwnarg_4228683783 : crate::root::minecraft::entity::effect::FactorCalculationData,) -> () { }
pub fn r#unknwnfn_1610375912(r#factorCalculationDataNbt : crate::root::minecraft::nbt::NbtCompound,r#unknwnarg_2071067029 : crate::root::minecraft::nbt::NbtElement,) -> () { }
pub fn r#getAmplifier() -> i32 { }
pub fn r#getEffectType() -> crate::root::minecraft::entity::effect::StatusEffect { }
pub fn r#setPermanent(r#permanent : bool,) -> () { }
pub fn r#shouldShowParticles() -> bool { }
pub fn r#writeNbt(r#nbt : crate::root::minecraft::nbt::NbtCompound,) -> crate::root::minecraft::nbt::NbtCompound { }
pub fn r#fromNbt(r#nbt : crate::root::minecraft::nbt::NbtCompound,) -> crate::root::minecraft::entity::effect::StatusEffectInstance { }
pub fn r#getDuration() -> i32 { }
pub fn r#update(r#entity : crate::root::minecraft::entity::LivingEntity,r#overwriteCallback : jni::object::JObject,) -> bool { }
pub fn r#getTranslationKey() -> jni::object::JObject { }
pub fn r#updateDuration() -> i32 { }
pub fn r#applyUpdateEffect(r#entity : crate::root::minecraft::entity::LivingEntity,) -> () { }
pub fn r#upgrade(r#that : crate::root::minecraft::entity::effect::StatusEffectInstance,) -> bool { }
pub fn r#isAmbient() -> bool { }
pub fn r#shouldShowIcon() -> bool { }
}
}
pub struct r#Entity<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for Entity<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl Entity<'_> {
const __map_sig : &str = "net/minecraft/class_1297";
/* crate::root::minecraft::util::math::Vec3d = Lnet/minecraft/class_243; */
pub fn r#get_movementMultiplier(&self) -> Result<crate::root::minecraft::util::math::Vec3d,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::util::math::Vec3d = Lnet/minecraft/class_243; */
pub fn r#set_movementMultiplier(&self,val : crate::root::minecraft::util::math::Vec3d) -> () {} 
/* crate::root::minecraft::entity::data::TrackedData = Lnet/minecraft/class_2940; */
pub fn r#get_POSE(&self) -> Result<crate::root::minecraft::entity::data::TrackedData,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::entity::data::TrackedData = Lnet/minecraft/class_2940; */
pub fn r#set_POSE(&self,val : crate::root::minecraft::entity::data::TrackedData) -> () {} 
/* crate::root::minecraft::entity::EntityDimensions = Lnet/minecraft/class_4048; */
pub fn r#get_dimensions(&self) -> Result<crate::root::minecraft::entity::EntityDimensions,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::entity::EntityDimensions = Lnet/minecraft/class_4048; */
pub fn r#set_dimensions(&self,val : crate::root::minecraft::entity::EntityDimensions) -> () {} 
/* f32 = F */
pub fn r#get_standingEyeHeight(&self) -> Result<f32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f32 = F */
pub fn r#set_standingEyeHeight(&self,val : f32) -> () {} 
/* crate::root::minecraft::util::math::Vec3d = Lnet/minecraft/class_243; */
pub fn r#get_velocity(&self) -> Result<crate::root::minecraft::util::math::Vec3d,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::util::math::Vec3d = Lnet/minecraft/class_243; */
pub fn r#set_velocity(&self,val : crate::root::minecraft::util::math::Vec3d) -> () {} 
/* crate::root::minecraft::util::math::Vec3d = Lnet/minecraft/class_243; */
pub fn r#get_pos(&self) -> Result<crate::root::minecraft::util::math::Vec3d,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::util::math::Vec3d = Lnet/minecraft/class_243; */
pub fn r#set_pos(&self,val : crate::root::minecraft::util::math::Vec3d) -> () {} 
/* crate::root::minecraft::util::math::BlockPos = Lnet/minecraft/class_2338; */
pub fn r#get_blockPos(&self) -> Result<crate::root::minecraft::util::math::BlockPos,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::util::math::BlockPos = Lnet/minecraft/class_2338; */
pub fn r#set_blockPos(&self,val : crate::root::minecraft::util::math::BlockPos) -> () {} 
/* bool = Z */
pub fn r#get_intersectionChecked(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_intersectionChecked(&self,val : bool) -> () {} 
/* jni::object::JObject = Ljava/util/Set; */
pub fn r#get_submergedFluidTag(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/Set; */
pub fn r#set_submergedFluidTag(&self,val : jni::object::JObject) -> () {} 
/* i32 = I */
pub fn r#get_lastChimeAge(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_lastChimeAge(&self,val : i32) -> () {} 
/* jni::object::JObject = Lnet/minecraft/class_1297$class_5529; */
pub fn r#get_removalReason(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lnet/minecraft/class_1297$class_5529; */
pub fn r#set_removalReason(&self,val : jni::object::JObject) -> () {} 
/* crate::root::minecraft::world::entity::EntityChangeListener = Lnet/minecraft/class_5569; */
pub fn r#get_changeListener(&self) -> Result<crate::root::minecraft::world::entity::EntityChangeListener,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::world::entity::EntityChangeListener = Lnet/minecraft/class_5569; */
pub fn r#set_changeListener(&self,val : crate::root::minecraft::world::entity::EntityChangeListener) -> () {} 
/* f32 = F */
pub fn r#get_lastChimeIntensity(&self) -> Result<f32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f32 = F */
pub fn r#set_lastChimeIntensity(&self,val : f32) -> () {} 
/* bool = Z */
pub fn r#get_inPowderSnow(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_inPowderSnow(&self,val : bool) -> () {} 
/* crate::root::minecraft::entity::data::TrackedData = Lnet/minecraft/class_2940; */
pub fn r#get_FROZEN_TICKS(&self) -> Result<crate::root::minecraft::entity::data::TrackedData,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::entity::data::TrackedData = Lnet/minecraft/class_2940; */
pub fn r#set_FROZEN_TICKS(&self,val : crate::root::minecraft::entity::data::TrackedData) -> () {} 
/* f32 = F */
pub fn r#get_speed(&self) -> Result<f32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f32 = F */
pub fn r#set_speed(&self,val : f32) -> () {} 
/* bool = Z */
pub fn r#get_wasInPowderSnow(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_wasInPowderSnow(&self,val : bool) -> () {} 
/* bool = Z */
pub fn r#get_wasOnFire(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_wasOnFire(&self,val : bool) -> () {} 
/* f32 = F */
pub fn r#get_DEFAULT_FRICTION(&self) -> Result<f32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f32 = F */
pub fn r#set_DEFAULT_FRICTION(&self,val : f32) -> () {} 
/* f32 = F */
pub fn r#get_MIN_RISING_BUBBLE_COLUMN_SPEED(&self) -> Result<f32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f32 = F */
pub fn r#set_MIN_RISING_BUBBLE_COLUMN_SPEED(&self,val : f32) -> () {} 
/* i32 = I */
pub fn r#get_SNEAKING_FLAG_INDEX(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_SNEAKING_FLAG_INDEX(&self,val : i32) -> () {} 
/* i32 = I */
pub fn r#get_SPRINTING_FLAG_INDEX(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_SPRINTING_FLAG_INDEX(&self,val : i32) -> () {} 
/* i32 = I */
pub fn r#get_SWIMMING_FLAG_INDEX(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_SWIMMING_FLAG_INDEX(&self,val : i32) -> () {} 
/* i32 = I */
pub fn r#get_INVISIBLE_FLAG_INDEX(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_INVISIBLE_FLAG_INDEX(&self,val : i32) -> () {} 
/* i32 = I */
pub fn r#get_ON_FIRE_FLAG_INDEX(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_ON_FIRE_FLAG_INDEX(&self,val : i32) -> () {} 
/* i32 = I */
pub fn r#get_GLOWING_FLAG_INDEX(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_GLOWING_FLAG_INDEX(&self,val : i32) -> () {} 
/* i32 = I */
pub fn r#get_FALL_FLYING_FLAG_INDEX(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_FALL_FLYING_FLAG_INDEX(&self,val : i32) -> () {} 
/* f64 = D */
pub fn r#get_SPEED_IN_LAVA_IN_NETHER(&self) -> Result<f64,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f64 = D */
pub fn r#set_SPEED_IN_LAVA_IN_NETHER(&self,val : f64) -> () {} 
/* f64 = D */
pub fn r#get_SPEED_IN_LAVA(&self) -> Result<f64,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f64 = D */
pub fn r#set_SPEED_IN_LAVA(&self,val : f64) -> () {} 
/* f64 = D */
pub fn r#get_SPEED_IN_WATER(&self) -> Result<f64,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f64 = D */
pub fn r#set_SPEED_IN_WATER(&self,val : f64) -> () {} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#get_ID_KEY(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#set_ID_KEY(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#get_PASSENGERS_KEY(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#set_PASSENGERS_KEY(&self,val : jni::object::JObject) -> () {} 
/* i32 = I */
pub fn r#get_MAX_RIDING_COOLDOWN(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_MAX_RIDING_COOLDOWN(&self,val : i32) -> () {} 
/* i32 = I */
pub fn r#get_DEFAULT_PORTAL_COOLDOWN(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_DEFAULT_PORTAL_COOLDOWN(&self,val : i32) -> () {} 
/* i32 = I */
pub fn r#get_MAX_SCOREBOARD_TAGS(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_MAX_SCOREBOARD_TAGS(&self,val : i32) -> () {} 
/* f64 = D */
pub fn r#get_VELOCITY_AFFECTING_POS_Y_OFFSET(&self) -> Result<f64,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f64 = D */
pub fn r#set_VELOCITY_AFFECTING_POS_Y_OFFSET(&self,val : f64) -> () {} 
/* i32 = I */
pub fn r#get_DEFAULT_MIN_FREEZE_DAMAGE_TICKS(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_DEFAULT_MIN_FREEZE_DAMAGE_TICKS(&self,val : i32) -> () {} 
/* i32 = I */
pub fn r#get_FREEZING_DAMAGE_INTERVAL(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_FREEZING_DAMAGE_INTERVAL(&self,val : i32) -> () {} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#get_UUID_KEY(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#set_UUID_KEY(&self,val : jni::object::JObject) -> () {} 
/* bool = Z */
pub fn r#get_hasVisualFire(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_hasVisualFire(&self,val : bool) -> () {} 
/* bool = Z */
pub fn r#get_collidedSoftly(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_collidedSoftly(&self,val : bool) -> () {} 
/* crate::root::minecraft::util::math::ChunkPos = Lnet/minecraft/class_1923; */
pub fn r#get_chunkPos(&self) -> Result<crate::root::minecraft::util::math::ChunkPos,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::util::math::ChunkPos = Lnet/minecraft/class_1923; */
pub fn r#set_chunkPos(&self,val : crate::root::minecraft::util::math::ChunkPos) -> () {} 
/* crate::root::minecraft::block::BlockState = Lnet/minecraft/class_2680; */
pub fn r#get_blockStateAtPos(&self) -> Result<crate::root::minecraft::block::BlockState,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::block::BlockState = Lnet/minecraft/class_2680; */
pub fn r#set_blockStateAtPos(&self,val : crate::root::minecraft::block::BlockState) -> () {} 
/* crate::root::minecraft::entity::TrackedPosition = Lnet/minecraft/class_7422; */
pub fn r#get_trackedPosition(&self) -> Result<crate::root::minecraft::entity::TrackedPosition,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::entity::TrackedPosition = Lnet/minecraft/class_7422; */
pub fn r#set_trackedPosition(&self,val : crate::root::minecraft::entity::TrackedPosition) -> () {} 
/* i32 = I */
pub fn r#get_ridingCooldown(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_ridingCooldown(&self,val : i32) -> () {} 
/* bool = Z */
pub fn r#get_onGround(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_onGround(&self,val : bool) -> () {} 
/* bool = Z */
pub fn r#get_firstUpdate(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_firstUpdate(&self,val : bool) -> () {} 
/* jni::object::JObject = Lorg/slf4j/Logger; */
pub fn r#get_LOGGER(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lorg/slf4j/Logger; */
pub fn r#set_LOGGER(&self,val : jni::object::JObject) -> () {} 
/* i32 = I */
pub fn r#get_fireTicks(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_fireTicks(&self,val : i32) -> () {} 
/* bool = Z */
pub fn r#get_touchingWater(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_touchingWater(&self,val : bool) -> () {} 
/* bool = Z */
pub fn r#get_glowing(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_glowing(&self,val : bool) -> () {} 
/* bool = Z */
pub fn r#get_noClip(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_noClip(&self,val : bool) -> () {} 
/* crate::root::minecraft::entity::EntityType = Lnet/minecraft/class_1299; */
pub fn r#get_type(&self) -> Result<crate::root::minecraft::entity::EntityType,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::entity::EntityType = Lnet/minecraft/class_1299; */
pub fn r#set_type(&self,val : crate::root::minecraft::entity::EntityType) -> () {} 
/* crate::root::minecraft::entity::data::TrackedData = Lnet/minecraft/class_2940; */
pub fn r#get_SILENT(&self) -> Result<crate::root::minecraft::entity::data::TrackedData,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::entity::data::TrackedData = Lnet/minecraft/class_2940; */
pub fn r#set_SILENT(&self,val : crate::root::minecraft::entity::data::TrackedData) -> () {} 
/* bool = Z */
pub fn r#get_inNetherPortal(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_inNetherPortal(&self,val : bool) -> () {} 
/* jni::object::JObject = Lit/unimi/dsi/fastutil/objects/Object2DoubleMap; */
pub fn r#get_fluidHeight(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lit/unimi/dsi/fastutil/objects/Object2DoubleMap; */
pub fn r#set_fluidHeight(&self,val : jni::object::JObject) -> () {} 
/* f32 = F */
pub fn r#get_pitch(&self) -> Result<f32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f32 = F */
pub fn r#set_pitch(&self,val : f32) -> () {} 
/* f64 = D */
pub fn r#get_prevZ(&self) -> Result<f64,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f64 = D */
pub fn r#set_prevZ(&self,val : f64) -> () {} 
/* f64 = D */
pub fn r#get_lastRenderY(&self) -> Result<f64,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f64 = D */
pub fn r#set_lastRenderY(&self,val : f64) -> () {} 
/* i32 = I */
pub fn r#get_netherPortalTime(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_netherPortalTime(&self,val : i32) -> () {} 
/* f32 = F */
pub fn r#get_horizontalSpeed(&self) -> Result<f32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f32 = F */
pub fn r#set_horizontalSpeed(&self,val : f32) -> () {} 
/* crate::root::minecraft::util::math::random::Random = Lnet/minecraft/class_5819; */
pub fn r#get_random(&self) -> Result<crate::root::minecraft::util::math::random::Random,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::util::math::random::Random = Lnet/minecraft/class_5819; */
pub fn r#set_random(&self,val : crate::root::minecraft::util::math::random::Random) -> () {} 
/* crate::root::minecraft::entity::data::TrackedData = Lnet/minecraft/class_2940; */
pub fn r#get_NAME_VISIBLE(&self) -> Result<crate::root::minecraft::entity::data::TrackedData,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::entity::data::TrackedData = Lnet/minecraft/class_2940; */
pub fn r#set_NAME_VISIBLE(&self,val : crate::root::minecraft::entity::data::TrackedData) -> () {} 
/* bool = Z */
pub fn r#get_horizontalCollision(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_horizontalCollision(&self,val : bool) -> () {} 
/* jni::object::JObject = Ljava/util/concurrent/atomic/AtomicInteger; */
pub fn r#get_CURRENT_ID(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/concurrent/atomic/AtomicInteger; */
pub fn r#set_CURRENT_ID(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Lcom/google/common/collect/ImmutableList; */
pub fn r#get_passengerList(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lcom/google/common/collect/ImmutableList; */
pub fn r#set_passengerList(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#get_uuidString(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#set_uuidString(&self,val : jni::object::JObject) -> () {} 
/* f32 = F */
pub fn r#get_prevYaw(&self) -> Result<f32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f32 = F */
pub fn r#set_prevYaw(&self,val : f32) -> () {} 
/* bool = Z */
pub fn r#get_ignoreCameraFrustum(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_ignoreCameraFrustum(&self,val : bool) -> () {} 
/* i32 = I */
pub fn r#get_id(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_id(&self,val : i32) -> () {} 
/* f64 = D */
pub fn r#get_lastRenderZ(&self) -> Result<f64,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f64 = D */
pub fn r#set_lastRenderZ(&self,val : f64) -> () {} 
/* crate::root::minecraft::entity::data::TrackedData = Lnet/minecraft/class_2940; */
pub fn r#get_FLAGS(&self) -> Result<crate::root::minecraft::entity::data::TrackedData,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::entity::data::TrackedData = Lnet/minecraft/class_2940; */
pub fn r#set_FLAGS(&self,val : crate::root::minecraft::entity::data::TrackedData) -> () {} 
/* crate::root::minecraft::util::math::BlockPos = Lnet/minecraft/class_2338; */
pub fn r#get_lastNetherPortalPosition(&self) -> Result<crate::root::minecraft::util::math::BlockPos,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::util::math::BlockPos = Lnet/minecraft/class_2338; */
pub fn r#set_lastNetherPortalPosition(&self,val : crate::root::minecraft::util::math::BlockPos) -> () {} 
/* bool = Z */
pub fn r#get_verticalCollision(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_verticalCollision(&self,val : bool) -> () {} 
/* std::vec::Vec<f64> = [D */
pub fn r#get_pistonMovementDelta(&self) -> Result<std::vec::Vec<f64>,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* std::vec::Vec<f64> = [D */
pub fn r#set_pistonMovementDelta(&self,val : std::vec::Vec<f64>) -> () {} 
/* f32 = F */
pub fn r#get_distanceTraveled(&self) -> Result<f32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f32 = F */
pub fn r#set_distanceTraveled(&self,val : f32) -> () {} 
/* crate::root::minecraft::entity::data::TrackedData = Lnet/minecraft/class_2940; */
pub fn r#get_NO_GRAVITY(&self) -> Result<crate::root::minecraft::entity::data::TrackedData,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::entity::data::TrackedData = Lnet/minecraft/class_2940; */
pub fn r#set_NO_GRAVITY(&self,val : crate::root::minecraft::entity::data::TrackedData) -> () {} 
/* i64 = J */
pub fn r#get_pistonMovementTick(&self) -> Result<i64,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i64 = J */
pub fn r#set_pistonMovementTick(&self,val : i64) -> () {} 
/* f64 = D */
pub fn r#get_renderDistanceMultiplier(&self) -> Result<f64,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f64 = D */
pub fn r#set_renderDistanceMultiplier(&self,val : f64) -> () {} 
/* bool = Z */
pub fn r#get_submergedInWater(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_submergedInWater(&self,val : bool) -> () {} 
/* crate::root::minecraft::world::World = Lnet/minecraft/class_1937; */
pub fn r#get_world(&self) -> Result<crate::root::minecraft::world::World,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::world::World = Lnet/minecraft/class_1937; */
pub fn r#set_world(&self,val : crate::root::minecraft::world::World) -> () {} 
/* f32 = F */
pub fn r#get_nextStepSoundDistance(&self) -> Result<f32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f32 = F */
pub fn r#set_nextStepSoundDistance(&self,val : f32) -> () {} 
/* f32 = F */
pub fn r#get_prevPitch(&self) -> Result<f32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f32 = F */
pub fn r#set_prevPitch(&self,val : f32) -> () {} 
/* crate::root::minecraft::util::math::Box = Lnet/minecraft/class_238; */
pub fn r#get_boundingBox(&self) -> Result<crate::root::minecraft::util::math::Box,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::util::math::Box = Lnet/minecraft/class_238; */
pub fn r#set_boundingBox(&self,val : crate::root::minecraft::util::math::Box) -> () {} 
/* bool = Z */
pub fn r#get_velocityDirty(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_velocityDirty(&self,val : bool) -> () {} 
/* i32 = I */
pub fn r#get_timeUntilRegen(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_timeUntilRegen(&self,val : i32) -> () {} 
/* bool = Z */
pub fn r#get_invulnerable(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_invulnerable(&self,val : bool) -> () {} 
/* crate::root::minecraft::entity::data::DataTracker = Lnet/minecraft/class_2945; */
pub fn r#get_dataTracker(&self) -> Result<crate::root::minecraft::entity::data::DataTracker,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::entity::data::DataTracker = Lnet/minecraft/class_2945; */
pub fn r#set_dataTracker(&self,val : crate::root::minecraft::entity::data::DataTracker) -> () {} 
/* i32 = I */
pub fn r#get_age(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_age(&self,val : i32) -> () {} 
/* f32 = F */
pub fn r#get_stepHeight(&self) -> Result<f32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f32 = F */
pub fn r#set_stepHeight(&self,val : f32) -> () {} 
/* f64 = D */
pub fn r#get_prevX(&self) -> Result<f64,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f64 = D */
pub fn r#set_prevX(&self,val : f64) -> () {} 
/* f32 = F */
pub fn r#get_fallDistance(&self) -> Result<f32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f32 = F */
pub fn r#set_fallDistance(&self,val : f32) -> () {} 
/* i32 = I */
pub fn r#get_portalCooldown(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_portalCooldown(&self,val : i32) -> () {} 
/* jni::object::JObject = Ljava/util/UUID; */
pub fn r#get_uuid(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/UUID; */
pub fn r#set_uuid(&self,val : jni::object::JObject) -> () {} 
/* crate::root::minecraft::util::math::Box = Lnet/minecraft/class_238; */
pub fn r#get_NULL_BOX(&self) -> Result<crate::root::minecraft::util::math::Box,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::util::math::Box = Lnet/minecraft/class_238; */
pub fn r#set_NULL_BOX(&self,val : crate::root::minecraft::util::math::Box) -> () {} 
/* crate::root::minecraft::entity::data::TrackedData = Lnet/minecraft/class_2940; */
pub fn r#get_CUSTOM_NAME(&self) -> Result<crate::root::minecraft::entity::data::TrackedData,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::entity::data::TrackedData = Lnet/minecraft/class_2940; */
pub fn r#set_CUSTOM_NAME(&self,val : crate::root::minecraft::entity::data::TrackedData) -> () {} 
/* jni::object::JObject = Ljava/util/Set; */
pub fn r#get_scoreboardTags(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/Set; */
pub fn r#set_scoreboardTags(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/util/List; */
pub fn r#get_EMPTY_STACK_LIST(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/List; */
pub fn r#set_EMPTY_STACK_LIST(&self,val : jni::object::JObject) -> () {} 
/* f32 = F */
pub fn r#get_yaw(&self) -> Result<f32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f32 = F */
pub fn r#set_yaw(&self,val : f32) -> () {} 
/* crate::root::minecraft::entity::data::TrackedData = Lnet/minecraft/class_2940; */
pub fn r#get_AIR(&self) -> Result<crate::root::minecraft::entity::data::TrackedData,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::entity::data::TrackedData = Lnet/minecraft/class_2940; */
pub fn r#set_AIR(&self,val : crate::root::minecraft::entity::data::TrackedData) -> () {} 
/* crate::root::minecraft::entity::Entity = Lnet/minecraft/class_1297; */
pub fn r#get_vehicle(&self) -> Result<crate::root::minecraft::entity::Entity,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::entity::Entity = Lnet/minecraft/class_1297; */
pub fn r#set_vehicle(&self,val : crate::root::minecraft::entity::Entity) -> () {} 
/* f64 = D */
pub fn r#get_prevY(&self) -> Result<f64,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f64 = D */
pub fn r#set_prevY(&self,val : f64) -> () {} 
/* bool = Z */
pub fn r#get_velocityModified(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_velocityModified(&self,val : bool) -> () {} 
/* f64 = D */
pub fn r#get_lastRenderX(&self) -> Result<f64,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f64 = D */
pub fn r#set_lastRenderX(&self,val : f64) -> () {} 
/* f32 = F */
pub fn r#get_prevHorizontalSpeed(&self) -> Result<f32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f32 = F */
pub fn r#set_prevHorizontalSpeed(&self,val : f32) -> () {} 
pub fn r#__zINDMINIT(r#type : crate::root::minecraft::entity::EntityType,r#world : crate::root::minecraft::world::World,) -> () { }
pub fn r#unknwnfn_2449462555(r#o : jni::object::JObject,) -> bool { }
pub fn r#getWidth() -> f32 { }
pub fn r#getHeight() -> f32 { }
pub fn r#adjustMovementForCollisions(r#movement : crate::root::minecraft::util::math::Vec3d,) -> crate::root::minecraft::util::math::Vec3d { }
pub fn r#createSpawnPacket() -> crate::root::minecraft::network::Packet { }
pub fn r#isInSneakingPose() -> bool { }
pub fn r#detach() -> () { }
pub fn r#getPose() -> crate::root::minecraft::entity::EntityPose { }
pub fn r#getDimensions(r#pose : crate::root::minecraft::entity::EntityPose,) -> crate::root::minecraft::entity::EntityDimensions { }
pub fn r#getEyeHeight(r#pose : crate::root::minecraft::entity::EntityPose,r#dimensions : crate::root::minecraft::entity::EntityDimensions,) -> f32 { }
pub fn r#tickPortal() -> () { }
pub fn r#setPose(r#pose : crate::root::minecraft::entity::EntityPose,) -> () { }
pub fn r#getEyeHeight(r#pose : crate::root::minecraft::entity::EntityPose,) -> f32 { }
pub fn r#calculateDimensions() -> () { }
pub fn r#adjustMovementForPiston(r#movement : crate::root::minecraft::util::math::Vec3d,) -> crate::root::minecraft::util::math::Vec3d { }
pub fn r#movementInputToVelocity(r#movementInput : crate::root::minecraft::util::math::Vec3d,r#speed : f32,r#yaw : f32,) -> crate::root::minecraft::util::math::Vec3d { }
pub fn r#adjustMovementForSneaking(r#movement : crate::root::minecraft::util::math::Vec3d,r#type : crate::root::minecraft::entity::MovementType,) -> crate::root::minecraft::util::math::Vec3d { }
pub fn r#calculatePistonMovementFactor(r#axis : crate::root::minecraft::util::math::Axis,r#offsetFactor : f64,) -> f64 { }
pub fn r#getVelocity() -> crate::root::minecraft::util::math::Vec3d { }
pub fn r#setVelocity(r#velocity : crate::root::minecraft::util::math::Vec3d,) -> () { }
pub fn r#setVelocity(r#x : f64,r#y : f64,r#z : f64,) -> () { }
pub fn r#getOppositeRotationVector(r#pitch : f32,r#yaw : f32,) -> crate::root::minecraft::util::math::Vec3d { }
pub fn r#getOppositeRotationVector(r#tickDelta : f32,) -> crate::root::minecraft::util::math::Vec3d { }
pub fn r#getPos() -> crate::root::minecraft::util::math::Vec3d { }
pub fn r#isInSwimmingPose() -> bool { }
pub fn r#wouldPoseNotCollide(r#pose : crate::root::minecraft::entity::EntityPose,) -> bool { }
pub fn r#calculateBoundsForPose(r#pos : crate::root::minecraft::entity::EntityPose,) -> crate::root::minecraft::util::math::Box { }
pub fn r#isCrawling() -> bool { }
pub fn r#teleport(r#destX : f64,r#destY : f64,r#destZ : f64,) -> () { }
pub fn r#adjustMovementForCollisions(r#entity : crate::root::minecraft::entity::Entity,r#movement : crate::root::minecraft::util::math::Vec3d,r#entityBoundingBox : crate::root::minecraft::util::math::Box,r#world : crate::root::minecraft::world::World,r#collisions : jni::object::JObject,) -> crate::root::minecraft::util::math::Vec3d { }
pub fn r#adjustMovementForCollisions(r#movement : crate::root::minecraft::util::math::Vec3d,r#entityBoundingBox : crate::root::minecraft::util::math::Box,r#collisions : jni::object::JObject,) -> crate::root::minecraft::util::math::Vec3d { }
pub fn r#getFireTicks() -> i32 { }
pub fn r#setFireTicks(r#fireTicks : i32,) -> () { }
pub fn r#bypassesSteppingEffects() -> bool { }
pub fn r#bypassesLandingEffects() -> bool { }
pub fn r#isSneaky() -> bool { }
pub fn r#isDescending() -> bool { }
pub fn r#getTeamColorValue() -> i32 { }
pub fn r#resetPosition() -> () { }
pub fn r#refreshPosition() -> () { }
pub fn r#getSteppingPos() -> crate::root::minecraft::util::math::BlockPos { }
pub fn r#getJumpVelocityMultiplier() -> f32 { }
pub fn r#getVelocityAffectingPos() -> crate::root::minecraft::util::math::BlockPos { }
pub fn r#getDefaultName() -> crate::root::minecraft::text::Text { }
pub fn r#offsetX(r#widthScale : f64,) -> f64 { }
pub fn r#getX() -> f64 { }
pub fn r#getY() -> f64 { }
pub fn r#getRandomBodyY() -> f64 { }
pub fn r#getEyeY() -> f64 { }
pub fn r#getZ() -> f64 { }
pub fn r#getParticleX(r#widthScale : f64,) -> f64 { }
pub fn r#getBodyY(r#heightScale : f64,) -> f64 { }
pub fn r#offsetZ(r#widthScale : f64,) -> f64 { }
pub fn r#getParticleZ(r#widthScale : f64,) -> f64 { }
pub fn r#getVelocityMultiplier() -> f32 { }
pub fn r#setPos(r#x : f64,r#y : f64,r#z : f64,) -> () { }
pub fn r#updatePassengerPosition(r#passenger : crate::root::minecraft::entity::Entity,r#positionUpdater : crate::root::minecraft::entity::PositionUpdater,) -> () { }
pub fn r#refreshPositionAfterTeleport(r#x : f64,r#y : f64,r#z : f64,) -> () { }
pub fn r#isInRange(r#entity : crate::root::minecraft::entity::Entity,r#radius : f64,) -> bool { }
pub fn r#getPassengerDismountOffset(r#vehicleWidth : f64,r#passengerWidth : f64,r#passengerYaw : f32,) -> crate::root::minecraft::util::math::Vec3d { }
pub fn r#isOnGround() -> bool { }
pub fn r#updatePassengerForDismount(r#passenger : crate::root::minecraft::entity::LivingEntity,) -> crate::root::minecraft::util::math::Vec3d { }
pub fn r#setOnGround(r#onGround : bool,) -> () { }
pub fn r#getSteppingBlockState() -> crate::root::minecraft::block::BlockState { }
pub fn r#shouldSpawnSprintingParticles() -> bool { }
pub fn r#dismountVehicle() -> () { }
pub fn r#getSwimHeight() -> f64 { }
pub fn r#unknwnfn_2791200532(r#state : crate::root::minecraft::block::BlockState,) -> bool { }
pub fn r#refreshPositionAfterTeleport(r#pos : crate::root::minecraft::util::math::Vec3d,) -> () { }
pub fn r#getLeashOffset() -> crate::root::minecraft::util::math::Vec3d { }
pub fn r#unknwnfn_1673621629(r#pos : crate::root::minecraft::util::math::Box,r#unknwnarg_1482692961 : crate::root::minecraft::util::math::BlockPos,) -> bool { }
pub fn r#removeFromDimension() -> () { }
pub fn r#resetPortalCooldown() -> () { }
pub fn r#hasPortalCooldown() -> bool { }
pub fn r#getTeleportTarget(r#destination : crate::root::minecraft::server::world::ServerWorld,) -> crate::root::minecraft::world::TeleportTarget { }
pub fn r#getPortalRect(r#destWorld : crate::root::minecraft::server::world::ServerWorld,r#destPos : crate::root::minecraft::util::math::BlockPos,r#destIsNether : bool,r#worldBorder : crate::root::minecraft::world::border::WorldBorder,) -> jni::object::JObject { }
pub fn r#unknwnfn_58074907(r#rect : crate::root::minecraft::server::world::ServerWorld,r#unknwnarg_1317412598 : crate::root::minecraft::world::Rectangle,) -> crate::root::minecraft::world::TeleportTarget { }
pub fn r#unknwnfn_1506640411(r#pos : crate::root::minecraft::block::BlockState,r#unknwnarg_3202371716 : crate::root::minecraft::util::math::BlockPos,) -> bool { }
pub fn r#collidesWithStateAtPos(r#pos : crate::root::minecraft::util::math::BlockPos,r#state : crate::root::minecraft::block::BlockState,) -> bool { }
pub fn r#positionInPortal(r#portalAxis : crate::root::minecraft::util::math::Axis,r#portalRect : crate::root::minecraft::world::Rectangle,) -> crate::root::minecraft::util::math::Vec3d { }
pub fn r#updatePosition(r#x : f64,r#y : f64,r#z : f64,) -> () { }
pub fn r#isCollidable() -> bool { }
pub fn r#collidesWith(r#other : crate::root::minecraft::entity::Entity,) -> bool { }
pub fn r#getLerpedPos(r#delta : f32,) -> crate::root::minecraft::util::math::Vec3d { }
pub fn r#getLeashPos(r#delta : f32,) -> crate::root::minecraft::util::math::Vec3d { }
pub fn r#getClientCameraPosVec(r#tickDelta : f32,) -> crate::root::minecraft::util::math::Vec3d { }
pub fn r#unknwnfn_831763192(r#entity : crate::root::minecraft::entity::Entity,) -> bool { }
pub fn r#unknwnfn_3976188363(r#entity : crate::root::minecraft::entity::Entity,r#unknwnarg_1921385745 : crate::root::minecraft::entity::Entity,) -> bool { }
pub fn r#onSpawnPacket(r#packet : crate::root::minecraft::network::packet::s2c::play::EntitySpawnS2CPacket,) -> () { }
pub fn r#discard() -> () { }
pub fn r#attemptTickInVoid() -> () { }
pub fn r#unknwnfn_1209359929(r#entity : crate::root::minecraft::entity::Entity,) -> () { }
pub fn r#getChunkPos() -> crate::root::minecraft::util::math::ChunkPos { }
pub fn r#getBlockX() -> i32 { }
pub fn r#getBlockY() -> i32 { }
pub fn r#getBlockZ() -> i32 { }
pub fn r#getPickBlockStack() -> crate::root::minecraft::item::ItemStack { }
pub fn r#isRemoved() -> bool { }
pub fn r#unsetRemoved() -> () { }
pub fn r#getFirstPassenger() -> crate::root::minecraft::entity::Entity { }
pub fn r#streamIntoPassengers() -> jni::object::JObject { }
pub fn r#getFrozenTicks() -> i32 { }
pub fn r#getFreezingScale() -> f32 { }
pub fn r#isFrozen() -> bool { }
pub fn r#getMinFreezeDamageTicks() -> i32 { }
pub fn r#canFreeze() -> bool { }
pub fn r#setFrozenTicks(r#frozenTicks : i32,) -> () { }
pub fn r#getStackReference(r#mappedIndex : i32,) -> crate::root::minecraft::inventory::StackReference { }
pub fn r#setInPowderSnow(r#inPowderSnow : bool,) -> () { }
pub fn r#emitGameEvent(r#event : crate::root::minecraft::world::event::GameEvent,r#entity : crate::root::minecraft::entity::Entity,) -> () { }
pub fn r#emitGameEvent(r#event : crate::root::minecraft::world::event::GameEvent,) -> () { }
pub fn r#occludeVibrationSignals() -> bool { }
pub fn r#unknwnfn_1826560441(r#pos : crate::root::minecraft::entity::EntityDimensions,r#unknwnarg_3900191076 : crate::root::minecraft::util::math::Vec3d,) -> () { }
pub fn r#calculateBoundingBox() -> crate::root::minecraft::util::math::Box { }
pub fn r#requestTeleportAndDismount(r#destX : f64,r#destY : f64,r#destZ : f64,) -> () { }
pub fn r#getMoveEffect() -> jni::object::JObject { }
pub fn r#getEyePos() -> crate::root::minecraft::util::math::Vec3d { }
pub fn r#setOnFire(r#onFire : bool,) -> () { }
pub fn r#addAirTravelEffects() -> () { }
pub fn r#setPosition(r#pos : crate::root::minecraft::util::math::Vec3d,) -> () { }
pub fn r#isRegionUnloaded() -> bool { }
pub fn r#getRemovalReason() -> jni::object::JObject { }
pub fn r#onRemoved() -> () { }
pub fn r#isGlowingLocal() -> bool { }
pub fn r#getYaw() -> f32 { }
pub fn r#getPitch() -> f32 { }
pub fn r#setYaw(r#yaw : f32,) -> () { }
pub fn r#setPitch(r#pitch : f32,) -> () { }
pub fn r#getBlockStateAtPos() -> crate::root::minecraft::block::BlockState { }
pub fn r#canModifyAt(r#world : crate::root::minecraft::world::World,r#pos : crate::root::minecraft::util::math::BlockPos,) -> bool { }
pub fn r#tryCheckBlockCollision() -> () { }
pub fn r#playExtinguishSound() -> () { }
pub fn r#playAmethystChimeSound(r#state : crate::root::minecraft::block::BlockState,) -> () { }
pub fn r#unknwnfn_2131269577(r#player : crate::root::minecraft::entity::Entity,) -> () { }
pub fn r#unknwnfn_1110890368(r#passenger : crate::root::minecraft::entity::Entity,) -> bool { }
pub fn r#getWorld() -> crate::root::minecraft::world::World { }
pub fn r#onLanding() -> () { }
pub fn r#hasCollidedSoftly(r#adjustedMovement : crate::root::minecraft::util::math::Vec3d,) -> bool { }
pub fn r#shouldEscapePowderSnow() -> bool { }
pub fn r#getHandPosOffset(r#item : crate::root::minecraft::item::Item,) -> crate::root::minecraft::util::math::Vec3d { }
pub fn r#isInPose(r#pose : crate::root::minecraft::entity::EntityPose,) -> bool { }
pub fn r#updateEventHandler(r#callback : jni::object::JObject,) -> () { }
pub fn r#hasPrimaryPassenger() -> bool { }
pub fn r#playSoundIfNotSilent(r#event : crate::root::minecraft::sound::SoundEvent,) -> () { }
pub fn r#getBodyYaw() -> f32 { }
pub fn r#getPosWithYOffset(r#offset : f32,) -> crate::root::minecraft::util::math::BlockPos { }
pub fn r#isInRange(r#entity : crate::root::minecraft::entity::Entity,r#horizontalRadius : f64,r#verticalRadius : f64,) -> bool { }
pub fn r#getLandingPos() -> crate::root::minecraft::util::math::BlockPos { }
pub fn r#getLandingBlockState() -> crate::root::minecraft::block::BlockState { }
pub fn r#getTrackedPosition() -> crate::root::minecraft::entity::TrackedPosition { }
pub fn r#getSyncedPos() -> crate::root::minecraft::util::math::Vec3d { }
pub fn r#updateTrackedPosition(r#x : f64,r#y : f64,r#z : f64,) -> () { }
pub fn r#getMessageSourceProfile() -> crate::root::minecraft::network::message::MessageSourceProfile { }
pub fn r#getMountedHeightOffset() -> f64 { }
pub fn r#onBlockCollision(r#state : crate::root::minecraft::block::BlockState,) -> () { }
pub fn r#fall(r#heightDifference : f64,r#onGround : bool,r#state : crate::root::minecraft::block::BlockState,r#landedPosition : crate::root::minecraft::util::math::BlockPos,) -> () { }
pub fn r#isSprinting() -> bool { }
pub fn r#getSplashSound() -> crate::root::minecraft::sound::SoundEvent { }
pub fn r#hasPassenger(r#passenger : crate::root::minecraft::entity::Entity,) -> bool { }
pub fn r#addPassenger(r#passenger : crate::root::minecraft::entity::Entity,) -> () { }
pub fn r#doesNotCollide(r#box : crate::root::minecraft::util::math::Box,) -> bool { }
pub fn r#updateSubmergedInWaterState() -> () { }
pub fn r#getRotationVector(r#pitch : f32,r#yaw : f32,) -> crate::root::minecraft::util::math::Vec3d { }
pub fn r#pushOutOfBlocks(r#x : f64,r#y : f64,r#z : f64,) -> () { }
pub fn r#getSoundCategory() -> crate::root::minecraft::sound::SoundCategory { }
pub fn r#setBodyYaw(r#bodyYaw : f32,) -> () { }
pub fn r#isWet() -> bool { }
pub fn r#shouldSetPositionOnLoad() -> bool { }
pub fn r#setOnFireFor(r#seconds : i32,) -> () { }
pub fn r#shouldRender(r#distance : f64,) -> bool { }
pub fn r#updatePositionAndAngles(r#x : f64,r#y : f64,r#z : f64,r#yaw : f32,r#pitch : f32,) -> () { }
pub fn r#getPrimaryPassenger() -> crate::root::minecraft::entity::Entity { }
pub fn r#damage(r#source : crate::root::minecraft::entity::damage::DamageSource,r#amount : f32,) -> bool { }
pub fn r#onPassengerLookAround(r#passenger : crate::root::minecraft::entity::Entity,) -> () { }
pub fn r#isTeamPlayer(r#team : crate::root::minecraft::scoreboard::AbstractTeam,) -> bool { }
pub fn r#extinguish() -> () { }
pub fn r#writeNbt(r#nbt : crate::root::minecraft::nbt::NbtCompound,) -> crate::root::minecraft::nbt::NbtCompound { }
pub fn r#setInvisible(r#invisible : bool,) -> () { }
pub fn r#squaredDistanceTo(r#x : f64,r#y : f64,r#z : f64,) -> f64 { }
pub fn r#remove(r#reason : jni::object::JObject,) -> () { }
pub fn r#readNbt(r#nbt : crate::root::minecraft::nbt::NbtCompound,) -> () { }
pub fn r#writeCustomDataToNbt(r#nbt : crate::root::minecraft::nbt::NbtCompound,) -> () { }
pub fn r#getSavedEntityId() -> jni::object::JObject { }
pub fn r#doesNotCollide(r#offsetX : f64,r#offsetY : f64,r#offsetZ : f64,) -> bool { }
pub fn r#isInvulnerable() -> bool { }
pub fn r#getPistonBehavior() -> crate::root::minecraft::block::piston::PistonBehavior { }
pub fn r#isImmuneToExplosion() -> bool { }
pub fn r#setSneaking(r#sneaking : bool,) -> () { }
pub fn r#getArmorItems() -> jni::object::JObject { }
pub fn r#saveNbt(r#nbt : crate::root::minecraft::nbt::NbtCompound,) -> bool { }
pub fn r#getRotationVecClient() -> crate::root::minecraft::util::math::Vec3d { }
pub fn r#interactAt(r#player : crate::root::minecraft::entity::player::PlayerEntity,r#hitPos : crate::root::minecraft::util::math::Vec3d,r#hand : crate::root::minecraft::util::Hand,) -> crate::root::minecraft::util::ActionResult { }
pub fn r#setCustomName(r#name : crate::root::minecraft::text::Text,) -> () { }
pub fn r#getRootVehicle() -> crate::root::minecraft::entity::Entity { }
pub fn r#getAir() -> i32 { }
pub fn r#baseTick() -> () { }
pub fn r#getCommandSource() -> crate::root::minecraft::server::command::ServerCommandSource { }
pub fn r#getHighSpeedSplashSound() -> crate::root::minecraft::sound::SoundEvent { }
pub fn r#equipStack(r#slot : crate::root::minecraft::entity::EquipmentSlot,r#stack : crate::root::minecraft::item::ItemStack,) -> () { }
pub fn r#onTrackedDataSet(r#data : crate::root::minecraft::entity::data::TrackedData,) -> () { }
pub fn r#isPushedByFluids() -> bool { }
pub fn r#getBurningDuration() -> i32 { }
pub fn r#getHeightOffset() -> f64 { }
pub fn r#isInvulnerableTo(r#damageSource : crate::root::minecraft::entity::damage::DamageSource,) -> bool { }
pub fn r#canBeSpectated(r#spectator : crate::root::minecraft::server::network::ServerPlayerEntity,) -> bool { }
pub fn r#isSwimming() -> bool { }
pub fn r#getServer() -> jni::object::JObject { }
pub fn r#updateTrackedHeadRotation(r#yaw : f32,r#interpolationSteps : i32,) -> () { }
pub fn r#setInvulnerable(r#invulnerable : bool,) -> () { }
pub fn r#getPassengerList() -> jni::object::JObject { }
pub fn r#hasPermissionLevel(r#permissionLevel : i32,) -> bool { }
pub fn r#interact(r#player : crate::root::minecraft::entity::player::PlayerEntity,r#hand : crate::root::minecraft::util::Hand,) -> crate::root::minecraft::util::ActionResult { }
pub fn r#getPermissionLevel() -> i32 { }
pub fn r#updateMovementInFluid(r#tag : crate::root::minecraft::tag::TagKey,r#speed : f64,) -> bool { }
pub fn r#initDataTracker() -> () { }
pub fn r#onPlayerCollision(r#player : crate::root::minecraft::entity::player::PlayerEntity,) -> () { }
pub fn r#getPitch(r#tickDelta : f32,) -> f32 { }
pub fn r#canAvoidTraps() -> bool { }
pub fn r#pushAwayFrom(r#entity : crate::root::minecraft::entity::Entity,) -> () { }
pub fn r#handleAttack(r#attacker : crate::root::minecraft::entity::Entity,) -> bool { }
pub fn r#dropStack(r#stack : crate::root::minecraft::item::ItemStack,r#yOffset : f32,) -> crate::root::minecraft::entity::ItemEntity { }
pub fn r#onBubbleColumnSurfaceCollision(r#drag : bool,) -> () { }
pub fn r#isSilent() -> bool { }
pub fn r#lookAt(r#anchorPoint : crate::root::minecraft::command::argument::EntityAnchor,r#target : crate::root::minecraft::util::math::Vec3d,) -> () { }
pub fn r#hasPassenger(r#predicate : jni::object::JObject,) -> bool { }
pub fn r#getYaw(r#tickDelta : f32,) -> f32 { }
pub fn r#dropItem(r#item : crate::root::minecraft::item::ItemConvertible,) -> crate::root::minecraft::entity::ItemEntity { }
pub fn r#squaredDistanceTo(r#vector : crate::root::minecraft::util::math::Vec3d,) -> f64 { }
pub fn r#isLiving() -> bool { }
pub fn r#setRotation(r#yaw : f32,r#pitch : f32,) -> () { }
pub fn r#handleStatus(r#status : u8,) -> () { }
pub fn r#playStepSound(r#pos : crate::root::minecraft::util::math::BlockPos,r#state : crate::root::minecraft::block::BlockState,) -> () { }
pub fn r#checkWaterState() -> () { }
pub fn r#isSneaking() -> bool { }
pub fn r#updateKilledAdvancementCriterion(r#entityKilled : crate::root::minecraft::entity::Entity,r#score : i32,r#damageSource : crate::root::minecraft::entity::damage::DamageSource,) -> () { }
pub fn r#setInNetherPortal(r#pos : crate::root::minecraft::util::math::BlockPos,) -> () { }
pub fn r#getBrightnessAtEyes() -> f32 { }
pub fn r#copyPositionAndRotation(r#entity : crate::root::minecraft::entity::Entity,) -> () { }
pub fn r#getRotationVector() -> crate::root::minecraft::util::math::Vec3d { }
pub fn r#isTouchingWaterOrRain() -> bool { }
pub fn r#isTeammate(r#other : crate::root::minecraft::entity::Entity,) -> bool { }
pub fn r#applyDamageEffects(r#attacker : crate::root::minecraft::entity::LivingEntity,r#target : crate::root::minecraft::entity::Entity,) -> () { }
pub fn r#updateVelocity(r#speed : f32,r#movementInput : crate::root::minecraft::util::math::Vec3d,) -> () { }
pub fn r#refreshPositionAndAngles(r#pos : crate::root::minecraft::util::math::BlockPos,r#yaw : f32,r#pitch : f32,) -> () { }
pub fn r#toNbtList(r#values : f32,) -> crate::root::minecraft::nbt::NbtList { }
pub fn r#shouldRender(r#cameraX : f64,r#cameraY : f64,r#cameraZ : f64,) -> bool { }
pub fn r#setSprinting(r#sprinting : bool,) -> () { }
pub fn r#setFlag(r#index : i32,r#value : bool,) -> () { }
pub fn r#setOnFireFromLava() -> () { }
pub fn r#moveToWorld(r#destination : crate::root::minecraft::server::world::ServerWorld,) -> crate::root::minecraft::entity::Entity { }
pub fn r#isAttackable() -> bool { }
pub fn r#shouldRenderName() -> bool { }
pub fn r#playSwimSound(r#volume : f32,) -> () { }
pub fn r#getHorizontalFacing() -> crate::root::minecraft::util::math::Direction { }
pub fn r#getPassengersDeep() -> jni::object::JObject { }
pub fn r#getSwimSound() -> crate::root::minecraft::sound::SoundEvent { }
pub fn r#removeScoreboardTag(r#tag : jni::object::JObject,) -> bool { }
pub fn r#distanceTo(r#entity : crate::root::minecraft::entity::Entity,) -> f32 { }
pub fn r#hasNoGravity() -> bool { }
pub fn r#getMaxNetherPortalTime() -> i32 { }
pub fn r#onStoppedTrackingBy(r#player : crate::root::minecraft::server::network::ServerPlayerEntity,) -> () { }
pub fn r#getItemsEquipped() -> jni::object::JObject { }
pub fn r#raycast(r#maxDistance : f64,r#tickDelta : f32,r#includeFluids : bool,) -> crate::root::minecraft::util::hit::HitResult { }
pub fn r#onSwimmingStart() -> () { }
pub fn r#handleFallDamage(r#fallDistance : f32,r#damageMultiplier : f32,r#damageSource : crate::root::minecraft::entity::damage::DamageSource,) -> bool { }
pub fn r#getMaxAir() -> i32 { }
pub fn r#readCustomDataFromNbt(r#nbt : crate::root::minecraft::nbt::NbtCompound,) -> () { }
pub fn r#setVelocityClient(r#x : f64,r#y : f64,r#z : f64,) -> () { }
pub fn r#getStandingEyeHeight() -> f32 { }
pub fn r#getScoreboardTags() -> jni::object::JObject { }
pub fn r#isFireImmune() -> bool { }
pub fn r#getMovementDirection() -> crate::root::minecraft::util::math::Direction { }
pub fn r#isInvisibleTo(r#player : crate::root::minecraft::entity::player::PlayerEntity,) -> bool { }
pub fn r#isInsideWall() -> bool { }
pub fn r#updateTrackedPositionAndAngles(r#x : f64,r#y : f64,r#z : f64,r#yaw : f32,r#pitch : f32,r#interpolationSteps : i32,r#interpolate : bool,) -> () { }
pub fn r#tickPortalCooldown() -> () { }
pub fn r#addVelocity(r#deltaX : f64,r#deltaY : f64,r#deltaZ : f64,) -> () { }
pub fn r#applyMirror(r#mirror : crate::root::minecraft::util::BlockMirror,) -> f32 { }
pub fn r#onBubbleColumnCollision(r#drag : bool,) -> () { }
pub fn r#hasVehicle() -> bool { }
pub fn r#isInvisible() -> bool { }
pub fn r#kill() -> () { }
pub fn r#getHoverEvent() -> crate::root::minecraft::text::HoverEvent { }
pub fn r#getEntityWorld() -> crate::root::minecraft::world::World { }
pub fn r#isInLava() -> bool { }
pub fn r#removeAllPassengers() -> () { }
pub fn r#tick() -> () { }
pub fn r#getEffectiveExplosionResistance(r#explosion : crate::root::minecraft::world::explosion::Explosion,r#world : crate::root::minecraft::world::BlockView,r#pos : crate::root::minecraft::util::math::BlockPos,r#blockState : crate::root::minecraft::block::BlockState,r#fluidState : crate::root::minecraft::fluid::FluidState,r#max : f32,) -> f32 { }
pub fn r#dropStack(r#stack : crate::root::minecraft::item::ItemStack,) -> crate::root::minecraft::entity::ItemEntity { }
pub fn r#hasWings() -> bool { }
pub fn r#isSubmergedIn(r#fluidTag : crate::root::minecraft::tag::TagKey,) -> bool { }
pub fn r#isBeingRainedOn() -> bool { }
pub fn r#isPartOf(r#entity : crate::root::minecraft::entity::Entity,) -> bool { }
pub fn r#addScoreboardTag(r#tag : jni::object::JObject,) -> bool { }
pub fn r#getScoreboardTeam() -> crate::root::minecraft::scoreboard::AbstractTeam { }
pub fn r#hasPassengers() -> bool { }
pub fn r#playSound(r#sound : crate::root::minecraft::sound::SoundEvent,r#volume : f32,r#pitch : f32,) -> () { }
pub fn r#move(r#movementType : crate::root::minecraft::entity::MovementType,r#movement : crate::root::minecraft::util::math::Vec3d,) -> () { }
pub fn r#scheduleVelocityUpdate() -> () { }
pub fn r#saveSelfNbt(r#nbt : crate::root::minecraft::nbt::NbtCompound,) -> bool { }
pub fn r#isLogicalSideForUpdatingMovement() -> bool { }
pub fn r#canBeRiddenInWater() -> bool { }
pub fn r#updateSwimming() -> () { }
pub fn r#getHeadYaw() -> f32 { }
pub fn r#removePassenger(r#passenger : crate::root::minecraft::entity::Entity,) -> () { }
pub fn r#isConnectedThroughVehicle(r#entity : crate::root::minecraft::entity::Entity,) -> bool { }
pub fn r#getFlag(r#index : i32,) -> bool { }
pub fn r#setSwimming(r#swimming : bool,) -> () { }
pub fn r#isInsideBubbleColumn() -> bool { }
pub fn r#isTouchingWater() -> bool { }
pub fn r#onStruckByLightning(r#world : crate::root::minecraft::server::world::ServerWorld,r#lightning : crate::root::minecraft::entity::LightningEntity,) -> () { }
pub fn r#addFlapEffects() -> () { }
pub fn r#getRotationClient() -> crate::root::minecraft::util::math::Vec2f { }
pub fn r#setSilent(r#silent : bool,) -> () { }
pub fn r#startRiding(r#entity : crate::root::minecraft::entity::Entity,) -> bool { }
pub fn r#isAlive() -> bool { }
pub fn r#getDefaultPortalCooldown() -> i32 { }
pub fn r#isCustomNameVisible() -> bool { }
pub fn r#refreshPositionAndAngles(r#x : f64,r#y : f64,r#z : f64,r#yaw : f32,r#pitch : f32,) -> () { }
pub fn r#isOnFire() -> bool { }
pub fn r#isPushable() -> bool { }
pub fn r#unknwnfn_958175633(r#style : crate::root::minecraft::text::Style,) -> crate::root::minecraft::text::Style { }
pub fn r#setPosition(r#x : f64,r#y : f64,r#z : f64,) -> () { }
pub fn r#isInsideWaterOrBubbleColumn() -> bool { }
pub fn r#hasPlayerRider() -> bool { }
pub fn r#canAddPassenger(r#passenger : crate::root::minecraft::entity::Entity,) -> bool { }
pub fn r#populateCrashReport(r#section : crate::root::minecraft::util::crash::CrashReportSection,) -> () { }
pub fn r#getEntityName() -> jni::object::JObject { }
pub fn r#hasPassengerDeep(r#passenger : crate::root::minecraft::entity::Entity,) -> bool { }
pub fn r#canUsePortals() -> bool { }
pub fn r#getRenderDistanceMultiplier() -> f64 { }
pub fn r#tickInVoid() -> () { }
pub fn r#setUuid(r#uuid : jni::object::JObject,) -> () { }
pub fn r#getRotationVec(r#tickDelta : f32,) -> crate::root::minecraft::util::math::Vec3d { }
pub fn r#getVisibilityBoundingBox() -> crate::root::minecraft::util::math::Box { }
pub fn r#applyRotation(r#rotation : crate::root::minecraft::util::BlockRotation,) -> f32 { }
pub fn r#entityDataRequiresOperator() -> bool { }
pub fn r#setGlowing(r#glowing : bool,) -> () { }
pub fn r#getCameraPosVec(r#tickDelta : f32,) -> crate::root::minecraft::util::math::Vec3d { }
pub fn r#onStartedTrackingBy(r#player : crate::root::minecraft::server::network::ServerPlayerEntity,) -> () { }
pub fn r#setId(r#id : i32,) -> () { }
pub fn r#spawnSprintingParticles() -> () { }
pub fn r#setRenderDistanceMultiplier(r#value : f64,) -> () { }
pub fn r#getDataTracker() -> crate::root::minecraft::entity::data::DataTracker { }
pub fn r#tickRiding() -> () { }
pub fn r#slowMovement(r#state : crate::root::minecraft::block::BlockState,r#multiplier : crate::root::minecraft::util::math::Vec3d,) -> () { }
pub fn r#getUuidAsString() -> jni::object::JObject { }
pub fn r#toNbtList(r#values : f64,) -> crate::root::minecraft::nbt::NbtList { }
pub fn r#setHeadYaw(r#headYaw : f32,) -> () { }
pub fn r#stopRiding() -> () { }
pub fn r#getSafeFallDistance() -> i32 { }
pub fn r#isGlowing() -> bool { }
pub fn r#checkBlockCollision() -> () { }
pub fn r#canExplosionDestroyBlock(r#explosion : crate::root::minecraft::world::explosion::Explosion,r#world : crate::root::minecraft::world::BlockView,r#pos : crate::root::minecraft::util::math::BlockPos,r#state : crate::root::minecraft::block::BlockState,r#explosionPower : f32,) -> bool { }
pub fn r#getVehicle() -> crate::root::minecraft::entity::Entity { }
pub fn r#setAir(r#air : i32,) -> () { }
pub fn r#removeClickEvents(r#textComponent : crate::root::minecraft::text::Text,) -> crate::root::minecraft::text::Text { }
pub fn r#setBoundingBox(r#boundingBox : crate::root::minecraft::util::math::Box,) -> () { }
pub fn r#squaredDistanceTo(r#entity : crate::root::minecraft::entity::Entity,) -> f64 { }
pub fn r#requestTeleport(r#destX : f64,r#destY : f64,r#destZ : f64,) -> () { }
pub fn r#canStartRiding(r#entity : crate::root::minecraft::entity::Entity,) -> bool { }
pub fn r#getFluidHeight(r#fluid : crate::root::minecraft::tag::TagKey,) -> f64 { }
pub fn r#doesRenderOnFire() -> bool { }
pub fn r#canHit() -> bool { }
pub fn r#getType() -> crate::root::minecraft::entity::EntityType { }
pub fn r#updatePassengerPosition(r#passenger : crate::root::minecraft::entity::Entity,) -> () { }
pub fn r#calculateNextStepSoundDistance() -> f32 { }
pub fn r#isSubmergedInWater() -> bool { }
pub fn r#dropItem(r#item : crate::root::minecraft::item::ItemConvertible,r#yOffset : i32,) -> crate::root::minecraft::entity::ItemEntity { }
pub fn r#getTargetingMargin() -> f32 { }
pub fn r#changeLookDirection(r#cursorDeltaX : f64,r#cursorDeltaY : f64,) -> () { }
pub fn r#startRiding(r#entity : crate::root::minecraft::entity::Entity,r#force : bool,) -> bool { }
pub fn r#onKilledOther(r#world : crate::root::minecraft::server::world::ServerWorld,r#other : crate::root::minecraft::entity::LivingEntity,) -> bool { }
pub fn r#setNoGravity(r#noGravity : bool,) -> () { }
pub fn r#updateWaterState() -> bool { }
pub fn r#getHandItems() -> jni::object::JObject { }
pub fn r#copyFrom(r#original : crate::root::minecraft::entity::Entity,) -> () { }
pub fn r#animateDamage() -> () { }
pub fn r#setCustomNameVisible(r#visible : bool,) -> () { }
pub fn r#checkDespawn() -> () { }
}
}
pub mod fluid {
pub struct r#LavaFluid<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for LavaFluid<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl LavaFluid<'_> {
const __map_sig : &str = "net/minecraft/class_3616";
pub fn r#hasBurnableBlock(r#world : crate::root::minecraft::world::WorldView,r#pos : crate::root::minecraft::util::math::BlockPos,) -> bool { }
pub fn r#playExtinguishEvent(r#world : crate::root::minecraft::world::WorldAccess,r#pos : crate::root::minecraft::util::math::BlockPos,) -> () { }
}
}
pub mod inventory {
}
pub mod item {
pub struct r#FireworkRocketItem<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for FireworkRocketItem<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl FireworkRocketItem<'_> {
const __map_sig : &str = "net/minecraft/class_1781";
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#get_FIREWORKS_KEY(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#set_FIREWORKS_KEY(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#get_EXPLOSION_KEY(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#set_EXPLOSION_KEY(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#get_EXPLOSIONS_KEY(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#set_EXPLOSIONS_KEY(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#get_FLIGHT_KEY(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#set_FLIGHT_KEY(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#get_TYPE_KEY(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#set_TYPE_KEY(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#get_TRAIL_KEY(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#set_TRAIL_KEY(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#get_FLICKER_KEY(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#set_FLICKER_KEY(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#get_COLORS_KEY(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#set_COLORS_KEY(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#get_FADE_COLORS_KEY(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#set_FADE_COLORS_KEY(&self,val : jni::object::JObject) -> () {} 
}
}
pub mod loot {
pub mod condition {
pub struct r#AlternativeLootCondition<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for AlternativeLootCondition<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl AlternativeLootCondition<'_> {
const __map_sig : &str = "net/minecraft/class_186";
/* std::vec::Vec<crate::root::minecraft::loot::condition::LootCondition> = [Lnet/minecraft/class_5341; */
pub fn r#get_terms(&self) -> Result<std::vec::Vec<crate::root::minecraft::loot::condition::LootCondition>,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* std::vec::Vec<crate::root::minecraft::loot::condition::LootCondition> = [Lnet/minecraft/class_5341; */
pub fn r#set_terms(&self,val : std::vec::Vec<crate::root::minecraft::loot::condition::LootCondition>) -> () {} 
/* jni::object::JObject = Ljava/util/function/Predicate; */
pub fn r#get_predicate(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/function/Predicate; */
pub fn r#set_predicate(&self,val : jni::object::JObject) -> () {} 
pub fn r#__zINDMINIT(r#terms : crate::root::minecraft::loot::condition::LootCondition,) -> () { }
pub fn r#builder(r#terms : crate::root::minecraft::loot::condition::Builder,) -> crate::root::minecraft::loot::condition::Builder { }
}
}
pub mod context {
pub struct r#LootContext<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for LootContext<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl LootContext<'_> {
const __map_sig : &str = "net/minecraft/class_47";
/* jni::object::JObject = Ljava/util/function/Function; */
pub fn r#get_conditionGetter(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/function/Function; */
pub fn r#set_conditionGetter(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/util/Set; */
pub fn r#get_conditions(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/Set; */
pub fn r#set_conditions(&self,val : jni::object::JObject) -> () {} 
/* crate::root::minecraft::util::math::random::Random = Lnet/minecraft/class_5819; */
pub fn r#get_random(&self) -> Result<crate::root::minecraft::util::math::random::Random,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::util::math::random::Random = Lnet/minecraft/class_5819; */
pub fn r#set_random(&self,val : crate::root::minecraft::util::math::random::Random) -> () {} 
/* jni::object::JObject = Ljava/util/function/Function; */
pub fn r#get_tableGetter(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/function/Function; */
pub fn r#set_tableGetter(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/util/Map; */
pub fn r#get_parameters(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/Map; */
pub fn r#set_parameters(&self,val : jni::object::JObject) -> () {} 
/* f32 = F */
pub fn r#get_luck(&self) -> Result<f32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f32 = F */
pub fn r#set_luck(&self,val : f32) -> () {} 
/* jni::object::JObject = Ljava/util/Set; */
pub fn r#get_activeTables(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/Set; */
pub fn r#set_activeTables(&self,val : jni::object::JObject) -> () {} 
/* crate::root::minecraft::server::world::ServerWorld = Lnet/minecraft/class_3218; */
pub fn r#get_world(&self) -> Result<crate::root::minecraft::server::world::ServerWorld,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::server::world::ServerWorld = Lnet/minecraft/class_3218; */
pub fn r#set_world(&self,val : crate::root::minecraft::server::world::ServerWorld) -> () {} 
/* jni::object::JObject = Ljava/util/Map; */
pub fn r#get_drops(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/Map; */
pub fn r#set_drops(&self,val : jni::object::JObject) -> () {} 
pub fn r#__zINDMINIT(r#random : crate::root::minecraft::util::math::random::Random,r#luck : f32,r#world : crate::root::minecraft::server::world::ServerWorld,r#tableGetter : jni::object::JObject,r#conditionGetter : jni::object::JObject,r#parameters : jni::object::JObject,r#drops : jni::object::JObject,) -> () { }
pub fn r#addCondition(r#condition : crate::root::minecraft::loot::condition::LootCondition,) -> bool { }
pub fn r#getTable(r#id : crate::root::minecraft::util::Identifier,) -> crate::root::minecraft::loot::LootTable { }
pub fn r#removeCondition(r#condition : crate::root::minecraft::loot::condition::LootCondition,) -> () { }
pub fn r#getCondition(r#id : crate::root::minecraft::util::Identifier,) -> crate::root::minecraft::loot::condition::LootCondition { }
pub fn r#getRandom() -> crate::root::minecraft::util::math::random::Random { }
pub fn r#markInactive(r#table : crate::root::minecraft::loot::LootTable,) -> () { }
pub fn r#get(r#parameter : crate::root::minecraft::loot::context::LootContextParameter,) -> jni::object::JObject { }
pub fn r#drop(r#id : crate::root::minecraft::util::Identifier,r#lootConsumer : jni::object::JObject,) -> () { }
pub fn r#markActive(r#table : crate::root::minecraft::loot::LootTable,) -> bool { }
pub fn r#getWorld() -> crate::root::minecraft::server::world::ServerWorld { }
pub fn r#hasParameter(r#parameter : crate::root::minecraft::loot::context::LootContextParameter,) -> bool { }
pub fn r#getLuck() -> f32 { }
}
}
pub mod entry {
pub struct r#AlternativeEntry<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for AlternativeEntry<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl AlternativeEntry<'_> {
const __map_sig : &str = "net/minecraft/class_65";
pub fn r#builder(r#children : crate::root::minecraft::loot::entry::Builder,) -> crate::root::minecraft::loot::entry::Builder { }
pub fn r#unknwnfn_1723940699(r#context : crate::root::minecraft::loot::entry::EntryCombiner,r#lootChoiceExpander : crate::root::minecraft::loot::context::LootContext,r#unknwnarg_1746732568 : jni::object::JObject,) -> bool { }
}
}
pub mod function {
pub struct r#ApplyBonusLootFunction<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for ApplyBonusLootFunction<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl ApplyBonusLootFunction<'_> {
const __map_sig : &str = "net/minecraft/class_94";
/* jni::object::JObject = Lnet/minecraft/class_94$class_96; */
pub fn r#get_formula(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lnet/minecraft/class_94$class_96; */
pub fn r#set_formula(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/util/Map; */
pub fn r#get_FACTORIES(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/Map; */
pub fn r#set_FACTORIES(&self,val : jni::object::JObject) -> () {} 
/* crate::root::minecraft::enchantment::Enchantment = Lnet/minecraft/class_1887; */
pub fn r#get_enchantment(&self) -> Result<crate::root::minecraft::enchantment::Enchantment,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::enchantment::Enchantment = Lnet/minecraft/class_1887; */
pub fn r#set_enchantment(&self,val : crate::root::minecraft::enchantment::Enchantment) -> () {} 
pub fn r#__zINDMINIT(r#conditions : crate::root::minecraft::loot::condition::LootCondition,r#enchantment : crate::root::minecraft::enchantment::Enchantment,r#formula : jni::object::JObject,) -> () { }
pub fn r#unknwnfn_1882579518(r#conditions : crate::root::minecraft::enchantment::Enchantment,r#unknwnarg_814403136 : crate::root::minecraft::loot::condition::LootCondition,) -> crate::root::minecraft::loot::function::LootFunction { }
pub fn r#oreDrops(r#enchantment : crate::root::minecraft::enchantment::Enchantment,) -> crate::root::minecraft::loot::function::Builder { }
pub fn r#uniformBonusCount(r#enchantment : crate::root::minecraft::enchantment::Enchantment,) -> crate::root::minecraft::loot::function::Builder { }
pub fn r#unknwnfn_3360278548(r#conditions : crate::root::minecraft::enchantment::Enchantment,r#unknwnarg_1550835207 : crate::root::minecraft::loot::condition::LootCondition,) -> crate::root::minecraft::loot::function::LootFunction { }
pub fn r#unknwnfn_823644949(r#conditions : crate::root::minecraft::enchantment::Enchantment,r#unknwnarg_2949420658 : i32,r#unknwnarg_798629365 : f32,r#unknwnarg_3117052385 : crate::root::minecraft::loot::condition::LootCondition,) -> crate::root::minecraft::loot::function::LootFunction { }
pub fn r#uniformBonusCount(r#enchantment : crate::root::minecraft::enchantment::Enchantment,r#bonusMultiplier : i32,) -> crate::root::minecraft::loot::function::Builder { }
pub fn r#unknwnfn_3706745285(r#conditions : crate::root::minecraft::enchantment::Enchantment,r#unknwnarg_3719610935 : i32,r#unknwnarg_326642055 : crate::root::minecraft::loot::condition::LootCondition,) -> crate::root::minecraft::loot::function::LootFunction { }
}
}
pub struct r#LootPool<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for LootPool<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl LootPool<'_> {
const __map_sig : &str = "net/minecraft/class_55";
/* jni::object::JObject = Ljava/util/function/BiFunction; */
pub fn r#get_javaFunctions(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/function/BiFunction; */
pub fn r#set_javaFunctions(&self,val : jni::object::JObject) -> () {} 
/* std::vec::Vec<crate::root::minecraft::loot::entry::LootPoolEntry> = [Lnet/minecraft/class_79; */
pub fn r#get_entries(&self) -> Result<std::vec::Vec<crate::root::minecraft::loot::entry::LootPoolEntry>,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* std::vec::Vec<crate::root::minecraft::loot::entry::LootPoolEntry> = [Lnet/minecraft/class_79; */
pub fn r#set_entries(&self,val : std::vec::Vec<crate::root::minecraft::loot::entry::LootPoolEntry>) -> () {} 
/* std::vec::Vec<crate::root::minecraft::loot::condition::LootCondition> = [Lnet/minecraft/class_5341; */
pub fn r#get_conditions(&self) -> Result<std::vec::Vec<crate::root::minecraft::loot::condition::LootCondition>,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* std::vec::Vec<crate::root::minecraft::loot::condition::LootCondition> = [Lnet/minecraft/class_5341; */
pub fn r#set_conditions(&self,val : std::vec::Vec<crate::root::minecraft::loot::condition::LootCondition>) -> () {} 
/* jni::object::JObject = Ljava/util/function/Predicate; */
pub fn r#get_predicate(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/function/Predicate; */
pub fn r#set_predicate(&self,val : jni::object::JObject) -> () {} 
/* std::vec::Vec<crate::root::minecraft::loot::function::LootFunction> = [Lnet/minecraft/class_117; */
pub fn r#get_functions(&self) -> Result<std::vec::Vec<crate::root::minecraft::loot::function::LootFunction>,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* std::vec::Vec<crate::root::minecraft::loot::function::LootFunction> = [Lnet/minecraft/class_117; */
pub fn r#set_functions(&self,val : std::vec::Vec<crate::root::minecraft::loot::function::LootFunction>) -> () {} 
/* crate::root::minecraft::loot::provider::number::LootNumberProvider = Lnet/minecraft/class_5658; */
pub fn r#get_rolls(&self) -> Result<crate::root::minecraft::loot::provider::number::LootNumberProvider,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::loot::provider::number::LootNumberProvider = Lnet/minecraft/class_5658; */
pub fn r#set_rolls(&self,val : crate::root::minecraft::loot::provider::number::LootNumberProvider) -> () {} 
/* crate::root::minecraft::loot::provider::number::LootNumberProvider = Lnet/minecraft/class_5658; */
pub fn r#get_bonusRolls(&self) -> Result<crate::root::minecraft::loot::provider::number::LootNumberProvider,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::loot::provider::number::LootNumberProvider = Lnet/minecraft/class_5658; */
pub fn r#set_bonusRolls(&self,val : crate::root::minecraft::loot::provider::number::LootNumberProvider) -> () {} 
pub fn r#__zINDMINIT(r#entries : crate::root::minecraft::loot::entry::LootPoolEntry,r#conditions : crate::root::minecraft::loot::condition::LootCondition,r#functions : crate::root::minecraft::loot::function::LootFunction,r#rolls : crate::root::minecraft::loot::provider::number::LootNumberProvider,r#bonusRolls : crate::root::minecraft::loot::provider::number::LootNumberProvider,) -> () { }
pub fn r#addGeneratedLoot(r#lootConsumer : jni::object::JObject,r#context : crate::root::minecraft::loot::context::LootContext,) -> () { }
pub fn r#unknwnfn_3372915791(r#choice : crate::root::minecraft::loot::context::LootContext,r#unknwnarg_21250609 : jni::object::JObject,r#unknwnarg_1951455832 : jni::object::JObject,r#unknwnarg_3150133285 : crate::root::minecraft::loot::LootChoice,) -> () { }
pub fn r#supplyOnce(r#lootConsumer : jni::object::JObject,r#context : crate::root::minecraft::loot::context::LootContext,) -> () { }
pub fn r#builder() -> crate::root::minecraft::loot::Builder { }
}
}
pub mod nbt {
pub struct r#NbtByte<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for NbtByte<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl NbtByte<'_> {
const __map_sig : &str = "net/minecraft/class_2481";
/* u8 = B */
pub fn r#get_value(&self) -> Result<u8,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* u8 = B */
pub fn r#set_value(&self,val : u8) -> () {} 
/* crate::root::minecraft::nbt::NbtType = Lnet/minecraft/class_4614; */
pub fn r#get_TYPE(&self) -> Result<crate::root::minecraft::nbt::NbtType,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::nbt::NbtType = Lnet/minecraft/class_4614; */
pub fn r#set_TYPE(&self,val : crate::root::minecraft::nbt::NbtType) -> () {} 
/* crate::root::minecraft::nbt::NbtByte = Lnet/minecraft/class_2481; */
pub fn r#get_ZERO(&self) -> Result<crate::root::minecraft::nbt::NbtByte,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::nbt::NbtByte = Lnet/minecraft/class_2481; */
pub fn r#set_ZERO(&self,val : crate::root::minecraft::nbt::NbtByte) -> () {} 
/* crate::root::minecraft::nbt::NbtByte = Lnet/minecraft/class_2481; */
pub fn r#get_ONE(&self) -> Result<crate::root::minecraft::nbt::NbtByte,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::nbt::NbtByte = Lnet/minecraft/class_2481; */
pub fn r#set_ONE(&self,val : crate::root::minecraft::nbt::NbtByte) -> () {} 
/* i32 = I */
pub fn r#get_SIZE(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_SIZE(&self,val : i32) -> () {} 
pub fn r#__zINDMINIT(r#value : u8,) -> () { }
pub fn r#unknwnfn_2752100075(r#o : jni::object::JObject,) -> bool { }
pub fn r#of(r#value : u8,) -> crate::root::minecraft::nbt::NbtByte { }
}
}
pub mod network {
pub struct r#ClientConnection<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for ClientConnection<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl ClientConnection<'_> {
const __map_sig : &str = "net/minecraft/class_2535";
/* jni::object::JObject = Lorg/slf4j/Marker; */
pub fn r#get_NETWORK_PACKETS_MARKER(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lorg/slf4j/Marker; */
pub fn r#set_NETWORK_PACKETS_MARKER(&self,val : jni::object::JObject) -> () {} 
/* bool = Z */
pub fn r#get_errored(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_errored(&self,val : bool) -> () {} 
/* jni::object::JObject = Lorg/slf4j/Marker; */
pub fn r#get_NETWORK_MARKER(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lorg/slf4j/Marker; */
pub fn r#set_NETWORK_MARKER(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Lorg/slf4j/Logger; */
pub fn r#get_LOGGER(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lorg/slf4j/Logger; */
pub fn r#set_LOGGER(&self,val : jni::object::JObject) -> () {} 
/* crate::root::minecraft::network::NetworkSide = Lnet/minecraft/class_2598; */
pub fn r#get_side(&self) -> Result<crate::root::minecraft::network::NetworkSide,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::network::NetworkSide = Lnet/minecraft/class_2598; */
pub fn r#set_side(&self,val : crate::root::minecraft::network::NetworkSide) -> () {} 
/* jni::object::JObject = Ljava/util/Queue; */
pub fn r#get_packetQueue(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/Queue; */
pub fn r#set_packetQueue(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/net/SocketAddress; */
pub fn r#get_address(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/net/SocketAddress; */
pub fn r#set_address(&self,val : jni::object::JObject) -> () {} 
/* bool = Z */
pub fn r#get_disconnected(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_disconnected(&self,val : bool) -> () {} 
/* bool = Z */
pub fn r#get_encrypted(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_encrypted(&self,val : bool) -> () {} 
/* jni::object::JObject = Lio/netty/util/AttributeKey; */
pub fn r#get_PROTOCOL_ATTRIBUTE_KEY(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lio/netty/util/AttributeKey; */
pub fn r#set_PROTOCOL_ATTRIBUTE_KEY(&self,val : jni::object::JObject) -> () {} 
/* crate::root::minecraft::util::Lazy = Lnet/minecraft/class_3528; */
pub fn r#get_LOCAL_CLIENT_IO_GROUP(&self) -> Result<crate::root::minecraft::util::Lazy,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::util::Lazy = Lnet/minecraft/class_3528; */
pub fn r#set_LOCAL_CLIENT_IO_GROUP(&self,val : crate::root::minecraft::util::Lazy) -> () {} 
/* crate::root::minecraft::util::Lazy = Lnet/minecraft/class_3528; */
pub fn r#get_CLIENT_IO_GROUP(&self) -> Result<crate::root::minecraft::util::Lazy,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::util::Lazy = Lnet/minecraft/class_3528; */
pub fn r#set_CLIENT_IO_GROUP(&self,val : crate::root::minecraft::util::Lazy) -> () {} 
/* jni::object::JObject = Lio/netty/channel/Channel; */
pub fn r#get_channel(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lio/netty/channel/Channel; */
pub fn r#set_channel(&self,val : jni::object::JObject) -> () {} 
/* crate::root::minecraft::network::listener::PacketListener = Lnet/minecraft/class_2547; */
pub fn r#get_packetListener(&self) -> Result<crate::root::minecraft::network::listener::PacketListener,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::network::listener::PacketListener = Lnet/minecraft/class_2547; */
pub fn r#set_packetListener(&self,val : crate::root::minecraft::network::listener::PacketListener) -> () {} 
/* f32 = F */
pub fn r#get_averagePacketsSent(&self) -> Result<f32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f32 = F */
pub fn r#set_averagePacketsSent(&self,val : f32) -> () {} 
/* f32 = F */
pub fn r#get_averagePacketsReceived(&self) -> Result<f32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f32 = F */
pub fn r#set_averagePacketsReceived(&self,val : f32) -> () {} 
/* i32 = I */
pub fn r#get_ticks(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_ticks(&self,val : i32) -> () {} 
/* i32 = I */
pub fn r#get_packetsSentCounter(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_packetsSentCounter(&self,val : i32) -> () {} 
/* crate::root::minecraft::util::Lazy = Lnet/minecraft/class_3528; */
pub fn r#get_EPOLL_CLIENT_IO_GROUP(&self) -> Result<crate::root::minecraft::util::Lazy,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::util::Lazy = Lnet/minecraft/class_3528; */
pub fn r#set_EPOLL_CLIENT_IO_GROUP(&self,val : crate::root::minecraft::util::Lazy) -> () {} 
/* i32 = I */
pub fn r#get_packetsReceivedCounter(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_packetsReceivedCounter(&self,val : i32) -> () {} 
/* crate::root::minecraft::text::Text = Lnet/minecraft/class_2561; */
pub fn r#get_disconnectReason(&self) -> Result<crate::root::minecraft::text::Text,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::text::Text = Lnet/minecraft/class_2561; */
pub fn r#set_disconnectReason(&self,val : crate::root::minecraft::text::Text) -> () {} 
/* f32 = F */
pub fn r#get_CURRENT_PACKET_COUNTER_WEIGHT(&self) -> Result<f32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f32 = F */
pub fn r#set_CURRENT_PACKET_COUNTER_WEIGHT(&self,val : f32) -> () {} 
/* jni::object::JObject = Lorg/slf4j/Marker; */
pub fn r#get_PACKET_RECEIVED_MARKER(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lorg/slf4j/Marker; */
pub fn r#set_PACKET_RECEIVED_MARKER(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Lorg/slf4j/Marker; */
pub fn r#get_PACKET_SENT_MARKER(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lorg/slf4j/Marker; */
pub fn r#set_PACKET_SENT_MARKER(&self,val : jni::object::JObject) -> () {} 
pub fn r#__zINDMINIT(r#side : crate::root::minecraft::network::NetworkSide,) -> () { }
pub fn r#unknwnfn_632405366(r#context : jni::object::JObject,) -> () { }
pub fn r#unknwnfn_3823373304(r#context : jni::object::JObject,) -> () { }
pub fn r#unknwnfn_3385591986(r#context : jni::object::JObject,r#packet : jni::object::JObject,) -> () { }
pub fn r#unknwnfn_776899003(r#context : jni::object::JObject,r#ex : jni::object::JObject,) -> () { }
pub fn r#send(r#packet : crate::root::minecraft::network::Packet,) -> () { }
pub fn r#getPacketListener() -> crate::root::minecraft::network::listener::PacketListener { }
pub fn r#getAveragePacketsSent() -> f32 { }
pub fn r#setupEncryption(r#decryptionCipher : jni::object::JObject,r#encryptionCipher : jni::object::JObject,) -> () { }
pub fn r#disconnect(r#disconnectReason : crate::root::minecraft::text::Text,) -> () { }
pub fn r#getDisconnectReason() -> crate::root::minecraft::text::Text { }
pub fn r#setState(r#state : crate::root::minecraft::network::NetworkState,) -> () { }
pub fn r#sendQueuedPackets() -> () { }
pub fn r#send(r#packet : crate::root::minecraft::network::Packet,r#callbacks : crate::root::minecraft::network::PacketCallbacks,) -> () { }
pub fn r#connect(r#address : jni::object::JObject,r#useEpoll : bool,) -> crate::root::minecraft::network::ClientConnection { }
pub fn r#tick() -> () { }
pub fn r#getAddress() -> jni::object::JObject { }
pub fn r#isLocal() -> bool { }
pub fn r#disableAutoRead() -> () { }
pub fn r#isOpen() -> bool { }
pub fn r#handlePacket(r#packet : crate::root::minecraft::network::Packet,r#listener : crate::root::minecraft::network::listener::PacketListener,) -> () { }
pub fn r#setCompressionThreshold(r#compressionThreshold : i32,r#rejectsBadPackets : bool,) -> () { }
pub fn r#getAveragePacketsReceived() -> f32 { }
pub fn r#setPacketListener(r#listener : crate::root::minecraft::network::listener::PacketListener,) -> () { }
pub fn r#sendImmediately(r#packet : crate::root::minecraft::network::Packet,r#callbacks : crate::root::minecraft::network::PacketCallbacks,) -> () { }
pub fn r#handleDisconnection() -> () { }
pub fn r#connectLocal(r#address : jni::object::JObject,) -> crate::root::minecraft::network::ClientConnection { }
pub fn r#isEncrypted() -> bool { }
pub fn r#hasChannel() -> bool { }
pub fn r#updateStats() -> () { }
pub fn r#getState() -> crate::root::minecraft::network::NetworkState { }
pub fn r#getSide() -> crate::root::minecraft::network::NetworkSide { }
pub fn r#getOppositeSide() -> crate::root::minecraft::network::NetworkSide { }
pub fn r#sendInternal(r#packet : crate::root::minecraft::network::Packet,r#callbacks : crate::root::minecraft::network::PacketCallbacks,r#packetState : crate::root::minecraft::network::NetworkState,r#currentState : crate::root::minecraft::network::NetworkState,) -> () { }
pub fn r#unknwnfn_1457332962(r#marker : jni::object::JObject,) -> () { }
pub fn r#unknwnfn_586619585(r#marker : jni::object::JObject,) -> () { }
pub fn r#unknwnfn_2924090116(r#marker : jni::object::JObject,) -> () { }
}
}
pub mod obfuscate {
}
pub mod particle {
pub struct r#ParticleEffect<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for ParticleEffect<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl ParticleEffect<'_> {
const __map_sig : &str = "net/minecraft/class_2394";
pub fn r#asString() -> jni::object::JObject { }
pub fn r#write(r#buf : crate::root::minecraft::network::PacketByteBuf,) -> () { }
}
}
pub mod potion {
}
pub mod predicate {
pub mod block {
}
pub struct r#BlockPredicate<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for BlockPredicate<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl BlockPredicate<'_> {
const __map_sig : &str = "net/minecraft/class_4550";
/* crate::root::minecraft::predicate::BlockPredicate = Lnet/minecraft/class_4550; */
pub fn r#get_ANY(&self) -> Result<crate::root::minecraft::predicate::BlockPredicate,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::predicate::BlockPredicate = Lnet/minecraft/class_4550; */
pub fn r#set_ANY(&self,val : crate::root::minecraft::predicate::BlockPredicate) -> () {} 
/* crate::root::minecraft::tag::TagKey = Lnet/minecraft/class_6862; */
pub fn r#get_tag(&self) -> Result<crate::root::minecraft::tag::TagKey,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::tag::TagKey = Lnet/minecraft/class_6862; */
pub fn r#set_tag(&self,val : crate::root::minecraft::tag::TagKey) -> () {} 
/* jni::object::JObject = Ljava/util/Set; */
pub fn r#get_blocks(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/Set; */
pub fn r#set_blocks(&self,val : jni::object::JObject) -> () {} 
/* crate::root::minecraft::predicate::StatePredicate = Lnet/minecraft/class_4559; */
pub fn r#get_state(&self) -> Result<crate::root::minecraft::predicate::StatePredicate,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::predicate::StatePredicate = Lnet/minecraft/class_4559; */
pub fn r#set_state(&self,val : crate::root::minecraft::predicate::StatePredicate) -> () {} 
/* crate::root::minecraft::predicate::NbtPredicate = Lnet/minecraft/class_2105; */
pub fn r#get_nbt(&self) -> Result<crate::root::minecraft::predicate::NbtPredicate,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::predicate::NbtPredicate = Lnet/minecraft/class_2105; */
pub fn r#set_nbt(&self,val : crate::root::minecraft::predicate::NbtPredicate) -> () {} 
pub fn r#__zINDMINIT(r#tag : crate::root::minecraft::tag::TagKey,r#blocks : jni::object::JObject,r#state : crate::root::minecraft::predicate::StatePredicate,r#nbt : crate::root::minecraft::predicate::NbtPredicate,) -> () { }
pub fn r#toJson() -> jni::object::JObject { }
pub fn r#fromJson(r#json : jni::object::JObject,) -> crate::root::minecraft::predicate::BlockPredicate { }
pub fn r#test(r#world : crate::root::minecraft::server::world::ServerWorld,r#pos : crate::root::minecraft::util::math::BlockPos,) -> bool { }
}
}
pub mod recipe {
pub mod book {
pub struct r#RecipeBookOptions<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for RecipeBookOptions<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl RecipeBookOptions<'_> {
const __map_sig : &str = "net/minecraft/class_5411";
/* jni::object::JObject = Ljava/util/Map; */
pub fn r#get_CATEGORY_OPTION_NAMES(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/Map; */
pub fn r#set_CATEGORY_OPTION_NAMES(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/util/Map; */
pub fn r#get_categoryOptions(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/Map; */
pub fn r#set_categoryOptions(&self,val : jni::object::JObject) -> () {} 
pub fn r#__zINDMINIT(r#categoryOptions : jni::object::JObject,) -> () { }
pub fn r#unknwnfn_1587272841(r#o : jni::object::JObject,) -> bool { }
pub fn r#copy() -> crate::root::minecraft::recipe::book::RecipeBookOptions { }
pub fn r#copyFrom(r#other : crate::root::minecraft::recipe::book::RecipeBookOptions,) -> () { }
pub fn r#isGuiOpen(r#category : crate::root::minecraft::recipe::book::RecipeBookCategory,) -> bool { }
pub fn r#setGuiOpen(r#category : crate::root::minecraft::recipe::book::RecipeBookCategory,r#open : bool,) -> () { }
pub fn r#unknwnfn_2763637574(r#categoryOptions : jni::object::JObject,) -> () { }
pub fn r#fromNbt(r#nbt : crate::root::minecraft::nbt::NbtCompound,) -> crate::root::minecraft::recipe::book::RecipeBookOptions { }
pub fn r#unknwnfn_153093358(r#category : crate::root::minecraft::nbt::NbtCompound,r#pair : crate::root::minecraft::recipe::book::RecipeBookCategory,r#unknwnarg_2756890928 : jni::object::JObject,) -> () { }
pub fn r#unknwnfn_2374154779(r#category : crate::root::minecraft::nbt::NbtCompound,r#pair : jni::object::JObject,r#unknwnarg_3491921231 : crate::root::minecraft::recipe::book::RecipeBookCategory,r#unknwnarg_397015162 : jni::object::JObject,) -> () { }
pub fn r#fromPacket(r#buf : crate::root::minecraft::network::PacketByteBuf,) -> crate::root::minecraft::recipe::book::RecipeBookOptions { }
pub fn r#isFilteringCraftable(r#category : crate::root::minecraft::recipe::book::RecipeBookCategory,) -> bool { }
pub fn r#setFilteringCraftable(r#category : crate::root::minecraft::recipe::book::RecipeBookCategory,r#filtering : bool,) -> () { }
pub fn r#writeNbt(r#nbt : crate::root::minecraft::nbt::NbtCompound,) -> () { }
}
}
pub struct r#BrewingRecipeRegistry<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for BrewingRecipeRegistry<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl BrewingRecipeRegistry<'_> {
const __map_sig : &str = "net/minecraft/class_1845";
/* jni::object::JObject = Ljava/util/List; */
pub fn r#get_POTION_RECIPES(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/List; */
pub fn r#set_POTION_RECIPES(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/util/List; */
pub fn r#get_POTION_TYPES(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/List; */
pub fn r#set_POTION_TYPES(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/util/function/Predicate; */
pub fn r#get_POTION_TYPE_PREDICATE(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/function/Predicate; */
pub fn r#set_POTION_TYPE_PREDICATE(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/util/List; */
pub fn r#get_ITEM_RECIPES(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/List; */
pub fn r#set_ITEM_RECIPES(&self,val : jni::object::JObject) -> () {} 
pub fn r#isBrewable(r#potion : crate::root::minecraft::potion::Potion,) -> bool { }
pub fn r#isPotionRecipeIngredient(r#stack : crate::root::minecraft::item::ItemStack,) -> bool { }
pub fn r#hasItemRecipe(r#input : crate::root::minecraft::item::ItemStack,r#ingredient : crate::root::minecraft::item::ItemStack,) -> bool { }
pub fn r#registerItemRecipe(r#input : crate::root::minecraft::item::Item,r#ingredient : crate::root::minecraft::item::Item,r#output : crate::root::minecraft::item::Item,) -> () { }
pub fn r#hasRecipe(r#input : crate::root::minecraft::item::ItemStack,r#ingredient : crate::root::minecraft::item::ItemStack,) -> bool { }
pub fn r#unknwnfn_3323226125(r#stack : crate::root::minecraft::item::ItemStack,) -> bool { }
pub fn r#registerPotionRecipe(r#input : crate::root::minecraft::potion::Potion,r#item : crate::root::minecraft::item::Item,r#output : crate::root::minecraft::potion::Potion,) -> () { }
pub fn r#hasPotionRecipe(r#input : crate::root::minecraft::item::ItemStack,r#ingredient : crate::root::minecraft::item::ItemStack,) -> bool { }
pub fn r#registerDefaults() -> () { }
pub fn r#isValidIngredient(r#stack : crate::root::minecraft::item::ItemStack,) -> bool { }
pub fn r#craft(r#ingredient : crate::root::minecraft::item::ItemStack,r#input : crate::root::minecraft::item::ItemStack,) -> crate::root::minecraft::item::ItemStack { }
pub fn r#isItemRecipeIngredient(r#stack : crate::root::minecraft::item::ItemStack,) -> bool { }
}
}
pub mod resource {
pub mod metadata {
pub struct r#ResourceFilter<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for ResourceFilter<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl ResourceFilter<'_> {
const __map_sig : &str = "net/minecraft/class_7084";
/* crate::root::minecraft::resource::metadata::ResourceMetadataReader = Lnet/minecraft/class_3270; */
pub fn r#get_READER(&self) -> Result<crate::root::minecraft::resource::metadata::ResourceMetadataReader,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::resource::metadata::ResourceMetadataReader = Lnet/minecraft/class_3270; */
pub fn r#set_READER(&self,val : crate::root::minecraft::resource::metadata::ResourceMetadataReader) -> () {} 
/* jni::object::JObject = Lorg/slf4j/Logger; */
pub fn r#get_LOGGER(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lorg/slf4j/Logger; */
pub fn r#set_LOGGER(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Lcom/mojang/serialization/Codec; */
pub fn r#get_CODEC(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lcom/mojang/serialization/Codec; */
pub fn r#set_CODEC(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/util/List; */
pub fn r#get_blocks(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/List; */
pub fn r#set_blocks(&self,val : jni::object::JObject) -> () {} 
pub fn r#__zINDMINIT(r#blocks : jni::object::JObject,) -> () { }
pub fn r#unknwnfn_3942218077(r#filter : crate::root::minecraft::resource::metadata::ResourceFilter,) -> jni::object::JObject { }
pub fn r#unknwnfn_3153260746(r#instance : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#isNamespaceBlocked(r#namespace : jni::object::JObject,) -> bool { }
pub fn r#unknwnfn_3287964039(r#block : jni::object::JObject,r#unknwnarg_1480073386 : crate::root::minecraft::resource::metadata::BlockEntry,) -> bool { }
pub fn r#isPathBlocked(r#namespace : jni::object::JObject,) -> bool { }
}
}
pub struct r#NamespaceResourceManager<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for NamespaceResourceManager<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl NamespaceResourceManager<'_> {
const __map_sig : &str = "net/minecraft/class_3294";
/* jni::object::JObject = Ljava/util/List; */
pub fn r#get_packList(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/List; */
pub fn r#set_packList(&self,val : jni::object::JObject) -> () {} 
/* crate::root::minecraft::resource::ResourceType = Lnet/minecraft/class_3264; */
pub fn r#get_type(&self) -> Result<crate::root::minecraft::resource::ResourceType,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::resource::ResourceType = Lnet/minecraft/class_3264; */
pub fn r#set_type(&self,val : crate::root::minecraft::resource::ResourceType) -> () {} 
/* jni::object::JObject = Lorg/slf4j/Logger; */
pub fn r#get_LOGGER(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lorg/slf4j/Logger; */
pub fn r#set_LOGGER(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#get_namespace(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#set_namespace(&self,val : jni::object::JObject) -> () {} 
pub fn r#__zINDMINIT(r#type : crate::root::minecraft::resource::ResourceType,r#namespace : jni::object::JObject,) -> () { }
pub fn r#getMetadataPath(r#id : crate::root::minecraft::util::Identifier,) -> crate::root::minecraft::util::Identifier { }
pub fn r#createOpener(r#id : crate::root::minecraft::util::Identifier,r#pack : crate::root::minecraft::resource::ResourcePack,) -> crate::root::minecraft::resource::InputSupplier { }
pub fn r#isPathAbsolute(r#id : crate::root::minecraft::util::Identifier,) -> bool { }
pub fn r#addPack(r#pack : crate::root::minecraft::resource::ResourcePack,) -> () { }
pub fn r#addPack(r#pack : crate::root::minecraft::resource::ResourcePack,r#filter : jni::object::JObject,) -> () { }
pub fn r#unknwnfn_366404289(r#pack : jni::object::JObject,) -> crate::root::minecraft::resource::ResourcePack { }
pub fn r#findAndAdd(r#pack : jni::object::JObject,r#startingPath : jni::object::JObject,r#allowedPathPredicate : jni::object::JObject,r#idToEntryList : jni::object::JObject,) -> () { }
pub fn r#applyFilter(r#pack : jni::object::JObject,r#idToEntryList : jni::object::JObject,) -> () { }
pub fn r#addPack(r#name : jni::object::JObject,r#underlyingPack : crate::root::minecraft::resource::ResourcePack,r#filter : jni::object::JObject,) -> () { }
pub fn r#addPack(r#name : jni::object::JObject,r#filter : jni::object::JObject,) -> () { }
pub fn r#unknwnfn_4029680463(r#id : jni::object::JObject,r#entryList : crate::root::minecraft::util::Identifier,r#unknwnarg_3352603698 : jni::object::JObject,) -> () { }
pub fn r#unknwnfn_2977198011(r#id : crate::root::minecraft::util::Identifier,r#unknwnarg_1277851379 : crate::root::minecraft::util::Identifier,) -> jni::object::JObject { }
}
}
pub mod scoreboard {
pub struct r#AbstractTeam<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for AbstractTeam<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl AbstractTeam<'_> {
const __map_sig : &str = "net/minecraft/class_270";
pub fn r#getName() -> jni::object::JObject { }
pub fn r#decorateName(r#name : crate::root::minecraft::text::Text,) -> crate::root::minecraft::text::MutableText { }
pub fn r#shouldShowFriendlyInvisibles() -> bool { }
pub fn r#getDeathMessageVisibilityRule() -> jni::object::JObject { }
pub fn r#getNameTagVisibilityRule() -> jni::object::JObject { }
pub fn r#getColor() -> crate::root::minecraft::util::Formatting { }
pub fn r#getCollisionRule() -> crate::root::minecraft::scoreboard::CollisionRule { }
pub fn r#getPlayerList() -> jni::object::JObject { }
pub fn r#isFriendlyFireAllowed() -> bool { }
}
}
pub mod screen {
pub struct r#BeaconScreenHandler<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for BeaconScreenHandler<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl BeaconScreenHandler<'_> {
const __map_sig : &str = "net/minecraft/class_1704";
/* crate::root::minecraft::inventory::Inventory = Lnet/minecraft/class_1263; */
pub fn r#get_payment(&self) -> Result<crate::root::minecraft::inventory::Inventory,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::inventory::Inventory = Lnet/minecraft/class_1263; */
pub fn r#set_payment(&self,val : crate::root::minecraft::inventory::Inventory) -> () {} 
/* crate::root::minecraft::screen::PaymentSlot = Lnet/minecraft/class_1704$class_1705; */
pub fn r#get_paymentSlot(&self) -> Result<crate::root::minecraft::screen::PaymentSlot,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::screen::PaymentSlot = Lnet/minecraft/class_1704$class_1705; */
pub fn r#set_paymentSlot(&self,val : crate::root::minecraft::screen::PaymentSlot) -> () {} 
/* crate::root::minecraft::screen::ScreenHandlerContext = Lnet/minecraft/class_3914; */
pub fn r#get_context(&self) -> Result<crate::root::minecraft::screen::ScreenHandlerContext,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::screen::ScreenHandlerContext = Lnet/minecraft/class_3914; */
pub fn r#set_context(&self,val : crate::root::minecraft::screen::ScreenHandlerContext) -> () {} 
/* crate::root::minecraft::screen::PropertyDelegate = Lnet/minecraft/class_3913; */
pub fn r#get_propertyDelegate(&self) -> Result<crate::root::minecraft::screen::PropertyDelegate,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::screen::PropertyDelegate = Lnet/minecraft/class_3913; */
pub fn r#set_propertyDelegate(&self,val : crate::root::minecraft::screen::PropertyDelegate) -> () {} 
pub fn r#__zINDMINIT(r#syncId : i32,r#inventory : crate::root::minecraft::inventory::Inventory,) -> () { }
pub fn r#__zINDMINIT(r#syncId : i32,r#inventory : crate::root::minecraft::inventory::Inventory,r#propertyDelegate : crate::root::minecraft::screen::PropertyDelegate,r#context : crate::root::minecraft::screen::ScreenHandlerContext,) -> () { }
pub fn r#setEffects(r#primary : jni::object::JObject,r#secondary : jni::object::JObject,) -> () { }
pub fn r#getProperties() -> i32 { }
pub fn r#getPrimaryEffect() -> crate::root::minecraft::entity::effect::StatusEffect { }
pub fn r#getSecondaryEffect() -> crate::root::minecraft::entity::effect::StatusEffect { }
}
}
pub mod server {
pub mod chase {
pub struct r#ChaseClient<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for ChaseClient<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl ChaseClient<'_> {
const __map_sig : &str = "net/minecraft/class_6630";
/* jni::object::JObject = Lorg/slf4j/Logger; */
pub fn r#get_LOGGER(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lorg/slf4j/Logger; */
pub fn r#set_LOGGER(&self,val : jni::object::JObject) -> () {} 
/* i32 = I */
pub fn r#get_CONNECTION_RETRY_INTERVAL(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_CONNECTION_RETRY_INTERVAL(&self,val : i32) -> () {} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#get_ip(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#set_ip(&self,val : jni::object::JObject) -> () {} 
/* i32 = I */
pub fn r#get_port(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_port(&self,val : i32) -> () {} 
/* jni::object::JObject = Lnet/minecraft/server/MinecraftServer; */
pub fn r#get_minecraftServer(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lnet/minecraft/server/MinecraftServer; */
pub fn r#set_minecraftServer(&self,val : jni::object::JObject) -> () {} 
/* bool = Z */
pub fn r#get_running(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_running(&self,val : bool) -> () {} 
/* jni::object::JObject = Ljava/net/Socket; */
pub fn r#get_socket(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/net/Socket; */
pub fn r#set_socket(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/lang/Thread; */
pub fn r#get_thread(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/Thread; */
pub fn r#set_thread(&self,val : jni::object::JObject) -> () {} 
pub fn r#__zINDMINIT(r#ip : jni::object::JObject,r#port : i32,r#minecraftServer : jni::object::JObject,) -> () { }
pub fn r#start() -> () { }
pub fn r#parseMessage(r#message : jni::object::JObject,) -> () { }
pub fn r#executeTeleportCommand(r#scanner : jni::object::JObject,) -> () { }
pub fn r#unknwnfn_4015108091(r#pos : crate::root::minecraft::server::chase::TeleportPos,) -> () { }
pub fn r#stop() -> () { }
pub fn r#executeCommand(r#command : jni::object::JObject,) -> () { }
pub fn r#getTeleportPos(r#scanner : jni::object::JObject,) -> jni::object::JObject { }
}
}
pub mod command {
pub struct r#AdvancementCommand<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for AdvancementCommand<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl AdvancementCommand<'_> {
const __map_sig : &str = "net/minecraft/class_3008";
/* jni::object::JObject = Lcom/mojang/brigadier/suggestion/SuggestionProvider; */
pub fn r#get_SUGGESTION_PROVIDER(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lcom/mojang/brigadier/suggestion/SuggestionProvider; */
pub fn r#set_SUGGESTION_PROVIDER(&self,val : jni::object::JObject) -> () {} 
pub fn r#unknwnfn_3048142841(r#context : jni::object::JObject,) -> i32 { }
pub fn r#unknwnfn_1027142654(r#context : jni::object::JObject,) -> i32 { }
pub fn r#register(r#dispatcher : jni::object::JObject,) -> () { }
pub fn r#executeCriterion(r#source : crate::root::minecraft::server::command::ServerCommandSource,r#targets : jni::object::JObject,r#operation : crate::root::minecraft::server::command::Operation,r#advancement : crate::root::minecraft::advancement::Advancement,r#criterion : jni::object::JObject,) -> i32 { }
pub fn r#unknwnfn_151793601(r#context : jni::object::JObject,) -> i32 { }
pub fn r#unknwnfn_1523074605(r#context : jni::object::JObject,) -> i32 { }
pub fn r#unknwnfn_1514806780(r#source : crate::root::minecraft::server::command::ServerCommandSource,) -> bool { }
pub fn r#unknwnfn_845109649(r#context : jni::object::JObject,) -> i32 { }
pub fn r#unknwnfn_3490082067(r#context : jni::object::JObject,) -> i32 { }
pub fn r#unknwnfn_925003621(r#context : jni::object::JObject,r#builder : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#executeAdvancement(r#source : crate::root::minecraft::server::command::ServerCommandSource,r#targets : jni::object::JObject,r#operation : crate::root::minecraft::server::command::Operation,r#selection : jni::object::JObject,) -> i32 { }
pub fn r#unknwnfn_2976470297(r#context : jni::object::JObject,) -> i32 { }
pub fn r#addChildrenRecursivelyToList(r#parent : crate::root::minecraft::advancement::Advancement,r#childList : jni::object::JObject,) -> () { }
pub fn r#unknwnfn_3311549639(r#context : jni::object::JObject,r#builder : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#unknwnfn_511249989(r#context : jni::object::JObject,r#builder : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#unknwnfn_3544668665(r#context : jni::object::JObject,) -> i32 { }
pub fn r#unknwnfn_3134700296(r#context : jni::object::JObject,) -> i32 { }
pub fn r#unknwnfn_1893883947(r#context : jni::object::JObject,) -> i32 { }
pub fn r#select(r#advancement : crate::root::minecraft::advancement::Advancement,r#selection : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#unknwnfn_2647171389(r#context : jni::object::JObject,) -> i32 { }
}
}
pub mod dedicated {
pub struct r#AbstractPropertiesHandler<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for AbstractPropertiesHandler<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl AbstractPropertiesHandler<'_> {
const __map_sig : &str = "net/minecraft/class_3808";
/* jni::object::JObject = Ljava/util/Properties; */
pub fn r#get_properties(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/Properties; */
pub fn r#set_properties(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Lorg/slf4j/Logger; */
pub fn r#get_LOGGER(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lorg/slf4j/Logger; */
pub fn r#set_LOGGER(&self,val : jni::object::JObject) -> () {} 
pub fn r#__zINDMINIT(r#properties : jni::object::JObject,) -> () { }
pub fn r#transformedParseInt(r#key : jni::object::JObject,r#transformer : jni::object::JObject,r#fallback : i32,) -> i32 { }
pub fn r#wrapNumberParser(r#parser : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#combineParser(r#intParser : jni::object::JObject,r#fallbackParser : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#copyProperties() -> jni::object::JObject { }
pub fn r#accessor(r#key : jni::object::JObject,r#parser : jni::object::JObject,r#stringifier : jni::object::JObject,r#fallback : jni::object::JObject,) -> crate::root::minecraft::server::dedicated::PropertyAccessor { }
pub fn r#parseLong(r#key : jni::object::JObject,r#fallback : i64,) -> i64 { }
pub fn r#getInt(r#key : jni::object::JObject,r#fallback : i32,) -> i32 { }
pub fn r#loadProperties(r#path : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#saveProperties(r#path : jni::object::JObject,) -> () { }
pub fn r#unknwnfn_2172400280(r#string : jni::object::JObject,r#unknwnarg_2094857826 : jni::object::JObject,r#unknwnarg_4270129637 : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#accessor(r#key : jni::object::JObject,r#parser : jni::object::JObject,r#fallback : jni::object::JObject,) -> crate::root::minecraft::server::dedicated::PropertyAccessor { }
pub fn r#unknwnfn_1581129292(r#value : jni::object::JObject,r#unknwnarg_3925172162 : jni::object::JObject,r#unknwnarg_3334888269 : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#getString(r#key : jni::object::JObject,r#fallback : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#unknwnfn_2734982175(r#string : jni::object::JObject,r#unknwnarg_2017541624 : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#getStringValue(r#key : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#get(r#key : jni::object::JObject,r#parser : jni::object::JObject,r#parsedTransformer : jni::object::JObject,r#stringifier : jni::object::JObject,r#fallback : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#getDeprecatedBoolean(r#key : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#get(r#key : jni::object::JObject,r#parser : jni::object::JObject,r#fallback : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#getDeprecatedString(r#key : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#create(r#registryManager : crate::root::minecraft::util::registry::DynamicRegistryManager,r#properties : jni::object::JObject,) -> crate::root::minecraft::server::dedicated::AbstractPropertiesHandler { }
pub fn r#parseBoolean(r#key : jni::object::JObject,r#fallback : bool,) -> bool { }
pub fn r#get(r#key : jni::object::JObject,r#parser : jni::object::JObject,r#stringifier : jni::object::JObject,r#fallback : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#getDeprecated(r#key : jni::object::JObject,r#stringifier : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#intAccessor(r#key : jni::object::JObject,r#fallback : i32,) -> crate::root::minecraft::server::dedicated::PropertyAccessor { }
}
}
pub mod filter {
pub struct r#TextFilterer<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for TextFilterer<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl TextFilterer<'_> {
const __map_sig : &str = "net/minecraft/class_5514";
/* jni::object::JObject = Lorg/slf4j/Logger; */
pub fn r#get_LOGGER(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lorg/slf4j/Logger; */
pub fn r#set_LOGGER(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/util/concurrent/atomic/AtomicInteger; */
pub fn r#get_NEXT_WORKER_ID(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/concurrent/atomic/AtomicInteger; */
pub fn r#set_NEXT_WORKER_ID(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/util/concurrent/ThreadFactory; */
pub fn r#get_THREAD_FACTORY(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/concurrent/ThreadFactory; */
pub fn r#set_THREAD_FACTORY(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/net/URL; */
pub fn r#get_chatEndpoint(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/net/URL; */
pub fn r#set_chatEndpoint(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/net/URL; */
pub fn r#get_joinEndpoint(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/net/URL; */
pub fn r#set_joinEndpoint(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/net/URL; */
pub fn r#get_leaveEndpoint(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/net/URL; */
pub fn r#set_leaveEndpoint(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#get_apiKey(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#set_apiKey(&self,val : jni::object::JObject) -> () {} 
/* crate::root::minecraft::server::filter::HashIgnorer = Lnet/minecraft/class_5514$class_5515; */
pub fn r#get_ignorer(&self) -> Result<crate::root::minecraft::server::filter::HashIgnorer,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::server::filter::HashIgnorer = Lnet/minecraft/class_5514$class_5515; */
pub fn r#set_ignorer(&self,val : crate::root::minecraft::server::filter::HashIgnorer) -> () {} 
/* jni::object::JObject = Ljava/util/concurrent/ExecutorService; */
pub fn r#get_executor(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/concurrent/ExecutorService; */
pub fn r#set_executor(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#get_CHAT_ENDPOINT(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#set_CHAT_ENDPOINT(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Lnet/minecraft/class_5514$class_7242; */
pub fn r#get_messageEncoder(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lnet/minecraft/class_5514$class_7242; */
pub fn r#set_messageEncoder(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Lnet/minecraft/class_5514$class_7241; */
pub fn r#get_joinEncoder(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lnet/minecraft/class_5514$class_7241; */
pub fn r#set_joinEncoder(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Lnet/minecraft/class_5514$class_7241; */
pub fn r#get_leaveEncoder(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lnet/minecraft/class_5514$class_7241; */
pub fn r#set_leaveEncoder(&self,val : jni::object::JObject) -> () {} 
pub fn r#__zINDMINIT(r#chatEndpoint : jni::object::JObject,r#messageEncoder : jni::object::JObject,r#joinEndpoint : jni::object::JObject,r#joinEncoder : jni::object::JObject,r#leaveEndpoint : jni::object::JObject,r#leaveEncoder : jni::object::JObject,r#apiKey : jni::object::JObject,r#ignorer : crate::root::minecraft::server::filter::HashIgnorer,r#parallelism : i32,) -> () { }
pub fn r#sendJsonRequest(r#payload : jni::object::JObject,r#endpoint : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#createFilterer(r#gameProfile : jni::object::JObject,) -> crate::root::minecraft::server::filter::TextStream { }
pub fn r#filterMessage(r#gameProfile : jni::object::JObject,r#message : jni::object::JObject,r#ignorer : crate::root::minecraft::server::filter::HashIgnorer,r#executor : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#sendJoinOrLeaveRequest(r#gameProfile : jni::object::JObject,r#endpoint : jni::object::JObject,r#profileEncoder : jni::object::JObject,r#executor : jni::object::JObject,) -> () { }
pub fn r#consumeFully(r#inputStream : jni::object::JObject,) -> () { }
pub fn r#unknwnfn_3649495391(r#runnable : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#sendRequest(r#payload : jni::object::JObject,r#endpoint : jni::object::JObject,) -> () { }
pub fn r#createConnection(r#payload : jni::object::JObject,r#endpoint : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#load(r#config : jni::object::JObject,) -> crate::root::minecraft::server::filter::TextFilterer { }
pub fn r#getEndpoint(r#root : jni::object::JObject,r#endpoints : jni::object::JObject,r#key : jni::object::JObject,r#fallback : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#unknwnfn_1332875543(r#profile : i32,r#message : jni::object::JObject,r#unknwnarg_3737138965 : jni::object::JObject,r#unknwnarg_3216365254 : jni::object::JObject,r#unknwnarg_3274953080 : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#getValue(r#json : jni::object::JObject,r#key : jni::object::JObject,r#fallback : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#unknwnfn_2659213750(r#profile : jni::object::JObject,r#unknwnarg_3213513297 : jni::object::JObject,r#unknwnarg_2276743238 : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#unknwnfn_1325897231(r#profile : jni::object::JObject,r#message : jni::object::JObject,r#unknwnarg_1373595628 : jni::object::JObject,r#unknwnarg_1841480535 : jni::object::JObject,r#unknwnarg_1068928141 : jni::object::JObject,) -> jni::object::JObject { }
}
}
pub mod function {
pub struct r#CommandFunction<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for CommandFunction<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl CommandFunction<'_> {
const __map_sig : &str = "net/minecraft/class_2158";
/* std::vec::Vec<jni::object::JObject> = [Lnet/minecraft/class_2158$class_2161; */
pub fn r#get_elements(&self) -> Result<std::vec::Vec<jni::object::JObject>,()> {
std::vec::Vec<jni::object::JObject>::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* std::vec::Vec<jni::object::JObject> = [Lnet/minecraft/class_2158$class_2161; */
pub fn r#set_elements(&self,val : std::vec::Vec<jni::object::JObject>) -> () {} 
/* crate::root::minecraft::util::Identifier = Lnet/minecraft/class_2960; */
pub fn r#get_id(&self) -> Result<crate::root::minecraft::util::Identifier,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::util::Identifier = Lnet/minecraft/class_2960; */
pub fn r#set_id(&self,val : crate::root::minecraft::util::Identifier) -> () {} 
pub fn r#__zINDMINIT(r#id : crate::root::minecraft::util::Identifier,r#elements : jni::object::JObject,) -> () { }
pub fn r#getElements() -> jni::object::JObject { }
pub fn r#getId() -> crate::root::minecraft::util::Identifier { }
}
}
pub mod integrated {
}
pub struct r#DebugStart<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for DebugStart<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl DebugStart<'_> {
const __map_sig : &str = "net/minecraft/server/MinecraftServer$class_6414";
/* i64 = J */
pub fn r#get_time(&self) -> Result<i64,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i64 = J */
pub fn r#set_time(&self,val : i64) -> () {} 
/* i32 = I */
pub fn r#get_tick(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_tick(&self,val : i32) -> () {} 
pub fn r#isMainThread() -> bool { }
pub fn r#__zINDMINIT(r#time : i64,r#tick : i32,) -> () { }
}
}
pub mod sound {
}
pub mod stat {
}
pub mod state {
pub mod property {
pub struct r#Property<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for Property<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl Property<'_> {
const __map_sig : &str = "net/minecraft/class_2769";
/* jni::object::JObject = Ljava/lang/Class; */
pub fn r#get_type(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/Class; */
pub fn r#set_type(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#get_name(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#set_name(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/lang/Integer; */
pub fn r#get_hashCodeCache(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/Integer; */
pub fn r#set_hashCodeCache(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Lcom/mojang/serialization/Codec; */
pub fn r#get_codec(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lcom/mojang/serialization/Codec; */
pub fn r#set_codec(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Lcom/mojang/serialization/Codec; */
pub fn r#get_valueCodec(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lcom/mojang/serialization/Codec; */
pub fn r#set_valueCodec(&self,val : jni::object::JObject) -> () {} 
pub fn r#__zINDMINIT(r#name : jni::object::JObject,r#type : jni::object::JObject,) -> () { }
pub fn r#unknwnfn_2633979576(r#o : jni::object::JObject,) -> bool { }
pub fn r#computeHashCode() -> i32 { }
pub fn r#getValues() -> jni::object::JObject { }
pub fn r#getName() -> jni::object::JObject { }
pub fn r#parse(r#name : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#name(r#value : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#getType() -> jni::object::JObject { }
pub fn r#unknwnfn_23817287(r#value : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#createValue(r#state : crate::root::minecraft::state::State,) -> crate::root::minecraft::state::property::Value { }
pub fn r#createValue(r#value : jni::object::JObject,) -> crate::root::minecraft::state::property::Value { }
pub fn r#stream() -> jni::object::JObject { }
pub fn r#getValueCodec() -> jni::object::JObject { }
pub fn r#unknwnfn_1756523161(r#property : crate::root::minecraft::state::State,r#unknwnarg_160403376 : jni::object::JObject,) -> crate::root::minecraft::state::State { }
pub fn r#parse(r#ops : jni::object::JObject,r#state : crate::root::minecraft::state::State,r#input : jni::object::JObject,) -> jni::object::JObject { }
}
}
pub struct r#State<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for State<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl State<'_> {
const __map_sig : &str = "net/minecraft/class_2688";
/* jni::object::JObject = Ljava/util/function/Function; */
pub fn r#get_PROPERTY_MAP_PRINTER(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/function/Function; */
pub fn r#set_PROPERTY_MAP_PRINTER(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Lcom/google/common/collect/ImmutableMap; */
pub fn r#get_entries(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lcom/google/common/collect/ImmutableMap; */
pub fn r#set_entries(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/lang/Object; */
pub fn r#get_owner(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/Object; */
pub fn r#set_owner(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Lcom/mojang/serialization/MapCodec; */
pub fn r#get_codec(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lcom/mojang/serialization/MapCodec; */
pub fn r#set_codec(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Lcom/google/common/collect/Table; */
pub fn r#get_withTable(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lcom/google/common/collect/Table; */
pub fn r#set_withTable(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#get_NAME(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#set_NAME(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#get_PROPERTIES(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#set_PROPERTIES(&self,val : jni::object::JObject) -> () {} 
pub fn r#__zINDMINIT(r#owner : jni::object::JObject,r#entries : jni::object::JObject,r#codec : jni::object::JObject,) -> () { }
pub fn r#get(r#property : crate::root::minecraft::state::property::Property,) -> jni::object::JObject { }
pub fn r#getEntries() -> jni::object::JObject { }
pub fn r#with(r#property : crate::root::minecraft::state::property::Property,r#value : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#cycle(r#property : crate::root::minecraft::state::property::Property,) -> jni::object::JObject { }
pub fn r#createCodec(r#ownerToStateFunction : jni::object::JObject,r#unknwnarg_1905755440 : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#getNext(r#values : jni::object::JObject,r#value : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#createWithTable(r#states : jni::object::JObject,) -> () { }
pub fn r#contains(r#property : crate::root::minecraft::state::property::Property,) -> bool { }
pub fn r#toMapWith(r#property : crate::root::minecraft::state::property::Property,r#value : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#getOrEmpty(r#property : crate::root::minecraft::state::property::Property,) -> jni::object::JObject { }
}
}
pub mod structure {
pub struct r#BuriedTreasureGenerator<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for BuriedTreasureGenerator<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl BuriedTreasureGenerator<'_> {
const __map_sig : &str = "net/minecraft/class_3789";
}
}
pub mod tag {
pub struct r#TagEntry<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for TagEntry<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl TagEntry<'_> {
const __map_sig : &str = "net/minecraft/class_3497";
/* crate::root::minecraft::util::Identifier = Lnet/minecraft/class_2960; */
pub fn r#get_id(&self) -> Result<crate::root::minecraft::util::Identifier,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::util::Identifier = Lnet/minecraft/class_2960; */
pub fn r#set_id(&self,val : crate::root::minecraft::util::Identifier) -> () {} 
/* jni::object::JObject = Lcom/mojang/serialization/Codec; */
pub fn r#get_CODEC(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lcom/mojang/serialization/Codec; */
pub fn r#set_CODEC(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Lcom/mojang/serialization/Codec; */
pub fn r#get_ENTRY_CODEC(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lcom/mojang/serialization/Codec; */
pub fn r#set_ENTRY_CODEC(&self,val : jni::object::JObject) -> () {} 
/* bool = Z */
pub fn r#get_tag(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_tag(&self,val : bool) -> () {} 
/* bool = Z */
pub fn r#get_required(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_required(&self,val : bool) -> () {} 
pub fn r#__zINDMINIT(r#id : crate::root::minecraft::util::Identifier,r#tag : bool,r#required : bool,) -> () { }
pub fn r#__zINDMINIT(r#id : jni::object::JObject,r#required : bool,) -> () { }
pub fn r#resolve(r#valueGetter : crate::root::minecraft::tag::ValueGetter,r#idConsumer : jni::object::JObject,) -> bool { }
pub fn r#forEachRequiredTagId(r#idConsumer : jni::object::JObject,) -> () { }
pub fn r#canAdd(r#directEntryPredicate : jni::object::JObject,r#tagEntryPredicate : jni::object::JObject,) -> bool { }
pub fn r#getIdForCodec() -> jni::object::JObject { }
pub fn r#create(r#id : crate::root::minecraft::util::Identifier,) -> crate::root::minecraft::tag::TagEntry { }
pub fn r#unknwnfn_559054987(r#entry : crate::root::minecraft::tag::TagEntry,) -> jni::object::JObject { }
pub fn r#unknwnfn_2817121753(r#id : jni::object::JObject,) -> crate::root::minecraft::tag::TagEntry { }
pub fn r#unknwnfn_32121485(r#either : jni::object::JObject,) -> crate::root::minecraft::tag::TagEntry { }
pub fn r#unknwnfn_1330297729(r#instance : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#createOptional(r#id : crate::root::minecraft::util::Identifier,) -> crate::root::minecraft::tag::TagEntry { }
pub fn r#unknwnfn_122717915(r#entry : crate::root::minecraft::tag::TagEntry,) -> crate::root::minecraft::tag::TagEntry { }
pub fn r#forEachOptionalTagId(r#idConsumer : jni::object::JObject,) -> () { }
pub fn r#createTag(r#id : crate::root::minecraft::util::Identifier,) -> crate::root::minecraft::tag::TagEntry { }
pub fn r#unknwnfn_4200561603(r#entry : crate::root::minecraft::tag::TagEntry,) -> jni::object::JObject { }
}
}
pub mod test {
pub struct r#TestRunner<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for TestRunner<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl TestRunner<'_> {
const __map_sig : &str = "net/minecraft/class_4515";
/* jni::object::JObject = Lorg/slf4j/Logger; */
pub fn r#get_LOGGER(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lorg/slf4j/Logger; */
pub fn r#set_LOGGER(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/util/List; */
pub fn r#get_batches(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/List; */
pub fn r#set_batches(&self,val : jni::object::JObject) -> () {} 
/* crate::root::minecraft::util::math::BlockPos = Lnet/minecraft/class_2338; */
pub fn r#get_pos(&self) -> Result<crate::root::minecraft::util::math::BlockPos,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::util::math::BlockPos = Lnet/minecraft/class_2338; */
pub fn r#set_pos(&self,val : crate::root::minecraft::util::math::BlockPos) -> () {} 
/* crate::root::minecraft::server::world::ServerWorld = Lnet/minecraft/class_3218; */
pub fn r#get_world(&self) -> Result<crate::root::minecraft::server::world::ServerWorld,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::server::world::ServerWorld = Lnet/minecraft/class_3218; */
pub fn r#set_world(&self,val : crate::root::minecraft::server::world::ServerWorld) -> () {} 
/* crate::root::minecraft::test::TestManager = Lnet/minecraft/class_4521; */
pub fn r#get_testManager(&self) -> Result<crate::root::minecraft::test::TestManager,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::test::TestManager = Lnet/minecraft/class_4521; */
pub fn r#set_testManager(&self,val : crate::root::minecraft::test::TestManager) -> () {} 
/* jni::object::JObject = Ljava/util/List; */
pub fn r#get_tests(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/List; */
pub fn r#set_tests(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Lnet/minecraft/class_2338$class_2339; */
pub fn r#get_reusablePos(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lnet/minecraft/class_2338$class_2339; */
pub fn r#set_reusablePos(&self,val : jni::object::JObject) -> () {} 
/* i32 = I */
pub fn r#get_sizeZ(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_sizeZ(&self,val : i32) -> () {} 
pub fn r#__zINDMINIT(r#batches : jni::object::JObject,r#pos : crate::root::minecraft::util::math::BlockPos,r#rotation : crate::root::minecraft::util::BlockRotation,r#world : crate::root::minecraft::server::world::ServerWorld,r#testManager : crate::root::minecraft::test::TestManager,r#sizeZ : i32,) -> () { }
pub fn r#getTests() -> jni::object::JObject { }
pub fn r#runBatch(r#index : i32,) -> () { }
pub fn r#run() -> () { }
pub fn r#unknwnfn_3202219594(r#gameTest : jni::object::JObject,r#unknwnarg_1546061773 : crate::root::minecraft::test::GameTestState,) -> () { }
pub fn r#alignTestStructures(r#gameTests : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#unknwnfn_1521080420(r#batch : crate::root::minecraft::util::BlockRotation,r#unknwnarg_1820116600 : crate::root::minecraft::server::world::ServerWorld,r#unknwnarg_2131858417 : crate::root::minecraft::test::GameTestBatch,) -> jni::object::JObject { }
pub fn r#unknwnfn_3308121257(r#testFunction : crate::root::minecraft::util::BlockRotation,r#unknwnarg_2355436961 : crate::root::minecraft::server::world::ServerWorld,r#unknwnarg_3043732624 : crate::root::minecraft::test::TestFunction,) -> crate::root::minecraft::test::GameTestState { }
}
}
pub mod text {
pub struct r#ClickEvent<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for ClickEvent<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl ClickEvent<'_> {
const __map_sig : &str = "net/minecraft/class_2558";
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#get_value(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#set_value(&self,val : jni::object::JObject) -> () {} 
/* crate::root::minecraft::text::Action = Lnet/minecraft/class_2558$class_2559; */
pub fn r#get_action(&self) -> Result<crate::root::minecraft::text::Action,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::text::Action = Lnet/minecraft/class_2558$class_2559; */
pub fn r#set_action(&self,val : crate::root::minecraft::text::Action) -> () {} 
pub fn r#__zINDMINIT(r#action : crate::root::minecraft::text::Action,r#value : jni::object::JObject,) -> () { }
pub fn r#unknwnfn_2994933936(r#o : jni::object::JObject,) -> bool { }
pub fn r#getValue() -> jni::object::JObject { }
}
}
pub mod unused {
pub mod packageinfo {
}
}
pub mod util {
pub mod annotation {
}
pub mod collection {
pub struct r#DataPool<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for DataPool<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl DataPool<'_> {
const __map_sig : &str = "net/minecraft/class_6005";
pub fn r#builder() -> crate::root::minecraft::util::collection::Builder { }
pub fn r#createCodec(r#dataCodec : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#getDataOrEmpty(r#random : crate::root::minecraft::util::math::random::Random,) -> jni::object::JObject { }
pub fn r#of(r#object : jni::object::JObject,) -> crate::root::minecraft::util::collection::DataPool { }
pub fn r#empty() -> crate::root::minecraft::util::collection::DataPool { }
}
}
pub mod crash {
pub struct r#CrashReportSection<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for CrashReportSection<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl CrashReportSection<'_> {
const __map_sig : &str = "net/minecraft/class_129";
/* jni::object::JObject = Ljava/util/List; */
pub fn r#get_elements(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/List; */
pub fn r#set_elements(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#get_title(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#set_title(&self,val : jni::object::JObject) -> () {} 
/* std::vec::Vec<jni::object::JObject> = [Ljava/lang/StackTraceElement; */
pub fn r#get_stackTrace(&self) -> Result<std::vec::Vec<jni::object::JObject>,()> {
std::vec::Vec<jni::object::JObject>::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* std::vec::Vec<jni::object::JObject> = [Ljava/lang/StackTraceElement; */
pub fn r#set_stackTrace(&self,val : std::vec::Vec<jni::object::JObject>) -> () {} 
pub fn r#__zINDMINIT(r#title : jni::object::JObject,) -> () { }
pub fn r#addStackTrace(r#crashReportBuilder : jni::object::JObject,) -> () { }
pub fn r#getStackTrace() -> jni::object::JObject { }
pub fn r#add(r#name : jni::object::JObject,r#callable : crate::root::minecraft::util::crash::CrashCallable,) -> crate::root::minecraft::util::crash::CrashReportSection { }
pub fn r#add(r#name : jni::object::JObject,r#detail : jni::object::JObject,) -> crate::root::minecraft::util::crash::CrashReportSection { }
pub fn r#initStackTrace(r#ignoredCallCount : i32,) -> i32 { }
pub fn r#trimStackTraceEnd(r#callCount : i32,) -> () { }
pub fn r#createPositionString(r#world : crate::root::minecraft::world::HeightLimitView,r#x : i32,r#y : i32,r#z : i32,) -> jni::object::JObject { }
pub fn r#createPositionString(r#world : crate::root::minecraft::world::HeightLimitView,r#pos : crate::root::minecraft::util::math::BlockPos,) -> jni::object::JObject { }
pub fn r#createPositionString(r#world : crate::root::minecraft::world::HeightLimitView,r#x : f64,r#y : f64,r#z : f64,) -> jni::object::JObject { }
pub fn r#shouldGenerateStackTrace(r#prev : jni::object::JObject,r#next : jni::object::JObject,) -> bool { }
pub fn r#add(r#name : jni::object::JObject,r#throwable : jni::object::JObject,) -> () { }
}
}
pub struct r#CsvWriter<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for CsvWriter<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl CsvWriter<'_> {
const __map_sig : &str = "net/minecraft/class_4456";
/* jni::object::JObject = Ljava/io/Writer; */
pub fn r#get_writer(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/io/Writer; */
pub fn r#set_writer(&self,val : jni::object::JObject) -> () {} 
/* i32 = I */
pub fn r#get_column(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_column(&self,val : i32) -> () {} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#get_CRLF(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#set_CRLF(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#get_COMMA(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#set_COMMA(&self,val : jni::object::JObject) -> () {} 
pub fn r#__zINDMINIT(r#writer : jni::object::JObject,r#columns : jni::object::JObject,) -> () { }
pub fn r#makeHeader() -> crate::root::minecraft::util::Header { }
pub fn r#print(r#o : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#printRow(r#columns : jni::object::JObject,) -> () { }
}
}
pub mod village {
pub mod raid {
pub struct r#Raid<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for Raid<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl Raid<'_> {
const __map_sig : &str = "net/minecraft/class_3765";
/* i64 = J */
pub fn r#get_ticksActive(&self) -> Result<i64,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i64 = J */
pub fn r#set_ticksActive(&self,val : i64) -> () {} 
/* bool = Z */
pub fn r#get_active(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_active(&self,val : bool) -> () {} 
/* crate::root::minecraft::entity::boss::ServerBossBar = Lnet/minecraft/class_3213; */
pub fn r#get_bar(&self) -> Result<crate::root::minecraft::entity::boss::ServerBossBar,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::entity::boss::ServerBossBar = Lnet/minecraft/class_3213; */
pub fn r#set_bar(&self,val : crate::root::minecraft::entity::boss::ServerBossBar) -> () {} 
/* crate::root::minecraft::util::math::random::Random = Lnet/minecraft/class_5819; */
pub fn r#get_random(&self) -> Result<crate::root::minecraft::util::math::random::Random,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::util::math::random::Random = Lnet/minecraft/class_5819; */
pub fn r#set_random(&self,val : crate::root::minecraft::util::math::random::Random) -> () {} 
/* bool = Z */
pub fn r#get_started(&self) -> Result<bool,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* bool = Z */
pub fn r#set_started(&self,val : bool) -> () {} 
/* crate::root::minecraft::util::math::BlockPos = Lnet/minecraft/class_2338; */
pub fn r#get_center(&self) -> Result<crate::root::minecraft::util::math::BlockPos,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::util::math::BlockPos = Lnet/minecraft/class_2338; */
pub fn r#set_center(&self,val : crate::root::minecraft::util::math::BlockPos) -> () {} 
/* i32 = I */
pub fn r#get_preRaidTicks(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_preRaidTicks(&self,val : i32) -> () {} 
/* jni::object::JObject = Ljava/util/Map; */
pub fn r#get_waveToCaptain(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/Map; */
pub fn r#set_waveToCaptain(&self,val : jni::object::JObject) -> () {} 
/* i32 = I */
pub fn r#get_postRaidTicks(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_postRaidTicks(&self,val : i32) -> () {} 
/* jni::object::JObject = Ljava/util/Map; */
pub fn r#get_waveToRaiders(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/Map; */
pub fn r#set_waveToRaiders(&self,val : jni::object::JObject) -> () {} 
/* crate::root::minecraft::server::world::ServerWorld = Lnet/minecraft/class_3218; */
pub fn r#get_world(&self) -> Result<crate::root::minecraft::server::world::ServerWorld,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::server::world::ServerWorld = Lnet/minecraft/class_3218; */
pub fn r#set_world(&self,val : crate::root::minecraft::server::world::ServerWorld) -> () {} 
/* f32 = F */
pub fn r#get_totalHealth(&self) -> Result<f32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f32 = F */
pub fn r#set_totalHealth(&self,val : f32) -> () {} 
/* i32 = I */
pub fn r#get_wavesSpawned(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_wavesSpawned(&self,val : i32) -> () {} 
/* i32 = I */
pub fn r#get_badOmenLevel(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_badOmenLevel(&self,val : i32) -> () {} 
/* i32 = I */
pub fn r#get_id(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_id(&self,val : i32) -> () {} 
/* crate::root::minecraft::text::Text = Lnet/minecraft/class_2561; */
pub fn r#get_EVENT_TEXT(&self) -> Result<crate::root::minecraft::text::Text,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::text::Text = Lnet/minecraft/class_2561; */
pub fn r#set_EVENT_TEXT(&self,val : crate::root::minecraft::text::Text) -> () {} 
/* crate::root::minecraft::text::Text = Lnet/minecraft/class_2561; */
pub fn r#get_VICTORY_SUFFIX_TEXT(&self) -> Result<crate::root::minecraft::text::Text,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::text::Text = Lnet/minecraft/class_2561; */
pub fn r#set_VICTORY_SUFFIX_TEXT(&self,val : crate::root::minecraft::text::Text) -> () {} 
/* crate::root::minecraft::text::Text = Lnet/minecraft/class_2561; */
pub fn r#get_DEFEAT_SUFFIX_TEXT(&self) -> Result<crate::root::minecraft::text::Text,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::text::Text = Lnet/minecraft/class_2561; */
pub fn r#set_DEFEAT_SUFFIX_TEXT(&self,val : crate::root::minecraft::text::Text) -> () {} 
/* crate::root::minecraft::text::Text = Lnet/minecraft/class_2561; */
pub fn r#get_VICTORY_TITLE(&self) -> Result<crate::root::minecraft::text::Text,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::text::Text = Lnet/minecraft/class_2561; */
pub fn r#set_VICTORY_TITLE(&self,val : crate::root::minecraft::text::Text) -> () {} 
/* crate::root::minecraft::text::Text = Lnet/minecraft/class_2561; */
pub fn r#get_DEFEAT_TITLE(&self) -> Result<crate::root::minecraft::text::Text,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::text::Text = Lnet/minecraft/class_2561; */
pub fn r#set_DEFEAT_TITLE(&self,val : crate::root::minecraft::text::Text) -> () {} 
/* jni::object::JObject = Ljava/util/Set; */
pub fn r#get_heroesOfTheVillage(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/Set; */
pub fn r#set_heroesOfTheVillage(&self,val : jni::object::JObject) -> () {} 
/* i32 = I */
pub fn r#get_waveCount(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_waveCount(&self,val : i32) -> () {} 
/* jni::object::JObject = Lnet/minecraft/class_3765$class_4259; */
pub fn r#get_status(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lnet/minecraft/class_3765$class_4259; */
pub fn r#set_status(&self,val : jni::object::JObject) -> () {} 
/* i32 = I */
pub fn r#get_finishCooldown(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_finishCooldown(&self,val : i32) -> () {} 
/* jni::object::JObject = Ljava/util/Optional; */
pub fn r#get_preCalculatedRavagerSpawnLocation(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/Optional; */
pub fn r#set_preCalculatedRavagerSpawnLocation(&self,val : jni::object::JObject) -> () {} 
/* i32 = I */
pub fn r#get_MAX_DESPAWN_COUNTER(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_MAX_DESPAWN_COUNTER(&self,val : i32) -> () {} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#get_OMINOUS_BANNER_TRANSLATION_KEY(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#set_OMINOUS_BANNER_TRANSLATION_KEY(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#get_RAIDERS_REMAINING_TRANSLATION_KEY(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/String; */
pub fn r#set_RAIDERS_REMAINING_TRANSLATION_KEY(&self,val : jni::object::JObject) -> () {} 
/* i32 = I */
pub fn r#get_DEFAULT_PRE_RAID_TICKS(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_DEFAULT_PRE_RAID_TICKS(&self,val : i32) -> () {} 
/* i32 = I */
pub fn r#get_MAX_ACTIVE_TICKS(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_MAX_ACTIVE_TICKS(&self,val : i32) -> () {} 
pub fn r#__zINDMINIT(r#id : i32,r#world : crate::root::minecraft::server::world::ServerWorld,r#pos : crate::root::minecraft::util::math::BlockPos,) -> () { }
pub fn r#__zINDMINIT(r#world : crate::root::minecraft::server::world::ServerWorld,r#nbt : crate::root::minecraft::nbt::NbtCompound,) -> () { }
pub fn r#addToWave(r#wave : i32,r#entity : crate::root::minecraft::entity::raid::RaiderEntity,r#countHealth : bool,) -> bool { }
pub fn r#getGroupsSpawned() -> i32 { }
pub fn r#setWaveCaptain(r#wave : i32,r#entity : crate::root::minecraft::entity::raid::RaiderEntity,) -> () { }
pub fn r#getBadOmenLevel() -> i32 { }
pub fn r#getRaidId() -> i32 { }
pub fn r#getCenter() -> crate::root::minecraft::util::math::BlockPos { }
pub fn r#getCaptain(r#wave : i32,) -> crate::root::minecraft::entity::raid::RaiderEntity { }
pub fn r#updateBarToPlayers() -> () { }
pub fn r#removeLeader(r#wave : i32,) -> () { }
pub fn r#isInRaidDistance() -> jni::object::JObject { }
pub fn r#writeNbt(r#nbt : crate::root::minecraft::nbt::NbtCompound,) -> crate::root::minecraft::nbt::NbtCompound { }
pub fn r#isActive() -> bool { }
pub fn r#addToWave(r#wave : i32,r#entity : crate::root::minecraft::entity::raid::RaiderEntity,) -> bool { }
pub fn r#invalidate() -> () { }
pub fn r#unknwnfn_4178718202(r#wave : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#tick() -> () { }
pub fn r#removeFromWave(r#entity : crate::root::minecraft::entity::raid::RaiderEntity,r#countHealth : bool,) -> () { }
pub fn r#getCurrentRaiderHealth() -> f32 { }
pub fn r#getMaxAcceptableBadOmenLevel() -> i32 { }
pub fn r#getOminousBanner() -> crate::root::minecraft::item::ItemStack { }
pub fn r#addRaider(r#wave : i32,r#raider : crate::root::minecraft::entity::raid::RaiderEntity,r#pos : crate::root::minecraft::util::math::BlockPos,r#existing : bool,) -> () { }
pub fn r#getRaiderCount() -> i32 { }
pub fn r#start(r#player : crate::root::minecraft::entity::player::PlayerEntity,) -> () { }
pub fn r#canSpawnRaiders() -> bool { }
pub fn r#markDirty() -> () { }
pub fn r#playRaidHorn(r#pos : crate::root::minecraft::util::math::BlockPos,) -> () { }
pub fn r#spawnNextWave(r#pos : crate::root::minecraft::util::math::BlockPos,) -> () { }
pub fn r#updateBar() -> () { }
pub fn r#hasStarted() -> bool { }
pub fn r#getRavagerSpawnLocation(r#proximity : i32,r#tries : i32,) -> crate::root::minecraft::util::math::BlockPos { }
pub fn r#getWorld() -> crate::root::minecraft::world::World { }
pub fn r#isFinished() -> bool { }
pub fn r#shouldSpawnMoreGroups() -> bool { }
pub fn r#removeObsoleteRaiders() -> () { }
pub fn r#unknwnfn_272910966(r#player : crate::root::minecraft::server::network::ServerPlayerEntity,) -> bool { }
pub fn r#hasSpawnedFinalWave() -> bool { }
pub fn r#hasExtraWave() -> bool { }
pub fn r#hasSpawnedExtraWave() -> bool { }
pub fn r#isSpawningExtraWave() -> bool { }
pub fn r#getMaxWaves(r#difficulty : crate::root::minecraft::world::Difficulty,) -> i32 { }
pub fn r#addHero(r#entity : crate::root::minecraft::entity::Entity,) -> () { }
pub fn r#getCount(r#member : crate::root::minecraft::village::raid::Member,r#wave : i32,r#extra : bool,) -> i32 { }
pub fn r#getBonusCount(r#member : crate::root::minecraft::village::raid::Member,r#random : crate::root::minecraft::util::math::random::Random,r#wave : i32,r#localDifficulty : crate::root::minecraft::world::LocalDifficulty,r#extra : bool,) -> i32 { }
pub fn r#isPreRaid() -> bool { }
pub fn r#hasSpawned() -> bool { }
pub fn r#hasStopped() -> bool { }
pub fn r#hasWon() -> bool { }
pub fn r#hasLost() -> bool { }
pub fn r#getEnchantmentChance() -> f32 { }
pub fn r#preCalculateRavagerSpawnLocation(r#proximity : i32,) -> jni::object::JObject { }
pub fn r#setCenter(r#center : crate::root::minecraft::util::math::BlockPos,) -> () { }
pub fn r#unknwnfn_3902714533(r#pos : crate::root::minecraft::util::math::BlockPos,) -> f64 { }
pub fn r#moveRaidCenter() -> () { }
pub fn r#setBadOmenLevel(r#badOmenLevel : i32,) -> () { }
pub fn r#getTotalHealth() -> f32 { }
}
}
pub struct r#TradeOffers<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for TradeOffers<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl TradeOffers<'_> {
const __map_sig : &str = "net/minecraft/class_3853";
/* jni::object::JObject = Ljava/util/Map; */
pub fn r#get_PROFESSION_TO_LEVELED_TRADE(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/Map; */
pub fn r#set_PROFESSION_TO_LEVELED_TRADE(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Lit/unimi/dsi/fastutil/ints/Int2ObjectMap; */
pub fn r#get_WANDERING_TRADER_TRADES(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lit/unimi/dsi/fastutil/ints/Int2ObjectMap; */
pub fn r#set_WANDERING_TRADER_TRADES(&self,val : jni::object::JObject) -> () {} 
/* i32 = I */
pub fn r#get_DEFAULT_MAX_USES(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_DEFAULT_MAX_USES(&self,val : i32) -> () {} 
/* i32 = I */
pub fn r#get_COMMON_MAX_USES(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_COMMON_MAX_USES(&self,val : i32) -> () {} 
/* i32 = I */
pub fn r#get_RARE_MAX_USES(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_RARE_MAX_USES(&self,val : i32) -> () {} 
/* i32 = I */
pub fn r#get_NOVICE_SELL_XP(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_NOVICE_SELL_XP(&self,val : i32) -> () {} 
/* i32 = I */
pub fn r#get_NOVICE_BUY_XP(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_NOVICE_BUY_XP(&self,val : i32) -> () {} 
/* i32 = I */
pub fn r#get_APPRENTICE_SELL_XP(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_APPRENTICE_SELL_XP(&self,val : i32) -> () {} 
/* i32 = I */
pub fn r#get_APPRENTICE_BUY_XP(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_APPRENTICE_BUY_XP(&self,val : i32) -> () {} 
/* i32 = I */
pub fn r#get_JOURNEYMAN_SELL_XP(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_JOURNEYMAN_SELL_XP(&self,val : i32) -> () {} 
/* i32 = I */
pub fn r#get_JOURNEYMAN_BUY_XP(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_JOURNEYMAN_BUY_XP(&self,val : i32) -> () {} 
/* i32 = I */
pub fn r#get_EXPERT_SELL_XP(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_EXPERT_SELL_XP(&self,val : i32) -> () {} 
/* i32 = I */
pub fn r#get_EXPERT_BUY_XP(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_EXPERT_BUY_XP(&self,val : i32) -> () {} 
/* i32 = I */
pub fn r#get_MASTER_TRADE_XP(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_MASTER_TRADE_XP(&self,val : i32) -> () {} 
/* f32 = F */
pub fn r#get_LOW_PRICE_MULTIPLIER(&self) -> Result<f32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f32 = F */
pub fn r#set_LOW_PRICE_MULTIPLIER(&self,val : f32) -> () {} 
/* f32 = F */
pub fn r#get_HIGH_PRICE_MULTIPLIER(&self) -> Result<f32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* f32 = F */
pub fn r#set_HIGH_PRICE_MULTIPLIER(&self,val : f32) -> () {} 
pub fn r#copyToFastUtilMap(r#map : jni::object::JObject,) -> jni::object::JObject { }
}
}
pub mod world {
pub mod biome {
pub struct r#Biome<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for Biome<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl Biome<'_> {
const __map_sig : &str = "net/minecraft/class_1959";
/* jni::object::JObject = Ljava/lang/ThreadLocal; */
pub fn r#get_temperatureCache(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/lang/ThreadLocal; */
pub fn r#set_temperatureCache(&self,val : jni::object::JObject) -> () {} 
/* crate::root::minecraft::world::biome::BiomeEffects = Lnet/minecraft/class_4763; */
pub fn r#get_effects(&self) -> Result<crate::root::minecraft::world::biome::BiomeEffects,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::world::biome::BiomeEffects = Lnet/minecraft/class_4763; */
pub fn r#set_effects(&self,val : crate::root::minecraft::world::biome::BiomeEffects) -> () {} 
/* jni::object::JObject = Lcom/mojang/serialization/Codec; */
pub fn r#get_REGISTRY_CODEC(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lcom/mojang/serialization/Codec; */
pub fn r#set_REGISTRY_CODEC(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Lcom/mojang/serialization/Codec; */
pub fn r#get_CODEC(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lcom/mojang/serialization/Codec; */
pub fn r#set_CODEC(&self,val : jni::object::JObject) -> () {} 
/* crate::root::minecraft::util::math::noise::OctaveSimplexNoiseSampler = Lnet/minecraft/class_3543; */
pub fn r#get_FROZEN_OCEAN_NOISE(&self) -> Result<crate::root::minecraft::util::math::noise::OctaveSimplexNoiseSampler,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::util::math::noise::OctaveSimplexNoiseSampler = Lnet/minecraft/class_3543; */
pub fn r#set_FROZEN_OCEAN_NOISE(&self,val : crate::root::minecraft::util::math::noise::OctaveSimplexNoiseSampler) -> () {} 
/* jni::object::JObject = Lnet/minecraft/class_1959$class_5482; */
pub fn r#get_weather(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lnet/minecraft/class_1959$class_5482; */
pub fn r#set_weather(&self,val : jni::object::JObject) -> () {} 
/* crate::root::minecraft::world::biome::SpawnSettings = Lnet/minecraft/class_5483; */
pub fn r#get_spawnSettings(&self) -> Result<crate::root::minecraft::world::biome::SpawnSettings,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::world::biome::SpawnSettings = Lnet/minecraft/class_5483; */
pub fn r#set_spawnSettings(&self,val : crate::root::minecraft::world::biome::SpawnSettings) -> () {} 
/* jni::object::JObject = Lcom/mojang/serialization/Codec; */
pub fn r#get_NETWORK_CODEC(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lcom/mojang/serialization/Codec; */
pub fn r#set_NETWORK_CODEC(&self,val : jni::object::JObject) -> () {} 
/* crate::root::minecraft::world::biome::GenerationSettings = Lnet/minecraft/class_5485; */
pub fn r#get_generationSettings(&self) -> Result<crate::root::minecraft::world::biome::GenerationSettings,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::world::biome::GenerationSettings = Lnet/minecraft/class_5485; */
pub fn r#set_generationSettings(&self,val : crate::root::minecraft::world::biome::GenerationSettings) -> () {} 
/* jni::object::JObject = Lcom/mojang/serialization/Codec; */
pub fn r#get_REGISTRY_ENTRY_LIST_CODEC(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lcom/mojang/serialization/Codec; */
pub fn r#set_REGISTRY_ENTRY_LIST_CODEC(&self,val : jni::object::JObject) -> () {} 
/* i32 = I */
pub fn r#get_MAX_TEMPERATURE_CACHE_SIZE(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_MAX_TEMPERATURE_CACHE_SIZE(&self,val : i32) -> () {} 
/* crate::root::minecraft::util::math::noise::OctaveSimplexNoiseSampler = Lnet/minecraft/class_3543; */
pub fn r#get_FOLIAGE_NOISE(&self) -> Result<crate::root::minecraft::util::math::noise::OctaveSimplexNoiseSampler,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::util::math::noise::OctaveSimplexNoiseSampler = Lnet/minecraft/class_3543; */
pub fn r#set_FOLIAGE_NOISE(&self,val : crate::root::minecraft::util::math::noise::OctaveSimplexNoiseSampler) -> () {} 
/* crate::root::minecraft::util::math::noise::OctaveSimplexNoiseSampler = Lnet/minecraft/class_3543; */
pub fn r#get_TEMPERATURE_NOISE(&self) -> Result<crate::root::minecraft::util::math::noise::OctaveSimplexNoiseSampler,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::util::math::noise::OctaveSimplexNoiseSampler = Lnet/minecraft/class_3543; */
pub fn r#set_TEMPERATURE_NOISE(&self,val : crate::root::minecraft::util::math::noise::OctaveSimplexNoiseSampler) -> () {} 
pub fn r#__zINDMINIT(r#weather : jni::object::JObject,r#effects : crate::root::minecraft::world::biome::BiomeEffects,r#generationSettings : crate::root::minecraft::world::biome::GenerationSettings,r#spawnSettings : crate::root::minecraft::world::biome::SpawnSettings,) -> () { }
pub fn r#getTemperature(r#blockPos : crate::root::minecraft::util::math::BlockPos,) -> f32 { }
pub fn r#getFogColor() -> i32 { }
pub fn r#getEffects() -> crate::root::minecraft::world::biome::BiomeEffects { }
pub fn r#getParticleConfig() -> jni::object::JObject { }
pub fn r#getLoopSound() -> jni::object::JObject { }
pub fn r#getMoodSound() -> jni::object::JObject { }
pub fn r#getAdditionsSound() -> jni::object::JObject { }
pub fn r#getMusic() -> jni::object::JObject { }
pub fn r#unknwnfn_4024563639(r#biome : crate::root::minecraft::world::biome::Biome,) -> jni::object::JObject { }
pub fn r#unknwnfn_155009344(r#biome : crate::root::minecraft::world::biome::Biome,) -> crate::root::minecraft::world::biome::SpawnSettings { }
pub fn r#unknwnfn_901442481(r#biome : crate::root::minecraft::world::biome::Biome,) -> crate::root::minecraft::world::biome::GenerationSettings { }
pub fn r#unknwnfn_4078198813(r#biome : crate::root::minecraft::world::biome::Biome,) -> crate::root::minecraft::world::biome::BiomeEffects { }
pub fn r#unknwnfn_2070047997(r#biome : crate::root::minecraft::world::biome::Biome,) -> jni::object::JObject { }
pub fn r#getDefaultGrassColor() -> i32 { }
pub fn r#getDefaultFoliageColor() -> i32 { }
pub fn r#unknwnfn_2258580757(r#instance : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#getSpawnSettings() -> crate::root::minecraft::world::biome::SpawnSettings { }
pub fn r#getGenerationSettings() -> crate::root::minecraft::world::biome::GenerationSettings { }
pub fn r#unknwnfn_3338344484(r#biome : crate::root::minecraft::world::biome::Biome,) -> crate::root::minecraft::world::biome::BiomeEffects { }
pub fn r#isCold(r#pos : crate::root::minecraft::util::math::BlockPos,) -> bool { }
pub fn r#unknwnfn_519289768(r#weather : jni::object::JObject,r#effects : crate::root::minecraft::world::biome::BiomeEffects,) -> crate::root::minecraft::world::biome::Biome { }
pub fn r#unknwnfn_3346324422(r#instance : jni::object::JObject,) -> jni::object::JObject { }
pub fn r#doesNotSnow(r#pos : crate::root::minecraft::util::math::BlockPos,) -> bool { }
pub fn r#shouldGenerateLowerFrozenOceanSurface(r#pos : crate::root::minecraft::util::math::BlockPos,) -> bool { }
pub fn r#isHot(r#pos : crate::root::minecraft::util::math::BlockPos,) -> bool { }
pub fn r#canSetIce(r#world : crate::root::minecraft::world::WorldView,r#pos : crate::root::minecraft::util::math::BlockPos,r#doWaterCheck : bool,) -> bool { }
pub fn r#getWaterColor() -> i32 { }
pub fn r#getPrecipitation() -> jni::object::JObject { }
pub fn r#canSetSnow(r#world : crate::root::minecraft::world::WorldView,r#pos : crate::root::minecraft::util::math::BlockPos,) -> bool { }
pub fn r#getSkyColor() -> i32 { }
pub fn r#getFoliageColor() -> i32 { }
pub fn r#canSetIce(r#world : crate::root::minecraft::world::WorldView,r#blockPos : crate::root::minecraft::util::math::BlockPos,) -> bool { }
pub fn r#computeTemperature(r#pos : crate::root::minecraft::util::math::BlockPos,) -> f32 { }
pub fn r#getGrassColorAt(r#x : f64,r#z : f64,) -> i32 { }
pub fn r#getTemperature() -> f32 { }
pub fn r#getWaterFogColor() -> i32 { }
pub fn r#getDownfall() -> f32 { }
}
}
pub mod block {
pub struct r#ChainRestrictedNeighborUpdater<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for ChainRestrictedNeighborUpdater<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl ChainRestrictedNeighborUpdater<'_> {
const __map_sig : &str = "net/minecraft/class_7159";
/* jni::object::JObject = Lorg/slf4j/Logger; */
pub fn r#get_LOGGER(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Lorg/slf4j/Logger; */
pub fn r#set_LOGGER(&self,val : jni::object::JObject) -> () {} 
/* crate::root::minecraft::world::World = Lnet/minecraft/class_1937; */
pub fn r#get_world(&self) -> Result<crate::root::minecraft::world::World,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* crate::root::minecraft::world::World = Lnet/minecraft/class_1937; */
pub fn r#set_world(&self,val : crate::root::minecraft::world::World) -> () {} 
/* jni::object::JObject = Ljava/util/ArrayDeque; */
pub fn r#get_queue(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/ArrayDeque; */
pub fn r#set_queue(&self,val : jni::object::JObject) -> () {} 
/* jni::object::JObject = Ljava/util/List; */
pub fn r#get_pending(&self) -> Result<jni::object::JObject,()> {
jni::object::JObject::from(self.__map_internal.env.find_class(Self::__map_sig)?)} 
/* jni::object::JObject = Ljava/util/List; */
pub fn r#set_pending(&self,val : jni::object::JObject) -> () {} 
/* i32 = I */
pub fn r#get_depth(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_depth(&self,val : i32) -> () {} 
/* i32 = I */
pub fn r#get_maxChainDepth(&self) -> Result<i32,()> {
self.__map_internal.env.find_class(Self::__map_sig)?} 
/* i32 = I */
pub fn r#set_maxChainDepth(&self,val : i32) -> () {} 
pub fn r#__zINDMINIT(r#world : crate::root::minecraft::world::World,r#maxChainDepth : i32,) -> () { }
pub fn r#runQueuedUpdates() -> () { }
}
}
pub struct r#BlockLocating<'a> {
pub __map_internal : jni::object::JObject<'a>
}
impl From<jni::object::JObject<'_>> for BlockLocating<'_> {fn from(x : jni :: object :: JObject) -> Self { Self { __map_internal : x } }}
impl BlockLocating<'_> {
const __map_sig : &str = "net/minecraft/class_5459";
pub fn r#getLargestRectangle(r#center : crate::root::minecraft::util::math::BlockPos,r#primaryAxis : crate::root::minecraft::util::math::Axis,r#primaryMaxBlocks : i32,r#secondaryAxis : crate::root::minecraft::util::math::Axis,r#secondaryMaxBlocks : i32,r#predicate : jni::object::JObject,) -> crate::root::minecraft::world::Rectangle { }
pub fn r#moveWhile(r#predicate : jni::object::JObject,r#pos : jni::object::JObject,r#direction : crate::root::minecraft::util::math::Direction,r#max : i32,) -> i32 { }
pub fn r#findLargestRectangle(r#heights : i32,) -> jni::object::JObject { }
}
}
}
}
