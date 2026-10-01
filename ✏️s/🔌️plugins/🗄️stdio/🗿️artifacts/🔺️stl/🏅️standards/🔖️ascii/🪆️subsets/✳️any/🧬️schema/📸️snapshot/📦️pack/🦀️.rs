//! 📦️ Typed STL facet records preserve snapshot identity and every binary64 word.
use super::{StlSnapshot,StlTriangle};
#[derive(dsl::DslRecord)]
struct Point{x:f64,y:f64,z:f64}
impl From<[f64;3]> for Point{fn from([x,y,z]:[f64;3])->Self{Self{x,y,z}}}
impl From<Point> for [f64;3]{fn from(value:Point)->Self{[value.x,value.y,value.z]}}
#[derive(dsl::DslRecord)]
struct Facet{normal:Point,vertex0:Point,vertex1:Point,vertex2:Point}
#[derive(dsl::DslRecord)]
struct Snapshot{schema:String,solid_name:String,triangles:Vec<Facet>}
impl From<&StlSnapshot> for Snapshot{fn from(value:&StlSnapshot)->Self{Self{schema:value.schema.clone(),solid_name:value.solid_name.clone(),triangles:value.triangles.iter().map(|value|Facet{normal:value.normal.into(),vertex0:value.vertices[0].into(),vertex1:value.vertices[1].into(),vertex2:value.vertices[2].into()}).collect()}}}
impl From<Snapshot> for StlSnapshot{fn from(value:Snapshot)->Self{Self{schema:value.schema,solid_name:value.solid_name,triangles:value.triangles.into_iter().map(|value|StlTriangle{normal:value.normal.into(),vertices:[value.vertex0.into(),value.vertex1.into(),value.vertex2.into()]}).collect()}}}
impl store::ArtifactDsl for StlSnapshot{
 const EXTENSION:&'static str="stl";
 fn envelope_id()->&'static str{"stdio.stl"}
 fn parse_dsl(text:&str)->Result<Self,store::TextError>{let body=store::semio_format::split_text_preamble(text).map(|(_,body)|body).unwrap_or(text);let record=dsl::parse(body,&Snapshot::__dsl_spec(),&dsl::ParseOptions{limits:dsl::Limits::default(),mode:dsl::SourceMode::Document})?;Ok(Snapshot::__dsl_from_record(&record)?.into())}
 fn print_dsl(&self)->String{let snapshot=Snapshot::from(self);let body=dsl::print(&snapshot.__dsl_to_record(),&Snapshot::__dsl_spec(),dsl::JoinMode::Document);let envelope=store::semio_format::SemioEnvelope::from_envelope_id("stdio.stl",store::semio_format::Component::Dsl,1).expect("valid owned snapshot identity");store::semio_format::wrap_text(&envelope,&body)}
}
impl store::ArtifactPack for StlSnapshot{
 fn sqlite_snapshot_codec()->Option<store::ArtifactSqliteSnapshotCodec>{Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())}
 fn encode_pack_with(&self,options:&store::PackEncodeOptions)->Result<Vec<u8>,store::PackError>{let snapshot=Snapshot::from(self);let inner=store::pack_rt::encode_document(&Snapshot::__dsl_spec(),&snapshot.__dsl_to_record(),options)?;let envelope=store::semio_format::SemioEnvelope::from_envelope_id("stdio.stl",store::semio_format::Component::Pack,1).map_err(|error|store::PackError::Schema(error.to_string()))?;Ok(store::semio_format::wrap_binary(&envelope,&inner))}
 fn decode_pack_with(bytes:&[u8],options:&store::PackDecodeOptions)->Result<Self,store::PackError>{let(envelope,inner)=store::semio_format::unwrap_binary(bytes).map_err(|error|store::PackError::Schema(error.to_string()))?;if !envelope.matches_identity("stdio.stl",store::semio_format::Component::Pack,1){return Err(store::PackError::Schema("owned snapshot pack identity differs".into()));}let(record,_)=store::pack_rt::decode_document(&inner,&Snapshot::__dsl_spec(),options)?;Snapshot::__dsl_from_record(&record).map(Into::into).map_err(store::text_error_to_pack_error)}
 fn record_spec()->Option<dsl::RecordSpec>{Some(Snapshot::__dsl_spec())}
}
