//! 🎯️ Registry ownership, original typed/wire sources and paid erased-job closure share one actual caller.
use super::*;
use super::admission::tests::{source,close,grant,clock};
use semio_framework_job::{Generation,RevisionId,OriginalJobFixtureCaller,OriginalJobDecodeOwner,JobOutcomeBorrow,JobOutcomeDescriptor,JobOutcomeView,JobOutcomeKind,InteractiveJobCloseStep,RetainedCloneGrant,RetainedCloneProgress};
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand};
struct Payload{bytes:[u8;64],len:usize}
struct Job{bytes:[u8;64],len:usize,input:Option<RetainedToolWireInput>,checkpoint:Option<RetainedToolWireInput>,decoded:bool,writer:Option<semio_framework_job::RetainedJobPayloadWriter>,published:Option<semio_framework_job::RetainedJobPayload>,cursor:usize,closing:bool}
impl Job{
 fn demands(&self)->Result<RetirementDemand,ValueError>{if let Some(owner)=self.input.as_ref().or(self.checkpoint.as_ref()){return Ok(RetirementDemand{release_bytes:owner.next_close_release_byte_demand()?,depth:owner.next_close_depth_demand()?,..Default::default()})}if let Some(owner)=self.writer.as_ref(){return owner.retirement_demands()}self.published.as_ref().map_or(Ok(Default::default()),|owner|owner.retirement_demands())}
}
impl InteractiveJob for Job{
 fn step<'a>(&'a mut self,cx:&mut StepContext<'_>)->Result<Option<JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure>{
  if !self.decoded{let input=self.input.as_ref().map_or(0,RetainedToolWireInput::declared_bytes);let total=input+self.checkpoint.as_ref().map_or(0,RetainedToolWireInput::declared_bytes);if self.len<total{let(owner,index)=if self.len<input{(self.input.as_ref().unwrap(),self.len)}else{(self.checkpoint.as_ref().unwrap(),self.len-input)};self.bytes[self.len]=owner.page(index/TOOL_WIRE_PAGE_BYTES).unwrap()[index%TOOL_WIRE_PAGE_BYTES];self.len+=1;cx.consume_fuel(1);}if self.len<total{return JobOutcomeBorrow::admit_yield(cx).map_err(Into::into)}self.decoded=true;}
  if self.published.is_some(){return JobOutcomeBorrow::admit_complete(cx,None,self.published.as_ref()).map_err(Into::into)}let writer=self.writer.get_or_insert_with(||semio_framework_job::RetainedJobPayloadWriter::new(semio_framework_job::JobPayloadStream::CommitOutput));if !writer.write_slice_page(cx,&self.bytes[..self.len],&mut self.cursor).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"fixture output page refused"))?{return JobOutcomeBorrow::admit_yield(cx).map_err(Into::into)}match self.writer.take().unwrap().finish(){Ok(payload)=>self.published=Some(payload),Err(writer)=>{self.writer=Some(writer);return JobOutcomeBorrow::admit_yield(cx).map_err(Into::into)}}JobOutcomeBorrow::admit_complete(cx,None,self.published.as_ref()).map_err(Into::into)
 }
 fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{match descriptor.kind(){JobOutcomeKind::Yield=>descriptor.yielded(),JobOutcomeKind::Complete=>descriptor.complete(None,self.published.as_ref()),_=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"fixture publication descriptor differs"))}}
 fn begin_close(&mut self){self.closing=true;}
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{self.begin_close();let g=cx.retained_grant();let step=if let Some(owner)=self.input.as_mut(){let step=owner.close_step(g)?;if owner.terminal_is_empty(){self.input=None;}step}else if let Some(owner)=self.checkpoint.as_mut(){let step=owner.close_step(g)?;if owner.terminal_is_empty(){self.checkpoint=None;}step}else if let Some(owner)=self.writer.as_mut(){let step=owner.close_step(g)?;if owner.terminal_is_empty(){self.writer=None;}match step{semio_framework_value::retained_clone::RetainedCloneStep::Complete(progress)=>InteractiveJobCloseStep::Complete{progress},semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress)=>InteractiveJobCloseStep::Pending{progress}}}else if let Some(owner)=self.published.as_mut(){let step=owner.close_step(g)?;if owner.terminal_is_empty(){self.published=None;}match step{semio_framework_value::retained_clone::RetainedCloneStep::Complete(progress)=>InteractiveJobCloseStep::Complete{progress},semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress)=>InteractiveJobCloseStep::Pending{progress}}}else{InteractiveJobCloseStep::Complete{progress:Default::default()}};cx.consume_retained(step.progress())?;cx.consume_fuel(step.progress().copied_items as u64);Ok(if self.terminal_is_empty(){InteractiveJobCloseStep::Complete{progress:step.progress()}}else{InteractiveJobCloseStep::Pending{progress:step.progress()}})}
 fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands()?.copy_bytes)}
 fn next_close_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(self.demands()?.capacity_bytes)}
 fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands()?.release_bytes)}
 fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands()?.depth)}
 fn terminal_is_empty(&self)->bool{self.closing&&self.input.is_none()&&self.checkpoint.is_none()&&self.writer.is_none()&&self.published.is_none()}
}
struct Factory<const RAW:usize>{keys:Vec<ToolFactoryKey>,classification:InteractiveJobClassification}
impl<const RAW:usize> ToolJobFactory for Factory<RAW>{
 type Payload=Payload;type Job=Job;
 fn keys(&self)->&[ToolFactoryKey]{&self.keys}
 fn payload_schema_id(&self)->&str{if RAW==8{"test.number.v1"}else{"test.echo.v1"}}
 fn classification(&self)->InteractiveJobClassification{self.classification}
 fn execution_contract(&self)->ToolExecutionContract{if RAW==8{ToolExecutionContract::resumable(RAW,1,64,64,100,1,1)}else{ToolExecutionContract::bounded_first_step(RAW,1,64,64,100)}}
 fn admission_demands(&self,_:&Operation,payload:&Payload,input:Option<&RetainedToolWireInput>,checkpoint:Option<&RetainedToolWireInput>)->Result<ToolJobAdmissionDemand,ValueError>{if payload.len>64||input.is_some_and(|owner|!owner.sealed||owner.declared_bytes()>RAW)||checkpoint.is_some_and(|owner|input.is_none()||!owner.sealed||owner.declared_bytes()>8)||input.map_or(0,RetainedToolWireInput::declared_bytes)+checkpoint.map_or(0,RetainedToolWireInput::declared_bytes)>64{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original fixture wire or checkpoint extent refused"))}Ok(ToolJobAdmissionDemand::boxed_job::<Payload,Job>())}
 fn admit_job(&mut self,cx:&mut StepContext<'_>,operation:&Operation,payload:&mut Option<Box<Payload>>,input:&mut Option<RetainedToolWireInput>,checkpoint:&mut Option<RetainedToolWireInput>,output:&mut Option<Box<Job>>)->Result<bool,semio_framework_job::OriginalTurnFailure>{let demand=self.admission_demands(operation,payload.as_deref().unwrap(),input.as_ref(),checkpoint.as_ref())?;admit_original_tool_job(cx,operation,payload,input,checkpoint,output,demand,|_,payload,input,checkpoint|{let Payload{bytes,len}=*payload;let decoded=input.is_none();Box::new(Job{bytes,len:if decoded{len}else{0},input,checkpoint,decoded,writer:None,published:None,cursor:0,closing:false})})}
}
fn factory<const RAW:usize>(controller:&str,tools:&[&str])->Factory<RAW>{Factory{keys:tools.iter().map(|tool|ToolFactoryKey::new(controller,*tool)).collect(),classification:InteractiveJobClassification::Migrated}}
fn empty_spec(controller:&str,tool:&str)->ToolOperationSpec{let operation=Operation::new(semio_framework_job::allocate_operation_id(),RevisionId(7),Generation(3),11);let mut caller=OriginalJobFixtureCaller::admit(operation.operation,operation.generation,grant(),clock,u64::MAX,10000);let mut decoder=OriginalJobDecodeOwner::empty();let source=identity(&mut caller,&mut decoder);close(&mut caller,&mut decoder);caller.retire(grant());ToolOperationSpec{key:ToolFactoryKey::new(controller,tool),payload:ToolPayload{schema_id:"test.echo.v1".into(),value:None},operation,source}}
fn wire(bytes:&[u8])->RetainedToolWireInput{let mut input=RetainedToolWireInput::try_new(bytes.len(),bytes.len()).unwrap();for chunk in bytes.chunks(TOOL_WIRE_PAGE_BYTES){input.admit_page(ToolWirePage::try_copy_from(chunk).unwrap()).ok().unwrap();}input.seal().unwrap();input}
fn identity(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner)->OriginalToolJobIdentity{let mut output=None;caller.turn_with_decoder(grant(),decoder,|cx|admit_original_tool_construction(cx,&mut output,ToolJobAdmissionDemand::reused_job(),|identity|Ok((identity,RetainedCloneProgress{copied_items:1,..Default::default()})))).unwrap();output.unwrap()}
fn admitted<const RAW:usize>(typed:&[u8],input:Option<RetainedToolWireInput>,checkpoint:Option<RetainedToolWireInput>,expected:&[u8]){
 let bus=ActionBus::new();bus.register(factory::<RAW>("fixture",&["exact"])).unwrap();let operation=Operation::new(semio_framework_job::allocate_operation_id(),RevisionId(7),Generation(3),11);let mut caller=OriginalJobFixtureCaller::admit(operation.operation,operation.generation,grant(),clock,u64::MAX,10000);let mut decoder=OriginalJobDecodeOwner::empty();let mut bytes=[0;64];bytes[..typed.len()].copy_from_slice(typed);let payload=source(&mut caller,&mut decoder,Payload{bytes,len:typed.len()});let mut spec=ToolOperationSpec::from_owned(ToolFactoryKey::new("fixture","exact"),if RAW==8{"test.number.v1"}else{"test.echo.v1"}.into(),payload,operation,identity(&mut caller,&mut decoder));let mut input=input;let mut checkpoint=checkpoint;let original=input.as_ref().and_then(|owner|owner.page(0)).map(|page|page.as_ptr());let mut job=None;assert!(caller.turn_with_decoder(grant(),&mut decoder,|cx|bus.admit_job(cx,&mut spec,&mut input,&mut checkpoint,&mut job)).unwrap());assert_eq!(bus.dispatch_count(),1);assert!(spec.payload.retained_payload().is_none());assert!(input.is_none()&&checkpoint.is_none());assert_eq!(spec.operation,operation);let job=job.as_mut().unwrap();let mut complete=false;for _ in 0..16{let descriptor=caller.turn_with_decoder(grant(),&mut decoder,|cx|Ok(job.step(cx)?.map(JobOutcomeBorrow::into_descriptor))).unwrap();if let Some(descriptor)=descriptor{if let JobOutcomeView::Complete{output:Some(output),..}=job.borrow_outcome(&descriptor).unwrap(){assert_eq!(output.page(0),Some(expected));complete=true;break;}}}assert!(complete);if let Some(pointer)=original{assert!(!pointer.is_null());}job.begin_close();for _ in 0..32{if job.terminal_is_empty(){break;}let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(grant(),&mut decoder,|cx|job.close_step(cx)));let step=step.unwrap();assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));}assert!(job.terminal_is_empty());close(&mut caller,&mut decoder);caller.retire(grant());
}
#[test]fn original_typed_factory_preserves_operation_and_publishes_under_original_paid_context(){admitted::<64>(b"ping:ok",None,None,b"ping:ok");}
#[test]fn original_wire_factory_preserves_restart_checkpoint_and_retained_input_until_paid_close(){let mut expected=42u64.to_le_bytes().to_vec();expected.extend([7,8]);admitted::<8>(&[],Some(wire(&42u64.to_le_bytes())),Some(wire(&[7,8])),&expected);}
#[test]fn registration_rejects_every_non_migrated_factory(){for classification in [InteractiveJobClassification::Unclassified,InteractiveJobClassification::BatchOnlyPendingRewrite,InteractiveJobClassification::ForbiddenFromUi,InteractiveJobClassification::Deleted]{let bus=ActionBus::new();let mut owner=factory::<64>("app",&["ping"]);owner.classification=classification;assert!(matches!(bus.register(owner),Err(ToolRegistrationError::NonInteractiveClassification{classification:actual,..})if actual==classification));}}
#[test]fn exact_factory_ownership_never_falls_back_through_aliases(){let bus=ActionBus::new();let alias=ToolFactoryKey::new("app","alias");let target=ToolFactoryKey::new("app","exact");assert!(matches!(bus.register_alias(alias.clone(),target.clone()),Err(ToolRegistrationError::UnknownAliasTarget{..})));bus.register(factory::<64>("app",&["exact"])).unwrap();bus.register_alias(alias,target).unwrap();for key in ["alias","missing"]{assert!(bus.admission_demands(&empty_spec("app",key),None,None).is_err());assert!(matches!(bus.admit_exact_wire("app",key,"test.echo.v1",b"ok"),Err(ToolDispatchError::UnknownController{..})));}}
#[test]fn unknown_controller_is_an_explicit_original_admission_refusal(){assert!(ActionBus::new().admission_demands(&empty_spec("missing","ping"),None,None).is_err());}
#[test]fn exact_wire_admission_checks_schema_limit_and_factory_identity_before_decode(){let bus=ActionBus::new();bus.register(factory::<64>("app",&["exact"])).unwrap();assert!(matches!(bus.admit_exact_wire("app","exact","wrong.schema",b"ok"),Err(ToolDispatchError::Factory{..})));assert!(matches!(bus.admit_exact_wire("app","exact","test.echo.v1",&[0;65]),Err(ToolDispatchError::RawWireLimit{actual:65,maximum:64,..})));let admission=bus.admit_exact_wire("app","exact","test.echo.v1",b"ok").unwrap();assert_eq!(admission.factory_type_id,TypeId::of::<Factory<64>>());assert_eq!(admission.key,ToolFactoryKey::new("app","exact"));}
#[test]fn heterogeneous_factories_share_one_bus_with_exact_key_ownership(){let bus=ActionBus::new();bus.register(factory::<64>("text",&["one","two"])).unwrap();bus.register(factory::<8>("number",&["encode"])).unwrap();assert_eq!(bus.keys().len(),3);assert!(bus.contains(&ToolFactoryKey::new("text","two")));assert_ne!(bus.admit_exact_wire("text","one","test.echo.v1",b"a").unwrap().factory_type_id,bus.admit_exact_wire("number","encode","test.number.v1",b"a").unwrap().factory_type_id);}
#[test]fn duplicate_factory_keys_are_rejected_without_partial_registration(){let bus=ActionBus::new();bus.register(factory::<64>("app",&["one"])).unwrap();assert!(matches!(bus.register(factory::<64>("app",&["two","one"])),Err(ToolRegistrationError::DuplicateKey{..})));assert_eq!(bus.keys().len(),1);let bus=ActionBus::new();assert!(matches!(bus.register(factory::<64>("app",&["same","same"])),Err(ToolRegistrationError::DuplicateKey{..})));assert_eq!(bus.keys().len(),0);}
#[test]fn retained_wire_page_saturation_returns_the_exact_original_page(){let mut input=RetainedToolWireInput::try_new(3,3).unwrap();let page=ToolWirePage::try_copy_from(&[1,2,3,4]).unwrap();let(error,page)=input.admit_page(page).unwrap_err();assert!(!error.detail.is_empty());assert_eq!(page.as_slice(),[1,2,3,4]);assert_eq!(input.page_count(),0);input.admit_page(ToolWirePage::try_copy_from(&[1,2,3]).unwrap()).ok().unwrap();input.seal().unwrap();let pointer=input.page(0).unwrap().as_ptr();let capacity=input.next_close_release_byte_demand().unwrap();let denied=RetainedCloneGrant{maximum_release_bytes:capacity-1,..grant()};let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||input.close_step(denied).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(input.pages[0].as_slice().as_ptr(),pointer);let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||input.close_step(grant()).unwrap());assert_eq!(step.progress().released_bytes,capacity);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,capacity));assert!(input.terminal_is_empty());}
#[test]fn maximum_extent_seals_incremental_prefix_without_reallocation(){let mut input=RetainedToolWireInput::try_new(TOOL_WIRE_PAGE_BYTES*2,TOOL_WIRE_PAGE_BYTES*2).unwrap();let capacity=input.pages.capacity();input.admit_page(ToolWirePage::try_copy_from(b"exact").unwrap()).ok().unwrap();input.seal_admitted_prefix().unwrap();assert_eq!(input.declared_bytes(),5);assert_eq!(input.pages.capacity(),capacity);assert_eq!(input.page(0),Some(b"exact".as_slice()));input.begin_close();input.close_step(grant()).unwrap();assert!(input.terminal_is_empty());}

