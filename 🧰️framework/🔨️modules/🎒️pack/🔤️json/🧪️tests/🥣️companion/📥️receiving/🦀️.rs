//! 📥️ Original JSON receiving laws over the public receiving surface.
pub use semio_framework_pack_json::original_receiving::*;
pub use semio_framework_pack_json::*;
use semio_framework_value::{DslValue, RetainedWorkOutcome, RetirementDemand, ValueError, ValueRefusalKind, ArtifactOwnedValueRetirementFactory, FactoryAuthority, retained_clone::{RetainedCloneSource, RetainedCloneSourceAdmission, RetainedOwnedProjection, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetainedCloneClose, retained_clone_grant_admits}};
fn invalid(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,reason)}
use semio_framework_value_native_owned::{native_decoding::NativeDecodeControl, native_decoding::NativeReceive, native_decoding::OriginalReceivingPrimary, native_decoding::owned::NativeReceivingOriginal};
use std::{mem::{ManuallyDrop,size_of},sync::Arc};
#[path = "../../../📥️decode/🧳️receiving/🧪️tests/🦀️.rs"]
mod tests;
