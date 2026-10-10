//! 🎟️ Genuine embedding configuration declares independent full policies and finite original operation currency.
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,ValueError,ValueRefusalKind};
use semio_framework_value_derive::{FromValue,ToValue};
use super::super::GuestRelayWakeReceiptReceiver;
#[derive(Clone,Copy,Debug,PartialEq,Eq,serde::Serialize,serde::Deserialize,FromValue,ToValue)]
#[serde(deny_unknown_fields)]
pub struct GuestRelayDriverPolicy{pub admission:RetainedCloneGrant,pub drive:RetainedCloneGrant,pub wake:RetainedCloneGrant,pub operation:RetainedCloneGrant}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct GuestRelayDriverTurnGrant{pub epoch:u64,pub grant:RetainedCloneGrant}
/// 🧮️ Only acknowledged original effects debit this finite operation treasury; no automatic currency renewal exists.
pub struct GuestRelayDriverTreasury{policy:GuestRelayDriverPolicy,original_remaining:RetainedCloneGrant,next_epoch:u64,held:Option<GuestRelayDriverTurnGrant>}
impl GuestRelayDriverTreasury{
 pub fn new(policy:GuestRelayDriverPolicy)->Self{Self{policy,original_remaining:policy.operation,next_epoch:1,held:None}}
 pub fn policy(&self)->GuestRelayDriverPolicy{self.policy}
 pub fn original_remaining(&self)->RetainedCloneGrant{self.original_remaining}
 pub fn terminal_is_empty(&self)->bool{self.held.is_none()}
 pub fn record_admission(&mut self,progress:RetainedCloneProgress)->Result<(),ValueError>{if self.held.is_some(){return Err(refusal("original driver admission cannot overlap its held turn"))}if !progress.fits(self.policy.admission){return Err(refusal("original driver admission receipt exceeds independently supplied admission policy"))}self.debit(progress)}
 pub fn begin_turn(&mut self)->Result<Option<GuestRelayDriverTurnGrant>,ValueError>{self.begin_policy_turn(self.policy.wake)}
 pub fn begin_drive_turn(&mut self)->Result<Option<GuestRelayDriverTurnGrant>,ValueError>{self.begin_policy_turn(self.policy.drive)}
 fn begin_policy_turn(&mut self,policy:RetainedCloneGrant)->Result<Option<GuestRelayDriverTurnGrant>,ValueError>{if self.held.is_some(){return Ok(None)}let grant=intersect(policy,self.original_remaining);if grant.maximum_items==0{return Ok(None)}let epoch=self.next_epoch;self.next_epoch=self.next_epoch.checked_add(1).ok_or_else(||refusal("original driver epoch overflow"))?;let turn=GuestRelayDriverTurnGrant{epoch,grant};self.held=Some(turn);Ok(Some(turn))}
 pub fn return_turn(&mut self,turn:GuestRelayDriverTurnGrant,progress:RetainedCloneProgress)->Result<(),ValueError>{if self.held!=Some(turn)||!progress.fits(turn.grant){return Err(refusal("original driver returned receipt does not match its held authority"))}self.debit(progress)?;self.held=None;Ok(())}
 fn debit(&mut self,progress:RetainedCloneProgress)->Result<(),ValueError>{if !progress.fits(self.original_remaining){return Err(refusal("original driver receipt exceeds finite operation currency"))}self.original_remaining.maximum_items-=progress.copied_items;self.original_remaining.maximum_copy_bytes-=progress.copied_bytes;self.original_remaining.maximum_capacity_bytes-=progress.retained_capacity_bytes;self.original_remaining.maximum_release_bytes-=progress.released_bytes;Ok(())}
}
impl Drop for GuestRelayDriverTreasury{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original driver treasury reached Drop with an unacknowledged turn");}}
pub(super) fn intersect(a:RetainedCloneGrant,b:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:a.maximum_items.min(b.maximum_items),maximum_copy_bytes:a.maximum_copy_bytes.min(b.maximum_copy_bytes),maximum_capacity_bytes:a.maximum_capacity_bytes.min(b.maximum_capacity_bytes),maximum_release_bytes:a.maximum_release_bytes.min(b.maximum_release_bytes),maximum_depth:a.maximum_depth.min(b.maximum_depth)}}
fn refusal(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}

