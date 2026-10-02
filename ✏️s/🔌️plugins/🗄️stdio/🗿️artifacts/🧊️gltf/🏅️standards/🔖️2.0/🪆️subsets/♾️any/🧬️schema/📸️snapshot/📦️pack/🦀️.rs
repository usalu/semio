//! 📦️ Logical GLTF records preserve source form, independent buffers and all native fields.
use super::*;
use dsl::DslField;
#[derive(dsl::DslRecord)]
struct Attribute{semantic:String,accessor:usize}
#[derive(dsl::DslRecord)]
struct Target{attributes:Vec<Attribute>}
#[derive(dsl::DslRecord)]
struct Primitive{attributes:Vec<Attribute>,indices:Option<usize>,material:Option<usize>,mode:Option<u64>,targets:Vec<GltfMorphTarget>,extensions:Option<GltfJson>,extras:Option<GltfJson>}
#[derive(dsl::DslScalar)]
enum ProjectionKind{Perspective,Orthographic}
#[derive(dsl::DslRecord)]
struct Projection{kind:ProjectionKind,perspective:Option<GltfPerspective>,orthographic:Option<GltfOrthographic>}
fn attributes(value:&[(String,usize)])->Vec<Attribute>{value.iter().map(|(semantic,accessor)|Attribute{semantic:semantic.clone(),accessor:*accessor}).collect()}
fn pairs(value:Vec<Attribute>)->Vec<(String,usize)>{value.into_iter().map(|value|(value.semantic,value.accessor)).collect()}
#[path="🧩️extras/🦀️.rs"]
mod json;
fn optional<T:DslField>(value:&Option<T>)->dsl::FieldValue{match value{Some(value)=>value.to_value(),None=>dsl::FieldValue::Absent}}
fn retire_json(value:Option<GltfJson>){if let Some(value)=value{json::retire(value);}}
impl DslField for GltfJson{
 fn retire_decoded(self){json::retire(self)}
 fn shape()->dsl::Shape{json::Json::shape()}
 fn to_value(&self)->dsl::FieldValue{json::Json::from(self).to_value()}
 fn from_value(value:&dsl::FieldValue)->Result<Self,String>{json::Json::from_value(value)?.try_into()}
 fn from_value_controlled(value:&dsl::FieldValue,control:&mut dsl::NativeDecodeControl<'_>)->Result<Self,String>{json::Json::from_value_controlled(value,control)?.reconstruct_controlled(control)}
}
impl DslField for GltfMorphTarget{
 fn shape()->dsl::Shape{Target::shape()}
 fn to_value(&self)->dsl::FieldValue{Target{attributes:attributes(&self.0)}.to_value()}
 fn from_value(value:&dsl::FieldValue)->Result<Self,String>{let target=Target::from_value(value)?;Ok(Self(pairs(target.attributes)))}
 fn from_value_controlled(value:&dsl::FieldValue,control:&mut dsl::NativeDecodeControl<'_>)->Result<Self,String>{let target=Target::from_value_controlled(value,control)?;control.charge(target.attributes.len().checked_mul(std::mem::size_of::<(String,usize)>()).ok_or("GLTF attribute allocation overflow")?)?;let mut attributes=Vec::with_capacity(target.attributes.len());control.begin_stage(target.attributes.len())?;for value in target.attributes{control.step()?;attributes.push((value.semantic,value.accessor));}Ok(Self(attributes))}
}
fn optional_value<T: DslField>(value:&Option<T>)->dsl::FieldValue{match value{Some(value)=>value.to_value(),None=>dsl::FieldValue::Absent}}
impl DslField for GltfPrimitive{
 fn retire_decoded(self){if let Some(value)=self.extensions{GltfJson::retire_decoded(value);}if let Some(value)=self.extras{GltfJson::retire_decoded(value);}}
 fn shape()->dsl::Shape{Primitive::shape()}
 fn to_value(&self)->dsl::FieldValue{dsl::FieldValue::Record(dsl::RecordValue{fields:[(0,attributes(&self.attributes).to_value()),(1,optional_value(&self.indices)),(2,optional_value(&self.material)),(3,optional_value(&self.mode)),(4,self.targets.to_value()),(5,optional_value(&self.extensions)),(6,optional_value(&self.extras))].into_iter().collect()})}
 fn from_value(value:&dsl::FieldValue)->Result<Self,String>{let value=Primitive::from_value(value)?;Ok(Self{attributes:pairs(value.attributes),indices:value.indices,material:value.material,mode:value.mode,targets:value.targets,extensions:value.extensions,extras:value.extras})}
 fn from_value_controlled(value:&dsl::FieldValue,control:&mut dsl::NativeDecodeControl<'_>)->Result<Self,String>{let mut owner=dsl::__rt::DecodedFieldOwner::new(Primitive::from_value_controlled(value,control)?,Primitive::retire_decoded);let count=owner.as_mut().attributes.len();control.charge(count.checked_mul(std::mem::size_of::<(String,usize)>()).ok_or("GLTF attribute allocation overflow")?)?;control.begin_stage(count)?;for _ in&owner.as_mut().attributes{control.step()?;}let value=owner.take();let mut attributes=Vec::with_capacity(count);for attribute in value.attributes{attributes.push((attribute.semantic,attribute.accessor));}Ok(Self{attributes,indices:value.indices,material:value.material,mode:value.mode,targets:value.targets,extensions:value.extensions,extras:value.extras})}
}
impl DslField for GltfCameraProjection{
 fn retire_decoded(self){match self{Self::Perspective(value)=>GltfPerspective::retire_decoded(value),Self::Orthographic(value)=>GltfOrthographic::retire_decoded(value)}}
 fn shape()->dsl::Shape{Projection::shape()}
 fn to_value(&self)->dsl::FieldValue{let(kind,perspective,orthographic)=match self{Self::Perspective(value)=>(ProjectionKind::Perspective.to_value(),value.to_value(),dsl::FieldValue::Absent),Self::Orthographic(value)=>(ProjectionKind::Orthographic.to_value(),dsl::FieldValue::Absent,value.to_value())};dsl::FieldValue::Record(dsl::RecordValue{fields:[(0,kind),(1,perspective),(2,orthographic)].into_iter().collect()})}
 fn from_value(value:&dsl::FieldValue)->Result<Self,String>{let mut owner=dsl::__rt::DecodedFieldOwner::new(Projection::from_value(value)?,Projection::retire_decoded);let value=owner.as_mut();if !matches!((&value.kind,&value.perspective,&value.orthographic),(ProjectionKind::Perspective,Some(_),None)|(ProjectionKind::Orthographic,None,Some(_))){return Err("GLTF projection record components differ".into())}let value=owner.take();match value.kind{ProjectionKind::Perspective=>Ok(Self::Perspective(value.perspective.unwrap())),ProjectionKind::Orthographic=>Ok(Self::Orthographic(value.orthographic.unwrap()))}}
 fn from_value_controlled(value:&dsl::FieldValue,control:&mut dsl::NativeDecodeControl<'_>)->Result<Self,String>{let mut owner=dsl::__rt::DecodedFieldOwner::new(Projection::from_value_controlled(value,control)?,Projection::retire_decoded);let value=owner.as_mut();if !matches!((&value.kind,&value.perspective,&value.orthographic),(ProjectionKind::Perspective,Some(_),None)|(ProjectionKind::Orthographic,None,Some(_))){return Err("GLTF projection record components differ".into())}let value=owner.take();match value.kind{ProjectionKind::Perspective=>Ok(Self::Perspective(value.perspective.unwrap())),ProjectionKind::Orthographic=>Ok(Self::Orthographic(value.orthographic.unwrap()))}}
}
#[derive(dsl::DslScalar)]
enum ImageKind{Image}
#[derive(dsl::DslScalar)]
enum TextureKind{Texture}
#[derive(dsl::DslRecord)]
struct Image{kind:ImageKind,uri:Option<String>,mime_type:Option<String>,buffer_view:Option<usize>,name:Option<String>,extensions:Option<GltfJson>,extras:Option<GltfJson>}
#[derive(dsl::DslRecord)]
struct Texture{kind:TextureKind,sampler:Option<usize>,source:Option<usize>,name:Option<String>,extensions:Option<GltfJson>,extras:Option<GltfJson>}
impl DslField for GltfImage{
 fn retire_decoded(self){if let Some(value)=self.extensions{GltfJson::retire_decoded(value);}if let Some(value)=self.extras{GltfJson::retire_decoded(value);}}
 fn shape()->dsl::Shape{Image::shape()}
 fn to_value(&self)->dsl::FieldValue{dsl::FieldValue::Record(dsl::RecordValue{fields:[(0,ImageKind::Image.to_value()),(1,optional_value(&self.uri)),(2,optional_value(&self.mime_type)),(3,optional_value(&self.buffer_view)),(4,optional_value(&self.name)),(5,optional_value(&self.extensions)),(6,optional_value(&self.extras))].into_iter().collect()})}
 fn from_value(value:&dsl::FieldValue)->Result<Self,String>{let value=Image::from_value(value)?;Ok(Self{uri:value.uri,mime_type:value.mime_type,buffer_view:value.buffer_view,name:value.name,extensions:value.extensions,extras:value.extras})}
 fn from_value_controlled(value:&dsl::FieldValue,control:&mut dsl::NativeDecodeControl<'_>)->Result<Self,String>{let value=Image::from_value_controlled(value,control)?;Ok(Self{uri:value.uri,mime_type:value.mime_type,buffer_view:value.buffer_view,name:value.name,extensions:value.extensions,extras:value.extras})}
}
impl DslField for GltfTexture{
 fn retire_decoded(self){if let Some(value)=self.extensions{GltfJson::retire_decoded(value);}if let Some(value)=self.extras{GltfJson::retire_decoded(value);}}
 fn shape()->dsl::Shape{Texture::shape()}
 fn to_value(&self)->dsl::FieldValue{dsl::FieldValue::Record(dsl::RecordValue{fields:[(0,TextureKind::Texture.to_value()),(1,optional_value(&self.sampler)),(2,optional_value(&self.source)),(3,optional_value(&self.name)),(4,optional_value(&self.extensions)),(5,optional_value(&self.extras))].into_iter().collect()})}
 fn from_value(value:&dsl::FieldValue)->Result<Self,String>{let value=Texture::from_value(value)?;Ok(Self{sampler:value.sampler,source:value.source,name:value.name,extensions:value.extensions,extras:value.extras})}
 fn from_value_controlled(value:&dsl::FieldValue,control:&mut dsl::NativeDecodeControl<'_>)->Result<Self,String>{let value=Texture::from_value_controlled(value,control)?;Ok(Self{sampler:value.sampler,source:value.source,name:value.name,extensions:value.extensions,extras:value.extras})}
}
#[derive(dsl::DslRecord)]
struct ResolvedBuffer{bytes:Vec<u8>}
#[derive(dsl::DslRecord)]
struct Snapshot{schema:String,document:GltfDocument,buffers:Vec<ResolvedBuffer>,source_form:GltfSourceForm}
fn record(value:&GltfSnapshot)->dsl::RecordValue{let buffers=value.buffers.iter().map(|bytes|ResolvedBuffer{bytes:bytes.clone()}).collect::<Vec<_>>();dsl::RecordValue{fields:[(0,value.schema.to_value()),(1,value.document.to_value()),(2,buffers.to_value()),(3,value.source_form.to_value())].into_iter().collect()}}
impl From<Snapshot> for GltfSnapshot{fn from(value:Snapshot)->Self{Self{schema:value.schema,document:value.document,buffers:value.buffers.into_iter().map(|buffer|buffer.bytes).collect(),source_form:value.source_form}}}
impl store::ArtifactDsl for GltfSnapshot{
 const EXTENSION:&'static str="gltf";
 fn envelope_id()->&'static str{"stdio.gltf"}
 fn parse_dsl(text:&str)->Result<Self,store::TextError>{let body=match store::semio_format::split_text_preamble(text){Ok((envelope,body))=>{if !envelope.matches_identity("stdio.gltf",store::semio_format::Component::Dsl,1){return Err(dsl::__rt::field_error("GLTF logical text identity differs"))}body},Err(_)=>text};let record=dsl::parse_exact(body,&Snapshot::__dsl_spec(),&dsl::ParseOptions{limits:dsl::Limits::default(),mode:dsl::SourceMode::Document})?;Snapshot::__dsl_from_record(&record).map(Into::into)}
 fn print_dsl(&self)->String{let body=dsl::print(&record(self),&Snapshot::__dsl_spec(),dsl::JoinMode::Document);let envelope=store::semio_format::SemioEnvelope::from_envelope_id("stdio.gltf",store::semio_format::Component::Dsl,1).expect("valid GLTF owned identity");store::semio_format::wrap_text(&envelope,&body)}
}
impl store::ArtifactPack for GltfSnapshot{
 fn sqlite_snapshot_codec()->Option<store::ArtifactSqliteSnapshotCodec>{Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())}
 fn encode_pack_with(&self,options:&store::PackEncodeOptions)->Result<Vec<u8>,store::PackError>{let inner=store::pack_rt::encode_document(&Snapshot::__dsl_spec(),&record(self),options)?;let envelope=store::semio_format::SemioEnvelope::from_envelope_id("stdio.gltf",store::semio_format::Component::Pack,1).map_err(|error|store::PackError::Schema(error.to_string()))?;Ok(store::semio_format::wrap_binary(&envelope,&inner))}
 fn decode_pack_with(bytes:&[u8],options:&store::PackDecodeOptions)->Result<Self,store::PackError>{let(envelope,inner)=store::semio_format::unwrap_binary(bytes).map_err(|error|store::PackError::Schema(error.to_string()))?;if !envelope.matches_identity("stdio.gltf",store::semio_format::Component::Pack,1){return Err(store::PackError::Schema("GLTF owned pack identity differs".into()));}let(record,_)=store::pack_rt::decode_document(&inner,&Snapshot::__dsl_spec(),options)?;Snapshot::__dsl_from_record(&record).map(Into::into).map_err(store::text_error_to_pack_error)}
 fn record_spec()->Option<dsl::RecordSpec>{Some(Snapshot::__dsl_spec())}
}

/// 🛬️ Admit each declared native child and the resolved buffer collection before ownership transfer.
pub(super) fn reconstruct_record_controlled(record:&dsl::RecordValue,control:&mut dsl::NativeDecodeControl<'_>)->Result<GltfSnapshot,String>{
 let mut owner=dsl::__rt::DecodedFieldOwner::new(Snapshot::__dsl_from_record_controlled(record,control).map_err(|e|e.message)?,Snapshot::retire_decoded);let count=owner.as_mut().buffers.len();control.charge(count.checked_mul(std::mem::size_of::<Vec<u8>>()).ok_or("GLTF resolved buffer collection overflow")?)?;control.begin_stage(count)?;for _ in&owner.as_mut().buffers{control.step()?;}control.checkpoint()?;let Snapshot{schema,document,buffers,source_form}=owner.take();let mut owned=Vec::with_capacity(count);for buffer in buffers{owned.push(buffer.bytes);}Ok(GltfSnapshot{schema,document,buffers:owned,source_form})
}
