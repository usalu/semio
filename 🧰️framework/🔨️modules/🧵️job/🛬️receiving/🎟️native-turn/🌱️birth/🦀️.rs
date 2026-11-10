//! 🌱️ Source parents birth their return table and publisher through inline paid construction actions, each performed together with its Work settlement.
use super::*;
use semio_framework_job_context::{OriginalReturnBirthState,OriginalTurnFailure,receive_original_retained_work};
use semio_framework_job_return_budget::{OriginalReturnBirthPhase as Phase,OriginalReturnBirthStage as Stage};
use semio_framework_value_control_contract::{OriginalConstructionAcceptStatus,OriginalConstructionAuthority,OriginalConstructionCompletion,OriginalConstructionLoan,OriginalWork,OriginalWorkActionIdentity,OriginalWorkActionProof,OriginalWorkAuthority,OriginalWorkBorrow,RetainedCloneStep,RetainedWorkOutcome};
use semio_framework_value_native_outcome::OriginalConstructionAcceptedProof as Accepted;
use semio_framework_value_native_receiving_frame::NativeReceivingConstructionBinding as Binding;
type Progress=RetainedCloneProgress;
fn invalid()->ValueError{refused("original return birth action is not performable")}
const WAITS:&str="original return birth waits for a wider turn";
fn waits()->ValueError{refused(WAITS)}
fn finish_partial(work:&mut OriginalWorkBorrow<'_>,action:Option<OriginalWorkActionIdentity>,actual:u128,proof:&mut Option<OriginalWorkActionProof>)->Result<(),ValueError>{let mut partial=None;if !work.settle_partial_into(action,&mut partial)?{return Err(refused("original return birth Work settlement pending"))}let(_,_,_,admitted,refusal)=partial.take().ok_or_else(invalid)?.into_parts();if let Some(error)=refusal{return Err(error)}if !admitted{return Err(refused("original return birth Work exceeded its quote"))}if !work.commit(actual,action,proof)?{return Err(refused("original return birth Work commit pending"))}Ok(())}/// 🧾️ Keeps one accepted action's published completion and finished Work proof resident for later fences.
struct Held{completion:Option<OriginalConstructionCompletion>,proof:Option<OriginalWorkActionProof>,progress:Progress,complete:bool}
impl Held{
 fn accepted<'a>(&'a self,work:&OriginalWorkBorrow<'_>,physical:&OriginalConstructionLoan<'_>)->Result<Accepted<'a>,ValueError>{Accepted::authenticate(physical,self.completion.as_ref().ok_or_else(invalid)?,work,self.proof.as_ref().ok_or_else(invalid)?)?.ok_or_else(invalid)}
}
fn begin(work:&mut OriginalWorkBorrow<'_>,physical:&mut OriginalConstructionLoan<'_>,demand:RetirementDemand,units:u64,action:&mut Option<semio_framework_value_control_contract::OriginalConstructionAction>,identity:&mut Option<OriginalWorkActionIdentity>)->Result<semio_framework_value_control_contract::RetainedCloneGrant,ValueError>{
 physical.begin(demand,action,work)?;physical.quote(action.as_mut().ok_or_else(invalid)?)?;let grant=physical.recover_grant(action.as_mut().ok_or_else(invalid)?)?.ok_or_else(invalid)??;
 if work.prepare(units,identity)?!=Some(true){return Err(refused("original return birth lacks original Work"))}Ok(grant)
}
fn settle(work:&mut OriginalWorkBorrow<'_>,physical:&mut OriginalConstructionLoan<'_>,action:&mut semio_framework_value_control_contract::OriginalConstructionAction,identity:Option<OriginalWorkActionIdentity>,claim:&mut Option<Result<u128,ValueError>>,proof:&mut Option<OriginalWorkActionProof>,progress:Progress)->Result<(),ValueError>{
 let actual=claim.take().ok_or_else(invalid)??;finish_partial(work,identity,actual,proof)?;let marker=action.actual();physical.post(action,progress,marker)?;if physical.recover_accept(action)?!=Some(OriginalConstructionAcceptStatus::Accepted){return Err(refused("original return birth action was not accepted"))}Ok(())
}
/// 🧯️ Settles an abandoned Work action through its scope: a live proof is finished, a performed action commits exactly what it performed, and an unperformed one is cancelled, so nothing stays issued and the receipt stays exact.
fn abandon(work:&mut OriginalWorkBorrow<'_>,identity:&mut Option<OriginalWorkActionIdentity>,claim:&mut Option<Result<u128,ValueError>>,proof:&mut Option<OriginalWorkActionProof>){
 if proof.is_some(){let _=work.finish(proof,identity);return}
 if identity.is_some()&&work.cancel(identity).is_err(){let actual=claim.take().and_then(Result::ok).unwrap_or(0);if finish_partial(work,*identity,actual,proof).is_ok(){let _=work.finish(proof,identity);}}
}
/// 💳️ Performs one construction action and its separately paid completion so the accepted proof stays authenticatable.
fn paid(work:&mut OriginalWorkBorrow<'_>,physical:&mut OriginalConstructionLoan<'_>,demand:RetirementDemand,units:u64,run:impl FnOnce(&Binding,&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<Progress>,ValueError>)->Result<Held,ValueError>{
 let(mut original,mut payout)=(None,None);
 let(mut original_work,mut claim,mut proof)=(None,None,None);let(mut pwork,mut pclaim,mut pproof)=(None,None,None);
 let result=(||->Result<Held,ValueError>{
 let mut receipt=None;begin(work,physical,demand,units,&mut original,&mut original_work)?;
 let progress={let mut scope=work.performed_scope(original_work,&mut receipt,&mut claim)?;let mut effect=scope.construction_effect(physical,original.as_mut().ok_or_else(invalid)?,units)?;let binding=Binding::from_effect(&effect)?;run(&binding,&mut effect).result?.ok_or_else(waits)?};
 settle(work,physical,original.as_mut().ok_or_else(invalid)?,original_work,&mut claim,&mut proof,progress)?;work.finish_retained(&mut proof,&mut original_work)?;
 let(mut completion,mut preceipt,mut marker)=(None,None,None);let grant=begin(work,physical,OriginalConstructionCompletion::demand(),PAYOUT_WORK,&mut payout,&mut pwork)?;
 let staged={let witness=physical.witness(payout.as_ref().ok_or_else(invalid)?)?;let mut scope=work.performed_scope(pwork,&mut preceipt,&mut pclaim)?;let mut effect=scope.effect(grant,PAYOUT_WORK,&mut marker)?;physical.stage_completion(&mut original,&mut completion,&witness,&mut effect).result?.ok_or_else(invalid)?};
 settle(work,physical,payout.as_mut().ok_or_else(invalid)?,pwork,&mut pclaim,&mut pproof,staged)?;
 if !physical.publish_completion(completion.as_mut().ok_or_else(invalid)?,payout.as_mut().ok_or_else(invalid)?,work,pproof.as_ref().ok_or_else(invalid)?)?{return Err(refused("original return birth completion was not published"))}
 physical.finish_accepted(&mut payout)?;work.finish(&mut pproof,&mut pwork)?;
 Ok(Held{completion,proof:proof.take(),progress:progress.checked_add(staged)?,complete:false})
 })();
 if result.is_err(){
  for slot in[&mut original,&mut payout]{if let Some(action)=slot.as_mut(){let _=physical.cancel(action,false,Default::default(),None);}if physical.finish(slot).is_err(){let _=physical.finish_accepted(slot);}}
  abandon(work,&mut original_work,&mut claim,&mut proof);abandon(work,&mut pwork,&mut pclaim,&mut pproof);
 }
 result
}
/// 🗄️ Holds the completions one chain can leave behind in a fixed array sized by the chain (one per stage, the refreshed origin, its commit and one fill step), so parking never allocates.
struct Stash([Option<Held>;STASH]);
const STASH:usize=Stage::COUNT+3;
impl Default for Stash{fn default()->Self{Self(std::array::from_fn(|_|None))}}
impl Stash{
 fn is_empty(&self)->bool{self.0.iter().all(Option::is_none)}
 fn push(&mut self,held:Held){let slot=self.0.iter_mut().find(|slot|slot.is_none()).expect("a birth chain parks at most its stage count plus three completions");*slot=Some(held)}
 fn pop(&mut self)->Option<Held>{self.0.iter_mut().find_map(Option::take)}
}
/// 🅿️ Parks a held completion in the birth's stash whenever its chain stops early, so a later turn retires it instead of dropping it.
struct Park<'s>{held:Option<Held>,stash:&'s std::cell::RefCell<Stash>}
impl<'s> Park<'s>{
 fn new(held:Held,stash:&'s std::cell::RefCell<Stash>)->Self{Self{held:Some(held),stash}}
 fn held(&self)->&Held{self.held.as_ref().expect("parked completion is resident until retired")}
}
impl Drop for Park<'_>{fn drop(&mut self){if let Some(held)=self.held.take(){if held.completion.is_some(){self.stash.borrow_mut().push(held)}}}}
/// ♻️ Retires one parked completion through its own paid release action.
fn retire(work:&mut OriginalWorkBorrow<'_>,physical:&mut OriginalConstructionLoan<'_>,park:&mut Park<'_>)->Result<Progress,ValueError>{let mut spent=Progress::default();while let Some(held)=park.held.as_mut().filter(|held|held.completion.is_some()){let step=retire_once(work,physical,held)?;if step==Progress::default()&&held.completion.is_some(){return Err(refused("original return birth completion made no release progress"))}spent=spent.checked_add(step)?}drop(park.held.take());Ok(spent)}
fn retire_once(work:&mut OriginalWorkBorrow<'_>,physical:&mut OriginalConstructionLoan<'_>,held:&mut Held)->Result<Progress,ValueError>{
 let demand=held.completion.as_ref().ok_or_else(invalid)?.release_demand();let(mut payout,mut pwork,mut receipt,mut claim,mut proof,mut marker)=(None,None,None,None,None,None);let grant=begin(work,physical,demand,RETIRE_WORK,&mut payout,&mut pwork)?;
 let progress={let mut scope=work.performed_scope(pwork,&mut receipt,&mut claim)?;let mut effect=scope.effect(grant,RETIRE_WORK,&mut marker)?;physical.retire_completion(&mut held.completion,payout.as_ref().ok_or_else(invalid)?,&mut effect).result?.ok_or_else(invalid)?};
 settle(work,physical,payout.as_mut().ok_or_else(invalid)?,pwork,&mut claim,&mut proof,progress)?;physical.finish_accepted(&mut payout)?;work.finish(&mut proof,&mut pwork)?;Ok(progress)
}
fn stage(context:&OriginalJobNativeTurnContext<'_,'_,'_>,which:Stage,origin:&Accepted<'_>,table:Option<&Accepted<'_>>,birth:Option<&Accepted<'_>>,publisher:Option<&Accepted<'_>>,complete:&mut bool,binding:&Binding,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<Progress>,ValueError>{
 let result=context.with_source(|cx|cx.birth_stage_action(which,origin,table,birth,publisher,binding,effect));match result{Ok(step)=>{*complete=matches!(step,RetainedCloneStep::Complete(_));let progress=step.progress();RetainedWorkOutcome{result:Ok((progress!=Progress::default()).then_some(progress)),work:0}},Err(error)=>RetainedWorkOutcome{result:Err(error),work:0}}
}
fn staged(context:&OriginalJobNativeTurnContext<'_,'_,'_>,work:&mut OriginalWorkBorrow<'_>,physical:&mut OriginalConstructionLoan<'_>,which:Stage,proofs:(&Accepted<'_>,Option<&Accepted<'_>>,Option<&Accepted<'_>>,Option<&Accepted<'_>>))->Result<Held,ValueError>{
 let demand=context.with_source(|cx|cx.return_birth_demand(which))?;let mut complete=false;let mut held=paid(work,physical,demand,context.with_source(|cx|cx.return_birth_work(which))?,|b,e|stage(context,which,proofs.0,proofs.1,proofs.2,proofs.3,&mut complete,b,e))?;held.complete=complete;Ok(held)
}
/// 🔁️ Repeats one filling stage until its backing is complete, retiring only the intermediate completions.
fn filled<'s>(context:&OriginalJobNativeTurnContext<'_,'_,'_>,work:&mut OriginalWorkBorrow<'_>,physical:&mut OriginalConstructionLoan<'_>,which:Stage,proofs:(&Accepted<'_>,Option<&Accepted<'_>>,Option<&Accepted<'_>>,Option<&Accepted<'_>>),spent:&mut Progress,stash:&'s std::cell::RefCell<Stash>)->Result<Park<'s>,ValueError>{
 loop{let mut park=Park::new(staged(context,work,physical,which,proofs)?,stash);*spent=spent.checked_add(park.held().progress)?;if park.held().complete{return Ok(park)}*spent=spent.checked_add(retire(work,physical,&mut park)?)?}
}
const ACTION_WORK:u64=1;
const PAYOUT_WORK:u64=1;
const RETIRE_WORK:u64=1;
/// ⛽️ Quotes each stage's exact Work (origin, origin commit, table, table commit, publisher, publisher commit, release), derived from the actions and builders it will spend on, each including its paid completion and retirements.
pub fn original_return_birth_quote(cx:&StepContext<'_>)->Result<[u64;7],ValueError>{
 let state=cx.return_birth_state()?;if state.origin_current{return Ok([0;7])}
 let table=state.table!=Phase::Live;let publisher=state.publisher_wanted&&state.publisher!=Phase::Live;let held=ACTION_WORK+PAYOUT_WORK;
 Ok([held+RETIRE_WORK,held,if table{cx.return_birth_work(Stage::Table)?+PAYOUT_WORK+RETIRE_WORK}else{0},if table{held}else{0},if publisher{cx.return_birth_work(Stage::Publisher)?+PAYOUT_WORK+RETIRE_WORK}else{0},if publisher{held}else{0},held+RETIRE_WORK*(2+u64::from(table)+u64::from(publisher))])
}
/// 📏️ Sums the demand and item count of every action of one birth chain (the whole refresh when neither table nor publisher is born), each charged by the same quotes the chain pays.
fn chain_demand(cx:&StepContext<'_>,table:bool,publisher:bool)->Result<(usize,RetirementDemand,usize),ValueError>{
 let mut total=RetirementDemand::default();let mut items=0usize;
 let mut add=|count:usize,demand:RetirementDemand|{items+=count;total.copy_bytes=total.copy_bytes.saturating_add(demand.copy_bytes.saturating_mul(count));total.capacity_bytes=total.capacity_bytes.saturating_add(demand.capacity_bytes.saturating_mul(count));total.release_bytes=total.release_bytes.saturating_add(demand.release_bytes.saturating_mul(count));total.depth=total.depth.max(demand.depth)};
 let paid=OriginalConstructionCompletion::demand();let retire=OriginalConstructionCompletion::retire_demand();
 add(1,semio_framework_job_return_budget::OriginalReturnBudget::<semio_framework_job_context::original_return_source::OriginalReturnSourceInput>::refresh_turn_demand());add(1,paid);add(1,cx.return_origin_commit_cost()?);add(1,paid);add(1,retire);
 for(wanted,build,commit)in[(table,Stage::Table,Stage::TableCommit),(publisher,Stage::Publisher,Stage::PublisherCommit)]{if wanted{let(fill,whole)=cx.return_birth_total(build)?;add(1,whole);add(fill.saturating_sub(1),RetirementDemand{depth:whole.depth,..Default::default()});add(1,paid);add(1,cx.return_birth_demand(commit)?);add(1,paid);add(2,retire)}}
 add(1,cx.return_birth_demand(Stage::Release)?);add(1,paid);add(2,retire);
 Ok((items,total,cx.return_origin_commit_demand()?.copy_bytes.saturating_sub(cx.return_origin_commit_cost()?.copy_bytes)))
}
/// 🧮️ Quotes the exact retained progress the next origin-refresh chain charges its Source turn, including the table and publisher births it still owes.
pub fn original_return_refresh_quote(cx:&StepContext<'_>)->Result<RetainedCloneProgress,ValueError>{let state=cx.return_birth_state()?;let(items,demand,_)=chain_demand(cx,state.table!=Phase::Live,state.publisher_wanted&&state.publisher!=Phase::Live)?;Ok(RetainedCloneProgress{copied_items:items,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,released_bytes:demand.release_bytes})}
/// 📏️ Checks that this turn's grant funds the sum of every action of the chain (items, copy, capacity, release, depth), so an atomic chain never starts on a turn it cannot finish.
fn grant_admits(cx:&StepContext<'_>,table:bool,publisher:bool)->Result<bool,ValueError>{
 let grant=cx.retained_grant();let(items,total,slack)=chain_demand(cx,table,publisher)?;
 Ok(grant.maximum_items>=items&&grant.maximum_copy_bytes>=total.copy_bytes.saturating_add(slack)&&grant.maximum_capacity_bytes>=total.capacity_bytes&&grant.maximum_release_bytes>=total.release_bytes&&grant.maximum_depth>=total.depth)
}
/// ♻️ Retires every stashed completion of an earlier stopped chain, stopping quietly while the turn cannot fund the next release.
fn drain(context:&OriginalJobNativeTurnContext<'_,'_,'_>,work:&mut OriginalWorkBorrow<'_>,physical:&mut OriginalConstructionLoan<'_>,stash:&std::cell::RefCell<Stash>)->Result<bool,ValueError>{
 loop{
  let Some(held)=stash.borrow_mut().pop()else{return Ok(true)};let mut park=Park::new(held,stash);
  let demand=park.held().completion.as_ref().map_or(OriginalConstructionCompletion::retire_demand(),|completion|completion.release_demand());
  let funded=context.with_source(|cx|Ok::<_,ValueError>(retained_clone_grant_admits(cx.retained_grant(),demand)&&cx.fuel_remaining()>=RETIRE_WORK&&cx.original_source_parent()?.recorded_clock_allows(cx)?))?;
  if !funded{return Ok(false)}
  retire(work,physical,&mut park)?;
 }
}
fn chain(context:&OriginalJobNativeTurnContext<'_,'_,'_>,work:&mut OriginalWorkBorrow<'_>,physical:&mut OriginalConstructionLoan<'_>,state:OriginalReturnBirthState,stash:&std::cell::RefCell<Stash>)->Result<(Progress,[u64;7]),ValueError>{
 let mut spent=Progress::default();let mut stages=[0u64;7];let table_needed=state.table!=Phase::Live;let publisher_needed=state.publisher_wanted&&state.publisher!=Phase::Live;
 let fuel=||context.with_source(|cx|cx.fuel_remaining());
 if state.origin_current{if table_needed||publisher_needed{return Err(refused("original return birth requires its origin committed in this chain"))}return Ok((spent,stages))}
 let origin_demand=semio_framework_job_return_budget::OriginalReturnBudget::<semio_framework_job_context::original_return_source::OriginalReturnSourceInput>::refresh_turn_demand();
 let mark=fuel();let mut first=Park::new(paid(work,physical,origin_demand,ACTION_WORK,|b,e|context.with_source(|cx|cx.birth_origin_action(b,e)))?,stash);spent=spent.checked_add(first.held().progress)?;
 let commit_demand=context.with_source(|cx|cx.return_origin_commit_demand())?;
 let committed={let accepted=first.held().accepted(work,physical)?;stages[0]+=mark-fuel();let mark=fuel();let held=paid(work,physical,commit_demand,ACTION_WORK,|b,e|context.with_source(|cx|cx.birth_origin_commit_action(&accepted,b,e)))?;stages[1]+=mark-fuel();Park::new(held,stash)};spent=spent.checked_add(committed.held().progress)?;
 let mark=fuel();spent=spent.checked_add(retire(work,physical,&mut first)?)?;stages[0]+=mark-fuel();
 let mut table=None;let mut publisher=None;
 if table_needed{
  let origin=committed.held().accepted(work,physical)?;let mark=fuel();let mut built=filled(context,work,physical,Stage::Table,(&origin,None,None,None),&mut spent,stash)?;stages[2]+=mark-fuel();
  let mark=fuel();let sealed={let birth=built.held().accepted(work,physical)?;Park::new(staged(context,work,physical,Stage::TableCommit,(&origin,None,Some(&birth),None))?,stash)};stages[3]+=mark-fuel();spent=spent.checked_add(sealed.held().progress)?;
  let mark=fuel();spent=spent.checked_add(retire(work,physical,&mut built)?)?;stages[2]+=mark-fuel();table=Some(sealed);
 }
 if publisher_needed{
  let origin=committed.held().accepted(work,physical)?;let table_proof=match table.as_ref(){Some(park)=>Some(park.held().accepted(work,physical)?),None=>None};
  let mark=fuel();let mut built=filled(context,work,physical,Stage::Publisher,(&origin,table_proof.as_ref(),None,None),&mut spent,stash)?;stages[4]+=mark-fuel();
  let mark=fuel();let sealed={let birth=built.held().accepted(work,physical)?;Park::new(staged(context,work,physical,Stage::PublisherCommit,(&origin,table_proof.as_ref(),Some(&birth),None))?,stash)};stages[5]+=mark-fuel();spent=spent.checked_add(sealed.held().progress)?;
  let mark=fuel();spent=spent.checked_add(retire(work,physical,&mut built)?)?;stages[4]+=mark-fuel();publisher=Some(sealed);
 }
 let mark=fuel();let mut released={let origin=committed.held().accepted(work,physical)?;let table_proof=match table.as_ref(){Some(park)=>Some(park.held().accepted(work,physical)?),None=>None};let publisher_proof=match publisher.as_ref(){Some(park)=>Some(park.held().accepted(work,physical)?),None=>None};Park::new(staged(context,work,physical,Stage::Release,(&origin,table_proof.as_ref(),None,publisher_proof.as_ref()))?,stash)};
 spent=spent.checked_add(released.held().progress)?;spent=spent.checked_add(retire(work,physical,&mut released)?)?;
 for mut park in[Some(committed),table,publisher].into_iter().flatten(){spent=spent.checked_add(retire(work,physical,&mut park)?)?}
 stages[6]+=mark-fuel();Ok((spent,stages))
}
/// 🌗️ Reports what one Source turn's birth chain did.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum OriginalJobReturnBirthTurn{Admitting,ClockPending,Unaffordable{needed:u64},Narrow,Idle,Born{receipt:RetainedCloneProgress,stages:[u64;7]}}
/// 🌱️ Resident birth driver of one Source parent; it owns only the two paid native authorities and rebuilds each turn's chain from the budget phases.
#[derive(Default)]
pub struct OriginalJobReturnBirth{work:Option<OriginalWorkAuthority<OriginalJobNativeWork>>,construction:Option<OriginalConstructionAuthority<OriginalJobNativeConstruction>>,work_loan:Option<OriginalCallerSourceLoan>,construction_loan:Option<OriginalCallerSourceLoan>,work_born:Option<OriginalJobNativeWork>,construction_born:Option<OriginalJobNativeConstruction>,deadline:Option<semio_framework_job_context::OriginalClockDeadline>,stash:Stash}
impl OriginalJobReturnBirth{
 pub fn new()->Self{Self::default()}
 /// 🌱️ Production host entry: births the return table and publisher whenever a caller requested them, before the turn's business runs.
 pub fn service(&mut self,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{if cx.return_birth_requested(){self.advance(cx)?;}Ok(())}
 pub fn authorities_ready(&self)->bool{self.work.is_some()&&self.construction.is_some()}
 pub fn terminal_is_empty(&self)->bool{self.work.is_none()&&self.construction.is_none()&&self.work_loan.is_none()&&self.construction_loan.is_none()&&self.work_born.is_none()&&self.construction_born.is_none()&&self.stash.is_empty()}
 fn paid_step<R>(cx:&mut StepContext<'_>,run:impl FnOnce(&mut StepContext<'_>,&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<Progress>,ValueError>,_marker:std::marker::PhantomData<R>)->Result<bool,OriginalTurnFailure>{
  let grant=cx.retained_grant();let Some((outcome,_))=cx.with_original_source_effect(grant,run)?else{return Ok(false)};let done=matches!(outcome.result,Ok(Some(_)));
  receive_original_retained_work(cx,RetainedWorkOutcome{result:outcome.result.map(|progress|progress.map_or(RetainedCloneStep::Progress(Default::default()),RetainedCloneStep::Complete)),work:outcome.work})?;Ok(done)
 }
 fn admit_authorities(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.work.is_none(){
   if self.work_born.is_none(){if self.work_loan.is_none(){cx.original_source_parent()?.admit_loan(&mut self.work_loan,cx)?;return Ok(false)}let(loan,born_slot)=(&mut self.work_loan,&mut self.work_born);Self::paid_step::<()>(cx,|cx,e|OriginalJobNativeWork::admit(loan,born_slot,cx,e),Default::default())?;return Ok(false)}
   let(born,authority)=(&mut self.work_born,&mut self.work);Self::paid_step::<()>(cx,|cx,e|{let lock=RetainedOwnerGate::new(cx);let context=OriginalJobNativeTurnContext::new(&lock);let observed:&dyn OriginalJobNativeContext=&context;OriginalWorkAuthority::admit_from(born,authority,&observed,e)},Default::default())?;return Ok(false)
  }
  if self.construction.is_none(){
   if self.construction_born.is_none(){if self.construction_loan.is_none(){cx.original_source_parent()?.admit_loan(&mut self.construction_loan,cx)?;return Ok(false)}let(loan,born_slot)=(&mut self.construction_loan,&mut self.construction_born);Self::paid_step::<()>(cx,|cx,e|OriginalJobNativeConstruction::admit(loan,born_slot,cx,e),Default::default())?;return Ok(false)}
   let(born,authority)=(&mut self.construction_born,&mut self.construction);Self::paid_step::<()>(cx,|cx,e|{let lock=RetainedOwnerGate::new(cx);let context=OriginalJobNativeTurnContext::new(&lock);let observed:&dyn OriginalJobNativeContext=&context;OriginalConstructionAuthority::admit_from(born,authority,&observed,e)},Default::default())?;return Ok(false)
  }
  Ok(true)
 }
 /// 🌱️ Advances the admission of both authorities and, once affordable, the whole birth chain of this Source turn.
 fn clock_step<R>(cx:&mut StepContext<'_>,run:impl FnOnce(&mut StepContext<'_>,&mut RetainedAllocationEffect<'_>)->Result<R,ValueError>)->Result<Option<R>,OriginalTurnFailure>{let grant=cx.retained_grant();match cx.with_original_source_effect(grant,run)?{Some((result,_))=>Ok(Some(result?)),None=>Ok(None)}}
 fn clock_begin(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  use semio_framework_job_context::OriginalClockDecisionStatus as Status;
  if cx.original_source_parent()?.recorded_clock_allows(cx).is_ok(){return Ok(true)}
  let deadline=match self.deadline{Some(deadline)=>deadline,None=>{let mut reading=None;let mut ready=false;for _ in 0..4{match Self::clock_step(cx,|cx,e|cx.original_source_parent()?.poll_recorded_clock(cx,e))?{Some(Status::Ready)=>{ready=true;break},Some(Status::Pending)|None=>continue,Some(Status::Refused)=>return Err(refused("original Source clock reading was refused").into())}}if !ready{return Ok(false)}
  let parent=cx.original_source_parent()?;if !parent.take_recorded_clock_reading(cx,&mut reading)?{return Err(refused("original Source clock reading is not ready").into())}parent.finish_recorded_clock_reading(cx)?;
  let observation=reading.take().ok_or_else(||refused("original Source clock reading is absent"))??;let deadline=parent.recorded_clock_deadline(cx,observation,cx.original_absolute_deadline_us())?;self.deadline=Some(deadline);deadline}};
  for _ in 0..4{match Self::clock_step(cx,|cx,e|cx.original_source_parent()?.poll_recorded_clock_decision(cx,deadline,e))?{Some(Status::Ready)=>return Ok(true),Some(Status::Pending)|None=>continue,Some(Status::Refused)=>return Err(refused("original Source clock decision was refused").into())}}
  Ok(false)
 }
 fn clock_end(cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{let parent=cx.original_source_parent()?;let mut decision=None;if parent.take_recorded_clock_decision(cx,&mut decision)?{parent.finish_recorded_clock_decision(cx)?}Ok(())}
 /// 🌱️ Installs the Source clock, runs the per-turn clock protocol, then advances admission and, once affordable, the whole birth chain of this Source turn.
 pub fn advance(&mut self,cx:&mut StepContext<'_>)->Result<OriginalJobReturnBirthTurn,OriginalTurnFailure>{
  if cx.original_source_parent()?.recorded_clock_demand(cx).is_err(){Self::paid_step::<()>(cx,|cx,e|{let mut host=None;match cx.original_source_parent(){Ok(parent)=>parent.admit_system_clock(cx,&mut host,e),Err(error)=>RetainedWorkOutcome{result:Err(error.into()),work:0}}},Default::default())?;}
  if !self.clock_begin(cx)?{let parent=cx.original_source_parent()?;let _=parent.cancel_unstarted_recorded_clock_decision(cx);let _=parent.cancel_unstarted_recorded_clock(cx);return Ok(OriginalJobReturnBirthTurn::ClockPending)}let result=self.advance_inner(cx);let ended=Self::clock_end(cx);let step=result?;ended?;Ok(step)
 }
 /// 🔗️ Lends the resident Work and construction authorities, the Source context and the completion stash to one closure of this turn.
 fn with_chain<T>(&mut self,cx:&mut StepContext<'_>,run:impl FnOnce(&OriginalJobNativeTurnContext<'_,'_,'_>,&mut OriginalWorkBorrow<'_>,&mut OriginalConstructionLoan<'_>,&std::cell::RefCell<Stash>)->Result<T,ValueError>)->Result<Result<T,ValueError>,OriginalTurnFailure>{
  let lock=RetainedOwnerGate::new(&mut *cx);let context=OriginalJobNativeTurnContext::new(&lock);let observed:&dyn OriginalJobNativeContext=&context;
  let work=self.work.as_mut().ok_or_else(||refused("original return birth Work authority is absent"))?;let physical=self.construction.as_mut().ok_or_else(||refused("original return birth construction authority is absent"))?;
  let stash=std::cell::RefCell::new(std::mem::take(&mut self.stash));let(mut work_context,mut physical_context)=(observed,observed);
  let outcome=work.with_loan(&mut work_context,|work_loan|physical.with_loan(&mut physical_context,|physical_loan|{let mut loan=work_loan.reborrow();let mut original=OriginalWork::empty();original.recipient=Some(&mut loan);let mut borrow=OriginalWorkBorrow::new(&mut original);run(&context,&mut borrow,physical_loan,&stash)}));
  self.stash=stash.into_inner();drop(lock);Ok(outcome??)
 }
 fn advance_inner(&mut self,cx:&mut StepContext<'_>)->Result<OriginalJobReturnBirthTurn,OriginalTurnFailure>{
  let mut steps=0;while !self.admit_authorities(cx)?{steps+=1;if steps>16{return Ok(OriginalJobReturnBirthTurn::Admitting)}}
  if !self.stash.is_empty()&&!self.with_chain(cx,drain)??{return Ok(OriginalJobReturnBirthTurn::Narrow)}
  let state=cx.return_birth_state()?;if state.origin_current&&state.table==Phase::Live&&(state.publisher==Phase::Live||!state.publisher_wanted){return Ok(OriginalJobReturnBirthTurn::Idle)}
  let needed=original_return_birth_quote(cx)?.iter().sum::<u64>();if cx.fuel_remaining()<needed{return Ok(OriginalJobReturnBirthTurn::Unaffordable{needed})}if !grant_admits(cx,state.table!=Phase::Live,state.publisher_wanted&&state.publisher!=Phase::Live)?{return Ok(OriginalJobReturnBirthTurn::Narrow)}
  let spent=self.with_chain(cx,|context,work,physical,stash|chain(context,work,physical,state,stash))?;let spent=match spent{Ok(spent)=>spent,Err(error)=>{if error.message==WAITS{return Ok(OriginalJobReturnBirthTurn::Narrow)}for _ in 0..8{let st=cx.return_birth_state()?;if st.table!=Phase::Building&&st.publisher!=Phase::Building{break}Self::paid_step::<()>(cx,|cx,e|cx.cancel_return_birth(e),Default::default())?;}return Err(error.into())}};Ok(OriginalJobReturnBirthTurn::Born{receipt:spent.0,stages:spent.1})
 }
 /// ♻️ Returns both native authorities and their Source loans under paid Source actions; true once nothing remains.
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.terminal_is_empty(){return Ok(true)}
  if !self.stash.is_empty(){
   if !self.clock_begin(cx)?{let parent=cx.original_source_parent()?;let _=parent.cancel_unstarted_recorded_clock_decision(cx);let _=parent.cancel_unstarted_recorded_clock(cx);return Ok(false)}
   let drained=self.with_chain(cx,drain);let ended=Self::clock_end(cx);drained??;ended?;return Ok(false)
  }
  if self.work.is_some(){let(authority,born)=(&mut self.work,&mut self.work_born);Self::paid_step::<()>(cx,|cx,e|{let lock=RetainedOwnerGate::new(cx);let context=OriginalJobNativeTurnContext::new(&lock);let observed:&dyn OriginalJobNativeContext=&context;OriginalWorkAuthority::return_into(authority,born,&observed,e)},Default::default())?;return Ok(false)}
  if self.construction.is_some(){let(authority,born)=(&mut self.construction,&mut self.construction_born);Self::paid_step::<()>(cx,|cx,e|{let lock=RetainedOwnerGate::new(cx);let context=OriginalJobNativeTurnContext::new(&lock);let observed:&dyn OriginalJobNativeContext=&context;OriginalConstructionAuthority::return_into(authority,born,&observed,e)},Default::default())?;return Ok(false)}
  if self.work_born.is_some(){let(born,loan)=(&mut self.work_born,&mut self.work_loan);Self::paid_step::<()>(cx,|cx,e|OriginalJobNativeWork::return_into(born,loan,cx,e),Default::default())?;return Ok(false)}
  if self.construction_born.is_some(){let(born,loan)=(&mut self.construction_born,&mut self.construction_loan);Self::paid_step::<()>(cx,|cx,e|OriginalJobNativeConstruction::return_into(born,loan,cx,e),Default::default())?;return Ok(false)}
  for slot in[&mut self.work_loan,&mut self.construction_loan]{if let Some(loan)=slot.as_mut(){let parent=cx.original_source_parent()?;loan.close_step(parent,cx)?;if loan.terminal_is_empty(){slot.take();}return Ok(false)}}
  Ok(true)
 }
}
