//! 🪙️ Fixed Source-owned claim cells preserve actual held, redeemed and unused currencies.
use super::{RetainedCloneGrant,RetainedCloneProgress,RetainedWorkOutcome,ValueError,ValueRefusalKind};
#[path="🛡️guard/🦀️.rs"]
mod guard;
pub use guard::{OriginalReturnGuardPermit,OriginalReturnGuardDemand,OriginalReturnGuardPlan,OriginalReturnGuardSettlementPlan};
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct OriginalReturnBudgetSnapshot{pub granted:RetainedCloneGrant,pub fuel:u64,pub spent:RetainedCloneProgress,pub work:u128,pub held_items:usize,pub held_copy_bytes:usize,pub held_release_bytes:usize,pub held_capacity_bytes:usize,pub held_fuel:u64,pub claims:usize,pub serial:u64}
impl OriginalReturnBudgetSnapshot{pub fn conserved(&self)->bool{self.spent.fits(self.granted)&&self.spent.copied_items.checked_add(self.held_items).is_some_and(|items|items<=self.granted.maximum_items)&&self.spent.copied_bytes.checked_add(self.held_copy_bytes).is_some_and(|bytes|bytes<=self.granted.maximum_copy_bytes)&&self.spent.released_bytes.checked_add(self.held_release_bytes).is_some_and(|bytes|bytes<=self.granted.maximum_release_bytes)&&self.spent.retained_capacity_bytes.checked_add(self.held_capacity_bytes).is_some_and(|bytes|bytes<=self.granted.maximum_capacity_bytes)&&self.work.checked_add(u128::from(self.held_fuel)).is_some_and(|work|work<=u128::from(self.fuel))}pub fn raw_remaining(&self)->Result<(RetainedCloneGrant,u64),ValueError>{if !self.spent.fits(self.granted)||self.work>u128::from(self.fuel){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original return source spent exceeds its original stock"))}let grant=self.granted;let spent=self.spent;Ok((RetainedCloneGrant{maximum_items:grant.maximum_items.saturating_sub(spent.copied_items),maximum_copy_bytes:grant.maximum_copy_bytes.saturating_sub(spent.copied_bytes),maximum_capacity_bytes:grant.maximum_capacity_bytes.saturating_sub(spent.retained_capacity_bytes),maximum_release_bytes:grant.maximum_release_bytes.saturating_sub(spent.released_bytes),..grant},self.fuel-u64::try_from(self.work).map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"original return source CPU extent overflow"))?))}pub fn remaining(&self)->Result<(RetainedCloneGrant,u64),ValueError>{let(mut retained,fuel)=self.raw_remaining()?;retained.maximum_items=retained.maximum_items.checked_sub(self.held_items).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original return holds exceed retained lifetime stock"))?;retained.maximum_copy_bytes=retained.maximum_copy_bytes.checked_sub(self.held_copy_bytes).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original return copy holds exceed retained lifetime stock"))?;retained.maximum_release_bytes=retained.maximum_release_bytes.checked_sub(self.held_release_bytes).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original return release holds exceed retained lifetime stock"))?;retained.maximum_capacity_bytes=retained.maximum_capacity_bytes.checked_sub(self.held_capacity_bytes).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original return capacity holds exceed retained lifetime stock"))?;Ok((retained,fuel.checked_sub(self.held_fuel).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original return holds exceed original CPU stock"))?))}}
/// 🧮️ Data-only return-ledger refusal; Source ownership remains in its issuer.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum OriginalReturnLedgerIssue{ReceiptOverflow,OwnershipExceeded,WorkOverflow,WorkExceeded}
impl OriginalReturnLedgerIssue{pub fn failure(self)->(ValueRefusalKind,&'static str){match self{Self::ReceiptOverflow=>(ValueRefusalKind::OwnershipLimit,"original return source receipt overflow"),Self::OwnershipExceeded=>(ValueRefusalKind::OwnershipLimit,"original return source receipt exceeds spent and held authority"),Self::WorkOverflow=>(ValueRefusalKind::WorkLimit,"original return source CPU counter overflow"),Self::WorkExceeded=>(ValueRefusalKind::WorkLimit,"original return source CPU exceeds spent and held authority")}}}
impl OriginalReturnBudgetSnapshot{
 pub fn advance_retained(&mut self,progress:RetainedCloneProgress)->Option<OriginalReturnLedgerIssue>{let Ok(spent)=self.spent.checked_add(progress)else{return Some(OriginalReturnLedgerIssue::ReceiptOverflow)};self.spent=spent;(!self.conserved()).then_some(OriginalReturnLedgerIssue::OwnershipExceeded)}
 pub fn advance_work(&mut self,work:u64)->Option<OriginalReturnLedgerIssue>{self.advance_work_exact(u128::from(work))}
 pub fn advance_work_exact(&mut self,work:u128)->Option<OriginalReturnLedgerIssue>{let Some(total)=self.work.checked_add(work)else{return Some(OriginalReturnLedgerIssue::WorkOverflow)};self.work=total;(!self.conserved()).then_some(OriginalReturnLedgerIssue::WorkExceeded)}
}
/// 📋️ Data-only claim arithmetic cannot issue a Source credit.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalReturnClaimPlan{pub serial:u64,pub claims:u8,pub held_items:usize,pub held_fuel:u64,pub held_copy_bytes:usize,pub reserved_copy:usize}
impl OriginalReturnBudgetSnapshot{
 pub fn plan_claim(&self,copy_bytes:usize,row_copy:usize,overhead:usize)->Result<OriginalReturnClaimPlan,ValueError>{let(retained,fuel)=self.remaining()?;let fail=|message|ValueError::literal(ValueRefusalKind::InvariantViolated,message);let reserved_copy=copy_bytes.checked_add(overhead).ok_or_else(||fail("return claim copy overflow"))?;if retained.maximum_items<3||retained.maximum_copy_bytes<reserved_copy.checked_add(row_copy).ok_or_else(||fail("return claim copy overflow"))?||fuel<3||retained.maximum_depth==0{return Err(fail("original return claim lacks its genuine currencies, epoch or destination"))}Ok(OriginalReturnClaimPlan{serial:self.serial.checked_add(1).ok_or_else(||fail("original return claim serial exhausted"))?,claims:u8::try_from(self.claims).ok().and_then(|v|v.checked_add(1)).ok_or_else(||fail("original return claim count overflow"))?,held_items:self.held_items.checked_add(2).ok_or_else(||fail("original return item hold overflow"))?,held_fuel:u64::try_from(self.held_fuel).ok().and_then(|v|v.checked_add(2)).ok_or_else(||fail("original return CPU hold overflow"))?,held_copy_bytes:self.held_copy_bytes.checked_add(reserved_copy).ok_or_else(||fail("original return copy hold overflow"))?,reserved_copy})}
}
/// 🧮️ Data-only redemption arithmetic cannot issue a Source witness.
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct OriginalReturnRedemptionPlan{pub held:u8,pub held_copy:usize,pub spent:RetainedCloneProgress,pub work:u128,pub actual:OriginalReturnEscrowAmounts,pub unused:OriginalReturnEscrowAmounts}
impl OriginalReturnBudgetSnapshot{
 pub fn plan_redemption(&self,extent:usize,capture:bool,release:bool,local:bool,progress:RetainedCloneProgress,work:u64,overhead:(usize,usize,usize))->Result<OriginalReturnRedemptionPlan,ValueError>{let fail=||ValueError::literal(ValueRefusalKind::InvariantViolated,"original return witness lost its genuine held currencies");let held=1u8+u8::from(release);let tail=if capture{extent}else{overhead.1.checked_add(if local{overhead.2}else{0}).ok_or_else(fail)?};let actual_copy=overhead.0.checked_add(tail).ok_or_else(fail)?;let released_copy=if release{extent.checked_add(overhead.0).ok_or_else(fail)?}else{0};let held_copy=actual_copy.checked_add(released_copy).ok_or_else(fail)?;if self.held_copy_bytes<held_copy||progress.copied_bytes!=actual_copy||self.held_items<usize::from(held)||self.held_fuel<u64::from(held)||self.claims==0{return Err(fail())}let spent=self.spent.checked_add(progress)?;let work_total=self.work.checked_add(u128::from(work)).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original reserved return actual CPU overflow"))?;let actual=OriginalReturnEscrowAmounts{items:1,copy_bytes:progress.copied_bytes as u64,work,release_bytes:0,capacity_bytes:0};let unused=if release{OriginalReturnEscrowAmounts{items:1,copy_bytes:released_copy as u64,work:1,release_bytes:0,capacity_bytes:0}}else{Default::default()};Ok(OriginalReturnRedemptionPlan{held,held_copy,spent,work:work_total,actual,unused})}
}
/// 📏️ Target-native sizes for actual return-escrow metadata placements.
pub struct OriginalReturnEscrowAccounting;
impl OriginalReturnEscrowAccounting{pub const UPDATE:usize=2*std::mem::size_of::<OriginalReturnEscrowAmounts>()+std::mem::size_of::<bool>()+std::mem::size_of::<u64>()+std::mem::size_of::<usize>()+std::mem::size_of::<u64>()+std::mem::size_of::<usize>();pub const FINAL:usize=std::mem::size_of::<u8>();pub const LOCAL_ACK:usize=std::mem::size_of::<u32>();pub fn witness_copy(capture:bool,local:bool,copy:usize)->Result<usize,ValueError>{Self::UPDATE.checked_add(if capture{copy}else{Self::FINAL+if local{Self::LOCAL_ACK}else{0}}).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original return witness copy overflow"))}}
/// 🧱️ Pure structural demand checks never issue Source authority.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum OriginalReturnStructure{Table,Publisher}
pub fn require_return_structure(grant:RetainedCloneGrant,fuel:u64,items:usize,copy:usize,capacity:usize,work:u64,kind:OriginalReturnStructure)->Result<(),ValueError>{let(ownership,work_error)=match kind{OriginalReturnStructure::Table=>("original escrow table requires its structural copy and capacity budget","original escrow table requires its structural work budget"),OriginalReturnStructure::Publisher=>("escrow publisher requires its remaining structural ownership budget","escrow publisher requires its remaining structural work budget")};if grant.maximum_items<items||grant.maximum_depth==0||grant.maximum_copy_bytes<copy||grant.maximum_capacity_bytes<capacity{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,ownership))}if fuel<work{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,work_error))}Ok(())}
/// 🗣️ Exact Source-ledger diagnostics are owned with the return accounting vocabulary.
pub struct OriginalReturnEscrowMessages;
impl OriginalReturnEscrowMessages{pub const STOCK_ABSENT:&'static str="original return source has not admitted its lifetime stock";pub const CLAIM_OWNERSHIP:&'static str="original return claim exceeds its remaining lifetime ownership currency";pub const PUBLISHER_ABSENT:&'static str="escrow publisher has no actual Source";pub const PUBLISHER_LOST:&'static str="escrow publisher Source disappeared";}
pub const ORIGINAL_RETURN_ESCROW_CELLS:usize=32;
pub const ORIGINAL_RETURN_ESCROW_HEADER_BYTES:usize=48;
pub const ORIGINAL_RETURN_ESCROW_ROW_BYTES:usize=184;
pub const ORIGINAL_RETURN_ESCROW_RECLAIM_COPY_BYTES:usize=std::mem::size_of::<u32>()+std::mem::size_of::<u64>();
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct OriginalReturnEscrowAmounts{pub items:u64,pub copy_bytes:u64,pub work:u64,pub release_bytes:u64,pub capacity_bytes:u64}
impl OriginalReturnEscrowAmounts{
 const fn zero()->Self{Self{items:0,copy_bytes:0,work:0,release_bytes:0,capacity_bytes:0}}
 fn add(self,other:Self)->Option<Self>{Some(Self{items:self.items.checked_add(other.items)?,copy_bytes:self.copy_bytes.checked_add(other.copy_bytes)?,work:self.work.checked_add(other.work)?,release_bytes:self.release_bytes.checked_add(other.release_bytes)?,capacity_bytes:self.capacity_bytes.checked_add(other.capacity_bytes)?})}
 fn fits(self,limit:Self)->bool{self.items<=limit.items&&self.copy_bytes<=limit.copy_bytes&&self.work<=limit.work&&self.release_bytes<=limit.release_bytes&&self.capacity_bytes<=limit.capacity_bytes}
}
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct OriginalReturnEscrowIdentity{pub source:u64,pub source_birth:u64,pub recipient:u64,pub destination:u64,pub extent:u64,pub serial:u64,pub origin_epoch:u64}
impl OriginalReturnEscrowIdentity{const fn zero()->Self{Self{source:0,source_birth:0,recipient:0,destination:0,extent:0,serial:0,origin_epoch:0}}}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalReturnEscrowRow{pub identity:OriginalReturnEscrowIdentity,pub held:OriginalReturnEscrowAmounts,pub redeemed:OriginalReturnEscrowAmounts,pub released:OriginalReturnEscrowAmounts,pub tombstone:bool}
impl OriginalReturnEscrowRow{const fn zero()->Self{Self{identity:OriginalReturnEscrowIdentity::zero(),held:OriginalReturnEscrowAmounts::zero(),redeemed:OriginalReturnEscrowAmounts::zero(),released:OriginalReturnEscrowAmounts::zero(),tombstone:false}}}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum OriginalReturnEscrowStage{Blocked,Progress{copied_bytes:usize,issued_serial:Option<u64>},Ready{slot:usize}}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original return escrow changed its actual claim or exceeded its held currencies")}
/// 🗃️ Every Source cell is preborn and a staged cell remains invisible until its actual claim commits.
pub struct OriginalReturnEscrowTable{rows:[OriginalReturnEscrowRow;ORIGINAL_RETURN_ESCROW_CELLS],states:[u8;ORIGINAL_RETURN_ESCROW_CELLS],stage:Option<(usize,u8)>,revision:u64,borrowed:Option<(u64,u32)>,acknowledged:u32,guard_issued:u32}
/// 🌱️ Owns the original uninitialized table backing until each funded field and the final output are published.
pub struct OriginalReturnEscrowBirth{
 backing:Option<std::ptr::NonNull<OriginalReturnEscrowTable>>,
 binding:Option<semio_framework_value_control_contract::OriginalWorkBindingIdentity>,
 initialized:usize,
 header:bool,
}
/// 🧵️ The resident backing is exclusively owned and its elements contain no borrowed state.
unsafe impl Send for OriginalReturnEscrowBirth{}
impl Default for OriginalReturnEscrowBirth{
 fn default()->Self{Self::new()}
}
impl OriginalReturnEscrowBirth{
 pub const fn new()->Self{Self{backing:None,binding:None,initialized:0,header:false}}
 /// 📏️ Quotes only the actual next allocation, row, header or output placement.
 pub fn demand(&self)->semio_framework_value_retained_clone::RetirementDemand{
  use semio_framework_value_retained_clone::RetirementDemand as D;
  if self.backing.is_none(){D{copy_bytes:std::mem::size_of::<Option<std::ptr::NonNull<OriginalReturnEscrowTable>>>()+std::mem::size_of::<Option<semio_framework_value_control_contract::OriginalWorkBindingIdentity>>(),capacity_bytes:std::mem::size_of::<OriginalReturnEscrowTable>(),depth:1,..Default::default()}}
  else if self.initialized<ORIGINAL_RETURN_ESCROW_CELLS{D{copy_bytes:std::mem::size_of::<OriginalReturnEscrowRow>()+std::mem::size_of::<u8>()+std::mem::size_of::<usize>(),depth:1,..Default::default()}}
  else if !self.header{D{copy_bytes:std::mem::size_of::<Option<(usize,u8)>>()+std::mem::size_of::<u64>()+std::mem::size_of::<Option<(u64,u32)>>()  +2*std::mem::size_of::<u32>()+std::mem::size_of::<bool>(),depth:1,..Default::default()}}
  else{D{copy_bytes:std::mem::size_of::<Option<Box<OriginalReturnEscrowTable>>>()+std::mem::size_of::<Option<std::ptr::NonNull<OriginalReturnEscrowTable>>>()+std::mem::size_of::<Option<semio_framework_value_control_contract::OriginalWorkBindingIdentity>>()+std::mem::size_of::<usize>()+std::mem::size_of::<bool>(),depth:1,..Default::default()}}
 }
 /// 👁️ Reveals only whether the original backing remains in this birth.
 pub fn is_empty(&self)->bool{self.backing.is_none()&&self.binding.is_none()}
 /// 📤️ Reports only whether every field is initialized before the separate output placement.
 pub fn ready_to_publish(&self)->bool{self.backing.is_some()&&self.initialized==ORIGINAL_RETURN_ESCROW_CELLS&&self.header}
 /// 📥️ Records the actual paid native step directly in the caller's original reply.
 pub fn advance_into<'a>(&mut self,output:&mut Option<Box<OriginalReturnEscrowTable>>,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'a,semio_framework_value_retained_clone::RetainedCloneStep,ValueError>)->semio_framework_value_control_contract::OriginalEffectReplyComplete<'a,semio_framework_value_retained_clone::RetainedCloneStep,ValueError>{
  use semio_framework_value_retained_clone::{RetainedCloneStep as S,RetainedCloneProgress as P};
  let result=(||{
   if effect.work_source_turn().is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"escrow birth requires its original Source"))}
   if self.binding.as_ref().is_some_and(|binding|!binding.matches_effect(effect)){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"escrow birth changed its original Work source"))}
   let demand=self.demand();let grant=effect.grant();
   if output.is_some()||grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_depth<demand.depth||effect.remaining_work_units()?<1{return Ok(S::Progress(P::default()))}
   if self.backing.is_none()&&effect.actual().is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"escrow birth requires an unused allocation marker"))}
   if !effect.performed(1)?{return Ok(S::Progress(P::default()))}
   if self.backing.is_none(){
    *effect.actual()=Some(0);
    let layout=std::alloc::Layout::new::<OriginalReturnEscrowTable>();
    let pointer=unsafe{std::alloc::alloc(layout)}.cast::<OriginalReturnEscrowTable>();
    let Some(backing)=std::ptr::NonNull::new(pointer)else{return Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"original return escrow allocation failed"))};
    self.backing=Some(backing);self.binding=Some(semio_framework_value_control_contract::OriginalWorkBindingIdentity::from_effect(effect));*effect.actual()=Some(layout.size());
    return Ok(S::Progress(P{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:layout.size(),..Default::default()}))
   }
   let target=self.backing.unwrap().as_ptr();
   if self.initialized<ORIGINAL_RETURN_ESCROW_CELLS{
    unsafe{std::ptr::addr_of_mut!((*target).rows).cast::<OriginalReturnEscrowRow>().add(self.initialized).write(OriginalReturnEscrowRow::zero());std::ptr::addr_of_mut!((*target).states).cast::<u8>().add(self.initialized).write(0)}
    self.initialized+=1;
   }else if !self.header{
    unsafe{std::ptr::addr_of_mut!((*target).stage).write(None);std::ptr::addr_of_mut!((*target).revision).write(0);std::ptr::addr_of_mut!((*target).borrowed).write(None);std::ptr::addr_of_mut!((*target).acknowledged).write(0);std::ptr::addr_of_mut!((*target).guard_issued).write(0)}
    self.header=true;
   }else{
    *output=Some(unsafe{Box::from_raw(target)});self.backing=None;self.binding=None;self.initialized=0;self.header=false;
    return Ok(S::Complete(P{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))
   }
   Ok(S::Progress(P{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))
  })();
  reply.complete(result)
 }
 /// 📏️ Quotes the actual resident metadata and backing for partial-birth cancellation.
 pub fn cancel_demand(&self)->semio_framework_value_retained_clone::RetirementDemand{
  if self.backing.is_none(){return Default::default()}
  semio_framework_value_retained_clone::RetirementDemand{copy_bytes:std::mem::size_of::<Option<std::ptr::NonNull<OriginalReturnEscrowTable>>>()+std::mem::size_of::<Option<semio_framework_value_control_contract::OriginalWorkBindingIdentity>>()+std::mem::size_of::<usize>()+std::mem::size_of::<bool>(),release_bytes:std::mem::size_of::<OriginalReturnEscrowTable>(),depth:1,..Default::default()}
 }
 /// ♻️ Releases the exact partially initialized allocation only after its original payer funds release.
 pub fn cancel_into<'a>(&mut self,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'a,semio_framework_value_retained_clone::RetainedCloneStep,ValueError>)->semio_framework_value_control_contract::OriginalEffectReplyComplete<'a,semio_framework_value_retained_clone::RetainedCloneStep,ValueError>{
  use semio_framework_value_retained_clone::{RetainedCloneStep as S,RetainedCloneProgress as P};
  let result=(||{
   let Some(backing)=self.backing else{return Ok(S::Complete(P::default()))};
   if self.binding.as_ref().is_none_or(|binding|!binding.matches_effect(effect)){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"escrow cancellation changed its original Work source"))}
   let extent=std::mem::size_of::<OriginalReturnEscrowTable>();let copy=std::mem::size_of::<Option<std::ptr::NonNull<OriginalReturnEscrowTable>>>()+std::mem::size_of::<Option<semio_framework_value_control_contract::OriginalWorkBindingIdentity>>()+std::mem::size_of::<usize>()+std::mem::size_of::<bool>();let grant=effect.grant();
   if grant.maximum_items==0||grant.maximum_copy_bytes<copy||grant.maximum_release_bytes<extent||grant.maximum_depth==0||effect.remaining_work_units()?<1||!effect.performed(1)?{return Ok(S::Progress(P::default()))}
   unsafe{std::alloc::dealloc(backing.as_ptr().cast(),std::alloc::Layout::new::<OriginalReturnEscrowTable>())};
   self.backing=None;self.binding=None;self.initialized=0;self.header=false;
   Ok(S::Complete(P{copied_items:1,copied_bytes:copy,released_bytes:extent,..Default::default()}))
  })();reply.complete(result)
 }
}
impl Drop for OriginalReturnEscrowBirth{fn drop(&mut self){assert!(std::thread::panicking()||self.is_empty(),"original escrow birth requires paid completion or cancellation")}}

