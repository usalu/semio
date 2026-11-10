//! 🫴️ A borrowed original JSON output becomes one unique typed owner while every partial parser and receiver remains funded.
use semio_framework_pack_json::{JsonGrammarCursor,JsonSourceCursor,JsonReadLimits,JsonMemberPolicy,JsonError};
use semio_framework_value::{DslValue,NativeDecodeControl,OriginalRetirementControl,OriginalRetirementFailure,OriginalRetirementPoll,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll,RetirementDemand,RetirementPollExit,ValueError,ValueRefusalKind,retained_clone::{NativeAllocationIdentity,RetainedCloneSource,RetainedCloneSourceAdmission,RetainedCloneClose,retained_clone_grant_admits}};
use semio_framework_value_native_owned::{NativeReceive,OriginalReceivingPrimary};
use semio_framework_value_native_owned::native_decoding::{OriginalEffectAuthority};
use semio_framework_value_native_owned::native_decoding::owned::OriginalSourceTurnWitness;
use semio_framework_value_native_owned::owned::{NativeReceivingChild,NativeLocalAuthority,NativeLocalProducer,NativeLocalOperation};
use semio_framework_value_native_receiving_original::NativeReceivingOriginal;
use semio_framework_pack_json::close_native_receiver;
use std::mem::{ManuallyDrop,size_of};
#[derive(Clone,Copy,PartialEq,Eq)]
enum Phase{Parse,Bind,Source,Receiver,Receive,Take,CloseReceiver,CloseParser,Ready}
/// 🧳️ The transport retains the unchanged guest byte owner until this complete unique capture acknowledges it.
pub struct OriginalJsonOutputReceiving<T:NativeReceive<DslValue>+Sync>{ledger:NativeAllocationIdentity,recipient:Option<u64>,pointer:usize,length:usize,read_position:usize,parser:ManuallyDrop<Option<JsonGrammarCursor<DslValue>>>,parsed:ManuallyDrop<Option<DslValue>>,admission:ManuallyDrop<Option<RetainedCloneSourceAdmission<DslValue,DslValue>>>,source:ManuallyDrop<Option<RetainedCloneSource<DslValue>>>,receiver:ManuallyDrop<Option<NativeReceivingOriginal<DslValue,T>>>,output:ManuallyDrop<Option<T>>,syntax:ManuallyDrop<Option<JsonError>>,failure:ManuallyDrop<Option<NativeReceivingFailure>>,action:NativeReceivingChild<ValueError>,local_authority:ManuallyDrop<Option<NativeLocalAuthority<ValueError>>>,retirement:RetainedCloneClose,phase:Phase,closing:bool,faulted:bool}
/// 🎫️ A refused request stays inline; a failed action is stored once in the receiving owner and named by its receipted progress.
#[derive(Debug,semio_framework_value::RetireOwned)]
#[value(crate="::semio_framework_value")]
pub enum OriginalJsonOutputFailure{Refused(ValueError),Stored(RetainedCloneProgress)}
impl OriginalJsonOutputFailure{
 /// 📊️ Borrows the physical span the failing turn receipted.
 pub fn retained_progress(&self)->RetainedCloneProgress{match self{Self::Refused(error)=>error.retained_progress(),Self::Stored(progress)=>*progress}}
}
impl From<ValueError> for OriginalJsonOutputFailure{fn from(error:ValueError)->Self{Self::Refused(error)}}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original JSON output changed its source, decoder or ownership phase")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn add(mut d:RetirementDemand,bytes:usize)->Result<RetirementDemand,ValueError>{d.copy_bytes=d.copy_bytes.checked_add(bytes).ok_or_else(invalid)?;Ok(d)}
fn posted(mut p:RetainedCloneProgress,bytes:usize,work:u128)->Result<RetainedCloneProgress,ValueError>{if work!=0{p.copied_items=p.copied_items.max(1);p.copied_bytes=p.copied_bytes.checked_add(bytes).ok_or_else(invalid)?;}Ok(p)}
fn refused(failure:OriginalReceivingPrimary)->ValueError{let progress=failure.progress;failure.primary.or(failure.quote_refusal).or(failure.allocation_refusal).unwrap_or_else(invalid).with_retained_progress(progress)}
fn done<E>(result:Result<RetainedCloneStep,E>,work:u128)->RetainedWorkPoll<RetainedCloneStep,E>{RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work})}
/// 🧾️ Posts the metadata span onto the primary refusal that owns the progress.
fn posted_failure(mut failure:NativeReceivingFailure,metadata:usize,work:u128)->NativeReceivingFailure{if let NativeReceivingFailure::Primary(primary)=&mut failure{if let Ok(progress)=posted(primary.progress,metadata,work){primary.progress=progress}}failure}
fn remaining(g:RetainedCloneGrant,p:RetainedCloneProgress)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:g.maximum_items.saturating_sub(p.copied_items),maximum_copy_bytes:g.maximum_copy_bytes.saturating_sub(p.copied_bytes),maximum_capacity_bytes:g.maximum_capacity_bytes.saturating_sub(p.retained_capacity_bytes),maximum_release_bytes:g.maximum_release_bytes.saturating_sub(p.released_bytes),..g}}
fn sum(a:RetainedCloneProgress,b:RetainedCloneProgress)->Result<RetainedCloneProgress,ValueError>{let add=|x:usize,y:usize|x.checked_add(y).ok_or_else(invalid);Ok(RetainedCloneProgress{copied_items:add(a.copied_items,b.copied_items)?,copied_bytes:add(a.copied_bytes,b.copied_bytes)?,retained_capacity_bytes:add(a.retained_capacity_bytes,b.retained_capacity_bytes)?,released_bytes:add(a.released_bytes,b.released_bytes)?})}
/// 🧾️ Folds the physical span of the turn's earlier actions into the failing action's own receipt.
fn prior(mut failure:NativeReceivingFailure,earlier:RetainedCloneProgress)->NativeReceivingFailure{let primary=match &mut failure{NativeReceivingFailure::Primary(primary)=>primary,NativeReceivingFailure::ValueEffect(error)=>&mut error.receiving,NativeReceivingFailure::RetirementEffect(error)=>&mut error.receiving,NativeReceivingFailure::ReceivingEffect(error)=>&mut error.receiving};if let Ok(total)=sum(primary.progress,earlier){primary.progress=total}failure}
fn refusal(error:ValueError,earlier:RetainedCloneProgress)->OriginalJsonOutputFailure{let progress=sum(error.retained_progress(),earlier).unwrap_or(earlier);OriginalJsonOutputFailure::Refused(error.with_retained_progress(progress))}
fn progress_step(progress:RetainedCloneProgress)->RetainedCloneStep{RetainedCloneStep::Progress(progress)}
fn retirement_poll(poll:OriginalRetirementPoll)->RetainedWorkPoll<RetainedCloneStep,ValueError>{poll.map(|step|progress_step(step.progress())).map_err(OriginalRetirementFailure::into_value_error)}
impl<T:NativeReceive<DslValue>+Sync> OriginalJsonOutputReceiving<T>{
 pub fn constructor_demand()->RetirementDemand{fixed(2*size_of::<Self>())}
 pub fn admit(input:&[u8],limits:JsonReadLimits,control:&NativeDecodeControl<'_>,g:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),ValueError>{let d=Self::constructor_demand();if !retained_clone_grant_admits(g,d){return Err(invalid())}let(_,parser)=JsonSourceCursor::<_,DslValue>::new(input,JsonMemberPolicy::Reject,limits)?.into_grammar();Ok((Self{ledger:control.ledger_identity(),recipient:control.retirement_recipient_identity(),pointer:input.as_ptr()as usize,length:input.len(),read_position:0,parser:ManuallyDrop::new(Some(parser)),parsed:ManuallyDrop::new(None),admission:ManuallyDrop::new(None),source:ManuallyDrop::new(None),receiver:ManuallyDrop::new(None),output:ManuallyDrop::new(None),syntax:ManuallyDrop::new(None),failure:ManuallyDrop::new(None),action:Default::default(),local_authority:ManuallyDrop::new(Some(OriginalEffectAuthority::new(NativeLocalProducer::new()))),retirement:Default::default(),phase:Phase::Parse,closing:false,faulted:false},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
 fn validate(&self,input:&[u8],control:&NativeDecodeControl<'_>)->Result<(),ValueError>{if self.ledger!=control.ledger_identity()||self.recipient!=control.retirement_recipient_identity()||self.pointer!=input.as_ptr()as usize||self.length!=input.len(){return Err(invalid())}Ok(())}
 fn metadata(&self)->usize{size_of::<Phase>()+size_of::<bool>()+match self.phase{Phase::Parse=>size_of::<usize>()+size_of::<Option<DslValue>>()+size_of::<Option<JsonError>>(),Phase::Bind=>size_of::<Option<RetainedCloneSourceAdmission<DslValue,DslValue>>>()+size_of::<Option<DslValue>>(),Phase::Source=>size_of::<Option<RetainedCloneSource<DslValue>>>(),Phase::Receiver=>size_of::<Option<NativeReceivingOriginal<DslValue,T>>>(),Phase::Take=>size_of::<Option<T>>(),_=>0}}
 pub fn demands(&self,input:&[u8],body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{if self.closing{return self.close_demands(body)}if self.faulted{return Err(invalid())}if self.pointer!=input.as_ptr()as usize||self.length!=input.len(){return Err(invalid())}let body=body.saturating_sub(self.metadata());add(match self.phase{Phase::Parse=>self.parser.as_ref().ok_or_else(invalid)?.normal_step_demands(input).map_err(|error|ValueError::literal(error.kind(),"original JSON output grammar demand refused"))?,Phase::Bind|Phase::Receiver|Phase::Ready=>fixed(0),Phase::Source=>{let admission=self.admission.as_ref().ok_or_else(invalid)?;if admission.phase()=="projectionStart"{admission.owned_projection_demand()?}else if admission.phase()=="ready"{fixed(size_of::<Option<RetainedCloneSource<DslValue>>>())}else{admission.next_demand()?}},Phase::Receive=>self.receiver.as_ref().ok_or_else(invalid)?.demands(body,maximum_work_units)?,Phase::Take=>self.receiver.as_ref().ok_or_else(invalid)?.take_demand()?,Phase::CloseReceiver=>{let receiver=self.receiver.as_ref().ok_or_else(invalid)?;if receiver.terminal_is_empty(){fixed(size_of::<Option<NativeReceivingOriginal<DslValue,T>>>())}else if receiver.is_closing(){receiver.close_demands(body)?}else{receiver.begin_close_demand()?}},Phase::CloseParser=>if !self.retirement.is_empty(){self.retirement.next_owner_demands::<JsonGrammarCursor<DslValue>>(false,&None,body)?}else if self.parser.is_some(){self.retirement.next_owner_demands::<JsonGrammarCursor<DslValue>>(true,&None,body)?}else{fixed(0)}},self.metadata())}
 fn local(action:&mut NativeReceivingChild<ValueError>,authority:&mut Option<NativeLocalAuthority<ValueError>>,control:&mut NativeDecodeControl<'_>,d:RetirementDemand,g:RetainedCloneGrant,q:u64,maximum_work_units:u64,tag:u8,mut op:impl FnMut(&mut NativeDecodeControl<'_>,&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll<RetainedCloneStep,ValueError>)->Result<RetainedWorkPoll<RetainedCloneStep,NativeReceivingFailure>,ValueError>{
  action.tag=tag;let authority=authority.as_mut().ok_or_else(invalid)?;
  Ok(action.poll(control,d,g,q,maximum_work_units,authority,|control,effect,authority,action|authority.drive(&mut NativeLocalOperation::Poll(&mut |effect|op(control,effect)),effect,action)))
 }
 /// ⏯️ Performs original grammar, source, native and cleanup frontiers while the turn grant and work still admit the next one, forwarding their exact physical receipt.
 pub fn advance(&mut self,input:&[u8],control:&mut NativeDecodeControl<'_>,witness:OriginalSourceTurnWitness<'_>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->RetainedWorkPoll<RetainedCloneStep,OriginalJsonOutputFailure>{
  use RetainedWorkPoll::{Complete,Pending};
  let(mut total,mut work,mut first)=(RetainedCloneProgress::default(),0u128,true);
  loop{
   let rem=remaining(g,total);let left=maximum_work_units.saturating_sub(u64::try_from(work).unwrap_or(u64::MAX));
   if !first&&(left==0||rem.maximum_items==0||control.checkpoint().is_err()){return done(Ok(progress_step(total)),work)}
   let poll=match self.advance_turn(input,control,witness,body,maximum_work_units,g,rem,left){Ok(poll)=>poll,Err(error)=>return done(Err(refusal(error,total)),work)};
   match poll{
    Pending{work:pending}=>return if first{Pending{work:pending}}else if pending==0{done(Ok(progress_step(total)),work)}else{done(Err(refusal(invalid(),total)),work+pending)},
    Complete(turn)=>{
     work+=turn.work;
     match turn.result{
      Err(error)=>return done(Err(self.keep(prior(error,total))),work),
      Ok(step)=>{
       let progress=step.progress();let idle=turn.work==0&&progress==RetainedCloneProgress::default();
       total=match sum(total,progress){Ok(total)=>total,Err(error)=>return done(Err(refusal(error,total)),work)};
       if matches!(step,RetainedCloneStep::Complete(_)){return done(Ok(RetainedCloneStep::Complete(total)),work)}
       if idle{return done(Ok(progress_step(total)),work)}
      }
     }
    }
   }
   first=false;
  }
 }
 /// 🎫️ Stores the terminal action failure once in this owner's own funded slot and returns its receipted ticket.
 fn keep(&mut self,failure:NativeReceivingFailure)->OriginalJsonOutputFailure{
  assert!(self.failure.is_none(),"original JSON output stores one terminal receiving failure");
  let mut progress=failure.retained_progress();progress.copied_items=progress.copied_items.max(1);progress.copied_bytes=progress.copied_bytes.saturating_add(size_of::<Option<NativeReceivingFailure>>());
  *self.failure=Some(failure);self.faulted=true;OriginalJsonOutputFailure::Stored(progress)
 }
 /// 🔎️ Borrows the terminal receiving failure that the stored ticket names.
 pub fn failure(&self)->Option<&NativeReceivingFailure>{self.failure.as_ref()}
 fn advance_turn(&mut self,input:&[u8],control:&mut NativeDecodeControl<'_>,witness:OriginalSourceTurnWitness<'_>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant,rem:RetainedCloneGrant,left:u64)->Result<RetainedWorkPoll<RetainedCloneStep,NativeReceivingFailure>,ValueError>{
  use RetainedWorkPoll::{Complete,Pending};
  let g=RetainedCloneGrant{maximum_items:g.maximum_items.min(1),..g};let gate=RetainedCloneGrant{maximum_items:rem.maximum_items.min(1),..rem};
  self.validate(input,control)?;
  if self.ready(){return Ok(done(Ok(RetainedCloneStep::Complete(Default::default())),0))}
  let in_flight=self.action.demand.is_some();
  let idle=||if in_flight{Pending{work:0}}else{done(Ok(progress_step(Default::default())),0)};
  let metadata=self.metadata();let inner=body.saturating_sub(metadata);
  let d=self.demands(input,body,maximum_work_units)?;if left==0||!retained_clone_grant_admits(gate,d){return Ok(idle())}
  let q=if in_flight{self.action.work.unwrap_or(1)}else{match self.phase{Phase::Source=>match self.admission.as_ref(){Some(admission) if admission.phase()=="projectionStart"=>1,Some(admission) if admission.phase()!="ready"=>admission.advance_work_demand()?,_=>1},Phase::CloseReceiver=>match self.receiver.as_ref(){Some(receiver) if !receiver.terminal_is_empty()=>if receiver.is_closing(){receiver.close_work_demand(inner)?}else{receiver.begin_close_work_demand()?},_=>1},Phase::CloseParser if !self.retirement.is_empty()=>self.retirement.next_owner_work::<DslValue>(false,&None,inner)?,Phase::CloseParser if self.parser.is_some()=>self.retirement.next_owner_work::<JsonGrammarCursor<DslValue>>(true,&None,inner)?,_=>1}};if !in_flight&&q>left{return Ok(idle())}
  if !in_flight{control.checkpoint()?}
  let child=RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes.saturating_sub(metadata),..g};
  let poll=match self.phase{
   Phase::Parse=>{
    let parser=&mut self.parser;let parsed=&mut self.parsed;let syntax=&mut self.syntax;let faulted=&mut self.faulted;let phase=&mut self.phase;let read=&mut self.read_position;
    Self::local(&mut self.action,&mut self.local_authority,control,d,g,1,maximum_work_units,1,|control,effect|{
     let Some(parse)=parser.as_mut()else{return done(Err(invalid()),0)};
     let result=parse.step_source(input,1,control,&mut effect.nested(child),false);
     let work=u128::from(parse.last_turn_work());*read=parse.position();let p=parse.normal_step_progress();
     match result{Ok(Some(value))=>{**parsed=Some(value);*phase=Phase::Bind},Ok(None)=>{},Err(error)=>{let kind=error.kind();**syntax=Some(error);if work!=0{*faulted=true}return done(Err(ValueError::literal(kind,"original JSON output grammar refused").with_retained_progress(p)),work)}}
     done(Ok(progress_step(p)),work)
    })?
   },
   Phase::Bind=>{let value=self.parsed.take().ok_or_else(invalid)?;*self.admission=Some(RetainedCloneSource::<DslValue>::admission_owned(value,()));self.phase=Phase::Source;done(Ok(progress_step(Default::default())),1)},
   Phase::Source=>{
    let admission=self.admission.as_mut().ok_or_else(invalid)?;
    let ready=admission.phase()=="ready"||self.action.tag==9;let projecting=admission.phase()=="projectionStart"||self.action.tag==16;let source=&mut self.source;
    let poll=Self::local(&mut self.action,&mut self.local_authority,control,d,g,q,maximum_work_units,if projecting{16}else if ready{9}else{2},|_,effect|{let turn=if projecting{admission.project_owned(&mut effect.nested(child))}else if ready{admission.take_output(source,&mut effect.nested(child))}else{admission.advance(&mut effect.nested(child))};done(turn.result.map(|step|progress_step(step.progress())),turn.work)})?;
    if ready&&matches!(&poll,Complete(turn) if turn.result.is_ok())&&self.admission.as_ref().is_some_and(|admission|admission.terminal_is_empty()){self.admission.take();self.phase=Phase::Receiver}
    poll
   },
   Phase::Receiver=>{let source=self.source.take().ok_or_else(invalid)?;*self.receiver=Some(NativeReceivingOriginal::new(source));self.phase=Phase::Receive;done(Ok(progress_step(Default::default())),1)},
   Phase::Receive=>{
    let receiver=self.receiver.as_mut().ok_or_else(invalid)?;
    let poll=receiver.advance(control,witness,inner,maximum_work_units,child).map(|step|progress_step(step.progress()));
    if receiver.output_ready(){self.phase=Phase::Take}
    poll
   },
   Phase::Take=>{
    let receiver=self.receiver.as_mut().ok_or_else(invalid)?;let output=&mut self.output;
    let poll=Self::local(&mut self.action,&mut self.local_authority,control,d,g,q,maximum_work_units,3,|_,effect|match receiver.take(&mut **output,&mut effect.nested(child)){Complete(turn)=>done(turn.result.map(|step|progress_step(step.progress())).map_err(refused),turn.work),Pending{..}=>done(Err(invalid()),0)})?;
    if matches!(&poll,Complete(turn) if turn.result.is_ok())&&self.output.is_some(){self.phase=Phase::CloseReceiver}
    poll
   },
   Phase::CloseReceiver=>{
    let receiver=self.receiver.as_mut().ok_or_else(invalid)?;
    if receiver.terminal_is_empty(){self.receiver.take();self.phase=Phase::CloseParser;done(Ok(progress_step(RetainedCloneProgress{copied_bytes:d.copy_bytes.saturating_sub(metadata),..Default::default()})),1)}
    else{Self::local(&mut self.action,&mut self.local_authority,control,d,g,q,maximum_work_units,4,|_,effect|close_native_receiver(receiver,inner,&mut effect.nested(child)))?}
   },
   Phase::CloseParser=>{
    if !self.retirement.is_empty(){let retirement=&mut self.retirement;Self::local(&mut self.action,&mut self.local_authority,control,d,g,q,maximum_work_units,5,|_,effect|retirement_poll(retirement.step_granted(inner,&mut effect.nested(child))))?}
    else if self.parser.is_some(){let retirement=&mut self.retirement;let parser=&mut self.parser;Self::local(&mut self.action,&mut self.local_authority,control,d,g,q,maximum_work_units,6,|_,effect|{let turn=retirement.begin_granted(parser,&mut effect.nested(child));done(turn.result.map(|step|step.unwrap_or_else(||progress_step(Default::default()))),turn.work)})?}
    else{self.phase=Phase::Ready;done(Ok(progress_step(Default::default())),1)}
   },
   Phase::Ready=>return Err(invalid()),
  };
  let ready=self.ready();
  Ok(match poll{
   Pending{work}=>Pending{work},
   Complete(turn)=>{
    let work=turn.work;
    let result=match turn.result{
     Ok(step)=>posted(step.progress(),metadata,work).map(|p|if ready{RetainedCloneStep::Complete(p)}else{progress_step(p)}).map_err(NativeReceivingFailure::from),
     Err(error)=>{if work!=0{self.faulted=true}Err(posted_failure(error,metadata,work))}
    };
    done(result,work)
   }
  })
 }
 pub fn phase(&self)->&'static str{match self.phase{Phase::Parse=>"parse",Phase::Bind=>"bind",Phase::Source=>"source",Phase::Receiver=>"receiver",Phase::Receive=>"receive",Phase::Take=>"take",Phase::CloseReceiver=>"closeReceiver",Phase::CloseParser=>"closeParser",Phase::Ready=>"ready"}}
 pub fn bytes_read(&self)->usize{self.read_position}
 pub fn source_bytes(&self)->usize{self.length}
 pub fn ready(&self)->bool{!self.closing&&!self.faulted&&self.phase==Phase::Ready&&self.output.is_some()}
 pub fn syntax_error(&self)->Option<&JsonError>{self.syntax.as_ref()}
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(invalid())}Ok(fixed(size_of::<Option<T>>()))}
 /// 📤️ Moves the completed owner into the caller's slot as exactly one recorded action of the caller's effect.
 pub fn take_into(&mut self,output:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{
  let grant=effect.grant();
  let result=(||{
   if output.is_some(){return Err(invalid())}
   let d=self.take_demand()?;
   if !retained_clone_grant_admits(grant,d)||effect.maximum_work_units()==0{return Ok(progress_step(Default::default()))}
   if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original JSON output take exceeded its work authority"))}
   *output=self.output.take();self.closing=true;
   Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))
  })();
  RetainedWorkOutcome{result,work:effect.performed_work()}
 }
 pub fn begin_close(&mut self){self.closing=true}
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<DslValue>(false,&None,body)}if let Some(admission)=self.admission.as_ref(){return admission.next_return_demand()}if let Some(receiver)=self.receiver.as_ref(){return if receiver.terminal_is_empty(){Ok(fixed(size_of::<Option<NativeReceivingOriginal<DslValue,T>>>()))}else if receiver.is_closing(){receiver.close_demands(body)}else{receiver.begin_close_demand()}}if let Some(source)=self.source.as_ref(){return if source.terminal_is_empty(){Ok(fixed(size_of::<Option<RetainedCloneSource<DslValue>>>()))}else{source.next_close_demand(body)}}macro_rules! owner{($field:ident,$ty:ty)=>{if self.$field.is_some(){return self.retirement.next_owner_demands::<$ty>(true,&None,body)}};}owner!(parser,JsonGrammarCursor<DslValue>);owner!(parsed,DslValue);owner!(output,T);owner!(syntax,JsonError);owner!(failure,NativeReceivingFailure);owner!(local_authority,NativeLocalAuthority<ValueError>);Ok(Default::default())}
 /// ⚖️ Quotes the work of the next cleanup action so the caller can open exactly one effect for it.
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.retirement.is_empty(){return self.retirement.next_owner_work::<DslValue>(false,&None,body)}if let Some(admission)=self.admission.as_ref(){return admission.return_work_demand()}if let Some(receiver)=self.receiver.as_ref(){return if receiver.terminal_is_empty(){Ok(1)}else if receiver.is_closing(){receiver.close_work_demand(body)}else{receiver.begin_close_work_demand()}}if let Some(source)=self.source.as_ref(){return if source.terminal_is_empty(){Ok(1)}else{source.next_close_work_demand(body)}}macro_rules! owner{($field:ident,$ty:ty)=>{if self.$field.is_some(){return self.retirement.next_owner_work::<$ty>(true,&None,body)}};}owner!(parser,JsonGrammarCursor<DslValue>);owner!(parsed,DslValue);owner!(output,T);owner!(syntax,JsonError);owner!(failure,NativeReceivingFailure);owner!(local_authority,NativeLocalAuthority<ValueError>);Ok(0)}
 pub fn close_step(&mut self,control:&mut dyn OriginalRetirementControl,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  use RetirementPollExit as X;
  let g=effect.grant();let body=g.maximum_copy_bytes;
  let result=(||->Result<RetainedCloneStep,X>{
   if self.ledger!=control.ledger_identity()||self.recipient!=control.retirement_recipient_identity()||!self.closing{return Err(invalid().into())}
   if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
   let d=self.close_demands(body)?;if !retained_clone_grant_admits(g,d){return Ok(progress_step(Default::default()))}
   let unit=||progress_step(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()});
   if !self.retirement.is_empty(){return X::resume(self.retirement.step_granted(body,effect))}
   if let Some(admission)=self.admission.as_mut(){let mut returned=None;let turn=admission.return_inputs(&mut returned,effect);if let Some((value,()))=returned{*self.parsed=Some(value)}if admission.terminal_is_empty(){self.admission.take();}return turn.result.map_err(Into::into)}
   if let Some(receiver)=self.receiver.as_mut(){
    if receiver.terminal_is_empty(){self.receiver.take();X::pay(effect,1)?;return Ok(unit())}
    return match close_native_receiver(receiver,body,effect){RetainedWorkPoll::Pending{work}=>Err(X::Pending(work)),RetainedWorkPoll::Complete(turn)=>turn.result.map_err(Into::into)};
   }
   if let Some(source)=self.source.as_mut(){if source.terminal_is_empty(){self.source.take();X::pay(effect,1)?;return Ok(unit())}return X::resume(source.close_step(body,effect))}
   macro_rules! owned{($field:ident)=>{if self.$field.is_some(){let turn=self.retirement.begin_granted(&mut self.$field,effect);if let Some(step)=turn.result?{return Ok(step)}}};}
   owned!(parser);owned!(parsed);owned!(output);owned!(syntax);owned!(failure);owned!(local_authority);Ok(RetainedCloneStep::Complete(Default::default()))
  })();X::settle(effect,result)
 }
 /// ⏳️ Reports an open original Work action that no other cleanup may prepare beside.
 pub fn pending(&self)->bool{self.action.pending()||self.receiver.as_ref().is_some_and(|receiver|receiver.advance_pending())}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.parser.is_none()&&self.parsed.is_none()&&self.admission.is_none()&&self.source.is_none()&&self.receiver.is_none()&&self.output.is_none()&&self.syntax.is_none()&&self.failure.is_none()&&self.local_authority.is_none()&&self.action.terminal_is_empty()&&self.retirement.is_empty()}
}
impl<T:NativeReceive<DslValue>+Sync> Drop for OriginalJsonOutputReceiving<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"unique original JSON output retains every parser, partial typed value and source until paid close")}}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;

#[path="🎟️source/🦀️.rs"]mod source;
pub use source::{OriginalJsonOutputSource,OriginalJsonOutputOutcome,OriginalJsonOutputClose};
pub use semio_framework_value_native_owned::native_decoding::NativeReceivingFailure;
pub use semio_framework_job_native_decode::OriginalReceivingRefusal;
