//! 🧪️ Normal job tests use original owned admissions and the actual mandatory caller context.
use super::*;
use semio_framework_job::{Generation,OperationId,StepBudget,StepContext,root_cancel_token};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,ValueError,ValueRefusalKind};

fn fixture_grant()->RetainedCloneGrant{let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🧩️extension/🧫️fixtures/📨️invoke/🔣️.json")).unwrap();let grant=&fixture["executionGrant"];RetainedCloneGrant{maximum_items:grant["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:grant["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:grant["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:grant["maximumReleaseBytes"].as_u64().unwrap()as usize,maximum_depth:grant["maximumDepth"].as_u64().unwrap()as usize}}
fn fixture_identity()->(OperationId,Generation){let value:serde_json::Value=serde_json::from_str(include_str!("../../../../🧩️extension/🧫️fixtures/📨️invoke/🔣️.json")).unwrap();(OperationId(value["caller"]["operation"].as_u64().unwrap()),Generation(value["caller"]["generation"].as_u64().unwrap()))}

pub(super) fn admit_fixture(source:&mut Option<OriginalJobAdmission>,grant:RetainedCloneGrant)->Result<bool,ValueError>{
 let(operation,generation)=fixture_identity();let cancel=root_cancel_token();let mut sequence=0;let mut receipt=Default::default();let mut cx=StepContext::new(operation,generation,StepBudget::new(1,u64::MAX,grant),cancel,||Some(1),&mut sequence,&mut receipt);
 let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||crate::__async::poll::resolve_ready(start_job(source,&mut cx)));
 assert_eq!((heap.requested_bytes,heap.released_bytes),(cx.retained_progress().retained_capacity_bytes,cx.retained_progress().released_bytes));result
}

pub(super) fn step_fixture(job:u64,fuel:u64)->Result<JobStep,ValueError>{
 let grant=fixture_grant();let(operation,generation)=fixture_identity();let cancel=root_cancel_token();let mut sequence=0;let mut receipt=Default::default();let mut cx=StepContext::new(operation,generation,StepBudget::new(fuel,u64::MAX,grant),cancel,||Some(1),&mut sequence,&mut receipt);
 let mut decode_callback=|_|true;let mut encode_callback=|_|true;let mut decode=semio_framework_value::NativeDecodeControl::new(grant.maximum_capacity_bytes,&mut decode_callback);let mut encode=semio_framework_value::NativeEncodeControl::new(grant.maximum_capacity_bytes,&mut encode_callback);let mut original=IoRunControl::new(&mut decode,&mut encode,grant);let mut snapshot_callback=|_|true;let mut snapshot=SqliteSnapshotControl::new(&mut snapshot_callback,Default::default());
 let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||crate::__async::poll::resolve_ready(step_job(job,JobBudget{fuel,deadline_ms:1},&mut original,&mut snapshot,&mut cx)));
 assert_eq!((heap.requested_bytes,heap.released_bytes),(cx.retained_progress().retained_capacity_bytes,cx.retained_progress().released_bytes),"normal step must report every real allocation and release");assert!(cx.retained_progress().fits(grant));result
}

pub(super) fn terminal_fixture(job:u64)->JobStep{for _ in 0..100000{match step_fixture(job,WORK_UNITS_EXECUTE).unwrap(){JobStep::Running(_)=>{},terminal=>return terminal}}panic!("original fixture exceeded its finite step bound")}
fn admit_body_fixture(job:u64){for _ in 0..10000{if JOBS.with(|slots|slots.borrow().iter().flatten().find(|slot|slot.job==job).is_some_and(|slot|matches!(slot.body,Some(JobBody::Bounded(_))))){return}assert!(matches!(step_fixture(job,1).unwrap(),JobStep::Running(None)))}panic!("original fixture factory admission exceeded finite turns")}
pub(super) fn source_fixture(job:u64,kind:&str,input:Vec<u8>,checkpoint:Option<Vec<u8>>)->Option<OriginalJobAdmission>{Some(OriginalJobAdmission{job,kind:kind.into(),input:Some(input),checkpoint})}

pub(super) fn start_fixture(job:u64,kind:&str,input:Vec<u8>,checkpoint:Option<Vec<u8>>){let mut source=source_fixture(job,kind,input,checkpoint);assert!(admit_fixture(&mut source,fixture_grant()).unwrap(),"original fixture admission refused")}
pub(super) fn slice_fixture(job:u64)->JobStep{for _ in 0..100000{match step_fixture(job,WORK_UNITS_EXECUTE).unwrap(){JobStep::Running(None)=>{},step=>return step}}panic!("original fixture exceeded its finite slice bound")}

