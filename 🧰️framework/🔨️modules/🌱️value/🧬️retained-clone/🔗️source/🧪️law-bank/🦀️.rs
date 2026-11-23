//! 🧰️ Actual source owner laws debit every turn from the existing neutral finite stock.
use crate::{RetainedCloneSourceAdmission,RetainedCloneSource,RetainedCloneGrant as G,RetainedCloneProgress as P,RetainedWorkOutcome,RetirementDemand,ValueError};
use crate::retirement::RetireOwned;
pub trait LawFailure:std::fmt::Debug{fn progress(&self)->P;}
impl LawFailure for ValueError{fn progress(&self)->P{self.retained_progress()}}
impl LawFailure for crate::OriginalRetirementFailure{fn progress(&self)->P{self.retained_progress()}}
pub struct Producer(pub u64);
impl crate::OriginalWorkRecipient for Producer{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<crate::OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn admits<'reply,'turn,'source>(&mut self,_:&mut (),quote:crate::OriginalWorkQuote<'reply>,observed:u64,additional:u64)->crate::OriginalWorkQuoteComplete<'reply>where 'source:'turn{quote.complete(Ok(observed.checked_add(additional).is_some_and(|total|total<=self.0)))}
 fn settle_partial<'reply,'turn,'source>(&mut self,context:&mut (),check:crate::OriginalWorkPartialCheck<'reply>)->crate::OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(self.validate_context(context))}
 fn recover_partial<'reply,'turn,'source>(&mut self,context:&mut (),check:crate::OriginalWorkPartialCheck<'reply>)->crate::OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(self.validate_context(context))}
}
struct Invoked<F,R>{op:Option<F>,out:Option<R>,marker:Option<usize>}
impl<F:FnOnce(&mut crate::RetainedAllocationEffect<'_>)->R,R> crate::OriginalEffectProducer for Invoked<F,R>{
 type Output=crate::RetainedCloneStep;type Failure=ValueError;type Context<'a,'b>=() where 'b:'a;
 fn validate_context<'a,'b>(&self,_:&())->Result<(),ValueError>where 'b:'a{Ok(())}
 fn source_turn<'a,'b>(&self,_:&())->Option<crate::OriginalSourceTurnIdentity>where 'b:'a{None}
 fn receive<'r,'a,'b>(&mut self,_:&mut (),effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::OriginalEffectReply<'r,crate::RetainedCloneStep,ValueError>)->Result<crate::OriginalEffectReplyComplete<'r,crate::RetainedCloneStep,ValueError>,ValueError>where 'b:'a{*effect.actual()=self.marker;self.out=Some((self.op.take().ok_or_else(||ValueError::invariant("fixture turn has no operation"))?)(effect));self.marker=*effect.actual();Ok(reply.complete(Ok(crate::RetainedCloneStep::Complete(Default::default()))))}
 fn recover<'r,'a,'b>(&mut self,_:&mut (),_:&mut crate::RetainedAllocationEffect<'_>,_:crate::OriginalEffectReply<'r,crate::RetainedCloneStep,ValueError>)->Result<crate::OriginalEffectReplyComplete<'r,crate::RetainedCloneStep,ValueError>,ValueError>where 'b:'a{Err(ValueError::invariant("fixture turn has no original callback to recover"))}
}
thread_local!{static WORK:std::cell::RefCell<Option<semio_framework_value_testing_effect_bank::EffectBank>>=const{std::cell::RefCell::new(None)};}
/// 🪢️ Runs one operation inside one actual original effect, carrying a genuine producer invocation, over the caller's allocation marker and the thread's one persistent original Work, and returns the work the effect performed.
pub fn with_effect<R>(grant:G,maximum:u64,actual:&mut Option<usize>,operation:impl FnOnce(&mut crate::RetainedAllocationEffect<'_>)->R)->(R,u128){
 let marker=*actual;
 WORK.with(|cell|{let mut slot=cell.borrow_mut();let bank=slot.get_or_insert_with(||semio_framework_value_testing_effect_bank::EffectBank::new(semio_framework_value_testing_effect_bank::Wallet::new(G{maximum_items:1<<30,maximum_copy_bytes:usize::MAX>>2,maximum_capacity_bytes:usize::MAX>>2,maximum_release_bytes:usize::MAX>>2,maximum_depth:1<<10},1<<50)));
  let mut out=None;let mut performed=0u128;let mut operation=Some(operation);
  let(_,left)=bank.run(grant,maximum,|effect|{let before=effect.performed_work();let mut authority=crate::OriginalEffectAuthority::new(Invoked{op:operation.take(),out:None,marker});let mut action=None;assert!(authority.drive(&mut (),effect,&mut action).unwrap());let mut answer=None;action.as_mut().unwrap().transfer_reply(&mut answer).unwrap();authority.finish(&mut action).unwrap();assert!(answer.unwrap().is_ok());let mut producer=authority.into_producer().ok().expect("fixture producer settled");out=producer.out.take();*effect.actual()=producer.marker;performed=effect.performed_work()-before;crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok::<(),ValueError>(()),work:performed})});
  *actual=left;let _=bank.take_admitted();(out.expect("fixture operation ran"),performed)})
}
/// 🔎️ Runs one operation inside an original effect and reports its exact physical allocations.
pub fn observed<R>(grant:G,maximum:u64,actual:&mut Option<usize>,operation:impl FnOnce(&mut crate::RetainedAllocationEffect<'_>)->R)->(R,(usize,usize)){with_effect(grant,maximum,actual,|effect|crate::allocation_testing::observe_retirement_allocations(||operation(effect))).0}
/// 🧾️ Generous per-turn work ceiling for fixture turns.
pub const CPU:u64=1<<20;
/// 🏁️ A fixture drives every retirement turn to completion, so a suspended poll is a fixture fault.
pub fn settled<T,E>(poll:crate::RetainedWorkPoll<T,E>)->RetainedWorkOutcome<T,E>{match poll{crate::RetainedWorkPoll::Complete(turn)=>turn,crate::RetainedWorkPoll::Pending{..}=>panic!("original fixture turn stayed pending")}}
/// 🎞️ Drives one original-authority handback reply under a real producer invocation and reports the heap its receipt records.
pub fn drive_take<T:Sync+'static,A:RetireOwned+Sync+'static>(source:&mut RetainedCloneSource<T,A>,output:&mut Option<A>,g:G,cpu:u64)->(RetainedWorkOutcome<crate::RetainedCloneStep,crate::OriginalRetirementFailure>,(usize,usize)){
 semio_framework_value_clone_source_testing::drive::warm();let turn=semio_framework_value_clone_source_testing::drive::drive_reply(g,cpu,|reply:&Result<crate::RetainedCloneStep,crate::OriginalRetirementFailure>|match reply{Ok(step)=>step.progress(),Err(failure)=>failure.retained_progress()},|effect,reply,recover|{let body=effect.grant().maximum_copy_bytes;if recover{source.recover_take_authority_into(output,body,effect,reply)}else{source.take_authority_into(output,body,effect,reply)}});
 let p=match &turn.result{Ok(step)=>step.progress(),Err(failure)=>failure.retained_progress()};let heap=(p.retained_capacity_bytes,p.released_bytes);(turn,heap)
}
/// 🪢️ Retains one issued reference as an owned projection through its two separately funded original turns and reports their total receipt.
pub fn retain<U:?Sized+Sync+'static>(bank:&mut Bank,row:crate::RetainedCloneRef<'_,U>)->(crate::RetainedOwnedProjection<U>,P){
 let mut admission=crate::RetainedOwnedProjectionAdmission::<U>::default();let mut output=None;
 let g=bank.grant(admission.demand(row).unwrap());let first=bank.reply(g,|step:&crate::RetainedCloneStep|step.progress(),|effect,reply,recover|if recover{admission.recover_admit(reply)}else{admission.admit(row,effect,reply)}).result.unwrap().progress();
 let g=bank.grant(admission.take_demand().unwrap());let second=bank.reply(g,|step:&crate::RetainedCloneStep|step.progress(),|effect,reply,recover|if recover{admission.recover_take(reply)}else{admission.take(&mut output,effect,reply)}).result.unwrap().progress();
 assert!(admission.terminal_is_empty());(output.expect("owned projection published"),first.checked_add(second).unwrap())
}
/// 🚫️ A denied projection admission records no progress, performs no work and leaves nothing to close.
pub fn retain_denied<U:?Sized+Sync+'static>(bank:&Bank,row:crate::RetainedCloneRef<'_,U>,g:G){
 let mut admission=crate::RetainedOwnedProjectionAdmission::<U>::default();let _=bank;
 semio_framework_value_clone_source_testing::drive::warm();let turn=semio_framework_value_clone_source_testing::drive::drive_reply(g,CPU,|step:&Result<crate::RetainedCloneStep,ValueError>|match step{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()},|effect,reply,recover|if recover{admission.recover_admit(reply)}else{admission.admit(row,effect,reply)});
 assert_eq!(turn.work,0);assert_eq!(turn.result.map_or(P::default(),|step|step.progress()),P::default());assert!(admission.terminal_is_empty());
}
/// 📜️ Records every actual admission turn's quote, receipt and performed work.
pub struct SourceTrace{pub exact:bool,pub quoted:usize,pub total:P,pub works:Vec<u128>}
impl Default for SourceTrace{fn default()->Self{Self{exact:true,quoted:0,total:P::default(),works:Vec::new()}}}
impl SourceTrace{fn tally(&mut self,d:RetirementDemand,p:P,work:u128){self.exact&=p.copied_bytes==d.copy_bytes;self.quoted+=d.copy_bytes;self.total=self.total.checked_add(p).unwrap();self.works.push(work);}}
pub struct Bank{remaining:G,cpu:u64}
impl Bank{
 pub fn new()->Self{let f:serde_json::Value=serde_json::from_str(include_str!("../🎟️work/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();Self{remaining:serde_json::from_value(f["stock"].clone()).unwrap(),cpu:f["maximumCpuWork"].as_u64().unwrap()}}
 pub fn from_stock(remaining:G,cpu:u64)->Self{Self{remaining,cpu}}
 pub fn grant(&self,d:RetirementDemand)->G{let g=G{maximum_items:usize::from(self.remaining.maximum_items>0&&self.cpu>0),maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};assert!(g.maximum_copy_bytes<=self.remaining.maximum_copy_bytes&&g.maximum_capacity_bytes<=self.remaining.maximum_capacity_bytes&&g.maximum_release_bytes<=self.remaining.maximum_release_bytes&&g.maximum_depth<=self.remaining.maximum_depth);g}
 pub fn observe<R,E:LawFailure>(&mut self,g:G,run:impl FnOnce()->RetainedWorkOutcome<R,E>,receipt:impl FnOnce(&R)->P)->(RetainedWorkOutcome<R,E>,(usize,usize)){let(turn,heap)=crate::allocation_testing::observe_retirement_allocations(run);self.account(g,turn,heap,receipt)}
 pub fn observe_eff<R,E:LawFailure>(&mut self,g:G,run:impl FnOnce(&mut crate::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<R,E>,receipt:impl FnOnce(&R)->P)->(RetainedWorkOutcome<R,E>,(usize,usize)){self.observe_actual(g,&mut None,run,receipt)}
 pub fn observe_actual<R,E:LawFailure>(&mut self,g:G,actual:&mut Option<usize>,run:impl FnOnce(&mut crate::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<R,E>,receipt:impl FnOnce(&R)->P)->(RetainedWorkOutcome<R,E>,(usize,usize)){let(turn,heap)=observed(g,self.cpu,actual,run);self.account(g,turn,heap,receipt)}
 pub fn run_eff<R,E:LawFailure>(&mut self,g:G,run:impl FnOnce(&mut crate::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<R,E>,receipt:impl FnOnce(&R)->P)->(Result<R,E>,(usize,usize)){let(turn,heap)=self.observe_eff(g,run,receipt);(turn.result,heap)}
 pub fn success_eff<R,E:LawFailure>(&mut self,g:G,run:impl FnOnce(&mut crate::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<R,E>,receipt:impl FnOnce(&R)->P)->(R,(usize,usize)){let(turn,heap)=self.observe_eff(g,run,receipt);(turn.result.unwrap(),heap)}
 fn account<R,E:LawFailure>(&mut self,g:G,turn:RetainedWorkOutcome<R,E>,heap:(usize,usize),receipt:impl FnOnce(&R)->P)->(RetainedWorkOutcome<R,E>,(usize,usize)){assert!(turn.work<=u128::from(self.cpu));self.cpu-=u64::try_from(turn.work).unwrap();let p=match &turn.result{Ok(value)=>receipt(value),Err(error)=>error.progress()};assert!(p.fits(g));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));self.remaining.maximum_items=self.remaining.maximum_items.checked_sub(p.copied_items).unwrap();self.remaining.maximum_copy_bytes=self.remaining.maximum_copy_bytes.checked_sub(p.copied_bytes).unwrap();self.remaining.maximum_capacity_bytes=self.remaining.maximum_capacity_bytes.checked_sub(p.retained_capacity_bytes).unwrap();self.remaining.maximum_release_bytes=self.remaining.maximum_release_bytes.checked_sub(p.released_bytes).unwrap();(turn,heap)}
 pub fn reply<R:'static,E:LawFailure+'static>(&mut self,g:G,receipt:impl Fn(&R)->P,call:impl for<'r> FnMut(&mut crate::RetainedAllocationEffect<'_>,crate::OriginalEffectReply<'r,R,E>,bool)->Result<crate::OriginalEffectReplyComplete<'r,R,E>,ValueError>)->RetainedWorkOutcome<R,E>{semio_framework_value_clone_source_testing::drive::warm();let turn=semio_framework_value_clone_source_testing::drive::drive_reply(g,self.cpu,|step:&Result<R,E>|match step{Ok(value)=>receipt(value),Err(error)=>error.progress()},call);let p=match &turn.result{Ok(value)=>receipt(value),Err(error)=>error.progress()};let heap=(p.retained_capacity_bytes,p.released_bytes);self.account(g,turn,heap,&receipt).0}
 pub fn take<T:Sync+'static,A:RetireOwned+Sync+'static>(&mut self,source:&mut RetainedCloneSource<T,A>,output:&mut Option<A>,g:G)->(RetainedWorkOutcome<crate::RetainedCloneStep,crate::OriginalRetirementFailure>,(usize,usize)){let(turn,heap)=drive_take(source,output,g,self.cpu);self.account(g,turn,heap,|step|step.progress())}
 pub fn run<R,E:LawFailure>(&mut self,g:G,run:impl FnOnce()->RetainedWorkOutcome<R,E>,receipt:impl FnOnce(&R)->P)->(Result<R,E>,(usize,usize)){let(turn,heap)=self.observe(g,run,receipt);(turn.result,heap)}
 pub fn success<R,E:LawFailure>(&mut self,g:G,run:impl FnOnce()->RetainedWorkOutcome<R,E>,receipt:impl FnOnce(&R)->P)->(R,(usize,usize)){let(result,heap)=self.run(g,run,receipt);(result.unwrap(),heap)}
 fn turn(&mut self,d:RetirementDemand,actual:&mut Option<usize>,call:impl FnOnce(&mut crate::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<crate::RetainedCloneStep,ValueError>)->(P,u128){let g=self.grant(d);let cpu=self.cpu;let(turn,heap)=observed(g,cpu,actual,call);let(turn,heap)=self.account(g,turn,heap,|step|step.progress());if let Some(marker)=actual.take(){assert_eq!(marker,heap.0)}let work=turn.work;(turn.result.unwrap().progress(),work)}
 pub fn advance_to_ready<T:Sync+'static,A:RetireOwned+Sync+crate::OriginalSourceProjection<T>,B:RetireOwned>(&mut self,admission:&mut RetainedCloneSourceAdmission<T,A,B>)->SourceTrace{
  let mut trace=SourceTrace::default();let mut actual=None;let mut frame:Option<crate::OriginalProjectionFrame<T,A>>=None;
  for _ in 0..1048576{if admission.terminal_is_empty()||admission.phase()=="projectionStart"||admission.phase()=="ready"{break}let d=admission.next_demand().unwrap();let(p,w)=self.turn(d,&mut actual,|effect|admission.advance(effect));trace.tally(d,p,w);}
  if admission.phase()=="projectionStart"{
   let d=admission.begin_projection_demand();let(p,w)=self.turn(d,&mut actual,|effect|admission.begin_projection(&mut frame,effect));trace.tally(d,p,w);
   let selected=frame.as_mut().expect("original projection frame");let d=selected.projection_demand();let units=selected.projection_work_demand().unwrap();let g=self.grant(d);
   let mut action:Option<crate::retained_clone::OriginalEffectAction<crate::RetainedCloneStep,ValueError>>=None;let mut recorded=None;let mut observed_heap=(0usize,0usize);let mut work=0u128;let mut marker=None;let result;
   loop{let(ready,heap)=observed(g,units.max(1),&mut marker,|effect|{let before=effect.performed_work();let mut context=selected.context().expect("original projection context");let ready=selected.authority().expect("original projection authority").drive(&mut context,effect,&mut action).expect("original projection action");(ready,effect.performed_work()-before)});observed_heap.0+=heap.0;observed_heap.1+=heap.1;if let Some(actual)=marker.take(){assert_eq!(actual,observed_heap.0)}work+=ready.1;if ready.0{assert!(action.as_mut().expect("original projection action").transfer_reply(&mut recorded).unwrap());result=recorded.take().expect("original projection reply");selected.authority().unwrap().finish(&mut action).unwrap();break}}
   let outcome=self.account(g,RetainedWorkOutcome{result,work},observed_heap,|step|step.progress()).0;let work=outcome.work;let p=outcome.result.unwrap().progress();trace.tally(d,p,work);
   let selected=frame.as_mut().unwrap();let d=selected.producer_transfer_demand().unwrap();let(p,w)=self.turn(d,&mut actual,|effect|{let turn=selected.transfer_producer(effect);RetainedWorkOutcome{result:turn.result.map(|p|match p{Some(p)=>crate::RetainedCloneStep::Complete(p),None=>crate::RetainedCloneStep::Progress(Default::default())}),work:turn.work}});trace.tally(d,p,w);
   let d=admission.projection_answer_demand(selected).unwrap();let(p,w)=self.turn(d,&mut actual,|effect|admission.accept_projection(selected,effect));trace.tally(d,p,w);
   let d=admission.finish_projection_demand();let(p,w)=self.turn(d,&mut actual,|effect|admission.finish_projection(&mut frame,effect));trace.tally(d,p,w);
   for _ in 0..1048576{if admission.terminal_is_empty()||admission.phase()=="ready"{break}let d=admission.next_demand().unwrap();let(p,w)=self.turn(d,&mut actual,|effect|admission.advance(effect));trace.tally(d,p,w);}
  }
  trace
 }
 pub fn source_traced<T:Sync+'static,A:RetireOwned+Sync+crate::OriginalSourceProjection<T>,B:RetireOwned>(&mut self,mut admission:RetainedCloneSourceAdmission<T,A,B>)->(RetainedCloneSource<T,A>,SourceTrace){
  let mut trace=self.advance_to_ready(&mut admission);let mut output=None;let mut actual=None;
  for _ in 0..1048576{if admission.terminal_is_empty(){break}let d=if admission.phase()=="ready"{admission.take_output_demand()}else{admission.next_demand().unwrap()};let(p,w)=self.turn(d,&mut actual,|effect|if admission.phase()=="ready"{admission.take_output(&mut output,effect)}else{admission.advance(effect)});trace.tally(d,p,w);}
  assert!(admission.terminal_is_empty());(output.unwrap(),trace)
 }
 pub fn source_quoted<T:Sync+'static,A:RetireOwned+Sync+crate::OriginalSourceProjection<T>,B:RetireOwned>(&mut self,admission:RetainedCloneSourceAdmission<T,A,B>)->(RetainedCloneSource<T,A>,P,Option<usize>){let(source,trace)=self.source_traced(admission);(source,trace.total,trace.exact.then_some(trace.quoted))}
 pub fn source<T:Sync+'static,A:RetireOwned+Sync+crate::OriginalSourceProjection<T>,B:RetireOwned>(&mut self,admission:RetainedCloneSourceAdmission<T,A,B>)->(RetainedCloneSource<T,A>,P){let(source,total,_)=self.source_quoted(admission);(source,total)}
 pub fn denied<T:Sync+'static,A:RetireOwned+Sync,B:RetireOwned>(&mut self,admission:&mut RetainedCloneSourceAdmission<T,A,B>,axis:&str){let d=admission.next_demand().unwrap();let g=self.grant(d);let short=match axis{"items"=>G{maximum_items:0,..g},"capacity"=>G{maximum_capacity_bytes:d.capacity_bytes-1,..g},"depth"=>G{maximum_depth:0,..g},_=>panic!("unknown independent source denial axis")};let mut allocation_actual=None;let(turn,heap)=crate::allocation_testing::observe_retirement_allocations(||with_effect(short,self.cpu,&mut allocation_actual,|effect|admission.advance(effect)).0);assert!(allocation_actual.is_none());assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),P::default());assert_eq!(heap,(0,0));}
 pub fn return_inputs<T:Sync+'static,A:RetireOwned+Sync,B:RetireOwned>(&mut self,mut admission:RetainedCloneSourceAdmission<T,A,B>)->(A,B){let mut output=None;for _ in 0..1048576{if admission.terminal_is_empty(){break}let g=self.grant(admission.next_return_demand().unwrap());let cpu=self.cpu;let(result,_)=self.run(g,||with_effect(g,cpu,&mut None,|effect|admission.return_inputs(&mut output,effect)).0,|step|step.progress());result.unwrap();}assert!(admission.terminal_is_empty());output.unwrap()}
}
