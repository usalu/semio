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
impl DslField for GltfJson{
 fn shape()->dsl::Shape{json::Json::shape()}
 fn to_value(&self)->dsl::FieldValue{json::Json::from(self).to_value()}
 fn from_value(value:&dsl::FieldValue)->Result<Self,String>{json::Json::from_value(value)?.try_into()}
}
impl DslField for GltfMorphTarget{
 fn shape()->dsl::Shape{Target::shape()}
 fn to_value(&self)->dsl::FieldValue{Target{attributes:attributes(&self.0)}.to_value()}
 fn from_value(value:&dsl::FieldValue)->Result<Self,String>{let target=Target::from_value(value)?;Ok(Self(pairs(target.attributes)))}
}
impl DslField for GltfPrimitive{
 fn shape()->dsl::Shape{Primitive::shape()}
 fn to_value(&self)->dsl::FieldValue{dsl::FieldValue::Record(dsl::RecordValue{fields:[(0,attributes(&self.attributes).to_value()),(1,self.indices.to_value()),(2,self.material.to_value()),(3,self.mode.to_value()),(4,self.targets.to_value()),(5,DslField::to_value(&self.extensions)),(6,DslField::to_value(&self.extras))].into_iter().collect()})}
 fn from_value(value:&dsl::FieldValue)->Result<Self,String>{let value=Primitive::from_value(value)?;Ok(Self{attributes:pairs(value.attributes),indices:value.indices,material:value.material,mode:value.mode,targets:value.targets,extensions:value.extensions,extras:value.extras})}
}
impl DslField for GltfCameraProjection{
 fn shape()->dsl::Shape{Projection::shape()}
 fn to_value(&self)->dsl::FieldValue{let(kind,perspective,orthographic)=match self{Self::Perspective(value)=>(ProjectionKind::Perspective.to_value(),value.to_value(),dsl::FieldValue::Absent),Self::Orthographic(value)=>(ProjectionKind::Orthographic.to_value(),dsl::FieldValue::Absent,value.to_value())};dsl::FieldValue::Record(dsl::RecordValue{fields:[(0,kind),(1,perspective),(2,orthographic)].into_iter().collect()})}
 fn from_value(value:&dsl::FieldValue)->Result<Self,String>{let value=Projection::from_value(value)?;match(value.kind,value.perspective,value.orthographic){(ProjectionKind::Perspective,Some(value),None)=>Ok(Self::Perspective(value)),(ProjectionKind::Orthographic,None,Some(value))=>Ok(Self::Orthographic(value)),_=>Err("GLTF projection record components differ".into())}}
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
 fn parse_dsl(text:&str)->Result<Self,store::TextError>{let body=store::semio_format::split_text_preamble(text).map(|(_,body)|body).unwrap_or(text);let record=dsl::parse(body,&Snapshot::__dsl_spec(),&dsl::ParseOptions{limits:dsl::Limits::default(),mode:dsl::SourceMode::Document})?;Snapshot::__dsl_from_record(&record).map(Into::into)}
 fn print_dsl(&self)->String{let body=dsl::print(&record(self),&Snapshot::__dsl_spec(),dsl::JoinMode::Document);let envelope=store::semio_format::SemioEnvelope::from_envelope_id("stdio.gltf",store::semio_format::Component::Dsl,1).expect("valid GLTF owned identity");store::semio_format::wrap_text(&envelope,&body)}
}
impl store::ArtifactPack for GltfSnapshot{
 fn sqlite_snapshot_codec()->Option<store::ArtifactSqliteSnapshotCodec>{Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())}
 fn encode_pack_with(&self,options:&store::PackEncodeOptions)->Result<Vec<u8>,store::PackError>{let inner=store::pack_rt::encode_document(&Snapshot::__dsl_spec(),&record(self),options)?;let envelope=store::semio_format::SemioEnvelope::from_envelope_id("stdio.gltf",store::semio_format::Component::Pack,1).map_err(|error|store::PackError::Schema(error.to_string()))?;Ok(store::semio_format::wrap_binary(&envelope,&inner))}
 fn decode_pack_with(bytes:&[u8],options:&store::PackDecodeOptions)->Result<Self,store::PackError>{let(envelope,inner)=store::semio_format::unwrap_binary(bytes).map_err(|error|store::PackError::Schema(error.to_string()))?;if !envelope.matches_identity("stdio.gltf",store::semio_format::Component::Pack,1){return Err(store::PackError::Schema("GLTF owned pack identity differs".into()));}let(record,_)=store::pack_rt::decode_document(&inner,&Snapshot::__dsl_spec(),options)?;Snapshot::__dsl_from_record(&record).map(Into::into).map_err(store::text_error_to_pack_error)}
 fn record_spec()->Option<dsl::RecordSpec>{Some(Snapshot::__dsl_spec())}
}