struct FixedOriginalFixture{source:Option<(Vec<u8>,Option<Vec<u8>>)>,closing:Option<semio_framework_value::retirement::controlled::ControlledRetirement<(Vec<u8>,Option<Vec<u8>>)>>,tick:u8,restored:bool,never:bool}
impl BoundedJob for FixedOriginalFixture{
 fn step(&mut self,budget:JobBudget,_:&mut IoRunControl<'_,'_>,_:&mut SqliteSnapshotControl<'_>,cx:&mut StepContext<'_>)->Result<JobStep,ValueError>{
  if budget.fuel==0||self.never{return Ok(JobStep::Running(None))}let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(JobStep::Running(None))}
  if !self.restored{if let Some(bytes)=self.source.as_ref().and_then(|source|source.1.as_ref()){if !bytes.is_empty(){if grant.maximum_copy_bytes==0{return Ok(JobStep::Running(None))}self.tick=bytes[0];cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:1,..Default::default()})?;self.restored=true;return Ok(JobStep::Running(None))}}self.restored=true;cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;return Ok(JobStep::Running(None))}
  if grant.maximum_capacity_bytes<1||grant.maximum_copy_bytes<1{return Ok(JobStep::Running(None))}let mut bytes=Vec::with_capacity(1);self.tick+=1;bytes.push(self.tick);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:1,retained_capacity_bytes:bytes.capacity(),..Default::default()})?;Ok(if self.tick==1{JobStep::Running(Some(bytes))}else{JobStep::Done(bytes)})
 }
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,ValueError>{close_original(&mut self.source,&mut self.closing,cx)}
 fn retirement_demands(&self,copy:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{if let Some(owner)=&self.closing{return Ok(semio_framework_value::RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?,..Default::default()})}Ok(semio_framework_value::RetirementDemand{depth:1,..Default::default()})}
 fn cancel(&mut self){}
 fn checkpoint(&self)->Option<Vec<u8>>{Some(vec![self.tick])}
 fn terminal_drop_is_shallow(&self)->bool{self.source.is_none()&&self.closing.is_none()}
}
fn fixed_demands(_:u64,input:&Option<Vec<u8>>,restored:&Option<Vec<u8>>,_:&StepContext<'_>)->Result<RetainedCloneGrant,ValueError>{original_job_admission_demands::<FixedOriginalFixture>(input,restored)}
fn fixed_admit(job:u64,input:&mut Option<Vec<u8>>,restored:&mut Option<Vec<u8>>,cx:&mut StepContext<'_>)->Result<Option<Box<dyn BoundedJob>>,ValueError>{if job==9002{return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original fixture admission refused"))}admit_original_job(input,restored,cx,|input,checkpoint|FixedOriginalFixture{source:Some((input,checkpoint)),closing:None,tick:0,restored:false,never:job==9003})}
fn register_fixture(){assert!(register_bounded_job_kind("test.fixed-original",BoundedJobFactory{admit:fixed_admit,demands:fixed_demands}))}

/// 🧭️ Unknown identity produces an intact typed refusal without a synthetic payload.
#[test]
fn step_job_on_an_unknown_id_fails_without_panicking(){assert_eq!(step_fixture(999,WORK_UNITS_EXECUTE).unwrap_err().kind,ValueRefusalKind::InvalidValue)}

/// 📜️ Every declared normal builtin resolves to a real admission/demand pair.
#[test]
fn the_admitted_builtin_set_is_exactly_the_declared_builtin_set(){for kind in BUILTIN_JOB_KINDS{assert!(builtin_factory(kind).is_some())}assert!(builtin_factory("semio.not-a-kind").is_none());assert!(builtin_factory(semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND).is_some())}

/// 🤝️ Admission denial keeps the exact original raw/checkpoint pointers and registry slot free.
#[test]
fn original_factory_denial_keeps_whole_sources(){register_fixture();let mut source=source_fixture(9001,"test.fixed-original",b"fixed".to_vec(),Some(vec![1]));let pointer=source.as_ref().unwrap().input.as_ref().unwrap().as_ptr();assert!(!admit_fixture(&mut source,RetainedCloneGrant{maximum_items:0,..fixture_grant()}).unwrap());assert_eq!(source.as_ref().unwrap().input.as_ref().unwrap().as_ptr(),pointer);assert!(admit_fixture(&mut source,fixture_grant()).unwrap());assert!(source.is_none());crate::__async::poll::resolve_ready(cancel_job(9001));assert!(matches!(terminal_fixture(9001),JobStep::Failed(_)))}

/// 🎬️ One registered owner publishes paid progress and closes all source bodies before Done.
#[test]
fn a_registered_bounded_kind_advances_one_state_action(){register_fixture();let mut source=source_fixture(9001,"test.fixed-original",b"fixed".to_vec(),None);assert!(admit_fixture(&mut source,fixture_grant()).unwrap());let mut progress=false;for _ in 0..100000{match step_fixture(9001,WORK_UNITS_EXECUTE).unwrap(){JobStep::Running(Some(bytes))=>{assert_eq!(bytes,[1]);progress=true},JobStep::Running(None)=>{},JobStep::Done(bytes)=>{assert!(progress);assert_eq!(bytes,[2]);assert_eq!(step_fixture(9001,1).unwrap_err().kind,ValueRefusalKind::InvalidValue);return},JobStep::Failed(_)=>panic!("registered original fixture failed")}}panic!("registered original fixture did not complete")}

/// 📨️ A refused factory keeps its original input through paid typed failure closure.
#[test]
fn a_registered_bounded_kind_retains_its_admission_fault(){register_fixture();let mut source=source_fixture(9002,"test.fixed-original",b"refused".to_vec(),Some(vec![3]));assert!(admit_fixture(&mut source,fixture_grant()).unwrap());assert!(matches!(terminal_fixture(9002),JobStep::Failed(_)));assert_eq!(step_fixture(9002,1).unwrap_err().kind,ValueRefusalKind::InvalidValue)}

/// ⏳️ External pending progress remains a live original owner until explicit cancellation.
#[test]
fn repeated_pending_turns_preserve_original_owner(){register_fixture();let mut source=source_fixture(9003,"test.fixed-original",b"pending".to_vec(),None);assert!(admit_fixture(&mut source,fixture_grant()).unwrap());for _ in 0..128{assert!(matches!(step_fixture(9003,WORK_UNITS_EXECUTE).unwrap(),JobStep::Running(None)))}crate::__async::poll::resolve_ready(cancel_job(9003));assert!(matches!(terminal_fixture(9003),JobStep::Failed(_)))}

/// 🛑️ Cancellation signals the same deep boxed owner and waits for its actual close.
#[test]
fn original_cancelled_router_retains_deep_owner_until_its_exact_close_turns(){register_fixture();let mut source=source_fixture(9001,"test.fixed-original",vec![7;1799],None);assert!(admit_fixture(&mut source,fixture_grant()).unwrap());admit_body_fixture(9001);let pointer=JOBS.with(|slots|{let slots=slots.borrow();let slot=slots.iter().flatten().find(|slot|slot.job==9001).unwrap();let Some(JobBody::Bounded(owner))=slot.body.as_ref()else{panic!("original body")};owner.as_ref()as *const dyn BoundedJob as *const()});crate::__async::poll::resolve_ready(cancel_job(9001));JOBS.with(|slots|{let slots=slots.borrow();let slot=slots.iter().flatten().find(|slot|slot.job==9001).unwrap();let Some(JobBody::Bounded(owner))=slot.body.as_ref()else{panic!("same original body")};assert_eq!(owner.as_ref()as *const dyn BoundedJob as *const(),pointer)});assert!(matches!(terminal_fixture(9001),JobStep::Failed(_)))}

/// 📸️ Checkpoint publication must keep the original replay input after real body admission.
#[test]
fn original_checkpoint_keeps_replay_input(){register_fixture();let mut source=source_fixture(9001,"test.fixed-original",b"fixed".to_vec(),None);assert!(admit_fixture(&mut source,fixture_grant()).unwrap());admit_body_fixture(9001);let entries=crate::__async::poll::resolve_ready(checkpoint_jobs());let replay=entries.iter().find(|entry|entry.job==9001).unwrap();assert_eq!(replay.input.as_deref(),Some(b"fixed".as_slice()),"checkpoint must capture the same funded original replay input");crate::__async::poll::resolve_ready(cancel_job(9001));assert!(matches!(terminal_fixture(9001),JobStep::Failed(_)))}

/// 🧮️ Builtin semantic decode/execute may not hide allocation behind a fuel-only phase price.
#[test]
fn a_builtin_walks_decode_then_execute_to_a_terminal_outcome(){let payload=semio_framework_pack_json::to_json_string(&semio_framework::io_schema::IoPayload::Text("hello".into()));let input=format!("{{\"source\":\"s.stdio.binary@raw/*\",\"target\":\"s.jobtest.walk@1/*\",\"payload\":{payload}}}").into_bytes();let mut source=source_fixture(50,JOB_KIND_IO_SNIFF,input,None);assert!(admit_fixture(&mut source,fixture_grant()).unwrap());assert!(matches!(terminal_fixture(50),JobStep::Done(_)))}
