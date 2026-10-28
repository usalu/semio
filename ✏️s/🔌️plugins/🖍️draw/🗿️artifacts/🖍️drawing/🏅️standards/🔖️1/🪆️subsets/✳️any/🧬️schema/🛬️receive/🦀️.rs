//! 🛬️ Genuine Draw receiving declares its only non-scalar missing-field default without allocation.
use semio_framework_value::{RetirementDemand,ValueError,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,retained_clone_grant_admits}};
use semio_framework_value_native_owned::native_decoding::{owned::{NativeDefault,tree::{fixed,receipt,invalid}}};
impl NativeDefault for crate::DrawingAttributes{fn native_default_demand()->RetirementDemand{fixed(2*std::mem::size_of::<Self>())}fn native_default(g:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),ValueError>{let d=Self::native_default_demand();if !retained_clone_grant_admits(g,d){return Err(invalid())}Ok((Self{fill_rule:crate::FillRule::Evenodd,fill:None,stroke:None},receipt(d)))}}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
