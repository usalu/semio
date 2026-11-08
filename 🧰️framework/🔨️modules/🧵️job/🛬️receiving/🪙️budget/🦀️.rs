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
pub struct OriginalJobConstructionRecipient<'a>{original:RetainedCloneGrant,start:RetainedCloneProgress,spent:&'a mut RetainedCloneProgress,receiving_spent:&'a mut RetainedCloneProgress,submitted:RetainedCloneProgress,recovery:Option<RetainedCloneGrant>,construction_open:bool}
/// 🫴️ The resident producer; each Job turn lends its exact ledger as the context.
pub struct OriginalJobConstructionDriver;
impl semio_framework_value_native_owned::native_decoding::OriginalConstructionRecipient for OriginalJobConstructionDriver{
 type Context<'turn,'source>=OriginalJobConstructionRecipient<'turn> where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&OriginalJobConstructionRecipient<'turn>)->Result<(),ValueError> where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&OriginalJobConstructionRecipient<'turn>)->Option<semio_framework_value_native_owned::native_decoding::OriginalSourceTurnIdentity> where 'source:'turn{None}
 fn grant<'reply,'turn,'source>(&mut self,c:&mut OriginalJobConstructionRecipient<'turn>,quote:semio_framework_value_native_owned::native_decoding::OriginalConstructionGrant<'reply>)->semio_framework_value_native_owned::native_decoding::OriginalConstructionGrantComplete<'reply> where 'source:'turn{let demand=quote.demand();quote.complete(c.grant_for(demand))}
 fn accept<'reply,'turn,'source>(&mut self,c:&mut OriginalJobConstructionRecipient<'turn>,receipt:semio_framework_value_native_owned::native_decoding::OriginalConstructionAccept<'reply>)->semio_framework_value_native_owned::native_decoding::OriginalConstructionAcceptComplete<'reply> where 'source:'turn{let(grant,progress)=(receipt.grant(),receipt.progress());receipt.complete(c.accept_receipt(grant,progress))}
 fn submitted<'turn,'source>(&self,c:&OriginalJobConstructionRecipient<'turn>)->RetainedCloneProgress where 'source:'turn{c.submitted}
}
impl OriginalJobConstructionRecipient<'_>{
 fn grant_for(&mut self,demand:semio_framework_value_retained_clone::RetirementDemand)->Result<RetainedCloneGrant,ValueError>{if !self.construction_open{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original pending construction must return before another admission"))}let remaining=retained_remaining(self.original,retained_delta(*self.spent,self.start));if !semio_framework_value_retained_clone::retained_clone::retained_clone_grant_admits(remaining,demand){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"native construction exceeds original remaining Job grant"));}Ok(remaining)}
 fn accept_receipt(&mut self,grant:RetainedCloneGrant,progress:RetainedCloneProgress)->Result<(),ValueError>{self.submitted=self.submitted.checked_add(progress)?;let admitted=progress.fits(grant)&&match self.recovery{Some(held)=>progress.fits(retained_remaining(held,retained_delta(*self.spent,self.start))),None=>retained_is_subset(grant,retained_remaining(self.original,retained_delta(*self.spent,self.start)))};*self.spent=self.spent.checked_add(progress)?;*self.receiving_spent=self.receiving_spent.checked_add(progress)?;if !admitted{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native construction changed its accepted original Job receipt").with_retained_progress(progress));}Ok(())}
}
impl<'a> OriginalJobConstructionRecipient<'a>{
 /// ♻️ Returns only an actual frozen receipt against its same original pooled hold.
 pub fn new_recovery(original:RetainedCloneGrant,held:RetainedCloneGrant,construction_open:bool,start:RetainedCloneProgress,spent:&'a mut RetainedCloneProgress,receiving_spent:&'a mut RetainedCloneProgress)->Self{Self{original,start,spent,receiving_spent,submitted:Default::default(),recovery:Some(held),construction_open}}

 /// 🎫️ Borrows the initiating Job's exact physical ledger and its nested receiving receipt.
 pub fn new(original:RetainedCloneGrant,start:RetainedCloneProgress,spent:&'a mut RetainedCloneProgress,receiving_spent:&'a mut RetainedCloneProgress)->Self{Self{original,start,spent,receiving_spent,submitted:Default::default(),recovery:None,construction_open:true}}
}
