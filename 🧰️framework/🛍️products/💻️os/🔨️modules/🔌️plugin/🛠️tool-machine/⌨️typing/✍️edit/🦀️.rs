//! ⌨️ The accepted typing emission retains its actual fields until one atomic ledger and emission handoff.
use semio_framework_plugin_decl_app::{ArtifactApp,tool_contract::{OriginalTypingFoldPolicy,ArtifactOriginalTypingFoldProducer}};
use semio_framework_plugin_decl_emission::Emit;
use semio_framework_plugin_tool_run::tool_jobs_tool_machine::TypingTag;
use semio_framework_tool_machine::{TypingLedger,ToolStep};
use semio_framework_tool_typing_edit::{OriginalTypingEdit,OriginalTypingEditRequest,OriginalTypingEditResult,OriginalTypingFold};
use semio_framework_job::{StepContext,OriginalSourceBinding,OriginalTurnFailure,OriginalConstructionDemand,admit_original_construction,close_original_retained_step};
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,ValueError,ValueRefusalKind,retirement::controlled::ControlledRetirement};
use semio_framework_os_kernel as protocol;
use semio_framework::kernel::HybridLogicalTimestamp;
use std::mem::size_of;
/// 🧬️ The portable inlet contract describes actual owner admission and output custody.
pub const ORIGINAL_TYPING_SETTLEMENT_SCHEMA:&str=include_str!("🧬️schema/🔣️.json");

