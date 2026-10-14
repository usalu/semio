//! 🧠️ `semio-framework-plugin-interpreter`: the repository-owned WebAssembly interpreter ([`interpreter`]) and the guest
//! runtime it backs ([`OwnedRuntime`]), with the vocabulary every guest runtime shares. The plugin host links a JIT beside
//! it; a build tool that only describes or verifies components links nothing else.
//! @see https://webassembly.github.io/spec/core/
#[path = "../../🦀️.rs"]
pub mod interpreter;
#[path = "../../../🚪️io/🛂️authority/🦀️.rs"]
pub mod operation_authority;
#[path = "../../../⚛️reactor/🎟️context/🦀️.rs"]
pub mod original_actor_context;
#[path = "../../../🧬️schema/🪶️sqlite/🦀️.rs"]
pub mod sqlite_wire;
#[path = "../../🏃️runtime/🦀️.rs"]
mod runtime;
/// 🛂️ The neutral test caller admission the plugin host's runtime laws share; this crate's laws drive part of it.
#[cfg(test)]
#[allow(dead_code)]
#[path = "../../../🖥️host/🧪️tests/🛂️receiving/🦀️.rs"]
pub(crate) mod test_native_authority;
/// 🧮️ The heap witness the crate's allocation laws read through `semio_framework_trace::observe_heap_allocations_on_this_thread`.
#[cfg(test)]
#[global_allocator]
static INTERPRETER_HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;
pub use runtime::{
    OriginalGuestJobCall,OwnedActorStartup,OwnedActorStartupCall,OwnedActorStartupStatus,
    decode_guest_fault_bytes, decode_owned_result, identity_turn_fault, operation, owned_engine_identity, validate_cold_page, GuestCallCancellation, GuestDocumentMirror, GuestDocumentPair, IoEntryRouteConflict, IoRouteConflict, JobBudget, JobStep, OwnedCompiledGuest, OwnedGuestInstance, OwnedInvocation, OwnedRuntime, OwnedSnapshotInput,
    OwnedSnapshotOperation, PluginHostError, TurnFault, GUEST_DIAGNOSTICS_CAPACITY_BYTES, GUEST_FAULT_MAXIMUM_BYTES, OWNED_STEP_FUEL,
};
