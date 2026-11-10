//! 📦️ Public collection storage laws execute against the actual native owned library.
pub use semio_framework_value_native_owned::*;
pub use semio_framework_value_native_owned::owned::tree::*;
pub use semio_framework_value_native_owned::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep};
#[path="../../../🌳️value/📦️storage/🧪️tests/🦀️.rs"]mod laws;

pub use semio_framework_value_testing_admission::{EffectBank,Wallet,session_nested,session_open};
#[path="../../../../../🧪️testing/📥️admission/receiver-driver.rs"]mod driver;
pub fn receive<T,E>(bank:&EffectBank,g:RetainedCloneGrant,operation:impl FnOnce(&mut NativeDecodeControl<'_>)->RetainedWorkOutcome<T,E>)->RetainedWorkOutcome<T,E>{match session_nested(bank,g,|control,_|RetainedWorkPoll::Complete(operation(control))){RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{..}=>panic!("synchronous receive stayed pending")}}
