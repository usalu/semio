//! 🧪️ Normal job tests use original owned admissions and the actual mandatory caller context.
use crate::{BUILTIN_JOB_KINDS, BoundedJob, BoundedJobFactory, JOBS, JOB_KIND_IO_SNIFF, JobBody, WORK_UNITS_EXECUTE, admit_original_job, abi, builtin_factory, checkpoint_jobs, close_original, has_owned_jobs, original_job_admission_demands, register_bounded_job_kind};
use semio_framework_value_derive::{FromValue, ToValue};
use std::future::Future;
use semio_os_kernel::io::io_mechanism::IoRunControl;
use semio_framework_actor::OriginalJobStep as JobStep;
use semio_framework::kernel::OriginalJobSource as OriginalJobAdmission;
use semio_framework_job::OriginalTurnFailure;
use semio_os_kernel::sqlite_snapshot::SqliteSnapshotControl;
use semio_framework_job::{Generation, OperationId, StepContext};
use semio_plugin_testing_laws::OriginalWorkFixture;
use crate::registry_driver::{RegistryJobDriver,source_grant};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, ValueError, ValueRefusalKind};

fn fixture_grant()->RetainedCloneGrant{let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🧩️extension/🧪️tests/🧫️fixtures/📨️invoke/🔣️.json")).unwrap();let grant=&fixture["executionGrant"];RetainedCloneGrant{maximum_items:grant["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:grant["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:grant["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:grant["maximumReleaseBytes"].as_u64().unwrap()as usize,maximum_depth:grant["maximumDepth"].as_u64().unwrap()as usize}}
#[doc(hidden)]
pub fn fixture_identity()->(OperationId,Generation){let value:serde_json::Value=serde_json::from_str(include_str!("../../../../🧩️extension/🧪️tests/🧫️fixtures/📨️invoke/🔣️.json")).unwrap();(OperationId(value["caller"]["operation"].as_u64().unwrap()),Generation(value["caller"]["generation"].as_u64().unwrap()))}

