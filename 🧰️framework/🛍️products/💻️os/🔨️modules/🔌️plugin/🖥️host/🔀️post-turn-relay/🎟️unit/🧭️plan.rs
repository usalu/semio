//! 🧭️ The Source-native cold controller selects one authored ABI crossing from its actual protocol frontier.
use semio_framework_actor::OriginalJobBudget;
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_plugin_guest_contract::original_actor_context::ColdJobAction;
use semio_plugin_host_guest_runtime::{OriginalCoreValue,OwnedSemioExport};
use semio_plugin_host_wake::GuestRelayGuestPurpose;
/// 🎯️ A scalar plan contains only inline ABI values; it does not grant or duplicate Source authority.
pub struct ColdJobCallPlan{pub export:OwnedSemioExport,pub arguments:[OriginalCoreValue;OwnedSemioExport::MAX_SCALAR_ARGUMENTS],pub length:usize,pub purpose:Option<GuestRelayGuestPurpose>,pub page:bool}
impl ColdJobCallPlan{
 pub fn new(action:ColdJobAction,job:u64,failure:Option<semio_framework_actor::ActorFailureCallReference>,offset:usize,length:usize,budget:OriginalJobBudget)->Result<Self,ValueError>{
  require_core_wire(&action,budget)?;let actor_failure=matches!(action,ColdJobAction::PublishFailure|ColdJobAction::ReadFailure|ColdJobAction::AcknowledgeFailure|ColdJobAction::AcknowledgeFailureChunk|ColdJobAction::BorrowFailureChunk);let reference=if actor_failure{let reference=failure.ok_or_else(refused)?;reference.validate()?;Some(reference)}else{None};let mut arguments=[OriginalCoreValue::I64(0);OwnedSemioExport::MAX_SCALAR_ARGUMENTS];let(export,prefix,purpose,page)=match action{
   ColdJobAction::AdmitActor=>(OwnedSemioExport::AdmitActorContext,0,Some(GuestRelayGuestPurpose::Infrastructure),false),
   ColdJobAction::CloseJobs=>(OwnedSemioExport::CloseJobs,0,Some(GuestRelayGuestPurpose::Infrastructure),false),
   ColdJobAction::CloseActorOutput=>(OwnedSemioExport::CloseActorOutput,0,Some(GuestRelayGuestPurpose::Infrastructure),false),
   ColdJobAction::CloseActor=>(OwnedSemioExport::CloseActorContext,0,Some(GuestRelayGuestPurpose::Infrastructure),false),
   ColdJobAction::Start=>(OwnedSemioExport::StartJob,1,Some(GuestRelayGuestPurpose::Semantic),false),
   ColdJobAction::Step=>(OwnedSemioExport::StepJob,1,Some(GuestRelayGuestPurpose::Semantic),false),
   ColdJobAction::Read=>(OwnedSemioExport::ReadJobOutput,1,Some(GuestRelayGuestPurpose::Semantic),false),
   ColdJobAction::AcknowledgeOutput=>(OwnedSemioExport::AcknowledgeJobOutput,1,Some(GuestRelayGuestPurpose::Semantic),false),
   ColdJobAction::PublishFailure=>(OwnedSemioExport::PublishActorFailure,2,Some(GuestRelayGuestPurpose::Infrastructure),false),
   ColdJobAction::ReadFailure=>(OwnedSemioExport::ReadActorFailure,2,Some(GuestRelayGuestPurpose::Infrastructure),false),
   ColdJobAction::AcknowledgeFailure=>(OwnedSemioExport::AcknowledgeActorFailure,2,Some(GuestRelayGuestPurpose::Infrastructure),false),
   ColdJobAction::AcknowledgeChunk=>(OwnedSemioExport::AcknowledgeJobOutputChunk,3,Some(GuestRelayGuestPurpose::Semantic),false),
   ColdJobAction::AcknowledgeFailureChunk=>(OwnedSemioExport::AcknowledgeActorFailureChunk,4,Some(GuestRelayGuestPurpose::Infrastructure),false),
   ColdJobAction::BorrowChunk=>(OwnedSemioExport::BorrowJobOutput,5,None,true),
   ColdJobAction::BorrowFailureChunk=>(OwnedSemioExport::BorrowActorFailure,6,None,true),
   ColdJobAction::AdmitInput|ColdJobAction::Complete=>return Err(refused())};
  if let Some(reference)=reference{arguments[0]=OriginalCoreValue::I64(reference.failed_epoch as i64);arguments[1]=OriginalCoreValue::I64(reference.call_kind.tag()as i64);}else if prefix>0{if job==0{return Err(refused())}arguments[0]=OriginalCoreValue::I64(job as i64)}
  let base=usize::from(actor_failure);if prefix==3||prefix==4{arguments[base+1]=OriginalCoreValue::I64(i64::try_from(offset).map_err(|_|refused())?);arguments[base+2]=OriginalCoreValue::I64(i64::try_from(length).map_err(|_|refused())?);}
  if page{arguments[base+1]=OriginalCoreValue::I64(budget.input.operation as i64);arguments[base+2]=OriginalCoreValue::I64(budget.input.generation as i64);arguments[base+3]=OriginalCoreValue::I64(i64::try_from(offset).map_err(|_|refused())?);arguments[base+4]=OriginalCoreValue::I64(i64::try_from(length).map_err(|_|refused())?);return Ok(Self{export,arguments,length:prefix,purpose,page})}
  for(index,value)in budget.scalar_arguments().into_iter().enumerate(){arguments[prefix+index]=OriginalCoreValue::I64(value as i64)}Ok(Self{export,arguments,length:prefix+OriginalJobBudget::SCALAR_ARGUMENT_COUNT,purpose,page})
 }
 pub fn arguments(&self)->&[OriginalCoreValue]{&self.arguments[..self.length]}
}
pub(super) fn core_wire_header(action:&ColdJobAction)->Option<usize>{match action{ColdJobAction::AdmitActor|ColdJobAction::AdmitInput|ColdJobAction::CloseActorOutput|ColdJobAction::CloseActor=>Some(144),ColdJobAction::Start|ColdJobAction::Step|ColdJobAction::Read|ColdJobAction::AcknowledgeOutput|ColdJobAction::AcknowledgeChunk|ColdJobAction::CloseJobs|ColdJobAction::PublishFailure|ColdJobAction::ReadFailure|ColdJobAction::AcknowledgeFailure|ColdJobAction::AcknowledgeFailureChunk=>Some(184),ColdJobAction::BorrowChunk|ColdJobAction::BorrowFailureChunk|ColdJobAction::Complete=>None}}
/// 📏️ Uses the shared guest-publisher bound before any fixed-receipt Core crossing.
pub(super) fn require_core_wire(action:&ColdJobAction,budget:OriginalJobBudget)->Result<(),ValueError>{let Some(header)=core_wire_header(action)else{return Ok(())};let(items,demand,work)=semio_plugin_guest_contract::original_actor_context::original_core_first_response_reserve(header)?;for grant in[budget.input.grant,budget.retained_policy]{if grant.maximum_items<items||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"guest Core wire reserve exceeds its actual issued grant"))}}if budget.input.work.fuel<work||budget.fuel<work{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"guest Core wire reserve exceeds its actual issued work"))}Ok(())}
fn refused()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"cold job action has no valid scalar crossing")}