#[test]fn original_factory_refuses_foreign_paid_roots_even_with_equal_operation_and_generation(){let bus=ActionBus::new();bus.register(factory::<64>("fixture",&["exact"])).unwrap();let operation=Operation::new(semio_framework_job::allocate_operation_id(),RevisionId(7),Generation(3),11);let mut original=OriginalJobFixtureCaller::admit(operation.operation,operation.generation,grant(),clock,u64::MAX,10000);let mut foreign=OriginalJobFixtureCaller::admit(operation.operation,operation.generation,grant(),clock,u64::MAX,10000);let mut decoder=OriginalJobDecodeOwner::empty();let mut other_decoder=OriginalJobDecodeOwner::empty();let payload=source(&mut original,&mut decoder,Payload{bytes:[4;64],len:1});let pointer=&*payload as*const Payload;let source_identity=identity(&mut original,&mut decoder);let mut spec=ToolOperationSpec::from_owned(ToolFactoryKey::new("fixture","exact"),"test.echo.v1".into(),payload,operation,source_identity);let mut input=None;let mut checkpoint=None;let mut output=None;let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||foreign.turn_with_decoder(grant(),&mut other_decoder,|cx|{assert!(!source_identity.matches(cx));bus.admit_job(cx,&mut spec,&mut input,&mut checkpoint,&mut output)}));assert!(result.is_err());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(spec.payload.value.as_ref().unwrap().downcast_ref::<Payload>().unwrap()as*const Payload,pointer);assert!(output.is_none());let mut payload=Some(spec.payload.value.take().unwrap().downcast::<Payload>().ok().unwrap());original.turn(grant(),|cx|{drop(payload.take());cx.consume_retained(RetainedCloneProgress{copied_items:1,released_bytes:std::mem::size_of::<Payload>(),..Default::default()})?;cx.consume_fuel(1);Ok(())}).unwrap();close(&mut original,&mut decoder);close(&mut foreign,&mut other_decoder);original.retire(grant());foreign.retire(grant());eprintln!("[DEBUG] equal semantic IDs cannot substitute foreign original payload roots or bound clock");}

