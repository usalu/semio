//! ⌨️ The accepted typing emission retains its actual fields until one atomic ledger and emission handoff.
use semio_plugin_decl_app::{ArtifactApp,tool_contract::{OriginalTypingFoldPolicy,ArtifactOriginalTypingFoldProducer}};
use semio_plugin_decl_emission::Emit;
use semio_plugin_tool_run::tool_jobs_tool_machine::TypingTag;
use semio_framework_tool_machine::{TypingLedger,ToolStep};
use semio_framework_tool_typing_edit::{OriginalTypingEdit,OriginalTypingEditRequest,OriginalTypingEditResult,OriginalTypingFold};
use semio_framework_job::{StepContext,OriginalSourceBinding,OriginalCallerSourceLoan,OriginalTurnFailure,OriginalConstructionDemand,admit_original_construction,close_original_retained_step};
use semio_framework_value::{RetainedWorkOutcome,NativeDecodeControl,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,ValueError,ValueRefusalKind,retirement::controlled::ControlledRetirement};
use semio_os_kernel as protocol;
use semio_framework::kernel::HybridLogicalTimestamp;
use std::mem::{ManuallyDrop,size_of};
/// 🧬️ The portable inlet contract describes actual owner admission and output custody.
pub const ORIGINAL_TYPING_SETTLEMENT_SCHEMA:&str=include_str!("🧬️schema/🔣️.json");

