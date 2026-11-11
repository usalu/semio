//! 🎟️ Original gesture issuance keeps the runtime and captured alias behind a strong-only authority.
use semio_plugin_decl_views::gesture_contract::GestureCapture;
use semio_plugin_decl_views::gesture_contract::{GestureSlot, GestureSlotDecision};
use semio_framework_value::{RetainedAllocationEffect, ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value::shared_issuance::{SealedShared, SealedSharedBirth, SharedIssuer};

#[doc(hidden)]
#[derive(semio_framework_value::RetireOwned)]
pub struct GestureOperation<M:Send+'static>{pub operation:u64,pub slot:SealedShared<GestureSlot<M>>,pub decision:Option<GestureSlotDecision<M>>,pub settled:bool}
impl<M:Send+'static> GestureOperation<M>{pub fn request_cancellation(&mut self){self.settled=true;}}

/// 🪙️ Funds one live gesture issuance (empty birth, publication and strong-only handoff) as the sum of its two host limits.
#[doc(hidden)]
pub fn gesture_issuance_grant(birth:RetainedCloneGrant,handoff:RetainedCloneGrant)->RetainedCloneGrant{
 RetainedCloneGrant{maximum_items:birth.maximum_items+handoff.maximum_items,maximum_copy_bytes:birth.maximum_copy_bytes+handoff.maximum_copy_bytes,maximum_capacity_bytes:birth.maximum_capacity_bytes+handoff.maximum_capacity_bytes,maximum_release_bytes:birth.maximum_release_bytes+handoff.maximum_release_bytes,maximum_depth:birth.maximum_depth.max(handoff.maximum_depth)}
}

/// 🎟️ Reserves the actual empty sealed backing of one gesture slot inside the caller's original effect.
#[doc(hidden)]
pub fn reserve_live<M:Send+'static>(available:bool,effect:&mut RetainedAllocationEffect<'_>)->Result<(SealedSharedBirth<GestureSlot<M>>,RetainedCloneProgress),ValueError>{
 if !available{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"all original gesture operation slots remain occupied"));}
 let mut output=None;let progress=SealedShared::<GestureSlot<M>>::prepare_birth(&mut output,effect).result?.progress();
 output.map(|birth|(birth,progress)).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original gesture birth is not funded"))
}
/// 🎟️ Publishes the slot into its reserved backing and duplicates the strong-only capture inside the same original effect.
#[doc(hidden)]
pub fn issue_live<M:RetireOwned+Send+'static>(mut birth:SealedSharedBirth<GestureSlot<M>>,slot:GestureSlot<M>,operation:u64,effect:&mut RetainedAllocationEffect<'_>)->Result<(GestureOperation<M>,GestureCapture<M>,RetainedCloneProgress),ValueError>{
 let mut input=Some(slot);let mut sealed=None;
 let published=birth.publish(&mut input,SharedIssuer::owned(),&mut sealed,effect).result?.progress();
 let slot=sealed.ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original gesture publication is not funded"))?;
 let(alias,handoff)=slot.try_duplicate(effect).result?;
 let progress=published.checked_add(handoff)?;
 Ok((GestureOperation{operation,slot,decision:None,settled:false},GestureCapture::Issued(alias),progress))
}

#[doc(hidden)]
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
pub mod tests;