fn value_kind(failure:&OriginalTurnFailure)->ValueRefusalKind{match &failure.primary{semio_framework_job::OriginalTurnPrimary::Value(error)=>error.kind,_=>panic!("original refusal is a value refusal")}}
/// 💤️ Materialises the lazily allocated registry slot table, whose one-time allocation is cold setup and never part of a measured admission.
pub(crate) fn warm_registry(){JOBS.with(|slots|assert_eq!(slots.borrow().len(),crate::JOB_SLOTS))}
/// 🚨️ The code and message of a failure reply, read from the canonical wire value the reply publishes.
pub(crate) struct FaultView{pub code:String,pub message:String}
pub(crate) fn fault_view(bytes:&[u8])->FaultView{let decoded:serde_json::Value=semio_os_kernel::os_pack::pack_rt::decode_wire_value(bytes).unwrap().into();FaultView{code:decoded["code"].as_str().expect("failure reply code").to_owned(),message:decoded["message"].as_str().expect("failure reply message").to_owned()}}
/// ⛽️ Fuel issued to one reply step: a pack step performs its original effect work and debits its receipt on top, so a step costs a few units and the loan guard refuses any step that spends more than it was issued.
const REPLY_STEP_FUEL:u64=8;
pub(crate) fn delta(after:RetainedCloneProgress,before:RetainedCloneProgress)->RetainedCloneProgress{RetainedCloneProgress{copied_items:after.copied_items-before.copied_items,copied_bytes:after.copied_bytes-before.copied_bytes,retained_capacity_bytes:after.retained_capacity_bytes-before.retained_capacity_bytes,released_bytes:after.released_bytes-before.released_bytes}}
pub(crate) fn owner_for(operation:OperationId,generation:Generation)->OriginalWorkFixture{OriginalWorkFixture::with_source(operation,generation,OriginalWorkFixture::stock(),||Some(1),u64::MAX,1<<30)}
fn work_caller()->OriginalWorkFixture{owner_for(OperationId(991),Generation(17))}
pub(crate) fn loan<R>(owner:&mut OriginalWorkFixture,grant:RetainedCloneGrant,fuel:u64,deadline:u64,run:impl FnOnce(&mut StepContext<'_>)->Result<R,OriginalTurnFailure>)->Result<R,OriginalTurnFailure>{owner.turn(grant,|cx|{let mut value=None;cx.with_original_work_loan(fuel,deadline,|cx|{value=Some(run(cx)?);Ok(())})?;Ok(value.expect("original loan returned its value"))})}
/// 🎟️ One original caller per job: the registry's own admission, step and cancel entry points run funded turns of the same caller that later closes them.
#[derive(Default)]
pub(crate) struct Fixture{drivers:std::collections::BTreeMap<u64,RegistryJobDriver>}
impl Fixture{
 pub(super) fn driver(&mut self,job:u64)->&mut RegistryJobDriver{self.drivers.entry(job).or_insert_with(||RegistryJobDriver::new(job,fixture_grant()))}
 pub(super) fn admit(&mut self,source:&mut Option<OriginalJobAdmission>,grant:RetainedCloneGrant)->Result<bool,ValueError>{let job=source.as_ref().expect("original fixture admission source").job;self.driver(job).start_source(source,grant).map_err(|failure|match failure.primary{semio_framework_job::OriginalTurnPrimary::Value(error)=>error,_=>panic!("original admission refusal is a value refusal")})}
 pub(super) fn step(&mut self,job:u64,fuel:u64)->Result<JobStep,OriginalTurnFailure>{self.driver(job).step_with_fuel(fuel)}
 pub(super) fn cancel(&mut self,job:u64){let(signalled,heap,progress)=self.driver(job).turn(fixture_grant(),1,|cx|{let before=cx.retained_progress();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||abi::cancel(job,cx));Ok((result?,heap,delta(cx.retained_progress(),before)))}).unwrap();assert!(signalled);assert_eq!(progress,RetainedCloneProgress{copied_items:1,..Default::default()});assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
 pub(super) fn receipt(&mut self,job:u64)->RetainedCloneProgress{self.driver(job).receipt()}
 pub(super) fn terminal(&mut self,job:u64)->JobStep{for _ in 0..100000{match self.step(job,WORK_UNITS_EXECUTE).unwrap(){JobStep::Running(_)=>{},terminal=>return terminal}}panic!("original fixture exceeded its finite step bound")}
 fn admit_body(&mut self,job:u64){for _ in 0..10000{if JOBS.with(|slots|slots.borrow().iter().flatten().find(|slot|slot.job==job).is_some_and(|slot|matches!(slot.body,Some(JobBody::Bounded(_))))){return}assert!(matches!(self.step(job,1).unwrap(),JobStep::Running(None)))}panic!("original fixture factory admission exceeded finite turns")}
 pub(super) fn start(&mut self,job:u64,kind:&str,input:Vec<u8>,checkpoint:Option<Vec<u8>>){let mut source=source_fixture(job,kind,input,checkpoint);assert!(self.admit(&mut source,fixture_grant()).unwrap(),"original fixture admission refused")}
 pub(super) fn slice(&mut self,job:u64)->JobStep{for _ in 0..100000{match self.step(job,WORK_UNITS_EXECUTE).unwrap(){JobStep::Running(None)=>{},step=>return step}}panic!("original fixture exceeded its finite slice bound")}
}
pub(super) fn source_fixture(job:u64,kind:&str,input:Vec<u8>,checkpoint:Option<Vec<u8>>)->Option<OriginalJobAdmission>{Some(OriginalJobAdmission{job,kind:kind.into(),input:Some(input),checkpoint})}

struct FixedOriginalFixture{pub source:Option<(Vec<u8>,Option<Vec<u8>>)>,pub closing:Option<semio_framework_value::retirement::controlled::ControlledRetirement<(Vec<u8>,Option<Vec<u8>>)>>,pub tick:u8,pub restored:bool,pub never:bool,pub failed:bool}
impl BoundedJob for FixedOriginalFixture{
 fn step(&mut self,_:&mut IoRunControl<'_,'_>,_:&mut SqliteSnapshotControl<'_>,cx:&mut StepContext<'_>)->Result<JobStep,OriginalTurnFailure>{
  if cx.should_yield()||self.never{return Ok(JobStep::Running(None))}let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(JobStep::Running(None))}
  if self.failed{cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original fixture body failed").into())}
  if !self.restored{if let Some(bytes)=self.source.as_ref().and_then(|source|source.1.as_ref()){if !bytes.is_empty(){if grant.maximum_copy_bytes==0{return Ok(JobStep::Running(None))}self.tick=bytes[0];cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:1,..Default::default()})?;self.restored=true;return Ok(JobStep::Running(None))}}self.restored=true;cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;return Ok(JobStep::Running(None))}
  if grant.maximum_capacity_bytes<1||grant.maximum_copy_bytes<1{return Ok(JobStep::Running(None))}let mut bytes=Vec::with_capacity(1);self.tick+=1;bytes.push(self.tick);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:1,retained_capacity_bytes:bytes.capacity(),..Default::default()})?;Ok(if self.tick==1{JobStep::Running(Some(bytes))}else{JobStep::Done(bytes)})
 }
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if self.failed&&!self.never{return Ok(false)}close_original(&mut self.source,&mut self.closing,cx)}
 fn retirement_demands(&self,copy:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{if let Some(owner)=&self.closing{return Ok(owner.selected_demand(copy)?)}if let Some(source)=&self.source{return Ok(semio_framework_value::retirement::retirement_cursor_birth_demand(source)?)}Ok(semio_framework_value::RetirementDemand{depth:1,..Default::default()})}
 fn cancel(&mut self){self.never=true}
 fn checkpoint(&self)->Option<Vec<u8>>{Some(vec![self.tick])}
 fn replay_input(&self)->Option<&[u8]>{self.source.as_ref().map(|source|source.0.as_slice())}
 fn terminal_drop_is_shallow(&self)->bool{self.source.is_none()&&self.closing.is_none()}
}
fn fixed_demands(_:u64,input:&Option<Vec<u8>>,restored:&Option<Vec<u8>>,_:&StepContext<'_>)->Result<RetainedCloneGrant,ValueError>{original_job_admission_demands::<FixedOriginalFixture>(input,restored)}
fn fixed_admit(job:u64,input:&mut Option<Vec<u8>>,restored:&mut Option<Vec<u8>>,cx:&mut StepContext<'_>)->Result<Option<Box<dyn BoundedJob>>,ValueError>{if job==9002{return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original fixture admission refused"))}admit_original_job(input,restored,cx,|input,checkpoint|FixedOriginalFixture{source:Some((input,checkpoint)),closing:None,tick:0,restored:false,never:job==9003,failed:job==9004})}
fn register_fixture(){assert!(register_bounded_job_kind("test.fixed-original",BoundedJobFactory{admit:fixed_admit,demands:fixed_demands}))}

/// 🧭️ Unknown identity produces an intact typed refusal without a synthetic payload.
#[test]
fn step_job_on_an_unknown_id_fails_without_panicking(){let mut fx=Fixture::default();assert_eq!(value_kind(&fx.step(999,WORK_UNITS_EXECUTE).unwrap_err()),ValueRefusalKind::InvalidValue)}

/// 📜️ Every declared normal builtin resolves to a real admission/demand pair.
#[test]
fn the_admitted_builtin_set_is_exactly_the_declared_builtin_set(){for kind in BUILTIN_JOB_KINDS{assert!(builtin_factory(kind).is_some())}assert!(builtin_factory("semio.not-a-kind").is_none());assert!(builtin_factory(semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND).is_some())}

/// 🤝️ Admission denial keeps the exact original raw/checkpoint pointers and registry slot free.
#[test]
fn original_factory_denial_keeps_whole_sources(){let mut fx=Fixture::default();register_fixture();let mut source=source_fixture(9001,"test.fixed-original",b"fixed".to_vec(),Some(vec![1]));let pointer=source.as_ref().unwrap().input.as_ref().unwrap().as_ptr();assert!(!fx.admit(&mut source,RetainedCloneGrant{maximum_items:0,..fixture_grant()}).unwrap());assert_eq!(source.as_ref().unwrap().input.as_ref().unwrap().as_ptr(),pointer);assert!(fx.admit(&mut source,fixture_grant()).unwrap());assert!(source.is_none());fx.cancel(9001);assert!(matches!(fx.terminal(9001),JobStep::Failed(_)))}

/// 🎬️ One registered owner publishes paid progress and closes all source bodies before Done.
#[test]
fn a_registered_bounded_kind_advances_one_state_action(){let mut fx=Fixture::default();register_fixture();let mut source=source_fixture(9001,"test.fixed-original",b"fixed".to_vec(),None);assert!(fx.admit(&mut source,fixture_grant()).unwrap());let mut progress=false;for _ in 0..100000{match fx.step(9001,WORK_UNITS_EXECUTE).unwrap(){JobStep::Running(Some(bytes))=>{assert_eq!(bytes,[1]);progress=true},JobStep::Running(None)=>{},JobStep::Done(bytes)=>{assert!(progress);assert_eq!(bytes,[2]);assert_eq!(value_kind(&fx.step(9001,1).unwrap_err()),ValueRefusalKind::InvalidValue);return},JobStep::Failed(_)=>panic!("registered original fixture failed")}}panic!("registered original fixture did not complete")}

/// 📨️ A refused factory keeps its original input through paid typed failure closure.
#[test]
fn a_registered_bounded_kind_retains_its_admission_fault(){let mut fx=Fixture::default();register_fixture();let mut source=source_fixture(9002,"test.fixed-original",b"refused".to_vec(),Some(vec![3]));assert!(fx.admit(&mut source,fixture_grant()).unwrap());assert!(matches!(fx.terminal(9002),JobStep::Failed(_)));assert_eq!(value_kind(&fx.step(9002,1).unwrap_err()),ValueRefusalKind::InvalidValue)}

/// ⏳️ External pending progress remains a live original owner until explicit cancellation.
#[test]
fn repeated_pending_turns_preserve_original_owner(){let mut fx=Fixture::default();register_fixture();let mut source=source_fixture(9003,"test.fixed-original",b"pending".to_vec(),None);assert!(fx.admit(&mut source,fixture_grant()).unwrap());for _ in 0..128{assert!(matches!(fx.step(9003,WORK_UNITS_EXECUTE).unwrap(),JobStep::Running(None)))}fx.cancel(9003);assert!(matches!(fx.terminal(9003),JobStep::Failed(_)))}

/// 🛑️ Cancellation signals the same deep boxed owner and waits for its actual close.
#[test]
fn original_cancelled_router_retains_deep_owner_until_its_exact_close_turns(){let mut fx=Fixture::default();register_fixture();let mut source=source_fixture(9001,"test.fixed-original",vec![7;1799],None);assert!(fx.admit(&mut source,fixture_grant()).unwrap());fx.admit_body(9001);let pointer=JOBS.with(|slots|{let slots=slots.borrow();let slot=slots.iter().flatten().find(|slot|slot.job==9001).unwrap();let Some(JobBody::Bounded(owner))=slot.body.as_ref()else{panic!("original body")};owner.as_ref()as *const dyn BoundedJob as *const()});fx.cancel(9001);JOBS.with(|slots|{let slots=slots.borrow();let slot=slots.iter().flatten().find(|slot|slot.job==9001).unwrap();let Some(JobBody::Bounded(owner))=slot.body.as_ref()else{panic!("same original body")};assert_eq!(owner.as_ref()as *const dyn BoundedJob as *const(),pointer)});assert!(matches!(fx.terminal(9001),JobStep::Failed(_)))}

/// 🧯️ A normal body refusal holds its exact boxed owner until complete caller-paid cleanup.
#[test]
fn original_job_body_failure_retains_exact_owner_and_error_until_paid_close(){
let mut fx=Fixture::default(); let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🧩️extension/🧪️tests/🧫️fixtures/📨️invoke/🔣️.json")).unwrap();let failure=&fixture["bodyFailure"];let job=failure["job"].as_u64().unwrap();let(operation,generation)=(OperationId(job),Generation(1));register_fixture();let mut source=source_fixture(job,failure["kind"].as_str().unwrap(),fixture["text"].as_str().unwrap().repeat(fixture["repeat"].as_u64().unwrap()as usize).into_bytes(),Some(vec![7;257]));assert!(fx.admit(&mut source,fixture_grant()).unwrap());fx.admit_body(job);
 let pointer=JOBS.with(|slots|{let slots=slots.borrow();let slot=slots.iter().flatten().find(|slot|slot.job==job).unwrap();let Some(JobBody::Bounded(owner))=slot.body.as_ref()else{panic!("original admitted body")};owner.as_ref()as *const dyn BoundedJob as *const()});
 let(owned,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||has_owned_jobs(operation.0,generation.0));assert!(owned);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(!has_owned_jobs(operation.0+1,generation.0));assert!(!has_owned_jobs(operation.0,generation.0+1));
 assert!(matches!(fx.step(job,1).unwrap(),JobStep::Running(None)));
 JOBS.with(|slots|{let slots=slots.borrow();let slot=slots.iter().flatten().find(|slot|slot.job==job).unwrap();assert_eq!(slot.failed_owner.as_ref().unwrap().as_ref()as *const dyn BoundedJob as *const(),pointer);assert!(!slot.failed_owner.as_ref().unwrap().terminal_drop_is_shallow());let Some(JobBody::Refused(Some(semio_plugin_extension::extension_invocation_failure::ExtensionInvocationCause::Turn(turn)),None))=slot.body.as_ref()else{panic!("original whole body refusal")};let semio_framework_job::OriginalTurnPrimary::Value(error)=&turn.primary else{panic!("original body refusal is a value refusal")};assert_eq!(error.kind.as_str(),failure["code"].as_str().unwrap());assert_eq!(error.message,failure["message"].as_str().unwrap());assert!(slot.source.is_some());assert!(slot.outcome.is_none());});
 let mut bytes=None;for _ in 0..100000{assert!(has_owned_jobs(operation.0,generation.0));match fx.step(job,WORK_UNITS_EXECUTE).unwrap(){JobStep::Running(None)=>{},JobStep::Failed(output)=>{bytes=Some(output);break},_=>panic!("original body refusal published another outcome")}}
 assert!(!has_owned_jobs(operation.0,generation.0));let output=bytes.as_ref().expect("original body failure reaches publication");let decoded:serde_json::Value=semio_os_kernel::os_pack::pack_rt::decode_wire_value(output).unwrap().into();assert_eq!(decoded["code"],failure["code"]);assert_eq!(decoded["message"],failure["message"]);
 let mut closing=None;for _ in 0..10000{let turn=source_grant(fixture_grant(),&bytes,&closing);let done=fx.driver(job).turn(turn,1,|cx|{let before=cx.retained_progress();let(done,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_original_retirement(|cx|close_original(&mut bytes,&mut closing,cx)));let progress=delta(cx.retained_progress(),before);assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));Ok(done?)}).unwrap();if done{break}}assert!(bytes.is_none()&&closing.is_none());let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop((bytes,closing)));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
}

