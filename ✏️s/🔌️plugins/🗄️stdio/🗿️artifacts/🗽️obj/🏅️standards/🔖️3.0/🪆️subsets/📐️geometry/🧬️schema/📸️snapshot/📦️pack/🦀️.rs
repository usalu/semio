//! 📦️ Typed OBJ snapshot records preserve geometry, occurrence order, options and source positions.
use super::ObjSnapshot;
use semio_framework_diagnostic::{TextError,TextSpan};
use semio_framework_value::{ValueError,ValueRefusalKind};
pub(super)fn spec()->semio_framework_dsl_record::RecordSpec{ObjSnapshot::__dsl_spec()}
pub(super)fn spec_producer()->semio_framework_dsl_record::RecordSpecProducer{ObjSnapshot::__dsl_spec_producer()}
pub(super)fn reconstruct_record_controlled(source:&semio_framework_dsl_record::RecordValue,control:&mut semio_framework_value::NativeDecodeControl<'_>,maximum_rows:usize)->Result<ObjSnapshot,ValueError>{
 fn list(source:&semio_framework_dsl_record::RecordValue,id:u16)->Result<&[semio_framework_dsl_record::FieldValue],ValueError>{match source.fields.get(&id){Some(semio_framework_dsl_record::FieldValue::List(values))=>Ok(values),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"OBJ native list field differs"))}}
 fn record(value:&semio_framework_dsl_record::FieldValue)->Result<&semio_framework_dsl_record::RecordValue,ValueError>{match value{semio_framework_dsl_record::FieldValue::Record(value)=>Ok(value),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"OBJ native typed occurrence differs"))}}
 control.scoped_stage(|control|->Result<(),ValueError>{
  control.checkpoint()?;let faces=list(source,4)?;let groups=list(source,5)?;let objects=list(source,6)?;
  let overflow=||ValueError::new(ValueRefusalKind::WorkLimit,"OBJ native row count overflow");
  let mut rows=2usize.checked_add(faces.len()).ok_or_else(overflow)?;
  for id in[1,2,3,4,5,6,8,9,10]{rows=rows.checked_add(list(source,id)?.len()).ok_or_else(overflow)?;}
  let check=|rows|if rows>maximum_rows{Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"OBJ native row limit"))}else{Ok(())};check(rows)?;
  let units=faces.len().checked_add(groups.len()).and_then(|n|n.checked_add(objects.len())).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"OBJ native work count overflow"))?;control.begin_stage(units)?;
  for value in faces{control.step()?;rows=rows.checked_add(list(record(value)?,0)?.len()).ok_or_else(overflow)?;check(rows)?;}
  for value in groups{control.step()?;rows=rows.checked_add(list(record(value)?,1)?.len()).ok_or_else(overflow)?;check(rows)?;}
  for value in objects{control.step()?;rows=rows.checked_add(list(record(value)?,1)?.len()).ok_or_else(overflow)?;check(rows)?;}
  control.checkpoint()?;Ok(())
 })?;
 ObjSnapshot::__dsl_from_record_controlled(source,control)
}
impl store::ArtifactDsl for ObjSnapshot{
 const EXTENSION:&'static str="obj";
 fn envelope_id()->&'static str{"stdio.obj"}
 fn parse_dsl(text:&str)->Result<Self,semio_framework_diagnostic::TextError>{let(envelope,body)=store::semio_format::split_text_preamble(text).map_err(|error|semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(error.to_string()).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))?;if !envelope.matches_identity("stdio.obj",store::semio_format::Component::Dsl,1){return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("owned snapshot Text identity differs").to_string(),semio_framework_diagnostic::TextSpan::at(1,1)));}let record=semio_framework_dsl_record::parse_exact(body,&ObjSnapshot::__dsl_spec(),&semio_framework_dsl_record::ParseOptions{limits:semio_framework_diagnostic::Limits::default(),mode:semio_framework_dsl_record::SourceMode::Document})?;ObjSnapshot::__dsl_from_record(&record)}
 fn print_dsl(&self)->String{let snapshot=self;let body=semio_framework_dsl_record::print(&snapshot.__dsl_to_record(),&ObjSnapshot::__dsl_spec(),semio_framework_dsl_record::JoinMode::Document);let envelope=store::semio_format::SemioEnvelope::from_envelope_id("stdio.obj",store::semio_format::Component::Dsl,1).expect("valid owned snapshot identity");store::semio_format::wrap_text(&envelope,&body)}
}
impl store::ArtifactPack for ObjSnapshot{
 fn sqlite_snapshot_codec()->Option<store::ArtifactSqliteSnapshotCodec>{Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())}
 fn encode_pack_with(&self,options:&store::PackEncodeOptions)->Result<Vec<u8>,store::PackError>{let snapshot=self;let inner=store::pack_rt::encode_document(&ObjSnapshot::__dsl_spec(),&snapshot.__dsl_to_record(),options)?;let envelope=store::semio_format::SemioEnvelope::from_envelope_id("stdio.obj",store::semio_format::Component::Pack,1).map_err(|error|store::PackError::from(error.into_value_error()))?;Ok(store::semio_format::wrap_binary(&envelope,&inner))}
 fn decode_pack_with(bytes:&[u8],options:&store::PackDecodeOptions)->Result<Self,store::PackError>{let(envelope,inner)=store::semio_format::unwrap_binary(bytes).map_err(|error|store::PackError::from(error.into_value_error()))?;if !envelope.matches_identity("stdio.obj",store::semio_format::Component::Pack,1){return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "owned snapshot pack identity differs")));}let(record,_)=store::pack_rt::decode_document(&inner,&ObjSnapshot::__dsl_spec(),options)?;ObjSnapshot::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)}
 fn record_spec()->Option<semio_framework_dsl_record::RecordSpec>{Some(ObjSnapshot::__dsl_spec())}
}