#[derive(semio_framework_value::RetireOwned)]
struct Owners<M>{request:Option<OriginalTypingEditRequest<M>>,fold:Option<OriginalTypingFold<M>>,result:Option<OriginalTypingEditResult<M>>,empty:Option<Vec<M>>,previous:Option<protocol::TransactionRef>,unused:Option<protocol::TransactionRef>}
/// 🧵️ One native original source owns the actual tag, incoming vector and optional previous transaction through settlement.
pub struct OriginalTypingSettlement<A:ArtifactApp>{binding:OriginalSourceBinding,decoder:(u64,u64),operation:u64,base:[u8;32],actor:(usize,usize),backing:[(usize,usize);4],lengths:[usize;4],body:usize,published:bool,received:bool,closing:bool,policy:OriginalTypingFoldPolicy<A::Mutation>,writer:Option<Box<OriginalTypingEdit<A::Mutation>>>,producer:Option<Box<dyn ArtifactOriginalTypingFoldProducer<A::Mutation>>>,owners:ControlledRetirement<Owners<A::Mutation>>}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original typing settlement changed its source, emission or native receiver")}
fn construction(d:RetirementDemand)->OriginalConstructionDemand{OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
fn fixed<T>()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<T>(),depth:1,..Default::default()}}
fn backing<M,CM,D>(emit:&Emit<M,CM,D>)->[(usize,usize);4]{[(emit.artifact_mutations.as_ptr()as usize,emit.artifact_mutations.capacity()),(emit.child_emits.as_ptr()as usize,emit.child_emits.capacity()),(emit.config_mutations.as_ptr()as usize,emit.config_mutations.capacity()),(emit.window_config_mutations.as_ptr()as usize,emit.window_config_mutations.capacity())]}
fn lengths<M,CM,D>(emit:&Emit<M,CM,D>)->[usize;4]{[emit.artifact_mutations.len(),emit.child_emits.len(),emit.config_mutations.len(),emit.window_config_mutations.len()]}
fn child_grant(cx:&StepContext<'_>,d:RetirementDemand,extra:usize)->Option<RetainedCloneGrant>{let g=cx.retained_grant();if cx.should_yield()||cx.is_cancelled()||g.maximum_items==0||g.maximum_copy_bytes<d.copy_bytes.checked_add(extra)?||g.maximum_capacity_bytes<d.capacity_bytes||g.maximum_release_bytes<d.release_bytes||g.maximum_depth<d.depth{return None}Some(RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-extra,..g})}
impl<A:ArtifactApp> OriginalTypingSettlement<A>{
 /// 🌱️ Whole original fields move only after the native decoder funds the owner and the actual actor bytes.
 pub fn admit<D>(operation:u64,base:&[u8;32],actor:&str,emit:&mut Emit<A::Mutation,A::ConfigMutation,D>,tag_copy_bytes:usize,receive:impl FnOnce()->(TypingTag,HybridLogicalTimestamp),cx:&mut StepContext<'_>,output:&mut Option<Box<Self>>)->Result<bool,OriginalTurnFailure>{
  if output.is_some(){return Err(invalid().into())}
  let d=RetirementDemand{copy_bytes:2*size_of::<Self>()+2*size_of::<OriginalTypingEditRequest<A::Mutation>>()+2*size_of::<TypingTag>()+actor.len()+tag_copy_bytes,capacity_bytes:size_of::<Self>().checked_add(actor.len()).ok_or_else(invalid)?,depth:1,..Default::default()};
  let binding=OriginalSourceBinding::borrow(cx);let body=cx.retained_grant().maximum_copy_bytes;let actor_identity=(actor.as_ptr()as usize,actor.len());let base=*base;
  admit_original_construction(cx,output,construction(d),|native|{
   let decoder=(native.ledger_identity(),native.retirement_recipient_identity().ok_or_else(invalid)?);let mut text=String::with_capacity(actor.len());text.push_str(actor);let actual=size_of::<Self>()+text.capacity();let(TypingTag{window,tool,buffer},clock)=receive();
   let request=OriginalTypingEditRequest{window,tool,actor:protocol::ActorId(text.into()),buffer,leaves:std::mem::take(&mut emit.artifact_mutations),clock};
   Ok((Box::new(Self{binding,decoder,operation,base,actor:actor_identity,backing:backing(emit),lengths:lengths(emit),body,published:false,received:false,closing:false,policy:A::original_typing_fold(),writer:None,producer:None,owners:ControlledRetirement::retain(Owners{request:Some(request),fold:None,result:None,empty:None,previous:None,unused:None})}),RetainedCloneProgress{retained_capacity_bytes:actual,..receipt(d)}))
  })
 }
 pub fn operation(&self)->u64{self.operation}
 pub fn matches_context(&self,cx:&StepContext<'_>)->bool{self.binding.matches(cx)&&cx.native_receiving_identity()==Some(self.decoder)}
 pub fn accepts<D>(&self,operation:u64,base:&[u8;32],actor:&str,emit:&Emit<A::Mutation,A::ConfigMutation,D>)->bool{self.operation==operation&&self.base==*base&&self.actor==(actor.as_ptr()as usize,actor.len())&&self.backing==backing(emit)&&self.lengths==lengths(emit)}
 pub fn received(&self)->bool{self.received}
 pub fn result(&self)->bool{self.published}
 fn validate(&self,cx:&StepContext<'_>)->Result<(),ValueError>{if !self.matches_context(cx){return Err(invalid())}Ok(())}
 fn producer_release_demands(&self)->RetirementDemand{RetirementDemand{release_bytes:self.producer.as_ref().map_or(0,|owner|std::mem::size_of_val(&**owner)),..fixed::<Option<Box<dyn ArtifactOriginalTypingFoldProducer<A::Mutation>>>>()}}
 fn close_producer(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let Some(owner)=self.producer.as_mut()else{return Ok(true)};
  if !owner.terminal_is_empty(){owner.close_step(cx)?;return Ok(false)}
  let d=self.producer_release_demands();close_original_retained_step(cx,d,|_|{self.producer.take();Ok(RetainedCloneStep::Complete(receipt(d)))})?;Ok(self.producer.is_none())
 }
 /// ▶️ Every fold producer settles and closes before the genuine old or incoming vectors can move.
 pub fn advance<D>(&mut self,ledger:&mut TypingLedger<A::Mutation>,operation:u64,base:&[u8;32],actor:&str,emit:&mut Emit<A::Mutation,A::ConfigMutation,D>,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
  self.validate(cx)?;if !self.accepts(operation,base,actor,emit){return Err(invalid().into())}
  if self.received||self.closing||cx.is_cancelled(){self.close_step(ledger,cx)?;return Ok(())}
  let owners=self.owners.original_mut().ok_or_else(invalid)?;
  if self.writer.is_none(){OriginalTypingEdit::admit(ledger,&mut owners.request,&mut self.writer,cx)?;return Ok(())}
  let writer=self.writer.as_mut().unwrap();
  if owners.fold.is_some(){if self.producer.is_some(){self.close_producer(cx)?;return Ok(())}writer.accept_fold(&mut owners.fold,cx)?;return Ok(())}
  if let Some((previous,incoming))=writer.fold_inputs(ledger,cx)?{
   match &self.policy{
    OriginalTypingFoldPolicy::Split|OriginalTypingFoldPolicy::ReplaceIncoming=>{let d=fixed::<OriginalTypingFold<A::Mutation>>();admit_original_construction(cx,&mut owners.fold,construction(d),|_|Ok((if matches!(self.policy,OriginalTypingFoldPolicy::Split){OriginalTypingFold::Split}else{OriginalTypingFold::ReplaceIncoming},receipt(d))))?;},
    OriginalTypingFoldPolicy::Factory(factory)=>{
     if self.producer.is_none(){factory.admit(previous,incoming,cx,&mut self.producer)?;return Ok(())}
     let producer=self.producer.as_mut().unwrap();if producer.ready(){producer.take_into(&mut owners.fold,cx)?;}else{producer.advance(previous,incoming,cx)?;}
    }
   }return Ok(())
  }
  if !writer.ready(){writer.advance(ledger,cx)?;return Ok(())}
  let extra=2*size_of::<OriginalTypingEditResult<A::Mutation>>()+4*size_of::<Option<protocol::TransactionRef>>()+2*size_of::<Vec<A::Mutation>>()+size_of::<bool>()+size_of::<[(usize,usize);4]>()+size_of::<[usize;4]>();
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
  if let Some(owner)=self.producer.as_ref(){return if owner.terminal_is_empty(){Ok(self.producer_release_demands())}else{owner.close_demands(body)}}
  if let Some(owner)=self.writer.as_ref(){return if owner.terminal_is_empty(){Ok(owner.shell_close_demands())}else{owner.close_demands(body)}}
  self.owners.next_demand(body)
 }
 /// 🪦️ A cancelled prepublication edit closes its original input while the resident ledger remains unchanged.
 pub fn close_step(&mut self,ledger:&mut TypingLedger<A::Mutation>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(cx)?;self.closing=true;if self.terminal_is_empty(){return Ok(true)}
  if self.producer.is_some(){self.close_producer(cx)?;return Ok(false)}
  if let Some(owner)=self.writer.as_mut(){if !owner.terminal_is_empty(){owner.close_step(ledger,cx)?;return Ok(false)}let d=owner.shell_close_demands();close_original_retained_step(cx,d,|_|{self.writer.take();Ok(RetainedCloneStep::Complete(receipt(d)))})?;return Ok(false)}
  let d=self.owners.next_demand(self.body)?;close_original_retained_step(cx,d,|g|self.owners.step(g))?;Ok(self.terminal_is_empty())
 }
 pub fn terminal_is_empty(&self)->bool{self.producer.is_none()&&self.writer.is_none()&&self.owners.terminal_is_empty()}
 pub fn shell_close_demands(&self)->RetirementDemand{RetirementDemand{release_bytes:size_of::<Self>(),..fixed::<Option<Box<Self>>>()}}
}
impl<A:ArtifactApp> Drop for OriginalTypingSettlement<A>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original typing settlement requires controlled physical closure")}}