#[derive(semio_framework_value::RetireOwned)]
struct Owners<M>{request:Option<OriginalTypingEditRequest<M>>,fold:Option<OriginalTypingFold<M>>,result:Option<OriginalTypingEditResult<M>>,empty:Option<Vec<M>>,previous:Option<protocol::TransactionRef>,unused:Option<protocol::TransactionRef>}
/// 🧵️ One native original source owns the actual tag, incoming vector and optional previous transaction through settlement.
pub struct OriginalTypingSettlement<A:ArtifactApp>{binding:OriginalSourceBinding,decoder:(u64,u64),operation:u64,base:[u8;32],actor:(usize,usize),backing:[(usize,usize);4],lengths:[usize;4],body:usize,published:bool,received:bool,captured:bool,closing:bool,caller:ManuallyDrop<Option<OriginalCallerSourceLoan>>,policy:OriginalTypingFoldPolicy<A::Mutation>,writer:Option<Box<OriginalTypingEdit<A::Mutation>>>,producer:Option<Box<dyn ArtifactOriginalTypingFoldProducer<A::Mutation>>>,owners:ControlledRetirement<Owners<A::Mutation>>}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original typing settlement changed its source, emission or native receiver")}
fn construction(d:RetirementDemand)->OriginalConstructionDemand{OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
fn work(result:Result<RetainedCloneStep,ValueError>)->RetainedWorkOutcome{RetainedWorkOutcome{result,work:1}}
fn built<O>(cx:&mut StepContext<'_>,output:&mut Option<O>,demand:OriginalConstructionDemand,construct:impl FnOnce(&mut NativeDecodeControl<'_>)->Result<(O,RetainedCloneProgress),ValueError>)->Result<bool,OriginalTurnFailure>{admit_original_construction(cx,output,demand,|control|RetainedWorkOutcome{result:construct(control),work:1})}
fn fixed<T>()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<T>(),depth:1,..Default::default()}}
fn backing<M,CM,D>(emit:&Emit<M,CM,D>)->[(usize,usize);4]{[(emit.artifact_mutations.as_ptr()as usize,emit.artifact_mutations.capacity()),(emit.child_emits.as_ptr()as usize,emit.child_emits.capacity()),(emit.config_mutations.as_ptr()as usize,emit.config_mutations.capacity()),(emit.window_config_mutations.as_ptr()as usize,emit.window_config_mutations.capacity())]}
fn lengths<M,CM,D>(emit:&Emit<M,CM,D>)->[usize;4]{[emit.artifact_mutations.len(),emit.child_emits.len(),emit.config_mutations.len(),emit.window_config_mutations.len()]}
fn child_grant(cx:&StepContext<'_>,d:RetirementDemand,extra:usize)->Option<RetainedCloneGrant>{let g=cx.retained_grant();if cx.should_yield()||cx.is_cancelled()||g.maximum_items==0||g.maximum_copy_bytes<d.copy_bytes.checked_add(extra)?||g.maximum_capacity_bytes<d.capacity_bytes||g.maximum_release_bytes<d.release_bytes||g.maximum_depth<d.depth{return None}Some(RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-extra,..g})}
impl<A:ArtifactApp> OriginalTypingSettlement<A>{
 /// 🌱️ Whole original fields move only after the native decoder funds the owner and the actual actor bytes.
 pub fn admit<D>(operation:u64,base:&[u8;32],actor:&str,emit:&mut Emit<A::Mutation,A::ConfigMutation,D>,tag_copy_bytes:usize,receive:impl FnOnce()->(TypingTag,HybridLogicalTimestamp),cx:&mut StepContext<'_>,output:&mut Option<Box<Self>>)->Result<bool,OriginalTurnFailure>{
  if let Some(owner)=output.as_mut(){return owner.capture(operation,base,actor,emit,tag_copy_bytes,receive,cx)}
  let d=RetirementDemand{capacity_bytes:size_of::<Self>(),..fixed::<Self>()};let binding=OriginalSourceBinding::borrow(cx);let body=cx.retained_grant().maximum_copy_bytes;let actor_identity=(actor.as_ptr()as usize,actor.len());let base=*base;
  built(cx,output,construction(d),|native|{let decoder=(native.ledger_identity(),native.retirement_recipient_identity().ok_or_else(invalid)?);Ok((Box::new(Self{binding,decoder,operation,base,actor:actor_identity,backing:backing(emit),lengths:lengths(emit),body,published:false,received:false,captured:false,closing:false,caller:ManuallyDrop::new(None),policy:A::original_typing_fold(),writer:None,producer:None,owners:ControlledRetirement::retain(Owners{request:None,fold:None,result:None,empty:None,previous:None,unused:None})}),receipt(d)))})?;Ok(false)
 }
 /// 🧱️ Actor capture funds both the copied UTF8 allocation and the canonical shared ownership frame.
 pub fn capture_demands(actor:&str,tag_copy_bytes:usize)->Result<RetirementDemand,ValueError>{
  let shared=semio_framework_value::SharedUtf8::admission_demand();let capacity=actor.len().checked_add(shared.capacity_bytes).ok_or_else(invalid)?;
  Ok(RetirementDemand{copy_bytes:2*size_of::<OriginalTypingEditRequest<A::Mutation>>()+2*size_of::<TypingTag>()+actor.len()+tag_copy_bytes+2*size_of::<[(usize,usize);4]>()+2*size_of::<[usize;4]>()+size_of::<bool>()+shared.copy_bytes,capacity_bytes:capacity,depth:1,..Default::default()})
 }
 fn capture<D>(&mut self,operation:u64,base:&[u8;32],actor:&str,emit:&mut Emit<A::Mutation,A::ConfigMutation,D>,tag_copy_bytes:usize,receive:impl FnOnce()->(TypingTag,HybridLogicalTimestamp),cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(cx)?;if self.closing||!self.accepts(operation,base,actor,emit){return Err(invalid().into())}if self.captured{return Ok(true)}
  if self.caller.is_none(){if !cx.original_source_parent()?.admit_loan(&mut *self.caller,cx)?{return Ok(false)}}
  let owners=self.owners.original_mut().ok_or_else(invalid)?;let d=Self::capture_demands(actor,tag_copy_bytes)?;let grant=cx.retained_grant();
  built(cx,&mut owners.request,construction(d),|_|{let mut text=String::with_capacity(actor.len());text.push_str(actor);let text_capacity=text.capacity();let(actor_owner,actor_progress)=semio_framework_value::SharedUtf8::admit(text,grant).result.unwrap_or_else(|_|unreachable!("original typing actor frame was preadmitted"));let actual=text_capacity+actor_progress.retained_capacity_bytes;let(TypingTag{window,tool,buffer},clock)=receive();let request=OriginalTypingEditRequest{window,tool,actor:protocol::ActorId(actor_owner),buffer,leaves:std::mem::take(&mut emit.artifact_mutations),clock};self.backing=backing(emit);self.lengths=lengths(emit);self.captured=true;Ok((request,RetainedCloneProgress{retained_capacity_bytes:actual,..receipt(d)}))})?;Ok(self.captured)
 }
 pub fn captured(&self)->bool{self.captured}
 pub fn operation(&self)->u64{self.operation}
 pub fn matches_context(&self,cx:&StepContext<'_>)->bool{self.binding.matches(cx)&&cx.native_receiving_identity()==Some(self.decoder)}
 pub fn accepts<D>(&self,operation:u64,base:&[u8;32],actor:&str,emit:&Emit<A::Mutation,A::ConfigMutation,D>)->bool{self.operation==operation&&self.base==*base&&self.actor==(actor.as_ptr()as usize,actor.len())&&self.backing==backing(emit)&&self.lengths==lengths(emit)}
 pub fn received(&self)->bool{self.received}
 pub fn result(&self)->bool{self.published}
 fn validate(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{if !self.matches_context(cx){return Err(invalid().into())}if let Some(caller)=self.caller.as_ref(){caller.validate_context(cx)?}Ok(())}
 fn producer_release_demands(&self)->RetirementDemand{RetirementDemand{release_bytes:self.producer.as_ref().map_or(0,|owner|std::mem::size_of_val(&**owner)),..fixed::<Option<Box<dyn ArtifactOriginalTypingFoldProducer<A::Mutation>>>>()}}
 fn close_producer(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let Some(owner)=self.producer.as_mut()else{return Ok(true)};
  if !owner.terminal_is_empty(){owner.close_step(cx)?;return Ok(false)}
  let d=self.producer_release_demands();close_original_retained_step(cx,d,|_|{self.producer.take();work(Ok(RetainedCloneStep::Complete(receipt(d))))})?;Ok(self.producer.is_none())
 }
 /// ▶️ Every fold producer settles and closes before the genuine old or incoming vectors can move.
 pub fn advance<D>(&mut self,ledger:&mut TypingLedger<A::Mutation>,operation:u64,base:&[u8;32],actor:&str,emit:&mut Emit<A::Mutation,A::ConfigMutation,D>,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
  if !self.matches_context(cx)||!self.accepts(operation,base,actor,emit){return Err(invalid().into())}
  if self.received||self.closing||cx.is_cancelled(){self.close_step(ledger,cx)?;return Ok(())}
  self.validate(cx)?;if !self.captured{return Err(invalid().into())}
  let owners=self.owners.original_mut().ok_or_else(invalid)?;
  if self.writer.is_none(){OriginalTypingEdit::admit(ledger,&mut owners.request,&mut self.writer,cx)?;return Ok(())}
  let writer=self.writer.as_mut().unwrap();
  if owners.fold.is_some(){if self.producer.is_some(){self.close_producer(cx)?;return Ok(())}writer.accept_fold(&mut owners.fold,cx)?;return Ok(())}
  if let Some((previous,incoming))=writer.fold_inputs(ledger,cx)?{
   match &self.policy{
    OriginalTypingFoldPolicy::Split|OriginalTypingFoldPolicy::ReplaceIncoming=>{let d=fixed::<OriginalTypingFold<A::Mutation>>();built(cx,&mut owners.fold,construction(d),|_|Ok((if matches!(self.policy,OriginalTypingFoldPolicy::Split){OriginalTypingFold::Split}else{OriginalTypingFold::ReplaceIncoming},receipt(d))))?;},
    OriginalTypingFoldPolicy::Factory(factory)=>{
     if self.producer.is_none(){factory.admit(previous,incoming,cx,&mut self.producer)?;return Ok(())}
     let producer=self.producer.as_mut().unwrap();if producer.ready(){producer.take_into(&mut owners.fold,cx)?;}else{producer.advance(previous,incoming,cx)?;}
    }
   }return Ok(())
  }
  if !writer.ready(){writer.advance(ledger,cx)?;return Ok(())}
  let extra=2*size_of::<OriginalTypingEditResult<A::Mutation>>()+4*size_of::<Option<protocol::TransactionRef>>()+2*size_of::<Vec<A::Mutation>>()+2*size_of::<bool>()+2*size_of::<[(usize,usize);4]>()+2*size_of::<[usize;4]>();
  let d=writer.take_result_demands(ledger)?;let Some(g)=child_grant(cx,d,extra)else{return Ok(())};
  if cx.with_retained_loan(g,|same|writer.take_result(ledger,&mut owners.result,same))?{
   let result=owners.result.as_mut().ok_or_else(invalid)?;
   match result.previous.take(){
    Some(ToolStep::Committed(transaction,leaves))=>{owners.previous=emit.transaction.replace(transaction);owners.empty=Some(std::mem::replace(&mut emit.artifact_mutations,leaves));self.published=true;},
    Some(ToolStep::Empty(transaction))=>owners.unused=Some(transaction),
    None=>{},
    other=>{result.previous=other;return Err(invalid().into())}
   }
   self.backing=backing(emit);self.lengths=lengths(emit);self.received=true;
   cx.consume_retained(RetainedCloneProgress{copied_bytes:extra,..Default::default()})?;
  }Ok(())
 }
 /// 📏 The same source quotes the fold cursor, staged ledger owner and all genuine metadata residues in order.
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.closing{return Ok(fixed::<bool>())}
  if let Some(owner)=self.producer.as_ref(){return if owner.terminal_is_empty(){Ok(self.producer_release_demands())}else{owner.close_demands(body)}}
  if let Some(owner)=self.writer.as_ref(){return if owner.terminal_is_empty(){Ok(owner.shell_close_demands())}else{owner.close_demands(body)}}
  if !self.owners.terminal_is_empty(){return self.owners.next_demand(body)}
  if let Some(caller)=self.caller.as_ref(){return if caller.terminal_is_empty(){Ok(fixed::<Option<OriginalCallerSourceLoan>>())}else{Ok(caller.retirement_demands())}}
  Ok(Default::default())
 }
 /// 🪦️ A cancelled prepublication edit closes its original input while the resident ledger remains unchanged.
 pub fn close_step(&mut self,ledger:&mut TypingLedger<A::Mutation>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if !self.matches_context(cx){return Err(invalid().into())}if self.terminal_is_empty(){return Ok(true)}
  if let Some(caller)=self.caller.as_ref(){if !caller.terminal_is_empty(){caller.validate_retirement_context(cx)?}}
  if !self.closing{let d=fixed::<bool>();close_original_retained_step(cx,d,|_|{self.closing=true;work(Ok(RetainedCloneStep::Progress(receipt(d))))})?;return Ok(false)}
  if self.producer.is_some(){self.close_producer(cx)?;return Ok(false)}
  if let Some(owner)=self.writer.as_mut(){if !owner.terminal_is_empty(){owner.close_step(ledger,cx)?;return Ok(false)}let d=owner.shell_close_demands();close_original_retained_step(cx,d,|_|{self.writer.take();work(Ok(RetainedCloneStep::Complete(receipt(d))))})?;return Ok(false)}
  if !self.owners.terminal_is_empty(){let d=self.owners.next_demand(self.body)?;close_original_retained_step(cx,d,|g|work(self.owners.step(g)))?;return Ok(false)}
  if let Some(caller)=self.caller.as_mut(){if !caller.terminal_is_empty(){caller.close_step(cx.original_source_parent()?,cx)?;return Ok(false)}let d=fixed::<Option<OriginalCallerSourceLoan>>();close_original_retained_step(cx,d,|_|{self.caller.take();work(Ok(RetainedCloneStep::Complete(receipt(d))))})?;}Ok(self.terminal_is_empty())
 }
 pub fn terminal_is_empty(&self)->bool{self.producer.is_none()&&self.writer.is_none()&&self.owners.terminal_is_empty()&&self.caller.is_none()}
 pub fn shell_close_demands(&self)->RetirementDemand{RetirementDemand{release_bytes:size_of::<Self>(),..fixed::<Option<Box<Self>>>()}}
}
impl<A:ArtifactApp> Drop for OriginalTypingSettlement<A>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original typing settlement requires controlled physical closure")}}
