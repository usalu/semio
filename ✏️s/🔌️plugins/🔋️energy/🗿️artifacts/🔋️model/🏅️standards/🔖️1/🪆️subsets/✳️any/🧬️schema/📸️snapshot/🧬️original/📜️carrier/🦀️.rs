//! 📜️ The synchronous text and pack carriers print and receive the one-field intrinsic Record that the original pack borrows.
use super::EnergyModelSnapshot;
use crate::store::{PackDecodeOptions,PackEncodeOptions,PackError,pack_rt,semio_format::{Component,SemioEnvelope,split_text_preamble,unwrap_binary,wrap_binary,wrap_text}};
use semio_framework_diagnostic::{Limits,TextError,TextSpan};
use semio_framework_dsl_record::{BorrowedRecordSpecProducer,JoinMode,ParseOptions,RecordSpec,RecordValue,SourceMode};
use semio_framework_value::{DslValue,ToValue,ValueError,ValueRefusalKind};
use semio_framework_value_native_owned::blocking::receive_blocking;
use pack::snapshot::{ArtifactDsl,ArtifactPack};

/// 📄️ The owned image of the intrinsic Record: the canonical document value under its authored key.
#[derive(semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword="energy-model")]
struct EnergyModelDocument{document:DslValue}

impl EnergyModelDocument{
 fn record(snapshot:&EnergyModelSnapshot)->RecordValue{Self{document:snapshot.to_value()}.__dsl_to_record()}
 fn receive(record:&RecordValue)->Result<EnergyModelSnapshot,ValueError>{
  receive_blocking(Self::__dsl_from_record(record).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?.document)
 }
}

fn envelope(component:Component)->SemioEnvelope{SemioEnvelope::from_envelope_id(EnergyModelSnapshot::envelope_id(),component,1).expect("valid envelope_id")}

impl EnergyModelSnapshot{
 /// 🧬️ The canonical document value of one engine model under the authored schema id and empty link slots.
 pub fn document_value(model:&crate::model::Model)->DslValue{
  DslValue::object([("schema".to_string(),DslValue::String(crate::ENERGY_MODEL_DOCUMENT_SCHEMA.to_string())),("model".to_string(),model.to_value()),("referencedModel".to_string(),DslValue::Null),("weatherLink".to_string(),DslValue::Null)])
 }
 /// 🛬️ Receives one canonical document value into the native persistent owner.
 pub fn from_document(document:DslValue)->Result<Self,ValueError>{receive_blocking(document)}
 /// 🏗️ Receives one engine model into the native persistent owner through its canonical document value.
 pub fn from_model(model:&crate::model::Model)->Result<Self,ValueError>{Self::from_document(Self::document_value(model))}
}

/// 🌱️ The canonical empty engine model is the real empty document.
impl Default for EnergyModelSnapshot{
 fn default()->Self{Self::from_model(&crate::model::Model::default()).expect("the canonical empty model is admissible")}
}

impl ArtifactDsl for EnergyModelSnapshot{
 const EXTENSION:&'static str="energy";
 fn envelope_id()->&'static str{"energy.model"}
 fn parse_dsl(text:&str)->Result<Self,TextError>{
  let body=split_text_preamble(text).map_or(text,|(_,rest)|rest);
  let record=semio_framework_dsl_record::parse(body,&EnergyModelDocument::__dsl_spec(),&ParseOptions{limits:Limits::default(),mode:SourceMode::Document})?;
  EnergyModelDocument::receive(&record).map_err(|error|TextError::new(ValueRefusalKind::InvalidValue,error.to_string(),TextSpan::at(1,1)))
 }
 fn print_dsl(&self)->String{
  wrap_text(&envelope(Component::Dsl),&semio_framework_dsl_record::print(&EnergyModelDocument::record(self),&EnergyModelDocument::__dsl_spec(),JoinMode::Document))
 }
}

impl ArtifactPack for EnergyModelSnapshot{
 fn encode_pack_with(&self,options:&PackEncodeOptions)->Result<Vec<u8>,PackError>{
  Ok(wrap_binary(&envelope(Component::Pack),&pack_rt::encode_document(&EnergyModelDocument::__dsl_spec(),&EnergyModelDocument::record(self),options)?))
 }
 fn decode_pack_with(bytes:&[u8],options:&PackDecodeOptions)->Result<Self,PackError>{
  let(envelope,inner)=unwrap_binary(bytes).map_err(|error|PackError::from(error.into_value_error()))?;
  if !envelope.matches_identity(Self::envelope_id(),Component::Pack,1){return Err(PackError::from(ValueError::new(ValueRefusalKind::InvalidValue,format!("pack envelope mismatch: expected {}.pack v1, got {}",Self::envelope_id(),envelope.binary_token()))));}
  EnergyModelDocument::receive(&pack_rt::decode_document(&inner,&EnergyModelDocument::__dsl_spec(),options)?.0).map_err(PackError::from)
 }
 fn record_spec()->Option<RecordSpec>{Some(EnergyModelDocument::__dsl_spec())}
 fn borrowed_record_spec_producer()->Option<BorrowedRecordSpecProducer>{Some(BorrowedRecordSpecProducer::of::<Self>())}
}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