/// 🧭️ Cancellation after a real body refusal signals that original failed producer before close.
#[test]
fn original_job_body_failure_cancellation_signals_same_failed_owner(){
let mut fx=Fixture::default(); register_fixture();fx.start(9004,"test.fixed-original",vec![7;1799],None);fx.admit_body(9004);assert!(matches!(fx.step(9004,1).unwrap(),JobStep::Running(None)));let pointer=JOBS.with(|slots|slots.borrow().iter().flatten().find(|slot|slot.job==9004).unwrap().failed_owner.as_ref().unwrap().as_ref()as *const dyn BoundedJob as *const());fx.cancel(9004);assert!(matches!(fx.step(9004,1).unwrap(),JobStep::Running(None)));JOBS.with(|slots|{let slots=slots.borrow();let slot=slots.iter().flatten().find(|slot|slot.job==9004).unwrap();assert!(slot.cancellation_signalled);assert_eq!(slot.failed_owner.as_ref().unwrap().as_ref()as *const dyn BoundedJob as *const(),pointer)});assert!(matches!(fx.terminal(9004),JobStep::Failed(_)));
}

/// 📏️ Original source retirement honors the caller's single CPU unit and complete retained wallet.
#[test]
fn original_job_source_close_uses_exact_one_unit_caller_work(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../../🧩️extension/🧪️tests/🧫️fixtures/📨️invoke/🔣️.json")).unwrap();let work=&corpus["jobCloseWork"];
 for copy in work["copyGrants"].as_array().unwrap(){let grant=RetainedCloneGrant{maximum_copy_bytes:copy.as_u64().unwrap()as usize,..fixture_grant()};let mut owner=work_caller();let mut source=Some(vec![7u8;work["sourceBytes"].as_u64().unwrap()as usize]);let pointer=source.as_ref().unwrap().as_ptr();let mut closing=None;
  for(fuel,deadline)in [(0,2)]{let(done,progress,left,heap)=loan(&mut owner,grant,fuel,deadline,|cx|{let before=cx.retained_progress();let(done,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_original_retirement(|cx|close_original(&mut source,&mut closing,cx)));Ok((done?,delta(cx.retained_progress(),before),cx.fuel_remaining(),heap))}).unwrap();assert!(!done);assert_eq!(progress,RetainedCloneProgress::default());assert_eq!(left,fuel);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(source.as_ref().unwrap().as_ptr(),pointer);assert!(closing.is_none());}
  let mut terminal=false;for _ in 0..100000{let fuel=work["fuel"].as_u64().unwrap();let turn=source_grant(grant,&source,&closing);let(done,progress,left,heap)=loan(&mut owner,turn,fuel,1,|cx|{let before=cx.retained_progress();let(done,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_original_retirement(|cx|close_original(&mut source,&mut closing,cx)));Ok((done?,delta(cx.retained_progress(),before),cx.fuel_remaining(),heap))}).unwrap();assert!(progress.copied_items<=1);if progress!=RetainedCloneProgress::default(){assert_eq!(left,0)}assert!(progress.fits(turn));assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));if done{terminal=true;break}}
  assert!(terminal&&source.is_none()&&closing.is_none());let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop((source,closing)));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));owner.finish();
 }
}