/// 🏦️ Supplies wake turns from one genuine finite driver treasury and acknowledges their exact original return.
pub struct GuestRelayWakeDriver{original:semio_framework_value::retirement::controlled::RetainedOwnerGate<WakeDriverState>}
enum DriverCheckout{Wake{slot:usize,generation:u64,turn:GuestRelayDriverTurnGrant},Actor{input:semio_framework_actor::RetainedTurnInput,turn:GuestRelayDriverTurnGrant}}
struct WakeDriverState{treasury:GuestRelayDriverTreasury,checkout:Option<DriverCheckout>}
impl GuestRelayWakeDriver{
 pub fn new(policy:GuestRelayDriverPolicy)->Self{Self{original:semio_framework_value::retirement::controlled::RetainedOwnerGate::new(WakeDriverState{treasury:GuestRelayDriverTreasury::new(policy),checkout:None})}}
 pub fn original_remaining(&self)->Result<RetainedCloneGrant,ValueError>{let original=self.original.try_lock().map_err(|_|refusal("original driver treasury is checked out"))?;Ok(original.treasury.original_remaining())}
 pub fn record_admission(&self,progress:RetainedCloneProgress)->Result<(),ValueError>{let mut original=self.original.try_lock().map_err(|_|refusal("original driver treasury is checked out"))?;original.treasury.record_admission(progress)}
 pub fn begin_actor_turn(&self,operation:semio_framework_job::OperationId,generation:semio_framework_job::Generation)->Result<Option<semio_framework_actor::RetainedTurnInput>,ValueError>{
  if operation.0==0||generation.0==0{return Err(refusal("original actor retained turn requires its supplied operation and generation"))}let Ok(mut original)=self.original.try_lock()else{return Ok(None)};if original.checkout.is_some(){return Ok(None)}let Some(turn)=original.treasury.begin_drive_turn()?else{return Ok(None)};let input=semio_framework_actor::RetainedTurnInput{operation:operation.0,generation:generation.0,epoch:turn.epoch,grant:turn.grant};original.checkout=Some(DriverCheckout::Actor{input,turn});Ok(Some(input))
 }
 pub fn return_actor_turn(&self,receipt:semio_framework_actor::RetainedTurnReceipt)->Result<bool,ValueError>{
  let Ok(mut original)=self.original.try_lock()else{return Ok(false)};let Some(DriverCheckout::Actor{input,turn})=original.checkout.as_ref()else{return Err(refusal("original actor retained receipt has no matching treasury checkout"))};let input=*input;let turn=*turn;receipt.validate_for(input)?;original.treasury.return_turn(turn,receipt.spent)?;original.checkout=None;Ok(true)
 }

}
impl GuestRelayWakeReceiptReceiver for GuestRelayWakeDriver{
 fn checkout(&self,slot:usize,generation:u64)->Result<Option<super::GuestRelayWakeTurnGrant>,ValueError>{let Ok(mut original)=self.original.try_lock()else{return Ok(None)};if original.checkout.is_some(){return Ok(None)}let Some(turn)=original.treasury.begin_turn()?else{return Ok(None)};original.checkout=Some(DriverCheckout::Wake{slot,generation,turn});Ok(Some(super::GuestRelayWakeTurnGrant{epoch:turn.epoch,grant:turn.grant}))}
 fn collect(&self,receipt:&super::super::GuestRelayWakeReceipt)->Result<bool,ValueError>{
  let Ok(mut original)=self.original.try_lock()else{return Ok(false)};let Some(DriverCheckout::Wake{slot,generation,turn})=original.checkout.as_ref()else{return Err(refusal("original production wake receipt has no treasury checkout"))};let slot=*slot;let generation=*generation;let turn=*turn;let grant=receipt.grant;
  if receipt.slot!=slot||receipt.generation!=generation||receipt.epoch!=turn.epoch||intersect(grant,turn.grant)!=grant||!receipt.progress.fits(grant){return Err(refusal("original production wake receipt does not match its held treasury authority"))}
  original.treasury.return_turn(turn,receipt.progress)?;original.checkout=None;Ok(true)
 }
}
