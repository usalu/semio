//! 🏷️ Reserved preparation owns partial metadata, actual arguments and every admission receiver through original closure.
use semio_framework_plugin_decl_app::{ArtifactApp,OriginalCompletionBirth};
use semio_framework_plugin_decl_app::tool_contract::{ArtifactReservedToolInput,ArtifactReservedToolJobRequest,ArtifactReservedToolJob,ArtifactReservedJob,ArtifactToolCompletion};
use semio_framework_job::{StepContext,OriginalTurnFailure,InteractiveJob,InteractiveJobCloseStep};
use semio_framework_value::{DslValue,RetirementDemand,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::RetainedCloneClose;
use semio_framework_pack_json::ArtifactCanonicalJsonTreeWireCursor;
use semio_framework_plugin_instance_stores::stores_command_routes::AuthoringSeedWork;
use std::mem::{ManuallyDrop,size_of};

type ActionWire=ArtifactCanonicalJsonTreeWireCursor<(String,Option<DslValue>)>;
pub struct ReservedPreparation<A:ArtifactApp>{
 pub source:semio_framework::action_bus::OriginalToolSourceBinding,
 pub decoder:(u64,u64),
 pub revision:[u8;32],
 pub metadata:ManuallyDrop<Option<[Vec<u8>;4]>>,
 metadata_sources:[[(usize,usize);2];4],
 metadata_bound:[bool;4],
 metadata_transferred:[bool;4],
 pub seed:Option<AuthoringSeedWork>,
 pub seed_output:ManuallyDrop<Option<String>>,
 pub completion_birth:OriginalCompletionBirth<A>,
 pub completion:ManuallyDrop<Option<ArtifactToolCompletion<A>>>,
 pub action_wire:ManuallyDrop<Option<ActionWire>>,
 pub action_returned:ManuallyDrop<Option<(String,Option<DslValue>)>>,
 pub raw_wire:ManuallyDrop<Option<Vec<u8>>>,
 pub input:ManuallyDrop<Option<ArtifactReservedToolInput>>,
 pub request:ManuallyDrop<Option<ArtifactReservedToolJobRequest<A>>>,
 pub body:ManuallyDrop<Option<Box<dyn ArtifactReservedJob+Send>>>,
 pub job:ManuallyDrop<Option<ArtifactReservedToolJob>>,
 retirement:RetainedCloneClose,
 closing:bool,
}
impl<A:ArtifactApp> ReservedPreparation<A>{
 pub fn slot_close_demands(slot:&Option<Self>,body:usize)->Result<RetirementDemand,ValueError>{match slot.as_ref(){None=>Ok(Default::default()),Some(owner)if owner.terminal_is_empty()=>Ok(RetirementDemand{copy_bytes:size_of::<Option<Self>>(),depth:1,..Default::default()}),Some(owner)=>owner.close_demands(body)}}
 pub fn close_slot(slot:&mut Option<Self>,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
  let Some(owner)=slot.as_mut()else{return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})};owner.validate(owner.revision,cx)?;if !owner.terminal_is_empty(){return owner.close_step(cx)}let bytes=size_of::<Option<Self>>();semio_framework_job::close_original_retained_step(cx,RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()},|_|{drop(slot.take());RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})),work:1}})
 }
 pub fn admit(slot:&mut Option<Self>,revision:[u8;32],cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  cx.original_source_parent()?;
  if let Some(owner)=slot.as_ref(){owner.validate(revision,cx)?;return Ok(true)}
  let bytes=size_of::<Self>();let source=semio_framework::action_bus::OriginalToolSourceBinding::borrow(cx);
  semio_framework_job::admit_original_construction(cx,slot,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:bytes,capacity_bytes:0,release_bytes:0,depth:1},|control|{
   let result=control.retirement_recipient_identity().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"reserved preparation requires its original decoder recipient")).map(|recipient|(Self{source,decoder:(control.ledger_identity(),recipient),revision,metadata:ManuallyDrop::new(Some(Default::default())),metadata_sources:[[(0,0);2];4],metadata_bound:[false;4],metadata_transferred:[false;4],seed:None,seed_output:ManuallyDrop::new(None),completion_birth:Default::default(),completion:ManuallyDrop::new(None),action_wire:ManuallyDrop::new(None),action_returned:ManuallyDrop::new(None),raw_wire:ManuallyDrop::new(None),input:ManuallyDrop::new(None),request:ManuallyDrop::new(None),body:ManuallyDrop::new(None),job:ManuallyDrop::new(None),retirement:Default::default(),closing:false},RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}));
   RetainedWorkOutcome{result,work:1}
  })
 }
 pub fn validate(&self,revision:[u8;32],cx:&StepContext<'_>)->Result<(),ValueError>{if !self.matches(cx)||self.revision!=revision{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"reserved preparation requires its original caller, decoder and unchanged document revision"))}Ok(())}
 pub fn matches(&self,cx:&StepContext<'_>)->bool{self.source.matches(cx)&&cx.native_receiving_identity()==Some(self.decoder)}
 pub fn copy_metadata(&mut self,field:usize,parts:[&str;2],cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(self.revision,cx)?;if field>=4{return Err(ValueError::invariant("reserved metadata field exceeds its inventory").into())}
  let identities=parts.map(|part|(part.as_ptr()as usize,part.len()));if self.metadata_bound[field]&&self.metadata_sources[field]!=identities{return Err(ValueError::invariant("reserved metadata source changed during its granted copy").into())}
  if self.metadata_transferred[field]{return Ok(true)}
  let length=parts[0].len().checked_add(parts[1].len()).ok_or_else(||ValueError::invariant("reserved metadata extent overflow"))?;let target=&mut self.metadata.as_mut().ok_or_else(||ValueError::invariant("reserved metadata already transferred"))?[field];
  if target.len()==length&&self.metadata_bound[field]{return Ok(true)}let grant=cx.retained_grant();if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}
  if !self.metadata_bound[field]{if grant.maximum_capacity_bytes<length{return Ok(false)}target.try_reserve_exact(length).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"reserved metadata backing allocation refused"))?;self.metadata_sources[field]=identities;self.metadata_bound[field]=true;cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:target.capacity(),..Default::default()})?;return Ok(false)}
  let count=(length-target.len()).min(4096).min(grant.maximum_copy_bytes).min(cx.fuel_remaining()as usize);if count==0{return Ok(false)}let mut left=count;let mut offset=target.len();for part in parts{if offset>=part.len(){offset-=part.len();continue}let copied=left.min(part.len()-offset);target.extend_from_slice(&part.as_bytes()[offset..offset+copied]);left-=copied;offset=0;if left==0{break}}
  cx.consume_fuel(count as u64);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:count,..Default::default()})?;Ok(target.len()==length)
 }
 pub fn transfer_metadata(&mut self,field:usize,output:&mut String,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(self.revision,cx)?;if field>=4{return Err(ValueError::invariant("reserved metadata transfer exceeds its inventory").into())}if self.metadata_transferred[field]{return Ok(true)}
  let target=&mut self.metadata.as_mut().ok_or_else(||ValueError::invariant("reserved metadata was already retired"))?[field];let length=self.metadata_sources[field].iter().map(|(_,length)|*length).sum::<usize>();if !self.metadata_bound[field]||target.len()!=length{return Err(ValueError::invariant("reserved metadata transfer requires its complete original bytes").into())}if output.capacity()!=0{return Err(ValueError::invariant("reserved metadata receiving string is occupied").into())}
  let bytes=2*size_of::<String>();let grant=cx.retained_grant();if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}
  *output=unsafe{String::from_utf8_unchecked(std::mem::take(target))};self.metadata_transferred[field]=true;cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;Ok(true)
 }
 pub fn begin_action(&mut self,args:&mut Option<DslValue>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(self.revision,cx)?;if self.action_wire.is_some()||self.action_returned.is_some(){return Ok(true)}if !self.metadata_bound[1]||self.metadata.as_ref().unwrap()[1].len()!=self.metadata_sources[1].iter().map(|(_,length)|*length).sum::<usize>(){return Err(ValueError::invariant("reserved action verb is not completely admitted").into())}
  let bytes=size_of::<ActionWire>()+size_of::<Option<DslValue>>();let grant=cx.retained_grant();if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}
  let action=std::mem::take(&mut self.metadata.as_mut().unwrap()[1]);let action=unsafe{String::from_utf8_unchecked(action)};self.metadata_transferred[1]=true;*self.action_wire=Some(ActionWire::new((action,args.take())));cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;Ok(true)
 }
 pub fn advance_action_wire(&mut self,maximum:usize,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(self.revision,cx)?;if self.raw_wire.is_some(){return Ok(true)}let owner=self.action_wire.as_mut().ok_or_else(||ValueError::invariant("reserved action wire has no original source"))?;
  if owner.phase()=="allocate"&&owner.measured_byte_length()>maximum{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"reserved action wire exceeds its declared input bound").into())}
  if owner.phase()=="sourceClose"{let turn=owner.return_original(&mut self.action_returned,cx.retained_grant());semio_framework_job::receive_original_retained_work(cx,turn)?;return Ok(false)}
  let result=owner.advance(cx.retained_grant());let work=owner.last_turn_work();let result=result.map(|step|{*self.raw_wire=step.output;RetainedCloneStep::Progress(step.progress)});semio_framework_job::receive_original_retained_work(cx,RetainedWorkOutcome{result,work})?;Ok(self.raw_wire.is_some())
 }
 pub fn advance_builder(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(self.revision,cx)?;if self.job.is_some(){return Ok(true)}if self.body.is_none(){A::build_reserved_tool_job(&mut self.request,&mut self.body,cx)?;return Ok(false)}ArtifactReservedToolJob::admit_boxed_original(&mut self.body,&mut self.job,cx)
 }
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  let header=RetirementDemand{depth:1,..Default::default()};
  if let Some(job)=self.job.as_ref(){return if job.terminal_is_empty(){Ok(header)}else{Ok(RetirementDemand{copy_bytes:job.next_close_copy_byte_demand()?,capacity_bytes:job.next_close_capacity_byte_demand(body)?,release_bytes:job.next_close_release_byte_demand()?,depth:job.next_close_depth_demand()?})}}
  if let Some(job)=self.body.as_ref(){return if job.terminal_is_empty(){Ok(RetirementDemand{release_bytes:std::mem::size_of_val(&**job),..header})}else{Ok(RetirementDemand{copy_bytes:job.next_close_copy_byte_demand()?,capacity_bytes:job.next_close_capacity_byte_demand(body)?,release_bytes:job.next_close_release_byte_demand()?,depth:job.next_close_depth_demand()?})}}
  if let Some(wire)=self.action_wire.as_ref(){return if wire.terminal_is_empty(){Ok(header)}else{wire.retirement_demands(body)}}
  if !self.completion_birth.terminal_is_empty(){return Ok(self.completion_birth.close_demands())}
  if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<()>(false,&None,body)}
  macro_rules! lane{($field:ident,$type:ty)=>{if self.$field.is_some(){return self.retirement.next_owner_demands::<$type>(true,&None,body)}};}
  lane!(request,ArtifactReservedToolJobRequest<A>);lane!(input,ArtifactReservedToolInput);lane!(action_returned,(String,Option<DslValue>));lane!(raw_wire,Vec<u8>);lane!(metadata,[Vec<u8>;4]);lane!(seed_output,String);lane!(completion,ArtifactToolCompletion<A>);
  Ok(header)
 }
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
  self.validate(self.revision,cx)?;self.closing=true;
  if let Some(job)=self.job.as_mut(){if !job.terminal_is_empty(){job.begin_close();return job.close_step(cx)}return semio_framework_job::close_original_retained_step(cx,RetirementDemand{depth:1,..Default::default()},|_|{self.job.take();RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}})}
  if let Some(job)=self.body.as_mut(){if !job.terminal_is_empty(){job.begin_close();return job.close_step(cx)}let released_bytes=std::mem::size_of_val(&**job);return semio_framework_job::close_original_retained_step(cx,RetirementDemand{release_bytes:released_bytes,depth:1,..Default::default()},|_|{self.body.take();RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes,..Default::default()})),work:1}})}
  if let Some(wire)=self.action_wire.as_mut(){if !wire.terminal_is_empty(){let demand=wire.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;return semio_framework_job::close_original_retained_step(cx,demand,|grant|{let result=wire.close_step(grant).map(RetainedCloneStep::Progress);RetainedWorkOutcome{result,work:wire.last_turn_work()}})}return semio_framework_job::close_original_retained_step(cx,RetirementDemand{depth:1,..Default::default()},|_|{self.action_wire.take();RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}})}
  if !self.completion_birth.terminal_is_empty(){return self.completion_birth.close_step(cx)}
  let demand=self.close_demands(cx.retained_grant().maximum_copy_bytes)?;
  semio_framework_job::close_original_retained_step(cx,demand,|grant|{
   if !self.retirement.is_empty(){return self.retirement.step_granted(grant)}
   macro_rules! lane{($field:ident)=>{if self.$field.is_some(){let turn=self.retirement.begin_granted(&mut self.$field,grant);return RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Complete(Default::default()))),work:turn.work}}};}
   lane!(request);lane!(input);lane!(action_returned);lane!(raw_wire);lane!(metadata);lane!(seed_output);lane!(completion);
   self.seed=None;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}
  })
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.metadata.is_none()&&self.seed.is_none()&&self.seed_output.is_none()&&self.completion_birth.terminal_is_empty()&&self.completion.is_none()&&self.action_wire.is_none()&&self.action_returned.is_none()&&self.raw_wire.is_none()&&self.input.is_none()&&self.request.is_none()&&self.body.is_none()&&self.job.is_none()&&self.retirement.is_empty()}
}
impl<A:ArtifactApp> Drop for ReservedPreparation<A>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"reserved preparation retains original sources or admission receivers before funded close");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.metadata);ManuallyDrop::drop(&mut self.seed_output);ManuallyDrop::drop(&mut self.completion);ManuallyDrop::drop(&mut self.action_wire);ManuallyDrop::drop(&mut self.action_returned);ManuallyDrop::drop(&mut self.raw_wire);ManuallyDrop::drop(&mut self.input);ManuallyDrop::drop(&mut self.request);ManuallyDrop::drop(&mut self.body);ManuallyDrop::drop(&mut self.job);}}}}
