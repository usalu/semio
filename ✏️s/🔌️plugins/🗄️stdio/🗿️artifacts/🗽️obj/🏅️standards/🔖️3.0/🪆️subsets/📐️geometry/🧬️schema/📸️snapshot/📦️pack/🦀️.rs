//! 📦️ Typed OBJ snapshot records preserve geometry, occurrence order, options and source positions.
use super::ObjSnapshot;
impl store::ArtifactDsl for ObjSnapshot{
 const EXTENSION:&'static str="obj";
 fn envelope_id()->&'static str{"stdio.obj"}
 fn parse_dsl(text:&str)->Result<Self,store::TextError>{let body=store::semio_format::split_text_preamble(text).map(|(_,body)|body).unwrap_or(text);let record=dsl::parse(body,&ObjSnapshot::__dsl_spec(),&dsl::ParseOptions{limits:dsl::Limits::default(),mode:dsl::SourceMode::Document})?;ObjSnapshot::__dsl_from_record(&record)}
 fn print_dsl(&self)->String{let snapshot=self;let body=dsl::print(&snapshot.__dsl_to_record(),&ObjSnapshot::__dsl_spec(),dsl::JoinMode::Document);let envelope=store::semio_format::SemioEnvelope::from_envelope_id("stdio.obj",store::semio_format::Component::Dsl,1).expect("valid owned snapshot identity");store::semio_format::wrap_text(&envelope,&body)}
}
impl store::ArtifactPack for ObjSnapshot{
 fn sqlite_snapshot_codec()->Option<store::ArtifactSqliteSnapshotCodec>{Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())}
 fn encode_pack_with(&self,options:&store::PackEncodeOptions)->Result<Vec<u8>,store::PackError>{let snapshot=self;let inner=store::pack_rt::encode_document(&ObjSnapshot::__dsl_spec(),&snapshot.__dsl_to_record(),options)?;let envelope=store::semio_format::SemioEnvelope::from_envelope_id("stdio.obj",store::semio_format::Component::Pack,1).map_err(|error|store::PackError::Schema(error.to_string()))?;Ok(store::semio_format::wrap_binary(&envelope,&inner))}
 fn decode_pack_with(bytes:&[u8],options:&store::PackDecodeOptions)->Result<Self,store::PackError>{let(envelope,inner)=store::semio_format::unwrap_binary(bytes).map_err(|error|store::PackError::Schema(error.to_string()))?;if !envelope.matches_identity("stdio.obj",store::semio_format::Component::Pack,1){return Err(store::PackError::Schema("owned snapshot pack identity differs".into()));}let(record,_)=store::pack_rt::decode_document(&inner,&ObjSnapshot::__dsl_spec(),options)?;ObjSnapshot::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)}
 fn record_spec()->Option<dsl::RecordSpec>{Some(ObjSnapshot::__dsl_spec())}
}