mod current_caller_laws {
use super::*;
use semio_framework_job::{allocate_operation_id, Generation, RevisionId, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView};
use semio_framework_value::{ValueError,ValueRefusalKind};

fn fixture_step_grant()->semio_framework_job::RetainedCloneGrant{
    semio_framework_job::RetainedCloneGrant{maximum_items:8,maximum_copy_bytes:4096,maximum_capacity_bytes:8192,maximum_release_bytes:4096,maximum_depth:4}
}

fn fixture_scope_grant()->semio_framework_job::RetainedCloneGrant{
    semio_framework_job::RetainedCloneGrant{maximum_items:64,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:16}
}

struct ImmediateJob {
    output: Option<Vec<u8>>,
    writer: Option<semio_framework_job::RetainedJobPayloadWriter>,
    published: Option<semio_framework_job::RetainedJobPayload>,
    cursor: usize,
    closing: bool,
}
impl InteractiveJob for ImmediateJob {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure> {
        if self.published.is_some() { return JobOutcomeBorrow::admit_complete(cx,None,self.published.as_ref()).map_err(Into::into); }
        let writer = self.writer.get_or_insert_with(|| semio_framework_job::RetainedJobPayloadWriter::new(semio_framework_job::JobPayloadStream::CommitOutput));
        if !writer.write_slice_page(cx, self.output.as_deref().unwrap_or_default(), &mut self.cursor).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"original fixture payload page admission refused"))? { return JobOutcomeBorrow::admit_yield(cx).map_err(Into::into); }
        match self.writer.take().expect("immediate original output writer").finish(){Ok(payload)=>self.published=Some(payload),Err(writer)=>{self.writer=Some(writer);return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original fixture payload writer has not completed").into())}}
        JobOutcomeBorrow::admit_complete(cx,None,self.published.as_ref()).map_err(Into::into)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>,ValueError> {
        match descriptor.kind() {
            JobOutcomeKind::Yield => descriptor.yielded(),
            JobOutcomeKind::Complete => descriptor.complete(None,self.published.as_ref()),
            _ => Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"immediate fixture requires its original yielded or completed descriptor")),
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
        if let Some(writer) = self.writer.as_mut() {
            writer.begin_close();
        }
    }

    fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        if cx.should_yield()||cx.fuel_remaining()==0{return Ok(semio_framework_job::InteractiveJobCloseStep::Blocked)}
        let result=self.close_granted(cx.retained_grant());
        match result{Ok(step)=>{cx.consume_fuel(step.progress().copied_items as u64);cx.consume_retained(step.progress())?;Ok(step)},Err(error)=>{cx.consume_fuel(error.retained_progress().copied_items as u64);Err(semio_framework_job::OriginalTurnFailure::debit_original(error,cx))}}
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize,ValueError> { Ok(self.demands()?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self,_:usize) -> Result<usize,ValueError> { Ok(self.demands()?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize,ValueError> { Ok(self.demands()?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize,ValueError> { Ok(self.demands()?.depth) }
    fn terminal_is_empty(&self) -> bool { self.closing && self.output.is_none() && self.writer.is_none() && self.published.is_none() }

}

impl ImmediateJob {
    fn close_granted(&mut self,grant:semio_framework_job::RetainedCloneGrant)->Result<semio_framework_job::InteractiveJobCloseStep,ValueError>{
        use semio_framework_job::{InteractiveJobCloseStep,RetainedCloneProgress};
        if grant.maximum_items==0{return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
        self.begin_close();
        if let Some(writer)=self.writer.as_mut(){let step=writer.close_step(grant)?;if writer.terminal_is_empty(){self.writer=None;}return Ok(if self.terminal_is_empty(){InteractiveJobCloseStep::Complete{progress:step.progress()}}else{InteractiveJobCloseStep::Pending{progress:step.progress()}})}
        if let Some(output)=self.output.as_ref(){let bytes=output.capacity();if grant.maximum_release_bytes<bytes{return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}if bytes!=0&&grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original fixture output close exceeds depth"))}drop(self.output.take());let progress=RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()};return Ok(if self.terminal_is_empty(){InteractiveJobCloseStep::Complete{progress}}else{InteractiveJobCloseStep::Pending{progress}})}
        if let Some(payload)=self.published.as_mut(){let step=payload.close_step(grant)?;if payload.terminal_is_empty(){self.published=None;}return Ok(if self.terminal_is_empty(){InteractiveJobCloseStep::Complete{progress:step.progress()}}else{InteractiveJobCloseStep::Pending{progress:step.progress()}})}
        Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})
    }

    fn demands(&self)->Result<semio_framework_value::RetirementDemand,ValueError>{
        if let Some(writer)=self.writer.as_ref(){return writer.retirement_demands()}
        if let Some(output)=self.output.as_ref(){return Ok(semio_framework_value::RetirementDemand{release_bytes:output.capacity(),depth:usize::from(output.capacity()!=0),..Default::default()})}
        if let Some(payload)=self.published.as_ref(){return payload.retirement_demands()}
        Ok(Default::default())
    }
}
struct EchoFactory {
    keys: Vec<ToolFactoryKey>,
    classification: InteractiveJobClassification,
}

impl ToolJobFactory for EchoFactory {
    type Payload = String;
    type Job = ImmediateJob;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        "test.echo.v1"
    }

    fn classification(&self) -> InteractiveJobClassification {
        self.classification
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        ToolExecutionContract::bounded_first_step(64, 1, 1, 64, 100)
    }

    fn create_job(&mut self, _operation: Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ImmediateJob { output: Some(format!("{payload}:ok").into_bytes()), writer: None, published: None, cursor: 0, closing: false })
    }
}

fn operation_spec(controller_id: &str, tool_id: &str) -> ToolOperationSpec {
    ToolOperationSpec::new(controller_id, tool_id, "test.echo.v1", tool_id.to_string(), Operation::new(allocate_operation_id(), RevisionId(7), Generation(3), 11))
}

fn echo_factory(controller_id: &str, tool_ids: &[&str], classification: InteractiveJobClassification) -> EchoFactory {
    EchoFactory { keys: tool_ids.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect(), classification }
}

#[test]
fn dispatch_returns_a_resumable_job_and_preserves_operation_identity() {
    let bus = ActionBus::new();
    bus.register(echo_factory("app", &["ping"], InteractiveJobClassification::Migrated)).unwrap();
    let dispatch = bus.dispatch(operation_spec("app", "ping")).unwrap();
    assert_eq!(dispatch.spec.operation.base_revision, RevisionId(7));
    assert_eq!(dispatch.spec.operation.generation, Generation(3));
}

#[test]
fn registration_rejects_every_non_migrated_factory() {
    for classification in [InteractiveJobClassification::Unclassified, InteractiveJobClassification::BatchOnlyPendingRewrite, InteractiveJobClassification::ForbiddenFromUi, InteractiveJobClassification::Deleted] {
        let bus = ActionBus::new();
        assert!(matches!(
            bus.register(echo_factory("app", &["ping"], classification)),
            Err(ToolRegistrationError::NonInteractiveClassification { classification: rejected, .. }) if rejected == classification
        ));
    }
}

#[test]
fn unknown_controller_is_an_explicit_dispatch_error() {
    let bus = ActionBus::new();
    assert!(matches!(bus.dispatch(operation_spec("missing", "ping")), Err(ToolDispatchError::UnknownController { .. })));
}

#[test]
fn aliases_require_an_existing_exact_factory_and_never_fallback() {
    let bus = ActionBus::new();
    let alias = ToolFactoryKey::new("app", "alias");
    let target = ToolFactoryKey::new("app", "exact");
    assert!(matches!(bus.register_alias(alias.clone(), target.clone()), Err(ToolRegistrationError::UnknownAliasTarget { .. })));
    bus.register(echo_factory("app", &["exact"], InteractiveJobClassification::Migrated)).unwrap();
    bus.register_alias(alias, target).unwrap();
    assert!(matches!(bus.dispatch(operation_spec("app", "alias")), Err(ToolDispatchError::UnknownController { .. })));
    assert!(matches!(bus.dispatch(operation_spec("app", "missing")), Err(ToolDispatchError::UnknownController { .. })));
}

#[test]
fn exact_wire_admission_rejects_alias_schema_and_raw_limit_before_decode() {
    let bus = ActionBus::new();
    let exact = ToolFactoryKey::new("app", "exact");
    let alias = ToolFactoryKey::new("app", "alias");
    bus.register(echo_factory("app", &["exact"], InteractiveJobClassification::Migrated)).unwrap();
    bus.register_alias(alias, exact.clone()).unwrap();
    assert!(matches!(bus.admit_exact_wire("app", "alias", "test.echo.v1", b"ok"), Err(ToolDispatchError::UnknownController { .. })));
    let operation = Operation::new(allocate_operation_id(), RevisionId(0), Generation(0), 1);
    assert!(matches!(bus.dispatch_wire("app", "alias", "test.echo.v1", b"ok", None, operation), Err(ToolDispatchError::UnknownController { .. })));
    assert!(matches!(bus.admit_exact_wire("app", "exact", "wrong.schema", b"ok"), Err(ToolDispatchError::Factory { .. })));
    assert!(matches!(bus.admit_exact_wire("app", "exact", "test.echo.v1", &[0; 65]), Err(ToolDispatchError::RawWireLimit { actual: 65, maximum: 64, .. })));
    let admission = bus.admit_exact_wire("app", "exact", "test.echo.v1", b"ok").unwrap();
    assert_eq!(admission.key, exact);
    assert_eq!(admission.factory_type_id, TypeId::of::<EchoFactory>());
    assert_eq!(admission.contract.max_raw_wire_bytes, 64);
}

struct NumberFactory {
    keys: Vec<ToolFactoryKey>,
}

impl ToolJobFactory for NumberFactory {
    type Payload = u64;
    type Job = ImmediateJob;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        "test.number.v1"
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        ToolExecutionContract::bounded_first_step(8, 1, 1, 8, 100)
    }

    fn create_job(&mut self, _operation: Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ImmediateJob { output: Some(payload.to_le_bytes().to_vec()), writer: None, published: None, cursor: 0, closing: false })
    }

    fn create_job_from_wire(&mut self, _operation: Operation, payload: &[u8], checkpoint: Option<Vec<u8>>) -> Result<Self::Job, ToolJobFactoryError> {
        let value = u64::from_le_bytes(payload.try_into().map_err(|_| ToolJobFactoryError::new("number wire payload must contain exactly eight bytes"))?);
        let mut output = value.to_le_bytes().to_vec();
        output.extend(checkpoint.unwrap_or_default());
        Ok(ImmediateJob { output: Some(output), writer: None, published: None, cursor: 0, closing: false })
    }
}

#[test]
fn heterogeneous_factories_share_one_bus_with_exact_key_ownership() {
    let bus = ActionBus::new();
    bus.register(echo_factory("text", &["one", "two"], InteractiveJobClassification::Migrated)).unwrap();
    bus.register(NumberFactory { keys: vec![ToolFactoryKey::new("number", "encode")] }).unwrap();
    assert_eq!(bus.keys().len(), 3);
    assert!(bus.contains(&ToolFactoryKey::new("text", "two")));
    let spec = ToolOperationSpec::new("number", "encode", "test.number.v1", 42u64, Operation::new(allocate_operation_id(), RevisionId(0), Generation(0), 9));
    assert!(bus.dispatch(spec).is_ok());
}

#[test]
fn duplicate_factory_key_is_rejected_without_partial_registration() {
    let bus = ActionBus::new();
    bus.register(echo_factory("app", &["one"], InteractiveJobClassification::Migrated)).unwrap();
    let result = bus.register(echo_factory("app", &["two", "one"], InteractiveJobClassification::Migrated));
    assert!(matches!(result, Err(ToolRegistrationError::DuplicateKey { key }) if key == ToolFactoryKey::new("app", "one")));
    assert_eq!(bus.keys().len(), 1);
}

#[test]
fn duplicate_key_inside_one_factory_is_rejected_atomically() {
    let bus = ActionBus::new();
    let result = bus.register(echo_factory("app", &["same", "same"], InteractiveJobClassification::Migrated));
    assert!(matches!(result, Err(ToolRegistrationError::DuplicateKey { key }) if key == ToolFactoryKey::new("app", "same")));
    assert_eq!(bus.keys().len(), 0);
}

#[test]
fn wire_dispatch_uses_the_factory_decoder_and_preserves_the_restart_checkpoint() {
    let bus = ActionBus::new();
    bus.register(NumberFactory { keys: vec![ToolFactoryKey::new("number", "decode-wire")] }).unwrap();
    let operation = Operation::new(allocate_operation_id(), RevisionId(19), Generation(5), 13);
    let mut dispatch = bus.dispatch_wire("number", "decode-wire", "test.number.v1", &42u64.to_le_bytes(), Some(vec![7, 8]), operation).expect("wire dispatch");
    let mut sequence = 0;
    let mut retained_progress=semio_framework_job::RetainedCloneProgress::default();
    let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(64,660000000,fixture_scope_grant()), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence,&mut retained_progress);
    let mut expected = 42u64.to_le_bytes().to_vec();
    expected.extend([7, 8]);
    let descriptor=dispatch.job.step(&mut context).unwrap().expect("original complete admission").into_descriptor();
    {let JobOutcomeView::Complete{output:Some(output),..}=dispatch.job.borrow_outcome(&descriptor).unwrap()else{panic!("original wire output remains inside job")};assert_eq!(output.page(0),Some(expected.as_slice()));assert_eq!(output.page_count(),1);}
    dispatch.job.begin_close();
    for _ in 0..16 {
        if dispatch.job.terminal_is_empty() { break; }
        let grant=context.retained_grant();
        assert!(dispatch.job.close_step(&mut context).unwrap().progress().fits(grant));
    }
    assert!(dispatch.job.terminal_is_empty());
    assert!(matches!(bus.dispatch_wire("number", "decode-wire", "wrong.schema", &42u64.to_le_bytes(), None, operation), Err(ToolDispatchError::Factory { .. })));
    assert!(matches!(bus.dispatch_wire("number", "decode-wire", "test.number.v1", &[0; 9], None, operation), Err(ToolDispatchError::RawWireLimit { actual: 9, maximum: 8, .. })));
}

#[test]
fn completed_job_retains_original_input_until_exact_granted_close() {
    let fixture = semio_framework_pack_json::parse(include_str!("../../🧹️wire-retirement/🧪️tests/🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let law = &fixture["completedInput"];
    let mut original = Vec::with_capacity(law["capacityBytes"].as_u64().unwrap() as usize);
    original.extend([4, 5, 6]);
    let pointer = original.as_ptr();
    let capacity = original.capacity();
    let mut job = ImmediateJob { output: Some(original), writer: None, published: None, cursor: 0, closing: false };
    let mut sequence = 0;
    let operation = allocate_operation_id();
    let mut retained_progress=semio_framework_job::RetainedCloneProgress::default();
    let mut context = StepContext::new(operation, Generation(1), semio_framework_job::StepBudget::new(64,660000000,fixture_scope_grant()), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence,&mut retained_progress);
    let descriptor=job.step(&mut context).unwrap().expect("original complete admission").into_descriptor();
    assert!(matches!(job.borrow_outcome(&descriptor).unwrap(),JobOutcomeView::Complete{output:Some(_),..}));
    let retained = job.output.as_ref().expect("completion retains the original input allocation");
    assert_eq!(retained.as_ptr(), pointer);
    assert_eq!(retained.capacity(), capacity);
    assert_eq!(retained.as_slice(), [4, 5, 6]);
    job.begin_close();
    let grant=semio_framework_job::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:8192,maximum_release_bytes:64,maximum_depth:4};
    let denied = semio_framework_job::RetainedCloneGrant { maximum_release_bytes: capacity - 1, ..grant };
    let (step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.close_granted(denied).unwrap());
    context.consume_retained(step.progress()).unwrap();
    assert_eq!(step.progress(), Default::default());
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    assert_eq!(job.output.as_ref().unwrap().as_ptr(), pointer);
    assert!(context.retained_grant().maximum_release_bytes>=grant.maximum_release_bytes);
    let (step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.close_granted(grant).unwrap());
    context.consume_fuel(step.progress().copied_items as u64);context.consume_retained(step.progress()).unwrap();
    assert_eq!(step.progress().released_bytes, law["releasedAfterClose"].as_u64().unwrap() as usize);
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, capacity));
    assert!(step.progress().fits(grant));
    assert!(!job.terminal_is_empty());
    for _ in 0..16 {
        if job.terminal_is_empty(){break}
        let grant=context.retained_grant();
        let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.close_step(&mut context).unwrap());
        assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));
        assert!(step.progress().fits(grant));
    }
    assert!(job.terminal_is_empty());
}

