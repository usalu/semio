//! 🛡️ Exact Source-table guard work remains owned through later Source observation.
use super::{invalid,OriginalReturnBudgetSnapshot,OriginalReturnEscrowAmounts,OriginalReturnEscrowIdentity,OriginalReturnEscrowTable,RetainedCloneProgress,ValueError};
use std::mem::size_of;
/// 📏️ Conservative preparation scope assumes the actual Source table is already born.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalReturnGuardDemand{pub items:usize,pub copy_bytes:usize,pub work:u64,pub depth:usize}
/// 🧮️ Pure checked changes do not issue a Source claim or authorize a guard.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalReturnGuardPlan{pub serial:u64,pub claims:u8,pub held_items:u8,pub held_copy_bytes:usize,pub held_fuel:u8}
/// 🧾️ A later actual Source observer applies this checked debt settlement.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalReturnGuardSettlementPlan{pub claims:u8,pub held_items:u8,pub held_copy_bytes:usize,pub held_fuel:u8,pub spent:RetainedCloneProgress,pub work:u128}
impl OriginalReturnBudgetSnapshot{
 pub fn plan_guard(&self)->Result<OriginalReturnGuardPlan,ValueError>{let(available,fuel)=self.remaining()?;let copy=OriginalReturnEscrowTable::guard_check_copy_bytes();if available.maximum_items<1||available.maximum_copy_bytes<copy||fuel<2||self.claims>=super::ORIGINAL_RETURN_ESCROW_CELLS{return Err(invalid())}Ok(OriginalReturnGuardPlan{serial:self.serial.checked_add(1).ok_or_else(invalid)?,claims:u8::try_from(self.claims).map_err(|_|invalid())?.checked_add(1).ok_or_else(invalid)?,held_items:u8::try_from(self.held_items).map_err(|_|invalid())?.checked_add(1).ok_or_else(invalid)?,held_copy_bytes:self.held_copy_bytes.checked_add(copy).ok_or_else(invalid)?,held_fuel:u8::try_from(self.held_fuel).map_err(|_|invalid())?.checked_add(2).ok_or_else(invalid)?})}
 pub fn plan_guard_settlement(&self,spent:bool)->Result<OriginalReturnGuardSettlementPlan,ValueError>{let claims=u8::try_from(self.claims).map_err(|_|invalid())?.checked_sub(1).ok_or_else(invalid)?;let held_items=u8::try_from(self.held_items).map_err(|_|invalid())?.checked_sub(1).ok_or_else(invalid)?;let held_copy_bytes=self.held_copy_bytes.checked_sub(OriginalReturnEscrowTable::guard_check_copy_bytes()).ok_or_else(invalid)?;let held_fuel=u8::try_from(self.held_fuel).map_err(|_|invalid())?.checked_sub(2).ok_or_else(invalid)?;let progress=if spent{RetainedCloneProgress{copied_items:1,copied_bytes:OriginalReturnEscrowTable::guard_check_copy_bytes(),..Default::default()}}else{Default::default()};let actual=self.spent.checked_add(progress)?;let work=self.work.checked_add(if spent{2}else{0}).ok_or_else(invalid)?;let mut proposed=*self;proposed.spent=actual;proposed.held_items=usize::from(held_items);proposed.held_copy_bytes=held_copy_bytes;proposed.held_fuel=u64::from(held_fuel);proposed.work=work;if !proposed.conserved(){return Err(invalid())}Ok(OriginalReturnGuardSettlementPlan{claims,held_items,held_copy_bytes,held_fuel,spent:actual,work})}

}
/// 🎟️ A single table-issued guard binds one real receipt address and one stable owner slot.
pub struct OriginalReturnGuardPermit{table:usize,identity:OriginalReturnEscrowIdentity,slot:usize,destination:usize,phase:u8}
impl OriginalReturnGuardPermit{
 pub fn pending(&self)->Option<(RetainedCloneProgress,u64)>{match self.phase{1=>Some((RetainedCloneProgress{copied_items:1,copied_bytes:OriginalReturnEscrowTable::guard_check_copy_bytes(),..Default::default()},2)),2=>Some((Default::default(),0)),_=>None}}
 pub fn receipt_address(&self)->usize{self.identity.destination as usize}
 pub fn identity(&self)->OriginalReturnEscrowIdentity{self.identity}
}
impl Drop for OriginalReturnGuardPermit{fn drop(&mut self){assert!(std::thread::panicking()||self.phase==3,"guard permit requires actual Source observation")}}
impl OriginalReturnEscrowTable{
 /// 📏️ Quotes actual no-context metadata writes funded by the held SRE row.
 pub fn guard_check_copy_bytes()->usize{4*size_of::<u64>()+size_of::<bool>()+2*size_of::<u8>()}
 /// 📥️ Quotes later actual Source-observation metadata.
 pub fn guard_finish_copy_demand()->usize{size_of::<Option<OriginalReturnGuardPermit>>()+size_of::<u32>()+4*size_of::<u8>()+size_of::<usize>()+size_of::<u128>()+size_of::<RetainedCloneProgress>()}

