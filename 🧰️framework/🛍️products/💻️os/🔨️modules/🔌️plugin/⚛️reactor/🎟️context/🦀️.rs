//! 🌉️ OS guest entries compose the defining General Actor context with original job and wire ownership; the vocabulary lives in the guest contract, the wasm export bindings here.
pub use semio_framework_plugin_guest_contract::original_actor_context::*;
use semio_framework_actor::RetainedTurnInput;
use semio_framework_job::{StepBudget, StepContext};
use semio_framework_value::{RetainedCloneProgress, ValueError, ValueRefusalKind};
use std::mem::ManuallyDrop;

use semio_framework_actor::retained_context::{OriginalActorAdmissionReply, OriginalActorCall, OriginalActorCloseReply};

#[cfg(all(target_arch="wasm32",target_env="p2"))]
#[path="🌉️call/🎭️exports/🦀️.rs"]
pub mod exports;
