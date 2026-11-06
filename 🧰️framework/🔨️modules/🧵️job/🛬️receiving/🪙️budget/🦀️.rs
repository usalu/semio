//! 🪙️ Native Job receiving owns its physical budget algebra and original recipient.
use super::{RetainedCloneGrant,RetainedCloneProgress,RetirementDemand,ValueError,ValueRefusalKind};
/// 📊️ Computes the admitted suffix of the initiating Job receipt.
pub fn retained_delta(after:RetainedCloneProgress,before:RetainedCloneProgress)->RetainedCloneProgress{RetainedCloneProgress{copied_items:after.copied_items.saturating_sub(before.copied_items),copied_bytes:after.copied_bytes.saturating_sub(before.copied_bytes),retained_capacity_bytes:after.retained_capacity_bytes.saturating_sub(before.retained_capacity_bytes),released_bytes:after.released_bytes.saturating_sub(before.released_bytes)}}
/// 📐️ Returns only the initiating Job's remaining physical currencies.
pub fn retained_remaining(grant:RetainedCloneGrant,spent:RetainedCloneProgress)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:grant.maximum_items.saturating_sub(spent.copied_items),maximum_copy_bytes:grant.maximum_copy_bytes.saturating_sub(spent.copied_bytes),maximum_capacity_bytes:grant.maximum_capacity_bytes.saturating_sub(spent.retained_capacity_bytes),maximum_release_bytes:grant.maximum_release_bytes.saturating_sub(spent.released_bytes),..grant}}
/// 🎚️ Intersects two independently admitted physical grants.
pub fn retained_min(a:RetainedCloneGrant,b:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:a.maximum_items.min(b.maximum_items),maximum_copy_bytes:a.maximum_copy_bytes.min(b.maximum_copy_bytes),maximum_capacity_bytes:a.maximum_capacity_bytes.min(b.maximum_capacity_bytes),maximum_release_bytes:a.maximum_release_bytes.min(b.maximum_release_bytes),maximum_depth:a.maximum_depth.min(b.maximum_depth)}}
/// 🔎️ Checks that a physical child grant stays within its initiating caller.
pub fn retained_is_subset(a:RetainedCloneGrant,b:RetainedCloneGrant)->bool{retained_min(a,b)==a}
/// 🧳️ Retains a native receiving refusal without manufacturing an operation result.
pub fn native_call_context<T,E>(error:ValueError)->semio_framework_value_call::NativeCallFailure<T,E>{semio_framework_value_call::NativeCallFailure{output:None,operation:None,context:Some(error),receipt_refusal:None}}

/// 🫴️ The actual Job ledger receives every original physical effect once.
pub struct OriginalJobConstructionRecipient<'a>{original:RetainedCloneGrant,start:RetainedCloneProgress,spent:&'a mut RetainedCloneProgress,receiving_spent:&'a mut RetainedCloneProgress,submitted:RetainedCloneProgress}
impl semio_framework_value_native_owned::native_decoding::NativeDecodeConstructionRecipient for OriginalJobConstructionRecipient<'_>{
 fn submitted(&self)->RetainedCloneProgress{self.submitted}
 fn grant(&mut self,demand:semio_framework_value_retained_clone::RetirementDemand)->Result<RetainedCloneGrant,ValueError>{let remaining=retained_remaining(self.original,retained_delta(*self.spent,self.start));if !semio_framework_value_retained_clone::retained_clone::retained_clone_grant_admits(remaining,demand){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"native construction exceeds original remaining Job grant"));}Ok(remaining)}
 fn accept(&mut self,grant:RetainedCloneGrant,progress:RetainedCloneProgress)->Result<(),ValueError>{self.submitted=self.submitted.checked_add(progress)?;let admitted=progress.fits(grant)&&retained_is_subset(grant,retained_remaining(self.original,retained_delta(*self.spent,self.start)));*self.spent=self.spent.checked_add(progress)?;*self.receiving_spent=self.receiving_spent.checked_add(progress)?;if !admitted{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native construction changed its accepted original Job receipt").with_retained_progress(progress));}Ok(())}
}
impl<'a> OriginalJobConstructionRecipient<'a>{
 /// 🎫️ Borrows the initiating Job's exact physical ledger and its nested receiving receipt.
 pub fn new(original:RetainedCloneGrant,start:RetainedCloneProgress,spent:&'a mut RetainedCloneProgress,receiving_spent:&'a mut RetainedCloneProgress)->Self{Self{original,start,spent,receiving_spent,submitted:Default::default()}}
}