/// 🧮️ The normal typed fault encoder and every child close use original caller CPU authority.
#[test]
fn original_job_failure_projection_uses_exact_one_unit_caller_work(){
 use semio_plugin_extension::extension_invocation_failure::{ExtensionInvocationCause,RetainedExtensionFaultReply};
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../../🧩️extension/🧪️tests/🧫️fixtures/📨️invoke/🔣️.json")).unwrap();let work=&corpus["jobCloseWork"];
 for copy in work["copyGrants"].as_array().unwrap(){let grant=RetainedCloneGrant{maximum_copy_bytes:copy.as_u64().unwrap()as usize,..fixture_grant()};let mut owner=work_caller();let mut reply=RetainedExtensionFaultReply::new(ExtensionInvocationCause::Value(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original fixture body failed").into()));let mut bytes=None;
  for(fuel,deadline)in [(0,2),(1,1)]{let(output,progress,left,heap)=loan(&mut owner,grant,fuel,deadline,|cx|{let before=cx.retained_progress();let(output,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||reply.advance(cx));Ok((output?,delta(cx.retained_progress(),before),cx.fuel_remaining(),heap))}).unwrap();assert!(output.is_none());assert_eq!(progress,RetainedCloneProgress::default());assert_eq!(left,fuel);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
  for _ in 0..100000{let fuel=REPLY_STEP_FUEL;let demand=reply.demands(copy.as_u64().unwrap()as usize).unwrap();let turn=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes.max(demand.copy_bytes),maximum_capacity_bytes:grant.maximum_capacity_bytes.max(demand.capacity_bytes),maximum_release_bytes:grant.maximum_release_bytes.max(demand.release_bytes),..grant};let(output,progress,left,heap)=loan(&mut owner,turn,fuel,2,|cx|{let before=cx.retained_progress();let(output,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||reply.advance(cx));Ok((output?,delta(cx.retained_progress(),before),cx.fuel_remaining(),heap))}).unwrap();assert!(progress.copied_items<=1);if progress!=RetainedCloneProgress::default(){assert!(left<fuel)}else{assert_eq!(left,fuel)}assert!(progress.fits(turn));assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));if let Some(output)=output{bytes=Some(output);break}}
  assert!(reply.terminal_is_empty());let decoded:serde_json::Value=semio_os_kernel::os_pack::pack_rt::decode_wire_value(bytes.as_ref().expect("paid original fault publication")).unwrap().into();assert_eq!(decoded["code"],corpus["bodyFailure"]["code"]);assert_eq!(decoded["message"],corpus["bodyFailure"]["message"]);
  let mut closing=None;for _ in 0..100000{let turn=source_grant(grant,&bytes,&closing);let(done,progress,heap)=loan(&mut owner,turn,1,2,|cx|{let before=cx.retained_progress();let(done,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_original_retirement(|cx|close_original(&mut bytes,&mut closing,cx)));Ok((done?,delta(cx.retained_progress(),before),heap))}).unwrap();assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));if done{break}}assert!(bytes.is_none()&&closing.is_none());let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop((reply,bytes,closing)));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));owner.finish();
 }
}