 /// 🧮️ Supplies a cross-target ceiling for staged claim, permit metadata and held work.
 pub fn guard_prepare_reserve()->Result<OriginalReturnGuardDemand,ValueError>{let stage=2*size_of::<u64>()+size_of::<Option<(usize,u8)>>() +1+13*(size_of::<u64>()+size_of::<Option<(usize,u8)>>())+size_of::<u64>();let actual=stage.checked_add(Self::guard_mint_copy_demand()).and_then(|v|v.checked_add(Self::guard_check_copy_bytes())).and_then(|v|v.checked_add(Self::guard_finish_copy_demand())).and_then(|v|v.checked_add(256)).ok_or_else(invalid)?;if actual>2048{return Err(invalid())}Ok(OriginalReturnGuardDemand{items:32,copy_bytes:2048,work:64,depth:1})}

 /// 📦️ Quotes the actual fixed claim birth and prepaid guard-owner transitions.
 pub fn guard_mint_copy_demand()->usize{size_of::<super::OriginalReturnEscrowRow>()+size_of::<Option<OriginalReturnGuardPermit>>()+size_of::<u32>()+2*size_of::<u64>()+4*size_of::<u8>()+size_of::<usize>()}
 /// 🌱️ Reserves one exact work-only claim and its sole permit as one checked table transition.
 pub fn reserve_guard(&mut self,identity:OriginalReturnEscrowIdentity,out:&mut Option<OriginalReturnGuardPermit>)->Result<bool,ValueError>{
  if out.is_some()||self.stage.is_some()||identity.source==0||identity.source_birth==0||identity.recipient==0||identity.destination==0||identity.serial==0||identity.origin_epoch==0||!matches!(identity.extent,144|152|184)||self.row_after(identity.serial.saturating_sub(1)).is_some_and(|row|row.identity.serial==identity.serial){return Err(invalid())}
  let Some(slot)=self.states.iter().position(|state|*state==0)else{return Ok(false)};if self.guard_issued&(1u32<<slot)!=0{return Err(invalid())}let revision=self.revision.checked_add(1).ok_or_else(invalid)?;let destination=out as *mut Option<OriginalReturnGuardPermit> as usize;let table=self as *mut Self as usize;self.rows[slot]=super::OriginalReturnEscrowRow{identity,held:OriginalReturnEscrowAmounts{items:1,copy_bytes:Self::guard_check_copy_bytes() as u64,work:2},redeemed:OriginalReturnEscrowAmounts::zero(),released:OriginalReturnEscrowAmounts::zero(),tombstone:false};self.states[slot]=2;self.guard_issued|=1u32<<slot;self.revision=revision;*out=Some(OriginalReturnGuardPermit{table,identity,slot,destination,phase:0});Ok(true)
 }
 /// 🔎️ Checks the exact live table, receipt address and stable preborn owner slot.
 pub fn qualify_guard(&self,out:&Option<OriginalReturnGuardPermit>,pointer:usize)->Result<(),ValueError>{let permit=out.as_ref().ok_or_else(invalid)?;if permit.destination!=out as *const Option<OriginalReturnGuardPermit> as usize||permit.table!=self as *const Self as usize||permit.phase!=0||pointer!=permit.identity.destination as usize||self.guard_issued&(1u32<<permit.slot)==0{return Err(invalid())}let row=self.row_at(permit.slot,permit.identity.serial).ok_or_else(invalid)?;if row.identity!=permit.identity||row.tombstone{return Err(invalid())}Ok(())}
 /// ⚡️ Spends the preissued work once before the exact no-context range check.
 pub fn begin_guard_check(&mut self,out:&mut Option<OriginalReturnGuardPermit>,pointer:usize)->Result<(),ValueError>{
  let destination=out as *mut Option<OriginalReturnGuardPermit> as usize;let permit=out.as_mut().ok_or_else(invalid)?;if permit.destination!=destination||permit.table!=self as *mut Self as usize||permit.phase!=0||pointer!=permit.identity.destination as usize||self.guard_issued&(1u32<<permit.slot)==0{return Err(invalid())}
  let row=self.row_at(permit.slot,permit.identity.serial).ok_or_else(invalid)?;if row.identity!=permit.identity||row.held!=(OriginalReturnEscrowAmounts{items:1,copy_bytes:Self::guard_check_copy_bytes() as u64,work:2})||row.redeemed!=OriginalReturnEscrowAmounts::zero()||row.released!=OriginalReturnEscrowAmounts::zero()||row.tombstone{return Err(invalid())}let revision=self.revision.checked_add(1).ok_or_else(invalid)?;let row=&mut self.rows[permit.slot];row.redeemed=row.held;row.tombstone=true;self.states[permit.slot]=3;self.revision=revision;permit.phase=1;Ok(())
 }
 /// ↩️ Releases unused work only through its actual Source cancellation path.
 pub fn release_guard(&mut self,out:&mut Option<OriginalReturnGuardPermit>)->Result<(),ValueError>{
  let destination=out as *mut Option<OriginalReturnGuardPermit> as usize;let permit=out.as_mut().ok_or_else(invalid)?;if permit.destination!=destination||permit.table!=self as *mut Self as usize||permit.phase!=0||self.guard_issued&(1u32<<permit.slot)==0{return Err(invalid())}let row=self.row_at(permit.slot,permit.identity.serial).ok_or_else(invalid)?;if row.identity!=permit.identity||row.held!=(OriginalReturnEscrowAmounts{items:1,copy_bytes:Self::guard_check_copy_bytes() as u64,work:2})||row.redeemed!=OriginalReturnEscrowAmounts::zero()||row.released!=OriginalReturnEscrowAmounts::zero()||row.tombstone{return Err(invalid())}let revision=self.revision.checked_add(1).ok_or_else(invalid)?;let row=&mut self.rows[permit.slot];row.released=row.held;row.tombstone=true;self.states[permit.slot]=3;self.revision=revision;permit.phase=2;Ok(())
 }
 /// 🧾️ Clears the permit only after its Source has posted the matching pending observation.
 pub fn finish_guard(&mut self,out:&mut Option<OriginalReturnGuardPermit>)->Result<(),ValueError>{
  let destination=out as *mut Option<OriginalReturnGuardPermit> as usize;let permit=out.as_mut().ok_or_else(invalid)?;if permit.destination!=destination||permit.table!=self as *mut Self as usize||!matches!(permit.phase,1|2)||self.guard_issued&(1u32<<permit.slot)==0{return Err(invalid())}let row=self.row_at(permit.slot,permit.identity.serial).ok_or_else(invalid)?;if row.identity!=permit.identity||!row.tombstone{return Err(invalid())}self.guard_issued&=!(1u32<<permit.slot);permit.phase=3;out.take();Ok(())
 }
}

