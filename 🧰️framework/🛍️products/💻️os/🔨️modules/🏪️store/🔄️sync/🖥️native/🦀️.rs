//! 🖥️ The native Store sync actor and the folder event log storage.
//!
//! 🧭️ Native only. Depends on `os-store-sync` and the engine; nothing below depends on it: `ArtifactHost::open` owns the native actor spawn here and the shared registry lives in `ArtifactHostCore`.

#[cfg(not(target_arch = "wasm32"))]
#[path = "🎭️actor/🦀️.rs"]
pub mod native_actor;
#[path = "📥️lifted/🔄️sync/🦀️.rs"]
pub mod lifted_sync;
#[cfg(not(target_arch = "wasm32"))]
pub use native_actor::{ArtifactActorRunnerHandle, ArtifactActorRunnerTicket, ArtifactActorTerminalJob, ArtifactActorTerminalReason, ArtifactActorTerminalState, ArtifactChannels, ArtifactHost};
pub use lifted_sync::{FOLDER_EVENT_MAGIC, FolderEventLogStorage, FolderTextStorage};
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod unit_tests;
