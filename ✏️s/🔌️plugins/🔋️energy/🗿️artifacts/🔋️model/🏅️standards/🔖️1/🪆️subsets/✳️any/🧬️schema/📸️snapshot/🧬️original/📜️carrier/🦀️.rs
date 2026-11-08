//! 📜️ The synchronous text and pack carriers print and receive the one-field intrinsic Record that the original pack borrows.
use super::EnergyModelSnapshot;
use crate::store::{PackDecodeOptions,PackEncodeOptions,PackError,pack_rt,semio_format::{Component,SemioEnvelope,split_text_preamble,unwrap_binary,wrap_binary,wrap_text}};
use semio_framework_diagnostic::{Limits,TextError,TextSpan};
use semio_framework_dsl_record::{BorrowedRecordSpecProducer,FieldSpec,FieldValue,JoinMode,ParseOptions,RecordLayout,RecordSpec,RecordValue,Shape,SourceMode};
use semio_framework_value::{DslValue,ToValue,ValueError,ValueRefusalKind};
use semio_s_artifact_energy_model_snapshot_bridge::{receive_stepped,retire_stepped};
use pack::snapshot::{ArtifactDsl,ArtifactPack};

/// 📄️ The owned image of the intrinsic Record: the canonical document value under its authored field id.
struct EnergyModelDocument;

impl EnergyModelDocument{
 const FIELD:u16=1;
 fn __dsl_spec()->RecordSpec{
  RecordSpec{keyword:Some("energy-model".into()),layout:RecordLayout::Inline,fields:vec![FieldSpec{id:Self::FIELD,key:"document".into(),position:None,shape:Shape::Value,optional:false,flatten:false,defines:None,is_call_name:false}]}
 }
 fn record(snapshot:&EnergyModelSnapshot)->RecordValue{RecordValue{fields:[(Self::FIELD,FieldValue::Value(snapshot.to_value()))].into_iter().collect()}}
 fn receive(record:&RecordValue)->Result<EnergyModelSnapshot,ValueError>{
  match record.fields.get(&Self::FIELD){
   Some(FieldValue::Value(document))=>receive_stepped(document.clone()),
   _=>Err(ValueError::literal(ValueRefusalKind::InvalidValue,"the energy-model record lacks its document value")),
  }
 }
}

fn envelope(component:Component)->SemioEnvelope{SemioEnvelope::from_envelope_id(EnergyModelSnapshot::envelope_id(),component,1).expect("valid envelope_id")}

impl EnergyModelSnapshot{
 /// 🧬️ The canonical document value of one engine model under the authored schema id and empty link slots.
 pub fn document_value(model:&crate::model::Model)->DslValue{
  DslValue::object([("schema".to_string(),DslValue::String(crate::ENERGY_MODEL_DOCUMENT_SCHEMA.to_string())),("model".to_string(),model.to_value()),("referencedModel".to_string(),DslValue::Null),("weatherLink".to_string(),DslValue::Null)])
 }
 /// 🛬️ Receives one canonical document value into the native persistent owner.
 pub fn from_document(document:DslValue)->Result<Self,ValueError>{receive_stepped(document)}
 /// 🏗️ Receives one engine model into the native persistent owner through its canonical document value.
 pub fn from_model(model:&crate::model::Model)->Result<Self,ValueError>{Self::from_document(Self::document_value(model))}
}

impl EnergyModelSnapshot{
 /// 🌱️ Constructs the canonical empty engine model as the real empty document; a refused construction is a typed failure, never a panic.
 pub fn empty()->Result<Self,ValueError>{Self::from_model(&crate::model::Model::default())}
 /// 📥️ Receives one document value into the native persistent owner in bounded paid steps.
 pub fn from_value(value:DslValue)->Result<Self,ValueError>{receive_stepped(value)}
}

impl EnergyModelSnapshot{
 /// ♻️ Retires this native owner in bounded paid steps.
 pub fn retire(self)->Result<(),ValueError>{retire_stepped(self)}
 /// ❄️ Projects the persistent owner to the plain explicit reference rows through its canonical wire value.
 pub fn to_reference(&self)->Result<crate::reference::EnergyModelSnapshot,ValueError>{
  let model=<crate::model::Model as semio_framework_value::FromValue>::from_value(self.model.to_value())?;
  let mut reference=crate::reference::energy_snapshot_owning(self.schema.get().clone(),model,self.referenced_model.get().clone());
  reference.weather_link=self.weather_link.get().clone();
  Ok(reference)
 }
 /// 🔥️ Receives the plain explicit reference rows back into the persistent owner.
 pub fn from_reference(reference:&crate::reference::EnergyModelSnapshot)->Result<Self,ValueError>{
  Self::from_document(DslValue::object([("schema".to_string(),DslValue::String(reference.schema.clone())),("model".to_string(),reference.model.to_value()),("referencedModel".to_string(),reference.referenced_model.to_value()),("weatherLink".to_string(),reference.weather_link.to_value())]))
 }
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
