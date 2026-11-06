//! 🪙️ Typed return-budget storage, conservation and physical closure without context capability minting.
use semio_framework_value_retained_clone::{RetainedCloneGrant,RetainedCloneProgress as P,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind};
use semio_framework_value_retained_clone::retirement::controlled::{RetainedOwnerGate,RetainedOwnerGateGuard};
use semio_framework_job_return_escrow_publication::{OriginalEscrowPublisher,OriginalEscrowPublication};
use semio_framework_job_payload::{OriginalReturnEscrowTable,OriginalReturnEscrowMessages as E,OriginalReturnEscrowAccounting as M,OriginalReturnEscrowRow,OriginalReturnEscrowIdentity,ORIGINAL_RETURN_ESCROW_RECLAIM_COPY_BYTES};
pub use semio_framework_job_payload::{OriginalReturnBudgetSnapshot,OriginalReturnEscrowAmounts};
const RETURN_SOURCE_CONTROL_WORK:u64=1;
fn refused(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
/// 🪪️ Data-only origin observations; the context owns the actual Source-qualified input constructor.
pub trait OriginalReturnOrigin:Copy{
 fn same_origin(&self,other:&Self)->bool;
 fn recipient(&self)->usize;
 fn source_identity(&self)->usize;
 fn recipient_identity(&self)->usize{self.recipient()}
 fn granted(&self)->RetainedCloneGrant;
 fn fuel(&self)->u64;
 fn spent(&self)->P;
 fn work(&self)->u128;
}
/// 🔒️ See `README.md`.
pub struct OriginalReturnSource<I:OriginalReturnOrigin>{budget:RetainedOwnerGate<OriginalReturnBudget<I>>}
impl<I:OriginalReturnOrigin> OriginalReturnSource<I>{
 pub const fn empty(birth:u64)->Self{Self{budget:RetainedOwnerGate::new(OriginalReturnBudget::<I>::empty(birth))}}
 pub fn try_borrow(&self)->Option<OriginalReturnBudgetReference<'_,I>>{self.budget.try_lock().ok().map(OriginalReturnBudgetReference::Owned)}
 pub fn guards_header_pointer(&self,pointer:usize)->Result<bool,ValueError>{let b=self.try_borrow().ok_or_else(||refused("header Source busy"))?;Ok(b.get().publisher.as_ref().is_some_and(|p|p.owns_header_pointer(pointer)))}
 pub fn retains_claims(&self)->bool{self.budget.try_lock().map_or(true,|budget|budget.publication.is_some()||budget.claims!=0||budget.held_items!=0||budget.held_copy_bytes!=0||budget.held_release_bytes!=0||budget.held_capacity_bytes!=0||budget.held_fuel!=0||budget.table.as_ref().is_some_and(|table|!table.is_empty()))}
 pub fn terminal_is_empty(&self)->bool{self.budget.try_lock().is_ok_and(|budget|budget.origin.is_none()&&budget.table.is_none()&&budget.publisher.is_none()&&budget.publication.is_none()&&!budget.publisher_placed&&!budget.publisher_fenced&&budget.held_items==0&&budget.held_copy_bytes==0&&budget.held_release_bytes==0&&budget.held_capacity_bytes==0&&budget.held_fuel==0&&budget.claims==0)}
 /// 🧮️ Quotes only the same selected physical close primitive that close_step can execute.
 pub fn retirement_work_demand(&self)->Result<u64,ValueError>{let budget=self.try_borrow().ok_or_else(||refused("return-source close quote is busy"))?;let b=budget.get();if b.publication.is_some(){return Ok(0)}if let Some(publisher)=b.publisher.as_ref(){return Ok(if publisher.terminal_is_empty(){OriginalEscrowPublisher::retirement_demand().2}else{0})}if let Some(table)=b.table.as_ref(){return Ok(if table.has_reclaimable()||table.is_empty(){RETURN_SOURCE_CONTROL_WORK}else{0})}Ok(if b.origin.is_some()&&b.held_items==0&&b.held_copy_bytes==0&&b.held_release_bytes==0&&b.held_capacity_bytes==0&&b.held_fuel==0&&b.claims==0{RETURN_SOURCE_CONTROL_WORK}else{0})}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  let budget=self.budget.get_mut();if budget.publication.is_some(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(P::default())),work:0}}if let Some(publisher)=budget.publisher.as_ref(){let(copy,release,work)=OriginalEscrowPublisher::retirement_demand();let copy=copy+2*std::mem::size_of::<bool>();if !publisher.terminal_is_empty()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copy||grant.maximum_release_bytes<release{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(P::default())),work:0}}drop(budget.publisher.take());budget.publisher_placed=false;budget.publisher_fenced=false;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(P{copied_items:1,copied_bytes:copy,released_bytes:release,..Default::default()})),work}}if let Some(table)=budget.table.as_ref(){if table.has_reclaimable(){if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<ORIGINAL_RETURN_ESCROW_RECLAIM_COPY_BYTES{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(P::default())),work:0}}let result=budget.table.as_mut().unwrap().reclaim_one();return RetainedWorkOutcome{result:result.map(|_|RetainedCloneStep::Progress(P{copied_items:1,copied_bytes:ORIGINAL_RETURN_ESCROW_RECLAIM_COPY_BYTES,..Default::default()})),work:RETURN_SOURCE_CONTROL_WORK}}let bytes=std::mem::size_of::<OriginalReturnEscrowTable>();let copy=std::mem::size_of::<Option<Box<OriginalReturnEscrowTable>>>();if !table.is_empty()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copy||grant.maximum_release_bytes<bytes{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(P::default())),work:0}}drop(budget.table.take());return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(P{copied_items:1,copied_bytes:copy,released_bytes:bytes,..Default::default()})),work:RETURN_SOURCE_CONTROL_WORK}}if budget.origin.is_none(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(P::default())),work:0}}
  if budget.held_items!=0||budget.held_copy_bytes!=0||budget.held_release_bytes!=0||budget.held_capacity_bytes!=0||budget.held_fuel!=0||budget.claims!=0||grant.maximum_items==0||grant.maximum_depth==0{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(P::default())),work:0}}
  *budget=OriginalReturnBudget::<I>::empty(budget.birth);RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(P{copied_items:1,..Default::default()})),work:RETURN_SOURCE_CONTROL_WORK}
 }
}
impl<I:OriginalReturnOrigin> Drop for OriginalReturnSource<I>{fn drop(&mut self){let budget=self.budget.get_mut();assert!(std::thread::panicking()||budget.origin.is_none()&&budget.table.is_none()&&budget.publisher.is_none()&&budget.publication.is_none()&&!budget.publisher_placed&&!budget.publisher_fenced&&budget.held_items==0&&budget.held_copy_bytes==0&&budget.held_release_bytes==0&&budget.held_capacity_bytes==0&&budget.held_fuel==0&&budget.claims==0,"original return source retains its bound budget or live claims");}}
pub enum OriginalReturnBudgetReference<'a,I:OriginalReturnOrigin>{Owned(RetainedOwnerGateGuard<'a,OriginalReturnBudget<I>>),Borrowed(&'a mut OriginalReturnBudget<I>)}
impl<I:OriginalReturnOrigin> OriginalReturnBudgetReference<'_,I>{
 pub fn get(&self)->&OriginalReturnBudget<I>{match self{Self::Owned(budget)=>budget,Self::Borrowed(budget)=>budget}}
 pub fn get_mut(&mut self)->&mut OriginalReturnBudget<I>{match self{Self::Owned(budget)=>budget,Self::Borrowed(budget)=>budget}}
}
#[derive(Clone,Copy,PartialEq,Eq)]
pub enum OriginalReturnDelivery{LocalWitness,CoreWireSnapshot}
pub struct OriginalReturnBudget<I:OriginalReturnOrigin>{scope:stock::OriginalReturnScope<I>,pub birth:u64,pub delivery:OriginalReturnDelivery,pub table:Option<Box<OriginalReturnEscrowTable>>,pub publisher:Option<Box<OriginalEscrowPublisher>>,pub publication:Option<OriginalEscrowPublication>,pub publisher_placed:bool,pub publisher_fenced:bool,pub origin:Option<I>,pub spent:P,pub work:u128,pub held_items:usize,pub held_copy_bytes:usize,pub held_release_bytes:usize,pub held_capacity_bytes:usize,pub held_fuel:u64,pub claims:u8,pub serial:u64,pub failure:Option<(ValueRefusalKind,&'static str)>}
impl<I:OriginalReturnOrigin> OriginalReturnBudget<I>{
 pub const fn empty(birth:u64)->Self{Self{scope:stock::OriginalReturnScope::empty(),birth,delivery:OriginalReturnDelivery::LocalWitness,table:None,publisher:None,publication:None,publisher_placed:false,publisher_fenced:false,origin:None,spent:P{copied_items:0,copied_bytes:0,retained_capacity_bytes:0,released_bytes:0},work:0,held_items:0,held_copy_bytes:0,held_release_bytes:0,held_capacity_bytes:0,held_fuel:0,claims:0,serial:0,failure:None}}
 pub fn is_bound(&self)->bool{self.origin.is_some()}
 pub fn bind(&mut self,input:I)->Result<bool,ValueError>{
  if input.recipient()!=self as*const Self as usize{return Err(refused("original return lifetime input requires its same preborn receiving budget"))}
  if let Some(origin)=self.origin{if !origin.same_origin(&input){return Err(refused("original return source cannot be rebound"))}return Ok(false)}
  if !input.spent().fits(input.granted())||input.work()>u128::from(input.fuel()){return Err(refused("original return source exceeds its admitting lifetime stock"))}
  self.origin=Some(input);self.spent=input.spent();self.work=input.work();Ok(true)
 }
 /// 🔁️ See `README.md`.
 pub fn renew_turn(&mut self)->Result<(),ValueError>{
  let origin=self.origin.ok_or_else(||refused(E::STOCK_ABSENT))?;
  if self.claims!=0||self.held_items!=0||self.held_copy_bytes!=0||self.held_release_bytes!=0||self.held_capacity_bytes!=0||self.held_fuel!=0||self.failure.is_some(){return Err(refused("original return source with live claims cannot renew its turn"))}
  self.spent=origin.spent();self.work=origin.work();Ok(())
 }
 pub fn snapshot(&self)->OriginalReturnBudgetSnapshot{let(origin,fuel)=self.origin.map_or((RetainedCloneGrant{maximum_items:0,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:0},0),|origin|(origin.granted(),origin.fuel()));OriginalReturnBudgetSnapshot{granted:origin,fuel,spent:self.spent,work:self.work,held_items:usize::from(self.held_items),held_copy_bytes:self.held_copy_bytes,held_release_bytes:self.held_release_bytes,held_capacity_bytes:self.held_capacity_bytes,held_fuel:u64::from(self.held_fuel),claims:usize::from(self.claims),serial:self.serial}}
 pub fn refusal(&self)->Option<ValueError>{self.failure.as_ref().map(|(kind,message)|ValueError::literal(kind.clone(),message))}
 pub fn raw_remaining(&self)->Result<(RetainedCloneGrant,u64),ValueError>{if let Some(error)=self.refusal(){return Err(error)}self.origin.ok_or_else(||refused(E::STOCK_ABSENT))?;self.snapshot().raw_remaining()}
 pub fn remaining(&self)->Result<(RetainedCloneGrant,u64),ValueError>{if let Some(error)=self.refusal(){return Err(error)}self.origin.ok_or_else(||refused(E::STOCK_ABSENT))?;self.snapshot().remaining()}
 pub fn record_retained(&mut self,progress:P)->Result<(),ValueError>{if self.origin.is_none(){return Ok(())}let mut state=self.snapshot();let issue=state.advance_retained(progress);self.spent=state.spent;if let Some(issue)=issue{self.failure=Some(issue.failure());return Err(self.refusal().unwrap().with_retained_progress(progress))}Ok(())}
 pub fn record_work(&mut self,work:u64){if self.origin.is_none(){return}let mut state=self.snapshot();let issue=state.advance_work(work);self.work=state.work;if let Some(issue)=issue{self.failure=Some(issue.failure())}}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum OriginalReturnPhase{Capture,Receive}
pub struct OriginalReturnClaimState{pub source:usize,pub serial:u64,pub slot:usize,pub local:bool,pub capture:bool,pub receive:bool,pub capture_observed:bool,pub receive_observed:bool,pub live:bool}
pub struct OriginalReturnWitnessState{pub source:usize,pub serial:u64,pub phase:OriginalReturnPhase,pub progress:P,pub work:u64,pub release_capture:bool,pub redeemed:bool,pub received:bool}
impl OriginalReturnClaimState{
 pub fn terminal_is_empty(&self)->bool{!self.live&&!self.capture&&!self.receive&&self.capture_observed&&self.receive_observed}
 pub fn has_unused_capture(&self)->bool{self.live&&self.capture&&self.receive}
 pub fn issue_return(&mut self,phase:OriginalReturnPhase,copy_bytes:usize)->Result<OriginalReturnWitnessState,ValueError>{if !self.live||!match phase{OriginalReturnPhase::Capture=>self.capture,OriginalReturnPhase::Receive=>self.receive&&!self.capture}{return Err(refused("original return claim phase was already issued or requires capture"))}let witness=self.witness(phase,false,copy_bytes)?;match phase{OriginalReturnPhase::Capture=>self.capture=false,OriginalReturnPhase::Receive=>self.receive=false}Ok(witness)}
 pub fn cancel_preissued(&mut self,w:&mut OriginalReturnWitnessState)->Result<(),ValueError>{if !self.live||self.capture||!self.receive||self.capture_observed||self.receive_observed||w.source!=self.source||w.serial!=self.serial||w.phase!=OriginalReturnPhase::Capture||w.redeemed||w.received{return Err(refused("original preissued capture cannot be cancelled"))}let copy=M::witness_copy(false,self.local,0)?;w.phase=OriginalReturnPhase::Receive;w.progress.copied_bytes=copy;w.release_capture=true;self.receive=false;Ok(())}
 pub fn issue_success_return(&mut self,copy_bytes:usize)->Result<OriginalReturnWitnessState,ValueError>{if !self.has_unused_capture(){return Err(refused("normal original return requires its unused capture and receiving holds"))}let witness=self.witness(OriginalReturnPhase::Receive,true,copy_bytes)?;self.capture=false;self.receive=false;Ok(witness)}
 fn witness(&self,phase:OriginalReturnPhase,release_capture:bool,copy_bytes:usize)->Result<OriginalReturnWitnessState,ValueError>{let copied=M::witness_copy(phase==OriginalReturnPhase::Capture,self.local,copy_bytes)?;Ok(OriginalReturnWitnessState{source:self.source,serial:self.serial,phase,progress:P{copied_items:1,copied_bytes:copied,..Default::default()},work:1,release_capture,redeemed:false,received:false})}

}

impl<I:OriginalReturnOrigin> OriginalReturnBudget<I>{
 pub fn claim(&mut self,copy_bytes:usize,destination:usize,epoch:u64)->Result<Option<OriginalReturnClaimCredit<I>>,ValueError>{
  let origin=self.origin.ok_or_else(||refused("original return claim requires its admitted source"))?;if origin.recipient_identity()!=self as*const Self as usize{return Err(refused("original return claim requires its actual bound budget address"))}if let Some(error)=self.refusal(){return Err(error)}let local=self.delivery==OriginalReturnDelivery::LocalWitness;if destination==0||!local&&epoch==0{return Err(refused("original return claim lacks its genuine currencies, epoch or destination"))}let overhead=2*M::UPDATE+M::FINAL+if local{M::LOCAL_ACK}else{0};let plan=self.snapshot().plan_claim(copy_bytes,original_return_claim_copy_bytes::<I>(),overhead)?;
  let identity=OriginalReturnEscrowIdentity{source:origin.source_identity() as u64,source_birth:self.birth,recipient:origin.recipient_identity() as u64,destination:destination as u64,extent:copy_bytes as u64,serial:plan.serial,origin_epoch:epoch};let held=OriginalReturnEscrowAmounts{items:2,copy_bytes:plan.reserved_copy as u64,work:2,release_bytes:0,capacity_bytes:0};let Some(slot)=self.table.as_mut().ok_or_else(||refused("original return table has not been admitted"))?.reserve(identity,held)?else{return Ok(None)};
  self.held_copy_bytes=plan.held_copy_bytes;self.serial=plan.serial;self.claims=plan.claims;self.held_items=plan.held_items;self.held_fuel=plan.held_fuel;Ok(Some(OriginalReturnClaimCredit::<I>{_origin:std::marker::PhantomData,data:OriginalReturnClaimState{source:origin.source_identity(),serial:plan.serial,slot,local,capture:true,receive:true,capture_observed:false,receive_observed:false,live:true}}))
 }
 pub fn redeem(&mut self,credit:&mut OriginalReturnClaimCredit<I>,witness:&mut OriginalReturnWitness<I>)->Result<(),ValueError>{
  let origin=self.origin.ok_or_else(||refused("original return source already closed"))?;if origin.recipient_identity()!=self as*const Self as usize{return Err(refused("original return redemption requires its actual bound budget address"))}if !credit.live&&!witness.redeemed||credit.source!=origin.source_identity()||credit.serial!=witness.serial||witness.source!=origin.source_identity(){return Err(refused("original return witness is foreign to its held claim"))}if witness.redeemed{return Ok(())}
  let observed=match witness.phase{OriginalReturnPhase::Capture=>credit.capture_observed,OriginalReturnPhase::Receive=>credit.receive_observed};if observed||match witness.phase{OriginalReturnPhase::Capture=>credit.capture,OriginalReturnPhase::Receive=>credit.receive}{return Err(refused("original return witness was not issued once by its claim"))}
  let identity=self.table.as_ref().and_then(|table|table.row_at(credit.slot,credit.serial)).ok_or_else(||refused("original return claim row disappeared"))?.identity;if identity.source!=origin.source_identity() as u64||identity.recipient!=origin.recipient_identity() as u64{return Err(refused("original return claim row changed Source"))}let extent=usize::try_from(identity.extent).map_err(|_|refused("original return extent exceeds native size"))?;if witness.release_capture&&(witness.phase!=OriginalReturnPhase::Receive||credit.capture_observed){return Err(refused("original return witness lost its genuine held currencies"))}let plan=self.snapshot().plan_redemption(extent,witness.phase==OriginalReturnPhase::Capture,witness.release_capture,credit.local,witness.progress,witness.work,(M::UPDATE,M::FINAL,M::LOCAL_ACK))?;let held=plan.held;let held_copy=plan.held_copy;let spent=plan.spent;let work=plan.work;let actual=plan.actual;let unused=plan.unused;self.table.as_mut().ok_or_else(||refused("original return table disappeared"))?.redeem(identity,actual,unused)?;self.held_items-=usize::from(held);self.held_copy_bytes-=held_copy;self.held_fuel-=u64::from(held);self.spent=spent;self.work=work;match witness.phase{OriginalReturnPhase::Capture=>credit.data.capture_observed=true,OriginalReturnPhase::Receive=>credit.data.receive_observed=true}if witness.release_capture{credit.data.capture_observed=true;}if credit.capture_observed&&credit.receive_observed{self.claims-=1;credit.data.live=false;}witness.data.redeemed=true;Ok(())
 }

}

/// 🎟️ Nominal origin typing keeps claimed authority distinct from freely constructible phase data.
#[repr(transparent)]
pub struct OriginalReturnClaimCredit<I:OriginalReturnOrigin>{data:OriginalReturnClaimState,_origin:std::marker::PhantomData<fn()->I>}
impl<I:OriginalReturnOrigin> std::ops::Deref for OriginalReturnClaimCredit<I>{type Target=OriginalReturnClaimState;fn deref(&self)->&Self::Target{&self.data}}
impl<I:OriginalReturnOrigin> OriginalReturnClaimCredit<I>{
 pub fn terminal_is_empty(&self)->bool{self.data.terminal_is_empty()}
 pub fn has_unused_capture(&self)->bool{self.data.has_unused_capture()}
 pub fn issue_return(&mut self,phase:OriginalReturnPhase,copy_bytes:usize)->Result<OriginalReturnWitness<I>,ValueError>{self.data.issue_return(phase,copy_bytes).map(|data|OriginalReturnWitness{data,_origin:std::marker::PhantomData})}
 pub fn cancel_preissued(&mut self,w:&mut OriginalReturnWitness<I>)->Result<(),ValueError>{self.data.cancel_preissued(&mut w.data)}
 pub fn issue_success_return(&mut self,copy_bytes:usize)->Result<OriginalReturnWitness<I>,ValueError>{self.data.issue_success_return(copy_bytes).map(|data|OriginalReturnWitness{data,_origin:std::marker::PhantomData})}
}
impl<I:OriginalReturnOrigin> Drop for OriginalReturnClaimCredit<I>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original return claim retains unspent original currencies");}}
/// 🧾️ Only a genuine typed claim can mint this held physical-effect witness.
#[repr(transparent)]
pub struct OriginalReturnWitness<I:OriginalReturnOrigin>{data:OriginalReturnWitnessState,_origin:std::marker::PhantomData<fn()->I>}
impl<I:OriginalReturnOrigin> std::ops::Deref for OriginalReturnWitness<I>{type Target=OriginalReturnWitnessState;fn deref(&self)->&Self::Target{&self.data}}
impl<I:OriginalReturnOrigin> OriginalReturnWitness<I>{
 pub fn is_capture(&self)->bool{self.phase==OriginalReturnPhase::Capture}
 pub fn releases_unused_capture(&self)->bool{self.release_capture}
 pub fn mark_received(&mut self)->Result<(),ValueError>{if !self.redeemed||self.received{return Err(refused("original held return witness has not been redeemed exactly once"))}self.data.received=true;Ok(())}
}
impl<I:OriginalReturnOrigin> Drop for OriginalReturnWitness<I>{fn drop(&mut self){assert!(std::thread::panicking()||self.received,"original held return witness requires its actual receiving caller");}}
pub fn original_return_claim_copy_bytes<I:OriginalReturnOrigin>()->usize{std::mem::size_of::<OriginalReturnEscrowRow>()+std::mem::size_of::<Option<OriginalReturnClaimCredit<I>>>()+3*std::mem::size_of::<u64>()+2*std::mem::size_of::<u8>()+2*std::mem::size_of::<usize>()}

impl<I:OriginalReturnOrigin> OriginalReturnBudget<I>{
 /// 🔎️ Qualifies the original typed preissued capture against its actual receiving budget and destination.
 pub fn qualify_preissued_destination(&self,credit:&OriginalReturnClaimCredit<I>,w:&OriginalReturnWitness<I>,destination:usize,extent:usize)->Result<(),ValueError>{let b=self;let origin=b.origin.ok_or_else(||refused("header exposure changed its actual Source, cell or permit"))?;let row=b.table.as_ref().and_then(|t|t.row_at(credit.slot,credit.serial)).ok_or_else(||refused("header exposure changed its actual Source, cell or permit"))?;if origin.recipient_identity()!=b as*const Self as usize||!credit.live||credit.source!=origin.source_identity()||w.source!=credit.source||w.serial!=credit.serial||w.phase!=OriginalReturnPhase::Capture||w.redeemed||w.received||credit.capture||!credit.receive||credit.capture_observed||credit.receive_observed||row.tombstone||row.identity.source!=origin.source_identity() as u64||row.identity.recipient!=origin.recipient_identity() as u64||row.identity.destination!=destination as u64||row.identity.extent!=extent as u64{return Err(refused("header exposure changed its actual Source, cell or permit"))}Ok(())}
}

/// 📏️ A sized return quote is data; only the Context can qualify its actual Source destination.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub struct OriginalSizedClaimReserve{pub items:usize,pub immediate_copy_bytes:usize,pub lifetime_copy_bytes:usize,pub work:u64,pub depth:usize}
impl OriginalSizedClaimReserve{pub fn for_target(extent:usize,claim:usize,identity:usize,destination:usize)->Result<Self,ValueError>{let actual=claim.checked_add(identity).and_then(|v|v.checked_add(destination)).and_then(|v|M::UPDATE.checked_mul(2).and_then(|update|v.checked_add(update))).and_then(|v|v.checked_add(M::FINAL)).ok_or_else(||refused("sized claim reserve overflow"))?;if actual>512{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"sized claim target exceeds portable metadata ceiling"))}Ok(Self{items:3,immediate_copy_bytes:512,lifetime_copy_bytes:extent.checked_add(512).ok_or_else(||refused("sized claim reserve overflow"))?,work:3,depth:1})}}
/// 🧮️ Reserved effects never become a second fresh charge.
pub fn source_fresh_progress(total:P,reserved:P)->Option<P>{Some(P{copied_items:total.copied_items.checked_sub(reserved.copied_items)?,copied_bytes:total.copied_bytes.checked_sub(reserved.copied_bytes)?,retained_capacity_bytes:total.retained_capacity_bytes.checked_sub(reserved.retained_capacity_bytes)?,released_bytes:total.released_bytes.checked_sub(reserved.released_bytes)?})}

#[path="🏠️native/🦀️.rs"]pub mod native_storage;

#[path="📦️stock/🦀️.rs"]mod stock;
pub use stock::{OriginalReturnStockClaim,OriginalReturnStockWitness,original_return_stock_claim_copy_bytes,original_return_stock_settlement_copy_bytes,original_return_stock_top_up_copy_bytes,OriginalReturnScopeLoan,original_return_scope_admission_copy_bytes,original_return_scope_completion_copy_bytes,original_return_scope_setup_demand,OriginalReturnScopeCounters,OriginalReturnScopePlan,original_return_scope_quantum,original_return_scope_transfer_copy_bytes};
