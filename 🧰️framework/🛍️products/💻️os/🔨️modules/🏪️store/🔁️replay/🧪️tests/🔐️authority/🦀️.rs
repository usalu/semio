//! 🔐️ Executable public raw replay authority laws share independent neutral witnesses.
#![cfg(test)]
use semio_framework_os_spr as protocol;
use semio_framework_os_store_replay::{EditReplay,EffectiveSupersessions,ReplayEdits,ReplayMode};
use semio_framework_value::{DslValue,FromValue,ToValue,ValueError,RetainedCloneGrant,ValueRefusalKind};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use std::{cell::Cell,sync::Arc};
thread_local!{static CALLS:Cell<usize>=const{Cell::new(0)};}
#[derive(Clone,Debug,PartialEq)]
struct SetCounter(i64);
#[derive(Clone,Default,PartialEq)]
struct CounterDiff(Option<i64>);
impl ToValue for SetCounter{fn to_value(&self)->DslValue{self.0.to_value()}}
impl FromValue for SetCounter{fn from_value(value:DslValue)->Result<Self,ValueError>{i64::from_value(value).map(Self)}}
impl ToValue for CounterDiff{fn to_value(&self)->DslValue{self.0.to_value()}}
impl FromValue for CounterDiff{fn from_value(value:DslValue)->Result<Self,ValueError>{Option::<i64>::from_value(value).map(Self)}}
impl protocol::DiffAlgebra<i64> for CounterDiff{fn is_empty(&self)->bool{self.0.is_none()}}
impl protocol::MutationDiff<i64> for CounterDiff{}
impl protocol::ColdDiffAlgebra<i64> for CounterDiff{fn inverse(&self,base:&i64)->Self{Self(self.0.map(|_|*base))}}
impl protocol::ColdMutationDiff<i64> for CounterDiff{
 fn absorb(&mut self,other:Self){if other.0.is_some(){self.0=other.0}}
 fn apply(&self,unpublished:&mut i64,_capability:protocol::ApplyCapability)->protocol::MutationApplyResult<()>{if let Some(value)=self.0{*unpublished=value}Ok(())}
}
const DESCRIPTOR:protocol::MutationLeafDescriptor=protocol::MutationLeafDescriptor{
 schema_version:1,owner:"testkit.replay.authority.set-counter",semantic_kind:"set-counter",display_name:"Set Counter",emoji:"🔢️",aggregate_variant:"SetCounter",payload_schema:include_str!("payload.json"),text_opcode:None,binary_tag:Some(1),invertibility:protocol::MutationInvertibility::ExplicitMutation,diff_participation:protocol::MutationDiffParticipation::Detect,outcome_classes:&[protocol::MutationOutcomeClass::Applied],composition:protocol::MutationComposition::Atomic,required_language_surfaces:&[protocol::MutationLanguageSurface::Rust,protocol::MutationLanguageSurface::JsonSchema],
};
impl protocol::Mutation<i64> for SetCounter{
 type Diff=CounterDiff;
 const DESCRIPTORS:&'static[protocol::MutationLeafDescriptor]=&[DESCRIPTOR];
 fn descriptor(&self)->&'static protocol::MutationLeafDescriptor{&DESCRIPTOR}
 fn may_emit_foreign_steps(&self)->bool{false}
}
impl protocol::ColdMutation<i64> for SetCounter{
 type ReferenceDiff=CounterDiff;
 fn diff(&self,_base:&i64)->protocol::MutationOutcome<CounterDiff>{CALLS.with(|calls|calls.set(calls.get()+1));protocol::MutationOutcome::new(CounterDiff(Some(self.0)))}
 fn inverse(&self,base:&i64)->Result<Vec<Self>,ValueError>{Ok(vec![Self(*base)])}
}
impl protocol::OpBinary for SetCounter{
 fn encode_op(&self)->Result<Vec<u8>,protocol::ProtocolError>{Ok(self.0.to_le_bytes().to_vec())}
 fn decode_op(bytes:&[u8])->Result<Self,protocol::ProtocolError>{let bytes:[u8;8]=bytes.try_into().map_err(|_|protocol::ProtocolError::Malformed{what:"counter",offset:0,detail:String::from("expected eight bytes")})?;Ok(Self(i64::from_le_bytes(bytes)))}
}
struct Edits(protocol::Edit<SetCounter>);
impl ReplayEdits<SetCounter> for Edits{fn replay_edit(&self,id:&str)->Option<&protocol::Edit<SetCounter>>{(self.0.id==id).then_some(&self.0)}}
fn run_case(name:&str){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 assert_eq!(fixture["schema"],"semio.replay-authority.v1");
 let row=fixture["cases"].as_array().unwrap().iter().find(|row|row["name"]==name).unwrap();
 CALLS.with(|calls|calls.set(0));
 let id=String::from("replay-authority-edit");
 let edits=Edits(protocol::Edit{id:id.clone(),actor:None,line:None,forwards:vec![SetCounter(row["nextCounter"].as_i64().unwrap())],inverse:Vec::new().into(),mutation_meta:Vec::new(),verb:None,sequence_number:0,started_at:String::from("2026-10-28T00:00:00Z"),finished_at:None});
 let mut order=semio_framework_os_vcs::HistoryPageStack::new();order.push(id);
 let baseline=row["state"]["counter"].as_i64().unwrap();
 let mut raw=EditReplay::new_counted(ReplayMode::Report,Arc::new(baseline),order,0,String::new(),EffectiveSupersessions::new(),1).unwrap();
 assert!(!row["producerInstalled"].as_bool().unwrap());assert!(raw.operation_preparation_factory.is_none());
 let before=(Arc::as_ptr(raw.state.as_ref().unwrap()),raw.cursor,raw.operation,raw.progress(),raw.outcomes.len(),raw.committed.len(),raw.rebased_inverse.len());
 let error=if row["authority"]=="original"{
  let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64};
  let mut callback=|_|true;let mut native=NativeDecodeControl::new_retained(&mut callback);native.admit_turn_capacity(grant.maximum_capacity_bytes).unwrap();
  let error=raw.step_granted(&edits,grant,&mut native,&mut||false).unwrap_err();
  assert!(matches!(error,semio_framework_os_vcs::VcsError::InverseRefused(ref error)if error.kind==ValueRefusalKind::UnsupportedOwner));
  assert_eq!((Arc::as_ptr(raw.state.as_ref().unwrap()),raw.cursor,raw.operation,raw.progress(),raw.outcomes.len(),raw.committed.len(),raw.rebased_inverse.len()),before);
  assert_eq!(raw.take_native_receipt(),Default::default());assert_eq!(native.owned_bytes(),0);
  Some("UnsupportedOwner")
 }else{
  assert_eq!(row["authority"],"cold");raw.step_operations(&edits,1,&mut||false).unwrap();assert_eq!(raw.operation,1);None
 };
 let state=serde_json::json!({"counter":**raw.state.as_ref().unwrap(),"calls":CALLS.with(Cell::get)});
 let actual=serde_json::json!({"accepted":error.is_none(),"unchanged":state==row["state"],"error":error,"state":state});
 assert_eq!(actual,row["expected"]);println!("authority-case {name}: {actual}");
 raw.retire_unscheduled().unwrap();
}
#[test]
fn original_missing_producer_preserves_original_state_and_frontier(){run_case("original-missing-producer")}
#[test]
fn explicit_cold_executor_matches_independent_authority_witness(){run_case("explicit-cold-executor")}