/// 🧳️ Checkpoint publication retains its original replay input after actual body admission.
#[test]
fn original_checkpoint_keeps_replay_input(){let mut fx=Fixture::default();register_fixture();let mut source=source_fixture(9001,"test.fixed-original",b"fixed".to_vec(),None);assert!(fx.admit(&mut source,fixture_grant()).unwrap());let entries=semio_framework_async::poll::resolve_ready(checkpoint_jobs());let replay=entries.iter().find(|entry|entry.job==9001).unwrap();assert_eq!(replay.input.as_deref(),Some(b"fixed".as_slice()),"checkpoint must capture the same funded original replay input");assert_eq!(replay.checkpoint,None);fx.admit_body(9001);let entries=semio_framework_async::poll::resolve_ready(checkpoint_jobs());let replay=entries.iter().find(|entry|entry.job==9001).unwrap();assert_eq!(replay.input.as_deref(),Some(b"fixed".as_slice()),"the admitted body keeps the replay input the checkpoint needs");assert_eq!(replay.checkpoint.as_deref(),Some([0u8].as_slice()),"the admitted body publishes its own checkpoint");fx.cancel(9001);assert!(matches!(fx.terminal(9001),JobStep::Failed(_)))}

/// 🧮️ Builtin semantic decode/execute may not hide allocation behind a fuel-only phase price.
#[test]
fn a_builtin_walks_decode_then_execute_to_a_terminal_outcome(){let mut fx=Fixture::default();let payload=semio_framework_pack_json::to_json_string(&semio_framework::io_schema::IoPayload::Text("hello".into()));let input=format!("{{\"source\":\"framework.io.binary@raw/*\",\"target\":\"s.jobtest.walk@1/*\",\"payload\":{payload}}}").into_bytes();let mut source=source_fixture(50,JOB_KIND_IO_SNIFF,input,None);assert!(fx.admit(&mut source,fixture_grant()).unwrap());assert!(matches!(fx.terminal(50),JobStep::Done(_)))}

