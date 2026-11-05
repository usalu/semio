//! 📥️ Actor ingress reports readiness separately from the actual retained or returned wire owner.
use super::{RetainedTurnInput,RetainedCloneProgress,ValueRefusalKind};
#[derive(Clone,Copy,Debug,PartialEq,Eq,serde::Serialize,serde::Deserialize,semio_framework_value::RetireOwned)]
#[serde(rename_all="camelCase")]
#[value(crate="::semio_framework_value")]
pub enum OriginalActorInputDisposition{Borrowed,Retained,Returned}
/// 🧵️ WIT transfers an owned wire; Owned Core's separate fixed borrow witness never uses this vector disposition.
#[derive(Debug,PartialEq,Eq,serde::Serialize,serde::Deserialize,semio_framework_value::RetireOwned)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
#[value(crate="::semio_framework_value")]
pub struct OriginalActorInputAdmissionReply{pub input:RetainedTurnInput,pub spent:RetainedCloneProgress,pub work_units_spent:u64,pub ready:bool,pub disposition:OriginalActorInputDisposition,pub returned_wire:Option<Vec<u8>>,pub refusal:Option<ValueRefusalKind>}
impl OriginalActorInputAdmissionReply{
 pub fn disposition_is_well_formed(&self)->bool{match self.disposition{OriginalActorInputDisposition::Returned=>self.returned_wire.is_some(),OriginalActorInputDisposition::Borrowed|OriginalActorInputDisposition::Retained=>self.returned_wire.is_none()}}
}
#[cfg(test)]
mod tests{
 use super::*;
 #[test]
 fn actor_ingress_keeps_returned_wire_and_readiness_as_separate_raw_facts(){let corpus:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();for row in corpus["cases"].as_array().unwrap(){for ready in[false,true]{let reply=OriginalActorInputAdmissionReply{input:RetainedTurnInput{operation:1,generation:1,epoch:1,work:semio_framework_actor_retained_turn::OriginalActorTurnWork{clock_binding:1,absolute_deadline_us:1,fuel:1},grant:Default::default()},spent:Default::default(),work_units_spent:0,ready,disposition:serde_json::from_value(row["disposition"].clone()).unwrap(),returned_wire:serde_json::from_value(row["returnedWire"].clone()).unwrap(),refusal:None};assert_eq!(reply.disposition_is_well_formed(),row["valid"].as_bool().unwrap());let wire=serde_json::to_value(&reply).unwrap();assert_eq!(wire["returnedWire"],row["returnedWire"]);let original:OriginalActorInputAdmissionReply=serde_json::from_value(wire).unwrap();assert_eq!(original,reply);}}}
}
