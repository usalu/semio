//! 🧾️ Original Host turn currency has one exclusive checked-out wake and no implicit renewal.
use super::{GuestRelayWakeReceipt,GuestRelayWakeReceiptReceiver};
use semio_framework_value::{ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress},retirement::controlled::RetainedOwnerGate};
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct GuestRelayWakeTurnGrant{pub epoch:u64,pub grant:RetainedCloneGrant}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct GuestRelayWakeTurnReceipt{pub epoch:u64,pub original:RetainedCloneGrant,pub spent:RetainedCloneProgress,pub remaining:RetainedCloneGrant}
struct OriginalTurn{epoch:u64,original:RetainedCloneGrant,remaining:RetainedCloneGrant,spent:RetainedCloneProgress,checkout:Option<(usize,u64,RetainedCloneGrant)>}
struct DriverState{last_epoch:u64,turn:Option<OriginalTurn>}
pub struct GuestRelayWakeReceiptLedger{original:RetainedOwnerGate<DriverState>}
impl Drop for GuestRelayWakeReceiptLedger{fn drop(&mut self){assert!(std::thread::panicking()||self.original.get_mut().turn.is_none(),"original Host wake turn or receipt was not returned to its driver");}}
impl GuestRelayWakeReceiptLedger{
 pub const fn new()->Self{Self{original:RetainedOwnerGate::new(DriverState{last_epoch:0,turn:None})}}
 /// 🎬️ Only the genuine embedding driver supplies a new original turn after the preceding turn returns.
 pub fn begin_turn(&self,epoch:u64,grant:RetainedCloneGrant)->Result<bool,ValueError>{
  if epoch==0{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original Host wake turn requires its nonzero epoch"))}
  let Ok(mut original)=self.original.try_lock()else{return Ok(false)};if original.turn.is_some(){return Ok(false)}if epoch<=original.last_epoch{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original Host wake turn epoch cannot replay returned authority"))}
  original.last_epoch=epoch;original.turn=Some(OriginalTurn{epoch,original:grant,remaining:grant,spent:Default::default(),checkout:None});Ok(true)
 }
 /// 📤️ Returns the actual complete receipt only when every original checkout has returned.
 pub fn take_turn(&self,epoch:u64)->Result<Option<GuestRelayWakeTurnReceipt>,ValueError>{
  let Ok(mut original)=self.original.try_lock()else{return Ok(None)};let Some(turn)=original.turn.as_ref()else{return Ok(None)};
  if turn.epoch!=epoch{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original Host wake turn epoch mismatch"))}
  if turn.checkout.is_some(){return Ok(None)}
  let turn=original.turn.take().unwrap();Ok(Some(GuestRelayWakeTurnReceipt{epoch:turn.epoch,original:turn.original,spent:turn.spent,remaining:turn.remaining}))
 }
}
impl GuestRelayWakeReceiptReceiver for GuestRelayWakeReceiptLedger{
 fn checkout(&self,slot:usize,generation:u64)->Result<Option<GuestRelayWakeTurnGrant>,ValueError>{
  let Ok(mut original)=self.original.try_lock()else{return Ok(None)};let Some(turn)=original.turn.as_mut()else{return Ok(None)};
  if turn.checkout.is_some()||turn.remaining.maximum_items==0{return Ok(None)}
  let grant=turn.remaining;turn.checkout=Some((slot,generation,grant));Ok(Some(GuestRelayWakeTurnGrant{epoch:turn.epoch,grant}))
 }
 fn collect(&self,receipt:&GuestRelayWakeReceipt)->Result<bool,ValueError>{
  let Ok(mut original)=self.original.try_lock()else{return Ok(false)};let Some(turn)=original.turn.as_mut()else{return Ok(false)};
  let Some((slot,generation,quoted))=turn.checkout else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original Host wake receipt has no checked-out turn"))};
  let grant=receipt.grant;let progress=receipt.progress;
  if receipt.epoch!=turn.epoch||receipt.slot!=slot||receipt.generation!=generation||grant.maximum_items>quoted.maximum_items||grant.maximum_copy_bytes>quoted.maximum_copy_bytes||grant.maximum_capacity_bytes>quoted.maximum_capacity_bytes||grant.maximum_release_bytes>quoted.maximum_release_bytes||grant.maximum_depth>quoted.maximum_depth||!progress.fits(grant){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original Host wake receipt exceeds its exact checked-out authority"))}
  let Some(items)=turn.spent.copied_items.checked_add(progress.copied_items)else{return Err(overflow())};let Some(copy)=turn.spent.copied_bytes.checked_add(progress.copied_bytes)else{return Err(overflow())};let Some(capacity)=turn.spent.retained_capacity_bytes.checked_add(progress.retained_capacity_bytes)else{return Err(overflow())};let Some(release)=turn.spent.released_bytes.checked_add(progress.released_bytes)else{return Err(overflow())};
  turn.remaining.maximum_items-=progress.copied_items;turn.remaining.maximum_copy_bytes-=progress.copied_bytes;turn.remaining.maximum_capacity_bytes-=progress.retained_capacity_bytes;turn.remaining.maximum_release_bytes-=progress.released_bytes;turn.spent=RetainedCloneProgress{copied_items:items,copied_bytes:copy,retained_capacity_bytes:capacity,released_bytes:release};turn.checkout=None;Ok(true)
 }
}
fn overflow()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original Host wake cumulative receipt overflow")}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