/// 🧷️ The registered driver funds the original source close beyond its narrow execution budget.
#[test]
fn fixture_registry_driver_retains_its_original_source_through_terminal_publication() {
    use crate::registry_driver::RegistryJobDriver;
    use semio_framework_value::retirement::retirement_cursor_birth_demand;

    register_fixture();
    let scenario: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🎛️registry-driver/🔣️.json")).unwrap();
    let job = scenario["job"].as_u64().unwrap();
    let kind = scenario["kind"].as_str().unwrap();
    let input = scenario["input"].as_str().unwrap().as_bytes();
    let expected: Vec<u8> = serde_json::from_value(scenario["output"].clone()).unwrap();
    let original = (input.to_vec(), None::<Vec<u8>>);
    let close_copy = retirement_cursor_birth_demand(&original).unwrap().copy_bytes;
    let policy = fixture_grant();
    let grant = RetainedCloneGrant {
        maximum_copy_bytes: policy.maximum_copy_bytes.min(close_copy - 1),
        ..policy
    };

    let mut driver = RegistryJobDriver::new(job, grant);
    driver.start(kind, original.0, original.1).unwrap();
    let original_kind = || JOBS.with(|jobs| {
        let jobs = jobs.borrow();
        jobs.iter().flatten().find(|slot| slot.job == job)
            .and_then(|slot| slot.source.as_ref())
            .map(|source| source.kind.as_ptr())
    });
    let pointer = original_kind().unwrap();
    let mut progress_pages = 0;
    let mut observed = Vec::new();
    for _ in 0..100_000 {
        match driver.step().unwrap() {
            JobStep::Running(Some(page)) => {
                assert_eq!(original_kind(), Some(pointer));
                progress_pages += 1;
                observed.extend(page);
            }
            JobStep::Running(None) => {}
            JobStep::Done(page) => {
                observed.extend(page);
                assert_eq!(progress_pages, 1);
                assert_eq!(observed, expected);
                assert!(!has_owned_jobs(job, 1));
                break;
            }
            JobStep::Failed(error) => panic!("registry fixture failed: {}", String::from_utf8_lossy(&error)),
        }
    }
    assert!(!has_owned_jobs(job, 1), "registry fixture did not publish its terminal result");
    drop(driver);

    let mut abandoned = RegistryJobDriver::new(job, grant);
    abandoned.start(kind, input.to_vec(), None).unwrap();
    for _ in 0..100_000 {
        match abandoned.step().unwrap() {
            JobStep::Running(Some(_)) => {
                drop(abandoned);
                assert!(!has_owned_jobs(job, 1));
                return;
            }
            JobStep::Running(None) => {}
            _ => panic!("registry fixture completed before its progress page"),
        }
    }
    panic!("registry fixture did not publish progress before caller close");
}