struct RetainedNumberJob {
    checkpoint: semio_framework_job::RetainedJobPayload,
    input: Option<RetainedToolWireInput>,
    bytes: [u8; 8],
    cursor: usize,
    output: Option<ImmediateJob>,
    closing: bool,
}

impl RetainedNumberJob{
    fn close_granted(&mut self,grant:semio_framework_job::RetainedCloneGrant)->Result<semio_framework_job::InteractiveJobCloseStep,ValueError>{use semio_framework_job::InteractiveJobCloseStep;(||{if grant.maximum_items==0{return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}if !self.terminal_is_empty()&&grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original fixture child close exceeds depth"))}self.begin_close();let child=semio_framework_job::RetainedCloneGrant{maximum_depth:grant.maximum_depth.saturating_sub(1),..grant};let step=if let Some(input)=self.input.as_mut(){let step=input.close_step(child)?;if input.terminal_is_empty(){self.input=None;}step}else if let Some(output)=self.output.as_mut(){let step=output.close_granted(child)?;if output.terminal_is_empty(){self.output=None;}step}else{return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})};Ok(match step{InteractiveJobCloseStep::Pending{progress}|InteractiveJobCloseStep::Complete{progress}=>if self.terminal_is_empty(){InteractiveJobCloseStep::Complete{progress}}else{InteractiveJobCloseStep::Pending{progress}},step=>step})})()}
}

impl InteractiveJob for RetainedNumberJob {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure> {
        if cx.is_cancelled() {
            return JobOutcomeBorrow::admit_cancelled(cx).map_err(Into::into);
        }
        if cx.should_yield() || cx.fuel_remaining() == 0 {
            return JobOutcomeBorrow::admit_yield(cx).map_err(Into::into);
        }
        if self.cursor < self.bytes.len() {
            self.bytes[self.cursor] = self.input.as_ref().and_then(|input| input.page(0)).and_then(|page| page.get(self.cursor)).copied().unwrap_or_default();
            self.cursor += 1;
            cx.consume_fuel(1);
            return JobOutcomeBorrow::admit_checkpoint(cx,&self.checkpoint,self.cursor as u64).map_err(Into::into);
        }
        self.output.get_or_insert_with(|| ImmediateJob { output: Some(self.bytes.to_vec()), writer: None, published: None, cursor: 0, closing: false }).step(cx)
    }

    fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{
        match descriptor.kind(){
            JobOutcomeKind::Yield=>descriptor.yielded(),JobOutcomeKind::Cancelled=>descriptor.cancelled(),
            JobOutcomeKind::CheckpointReady{..}=>descriptor.checkpoint(&self.checkpoint),
            JobOutcomeKind::Complete=>self.output.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"retained fixture original completed output absent"))?.borrow_outcome(descriptor),
            _=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"retained fixture original descriptor kind mismatch")),
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
        if let Some(input) = self.input.as_mut() {
            input.begin_close();
        }
        if let Some(output) = self.output.as_mut() {
            output.begin_close();
        }
    }

    fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        if cx.should_yield()||cx.fuel_remaining()==0{return Ok(semio_framework_job::InteractiveJobCloseStep::Blocked)}
        let result=self.close_granted(cx.retained_grant());
        match result{Ok(step)=>{cx.consume_fuel(step.progress().copied_items as u64);cx.consume_retained(step.progress())?;Ok(step)},Err(error)=>{cx.consume_fuel(error.retained_progress().copied_items as u64);Err(semio_framework_job::OriginalTurnFailure::debit_original(error,cx))}}
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { self.input.as_ref().map_or_else(|| self.output.as_ref().map_or(Ok(0), |owner| owner.next_close_copy_byte_demand()), |owner| owner.next_close_copy_byte_demand()) }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { self.input.as_ref().map_or_else(|| self.output.as_ref().map_or(Ok(0), |owner| owner.next_close_capacity_byte_demand(body)), |owner| owner.next_close_capacity_byte_demand(body)) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { self.input.as_ref().map_or_else(|| self.output.as_ref().map_or(Ok(0), |owner| owner.next_close_release_byte_demand()), |owner| owner.next_close_release_byte_demand()) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { let depth = self.input.as_ref().map_or_else(|| self.output.as_ref().map_or(Ok(0), |owner| owner.next_close_depth_demand()), |owner| owner.next_close_depth_demand())?; Ok(if self.input.is_some() || self.output.is_some() { depth + 1 } else { 0 }) }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.input.is_none() && self.output.is_none()
    }
}

struct RetainedNumberFactory {
    keys: Vec<ToolFactoryKey>,
}

impl ToolJobFactory for RetainedNumberFactory {
    type Payload = RetainedNumberJob;
    type Job = RetainedNumberJob;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        "test.retained-number.v1"
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        ToolExecutionContract::resumable(8, 1, 1, 8, 100, 1, 1)
    }

