//! 🎟️ One genuine source holds only the previously issued currencies of a claimed Job return.
use crate::{StepContext,OriginalTurnFailure,OperationId,Generation};
use semio_framework_value_retained_clone::{RetainedCloneGrant,RetainedCloneProgress as P,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind};
use semio_framework_value_retained_clone::retirement::controlled::{RetainedOwnerGate,RetainedOwnerGateGuard};

fn refused(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
#[derive(Clone,Copy)]
pub struct OriginalReturnSourceInput{source:usize,clock:u64,now:fn()->Option<u64>,deadline:u64,recipient:usize,operation:OperationId,generation:Generation,granted:RetainedCloneGrant,fuel:u64,spent:P,work:u128}
impl OriginalReturnSourceInput{
 pub fn capture(cx:&StepContext<'_>)->Result<Self,ValueError>{
  let parent=cx.original_source_parent()?;parent.validate_borrowed_context(cx)?;let clock=cx.original_clock_binding().ok_or_else(||refused("original return source requires its genuine bound clock"))?;
  if let Some(origin)=cx.return_budget.as_ref().and_then(|budget|budget.get().origin){if origin.clock!=clock||!std::ptr::fn_addr_eq(origin.now,cx.clock_source())||cx.original_absolute_deadline_us()!=origin.deadline{return Err(refused("original return source changed its initiating clock or deadline"))}return Ok(origin)}
  Ok(Self{source:parent.source_identity()?,clock,now:cx.clock_source(),deadline:cx.original_absolute_deadline_us(),recipient:cx.return_budget.as_ref().map(|budget|budget.get()as*const OriginalReturnBudget as usize).ok_or_else(||refused("original return source lacks its preborn receiving budget"))?,operation:cx.operation(),generation:cx.generation(),granted:cx.original_total_retained,fuel:cx.original_total_fuel,spent:super::super::retained_delta(cx.retained_progress(),cx.original_total_retained_start),work:cx.work_units_spent.saturating_sub(cx.original_total_work_start)})
 }
}
/// 🔒️ The actual source owns this fixed counter through every alias and return claim.
pub struct OriginalReturnSource{budget:RetainedOwnerGate<OriginalReturnBudget>}
impl OriginalReturnSource{
 pub const fn empty()->Self{Self{budget:RetainedOwnerGate::new(OriginalReturnBudget::empty())}}
 pub fn try_borrow(&self)->Option<OriginalReturnBudgetReference<'_>>{self.budget.try_lock().ok().map(OriginalReturnBudgetReference::Owned)}
 pub fn retains_claims(&self)->bool{self.budget.try_lock().map_or(true,|budget|budget.claims!=0||budget.held_items!=0||budget.held_fuel!=0)}
 pub fn terminal_is_empty(&self)->bool{self.budget.try_lock().is_ok_and(|budget|budget.origin.is_none()&&budget.held_items==0&&budget.held_fuel==0&&budget.claims==0)}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  let budget=self.budget.get_mut();if budget.origin.is_none(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(P::default())),work:0}}
  if budget.held_items!=0||budget.held_fuel!=0||budget.claims!=0||grant.maximum_items==0||grant.maximum_depth==0{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(P::default())),work:0}}
  *budget=OriginalReturnBudget::empty();RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(P{copied_items:1,..Default::default()})),work:1}
 }
}
impl Drop for OriginalReturnSource{fn drop(&mut self){let budget=self.budget.get_mut();assert!(std::thread::panicking()||budget.origin.is_none()&&budget.held_items==0&&budget.held_fuel==0&&budget.claims==0,"original return source retains its bound budget or live claims");}}
pub enum OriginalReturnBudgetReference<'a>{Owned(RetainedOwnerGateGuard<'a,OriginalReturnBudget>),Borrowed(&'a mut OriginalReturnBudget)}
impl OriginalReturnBudgetReference<'_>{
 pub fn get(&self)->&OriginalReturnBudget{match self{Self::Owned(budget)=>budget,Self::Borrowed(budget)=>budget}}
 pub fn get_mut(&mut self)->&mut OriginalReturnBudget{match self{Self::Owned(budget)=>budget,Self::Borrowed(budget)=>budget}}
}
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct OriginalReturnBudgetSnapshot{pub granted:RetainedCloneGrant,pub fuel:u64,pub spent:P,pub work:u128,pub held_items:usize,pub held_fuel:u64,pub claims:usize,pub serial:u64}
impl OriginalReturnBudgetSnapshot{pub fn conserved(&self)->bool{self.spent.fits(self.granted)&&self.spent.copied_items.checked_add(self.held_items).is_some_and(|items|items<=self.granted.maximum_items)&&self.work.checked_add(u128::from(self.held_fuel)).is_some_and(|work|work<=u128::from(self.fuel))}}
pub struct OriginalReturnBudget{origin:Option<OriginalReturnSourceInput>,spent:P,work:u128,held_items:usize,held_fuel:u64,claims:usize,serial:u64,failure:Option<(ValueRefusalKind,&'static str)>}
impl OriginalReturnBudget{
 const fn empty()->Self{Self{origin:None,spent:P{copied_items:0,copied_bytes:0,retained_capacity_bytes:0,released_bytes:0},work:0,held_items:0,held_fuel:0,claims:0,serial:0,failure:None}}
 pub fn is_bound(&self)->bool{self.origin.is_some()}
 pub fn bind(&mut self,input:OriginalReturnSourceInput)->Result<bool,ValueError>{
  if input.recipient!=self as*const Self as usize{return Err(refused("original return lifetime input requires its same preborn receiving budget"))}
  if let Some(origin)=self.origin{if origin.source!=input.source||origin.clock!=input.clock||!std::ptr::fn_addr_eq(origin.now,input.now)||origin.deadline!=input.deadline||origin.recipient!=input.recipient||origin.operation!=input.operation||origin.generation!=input.generation{return Err(refused("original return source cannot be rebound"))}return Ok(false)}
  if !input.spent.fits(input.granted)||input.work>u128::from(input.fuel){return Err(refused("original return source exceeds its admitting lifetime stock"))}
  self.origin=Some(input);self.spent=input.spent;self.work=input.work;Ok(true)
 }
 pub fn snapshot(&self)->OriginalReturnBudgetSnapshot{let(origin,fuel)=self.origin.map_or((RetainedCloneGrant{maximum_items:0,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:0},0),|origin|(origin.granted,origin.fuel));OriginalReturnBudgetSnapshot{granted:origin,fuel,spent:self.spent,work:self.work,held_items:self.held_items,held_fuel:self.held_fuel,claims:self.claims,serial:self.serial}}
 pub fn refusal(&self)->Option<ValueError>{self.failure.as_ref().map(|(kind,message)|ValueError::literal(kind.clone(),message))}
 pub fn raw_remaining(&self)->Result<(RetainedCloneGrant,u64),ValueError>{if let Some(error)=self.refusal(){return Err(error)}let origin=self.origin.ok_or_else(||refused("original return source has not admitted its lifetime stock"))?;if !self.spent.fits(origin.granted)||self.work>u128::from(origin.fuel){return Err(refused("original return source spent exceeds its original stock"))}Ok((super::super::retained_remaining(origin.granted,self.spent),origin.fuel-u64::try_from(self.work).map_err(|_|refused("original return source CPU extent overflow"))?))}
 pub fn remaining(&self)->Result<(RetainedCloneGrant,u64),ValueError>{let(mut retained,fuel)=self.raw_remaining()?;retained.maximum_items=retained.maximum_items.checked_sub(self.held_items).ok_or_else(||refused("original return holds exceed retained lifetime stock"))?;Ok((retained,fuel.checked_sub(self.held_fuel).ok_or_else(||refused("original return holds exceed original CPU stock"))?))}
 pub fn record_retained(&mut self,progress:P)->Result<(),ValueError>{
  if self.origin.is_none(){return Ok(())}let Some(spent)=self.spent.checked_add(progress).ok()else{self.failure=Some((ValueRefusalKind::OwnershipLimit,"original return source receipt overflow"));return Err(self.refusal().unwrap().with_retained_progress(progress))};self.spent=spent;
  if !self.snapshot().conserved(){self.failure=Some((ValueRefusalKind::OwnershipLimit,"original return source receipt exceeds spent and held authority"));return Err(self.refusal().unwrap().with_retained_progress(progress))}Ok(())
 }
 pub fn record_work(&mut self,work:u64){if self.origin.is_none(){return}match self.work.checked_add(u128::from(work)){Some(total)=>self.work=total,None=>self.failure=Some((ValueRefusalKind::WorkLimit,"original return source CPU counter overflow"))}if !self.snapshot().conserved(){self.failure=Some((ValueRefusalKind::WorkLimit,"original return source CPU exceeds spent and held authority"))}}
 fn claim(&mut self)->Result<OriginalReturnClaimCredit,ValueError>{
  let origin=self.origin.ok_or_else(||refused("original return claim requires its admitted source"))?;let(retained,fuel)=self.remaining()?;if retained.maximum_items<3||fuel<3||retained.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original return claim requires control and both genuine return credits"))}
  let serial=self.serial.checked_add(1).ok_or_else(||refused("original return claim serial exhausted"))?;let claims=self.claims.checked_add(1).ok_or_else(||refused("original return claim count overflow"))?;let held_items=self.held_items.checked_add(2).ok_or_else(||refused("original return item hold overflow"))?;let held_fuel=self.held_fuel.checked_add(2).ok_or_else(||refused("original return CPU hold overflow"))?;self.serial=serial;self.claims=claims;self.held_items=held_items;self.held_fuel=held_fuel;Ok(OriginalReturnClaimCredit{source:origin.source,clock:origin.clock,recipient:origin.recipient,serial,capture:true,receive:true,capture_observed:false,receive_observed:false,live:true})
 }
 fn redeem(&mut self,credit:&mut OriginalReturnClaimCredit,witness:&mut OriginalReturnWitness)->Result<(),ValueError>{
  let origin=self.origin.ok_or_else(||refused("original return source already closed"))?;if !credit.live&&!witness.redeemed||credit.source!=origin.source||credit.clock!=origin.clock||credit.serial!=witness.serial||witness.source!=origin.source||witness.clock!=origin.clock||witness.recipient!=origin.recipient{return Err(refused("original return witness is foreign to its held claim"))}if witness.redeemed{return Ok(())}
  let observed=match witness.phase{OriginalReturnPhase::Capture=>credit.capture_observed,OriginalReturnPhase::Receive=>credit.receive_observed};if observed||match witness.phase{OriginalReturnPhase::Capture=>credit.capture,OriginalReturnPhase::Receive=>credit.receive}{return Err(refused("original return witness was not issued once by its claim"))}
  let held=1+usize::from(witness.release_capture);if self.held_items<held||self.held_fuel<u64::try_from(held).unwrap()||self.claims==0||witness.release_capture&&(witness.phase!=OriginalReturnPhase::Receive||credit.capture_observed){return Err(refused("original return witness lost its genuine held currencies"))}
  let spent=self.spent.checked_add(witness.progress)?;let work=self.work.checked_add(u128::from(witness.work)).ok_or_else(||refused("original reserved return actual CPU overflow"))?;self.held_items-=held;self.held_fuel-=u64::try_from(held).unwrap();self.spent=spent;self.work=work;match witness.phase{OriginalReturnPhase::Capture=>credit.capture_observed=true,OriginalReturnPhase::Receive=>credit.receive_observed=true}if witness.release_capture{credit.capture_observed=true;}if credit.capture_observed&&credit.receive_observed{self.claims-=1;credit.live=false;}witness.redeemed=true;Ok(())
 }

}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum OriginalReturnPhase{Capture,Receive}
pub struct OriginalReturnClaimCredit{source:usize,clock:u64,recipient:usize,serial:u64,capture:bool,receive:bool,capture_observed:bool,receive_observed:bool,live:bool}
pub struct OriginalReturnBodyIdentity{identity:super::super::OriginalRetainedLoanIdentity,clock:Option<u64>,deadline:u64}
impl OriginalReturnBodyIdentity{pub fn capture(cx:&StepContext<'_>)->Self{Self{identity:super::super::OriginalRetainedLoanIdentity::capture(cx),clock:cx.original_clock_binding(),deadline:cx.original_absolute_deadline_us()}}}
impl OriginalReturnClaimCredit{
 pub fn terminal_is_empty(&self)->bool{!self.live&&!self.capture&&!self.receive&&self.capture_observed&&self.receive_observed}
 pub fn has_unused_capture(&self)->bool{self.live&&self.capture&&self.receive}
 pub fn issue_return(&mut self,phase:OriginalReturnPhase)->Result<OriginalReturnWitness,ValueError>{if !self.live||!match phase{OriginalReturnPhase::Capture=>self.capture,OriginalReturnPhase::Receive=>self.receive&&!self.capture}{return Err(refused("original return claim phase was already issued or requires capture"))}match phase{OriginalReturnPhase::Capture=>self.capture=false,OriginalReturnPhase::Receive=>self.receive=false}Ok(self.witness(phase,false))}
 pub fn issue_success_return(&mut self)->Result<OriginalReturnWitness,ValueError>{if !self.has_unused_capture(){return Err(refused("normal original return requires its unused capture and receiving holds"))}self.capture=false;self.receive=false;Ok(self.witness(OriginalReturnPhase::Receive,true))}
 fn witness(&self,phase:OriginalReturnPhase,release_capture:bool)->OriginalReturnWitness{OriginalReturnWitness{source:self.source,clock:self.clock,recipient:self.recipient,serial:self.serial,phase,progress:P{copied_items:1,..Default::default()},work:1,release_capture,redeemed:false,received:false}}

 pub fn qualify(&self,cx:&StepContext<'_>,identity:&OriginalReturnBodyIdentity)->Result<(),OriginalTurnFailure>{
  let context=if !identity.identity.matches(cx)||identity.clock!=cx.original_clock_binding()||identity.deadline!=cx.original_absolute_deadline_us(){Some(refused("original return body changed its initiating context"))}else{None};self.qualify_context(cx,context)
 }
 pub fn qualify_source(&self,cx:&StepContext<'_>,binding:crate::OriginalSourceBinding)->Result<(),OriginalTurnFailure>{self.qualify_context(cx,(!binding.matches(cx)).then(||refused("original return place changed its initiating source")))}
 fn qualify_context(&self,cx:&StepContext<'_>,context:Option<ValueError>)->Result<(),OriginalTurnFailure>{
  let context=context.or_else(||cx.original_source_parent().and_then(|parent|parent.validate_borrowed_context(cx)).err());let budget=cx.return_budget.as_ref().map(|budget|budget.get());let source=match budget.and_then(|budget|budget.origin){Some(origin)if self.live&&origin.source==self.source&&origin.clock==self.clock=>None,_=>Some(refused("original return body lost its claimed source"))};let receipt=budget.and_then(OriginalReturnBudget::refusal);match(context,source,receipt){(Some(error),source,receipt)=>{let mut failure=OriginalTurnFailure::from(error);failure.context_refusal=source;failure.receipt_refusal=receipt;Err(failure)},(None,Some(error),receipt)=>{let mut failure=OriginalTurnFailure::from(error);failure.receipt_refusal=receipt;Err(failure)},(None,None,Some(error))=>Err(error.into()),(None,None,None)=>Ok(())}
 }
}
impl Drop for OriginalReturnClaimCredit{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original return claim retains unspent original currencies");}}
pub struct OriginalReturnWitness{source:usize,clock:u64,recipient:usize,serial:u64,phase:OriginalReturnPhase,progress:P,work:u64,release_capture:bool,redeemed:bool,received:bool}
impl OriginalReturnWitness{pub fn releases_unused_capture(&self)->bool{self.release_capture}}
impl Drop for OriginalReturnWitness{fn drop(&mut self){assert!(std::thread::panicking()||self.received,"original held return witness requires its actual receiving caller");}}
pub fn claim_original_return(cx:&mut StepContext<'_>,output:&mut Option<OriginalReturnClaimCredit>)->Result<bool,OriginalTurnFailure>{
 cx.original_source_parent()?.validate_borrowed_context(cx)?;if let Some(credit)=output{let budget=cx.return_budget.as_ref().ok_or_else(||refused("cached original return claim lacks its genuine budget"))?.get();let origin=budget.origin.ok_or_else(||refused("cached original return claim source is unbound"))?;if !credit.live||credit.source!=origin.source||credit.clock!=origin.clock||credit.serial>budget.serial{return Err(refused("cached original return claim is foreign or already consumed").into())}return Ok(true)}if cx.is_cancelled()||cx.should_yield()||!cx.retained_recipient_is_admitted()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(false)}let budget=cx.return_budget.as_mut().ok_or_else(||refused("original return claim has no genuine source budget"))?;let(available,fuel)=budget.get().remaining()?;if available.maximum_items<3||fuel<3{return Ok(false)}*output=Some(budget.get_mut().claim()?);let progress=P{copied_items:1,..Default::default()};let posted=cx.consume_retained(progress);cx.consume_fuel(1);posted?;Ok(true)
}
pub fn receive_original_return_witness(cx:&mut StepContext<'_>,credit:&mut OriginalReturnClaimCredit,input:&mut Option<OriginalReturnWitness>)->Result<bool,OriginalTurnFailure>{
 let Some(witness)=input.as_mut()else{return Ok(true)};cx.original_source_parent()?.validate_borrowed_context(cx)?;let budget=cx.return_budget.as_ref().ok_or_else(||refused("source-paid return has no genuine receiving source"))?;let origin=budget.get().origin.ok_or_else(||refused("source-paid return receiver source is unbound"))?;if origin.source!=witness.source||origin.clock!=witness.clock||origin.recipient!=witness.recipient||origin.recipient!=budget.get()as*const OriginalReturnBudget as usize||cx.original_clock_binding()!=Some(origin.clock)||!std::ptr::fn_addr_eq(cx.clock_source(),origin.now)||cx.original_absolute_deadline_us()!=origin.deadline{return Err(refused("source-paid return requires its unchanged original receiving source").into())}
 cx.preflight_original_reserved_return(witness.progress,witness.work)?;cx.return_budget.as_mut().unwrap().get_mut().redeem(credit,witness)?;cx.receive_original_reserved_return(witness.progress,witness.work)?;witness.received=true;input.take();Ok(true)
}