/// 📸️ Restoring from the checkpoint entry of a job taken mid-run, then stepping to the end, yields the same output and the same final receipt as the uninterrupted run.
#[test]
fn restoring_a_mid_run_checkpoint_matches_the_uninterrupted_run_in_output_and_receipts(){
 let mut fx=Fixture::default();register_fixture();
 let run=|fx:&mut Fixture,job:u64|{let mut pages=Vec::new();loop{let before=fx.receipt(job);match fx.step(job,WORK_UNITS_EXECUTE).unwrap(){JobStep::Running(None)=>{},JobStep::Running(Some(page))=>pages.push(page),JobStep::Done(bytes)=>{let spent=delta(fx.receipt(job),before);return(pages,bytes,spent)},JobStep::Failed(_)=>panic!("the fixture job failed")}}};
 fx.start(9101,"test.fixed-original",b"fixed".to_vec(),None);
 let(pages,done,spent)=run(&mut fx,9101);
 fx.start(9102,"test.fixed-original",b"fixed".to_vec(),None);
 let mut seen=Vec::new();let entry=loop{match fx.step(9102,WORK_UNITS_EXECUTE).unwrap(){JobStep::Running(Some(page))=>{seen.push(page);break semio_framework_async::poll::resolve_ready(checkpoint_jobs()).into_iter().find(|entry|entry.job==9102).expect("a running job is checkpointed")},JobStep::Running(None)=>{},_=>panic!("the job ends before its progress page")}};
 assert_eq!(seen,pages);assert_eq!(entry.input.as_deref(),Some(b"fixed".as_slice()));assert_eq!(entry.checkpoint.as_deref(),Some([1u8].as_slice()));
 fx.cancel(9102);assert!(matches!(fx.terminal(9102),JobStep::Failed(_)));
 fx.start(9103,&entry.kind,entry.input.clone().unwrap(),entry.checkpoint.clone());
 let(restored_pages,restored_done,restored_spent)=run(&mut fx,9103);
 assert!(restored_pages.is_empty());assert_eq!(restored_done,done);assert_eq!(restored_spent,spent);
}
