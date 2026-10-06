//! 📦️ Typed STL facet records preserve snapshot identity and every binary64 word.
use crate::standards::v_ascii::subsets::any::schema::snapshot::{StlSnapshot,StlTriangle};
use semio_framework_diagnostic::{TextError,TextSpan};
use semio_framework_value::{ValueError,ValueRefusalKind};
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub(crate) struct Point{x:f64,y:f64,z:f64}
impl From<[f64;3]> for Point{fn from([x,y,z]:[f64;3])->Self{Self{x,y,z}}}
impl From<Point> for [f64;3]{fn from(value:Point)->Self{[value.x,value.y,value.z]}}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub(crate) struct Facet{normal:Point,vertex0:Point,vertex1:Point,vertex2:Point}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub(crate) struct Snapshot{schema:String,solid_name:String,triangles:Vec<Facet>}
impl From<&StlSnapshot> for Snapshot{fn from(value:&StlSnapshot)->Self{Self{schema:value.schema.clone(),solid_name:value.solid_name.clone(),triangles:value.triangles.iter().map(|value|Facet{normal:value.normal.into(),vertex0:value.vertices[0].into(),vertex1:value.vertices[1].into(),vertex2:value.vertices[2].into()}).collect()}}}
impl From<Snapshot> for StlSnapshot{fn from(value:Snapshot)->Self{Self{schema:value.schema,solid_name:value.solid_name,triangles:value.triangles.into_iter().map(|value|StlTriangle{normal:value.normal.into(),vertices:[value.vertex0.into(),value.vertex1.into(),value.vertex2.into()]}).collect()}}}

impl store::ArtifactPack for StlSnapshot{
 fn sqlite_snapshot_codec()->Option<store::ArtifactSqliteSnapshotCodec>{Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())}
 fn encode_pack_with(&self,options:&store::PackEncodeOptions)->Result<Vec<u8>,store::PackError>{let snapshot=Snapshot::from(self);let inner=store::pack_rt::encode_document(&Snapshot::__dsl_spec(),&snapshot.__dsl_to_record(),options)?;let envelope=store::semio_format::SemioEnvelope::from_envelope_id("stdio.stl",store::semio_format::Component::Pack,1).map_err(|error|store::PackError::from(error.into_value_error()))?;Ok(store::semio_format::wrap_binary(&envelope,&inner))}
 fn decode_pack_with(bytes:&[u8],options:&store::PackDecodeOptions)->Result<Self,store::PackError>{let(envelope,inner)=store::semio_format::unwrap_binary(bytes).map_err(|error|store::PackError::from(error.into_value_error()))?;if !envelope.matches_identity("stdio.stl",store::semio_format::Component::Pack,1){return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "owned snapshot pack identity differs")));}let(record,_)=store::pack_rt::decode_document(&inner,&Snapshot::__dsl_spec(),options)?;Snapshot::__dsl_from_record(&record).map(Into::into).map_err(store::text_error_to_pack_error)}
 fn record_spec()->Option<semio_framework_dsl_record::RecordSpec>{Some(Snapshot::__dsl_spec())}
}

pub(crate)fn spec()->semio_framework_dsl_record::RecordSpec{Snapshot::__dsl_spec()}
pub(crate)fn spec_producer()->semio_framework_dsl_record::RecordSpecProducer{Snapshot::__dsl_spec_producer()}
pub(crate)fn record_controlled(snapshot:&StlSnapshot,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::RecordValue,ValueError>{
 let schema=control.copy_text(&snapshot.schema)?;let solid_name=control.copy_text(&snapshot.solid_name)?;control.begin_stage(snapshot.triangles.len())?;let mut triangles=control.allocate_vec::<Facet>(snapshot.triangles.len())?;
 for value in &snapshot.triangles{control.step()?;triangles.push(Facet{normal:value.normal.into(),vertex0:value.vertices[0].into(),vertex1:value.vertices[1].into(),vertex2:value.vertices[2].into()});}
 Snapshot{schema,solid_name,triangles}.__dsl_to_record_controlled(control)
}
pub(crate)fn reconstruct_record_controlled(record:&semio_framework_dsl_record::RecordValue,control:&mut semio_framework_value::NativeDecodeControl<'_>,maximum_rows:usize)->Result<StlSnapshot,ValueError>{
 control.checkpoint()?;let count=match record.fields.get(&2){Some(semio_framework_dsl_record::FieldValue::List(values))=>values.len(),_=>return Err(semio_framework_value::ValueError::new(ValueRefusalKind::InvalidValue,"STL native facet list differs"))};
 let rows=count.checked_mul(4).and_then(|n|n.checked_add(1)).ok_or_else(||semio_framework_value::ValueError::new(ValueRefusalKind::WorkLimit,"STL native row count overflow"))?;if rows>maximum_rows{return Err(semio_framework_value::ValueError::new(ValueRefusalKind::OwnershipLimit,"STL native row limit"))}
 let Snapshot{schema,solid_name,triangles}=Snapshot::__dsl_from_record_controlled(record,control)?;
 control.begin_stage(count)?;let mut owned=control.allocate_vec::<StlTriangle>(count)?;
 for value in triangles{control.step()?;owned.push(StlTriangle{normal:value.normal.into(),vertices:[value.vertex0.into(),value.vertex1.into(),value.vertex2.into()]});}
 control.checkpoint()?;Ok(StlSnapshot{schema,solid_name,triangles:owned})
}
