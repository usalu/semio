//! 🧠️ Repository-owned WebAssembly core interpreter for the Semio plugin sandbox.
//!
//! The interpreter is deliberately execution-engine neutral: component canonical-ABI lowering is
//! an adapter above this module, while every guest core instruction is decoded, fuelled, cancelled,
//! and checkpointed here. Browser hosts keep using the browser's WebAssembly engine.

use std::collections::{BTreeMap, BTreeSet};

/// 🪪️ The interpreter's authored source: the half of the owned engine identity that names every instruction a guest executes.
pub const INTERPRETER_SOURCE: &str = concat!(
    include_str!("🦀️.rs"),
    include_str!("🧱️core-types-and-module-model/🦀️.rs"),
    include_str!("🧩️component-artifact/🦀️.rs"),
    include_str!("🔌️owned-semio-boundary/🦀️.rs"),
    include_str!("🔎️binary-decoder-and-module-parser/🦀️.rs"),
    include_str!("🧭️instruction-shape/🦀️.rs"),
    include_str!("🏛️instance-state/🦀️.rs"),
    include_str!("⚙️instruction-execution/🦀️.rs"),
    include_str!("💾️memory-and-bulk-execution/🦀️.rs"),
    include_str!("🧮️numeric-execution/🦀️.rs"),
    include_str!("🔢️numeric-semantics/🦀️.rs"),
    include_str!("📸️checkpoint-codec/🦀️.rs"),
);

/// 🧮️ The heap witness the crate's allocation laws read through `semio_framework_trace::observe_heap_allocations_on_this_thread`.
#[cfg(test)]
#[global_allocator]
static INTERPRETER_HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

#[path = "🧱️core-types-and-module-model/🦀️.rs"]
mod core_types_and_module_model;
pub use core_types_and_module_model::{CoreError, CoreModule, CoreStepOutcome, ExportKind, FunctionType, HostCall, MemoryLimits, StepControl, TableLimits, Value, ValueType};
use core_types_and_module_model::{CHECKPOINT_MAGIC, CHECKPOINT_VERSION, ConstExpr, ControlBounds, DataDecl, ElementDecl, FunctionDecl, GlobalDecl, SegmentMode, WASM_PAGE_BYTES};

#[path = "🧩️component-artifact/🦀️.rs"]
mod component_artifact;
pub use component_artifact::{ComponentArtifact, ComponentLimits, ComponentSection, ComponentSectionKind, WasmArtifact, WasmArtifactKind};

#[path = "🔌️owned-semio-boundary/🦀️.rs"]
mod owned_semio_boundary;
pub use owned_semio_boundary::{OwnedSemioArtifact, OwnedSemioExport, OwnedSemioInstance, SEMIO_OWNED_ACKNOWLEDGE_JOB_OUTPUT_CHUNK_EXPORT, SEMIO_OWNED_ACKNOWLEDGE_JOB_OUTPUT_EXPORT, SEMIO_OWNED_ADMIT_ACTOR_CONTEXT_EXPORT, SEMIO_OWNED_ALLOC_EXPORT, SEMIO_OWNED_APPLY_OPS_EXPORT, SEMIO_OWNED_BORROW_JOB_OUTPUT_EXPORT, SEMIO_OWNED_CANCEL_JOB_EXPORT, SEMIO_OWNED_CHANNEL_VERSION_EXPORT, SEMIO_OWNED_CHECKPOINT_EXPORT, SEMIO_OWNED_CLOSE_ACTOR_CONTEXT_EXPORT, SEMIO_OWNED_CLOSE_JOBS_EXPORT, SEMIO_OWNED_DEALLOC_EXPORT, SEMIO_OWNED_DESCRIBE_EXPORT, SEMIO_OWNED_GENESIS_EXPORT, SEMIO_OWNED_PACK_SCHEMA_HASH_EXPORT, SEMIO_OWNED_POLL_EXPORT, SEMIO_OWNED_PRINT_MIRROR_EXPORT, SEMIO_OWNED_READ_JOB_OUTPUT_EXPORT, SEMIO_OWNED_REPLAY_ENVELOPES_EXPORT, SEMIO_OWNED_RESTORE_EXPORT, SEMIO_OWNED_SQLITE_CLOSE_EXPORT, SEMIO_OWNED_SQLITE_EXPORT_EXPORT, SEMIO_OWNED_SQLITE_IMPORT_EXPORT, SEMIO_OWNED_SQLITE_RETIREMENT_EXPORT, SEMIO_OWNED_SQLITE_SCHEMA_EXPORT, SEMIO_OWNED_SQLITE_TAKE_FILE_EXPORT, SEMIO_OWNED_SQLITE_TAKE_PAYLOAD_EXPORT, SEMIO_OWNED_START_JOB_EXPORT, SEMIO_OWNED_STEP_JOB_EXPORT};