#[cfg(test)]
mod tests{
 use super::*;
 use semio_framework_actor::{OriginalActorTurnWork,RetainedTurnInput};
 use semio_framework_value::RetainedCloneGrant;
 #[test]
 fn core_wire_reserve_refuses_before_any_fixed_plan(){
  let grant=semio_framework_value::RetainedCloneGrant{maximum_items:100,maximum_copy_bytes:1048576,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:2};let input=semio_framework_actor::RetainedTurnInput{operation:1,generation:2,epoch:3,work:semio_framework_actor::OriginalActorTurnWork{clock_binding:4,absolute_deadline_us:100,fuel:10000},grant};let mut budget=OriginalJobBudget{input,retained_policy:grant,fuel:10000,deadline_us:90,cancelled:false};assert!(require_core_wire(&ColdJobAction::Start,budget).is_ok());let(_,demand,work)=semio_plugin_guest_contract::original_actor_context::original_core_first_response_reserve(184).unwrap();budget.retained_policy.maximum_copy_bytes=demand.copy_bytes-1;assert!(require_core_wire(&ColdJobAction::Start,budget).is_err());budget.retained_policy=grant;budget.fuel=work-1;assert!(require_core_wire(&ColdJobAction::Start,budget).is_err());assert!(require_core_wire(&ColdJobAction::BorrowChunk,budget).is_ok());
 }
 #[test]
 fn scalar_plans_preserve_neutral_abi_shapes_and_every_budget_field(){
  let law:serde_json::Value=serde_json::from_str(include_str!("plan.json")).unwrap();let grant=RetainedCloneGrant{maximum_items:600,maximum_copy_bytes:1048576,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:25};let budget=OriginalJobBudget{input:RetainedTurnInput{operation:11,generation:12,epoch:13,work:OriginalActorTurnWork{clock_binding:14,absolute_deadline_us:99,fuel:6000},grant},retained_policy:grant,fuel:5000,deadline_us:98,cancelled:false};
  for row in law["cases"].as_array().unwrap(){let action=match row["action"].as_str().unwrap(){"admit-actor"=>ColdJobAction::AdmitActor,"start"=>ColdJobAction::Start,"acknowledge-chunk"=>ColdJobAction::AcknowledgeChunk,"borrow-chunk"=>ColdJobAction::BorrowChunk,"close-actor"=>ColdJobAction::CloseActor,"close-jobs"=>ColdJobAction::CloseJobs,"close-actor-output"=>ColdJobAction::CloseActorOutput,"publish-failure"=>ColdJobAction::PublishFailure,"read-failure"=>ColdJobAction::ReadFailure,"acknowledge-failure"=>ColdJobAction::AcknowledgeFailure,"acknowledge-failure-chunk"=>ColdJobAction::AcknowledgeFailureChunk,"borrow-failure-chunk"=>ColdJobAction::BorrowFailureChunk,"read"=>ColdJobAction::Read,_=>panic!("unknown neutral action")};let plan=ColdJobCallPlan::new(action,law["job"].as_u64().unwrap(),Some(semio_framework_actor::ActorFailureCallReference{failed_epoch:law["failure"]["failedEpoch"].as_u64().unwrap(),call_kind:semio_framework_actor::ActorFailureCallKind::from_tag(law["failure"]["callKind"].as_u64().unwrap()).unwrap()}),law["offset"].as_u64().unwrap()as usize,law["length"].as_u64().unwrap()as usize,budget).unwrap();assert_eq!(format!("{:?}",plan.export),row["export"].as_str().unwrap());assert_eq!(plan.length,row["arguments"].as_u64().unwrap()as usize);assert_eq!(plan.purpose.map(|purpose|match purpose{GuestRelayGuestPurpose::Semantic=>"semantic",GuestRelayGuestPurpose::Infrastructure=>"infrastructure"}),row["purpose"].as_str());let prefix=row["prefix"].as_array().unwrap();for(index,value)in prefix.iter().enumerate(){assert_eq!(plan.arguments[index],OriginalCoreValue::I64(value.as_u64().unwrap()as i64));}if !plan.page{let prefix=plan.length-OriginalJobBudget::SCALAR_ARGUMENT_COUNT;for(index,value)in budget.scalar_arguments().into_iter().enumerate(){assert_eq!(plan.arguments[prefix+index],OriginalCoreValue::I64(value as i64));}}}
 }
}