impl OriginalReturnEscrowTable{
 pub const fn empty()->Self{Self{rows:[OriginalReturnEscrowRow::zero();ORIGINAL_RETURN_ESCROW_CELLS],states:[0;ORIGINAL_RETURN_ESCROW_CELLS],stage:None,revision:0,borrowed:None,acknowledged:0,guard_issued:0}}
 pub fn birth_demand()->(usize,usize,u64){let pointer=std::mem::size_of::<Option<std::ptr::NonNull<Self>>>();let binding=std::mem::size_of::<Option<semio_framework_value_control_contract::OriginalWorkBindingIdentity>>();let row=std::mem::size_of::<OriginalReturnEscrowRow>()+std::mem::size_of::<u8>()+std::mem::size_of::<usize>();let header=std::mem::size_of::<Option<(usize,u8)>>()+std::mem::size_of::<u64>()+std::mem::size_of::<Option<(u64,u32)>>() +2*std::mem::size_of::<u32>()+std::mem::size_of::<bool>();let output=std::mem::size_of::<Option<Box<Self>>>()+pointer+binding+std::mem::size_of::<usize>()+std::mem::size_of::<bool>();(pointer+binding+ORIGINAL_RETURN_ESCROW_CELLS*row+header+output,std::mem::size_of::<Self>(),ORIGINAL_RETURN_ESCROW_CELLS as u64+3)}
 pub fn admit(grant:RetainedCloneGrant,fuel:u64)->RetainedWorkOutcome<(Option<Box<Self>>,RetainedCloneProgress)>{let(copy,capacity,work)=Self::birth_demand();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copy||grant.maximum_capacity_bytes<capacity||fuel<work{return RetainedWorkOutcome{result:Ok((None,Default::default())),work:0}}let mut owner=Box::<Self>::new_uninit();unsafe{owner.as_mut_ptr().write(Self::empty());let owner=owner.assume_init();RetainedWorkOutcome{result:Ok((Some(owner),RetainedCloneProgress{copied_items:1,copied_bytes:copy,retained_capacity_bytes:capacity,..Default::default()})),work:u128::from(work)}}}
 pub fn revision(&self)->u64{self.revision}
 pub fn count(&self)->usize{self.states.iter().filter(|state|**state>=2).count()}
 pub fn has_capacity(&self)->bool{self.states.iter().any(|state|*state==0)}
 pub fn is_empty(&self)->bool{self.states.iter().all(|state|*state==0)&&self.stage.is_none()&&self.borrowed.is_none()&&self.acknowledged==0&&self.guard_issued==0}
 pub fn row_after(&self,serial:u64)->Option<&OriginalReturnEscrowRow>{self.rows.iter().zip(self.states).filter(|(row,state)|*state>=2&&row.identity.serial>serial).map(|(row,_)|row).min_by_key(|row|row.identity.serial)}
 pub fn slot_for(&self,serial:u64)->Option<usize>{self.rows.iter().zip(self.states).position(|(row,state)|state>=2&&row.identity.serial==serial)}
 pub fn row_at(&self,slot:usize,serial:u64)->Option<&OriginalReturnEscrowRow>{self.states.get(slot).filter(|state|**state>=2).and_then(|_|self.rows.get(slot)).filter(|row|row.identity.serial==serial)}
 pub fn committed_stage_pending(&self)->bool{self.stage.is_some_and(|(slot,_)|self.states[slot]>=2)}
 pub fn staged_serial(&self,destination:u64,extent:u64)->Option<u64>{self.stage.and_then(|(slot,phase)|(self.states[slot]==1&&phase>3&&self.rows[slot].identity.destination==destination&&self.rows[slot].identity.extent==extent).then_some(self.rows[slot].identity.serial))}
 pub fn stage_remaining(&self,destination:u64,extent:u64)->Option<(usize,usize)>{let step=std::mem::size_of::<u64>()+std::mem::size_of::<Option<(usize,u8)>>();match self.stage{None if self.has_capacity()=>Some((20,2*std::mem::size_of::<u64>()+std::mem::size_of::<Option<(usize,u8)>>() +1+19*step+std::mem::size_of::<u64>())),Some((slot,phase))if self.states[slot]==1&&self.rows[slot].identity.destination==destination&&self.rows[slot].identity.extent==extent=>Some((usize::from(19-phase),usize::from(19-phase)*step+if phase<=3{std::mem::size_of::<u64>()}else{0})),_=>None}}
 pub fn staged_for(&self,destination:u64,extent:u64)->bool{self.stage.is_some_and(|(slot,_)|self.states[slot]==1&&self.rows[slot].identity.destination==destination&&self.rows[slot].identity.extent==extent)}
 pub fn stage_step(&mut self,identity:&OriginalReturnEscrowIdentity,held:OriginalReturnEscrowAmounts,copy:usize)->Result<OriginalReturnEscrowStage,ValueError>{
  if identity.source==0||identity.source_birth==0||identity.recipient==0||identity.destination==0||identity.serial==0{return Err(invalid())}
  let(slot,phase)=match self.stage{None=>{let Some(slot)=self.states.iter().position(|state|*state==0)else{return Ok(OriginalReturnEscrowStage::Blocked)};let bytes=2*std::mem::size_of::<u64>()+std::mem::size_of::<Option<(usize,u8)>>() +1;if copy<bytes{return Ok(OriginalReturnEscrowStage::Blocked)}self.rows[slot].identity.destination=identity.destination;self.rows[slot].identity.extent=identity.extent;self.states[slot]=1;self.stage=Some((slot,0));return Ok(OriginalReturnEscrowStage::Progress{copied_bytes:bytes,issued_serial:None})},Some((slot,phase))=>{if self.states[slot]!=1{return Ok(OriginalReturnEscrowStage::Blocked)}let row=&self.rows[slot];if row.identity.destination!=identity.destination||row.identity.extent!=identity.extent{return Ok(OriginalReturnEscrowStage::Blocked)}(slot,phase)}};
  let row=&self.rows[slot];if phase>0&&row.identity.source!=identity.source||phase>1&&row.identity.source_birth!=identity.source_birth||phase>2&&row.identity.recipient!=identity.recipient||phase>3&&row.identity.serial!=identity.serial||phase>4&&row.held.items!=held.items||phase>5&&row.held.copy_bytes!=held.copy_bytes||phase>6&&row.held.work!=held.work||phase>13&&row.held.release_bytes!=held.release_bytes||phase>16&&row.held.capacity_bytes!=held.capacity_bytes{return Err(invalid())}if phase==19{return Ok(OriginalReturnEscrowStage::Ready{slot})}let bytes=std::mem::size_of::<u64>()+std::mem::size_of::<Option<(usize,u8)>>();if copy<bytes+if phase==3{std::mem::size_of::<u64>()}else{0}{return Ok(OriginalReturnEscrowStage::Blocked)}let row=&mut self.rows[slot];match phase{0=>row.identity.source=identity.source,1=>row.identity.source_birth=identity.source_birth,2=>row.identity.recipient=identity.recipient,3=>row.identity.serial=identity.serial,4=>row.held.items=held.items,5=>row.held.copy_bytes=held.copy_bytes,6=>row.held.work=held.work,7=>row.redeemed.items=0,8=>row.redeemed.copy_bytes=0,9=>row.redeemed.work=0,10=>row.released.items=0,11=>row.released.copy_bytes=0,12=>row.released.work=0,13=>row.held.release_bytes=held.release_bytes,14=>row.redeemed.release_bytes=0,15=>row.released.release_bytes=0,16=>row.held.capacity_bytes=held.capacity_bytes,17=>row.redeemed.capacity_bytes=0,18=>row.released.capacity_bytes=0,_=>return Err(invalid())}self.stage=Some((slot,phase+1));Ok(OriginalReturnEscrowStage::Progress{copied_bytes:bytes,issued_serial:(phase==3).then_some(identity.serial)})
 }
 pub fn commit_stage(&mut self,slot:usize,identity:OriginalReturnEscrowIdentity,held:OriginalReturnEscrowAmounts)->Result<(),ValueError>{if self.stage!=Some((slot,19))||self.states.get(slot)!=Some(&1){return Err(invalid())}let row=&self.rows[slot];if row.identity.source!=identity.source||row.identity.source_birth!=identity.source_birth||row.identity.recipient!=identity.recipient||row.identity.destination!=identity.destination||row.identity.extent!=identity.extent||row.identity.serial!=identity.serial||row.held!=held||row.redeemed!=OriginalReturnEscrowAmounts::zero()||row.released!=OriginalReturnEscrowAmounts::zero(){return Err(invalid())}let revision=self.revision.checked_add(1).ok_or_else(invalid)?;let row=&mut self.rows[slot];row.identity.origin_epoch=identity.origin_epoch;row.tombstone=false;self.states[slot]=2;self.revision=revision;Ok(())}
 pub fn finish_committed_stage(&mut self)->bool{let Some((slot,_))=self.stage else{return false};if self.states[slot]<2{return false}self.stage=None;true}
 pub fn cancel_stage(&mut self,destination:u64,extent:u64)->Result<bool,ValueError>{let Some((slot,_))=self.stage else{return Ok(true)};if self.rows[slot].identity.destination!=destination||self.rows[slot].identity.extent!=extent||self.states[slot]!=1{return Err(invalid())}self.states[slot]=0;self.stage=None;Ok(true)}
 pub fn reserve(&mut self,identity:OriginalReturnEscrowIdentity,held:OriginalReturnEscrowAmounts)->Result<Option<usize>,ValueError>{if self.stage.is_some()||identity.source==0||identity.source_birth==0||identity.recipient==0||identity.destination==0||identity.serial==0||self.row_after(identity.serial.saturating_sub(1)).is_some_and(|row|row.identity.serial==identity.serial){return Err(invalid())}let Some(slot)=self.states.iter().position(|state|*state==0)else{return Ok(None)};let revision=self.revision.checked_add(1).ok_or_else(invalid)?;self.rows[slot]=OriginalReturnEscrowRow{identity,held,redeemed:Default::default(),released:Default::default(),tombstone:false};self.states[slot]=2;self.revision=revision;Ok(Some(slot))}
 /// 📈️ Extends only the exact live row before any snapshot or staged publication borrows it.
 pub fn top_up(&mut self,identity:OriginalReturnEscrowIdentity,additional:OriginalReturnEscrowAmounts)->Result<(),ValueError>{if self.stage.is_some()||self.borrowed.is_some()||additional==Default::default(){return Err(invalid())}let slot=self.rows.iter().zip(self.states).position(|(row,state)|state==2&&!row.tombstone&&row.identity==identity).ok_or_else(invalid)?;let held=self.rows[slot].held.add(additional).ok_or_else(invalid)?;let revision=self.revision.checked_add(1).ok_or_else(invalid)?;self.rows[slot].held=held;self.revision=revision;Ok(())}
 /// 📍️ Moves a live row's registered destination with its credit, so settlement qualifies at the carrier that now owns it.
 pub fn rebind(&mut self,identity:OriginalReturnEscrowIdentity,destination:u64,extent:u64)->Result<OriginalReturnEscrowIdentity,ValueError>{if self.stage.is_some()||self.borrowed.is_some(){return Err(invalid())}let slot=self.rows.iter().zip(self.states).position(|(row,state)|state==2&&!row.tombstone&&row.identity==identity).ok_or_else(invalid)?;let revision=self.revision.checked_add(1).ok_or_else(invalid)?;self.rows[slot].identity.destination=destination;self.rows[slot].identity.extent=extent;self.revision=revision;Ok(self.rows[slot].identity)}
 pub fn redeem(&mut self,identity:OriginalReturnEscrowIdentity,redeemed:OriginalReturnEscrowAmounts,released:OriginalReturnEscrowAmounts)->Result<(),ValueError>{let slot=self.rows.iter().zip(self.states).position(|(row,state)|state==2&&row.identity==identity).ok_or_else(invalid)?;let row=&self.rows[slot];let actual=row.redeemed.add(redeemed).ok_or_else(invalid)?;let unused=row.released.add(released).ok_or_else(invalid)?;let total=actual.add(unused).ok_or_else(invalid)?;if !total.fits(row.held){return Err(invalid())}let revision=self.revision.checked_add(1).ok_or_else(invalid)?;let row=&mut self.rows[slot];row.redeemed=actual;row.released=unused;row.tombstone=total==row.held;if row.tombstone{self.states[slot]=3}self.revision=revision;Ok(())}
 pub fn borrow_snapshot(&mut self,revision:u64)->Result<(),ValueError>{if self.borrowed.is_some()||revision!=self.revision{return Err(invalid())}let mut tombstones=0u32;for(index,state)in self.states.iter().enumerate(){if *state==3{tombstones|=1u32<<index;}}self.borrowed=Some((revision,tombstones));Ok(())}
 pub fn hand_back_snapshot(&mut self,revision:u64)->Result<(),ValueError>{let Some((borrowed,tombstones))=self.borrowed else{return Err(invalid())};if borrowed!=revision{return Err(invalid())}self.acknowledged|=tombstones;self.borrowed=None;Ok(())}
 pub fn abandon_unexposed_snapshot(&mut self,revision:u64)->Result<(),ValueError>{if self.borrowed.is_none_or(|(borrowed,_)|borrowed!=revision){return Err(invalid())}self.borrowed=None;Ok(())}
 pub fn mark_local_received(&mut self,identity:OriginalReturnEscrowIdentity)->Result<(),ValueError>{let index=self.rows.iter().zip(self.states).position(|(row,state)|state==3&&row.identity==identity).ok_or_else(invalid)?;self.acknowledged|=1u32<<index;Ok(())}
 /// 🧹️ Only acknowledged settled rows remain; close reclaims them without any live claim.
 pub fn is_reclaim_only(&self)->bool{self.stage.is_none()&&self.borrowed.is_none()&&self.guard_issued==0&&self.states.iter().enumerate().all(|(index,state)|*state==0||self.acknowledged&(1u32<<index)!=0&&*state==3)}
 pub fn has_reclaimable(&self)->bool{self.states.iter().enumerate().any(|(index,state)|self.acknowledged&(1u32<<index)!=0&&self.guard_issued&(1u32<<index)==0&&*state==3&&self.stage.is_none_or(|(slot,_)|slot!=index)&&self.borrowed.is_none_or(|(_,mask)|mask&(1u32<<index)==0))}
 pub fn reclaim_one(&mut self)->Result<bool,ValueError>{let Some(index)=self.states.iter().enumerate().position(|(index,state)|self.acknowledged&(1u32<<index)!=0&&self.guard_issued&(1u32<<index)==0&&*state==3&&self.stage.is_none_or(|(slot,_)|slot!=index)&&self.borrowed.is_none_or(|(_,mask)|mask&(1u32<<index)==0))else{return Ok(false)};let revision=self.revision.checked_add(1).ok_or_else(invalid)?;self.states[index]=0;self.acknowledged&=!(1u32<<index);self.revision=revision;Ok(true)}
}

