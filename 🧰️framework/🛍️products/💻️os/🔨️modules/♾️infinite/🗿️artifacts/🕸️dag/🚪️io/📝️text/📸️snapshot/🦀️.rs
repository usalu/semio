//! 🚪️ Native artifact representation codecs.
use super::super::super::*;

/// 📜️ The actual persisted owner uses its own literal record factories.
impl crate::os_store::ArtifactDsl for DagSnapshot {
 const EXTENSION:&'static str=Self::__DSL_EXTENSION;
 fn envelope_id()->&'static str{Self::__DSL_ENVELOPE_ID}
 fn parse_dsl(text:&str)->Result<Self,crate::os_store::TextError>{
  let body=match crate::os_store::semio_format::split_text_preamble(text){Ok((_,rest))=>rest,Err(_)=>text};
  let record=semio_framework_dsl_record::parse(body,&Self::__dsl_spec(),&semio_framework_dsl_record::ParseOptions{limits:semio_framework_diagnostic::Limits::default(),mode:semio_framework_dsl_record::SourceMode::Document})?;
  Self::__dsl_from_record(&record)
 }
 fn print_dsl(&self)->String{
  let body=semio_framework_dsl_record::print(&self.__dsl_to_record(),&Self::__dsl_spec(),semio_framework_dsl_record::JoinMode::Document);
  let envelope=crate::os_store::semio_format::SemioEnvelope::from_envelope_id(<Self as crate::os_store::ArtifactDsl>::envelope_id(),crate::os_store::semio_format::Component::Dsl,1).expect("valid actual DAG envelope");
  crate::os_store::semio_format::wrap_text(&envelope,&body)
 }
}
