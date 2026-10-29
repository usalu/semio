//! 📤️ Original provisional rows move out of resident addresses without opaque map-node release.
use super::*;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,RetainedCloneGrant,RetainedCloneProgress};
use std::mem::size_of;

/// 🔗️ Lends an actual resident tool-state source while its immutable native reader remains issued.
pub trait OriginalToolLedgerSource{
 type State:Send+Sync+'static;
 fn original_revision(&self)->u64;
 fn original_next(&self,after:Option<&str>)->Option<(&semio_framework_value::SharedUtf8,RetainedOriginalSlotKey)>;
 fn original_live(&self,window:&str,key:RetainedOriginalSlotKey)->Result<bool,ValueError>;
 fn original_read_demands(&self,window:&str,key:RetainedOriginalSlotKey)->Result<RetirementDemand,ValueError>;
 fn original_read_advance(&mut self,window:&str,key:RetainedOriginalSlotKey,factory:&std::sync::Arc<dyn semio_framework_value::ArtifactOwnedValueRetirementFactory<Self::State>>,grant:RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneStep,ValueError>;
 fn original_read(&self,window:&str,key:RetainedOriginalSlotKey)->Result<semio_framework_value::RetainedCloneRef<'_,Self::State>,ValueError>;
 fn original_return_demands(&self,window:&str,key:RetainedOriginalSlotKey,body:usize)->Result<RetirementDemand,ValueError>;
 fn original_return(&mut self,window:&str,key:RetainedOriginalSlotKey,factory:&mut Option<std::sync::Arc<dyn semio_framework_value::ArtifactOwnedValueRetirementFactory<Self::State>>>,grant:RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneStep,ValueError>;
}

macro_rules! original_transfer{($ledger:ident,$state:ident,$slot:ident,$slot_mut:ident)=>{
 impl<M:Send+Sync+'static> OriginalToolLedgerSource for $ledger<M>{
  type State=$state<M>;
  fn original_revision(&self)->u64{self.source_revision}
  fn original_next(&self,after:Option<&str>)->Option<(&semio_framework_value::SharedUtf8,RetainedOriginalSlotKey)>{self.next_retained_window(after)}
  fn original_live(&self,window:&str,key:RetainedOriginalSlotKey)->Result<bool,ValueError>{let slot=self.$slot(window).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original resident tool address absent"))?;if slot.key()!=key{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original resident tool key changed"))}Ok(!slot.is_empty())}
  fn original_read_demands(&self,window:&str,key:RetainedOriginalSlotKey)->Result<RetirementDemand,ValueError>{self.retained_read_demands(window,key)}
  fn original_read_advance(&mut self,window:&str,key:RetainedOriginalSlotKey,factory:&std::sync::Arc<dyn semio_framework_value::ArtifactOwnedValueRetirementFactory<Self::State>>,grant:RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneStep,ValueError>{self.advance_retained_read(window,key,factory,grant)}
  fn original_read(&self,window:&str,key:RetainedOriginalSlotKey)->Result<semio_framework_value::RetainedCloneRef<'_,Self::State>,ValueError>{self.retained_read(window,key)}
  fn original_return_demands(&self,window:&str,key:RetainedOriginalSlotKey,body:usize)->Result<RetirementDemand,ValueError>{self.retained_read_return_demands(window,key,body)}
  fn original_return(&mut self,window:&str,key:RetainedOriginalSlotKey,factory:&mut Option<std::sync::Arc<dyn semio_framework_value::ArtifactOwnedValueRetirementFactory<Self::State>>>,grant:RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneStep,ValueError>{self.return_retained_read(window,key,factory,grant)}
 }
 impl<M:Send+Sync+'static> $ledger<M>{
  /// 📏️ Quotes the original row, resident generation and complete-ledger source frontier once.
  pub fn original_transfer_demands()->RetirementDemand{let mut d=RetainedOriginalSlot::<$state<M>>::transfer_demand();d.copy_bytes+=size_of::<u64>()+size_of::<usize>();d}
  /// 🪪️ Borrows the sealed resident identity, including a previously withdrawn address.
  pub fn resident_key(&self,window:&str)->Option<RetainedOriginalSlotKey>{self.$slot(window).map(RetainedOriginalSlot::key)}
  /// 📤️ Hides a run and donates the same original row after every semantic reader returns.
  pub fn withdraw_original(&mut self,window:&str,key:RetainedOriginalSlotKey,output:&mut Option<$state<M>>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{let revision=self.source_revision.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original tool source revision exhausted"))?;let d=Self::original_transfer_demands();let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes.saturating_sub(size_of::<u64>()+size_of::<usize>()),..grant};let slot=self.$slot_mut(window).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original resident tool address absent"))?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(None)}let Some(mut p)=slot.take_original(key,output,child)?else{return Ok(None)};self.source_revision=revision;self.live_windows=self.live_windows.checked_sub(1).expect("original resident tool count");p.copied_bytes+=size_of::<u64>()+size_of::<usize>();Ok(Some(p))}
  /// 📥️ Adopts an authored original into the same vacant address without reconstructing its identity.
  pub fn install_resident_original(&mut self,window:&str,key:RetainedOriginalSlotKey,input:&mut Option<$state<M>>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{let revision=self.source_revision.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original tool source revision exhausted"))?;let live=self.live_windows.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original tool source count exhausted"))?;let d=Self::original_transfer_demands();if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(None)}let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<u64>()-size_of::<usize>(),..grant};let slot=self.$slot_mut(window).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original resident tool address absent"))?;let Some(mut p)=slot.install_original(key,input,child)?else{return Ok(None)};self.source_revision=revision;self.live_windows=live;p.copied_bytes+=size_of::<u64>()+size_of::<usize>();Ok(Some(p))}
 }
};}
original_transfer!(ScrubLedger,ScrubState,slot,slot_mut);
original_transfer!(TypingLedger,TypingState,original_slot,original_slot_mut);
original_transfer!(GestureLedger,GestureState,original_slot,original_slot_mut);

macro_rules! map_slots{($ledger:ident,$state:ident)=>{impl<M> $ledger<M>{fn original_slot(&self,window:&str)->Option<&RetainedOriginalSlot<$state<M>>>{self.windows.get(window)}fn original_slot_mut(&mut self,window:&str)->Option<&mut RetainedOriginalSlot<$state<M>>>{self.windows.get_mut(window)}}};}
map_slots!(TypingLedger,TypingState);
map_slots!(GestureLedger,GestureState);