#[cfg(test)]
mod tests{
 use super::*;
 #[test]
 fn exact_original_work_keeps_full_extent_after_overrun(){let cases:serde_json::Value=serde_json::from_str(include_str!("../../../🛬️receiving/🎟️native-turn/🧪️tests/work-exact.json")).unwrap();for row in cases.as_array().unwrap(){let n=|key:&str|u128::from(row[key].as_u64().unwrap());let mut state=OriginalReturnBudgetSnapshot{granted:RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1},fuel:row["fuel"].as_u64().unwrap(),spent:Default::default(),work:n("priorHi")*1_000_000_000+n("priorLo"),held_items:0,held_copy_bytes:0,held_release_bytes:0,held_capacity_bytes:0,held_fuel:0,claims:0,serial:0};let delta=n("deltaHi")*1_000_000_000+n("deltaLo");let expected=n("nextHi")*1_000_000_000+n("nextLo");let issue=state.advance_work_exact(delta);assert_eq!(state.work,expected);assert_eq!(issue.is_some(),expected>u128::from(state.fuel))}}
 fn identity(serial:u64)->OriginalReturnEscrowIdentity{OriginalReturnEscrowIdentity{source:1,source_birth:2,recipient:3,destination:4,extent:48,serial,origin_epoch:7}}
 #[test]
 fn data_only_ledger_plans_preserve_held_stock_and_real_progress(){
  let grant=RetainedCloneGrant{maximum_items:8,maximum_copy_bytes:1000,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:2};let mut s=OriginalReturnBudgetSnapshot{granted:grant,fuel:20,spent:Default::default(),work:0,held_items:0,held_copy_bytes:0,held_release_bytes:0,held_capacity_bytes:0,held_fuel:0,claims:0,serial:4};let claim=s.plan_claim(48,10,20).unwrap();assert_eq!((claim.serial,claim.claims,claim.held_items,claim.held_fuel,claim.reserved_copy),(5,1,2,2,68));s.claims=1;s.held_items=2;s.held_fuel=2;s.held_copy_bytes=100;let p=RetainedCloneProgress{copied_items:1,copied_bytes:58,..Default::default()};let r=s.plan_redemption(48,true,false,true,p,1,(10,1,4)).unwrap();assert_eq!((r.held,r.held_copy,r.actual.items,r.actual.copy_bytes,r.unused.copy_bytes),(1,58,1,58,0));assert!(s.conserved());s.held_copy_bytes=57;assert!(s.plan_redemption(48,true,false,true,p,1,(10,1,4)).is_err());
 }
 #[test]
 fn staged_cell_stays_invisible_until_the_actual_committing_epoch(){
  let mut table=OriginalReturnEscrowTable::empty();let held=OriginalReturnEscrowAmounts{items:2,copy_bytes:48,work:2,release_bytes:0,capacity_bytes:0};let mut original=identity(1);let mut turns=0;let slot=loop{let step=table.stage_step(&original,held,64).unwrap();assert_eq!(table.count(),0);turns+=1;match step{OriginalReturnEscrowStage::Blocked=>panic!("small metadata grant must progress"),OriginalReturnEscrowStage::Progress{copied_bytes,..}=>assert!(copied_bytes<=64),OriginalReturnEscrowStage::Ready{slot}=>break slot}assert!(turns<32)};let mut foreign=original;foreign.destination+=1;assert_eq!(table.stage_step(&foreign,held,64).unwrap(),OriginalReturnEscrowStage::Blocked);original.origin_epoch=9;table.commit_stage(slot,original,held).unwrap();assert_eq!(table.row_after(0).unwrap().identity,original);assert!(table.finish_committed_stage());
 }
 #[test]
 fn staged_cells_fill_the_real_capacity_and_reuse_only_after_exact_handback(){
  let mut table=OriginalReturnEscrowTable::empty();let held=OriginalReturnEscrowAmounts{items:2,copy_bytes:48,work:2,release_bytes:0,capacity_bytes:0};for serial in 1..=32{let value=identity(serial);let slot=loop{match table.stage_step(&value,held,64).unwrap(){OriginalReturnEscrowStage::Progress{copied_bytes,..}=>assert!(copied_bytes<=64),OriginalReturnEscrowStage::Ready{slot}=>break slot,OriginalReturnEscrowStage::Blocked=>panic!("free cell must progress")}};table.commit_stage(slot,value,held).unwrap();assert!(table.finish_committed_stage())}assert_eq!(table.count(),32);assert_eq!(table.stage_step(&identity(33),held,64).unwrap(),OriginalReturnEscrowStage::Blocked);table.redeem(identity(1),held,Default::default()).unwrap();let revision=table.revision();table.borrow_snapshot(revision).unwrap();assert!(!table.reclaim_one().unwrap());table.hand_back_snapshot(revision).unwrap();assert!(table.reclaim_one().unwrap());assert!(matches!(table.stage_step(&identity(33),held,64).unwrap(),OriginalReturnEscrowStage::Progress{..}));
 }
 #[test]
 fn borrowed_snapshot_fences_a_previously_acknowledged_tombstone(){let mut table=OriginalReturnEscrowTable::empty();let value=identity(1);let held=OriginalReturnEscrowAmounts{items:2,copy_bytes:48,work:2,release_bytes:0,capacity_bytes:0};let slot=loop{match table.stage_step(&value,held,64).unwrap(){OriginalReturnEscrowStage::Progress{..}=>{},OriginalReturnEscrowStage::Ready{slot}=>break slot,OriginalReturnEscrowStage::Blocked=>panic!("staging refused")}};table.commit_stage(slot,value,held).unwrap();assert!(table.finish_committed_stage());table.redeem(value,held,Default::default()).unwrap();let revision=table.revision();table.borrow_snapshot(revision).unwrap();table.hand_back_snapshot(revision).unwrap();assert!(table.has_reclaimable());table.borrow_snapshot(revision).unwrap();assert!(!table.has_reclaimable());assert!(!table.reclaim_one().unwrap());table.hand_back_snapshot(revision).unwrap();assert!(table.reclaim_one().unwrap());}
 #[test]
 fn source_table_birth_quotes_real_capacity_and_pointer_copy_separately(){
  use semio_framework_value_testing_effect_bank::{EffectBank,Wallet,RetainedWorkPoll,OriginalEffectReply};
  let capacity=std::mem::size_of::<OriginalReturnEscrowTable>();
  let stock=RetainedCloneGrant{maximum_items:80,maximum_copy_bytes:65536,maximum_capacity_bytes:capacity,maximum_release_bytes:capacity,maximum_depth:4};
  let mut bank=EffectBank::new(Wallet::new(stock,80));let mut birth=OriginalReturnEscrowBirth::default();let mut output=None;
  for _ in 0..35{
   let demand=birth.demand();assert!(demand.copy_bytes<=4096);
   let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:0,maximum_depth:demand.depth};
   let ((turn,marker),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||bank.run(grant,1,|effect|{
    let mut reply=None;let complete=birth.advance_into(&mut output,effect,OriginalEffectReply::borrow(&mut reply).unwrap());assert!(complete.is_recorded());let step=reply.unwrap();RetainedWorkPoll::Complete(RetainedWorkOutcome{result:step,work:effect.performed_work()})
   }));
   let RetainedWorkPoll::Complete(turn)=turn else{panic!("native birth must finish its selected step")};let progress=turn.result.unwrap().progress();assert_eq!(turn.work,1);assert_eq!(marker,if progress.retained_capacity_bytes==0{None}else{Some(capacity)});assert_eq!(heap.requested_bytes,progress.retained_capacity_bytes);assert_eq!(heap.released_bytes,0);assert_eq!(bank.take_admitted(),Some(true));bank.post(progress,grant,(heap.requested_bytes,heap.released_bytes));
  }
  assert!(birth.is_empty());assert!(output.as_ref().unwrap().is_empty());
  let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:std::mem::size_of::<Option<Box<OriginalReturnEscrowTable>>>(),maximum_capacity_bytes:0,maximum_release_bytes:capacity,maximum_depth:1};
  let ((turn,_),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||bank.run(grant,1,|effect|{assert!(effect.performed(1).unwrap());drop(output.take());RetainedWorkPoll::Complete(RetainedWorkOutcome::<_,ValueError>{result:Ok(RetainedCloneProgress{copied_items:1,copied_bytes:grant.maximum_copy_bytes,released_bytes:capacity,..Default::default()}),work:1})}));
  let RetainedWorkPoll::Complete(turn)=turn else{panic!("paid table release must complete")};let progress=turn.result.unwrap();assert_eq!((heap.requested_bytes,heap.released_bytes),(0,capacity));assert_eq!(bank.take_admitted(),Some(true));bank.post(progress,grant,(0,capacity));assert!(bank.is_idle());
 }

 #[test]
 fn full_table_preserves_tombstones_until_their_exact_borrowed_snapshot_returns(){
  let held=OriginalReturnEscrowAmounts{items:2,copy_bytes:48,work:2,release_bytes:0,capacity_bytes:0};let mut table=OriginalReturnEscrowTable::empty();for serial in 1..=32{assert!(table.reserve(identity(serial),held).unwrap().is_some())}assert!(table.reserve(identity(33),held).unwrap().is_none());
  table.redeem(identity(1),held,Default::default()).unwrap();let old=table.revision();table.borrow_snapshot(old).unwrap();table.redeem(identity(2),held,Default::default()).unwrap();assert!(table.hand_back_snapshot(old+1).is_err());assert!(table.reserve(identity(33),held).unwrap().is_none());table.hand_back_snapshot(old).unwrap();assert!(table.reclaim_one().unwrap());assert!(!table.reclaim_one().unwrap());assert!(table.reserve(identity(33),held).unwrap().is_some());assert!(table.row_after(1).unwrap().tombstone);assert_eq!(table.count(),32);
 }
}

