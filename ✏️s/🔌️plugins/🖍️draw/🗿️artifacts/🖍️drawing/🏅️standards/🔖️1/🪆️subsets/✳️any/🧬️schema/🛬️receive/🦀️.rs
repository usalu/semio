//! 🛬️ Genuine Draw receiving declares its only non-scalar missing-field default without allocation.
use semio_framework_value::{RetirementDemand,RetainedWorkOutcome,retained_clone::{RetainedCloneGrant,RetainedCloneStep,retained_clone_grant_admits}};
use semio_framework_value_native_owned::native_decoding::owned::{NativeDefault,NativeDefaultValue,NativeDefaultValueCursor,tree::{fixed,receipt}};
impl NativeDefaultValue for crate::DrawingAttributes{fn default_demand()->RetirementDemand{fixed(2*std::mem::size_of::<Self>())}fn admit_default(output:&mut Option<Self>,g:RetainedCloneGrant)->RetainedWorkOutcome{if output.is_some(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}let d=<Self as NativeDefaultValue>::default_demand();if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}*output=Some(Self{fill_rule:crate::FillRule::Evenodd,fill:None,stroke:None});RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(receipt(d))),work:1}}}
impl NativeDefault for crate::DrawingAttributes{type Cursor=NativeDefaultValueCursor<Self>;fn default_cursor()->Self::Cursor{NativeDefaultValueCursor::new()}}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
