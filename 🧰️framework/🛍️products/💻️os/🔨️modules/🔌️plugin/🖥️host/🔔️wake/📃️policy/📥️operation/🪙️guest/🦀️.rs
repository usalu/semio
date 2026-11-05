//! 🪙️ Actual Source authority reserves guest logical stock separately from physical host receipts.
use super::{consumable_remaining,intersect,invalid,spent,GuestRelayWakeDriver,GuestRelayAdmissionCall,GuestRelayAdmissionFailure};
use semio_framework_actor::{RetainedTurnInput,OriginalActorTurnWork,OriginalJobBudget};
use semio_framework_job::{OriginalNativeLease,OriginalNativeRetirement,OriginalNativeRecipientIdentity,OriginalTurnFailure};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,ValueError};

#[derive(Clone,Copy,Default)]
pub(super) struct GuestUsage{items:usize,copy:usize,capacity:usize,release:usize,work:u64}
impl GuestUsage{
 fn actual(progress:RetainedCloneProgress,work:u64)->Self{Self{items:progress.copied_items,copy:progress.copied_bytes,capacity:progress.retained_capacity_bytes,release:progress.released_bytes,work}}
 fn reservation(grant:RetainedCloneGrant,work:u64)->Self{Self{items:grant.maximum_items,copy:grant.maximum_copy_bytes,capacity:grant.maximum_capacity_bytes,release:grant.maximum_release_bytes,work}}
 fn add(self,other:Self)->Result<Self,ValueError>{let overflow=||invalid("guest logical reservation counter overflow");Ok(Self{items:self.items.checked_add(other.items).ok_or_else(overflow)?,copy:self.copy.checked_add(other.copy).ok_or_else(overflow)?,capacity:self.capacity.checked_add(other.capacity).ok_or_else(overflow)?,release:self.release.checked_add(other.release).ok_or_else(overflow)?,work:self.work.checked_add(other.work).ok_or_else(overflow)?})}
 fn policy(self)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:self.items,maximum_copy_bytes:self.copy,maximum_capacity_bytes:self.capacity,maximum_release_bytes:self.release,maximum_depth:0}}
}
/// 🎫️ This sealed descriptor selects an existing issuer-owned reservation; its data alone never funds a callback.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum GuestRelayGuestPurpose{Semantic,Infrastructure}
impl GuestRelayGuestPurpose{fn index(self)->usize{match self{Self::Semantic=>0,Self::Infrastructure=>1}}}
#[derive(Clone,Copy)]
pub struct GuestRelayGuestSubloan{serial:u64,purpose:GuestRelayGuestPurpose,budget:OriginalJobBudget}
impl GuestRelayGuestSubloan{pub fn budget(&self)->OriginalJobBudget{self.budget}pub fn purpose(&self)->GuestRelayGuestPurpose{self.purpose}}
#[derive(Clone,Copy)]
struct GuestHold{descriptor:GuestRelayGuestSubloan,producer:OriginalNativeRecipientIdentity}
#[derive(Default)]
struct GuestAccount{observed:GuestUsage,forfeited:GuestUsage}
impl GuestAccount{fn usage(&self)->Result<GuestUsage,ValueError>{self.observed.add(self.forfeited)}}
#[derive(Default)]
pub(crate) struct GuestLedger{accounts:[GuestAccount;2],hold:Option<GuestHold>}
impl GuestLedger{
 pub(crate) fn pending(&self)->bool{self.hold.is_some()}
 fn usage(&self,held:bool)->Result<GuestUsage,ValueError>{let mut usage=self.accounts[0].usage()?.add(self.accounts[1].usage()?)?;if held{if let Some(hold)=self.hold{usage=usage.add(GuestUsage::reservation(hold.descriptor.budget.retained_policy,hold.descriptor.budget.fuel))?;}}Ok(usage)}
 pub(super) fn physical_remaining(&self,total:RetainedCloneGrant)->Result<RetainedCloneGrant,ValueError>{consumable_remaining(total,self.usage(true)?.policy())}
 pub(super) fn physical_fuel(&self,total:u64)->Result<u64,ValueError>{total.checked_sub(self.usage(true)?.work).ok_or_else(||spent("host work is held or debited by actual guest authority"))}
 fn logical_remaining(&self,purpose:GuestRelayGuestPurpose,total:RetainedCloneGrant)->Result<RetainedCloneGrant,ValueError>{consumable_remaining(total,self.accounts[purpose.index()].usage()?.policy())}
 fn logical_fuel(&self,purpose:GuestRelayGuestPurpose,total:u64)->Result<u64,ValueError>{total.checked_sub(self.accounts[purpose.index()].usage()?.work).ok_or_else(||spent("requested guest purpose work is exhausted"))}
}
impl GuestRelayWakeDriver{
 /// 🌱️ Verifies the live typed producer on a paid callback before reserving only still-unspent guest authority.
 pub fn admit_guest_subloan<T:OriginalNativeRetirement>(&self,producer:&mut OriginalNativeLease<T>,purpose:GuestRelayGuestPurpose)->Result<Option<GuestRelayGuestSubloan>,GuestRelayAdmissionFailure>{
  let identity=producer.recipient_identity();let mut verified=false;
  if !self.dispatch_admission_step(false,|_,cx|GuestRelayAdmissionCall{result:producer.with_native(cx,|_,_|{verified=true;Ok(())}).map(|_|()),execution:None})?||!verified{return Ok(None)}
  let mut original=self.original.try_lock().map_err(|_|invalid("guest reservation issuer is borrowed"))?;
  if original.checkout.is_some(){return Ok(None)}if (original.cancelled)(){return Err(spent("a cancelled operation cannot start another guest crossing").into())}
  let operation=original.operation.as_ref().ok_or_else(||invalid("guest reservation requires an actual per-operation Source"))?;
  if operation.cleanup||operation.closed{return Err(invalid("guest semantic admission cannot follow Source cleanup").into())}if operation.guest.pending(){return Err(invalid("guest crossing still owes its exact fixed receipt").into())}
  let physical=operation.guest.physical_remaining(original.treasury.original_remaining())?;let available=consumable_remaining(consumable_remaining(physical,operation.reserve.retained)?,operation.transit.retained)?;
  let semantic=operation.guest.logical_remaining(GuestRelayGuestPurpose::Semantic,operation.request.retained)?;let infrastructure=operation.guest.logical_remaining(GuestRelayGuestPurpose::Infrastructure,operation.infrastructure.retained)?;
  let total=GuestUsage::reservation(semantic,0).add(GuestUsage::reservation(infrastructure,0))?;let header_grant=intersect(available,RetainedCloneGrant{maximum_depth:semantic.maximum_depth.max(infrastructure.maximum_depth),..total.policy()});
  let grant=intersect(header_grant,match purpose{GuestRelayGuestPurpose::Semantic=>semantic,GuestRelayGuestPurpose::Infrastructure=>infrastructure});
  let physical_fuel=operation.guest.physical_fuel(original.root.fuel_remaining())?.checked_sub(operation.reserve.work_fuel).and_then(|fuel|fuel.checked_sub(operation.transit.work_fuel)).ok_or_else(||spent("guest reservation cannot consume cleanup or transport work"))?;
  let semantic_fuel=operation.guest.logical_fuel(GuestRelayGuestPurpose::Semantic,operation.request.work_units)?;let infrastructure_fuel=operation.guest.logical_fuel(GuestRelayGuestPurpose::Infrastructure,operation.infrastructure.work_fuel)?;
  let header_fuel=semantic_fuel.checked_add(infrastructure_fuel).ok_or_else(||invalid("guest total work overflow"))?.min(physical_fuel);let fuel=header_fuel.min(match purpose{GuestRelayGuestPurpose::Semantic=>semantic_fuel,GuestRelayGuestPurpose::Infrastructure=>infrastructure_fuel});if fuel==0||grant.maximum_items==0{return Err(spent("no guest purpose stock remains beside host cleanup and transport").into())}
  let root_work=original.root.remaining_work();let deadline=root_work.absolute_deadline_us.checked_sub(operation.reserve.maximum_microseconds).ok_or_else(||invalid("guest deadline cannot remove its original cleanup window"))?;
  if original.root.clock_source()().is_none_or(|now|now>=deadline){return Err(spent("guest admission reached its original semantic deadline").into())}
  let epoch=original.treasury.issue_guest_epoch()?;let input=RetainedTurnInput{operation:original.root.policy().operation,generation:original.root.policy().generation,epoch,work:OriginalActorTurnWork{fuel:header_fuel,..root_work},grant:header_grant};input.validate()?;
  let descriptor=GuestRelayGuestSubloan{serial:semio_framework_trace::allocate_operation_id().0,purpose,budget:OriginalJobBudget{input,retained_policy:grant,fuel,deadline_us:deadline,cancelled:false}};
  original.operation.as_mut().unwrap().guest.hold=Some(GuestHold{descriptor,producer:identity});Ok(Some(descriptor))
 }
 /// 🧾️ Charges only the actual returned guest receipt; host physical spending remains in its own original ledger.
 pub fn receive_guest_subloan(&self,descriptor:GuestRelayGuestSubloan,input:RetainedTurnInput,progress:RetainedCloneProgress,work:u64)->Result<(),ValueError>{
  let mut original=self.original.try_lock().map_err(|_|invalid("guest receipt issuer is borrowed"))?;if original.checkout.is_some(){return Err(invalid("guest receipt waits for its actual host callback handback"))}
  let ledger=&mut original.operation.as_mut().ok_or_else(||invalid("guest receipt requires its original operation"))?.guest;let hold=ledger.hold.ok_or_else(||invalid("guest reservation was already returned"))?;
  if hold.descriptor.serial!=descriptor.serial||hold.descriptor.purpose!=descriptor.purpose||hold.descriptor.budget!=descriptor.budget||input!=hold.descriptor.budget.input||!progress.fits(hold.descriptor.budget.retained_policy)||work>hold.descriptor.budget.fuel{return Err(invalid("guest receipt changes its actual reservation or exceeds its held stock"))}
  let account=&mut ledger.accounts[descriptor.purpose.index()];let observed=account.observed.add(GuestUsage::actual(progress,work))?;account.observed=observed;ledger.hold=None;Ok(())
 }
 /// 🏁️ A forfeited logical hold requires paid retirement of its actual generation-qualified producer and every child Source.
 pub(super) fn forfeit_guest_subloan_step(&self)->Result<bool,GuestRelayAdmissionFailure>{
  let hold={let original=self.original.try_lock().map_err(|_|invalid("guest forfeit issuer is borrowed"))?;let Some(operation)=original.operation.as_ref()else{return Ok(true)};let Some(hold)=operation.guest.hold else{return Ok(true)};if original.root.native_sources_pending(){return Ok(false)}hold};
  let mut witnessed=false;if !self.dispatch_cleanup_admission_step(false,|_,cx|{let result=(||{if let Some(witness)=cx.original_native_retired_witness(hold.producer)?{if !witness.proves(hold.producer,cx){return Err(invalid("guest forfeit received another producer's closure witness").into())}witnessed=true;}Ok(())})();GuestRelayAdmissionCall{result,execution:None}})?||!witnessed{return Ok(false)}
  let mut original=self.original.try_lock().map_err(|_|invalid("guest forfeit issuer is borrowed"))?;let ledger=&mut original.operation.as_mut().ok_or_else(||invalid("guest forfeit lost its original operation"))?.guest;
  let actual=ledger.hold.ok_or_else(||invalid("guest hold changed during paid producer retirement"))?;if actual.descriptor.serial!=hold.descriptor.serial{return Err(invalid("guest forfeit belongs to another pending reservation").into())}
  let account=&mut ledger.accounts[hold.descriptor.purpose.index()];account.forfeited=account.forfeited.add(GuestUsage::reservation(hold.descriptor.budget.retained_policy,hold.descriptor.budget.fuel))?;ledger.hold=None;Ok(true)
 }
}
#[cfg(test)]
mod tests{
 use super::*;
 #[test]
 fn guest_logical_holds_and_forfeits_never_become_physical_clone_receipts(){
  let fixture:serde_json::Value=serde_json::from_str(include_str!("🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
  let usage=|row:&serde_json::Value|GuestUsage{items:row[0].as_u64().unwrap()as usize,copy:row[1].as_u64().unwrap()as usize,capacity:row[2].as_u64().unwrap()as usize,release:row[3].as_u64().unwrap()as usize,work:row[4].as_u64().unwrap()};
  let total=usage(&fixture["total"]);let base=usage(&fixture["cleanup"]).add(usage(&fixture["transit"])).unwrap();
  for row in fixture["cases"].as_array().unwrap(){let mut debit=base.add(usage(&row["observed"])).unwrap().add(usage(&row["forfeited"])).unwrap();if row["held"].as_bool().unwrap(){debit=debit.add(usage(&fixture["hold"])).unwrap()}let actual=consumable_remaining(total.policy(),debit.policy()).unwrap();assert_eq!([actual.maximum_items as u64,actual.maximum_copy_bytes as u64,actual.maximum_capacity_bytes as u64,actual.maximum_release_bytes as u64,total.work-debit.work],serde_json::from_value::<[u64;5]>(row["expected"].clone()).unwrap());}
 }
 #[test]
 fn purpose_headers_share_only_the_actual_remaining_total(){
  let fixture:serde_json::Value=serde_json::from_str(include_str!("🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let fixture=&fixture["purposes"];
  let usage=|row:&serde_json::Value|GuestUsage{items:row[0].as_u64().unwrap()as usize,copy:row[1].as_u64().unwrap()as usize,capacity:row[2].as_u64().unwrap()as usize,release:row[3].as_u64().unwrap()as usize,work:row[4].as_u64().unwrap()};
  for row in fixture["cases"].as_array().unwrap(){let mut ledger=GuestLedger::default();ledger.accounts[0].observed=usage(&row["semanticSpent"]);ledger.accounts[1].observed=usage(&row["infrastructureSpent"]);let mut remaining=[GuestUsage::default();2];for(index,key)in ["semantic","infrastructure"].iter().enumerate(){let purpose=if index==0{GuestRelayGuestPurpose::Semantic}else{GuestRelayGuestPurpose::Infrastructure};let total=usage(&fixture[*key]);remaining[index]=GuestUsage::reservation(ledger.logical_remaining(purpose,total.policy()).unwrap(),ledger.logical_fuel(purpose,total.work).unwrap());}let header=remaining[0].add(remaining[1]).unwrap();let scope=remaining[if row["purpose"]=="semantic"{0}else{1}];let array=|u:GuestUsage|[u.items as u64,u.copy as u64,u.capacity as u64,u.release as u64,u.work];assert_eq!(array(header),serde_json::from_value::<[u64;5]>(row["expectedHeader"].clone()).unwrap());assert_eq!(array(scope),serde_json::from_value::<[u64;5]>(row["expectedScope"].clone()).unwrap());}
 }
 #[test]
 fn real_source_guest_hold_refuses_foreign_receipts_and_forfeits_only_after_paid_native_producer_retirement(){
  struct Producer(bool);
  impl OriginalNativeRetirement for Producer{
   fn close_on_source(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,OriginalTurnFailure>{if self.0{return Ok(true)}if cx.retirement_should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_copy_bytes<1{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:1,..Default::default()})?;cx.consume_fuel(1);self.0=true;Ok(true)}
   fn terminal_is_empty(&self)->bool{self.0}
  }
  impl Drop for Producer{fn drop(&mut self){assert!(self.0,"actual native producer requires paid close");}}
  let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let caps:super::super::GuestRelayOperationCaps=serde_json::from_value(fixture["caps"].clone()).unwrap();let request:super::super::GuestRelayOperationRequest=serde_json::from_value(fixture["cases"][0]["request"].clone()).unwrap();let driver=caps.issue(request,||Some(1000),Box::new(||false)).unwrap();let mut input=Some(Producer(false));let mut lease=None;
  for _ in 0..64{driver.dispatch_admission_step(false,|_,cx|GuestRelayAdmissionCall{result:cx.admit_original_native(&mut input,&mut lease).map(|_|()),execution:None}).unwrap();if lease.is_some(){break}}assert!(input.is_none());let mut lease=lease.unwrap();
  let mut actual_parent=false;for _ in 0..32{if driver.dispatch_operation_step(false,false,|cx|{lease.with_native(cx,|_,_|{actual_parent=true;Ok(())})?;Ok(())}).unwrap(){break}}assert!(actual_parent);
  let mut infrastructure=None;for _ in 0..32{infrastructure=driver.admit_guest_subloan(&mut lease,GuestRelayGuestPurpose::Infrastructure).unwrap();if infrastructure.is_some(){break}}let infrastructure=infrastructure.unwrap();let infrastructure_budget=infrastructure.budget();assert!(infrastructure_budget.input.grant.maximum_capacity_bytes>request.retained.maximum_capacity_bytes);driver.receive_guest_subloan(infrastructure,infrastructure_budget.input,RetainedCloneProgress{copied_items:1,copied_bytes:1,..Default::default()},1).unwrap();
  let mut first=None;for _ in 0..32{first=driver.admit_guest_subloan(&mut lease,GuestRelayGuestPurpose::Semantic).unwrap();if first.is_some(){break}}let first=first.unwrap();let budget=first.budget();assert_eq!(budget.fuel,request.work_units);assert_eq!(budget.input.work.fuel+1,infrastructure_budget.input.work.fuel);assert_eq!(budget.retained_policy.maximum_capacity_bytes,request.retained.maximum_capacity_bytes);assert!(driver.receive_guest_subloan(first,RetainedTurnInput{epoch:budget.input.epoch+1,..budget.input},Default::default(),0).is_err());assert!(!driver.forfeit_guest_subloan_step().unwrap());
  let before=driver.original.try_lock().unwrap().treasury.original_remaining();driver.receive_guest_subloan(first,budget.input,RetainedCloneProgress{copied_items:1,copied_bytes:1,..Default::default()},1).unwrap();assert_eq!(before,driver.original.try_lock().unwrap().treasury.original_remaining());assert!(driver.receive_guest_subloan(first,budget.input,Default::default(),0).is_err());
  let mut pending=None;for _ in 0..32{pending=driver.admit_guest_subloan(&mut lease,GuestRelayGuestPurpose::Semantic).unwrap();if pending.is_some(){break}}let pending=pending.unwrap();assert!(!driver.forfeit_guest_subloan_step().unwrap());drop(lease);
  for _ in 0..256{if driver.close_root_step().unwrap(){break}}assert!(driver.operation_terminal_is_empty().unwrap());let original=driver.original.try_lock().unwrap();let ledger=&original.operation.as_ref().unwrap().guest;assert_eq!(ledger.accounts[0].observed.work,1);assert_eq!(ledger.accounts[0].forfeited.work,pending.budget().fuel);assert!(!ledger.pending());
 }

}
