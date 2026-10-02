//! 📦️ Typed OBJ snapshot records preserve geometry, occurrence order, options and source positions.
use super::ObjSnapshot;
pub(super)fn spec()->dsl::RecordSpec{ObjSnapshot::__dsl_spec()}
pub(super)fn spec_producer()->dsl::schema::RecordSpecProducer{ObjSnapshot::__dsl_spec_producer()}
pub(super)fn reconstruct_record_controlled(source:&dsl::RecordValue,control:&mut dsl::NativeDecodeControl<'_>,maximum_rows:usize)->Result<ObjSnapshot,store::TextError>{
 fn list(source:&dsl::RecordValue,id:u16)->Result<&[dsl::FieldValue],String>{match source.fields.get(&id){Some(dsl::FieldValue::List(values))=>Ok(values),_=>Err("OBJ native list field differs".into())}}
 fn record(value:&dsl::FieldValue)->Result<&dsl::RecordValue,String>{match value{dsl::FieldValue::Record(value)=>Ok(value),_=>Err("OBJ native typed occurrence differs".into())}}
 control.scoped_stage(|control|->Result<(),String>{
  control.checkpoint()?;let faces=list(source,4)?;let groups=list(source,5)?;let objects=list(source,6)?;
  let mut rows=2usize.checked_add(faces.len()).ok_or("OBJ native row count overflow")?;
  for id in[1,2,3,4,5,6,8,9,10]{rows=rows.checked_add(list(source,id)?.len()).ok_or("OBJ native row count overflow")?;}
  let check=|rows|if rows>maximum_rows{Err(String::from("OBJ native row limit"))}else{Ok(())};check(rows)?;
  let units=faces.len().checked_add(groups.len()).and_then(|n|n.checked_add(objects.len())).ok_or("OBJ native work count overflow")?;control.begin_stage(units)?;
  for value in faces{control.step()?;rows=rows.checked_add(list(record(value)?,0)?.len()).ok_or("OBJ native row count overflow")?;check(rows)?;}
  for value in groups{control.step()?;rows=rows.checked_add(list(record(value)?,1)?.len()).ok_or("OBJ native row count overflow")?;check(rows)?;}
  for value in objects{control.step()?;rows=rows.checked_add(list(record(value)?,1)?.len()).ok_or("OBJ native row count overflow")?;check(rows)?;}
  control.checkpoint()?;Ok(())
 }).map_err(dsl::__rt::field_error)?;
 ObjSnapshot::__dsl_from_record_controlled(source,control)
}
impl store::ArtifactDsl for ObjSnapshot{
 const EXTENSION:&'static str="obj";
 fn envelope_id()->&'static str{"stdio.obj"}
 fn parse_dsl(text:&str)->Result<Self,store::TextError>{let(envelope,body)=store::semio_format::split_text_preamble(text).map_err(|error|dsl::__rt::field_error(error.to_string()))?;if !envelope.matches_identity("stdio.obj",store::semio_format::Component::Dsl,1){return Err(dsl::__rt::field_error("owned snapshot Text identity differs"));}let record=dsl::parse_exact(body,&ObjSnapshot::__dsl_spec(),&dsl::ParseOptions{limits:dsl::Limits::default(),mode:dsl::SourceMode::Document})?;ObjSnapshot::__dsl_from_record(&record)}
 fn print_dsl(&self)->String{let snapshot=self;let body=dsl::print(&snapshot.__dsl_to_record(),&ObjSnapshot::__dsl_spec(),dsl::JoinMode::Document);let envelope=store::semio_format::SemioEnvelope::from_envelope_id("stdio.obj",store::semio_format::Component::Dsl,1).expect("valid owned snapshot identity");store::semio_format::wrap_text(&envelope,&body)}
}
impl store::ArtifactPack for ObjSnapshot{
 fn sqlite_snapshot_codec()->Option<store::ArtifactSqliteSnapshotCodec>{Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())}
 fn encode_pack_with(&self,options:&store::PackEncodeOptions)->Result<Vec<u8>,store::PackError>{let snapshot=self;let inner=store::pack_rt::encode_document(&ObjSnapshot::__dsl_spec(),&snapshot.__dsl_to_record(),options)?;let envelope=store::semio_format::SemioEnvelope::from_envelope_id("stdio.obj",store::semio_format::Component::Pack,1).map_err(|error|store::PackError::Schema(error.to_string()))?;Ok(store::semio_format::wrap_binary(&envelope,&inner))}
 fn decode_pack_with(bytes:&[u8],options:&store::PackDecodeOptions)->Result<Self,store::PackError>{let(envelope,inner)=store::semio_format::unwrap_binary(bytes).map_err(|error|store::PackError::Schema(error.to_string()))?;if !envelope.matches_identity("stdio.obj",store::semio_format::Component::Pack,1){return Err(store::PackError::Schema("owned snapshot pack identity differs".into()));}let(record,_)=store::pack_rt::decode_document(&inner,&ObjSnapshot::__dsl_spec(),options)?;ObjSnapshot::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)}
 fn record_spec()->Option<dsl::RecordSpec>{Some(ObjSnapshot::__dsl_spec())}
}