    fn create_job(&mut self, _operation: Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(payload)
    }

    fn create_job_from_wire_pages(&mut self, _operation: Operation, input: RetainedToolWireInput, checkpoint: Option<RetainedToolWireInput>) -> Result<Self::Job, (ToolJobFactoryError, RetainedToolWireInput, Option<RetainedToolWireInput>)> {
        if input.declared_bytes() != 8 || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("retained number requires eight bytes and no checkpoint"), input, checkpoint));
        }
        Ok(RetainedNumberJob { checkpoint: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CheckpointState), input: Some(input), bytes: [0; 8], cursor: 0, output: None, closing: false })
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: Operation,
        mut payload: Self::Payload,
        input: RetainedToolWireInput,
        checkpoint: Option<RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, RetainedToolWireInput, Option<RetainedToolWireInput>)> {
        if input.declared_bytes() != 8 || checkpoint.is_some() || payload.input.is_some() {
            return Err((ToolJobFactoryError::new("retained number payload requires one eight-byte raw owner and no checkpoint"), input, checkpoint));
        }
        payload.input = Some(input);
        Ok(payload)
    }
}

#[test]
fn retained_wire_pages_are_admitted_sealed_transferred_and_closed_by_logical_bytes() {
    let bus = ActionBus::new();
    bus.register(RetainedNumberFactory { keys: vec![ToolFactoryKey::new("number", "retained")] }).unwrap();
    let (admission, mut input) = bus.begin_exact_wire("number", "retained", "test.retained-number.v1", 8).unwrap();
    input.admit_page(ToolWirePage::try_copy_from(&42u64.to_le_bytes()).unwrap()).unwrap();
    assert!(input.seal().is_ok());
    let operation = Operation::new(allocate_operation_id(), RevisionId(1), Generation(2), 3);
    assert_eq!(admission.schema_id,"test.retained-number.v1");
    let mut factory=RetainedNumberFactory{keys:vec![ToolFactoryKey::new("number","retained")]};
    let mut job=match factory.create_job_from_wire_pages(operation,input,None){Ok(job)=>job,Err(_)=>panic!("typed original wire factory refused")};
    let mut sequence = 0;
    let mut retained_progress=semio_framework_job::RetainedCloneProgress::default();
    let mut context=StepContext::new(operation.operation,operation.generation,semio_framework_job::StepBudget::new(64,660000000,fixture_scope_grant()),semio_framework_job::root_cancel_token(),||Some(0),&mut sequence,&mut retained_progress);
    for _ in 0..8 {assert!(matches!(job.step(&mut context),Ok(Some(JobOutcomeBorrow::CheckpointReady{..}))));}
    job.begin_close();
    let fixture = semio_framework_pack_json::parse(include_str!("../../🧹️wire-retirement/🧪️tests/🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let law = &fixture["shortClose"];
    assert_eq!(law["logicalBytes"].as_u64(), Some(8));
    let grant=semio_framework_job::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:8,maximum_capacity_bytes:0,maximum_release_bytes:4096+size_of::<usize>(),maximum_depth:2};
    assert!(grant.maximum_release_bytes >= 8);
    for denied in fixture["denied"].as_array().unwrap() {
        let denied = match denied.as_str().unwrap() {
            "items" => semio_framework_job::RetainedCloneGrant { maximum_items: 0, ..grant },
            "release" => semio_framework_job::RetainedCloneGrant { maximum_release_bytes: grant.maximum_release_bytes - 1, ..grant },
            "depth" => semio_framework_job::RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant },
            _ => unreachable!(),
        };
        let result=job.close_granted(denied);
        let progress=match result{Ok(step)=>step.progress(),Err(failure)=>{assert_eq!(failure.kind,ValueRefusalKind::DepthLimit);failure.retained_progress()}};
        assert_eq!(progress, Default::default());
        assert!(!job.terminal_is_empty());
    }
    let step=job.close_granted(grant).unwrap();
    context.consume_fuel(step.progress().copied_items as u64);context.consume_retained(step.progress()).unwrap();
    assert!(matches!(step, semio_framework_job::InteractiveJobCloseStep::Complete { .. }));
    assert_eq!(step.progress().copied_items, law["copiedItems"].as_u64().unwrap() as usize);
    assert_eq!(step.progress().copied_bytes, law["copiedBytes"].as_u64().unwrap() as usize);
    assert_eq!(step.progress().retained_capacity_bytes, law["retainedCapacityBytes"].as_u64().unwrap() as usize);
    assert_eq!(step.progress().released_bytes, grant.maximum_release_bytes);
    assert!(step.progress().fits(grant));
    assert!(job.terminal_is_empty());
}
#[test]
fn production_typed_payload_and_retained_pages_enter_the_same_registered_factory_job() {
    let bus = ActionBus::new();
    bus.register(RetainedNumberFactory { keys: vec![ToolFactoryKey::new("number", "retained")] }).unwrap();
    let (admission, mut input) = bus.begin_exact_wire("number", "retained", "test.retained-number.v1", 8).unwrap();
    input.admit_page(ToolWirePage::try_copy_from(&42u64.to_le_bytes()).unwrap()).unwrap();
    input.seal().unwrap();
    let operation = Operation::new(allocate_operation_id(), RevisionId(1), Generation(2), 3);
    let payload = RetainedNumberJob { checkpoint: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CheckpointState), input: None, bytes: [0; 8], cursor: 0, output: None, closing: false };
    let spec = ToolOperationSpec::new("number", "retained", "test.retained-number.v1", payload, operation);
    let mut dispatch = match bus.dispatch_wire_retained_with_spec(&admission, input, None, spec) {
        Ok(dispatch) => dispatch,
        Err(_) => panic!("production retained payload dispatch was rejected"),
    };
    let mut sequence = 0;
    let mut retained_progress=semio_framework_job::RetainedCloneProgress::default();
    let mut context=StepContext::new(operation.operation,operation.generation,semio_framework_job::StepBudget::new(64,660000000,fixture_scope_grant()),semio_framework_job::root_cancel_token(),||Some(0),&mut sequence,&mut retained_progress);
    for _ in 0..8 {assert!(matches!(dispatch.job.step(&mut context),Ok(Some(JobOutcomeBorrow::CheckpointReady{..}))));}
    dispatch.job.begin_close();
    for _ in 0..16 {
        if dispatch.job.terminal_is_empty() { break; }
        let grant=context.retained_grant();
        assert!(dispatch.job.close_step(&mut context).unwrap().progress().fits(grant));
    }
    assert!(dispatch.job.terminal_is_empty());
}

