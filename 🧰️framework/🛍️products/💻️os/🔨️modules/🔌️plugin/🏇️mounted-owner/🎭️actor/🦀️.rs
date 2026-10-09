//! 🎭️ Captures bounded original actor text only after its full publication grant admits real custody.
use semio_framework_value::{SharedUtf8,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress},retirement::{controlled::ControlledRetirement,shared::shared_retirement_allocation_bytes}};
use std::mem::size_of;
/// 📏️ The shared Store protocol admits at most this original identity extent.
pub const MOUNTED_ACTOR_TEXT_BYTES:usize=256;
/// 🧮️ Quotes the real text, Arc frame and original inline custody without constructing them.
pub fn actor_capture_demand(source:&str)->Result<RetirementDemand,ValueError>{
 if source.is_empty()||source.len()>MOUNTED_ACTOR_TEXT_BYTES{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"mounted actor exceeds its fixed original identity capacity"));}
 Ok(RetirementDemand{copy_bytes:source.len()+size_of::<String>()+size_of::<SharedUtf8>()+size_of::<ControlledRetirement<SharedUtf8>>(),capacity_bytes:source.len()+shared_retirement_allocation_bytes::<String>(),release_bytes:0,depth:1})
}
/// 🎟️ Keeps denied source text untouched and retains an accepted original capture until transfer or paid close.
pub fn admit_actor_capture(slot:&mut Option<ControlledRetirement<SharedUtf8>>,source:&str,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{
 if slot.is_some(){return Ok(None);}
 let demand=actor_capture_demand(source)?;
 if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(None);}
 let mut text=String::with_capacity(source.len());text.push_str(source);
 let(owner,receipt)=SharedUtf8::admit(text,grant).unwrap_or_else(|_|unreachable!("original actor quote admitted the exact SharedUtf8 constructor"));
 *slot=Some(ControlledRetirement::new(owner).unwrap_or_else(|_|unreachable!("SharedUtf8 has its original controlled retirement issuer")));
 let receipt=RetainedCloneProgress{copied_items:receipt.copied_items,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,released_bytes:receipt.released_bytes};
 if !receipt.fits(grant){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"mounted actor capture exceeded its original full caller grant").with_retained_progress(receipt));}
 Ok(Some(receipt))
}