#[path = "🔎️binary-decoder-and-module-parser/🦀️.rs"]
mod binary_decoder_and_module_parser;
use binary_decoder_and_module_parser::{Decoder, ModuleParser};

#[path = "🧭️instruction-shape/🦀️.rs"]
mod instruction_shape;
use instruction_shape::{analyze_controls, decode_block_type};

#[path = "🏛️instance-state/🦀️.rs"]
mod instance_state;
pub use instance_state::CoreInstance;
use instance_state::{ControlFrame, ControlKind, Frame, GlobalState, InstructionProgress, Machine, MemoryState, PendingHost, TableState};

#[path = "⚙️instruction-execution/🦀️.rs"]
mod instruction_execution;
use instruction_execution::{check_values, expect_type, pop};

#[path = "💾️memory-and-bulk-execution/🦀️.rs"]
mod memory_and_bulk_execution;

#[path = "🧮️numeric-execution/🦀️.rs"]
mod numeric_execution;
use numeric_execution::{execute_numeric, map_top};

#[path = "🔢️numeric-semantics/🦀️.rs"]
mod numeric_semantics;
use numeric_semantics::{canonical_f32, canonical_f64, round_ties_even_f32, round_ties_even_f64, saturating_i32_from_f32, saturating_i32_from_f64, saturating_i64_from_f32, saturating_i64_from_f64, trunc_i32_from_f32, trunc_i32_from_f64, trunc_i64_from_f32, trunc_i64_from_f64, wasm_max_f32, wasm_max_f64, wasm_min_f32, wasm_min_f64};

#[path = "📸️checkpoint-codec/🦀️.rs"]
mod checkpoint_codec;

#[path="🏗️instance/🦀️.rs"]
mod original_instance_opening;
pub use original_instance_opening::{CoreInstanceOpening,CoreOriginalInvocationCall,CoreOriginalInvocationStatus};

//#region 💾️CheckpointCodec

#[cfg(test)]
#[path="🚪️io/📤️checkpoint/🧪️tests/👓️borrowed/🦀️.rs"]
mod borrowed_actor_checkpoint_tests;

//#endregion 💾️CheckpointCodec

//#region 🧰️Utilities

fn pages_to_bytes(pages: u64) -> Result<usize, CoreError> {
    let bytes = pages.checked_mul(WASM_PAGE_BYTES as u64).ok_or_else(|| CoreError::Validation("memory page count overflows bytes".into()))?;
    usize::try_from(bytes).map_err(|_| CoreError::Validation("memory size exceeds host usize".into()))
}

fn checked_end(start: usize, length: usize, bound: usize, subject: &str) -> Result<usize, CoreError> {
    let end = start as u128 + length as u128;
    if end > bound as u128 {
        Err(CoreError::Trap(format!("{subject} is out of bounds: start={start} length={length} end={end} bound={bound}")))
    } else {
        Ok(end as usize)
    }
}

fn stable_fingerprint(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

//#endregion 🧰️Utilities

//#region 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#endregion 🧪️Tests

#[cfg(any(test, feature = "testkit"))]
pub mod testkit {
    pub use super::{core_types_and_module_model::*, component_artifact::*, owned_semio_boundary::*, binary_decoder_and_module_parser::*, instruction_shape::*, instance_state::*, instruction_execution::*, memory_and_bulk_execution::*, numeric_execution::*, numeric_semantics::*, checkpoint_codec::*};
    #[cfg(test)]
    pub use super::tests::*;
}