/// 🧮️ Pure retained-stock arithmetic does not issue Source authority.
pub fn original_retained_delta(after:RetainedCloneProgress,before:RetainedCloneProgress)->RetainedCloneProgress{RetainedCloneProgress{copied_items:after.copied_items.saturating_sub(before.copied_items),copied_bytes:after.copied_bytes.saturating_sub(before.copied_bytes),retained_capacity_bytes:after.retained_capacity_bytes.saturating_sub(before.retained_capacity_bytes),released_bytes:after.released_bytes.saturating_sub(before.released_bytes)}}
pub fn original_retained_remaining(grant:RetainedCloneGrant,spent:RetainedCloneProgress)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:grant.maximum_items.saturating_sub(spent.copied_items),maximum_copy_bytes:grant.maximum_copy_bytes.saturating_sub(spent.copied_bytes),maximum_capacity_bytes:grant.maximum_capacity_bytes.saturating_sub(spent.retained_capacity_bytes),maximum_release_bytes:grant.maximum_release_bytes.saturating_sub(spent.released_bytes),..grant}}
pub fn original_retained_min(a:RetainedCloneGrant,b:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:a.maximum_items.min(b.maximum_items),maximum_copy_bytes:a.maximum_copy_bytes.min(b.maximum_copy_bytes),maximum_capacity_bytes:a.maximum_capacity_bytes.min(b.maximum_capacity_bytes),maximum_release_bytes:a.maximum_release_bytes.min(b.maximum_release_bytes),maximum_depth:a.maximum_depth.min(b.maximum_depth)}}
pub fn original_retained_is_subset(a:RetainedCloneGrant,b:RetainedCloneGrant)->bool{original_retained_min(a,b)==a}