#[cfg(test)]
mod tests{
 use super::*;
 #[test]
 fn guard_spends_only_its_exact_held_table_row_and_waits_for_observation(){
  let identity=OriginalReturnEscrowIdentity{source:1,source_birth:2,recipient:3,destination:4,extent:144,serial:5,origin_epoch:6};let mut table=Box::new(OriginalReturnEscrowTable::empty());let mut permit=None;assert!(table.reserve_guard(identity,&mut permit).unwrap());let slot=table.slot_for(identity.serial).unwrap();let revision=table.revision();table.borrow_snapshot(revision).unwrap();assert!(table.begin_guard_check(&mut permit,5).is_err());assert!(permit.as_ref().unwrap().pending().is_none());table.begin_guard_check(&mut permit,4).unwrap();assert_eq!(permit.as_ref().unwrap().pending(),Some((RetainedCloneProgress{copied_items:1,copied_bytes:OriginalReturnEscrowTable::guard_check_copy_bytes(),..Default::default()},2)));assert!(table.begin_guard_check(&mut permit,4).is_err());table.hand_back_snapshot(revision).unwrap();assert!(!table.has_reclaimable());table.finish_guard(&mut permit).unwrap();assert!(permit.is_none());assert!(!table.has_reclaimable());let next=table.revision();table.borrow_snapshot(next).unwrap();table.hand_back_snapshot(next).unwrap();assert!(table.reclaim_one().unwrap());assert!(table.is_empty());
 }
 #[test]
 fn unused_guard_returns_held_work_without_redemption(){
  let identity=OriginalReturnEscrowIdentity{source:11,source_birth:12,recipient:13,destination:14,extent:144,serial:15,origin_epoch:16};let mut table=Box::new(OriginalReturnEscrowTable::empty());let mut permit=None;assert!(table.reserve_guard(identity,&mut permit).unwrap());let slot=table.slot_for(identity.serial).unwrap();table.release_guard(&mut permit).unwrap();assert_eq!(permit.as_ref().unwrap().pending(),Some((RetainedCloneProgress::default(),0)));let row=table.row_at(slot,identity.serial).unwrap();assert_eq!((row.redeemed.work,row.released.work),(0,2));table.finish_guard(&mut permit).unwrap();let revision=table.revision();table.borrow_snapshot(revision).unwrap();table.hand_back_snapshot(revision).unwrap();assert!(table.reclaim_one().unwrap());assert!(table.is_empty());
 }
 #[test]
 fn work_only_guard_plan_holds_then_spends_or_releases_exact_work(){
  let mut s=OriginalReturnBudgetSnapshot{granted:super::super::RetainedCloneGrant{maximum_items:4,maximum_copy_bytes:100,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1},fuel:5,spent:Default::default(),work:1,held_items:0,held_copy_bytes:0,held_fuel:0,claims:0,serial:4};let plan=s.plan_guard().unwrap();assert_eq!((plan.serial,plan.claims,plan.held_fuel),(5,1,2));s.serial=plan.serial;s.claims=usize::from(plan.claims);s.held_fuel=u64::from(plan.held_fuel);s.held_items=usize::from(plan.held_items);s.held_copy_bytes=plan.held_copy_bytes;let spent=s.plan_guard_settlement(true).unwrap();let unused=s.plan_guard_settlement(false).unwrap();assert_eq!((spent.claims,spent.held_fuel,spent.work),(0,0,3));assert_eq!((unused.claims,unused.held_fuel,unused.work),(0,0,1));s.fuel=2;assert!(s.plan_guard().is_err());
 }

}
