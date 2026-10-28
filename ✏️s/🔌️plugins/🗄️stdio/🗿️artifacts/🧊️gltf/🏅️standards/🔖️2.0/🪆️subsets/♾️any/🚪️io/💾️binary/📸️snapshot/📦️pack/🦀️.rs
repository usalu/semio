//! 📦️ Logical GLTF records preserve source form, independent buffers and all native fields.
use crate::standards::v2_0::subsets::any::schema::snapshot::*;
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_framework_dsl_record::DslField;
use semio_framework_dsl_record::{BorrowedDslField,BorrowedShape};

#[path="🛫️encoding/🦀️.rs"]pub mod encoding;
#[path="geometry.rs"]pub mod geometry;
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub struct Attribute{semantic:String,accessor:usize}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub struct Target{attributes:Vec<Attribute>}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub struct Primitive{attributes:Vec<Attribute>,indices:Option<usize>,material:Option<usize>,mode:Option<u64>,targets:Vec<GltfMorphTarget>,extensions:Option<GltfJson>,extras:Option<GltfJson>}
#[derive(semio_framework_dsl_record_derive::DslScalar)]
pub enum ProjectionKind{Perspective,Orthographic}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub struct Projection{kind:ProjectionKind,perspective:Option<GltfPerspective>,orthographic:Option<GltfOrthographic>}
fn attributes(value:&[(String,usize)])->Vec<Attribute>{value.iter().map(|(semantic,accessor)|Attribute{semantic:semantic.clone(),accessor:*accessor}).collect()}
fn pairs(value:Vec<Attribute>)->Vec<(String,usize)>{value.into_iter().map(|value|(value.semantic,value.accessor)).collect()}
#[path="🧩️extras/🦀️.rs"]
pub mod json;
impl BorrowedDslField for GltfJson{const SHAPE:BorrowedShape=<json::Json as BorrowedDslField>::SHAPE;}
impl BorrowedDslField for GltfMorphTarget{const SHAPE:BorrowedShape=<Target as BorrowedDslField>::SHAPE;}
impl BorrowedDslField for GltfPrimitive{const SHAPE:BorrowedShape=<Primitive as BorrowedDslField>::SHAPE;}
impl BorrowedDslField for GltfCameraProjection{const SHAPE:BorrowedShape=<Projection as BorrowedDslField>::SHAPE;}
impl BorrowedDslField for GltfImage{const SHAPE:BorrowedShape=<Image as BorrowedDslField>::SHAPE;}
impl BorrowedDslField for GltfTexture{const SHAPE:BorrowedShape=<Texture as BorrowedDslField>::SHAPE;}
fn optional<T:DslField>(value:&Option<T>)->semio_framework_dsl_record::FieldValue{match value{Some(value)=>value.to_value(),None=>semio_framework_dsl_record::FieldValue::Absent}}
fn retire_json(value:Option<GltfJson>){if let Some(value)=value{json::retire(value);}}
impl DslField for GltfJson{
 fn shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::Shape,ValueError>{encoding::json_shape(control)}
 fn to_value_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,ValueError>{encoding::json(self,control)}
 fn retire_decoded(self){json::retire(self)}
 fn shape()->semio_framework_dsl_record::Shape{json::Json::shape()}
 fn to_value(&self)->semio_framework_dsl_record::FieldValue{json::Json::from(self).to_value()}
 fn from_value(value:&semio_framework_dsl_record::FieldValue)->Result<Self,String>{json::Json::from_value(value)?.try_into()}
 fn from_value_controlled(value:&semio_framework_dsl_record::FieldValue,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<Self,ValueError>{json::Json::from_value_controlled(value,control)?.reconstruct_controlled(control)}
}
impl DslField for GltfMorphTarget{
 fn shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::Shape,ValueError>{encoding::morph_shape(control)}
 fn to_value_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,ValueError>{encoding::morph(self,control)}
 fn shape()->semio_framework_dsl_record::Shape{Target::shape()}
 fn to_value(&self)->semio_framework_dsl_record::FieldValue{Target{attributes:attributes(&self.0)}.to_value()}
 fn from_value(value:&semio_framework_dsl_record::FieldValue)->Result<Self,String>{let target=Target::from_value(value)?;Ok(Self(pairs(target.attributes)))}
 fn from_value_controlled(value:&semio_framework_dsl_record::FieldValue,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<Self,ValueError>{let target=Target::from_value_controlled(value,control)?;let mut attributes=control.allocate_vec(target.attributes.len())?;control.begin_stage(target.attributes.len())?;for value in target.attributes{control.step()?;attributes.push((value.semantic,value.accessor));}Ok(Self(attributes))}
}
fn optional_value<T: DslField>(value:&Option<T>)->semio_framework_dsl_record::FieldValue{match value{Some(value)=>value.to_value(),None=>semio_framework_dsl_record::FieldValue::Absent}}
impl DslField for GltfPrimitive{
 fn shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::Shape,ValueError>{encoding::primitive_shape(control)}
 fn to_value_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,ValueError>{encoding::primitive(self,control)}
 fn retire_decoded(self){if let Some(value)=self.extensions{GltfJson::retire_decoded(value);}if let Some(value)=self.extras{GltfJson::retire_decoded(value);}}
 fn shape()->semio_framework_dsl_record::Shape{Primitive::shape()}
 fn to_value(&self)->semio_framework_dsl_record::FieldValue{semio_framework_dsl_record::FieldValue::Record(semio_framework_dsl_record::RecordValue{fields:[(0,attributes(&self.attributes).to_value()),(1,optional_value(&self.indices)),(2,optional_value(&self.material)),(3,optional_value(&self.mode)),(4,self.targets.to_value()),(5,optional_value(&self.extensions)),(6,optional_value(&self.extras))].into_iter().collect()})}
 fn from_value(value:&semio_framework_dsl_record::FieldValue)->Result<Self,String>{let value=Primitive::from_value(value)?;Ok(Self{attributes:pairs(value.attributes),indices:value.indices,material:value.material,mode:value.mode,targets:value.targets,extensions:value.extensions,extras:value.extras})}
 fn from_value_controlled(value:&semio_framework_dsl_record::FieldValue,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<Self,ValueError>{let mut owner=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(Primitive::from_value_controlled(value,control)?,Primitive::retire_decoded);let count=owner.as_mut().attributes.len();let mut attributes=control.allocate_vec(count)?;control.begin_stage(count)?;for _ in&owner.as_mut().attributes{control.step()?;}let value=owner.take();for attribute in value.attributes{attributes.push((attribute.semantic,attribute.accessor));}Ok(Self{attributes,indices:value.indices,material:value.material,mode:value.mode,targets:value.targets,extensions:value.extensions,extras:value.extras})}
}
impl DslField for GltfCameraProjection{
 fn shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::Shape,ValueError>{encoding::camera_shape(control)}
 fn to_value_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,ValueError>{encoding::camera(self,control)}
 fn retire_decoded(self){match self{Self::Perspective(value)=>GltfPerspective::retire_decoded(value),Self::Orthographic(value)=>GltfOrthographic::retire_decoded(value)}}
 fn shape()->semio_framework_dsl_record::Shape{Projection::shape()}
 fn to_value(&self)->semio_framework_dsl_record::FieldValue{let(kind,perspective,orthographic)=match self{Self::Perspective(value)=>(ProjectionKind::Perspective.to_value(),value.to_value(),semio_framework_dsl_record::FieldValue::Absent),Self::Orthographic(value)=>(ProjectionKind::Orthographic.to_value(),semio_framework_dsl_record::FieldValue::Absent,value.to_value())};semio_framework_dsl_record::FieldValue::Record(semio_framework_dsl_record::RecordValue{fields:[(0,kind),(1,perspective),(2,orthographic)].into_iter().collect()})}
 fn from_value(value:&semio_framework_dsl_record::FieldValue)->Result<Self,String>{let mut owner=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(Projection::from_value(value)?,Projection::retire_decoded);let value=owner.as_mut();if !matches!((&value.kind,&value.perspective,&value.orthographic),(ProjectionKind::Perspective,Some(_),None)|(ProjectionKind::Orthographic,None,Some(_))){return Err("GLTF projection record components differ".into())}let value=owner.take();match value.kind{ProjectionKind::Perspective=>Ok(Self::Perspective(value.perspective.unwrap())),ProjectionKind::Orthographic=>Ok(Self::Orthographic(value.orthographic.unwrap()))}}
 fn from_value_controlled(value:&semio_framework_dsl_record::FieldValue,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<Self,ValueError>{let mut owner=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(Projection::from_value_controlled(value,control)?,Projection::retire_decoded);let value=owner.as_mut();if !matches!((&value.kind,&value.perspective,&value.orthographic),(ProjectionKind::Perspective,Some(_),None)|(ProjectionKind::Orthographic,None,Some(_))){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"GLTF projection record components differ"))}let value=owner.take();match value.kind{ProjectionKind::Perspective=>Ok(Self::Perspective(value.perspective.unwrap())),ProjectionKind::Orthographic=>Ok(Self::Orthographic(value.orthographic.unwrap()))}}
}
#[derive(semio_framework_dsl_record_derive::DslScalar)]
pub enum ImageKind{Image}
#[derive(semio_framework_dsl_record_derive::DslScalar)]
pub enum TextureKind{Texture}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub struct Image{kind:ImageKind,uri:Option<String>,mime_type:Option<String>,buffer_view:Option<usize>,name:Option<String>,extensions:Option<GltfJson>,extras:Option<GltfJson>}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub struct Texture{kind:TextureKind,sampler:Option<usize>,source:Option<usize>,name:Option<String>,extensions:Option<GltfJson>,extras:Option<GltfJson>}
impl DslField for GltfImage{
 fn shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::Shape,ValueError>{encoding::image_shape(control)}
 fn to_value_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,ValueError>{encoding::image(self,control)}
 fn retire_decoded(self){if let Some(value)=self.extensions{GltfJson::retire_decoded(value);}if let Some(value)=self.extras{GltfJson::retire_decoded(value);}}
 fn shape()->semio_framework_dsl_record::Shape{Image::shape()}
 fn to_value(&self)->semio_framework_dsl_record::FieldValue{semio_framework_dsl_record::FieldValue::Record(semio_framework_dsl_record::RecordValue{fields:[(0,ImageKind::Image.to_value()),(1,optional_value(&self.uri)),(2,optional_value(&self.mime_type)),(3,optional_value(&self.buffer_view)),(4,optional_value(&self.name)),(5,optional_value(&self.extensions)),(6,optional_value(&self.extras))].into_iter().collect()})}
 fn from_value(value:&semio_framework_dsl_record::FieldValue)->Result<Self,String>{let value=Image::from_value(value)?;Ok(Self{uri:value.uri,mime_type:value.mime_type,buffer_view:value.buffer_view,name:value.name,extensions:value.extensions,extras:value.extras})}
 fn from_value_controlled(value:&semio_framework_dsl_record::FieldValue,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<Self,ValueError>{let value=Image::from_value_controlled(value,control)?;Ok(Self{uri:value.uri,mime_type:value.mime_type,buffer_view:value.buffer_view,name:value.name,extensions:value.extensions,extras:value.extras})}
}
impl DslField for GltfTexture{
 fn shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::Shape,ValueError>{encoding::texture_shape(control)}
 fn to_value_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,ValueError>{encoding::texture(self,control)}
 fn retire_decoded(self){if let Some(value)=self.extensions{GltfJson::retire_decoded(value);}if let Some(value)=self.extras{GltfJson::retire_decoded(value);}}
 fn shape()->semio_framework_dsl_record::Shape{Texture::shape()}
 fn to_value(&self)->semio_framework_dsl_record::FieldValue{semio_framework_dsl_record::FieldValue::Record(semio_framework_dsl_record::RecordValue{fields:[(0,TextureKind::Texture.to_value()),(1,optional_value(&self.sampler)),(2,optional_value(&self.source)),(3,optional_value(&self.name)),(4,optional_value(&self.extensions)),(5,optional_value(&self.extras))].into_iter().collect()})}
 fn from_value(value:&semio_framework_dsl_record::FieldValue)->Result<Self,String>{let value=Texture::from_value(value)?;Ok(Self{sampler:value.sampler,source:value.source,name:value.name,extensions:value.extensions,extras:value.extras})}
 fn from_value_controlled(value:&semio_framework_dsl_record::FieldValue,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<Self,ValueError>{let value=Texture::from_value_controlled(value,control)?;Ok(Self{sampler:value.sampler,source:value.source,name:value.name,extensions:value.extensions,extras:value.extras})}
}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub struct ResolvedBuffer{pub bytes:Vec<u8>}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub struct Snapshot{pub schema:String,pub document:GltfDocument,pub buffers:Vec<ResolvedBuffer>,pub source_form:GltfSourceForm}
/// 🏭️ Supplies the actual controlled metadata authority without constructing ordinary metadata.
pub fn controlled_spec_producer()->semio_framework_dsl_record::RecordSpecProducer{encoding::spec_producer()}
