//! 🪜️ One-item command and envelope continuations borrow the unchanged original native source on every turn.
use super::{invalid,lane,ArtifactCommand,HistoryLane,SupersedeInput,Author,MutationId,TransactionRef};
use protocol::{OpBinaryReceiving,OpBinaryReceivingCursor,codec::{RetainedWireFieldDecode,RetainedWireBytesDecode}};
use semio_framework_job::{OriginalTurnFailure,StepContext};
use semio_framework_job_native_decode::{OriginalNativeDecodeSource,OriginalNativeDecodeIssued,OriginalNativeDecodeOutcome};
use semio_framework_value::{OriginalRetirementFailure, OriginalRetirementPoll, RetainedAllocationEffect, RetainedCloneGrant, RetainedCloneProgress, RetainedWorkOutcome, RetainedWorkPoll, RetainedCloneStep, RetirementDemand, RetirementPollExit, SharedUtf8, ValueError, ValueRefusalKind, retirement::{RetireOwned, controlled::ControlledRetirement}};
use semio_framework_value_native_owned::{OriginalReceivingPrimary,native_decoding::{NativeDecodeControl,NativeReceivingEffect,NativeReceivingEffectPhase,NativeReceivingEffectPoll,allocate_empty_vec}};
use semio_framework_value_native_storage::NativeOwnedStorage;
use std::mem::ManuallyDrop;

#[derive(Clone,Copy)]
enum Text{First,Second,Message,MutationId,DocumentId,DiffSchema,InverseSchema,Verb,Line}
#[derive(Clone,Copy)]
enum List{Mutations,Inputs,Authors,Dependencies,Target}
#[derive(Clone,Copy)]
enum Task{Format,Ordinal,Text(Text),Bytes(bool),Flag(u8),Scalar(u8),Count(List),Allocate(List),Loop(List),Append(List),Mutation,Nested(bool),SharedActor,Finish}
#[derive(semio_framework_value::RetireOwned)]
struct Fields<M:RetireOwned>{first:Option<String>,second:Option<String>,message:Option<String>,transaction:Option<TransactionRef>,authors:Vec<Author>,mutations:Vec<M>,inputs:Vec<SupersedeInput<M>>,replacement:Option<M>,nested:Option<Box<ArtifactCommand<M>>>,envelope:Option<protocol::MutationEnvelope>,mutation_id:Option<String>,document_id:Option<String>,actor:Option<SharedUtf8>,dependencies:Vec<MutationId>,observed:Option<MutationId>,target:Vec<String>,diff_schema:Option<String>,diff_payload:Option<protocol::CausalPayload>,inverse_schema:Option<String>,inverse_payload:Option<protocol::CausalPayload>,verb:Option<String>,line:Option<String>,output:Option<ArtifactCommand<M>>}
impl<M:RetireOwned> Fields<M>{fn empty()->Self{Self{first:None,second:None,message:None,transaction:None,authors:Vec::new(),mutations:Vec::new(),inputs:Vec::new(),replacement:None,nested:None,envelope:None,mutation_id:None,document_id:None,actor:None,dependencies:Vec::new(),observed:None,target:Vec::new(),diff_schema:None,diff_payload:None,inverse_schema:None,inverse_payload:None,verb:None,line:None,output:None}}}

enum PendingList<M:RetireOwned>{Mutations(Option<Vec<M>>),Inputs(Option<Vec<SupersedeInput<M>>>),Authors(Option<Vec<Author>>),Dependencies(Option<Vec<MutationId>>),Target(Option<Vec<String>>)}
impl<M:RetireOwned> PendingList<M>{
 fn new(list:List)->Self{match list{List::Mutations=>Self::Mutations(None),List::Inputs=>Self::Inputs(None),List::Authors=>Self::Authors(None),List::Dependencies=>Self::Dependencies(None),List::Target=>Self::Target(None)}}
 fn ready(&self)->bool{match self{Self::Mutations(v)=>v.is_some(),Self::Inputs(v)=>v.is_some(),Self::Authors(v)=>v.is_some(),Self::Dependencies(v)=>v.is_some(),Self::Target(v)=>v.is_some()}}
 fn admit(&mut self,count:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneProgress,ValueError>{match self{Self::Mutations(v)=>admit_vec(v,count,effect),Self::Inputs(v)=>admit_vec(v,count,effect),Self::Authors(v)=>admit_vec(v,count,effect),Self::Dependencies(v)=>admit_vec(v,count,effect),Self::Target(v)=>admit_vec(v,count,effect)}}
 fn publish(self,fields:&mut Fields<M>){match self{Self::Mutations(Some(v))=>fields.mutations=v,Self::Inputs(Some(v))=>fields.inputs=v,Self::Authors(Some(v))=>fields.authors=v,Self::Dependencies(Some(v))=>fields.dependencies=v,Self::Target(Some(v))=>fields.target=v,_=>unreachable!("only an admitted empty list publishes")}}
 fn release_bytes(&self)->usize{match self{Self::Mutations(v)=>v.as_ref().map_or(0,|v|v.capacity()*size_of::<M>()),Self::Inputs(v)=>v.as_ref().map_or(0,|v|v.capacity()*size_of::<SupersedeInput<M>>()),Self::Authors(v)=>v.as_ref().map_or(0,|v|v.capacity()*size_of::<Author>()),Self::Dependencies(v)=>v.as_ref().map_or(0,|v|v.capacity()*size_of::<MutationId>()),Self::Target(v)=>v.as_ref().map_or(0,|v|v.capacity()*size_of::<String>())}}
 fn release(self){match self{Self::Mutations(v)=>{if let Some(v)=v{assert!(v.is_empty());drop(v)}},Self::Inputs(v)=>{if let Some(v)=v{assert!(v.is_empty());drop(v)}},Self::Authors(v)=>{if let Some(v)=v{assert!(v.is_empty());drop(v)}},Self::Dependencies(v)=>{if let Some(v)=v{assert!(v.is_empty());drop(v)}},Self::Target(v)=>{if let Some(v)=v{assert!(v.is_empty());drop(v)}}}}
}
/// 🗂️ Allocates the exact empty vector backing inside the funded action and records its real marker.
fn admit_vec<T>(output:&mut Option<Vec<T>>,count:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneProgress,ValueError>{
 if output.is_some(){return Err(invalid("native vector admission requires an empty destination"))}
 let vector=allocate_empty_vec::<T>(count,effect.actual())?;let capacity=vector.capacity().checked_mul(size_of::<T>()).ok_or_else(||invalid("native vector extent overflow"))?;*output=Some(vector);
 Ok(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Vec<T>>(),retained_capacity_bytes:capacity,..Default::default()})
}
type Outcome=RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>;
fn invariant(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,reason)}
fn progressed(poll:RetainedWorkPoll<RetainedCloneStep,ValueError>)->(Result<RetainedCloneProgress,ValueError>,u128){match poll{RetainedWorkPoll::Pending{..}=>(Ok(Default::default()),0),RetainedWorkPoll::Complete(turn)=>(turn.result.map(|step|step.progress()),turn.work)}}
fn outcome_progress(turn:RetainedWorkOutcome<RetainedCloneStep>)->(Result<RetainedCloneProgress,ValueError>,u128){(turn.result.map(|step|step.progress()),turn.work)}
fn closed_progress(poll:OriginalRetirementPoll)->Result<RetainedCloneStep,RetirementPollExit>{RetirementPollExit::resume(poll).map(|step|RetainedCloneStep::Progress(step.progress()))}
fn move_progress(turn:RetainedWorkOutcome<RetainedCloneStep>)->Result<RetainedCloneStep,RetirementPollExit>{turn.result.map(|step|RetainedCloneStep::Progress(step.progress())).map_err(Into::into)}
fn narrow(work:u128)->u64{u64::try_from(work).unwrap_or(u64::MAX)}
fn settled(poll:OriginalRetirementPoll)->RetainedWorkOutcome<RetainedCloneStep,ValueError>{match RetirementPollExit::settled(poll){Ok(turn)=>RetainedWorkOutcome{result:turn.result.map_err(OriginalRetirementFailure::into_value_error),work:turn.work},Err(exit)=>RetainedWorkOutcome{result:Err(invariant("original command close stayed pending on a synchronous recipient")),work:match exit{RetirementPollExit::Pending(work)=>work,_=>0}}}}
fn closed_outcome(poll:OriginalRetirementPoll)->(Result<RetainedCloneProgress,ValueError>,u128){match poll{RetainedWorkPoll::Pending{..}=>(Ok(Default::default()),0),RetainedWorkPoll::Complete(turn)=>(turn.result.map(|step|step.progress()).map_err(OriginalRetirementFailure::into_value_error),turn.work)}}
fn progressed_exit(poll:RetainedWorkPoll<RetainedCloneStep,ValueError>)->Result<RetainedCloneStep,RetirementPollExit>{match poll{RetainedWorkPoll::Pending{work}=>Err(RetirementPollExit::Pending(work)),RetainedWorkPoll::Complete(turn)=>turn.result.map(|step|RetainedCloneStep::Progress(step.progress())).map_err(Into::into)}}