#[test]
fn retained_wire_admission_rejects_plus_one_and_returns_the_page_owner_on_saturation() {
    let bus = ActionBus::new();
    bus.register(RetainedNumberFactory { keys: vec![ToolFactoryKey::new("number", "retained")] }).unwrap();
    assert!(matches!(bus.begin_exact_wire("number", "retained", "test.retained-number.v1", 9), Err(ToolDispatchError::RawWireLimit { actual: 9, maximum: 8, .. })));
    let (_, mut input) = bus.begin_exact_wire("number", "retained", "test.retained-number.v1", 8).unwrap();
    input.admit_page(ToolWirePage::try_copy_from(&[0; 8]).unwrap()).unwrap();
    let rejected = input.admit_page(ToolWirePage::try_copy_from(&[1]).unwrap()).expect_err("plus-one page owner must be returned");
    assert_eq!(rejected.1.as_slice(), &[1]);
}

#[test]
fn maximum_extent_owner_exists_before_incremental_encoding_and_seals_to_its_exact_prefix() {
    let bus = ActionBus::new();
    bus.register(RetainedNumberFactory { keys: vec![ToolFactoryKey::new("number", "retained")] }).unwrap();
    let (admission, mut input) = bus.begin_exact_wire("number", "retained", "test.retained-number.v1", 8).unwrap();
    input.admit_page(ToolWirePage::try_copy_from(&42u32.to_le_bytes()).unwrap()).unwrap();
    input.seal_admitted_prefix().unwrap();
    assert_eq!(input.declared_bytes(), 4);
    assert_eq!(input.page(0), Some(42u32.to_le_bytes().as_slice()));
    let operation = Operation::new(allocate_operation_id(), RevisionId(1), Generation(2), 3);
    let rejected = match bus.dispatch_wire_retained(admission, input, None, operation) {
        Ok(_) => panic!("factory owns the exact eight-byte decoder and must reject a truthful four-byte prefix"),
        Err(rejected) => rejected,
    };
    assert_eq!(rejected.input.declared_bytes(), 4);
}

}
