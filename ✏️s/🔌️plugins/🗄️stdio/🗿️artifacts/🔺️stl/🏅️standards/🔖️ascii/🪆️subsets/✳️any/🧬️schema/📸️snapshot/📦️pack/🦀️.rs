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
 fn parse_dsl(text:&str)->Result<Self,store::TextError>{let(envelope,body)=store::semio_format::split_text_preamble(text).map_err(|error|dsl::__rt::field_error(error.to_string()))?;if !envelope.matches_identity("stdio.stl",store::semio_format::Component::Dsl,1){return Err(dsl::__rt::field_error("owned snapshot Text identity differs"));}let record=dsl::parse_exact(body,&Snapshot::__dsl_spec(),&dsl::ParseOptions{limits:dsl::Limits::default(),mode:dsl::SourceMode::Document})?;Ok(Snapshot::__dsl_from_record(&record)?.into())}
 fn print_dsl(&self)->String{let snapshot=Snapshot::from(self);let body=dsl::print(&snapshot.__dsl_to_record(),&Snapshot::__dsl_spec(),dsl::JoinMode::Document);let envelope=store::semio_format::SemioEnvelope::from_envelope_id("stdio.stl",store::semio_format::Component::Dsl,1).expect("valid owned snapshot identity");store::semio_format::wrap_text(&envelope,&body)}
}
impl store::ArtifactPack for StlSnapshot{
 fn sqlite_snapshot_codec()->Option<store::ArtifactSqliteSnapshotCodec>{Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())}
 fn encode_pack_with(&self,options:&store::PackEncodeOptions)->Result<Vec<u8>,store::PackError>{let snapshot=Snapshot::from(self);let inner=store::pack_rt::encode_document(&Snapshot::__dsl_spec(),&snapshot.__dsl_to_record(),options)?;let envelope=store::semio_format::SemioEnvelope::from_envelope_id("stdio.stl",store::semio_format::Component::Pack,1).map_err(|error|store::PackError::Schema(error.to_string()))?;Ok(store::semio_format::wrap_binary(&envelope,&inner))}
 fn decode_pack_with(bytes:&[u8],options:&store::PackDecodeOptions)->Result<Self,store::PackError>{let(envelope,inner)=store::semio_format::unwrap_binary(bytes).map_err(|error|store::PackError::Schema(error.to_string()))?;if !envelope.matches_identity("stdio.stl",store::semio_format::Component::Pack,1){return Err(store::PackError::Schema("owned snapshot pack identity differs".into()));}let(record,_)=store::pack_rt::decode_document(&inner,&Snapshot::__dsl_spec(),options)?;Snapshot::__dsl_from_record(&record).map(Into::into).map_err(store::text_error_to_pack_error)}
 fn record_spec()->Option<dsl::RecordSpec>{Some(Snapshot::__dsl_spec())}
}

pub(super)fn spec()->dsl::RecordSpec{Snapshot::__dsl_spec()}
pub(super)fn spec_producer()->dsl::schema::RecordSpecProducer{Snapshot::__dsl_spec_producer()}
pub(super)fn record_controlled(snapshot:&StlSnapshot,control:&mut dsl::NativeEncodeControl<'_>)->Result<dsl::RecordValue,dsl::TextError>{
 let schema=control.copy_text(&snapshot.schema).map_err(dsl::__rt::field_error)?;let solid_name=control.copy_text(&snapshot.solid_name).map_err(dsl::__rt::field_error)?;control.begin_stage(snapshot.triangles.len()).map_err(dsl::__rt::field_error)?;let mut triangles=control.allocate_vec::<Facet>(snapshot.triangles.len()).map_err(dsl::__rt::field_error)?;
 for value in &snapshot.triangles{control.step().map_err(dsl::__rt::field_error)?;triangles.push(Facet{normal:value.normal.into(),vertex0:value.vertices[0].into(),vertex1:value.vertices[1].into(),vertex2:value.vertices[2].into()});}
 Snapshot{schema,solid_name,triangles}.__dsl_to_record_controlled(control)
}
pub(super)fn reconstruct_record_controlled(record:&dsl::RecordValue,control:&mut dsl::NativeDecodeControl<'_>,maximum_rows:usize)->Result<StlSnapshot,store::TextError>{
 control.checkpoint().map_err(dsl::__rt::field_error)?;let count=match record.fields.get(&2){Some(dsl::FieldValue::List(values))=>values.len(),_=>return Err(dsl::__rt::field_error("STL native facet list differs"))};
 let rows=count.checked_mul(4).and_then(|n|n.checked_add(1)).ok_or_else(||dsl::__rt::field_error("STL native row count overflow"))?;if rows>maximum_rows{return Err(dsl::__rt::field_error("STL native row limit"))}
 let Snapshot{schema,solid_name,triangles}=Snapshot::__dsl_from_record_controlled(record,control)?;
 control.begin_stage(count).map_err(dsl::__rt::field_error)?;let mut owned=control.allocate_vec::<StlTriangle>(count).map_err(dsl::__rt::field_error)?;
 for value in triangles{control.step().map_err(dsl::__rt::field_error)?;owned.push(StlTriangle{normal:value.normal.into(),vertices:[value.vertex0.into(),value.vertex1.into(),value.vertex2.into()]});}
 control.checkpoint().map_err(dsl::__rt::field_error)?;Ok(StlSnapshot{schema,solid_name,triangles:owned})
}