/// 📬️ A fixed task frontier keeps the real typed command, current field and nested mutation continuation.
pub struct RetainedArtifactCommandReceiving<M:OpBinaryReceiving>{last_work:u128,close_frame:NativeReceivingEffect<ValueError>,failed:bool,pending_list:Option<PendingList<M>>,nested_range:Option<(usize,usize,bool)>,child_storage:NativeOwnedStorage<Self>,command_storage:NativeOwnedStorage<ArtifactCommand<M>>,address:usize,extent:usize,start:usize,end:usize,position:usize,envelope_mode:bool,tasks:[Task;24],depth:usize,ordinal:u64,policy:u8,lane:u8,resolution:u8,count:usize,index:usize,timestamp:[u64;3],fields:ManuallyDrop<Option<Fields<M>>>,field:Option<RetainedWireFieldDecode>,binary:Option<RetainedWireBytesDecode>,operation:Option<M::Receiving>,child:Option<Box<Self>>,retiring:Option<ControlledRetirement<Fields<M>>>,ready:bool,closing:bool,closed:bool}

/// 📨️ The same retained task machine receives the complete nested causal envelope.
pub struct RetainedCommandEnvelopeReceiving<M:OpBinaryReceiving>(RetainedArtifactCommandReceiving<M>);

fn item()->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,..Default::default()}}
fn funds(demand:RetirementDemand,grant:RetainedCloneGrant)->bool{grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
impl<M:OpBinaryReceiving> RetainedArtifactCommandReceiving<M>{
 pub fn domain_failure(&self)->Option<&M::DomainFailure>{self.operation.as_ref().and_then(|owner|owner.domain_failure()).or_else(||self.child.as_ref().and_then(|owner|owner.domain_failure()))}
 pub fn domain_failure_demand(&self)->Result<RetirementDemand,ValueError>{if let Some(owner)=self.operation.as_ref().filter(|owner|owner.domain_failure().is_some()){return owner.domain_failure_demand()}if let Some(owner)=self.child.as_ref().filter(|owner|owner.domain_failure().is_some()){return owner.domain_failure_demand()}Ok(Default::default())}
 pub fn domain_failure_work_demand(&self)->Result<u64,ValueError>{if let Some(owner)=self.operation.as_ref().filter(|owner|owner.domain_failure().is_some()){return owner.domain_failure_work_demand()}if let Some(owner)=self.child.as_ref().filter(|owner|owner.domain_failure().is_some()){return owner.domain_failure_work_demand()}Ok(0)}
 pub fn take_domain_failure_into(&mut self,recipient:&mut Option<M::DomainFailure>,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress>{if recipient.is_some(){return RetainedWorkOutcome{result:Err(ValueError::invariant("command domain failure requires its empty original recipient")),work:0}}if let Some(owner)=self.operation.as_mut().filter(|owner|owner.domain_failure().is_some()){return owner.take_domain_failure_into(recipient,grant)}if let Some(owner)=self.child.as_mut().filter(|owner|owner.domain_failure().is_some()){return owner.take_domain_failure_into(recipient,grant)}RetainedWorkOutcome{result:Ok(Default::default()),work:0}}
 fn bind(source:&[u8],range:(usize,usize),envelope_mode:bool)->Result<Self,ValueError>{if range.0>range.1||range.1>source.len(){return Err(invalid("original command receiving range escaped its source"))}let mut cursor=Self{last_work:0,close_frame:Default::default(),failed:false,pending_list:None,nested_range:None,child_storage:NativeOwnedStorage::empty(),command_storage:NativeOwnedStorage::empty(),address:source.as_ptr()as usize,extent:source.len(),start:range.0,end:range.1,position:range.0,envelope_mode,tasks:[Task::Finish;24],depth:0,ordinal:0,policy:0,lane:0,resolution:0,count:0,index:0,timestamp:[0;3],fields:ManuallyDrop::new(Some(Fields::empty())),field:None,binary:None,operation:None,child:None,retiring:None,ready:false,closing:false,closed:false};if envelope_mode{cursor.sequence(&[Task::Text(Text::MutationId),Task::Text(Text::DocumentId),Task::Text(Text::First),Task::SharedActor,Task::Count(List::Dependencies),Task::Flag(4),Task::Count(List::Target),Task::Text(Text::DiffSchema),Task::Bytes(false),Task::Text(Text::InverseSchema),Task::Bytes(true),Task::Scalar(3),Task::Scalar(4),Task::Scalar(5),Task::Flag(5),Task::Finish])?}else{cursor.sequence(&[Task::Format,Task::Ordinal])?}Ok(cursor)}
 /// 🎟️ Admits only metadata from the independently supplied original one-item turn.
 pub fn admit(source:&[u8],range:(usize,usize),grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(Self,RetainedCloneProgress)>>{if !funds(RetirementDemand{depth:1,..Default::default()},grant){return RetainedWorkOutcome{result:Ok(None),work:0}}let owner=match Self::bind(source,range,false){Ok(owner)=>owner,Err(e)=>return RetainedWorkOutcome{result:Err(e),work:0}};RetainedWorkOutcome{result:Ok(Some((owner,item()))),work:1}}

 fn validate(&self,source:&[u8])->Result<(),ValueError>{if self.address!=source.as_ptr()as usize||self.extent!=source.len(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original command receiving source identity changed"))}Ok(())}
 fn sequence(&mut self,tasks:&[Task])->Result<(),ValueError>{if self.depth+tasks.len()>self.tasks.len(){return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original command fixed task frontier is full"))}for task in tasks.iter().rev(){self.tasks[self.depth]=*task;self.depth+=1}Ok(())}
 fn byte(&mut self,source:&[u8])->Result<u8,ValueError>{let byte=source.get(self.position).filter(|_|self.position<self.end).copied().ok_or_else(||invalid("original command scalar is truncated"))?;self.position+=1;Ok(byte)}
 fn integer(&mut self,source:&[u8])->Result<u64,ValueError>{let mut value=0;for shift in(0..70).step_by(7){let byte=self.byte(source)?;if shift==63&&byte>1{return Err(invalid("original command integer overflow"))}value|=u64::from(byte&127)<<shift;if byte&128==0{return Ok(value)}}Err(invalid("original command integer overflow"))}
 fn range(&mut self,source:&[u8])->Result<(usize,usize),ValueError>{let count=usize::try_from(self.integer(source)?).map_err(|_|invalid("original command field extent overflow"))?;let start=self.position;let end=start.checked_add(count).filter(|end|*end<=self.end).ok_or_else(||invalid("original command field is truncated"))?;self.position=end;Ok((start,end))}
 fn list_capacity(&self,list:List)->Result<usize,ValueError>{self.count.checked_mul(match list{List::Mutations=>size_of::<M>(),List::Inputs=>size_of::<SupersedeInput<M>>(),List::Authors=>size_of::<Author>(),List::Dependencies=>size_of::<MutationId>(),List::Target=>size_of::<String>()}).ok_or_else(||invalid("original command vector extent overflow"))}
 /// 📏️ Quotes the actual retained frontier without deriving any caller authority.
 pub fn demands(&self,source:&[u8],copy:usize)->Result<RetirementDemand,ValueError>{
  self.validate(source)?;if self.closing{return self.close_demands(copy)}if self.ready||self.closed||self.failed{return Ok(RetirementDemand{depth:1,..Default::default()})}
  if let Some(field)=self.field.as_ref(){return if field.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{field.demands(source,copy)}}
  if let Some(binary)=self.binary.as_ref(){return if binary.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{binary.demands(source,copy)}}
  if let Some(operation)=self.operation.as_ref(){if operation.terminal_is_empty(){return Ok(RetirementDemand{depth:1,..Default::default()})}return if self.fields.as_ref().ok_or_else(||ValueError::invariant("fields absent"))?.replacement.is_some(){operation.close_demands(copy)}else{operation.demands(source,copy)}}
  let task=*self.tasks.get(self.depth.checked_sub(1).ok_or_else(||invalid("original command task frontier is empty"))?).ok_or_else(||invalid("original command task frontier escaped its inline storage"))?;
  if matches!(task,Task::Mutation)&&self.operation.is_none(){return Ok(RetirementDemand{depth:1,..Default::default()})}
  if let Task::Allocate(list)=task{return Ok(RetirementDemand{capacity_bytes:if self.pending_list.as_ref().is_some_and(|p|!p.ready()){self.list_capacity(list)?}else{0},depth:1,..Default::default()})}
  if let Task::Nested(envelope)=task{
   if self.command_storage.reserved()||self.child.as_ref().is_some_and(|child|child.closed&&!envelope)&&self.fields.as_ref().ok_or_else(||ValueError::invariant("fields absent"))?.output.is_some(){return Ok(if self.command_storage.reserved(){NativeOwnedStorage::<ArtifactCommand<M>>::move_demand()}else{NativeOwnedStorage::<ArtifactCommand<M>>::reserve_demand()})}
   if let Some(child)=self.child.as_ref(){if child.terminal_is_empty(){return Ok(RetirementDemand{release_bytes:size_of::<Self>(),depth:1,..Default::default()})}let mut d=if child.closed{child.close_demands(copy)?}else{child.demands(source,copy)?};d.depth=d.depth.checked_add(1).ok_or_else(||invalid("original nested command depth overflow"))?;return Ok(d)}
   return Ok(if self.nested_range.is_none()||self.child_storage.reserved(){NativeOwnedStorage::<Self>::move_demand()}else{NativeOwnedStorage::<Self>::reserve_demand()})
  }
  Ok(RetirementDemand{capacity_bytes:if matches!(task,Task::SharedActor)&&self.fields.as_ref().ok_or_else(||ValueError::invariant("fields absent"))?.actor.is_none(){SharedUtf8::admission_demand().capacity_bytes}else{0},depth:1,..Default::default()})
 }
 fn text_slot(fields:&mut Fields<M>,text:Text)->&mut Option<String>{match text{Text::First=>&mut fields.first,Text::Second=>&mut fields.second,Text::Message=>&mut fields.message,Text::MutationId=>&mut fields.mutation_id,Text::DocumentId=>&mut fields.document_id,Text::DiffSchema=>&mut fields.diff_schema,Text::InverseSchema=>&mut fields.inverse_schema,Text::Verb=>&mut fields.verb,Text::Line=>&mut fields.line}}
 fn field_step(&mut self,source:&[u8],body:usize,grant:RetainedCloneGrant,text:Text)->Result<RetainedCloneProgress,ValueError>{
  self.last_work=0;if self.field.as_ref().is_some_and(RetainedWireFieldDecode::terminal_is_empty){self.field=None;self.depth-=1;self.last_work=1;return Ok(item())}
  if self.field.is_none(){self.last_work=1;let before=self.position;let range=self.range(source).map_err(|e|e.with_retained_progress(if self.position!=before{item()}else{Default::default()}))?;self.field=Some(RetainedWireFieldDecode::new_range(source,range,true)?);return Ok(item())}
  let field=self.field.as_mut().ok_or_else(||ValueError::invariant("field absent"))?;let result=if field.is_ready(){field.take_text_into(Self::text_slot(self.fields.as_mut().ok_or_else(||ValueError::invariant("fields absent"))?,text),grant)}else{let _=body;field.advance(source,grant)};self.last_work=u128::from(field.last_turn_work());let progress=field.take_receipt().map_or_else(||result.as_ref().err().map_or(Default::default(),ValueError::retained_progress),|(_,progress)|progress);result.map(|_|progress).map_err(|error|error.with_retained_progress(progress))
 }
 fn bytes_step(&mut self,source:&[u8],grant:RetainedCloneGrant,inverse:bool)->Result<RetainedCloneProgress,ValueError>{
  self.last_work=0;if self.binary.as_ref().is_some_and(RetainedWireBytesDecode::terminal_is_empty){self.binary=None;self.depth-=1;self.last_work=1;return Ok(item())}
  if self.binary.is_none(){self.last_work=1;let before=self.position;let range=self.range(source).map_err(|e|e.with_retained_progress(if self.position!=before{item()}else{Default::default()}))?;self.binary=Some(RetainedWireBytesDecode::new_range(source,range)?);return Ok(item())}
  let binary=self.binary.as_mut().ok_or_else(||ValueError::invariant("binary absent"))?;let result=if binary.is_ready(){let fields=self.fields.as_mut().ok_or_else(||ValueError::invariant("fields absent"))?;binary.take_into(if inverse{&mut fields.inverse_payload}else{&mut fields.diff_payload},grant)}else{binary.advance(source,grant)};self.last_work=u128::from(binary.last_turn_work());let progress=if let Some((issued,p))=binary.take_receipt(){if issued!=grant{return Err(invalid("original command binary field changed its grant").with_retained_progress(p))}p}else{result.as_ref().err().map_or(Default::default(),ValueError::retained_progress)};result.map(|_|progress).map_err(|e|e.with_retained_progress(progress))
 }
 fn blocked()->Outcome{RetainedWorkOutcome{result:Ok(Default::default()),work:0}}
 fn refused(error:ValueError)->Outcome{RetainedWorkOutcome{result:Err(error.into()),work:0}}
 /// 🪙️ Funds exactly one action: the control's own quote opens the effect and its performed receipt is the single charge.
 fn act(native:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant,demand:RetirementDemand,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->(Result<RetainedCloneProgress,ValueError>,u128))->Outcome{
  if let Err(error)=native.checkpoint(){return Self::refused(error)}
  let ran=native.with_allocated_effect(demand,1,|_,effect|{
   let(mut result,claimed)=run(effect);let owed=claimed.saturating_sub(effect.performed_work());
   if owed!=0{match u64::try_from(owed).ok().map(|owed|effect.performed(owed)){Some(Ok(true))=>{},Some(Err(error))=>{if result.is_ok(){result=Err(error)}},_=>{if result.is_ok(){result=Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original command action exceeded its work"))}}}}
   let progress=match &result{Ok(progress)=>*progress,Err(error)=>error.retained_progress()};
   if result.is_ok()&&!progress.fits(grant){result=Err(invalid("native defining receipt exceeds its actual physical grant").with_retained_progress(progress))}
   if progress.retained_capacity_bytes!=0&&effect.actual().is_none(){*effect.actual()=Some(progress.retained_capacity_bytes)}
   RetainedWorkOutcome{result,work:effect.performed_work()}
  });
  match ran{Ok(Some(turn))=>RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work},Ok(None)=>Self::blocked(),Err(error)=>Self::refused(error)}
 }
 fn list_step(&mut self,list:List,native:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant,demand:RetirementDemand)->Outcome{
  let count=self.count;
  if self.pending_list.as_ref().is_some_and(|p|!p.ready()){let Some(pending)=self.pending_list.as_mut()else{return Self::refused(invariant("pending_list absent"))};return Self::act(native,grant,demand,|effect|(pending.admit(count,effect),1))}
  Self::act(native,grant,demand,|_|{
   let Some(fields)=self.fields.as_mut()else{return(Err(invariant("fields absent")),0)};
   if let Some(pending)=self.pending_list.take(){pending.publish(fields);self.tasks[self.depth-1]=Task::Loop(list)}else{self.pending_list=Some(PendingList::new(list))}
   (Ok(item()),1)
  })
 }
 fn mutation_step(&mut self,source:&[u8],native:&mut NativeDecodeControl<'_>,body:usize,grant:RetainedCloneGrant,demand:RetirementDemand)->Outcome{
  if self.operation.as_ref().is_some_and(OpBinaryReceivingCursor::terminal_is_empty){return Self::act(native,grant,demand,|_|{std::mem::forget(self.operation.take());self.depth-=1;(Ok(item()),1)})}
  if self.operation.is_none(){
   if self.nested_range.is_none(){return Self::act(native,grant,demand,|_|{let before=self.position;let result=self.range(source).map(|range|{self.nested_range=Some((range.0,range.1,false));item()}).map_err(|e|e.with_retained_progress(if self.position!=before{item()}else{Default::default()}));(result,1)})}
   let Some(range)=self.nested_range else{return Self::refused(invariant("nested_range absent"))};
   return Self::act(native,grant,demand,|_|{let turn=M::admit_receiving(source,(range.0,range.1),grant);let result=turn.result.map(|owner|match owner{Some((owner,p))=>{self.operation=Some(owner);self.nested_range=None;p},None=>Default::default()});(result,turn.work)})
  }
  let replacing=match self.fields.as_ref(){Some(fields)=>fields.replacement.is_some(),None=>return Self::refused(invariant("fields absent"))};
  let Some(operation)=self.operation.as_mut()else{return Self::refused(invariant("operation absent"))};
  if replacing{return Self::act(native,grant,demand,|effect|closed_outcome(operation.close_step(&mut effect.nested(grant))))}
  if operation.is_ready(){let Some(fields)=self.fields.as_mut()else{return Self::refused(invariant("fields absent"))};return Self::act(native,grant,demand,|_|{let turn=operation.take_into(&mut fields.replacement,grant);(turn.result,turn.work)})}
  operation.step(source,native,body,grant)
 }
 fn nested_step(&mut self,source:&[u8],native:&mut NativeDecodeControl<'_>,body:usize,grant:RetainedCloneGrant,envelope:bool,demand:RetirementDemand)->Outcome{
  let has_output=match self.fields.as_ref(){Some(fields)=>fields.output.is_some(),None=>return Self::refused(invariant("fields absent"))};
  if self.command_storage.reserved()||self.child.as_ref().is_some_and(|child|child.closed&&!envelope)&&has_output{
   if !self.command_storage.reserved(){return Self::act(native,grant,demand,|effect|progressed(self.command_storage.reserve_empty(effect)))}
   let Some(fields)=self.fields.as_mut()else{return Self::refused(invariant("fields absent"))};
   if !self.command_storage.initialized(){return Self::act(native,grant,demand,|effect|{let turn=self.command_storage.put_from(&mut fields.output,effect);outcome_progress(turn)})}
   return Self::act(native,grant,demand,|effect|progressed(self.command_storage.take_box_into(&mut fields.nested,effect)))
  }
  if self.child.is_none(){
   if self.nested_range.is_none(){return Self::act(native,grant,demand,|_|{let before=self.position;let result=self.range(source).map(|range|{self.nested_range=Some((range.0,range.1,envelope));item()}).map_err(|e|e.with_retained_progress(if self.position!=before{item()}else{Default::default()}));(result,1)})}
   if !self.child_storage.reserved(){return Self::act(native,grant,demand,|effect|progressed(self.child_storage.reserve_empty(effect)))}
   if !self.child_storage.initialized(){let Some(range)=self.nested_range else{return Self::refused(invariant("nested_range absent"))};return Self::act(native,grant,demand,|effect|match Self::bind(source,(range.0,range.1),range.2){Ok(value)=>{let mut value=Some(value);let turn=self.child_storage.put_from(&mut value,effect);if let Some(unplaced)=value{std::mem::forget(unplaced);return(Err(invariant("funded command initialization retains its original inline input")),turn.work)}outcome_progress(turn)},Err(error)=>(Err(error),0)})}
   return Self::act(native,grant,demand,|effect|{let polled=progressed(self.child_storage.take_box_into(&mut self.child,effect));if self.child.is_some(){self.nested_range=None}polled})
  }
  let terminal=self.child.as_ref().is_some_and(|child|child.terminal_is_empty());
  if terminal{return Self::act(native,grant,demand,|_|{let Some(owner)=self.child.take()else{return(Err(invariant("child absent")),0)};let owner=Box::into_raw(owner);unsafe{std::alloc::dealloc(owner.cast(),std::alloc::Layout::new::<Self>())};self.depth-=1;(Ok(RetainedCloneProgress{released_bytes:size_of::<Self>(),..item()}),1)})}
  let Some(child)=self.child.as_mut()else{return Self::refused(invariant("child absent"))};let loan=RetainedCloneGrant{maximum_depth:grant.maximum_depth.saturating_sub(1),..grant};
  if child.is_ready(){let Some(fields)=self.fields.as_mut()else{return Self::refused(invariant("fields absent"))};return Self::act(native,grant,demand,|_|{let turn=if envelope{child.take_envelope_into(&mut fields.envelope,loan)}else{child.take_into(&mut fields.output,loan)};(turn.result,turn.work)})}
  if child.closed{return Self::act(native,grant,demand,|effect|closed_outcome(child.close_step(&mut effect.nested(loan))))}
  child.step(source,native,body,loan)
 }
 fn append(&mut self,list:List)->Result<(),ValueError>{
  let absent=||invalid("original command append source is absent");let fields=self.fields.as_mut().ok_or_else(||invariant("fields absent"))?;
  match list{
   List::Mutations=>fields.mutations.push(fields.replacement.take().ok_or_else(absent)?),
   List::Inputs=>{let target=MutationId(fields.second.take().ok_or_else(absent)?);fields.inputs.push(SupersedeInput{target,replacement:fields.replacement.take()})},
   List::Authors=>{let id=fields.first.take().ok_or_else(absent)?;let name=fields.second.take().ok_or_else(absent)?;fields.authors.push(Author{id,name,avatar:None})},
   List::Dependencies=>fields.dependencies.push(MutationId(fields.first.take().ok_or_else(absent)?)),
   List::Target=>fields.target.push(fields.first.take().ok_or_else(absent)?),
  }
  self.index+=1;Ok(())
 }
 fn command_tasks(&mut self)->Result<(),ValueError>{let tasks:&[Task]=match self.ordinal{0|11=>&[Task::Flag(0),Task::Count(List::Mutations),Task::Finish],1|2|10=>&[Task::Finish],3=>&[Task::Scalar(0),Task::Flag(1),Task::Finish],4=>&[Task::Flag(2),Task::Count(List::Authors),Task::Finish],5|6|7|20|21=>&[Task::Text(Text::First),Task::Finish],9=>&[Task::Nested(true),Task::Finish],13|14=>&[Task::Scalar(1),Task::Finish],15=>&[Task::Scalar(2),Task::Finish],16=>&[Task::Text(Text::First),Task::Scalar(6),Task::Finish],17=>&[Task::Flag(3),Task::Count(List::Inputs),Task::Finish],18=>&[Task::Text(Text::First),Task::Count(List::Inputs),Task::Finish],19=>&[Task::Text(Text::First),Task::Text(Text::Second),Task::Flag(6),Task::Count(List::Mutations),Task::Finish],_=>return Err(invalid("original command ordinal is invalid"))};self.sequence(tasks)}
 fn flag(&mut self,flag:u8,source:&[u8])->Result<(),ValueError>{
  if flag==6{let fields=self.fields.as_mut().ok_or_else(||invariant("fields absent"))?;let absent=||invalid("original command transaction text is absent");let id=fields.first.take().ok_or_else(absent)?;let tool=fields.second.take().ok_or_else(absent)?;fields.transaction=Some(TransactionRef{id,tool});return Ok(())}
  let value=if flag==4||flag==5{self.integer(source)?}else{u64::from(self.byte(source)?)};
  match flag{
   0=>{if value&!2!=0{return Err(invalid("original command transaction presence is invalid"))}if self.ordinal==11{self.sequence(&[Task::Scalar(1)])?}if value&2!=0{self.sequence(&[Task::Text(Text::First),Task::Text(Text::Second),Task::Flag(6)])?}},
   1=>match value{0=>{},1=>self.sequence(&[Task::Nested(false)])?,_=>return Err(invalid("original nested command presence is invalid"))},
   2=>match value{0=>{},1=>self.sequence(&[Task::Text(Text::Message)])?,_=>return Err(invalid("original checkpoint message presence is invalid"))},
   3=>match value{0=>{},1=>self.sequence(&[Task::Text(Text::First)])?,_=>return Err(invalid("original supersede scope presence is invalid"))},
   4=>match value{0=>{},1=>self.sequence(&[Task::Text(Text::First),Task::Flag(7)])?,_=>return Err(invalid("original envelope observed presence is invalid"))},
   5=>{if value>7{return Err(invalid("original envelope trailing flags are invalid"))}if value&4!=0{self.sequence(&[Task::Text(Text::Line)])?}if value&2!=0{self.sequence(&[Task::Text(Text::Verb)])?}if value&1!=0{self.sequence(&[Task::Text(Text::First),Task::Text(Text::Second),Task::Flag(6)])?}},
   8=>match value{0=>self.sequence(&[Task::Mutation])?,1=>{},_=>return Err(invalid("original supersede replacement tag is invalid"))},
   _=>return Err(invalid("original command task flag is invalid")),
  }Ok(())
 }
 /// ⏯️ Advances exactly one funded frontier and returns only this original turn's actual effects.
 pub fn step(&mut self,source:&[u8],native:&mut NativeDecodeControl<'_>,body:usize,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{
  self.last_work=0;let turn=self.step_action(source,native,body,grant);self.last_work=turn.work;if turn.result.is_err()&&turn.work!=0{self.failed=true}turn
 }
 fn step_action(&mut self,source:&[u8],native:&mut NativeDecodeControl<'_>,body:usize,grant:RetainedCloneGrant)->Outcome{
  if let Err(e)=self.validate(source){return Self::refused(e)}if self.closing||self.closed||self.ready||self.failed{return Self::blocked()}let demand=match self.demands(source,body){Ok(d)=>d,Err(e)=>return Self::refused(e)};if !funds(demand,grant){return Self::blocked()}if native.owned_bytes().checked_add(demand.capacity_bytes).is_none_or(|n|n>native.maximum_bytes()){return Self::refused(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original command exceeds its native allocation ceiling"))}
  let task=self.tasks[self.depth-1];match task{
   Task::Mutation=>return self.mutation_step(source,native,body,grant,demand),Task::Nested(envelope)=>return self.nested_step(source,native,body,grant,envelope,demand),Task::Allocate(list)=>return self.list_step(list,native,grant,demand),
   Task::SharedActor=>{
    let admission=SharedUtf8::admission_demand();let Some(fields)=self.fields.as_mut()else{return Self::refused(invariant("fields absent"))};
    if fields.actor.is_none(){if !funds(admission,grant){return Self::blocked()}return Self::act(native,grant,admission,|effect|{let turn=SharedUtf8::receive_original(&mut fields.first,&mut fields.actor,effect);(turn.result,turn.work)})}
    return Self::act(native,grant,demand,|_|{self.depth-=1;(Ok(item()),1)})
   },
   _=>{}
  }
  if let Task::Text(text)=task{return Self::act(native,grant,demand,|_|{let result=self.field_step(source,body,grant,text);(result,self.last_work)})}if let Task::Bytes(inverse)=task{return Self::act(native,grant,demand,|_|{let result=self.bytes_step(source,grant,inverse);(result,self.last_work)})}
  Self::act(native,grant,demand,|_|{
   let result=(||{
   self.depth-=1;
   match task{
    Task::Format=>{if self.byte(source)?!=super::super::COMMAND_BINARY_FORMAT{return Err(invalid("original command format is invalid"))}},Task::Ordinal=>{self.ordinal=self.integer(source)?;self.command_tasks()?},
    Task::Count(list)=>{self.count=usize::try_from(self.integer(source)?).map_err(|_|invalid("original command count overflow"))?;self.index=0;self.sequence(&[Task::Allocate(list)])?},
    Task::Loop(list)=>{if self.index<self.count{self.sequence(&[Task::Loop(list)])?;let tasks:&[Task]=match list{List::Mutations=>&[Task::Mutation,Task::Append(List::Mutations)],List::Inputs=>&[Task::Text(Text::Second),Task::Flag(8),Task::Append(List::Inputs)],List::Authors=>&[Task::Text(Text::First),Task::Text(Text::Second),Task::Append(List::Authors)],List::Dependencies=>&[Task::Text(Text::First),Task::Append(List::Dependencies)],List::Target=>&[Task::Text(Text::First),Task::Append(List::Target)]};self.sequence(tasks)?}},
    Task::Append(list)=>{self.append(list)?},
    Task::Flag(7)=>{let fields=self.fields.as_mut().ok_or_else(||invariant("fields absent"))?;let first=fields.first.take().ok_or_else(||invalid("original envelope observed id absent"))?;fields.observed=Some(MutationId(first))},Task::Flag(flag)=>self.flag(flag,source)?,
    Task::Scalar(scalar)=>{let value=if(3..=5).contains(&scalar){self.integer(source)?}else{u64::from(self.byte(source)?)};match scalar{0 if value<=3=>self.policy=value as u8,1 if value<=1=>self.lane=value as u8,2 if protocol::MergePolicy::from_u8(value as u8).is_some()=>self.policy=value as u8,3..=5=>self.timestamp[scalar as usize-3]=value,6 if value<=1=>self.resolution=value as u8,_=>return Err(invalid("original command enum scalar is invalid"))}},
    Task::Finish=>{if self.position!=self.end{return Err(invalid("original command has trailing bytes"))}self.finish()?;self.ready=true},
    _=>return Err(invalid("original command task frontier is invalid")),
   }Ok(item())
   })();(result.map_err(|e:ValueError|e.with_retained_progress(item())),1)
  })
 }
 fn finish(&mut self)->Result<(),ValueError>{let absent=||invalid("original command field is absent");let fields=self.fields.as_mut().ok_or_else(||invariant("fields absent"))?;if self.envelope_mode{fields.envelope=Some(protocol::MutationEnvelope{mutation_id:MutationId(fields.mutation_id.take().ok_or_else(absent)?),document_id:protocol::ArtifactId(fields.document_id.take().ok_or_else(absent)?),actor:protocol::ActorId(fields.actor.take().ok_or_else(absent)?),dependencies:std::mem::take(&mut fields.dependencies),observed:fields.observed.take(),target:std::mem::take(&mut fields.target),diff:protocol::ArtifactDiff{schema:protocol::SchemaId(fields.diff_schema.take().ok_or_else(absent)?),payload:fields.diff_payload.take().ok_or_else(absent)?},inverse:protocol::InverseMutation{schema:protocol::SchemaId(fields.inverse_schema.take().ok_or_else(absent)?),payload:fields.inverse_payload.take().ok_or_else(absent)?},timestamp:protocol::HybridLogicalTimestamp{actor:self.timestamp[0],physical_ms:self.timestamp[1],logical:self.timestamp[2]},transaction:fields.transaction.take(),verb:fields.verb.take(),line:fields.line.take()});return Ok(())}let command=match self.ordinal{0=>ArtifactCommand::Apply{mutations:std::mem::take(&mut fields.mutations),transaction:fields.transaction.take()},1=>ArtifactCommand::Undo,2=>ArtifactCommand::Redo,3=>ArtifactCommand::UndoWithPolicy{policy:match self.policy{0=>protocol::UndoPolicy::ExactBaseOnly,1=>protocol::UndoPolicy::TransformAgainstConcurrent,2=>protocol::UndoPolicy::SemanticUndo,_=>protocol::UndoPolicy::CompensatingAction},semantic_command:fields.nested.take()},4=>ArtifactCommand::CommitCheckpoint{message:fields.message.take(),authors:std::mem::take(&mut fields.authors)},5=>ArtifactCommand::CreateAlternative{name:fields.first.take().ok_or_else(absent)?},6=>ArtifactCommand::SwitchAlternative{alternative_id:fields.first.take().ok_or_else(absent)?},7=>ArtifactCommand::CheckoutCheckpoint{checkpoint_id:fields.first.take().ok_or_else(absent)?},9=>ArtifactCommand::IngestRemote{envelope:fields.envelope.take().ok_or_else(absent)?},10=>ArtifactCommand::PruneDrafts,11=>ArtifactCommand::ApplyInLane{mutations:std::mem::take(&mut fields.mutations),lane:lane(self.lane)?,transaction:fields.transaction.take()},13=>ArtifactCommand::UndoInLane{lane:lane(self.lane)?},14=>ArtifactCommand::RedoInLane{lane:lane(self.lane)?},15=>ArtifactCommand::SetMergePolicy{policy:protocol::MergePolicy::from_u8(self.policy).ok_or_else(absent)?},16=>ArtifactCommand::ResolveConflict{conflict_id:fields.first.take().ok_or_else(absent)?,resolution:if self.resolution==0{protocol::ConflictResolution::Accept}else{protocol::ConflictResolution::Discard}},17=>ArtifactCommand::Supersede{scope:fields.first.take(),inputs:std::mem::take(&mut fields.inputs)},18=>ArtifactCommand::CreateAlternativeWithSupersede{name:fields.first.take().ok_or_else(absent)?,inputs:std::mem::take(&mut fields.inputs)},19=>ArtifactCommand::AppendTransaction{mutations:std::mem::take(&mut fields.mutations),transaction:fields.transaction.take().ok_or_else(absent)?},20=>ArtifactCommand::CommitTransaction{transaction_id:fields.first.take().ok_or_else(absent)?},21=>ArtifactCommand::AbortTransaction{transaction_id:fields.first.take().ok_or_else(absent)?},_=>return Err(invalid("original command ordinal is invalid"))};fields.output=Some(command);Ok(())}
 /// 🏁️ Exposes completion without taking ownership away from the receiving frame.
 pub fn is_ready(&self)->bool{self.ready&&!self.closing&&!self.closed&&!self.failed}
 /// 📤️ Moves the exact typed output under the original one-item metadata admission.
 pub fn take_into(&mut self,recipient:&mut Option<ArtifactCommand<M>>,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress>{if recipient.is_some()||!self.is_ready()||self.envelope_mode||!funds(RetirementDemand{depth:1,..Default::default()},grant){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}*recipient=match self.fields.as_mut(){Some(v)=>v,None=>return RetainedWorkOutcome{result:Err(ValueError::invariant("fields absent").into()),work:0}}.output.take();self.ready=false;self.closed=true;RetainedWorkOutcome{result:Ok(item()),work:1}}
 fn take_envelope_into(&mut self,recipient:&mut Option<protocol::MutationEnvelope>,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress>{if recipient.is_some()||!self.is_ready()||!self.envelope_mode||!funds(RetirementDemand{depth:1,..Default::default()},grant){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}*recipient=match self.fields.as_mut(){Some(v)=>v,None=>return RetainedWorkOutcome{result:Err(ValueError::invariant("fields absent").into()),work:0}}.envelope.take();self.ready=false;self.closed=true;RetainedWorkOutcome{result:Ok(item()),work:1}}
 /// 🔗️ The genuine source receives actual child work independently from its physical item receipt.
 pub fn receive_step(&mut self,source:&[u8],body:usize,cx:&mut StepContext<'_>,native:&mut OriginalNativeDecodeSource)->Result<Option<OriginalNativeDecodeOutcome<RetainedCloneProgress,OriginalReceivingPrimary>>,OriginalTurnFailure>{native.receive_step(cx,|control,grant|{let turn=self.step(source,control,body,grant);let progress=turn.result.as_ref().copied().unwrap_or_else(|e|e.retained_progress());OriginalNativeDecodeIssued{value:turn.result,grant,progress,work:narrow(turn.work)}})}
 /// 🪙️ Actual close effects remain owned before native allocation and receipt qualification.
 pub fn close_on_original(&mut self,cx:&mut StepContext<'_>,native:&mut OriginalNativeDecodeSource)->Result<Option<OriginalNativeDecodeOutcome<RetainedCloneProgress,OriginalReceivingPrimary>>,OriginalTurnFailure>{
  let fuel=cx.fuel_remaining();let mut frame=std::mem::take(&mut self.close_frame);
  let outcome=native.close_producer_step(cx,|control,grant|{
   let mut work=0;let value=control.with_retirement(grant,|retirement|{
    let body=grant.maximum_copy_bytes;let demand=self.close_demands(body)?;if !funds(demand,grant){return Ok::<_,ValueError>(RetainedCloneProgress::default())}
    match retirement.receive_effect(demand,fuel,&mut frame,|_,effect|settled(self.close_step(effect)))?{
     NativeReceivingEffectPoll::Pending{work:0}=>Ok(RetainedCloneProgress::default()),
     NativeReceivingEffectPoll::Pending{work:pending}=>{work=pending;Err(invariant("original command close stayed pending on a synchronous recipient"))},
     NativeReceivingEffectPoll::Complete(turn)=>{work=turn.work;turn.result.map(|step|step.progress()).map_err(|failure|{let progress=failure.retained_progress();failure.child.or(failure.producer_refusal).or(failure.work_refusal).unwrap_or_else(||invariant("original command close receiving refused")).with_retained_progress(progress)})}
    }
   });
   let progress=value.as_ref().copied().unwrap_or_else(|e|e.retained_progress());OriginalNativeDecodeIssued{value:value.map_err(OriginalReceivingPrimary::from),grant,progress,work:narrow(work)}
  });
  self.close_frame=frame;outcome
 }
 /// 📏️ Every quote describes the current live control or exact child frontier.
 pub fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if self.terminal_is_empty(){return Ok(Default::default())}if !self.closing{return Ok(RetirementDemand{depth:1,..Default::default()})}
  if let Some(p)=self.pending_list.as_ref(){return Ok(RetirementDemand{release_bytes:p.release_bytes(),depth:1,..Default::default()})}
  if self.command_storage.initialized(){return Ok(NativeOwnedStorage::<ArtifactCommand<M>>::move_demand())}if self.command_storage.reserved(){return Ok(self.command_storage.release_demand())}
  if let Some(child)=self.child_storage.get(){if child.terminal_is_empty(){return Ok(NativeOwnedStorage::<Self>::move_demand())}let mut d=child.close_demands(copy)?;d.depth=d.depth.checked_add(1).ok_or_else(||invalid("original staged command close depth overflow"))?;return Ok(d)}if self.child_storage.reserved(){return Ok(self.child_storage.release_demand())}
  if self.nested_range.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()})}
  if let Some(field)=self.field.as_ref(){return Ok(if field.terminal_is_empty(){RetirementDemand{depth:1,..Default::default()}}else{field.retirement_demands()})}if let Some(binary)=self.binary.as_ref(){return if binary.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{binary.close_demands(copy)}}
  if let Some(operation)=self.operation.as_ref(){return if operation.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{operation.close_demands(copy)}}
  if let Some(child)=self.child.as_ref(){if child.terminal_is_empty(){return Ok(RetirementDemand{release_bytes:size_of::<Self>(),depth:1,..Default::default()})}let mut d=child.close_demands(copy)?;d.depth=d.depth.checked_add(1).ok_or_else(||invalid("original nested close depth overflow"))?;return Ok(d)}
  if let Some(owner)=self.retiring.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{owner.next_demand(copy)}}
  Ok(RetirementDemand{depth:1,..Default::default()})
 }
 /// ♻️ A live close control, actual child turn and terminal header clear are funded by the turn's one effect.
 pub fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  self.last_work=0;let start=effect.performed_work();let result=self.close_action(effect);let poll=RetirementPollExit::settle(effect,result);self.last_work=effect.performed_work().saturating_sub(start);poll
 }
 fn close_action(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,RetirementPollExit>{
  let grant=effect.grant();let progress=|progress|Ok(RetainedCloneStep::Progress(progress));
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  let demand=self.close_demands(grant.maximum_copy_bytes)?;if !funds(demand,grant){return progress(Default::default())}
  if !self.closing{self.closing=true;return progress(item())}
  if let Some(pending)=self.pending_list.take(){let released_bytes=pending.release_bytes();pending.release();return progress(RetainedCloneProgress{released_bytes,..item()})}
  if self.command_storage.initialized(){let fields=self.fields.as_mut().ok_or_else(||invariant("fields absent"))?;return move_progress(self.command_storage.take_into(&mut fields.output,effect))}
  if self.command_storage.reserved(){return move_progress(self.command_storage.release(effect))}
  if let Some(child)=self.child_storage.get_mut(){if !child.terminal_is_empty(){return closed_progress(child.close_step(&mut effect.nested(RetainedCloneGrant{maximum_depth:grant.maximum_depth.saturating_sub(1),..grant})))}return progressed_exit(self.child_storage.take_box_into(&mut self.child,effect))}
  if self.child_storage.reserved(){return move_progress(self.child_storage.release(effect))}
  if self.nested_range.take().is_some(){return progress(item())}
  if let Some(binary)=self.binary.as_mut(){if binary.terminal_is_empty(){self.binary=None;return progress(item())}binary.cancel();let result=binary.close_step(effect);let settled=if let Some((issued,p))=binary.take_receipt(){if issued!=grant{return Err(invalid("original binary close changed its grant").with_retained_progress(p).into())}p}else{result.as_ref().err().map_or(Default::default(),OriginalRetirementFailure::retained_progress)};return result.map(|_|RetainedCloneStep::Progress(settled)).map_err(RetirementPollExit::from)}
  if let Some(field)=self.field.as_mut(){if field.terminal_is_empty(){self.field=None;return progress(item())}field.cancel();let result=field.close_step(effect);let settled=field.take_receipt().map_or_else(||result.as_ref().err().map_or(Default::default(),ValueError::retained_progress),|(_,p)|p);return result.map(|_|RetainedCloneStep::Progress(settled)).map_err(RetirementPollExit::from)}
  if let Some(operation)=self.operation.as_mut(){if operation.terminal_is_empty(){std::mem::forget(self.operation.take());return progress(item())}return closed_progress(operation.close_step(&mut effect.nested(grant)))}
  if let Some(child)=self.child.as_mut(){if !child.terminal_is_empty(){return closed_progress(child.close_step(&mut effect.nested(RetainedCloneGrant{maximum_depth:grant.maximum_depth.saturating_sub(1),..grant})))}let Some(owner)=self.child.take()else{return Err(invariant("child absent").into())};let pointer=Box::into_raw(owner);unsafe{std::alloc::dealloc(pointer.cast(),std::alloc::Layout::new::<Self>())};return progress(RetainedCloneProgress{released_bytes:size_of::<Self>(),..item()})}
  if self.retiring.is_none(){if !Fields::<M>::controlled_retirement_supported(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original command fields lack controlled retirement").into())}if let Some(fields)=self.fields.take(){match ControlledRetirement::new(fields){Ok(owner)=>self.retiring=Some(owner),Err((error,fields))=>{*self.fields=Some(fields);return Err(error.into())}}return progress(item())}}
  if let Some(owner)=self.retiring.as_mut(){if !owner.terminal_is_empty(){return closed_progress(owner.step(grant.maximum_copy_bytes,effect))}self.retiring=None;return progress(item())}
  self.closed=true;self.ready=false;Ok(RetainedCloneStep::Complete(item()))
 }
 /// 🔒️ Terminal custody includes all native fields, nested cursors, operations and final typed output.
 pub fn terminal_is_empty(&self)->bool{self.closed&&self.pending_list.is_none()&&self.nested_range.is_none()&&self.child_storage.terminal_is_empty()&&self.command_storage.terminal_is_empty()&&self.fields.is_none()&&self.field.is_none()&&self.binary.is_none()&&self.operation.is_none()&&self.child.is_none()&&self.retiring.is_none()&&self.close_frame.phase()==NativeReceivingEffectPhase::Empty}
}
impl<M:OpBinaryReceiving> RetainedCommandEnvelopeReceiving<M>{
 pub fn domain_failure(&self)->Option<&M::DomainFailure>{self.0.domain_failure()}
 pub fn domain_failure_demand(&self)->Result<RetirementDemand,ValueError>{self.0.domain_failure_demand()}
 pub fn domain_failure_work_demand(&self)->Result<u64,ValueError>{self.0.domain_failure_work_demand()}
 pub fn take_domain_failure_into(&mut self,recipient:&mut Option<M::DomainFailure>,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress>{self.0.take_domain_failure_into(recipient,grant)}
 pub fn admit(source:&[u8],range:(usize,usize),grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(Self,RetainedCloneProgress)>>{if grant.maximum_items==0||grant.maximum_depth==0{return RetainedWorkOutcome{result:Ok(None),work:0}}let owner=match RetainedArtifactCommandReceiving::bind(source,range,true){Ok(owner)=>owner,Err(e)=>return RetainedWorkOutcome{result:Err(e),work:0}};RetainedWorkOutcome{result:Ok(Some((Self(owner),item()))),work:1}}
 pub fn demands(&self,source:&[u8],copy:usize)->Result<RetirementDemand,ValueError>{self.0.demands(source,copy)}pub fn is_ready(&self)->bool{self.0.is_ready()}pub fn terminal_is_empty(&self)->bool{self.0.terminal_is_empty()}pub fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{self.0.close_demands(copy)}
 pub fn step(&mut self,source:&[u8],native:&mut NativeDecodeControl<'_>,body:usize,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{self.0.step(source,native,body,grant)}pub fn take_into(&mut self,recipient:&mut Option<protocol::MutationEnvelope>,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress>{self.0.take_envelope_into(recipient,grant)}pub fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{self.0.close_step(effect)}
}
impl<M:OpBinaryReceiving> Drop for RetainedArtifactCommandReceiving<M>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original command receiving cursor requires complete paid close");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.fields)}}}}
