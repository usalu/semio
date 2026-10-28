//! 📥️ Retained typed Pack and SPR loading for one exact window-config partition.

use super::{ WindowConfigOwner, WindowConfigPack, WindowConfigPartition, WindowRegistry};
use crate::{protocol, store};
use std::any::Any;
use std::mem::ManuallyDrop;
use semio_framework_value::{ValueError, ValueRefusalKind, RetirementDemand, retirement::{RetireOwned, RetirementCursor, controlled::ControlledRetirement}, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use super::PluginLifecycleStep;

#[path = "🧩️pack-load-contract-and-value-frames/🦀️.rs"]
mod pack_load_contract_and_value_frames;
pub use pack_load_contract_and_value_frames::{
    WindowConfigPackLoad,
    WindowConfigPackLoadDiagnostic, WindowConfigPackLoadPhase, WindowConfigPackLoadProgress,
    WindowConfigPackLoadStep,
};
pub use pack_load_contract_and_value_frames::ErasedWindowConfigPackLoad;

#[path = "📦️typed-pack-load/🦀️.rs"]
mod typed_pack_load;
#[cfg(test)]
use typed_pack_load::TypedWindowConfigPackLoad;
pub(super) use typed_pack_load::{
    admit_typed_window_config_pack_load, commit_typed_window_config_pack_load,cancel_typed_window_config_pack_load,typed_window_config_pack_load_commit_demands,
};
#[path="🎞️command/🦀️.rs"]
mod command;
pub use command::WindowConfigCommandLoad;