#[cfg(test)]
mod original_release_holds{
 use super::*;
 #[test]
 fn actual_snapshot_withholds_release_stock_before_allocation(){
  let law:serde_json::Value=serde_json::from_str(include_str!("🧪️tests/♻️release/📐️.json")).unwrap();
  for c in law["cases"].as_array().unwrap(){
   let stock=c["stock"].as_u64().unwrap()as usize;let spent=c["spent"].as_u64().unwrap()as usize;let held=c["held"].as_u64().unwrap()as usize;
   let s=OriginalReturnBudgetSnapshot{granted:RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:stock,maximum_depth:1},fuel:1,spent:RetainedCloneProgress{released_bytes:spent,..Default::default()},work:0,held_items:0,held_copy_bytes:0,held_release_bytes:held,held_capacity_bytes:0,held_fuel:0,claims:0,serial:0};
   assert_eq!(s.conserved(),c["conserved"].as_bool().unwrap());
   match c["remaining"].as_u64(){Some(n)=>assert_eq!(s.remaining().unwrap().0.maximum_release_bytes,n as usize),None=>assert!(s.remaining().is_err())}
  }
  assert_eq!(ORIGINAL_RETURN_ESCROW_ROW_BYTES,law["row"]["bytes"].as_u64().unwrap()as usize);
 }
 #[test]
 fn same_source_row_refuses_release_above_its_actual_hold(){
  let mut table=OriginalReturnEscrowTable::empty();let id=OriginalReturnEscrowIdentity{source:1,source_birth:2,recipient:3,destination:4,extent:0,serial:1,origin_epoch:1};
  let held=OriginalReturnEscrowAmounts{release_bytes:12,..Default::default()};table.reserve(id,held).unwrap().unwrap();
  assert!(table.redeem(id,OriginalReturnEscrowAmounts{release_bytes:13,..Default::default()},Default::default()).is_err());
  assert_eq!(table.row_at(0,1).unwrap().redeemed,Default::default());
  table.redeem(id,OriginalReturnEscrowAmounts{release_bytes:7,..Default::default()},OriginalReturnEscrowAmounts{release_bytes:5,..Default::default()}).unwrap();
  assert!(table.row_at(0,1).unwrap().tombstone);
 }
}

#[cfg(test)]
mod original_capacity_holds{
 use super::*;
 #[test]
 fn pending_capacity_stays_with_its_actual_source_claim(){
  let law:serde_json::Value=serde_json::from_str(include_str!("🧪️tests/♻️release/📐️.json")).unwrap();
  for c in law["capacityCases"].as_array().unwrap(){let stock=c["stock"].as_u64().unwrap()as usize;let spent=c["spent"].as_u64().unwrap()as usize;let held=c["held"].as_u64().unwrap()as usize;let s=OriginalReturnBudgetSnapshot{granted:RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:stock,maximum_release_bytes:0,maximum_depth:1},fuel:1,spent:RetainedCloneProgress{retained_capacity_bytes:spent,..Default::default()},work:0,held_items:0,held_copy_bytes:0,held_release_bytes:0,held_capacity_bytes:held,held_fuel:0,claims:0,serial:0};assert_eq!(s.conserved(),c["conserved"].as_bool().unwrap());match c["remaining"].as_u64(){Some(n)=>assert_eq!(s.remaining().unwrap().0.maximum_capacity_bytes,n as usize),None=>assert!(s.remaining().is_err())}}
 }
}
