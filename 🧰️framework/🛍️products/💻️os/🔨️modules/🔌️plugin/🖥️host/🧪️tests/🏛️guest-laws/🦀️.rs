//! 🏛️ Guest runtime laws: the owned and Wasmtime runtime, patch ownership, snapshot refusal and shared engine laws over the real guest runtime.

#[cfg(test)]
#[path = "../🔬️owned-instance-open/🦀️.rs"]
mod owned_instance_open_tests;

#[cfg(test)]
#[path = "../🔬️wasmtime-runtime/🦀️.rs"]
mod wasmtime_runtime_tests;

#[cfg(test)]
#[path = "../../📥️ui-patch/🧪️tests/🧪️component/🦀️.rs"]
mod ui_patch_component_tests;

#[cfg(test)]
#[path = "../🪶️sqlite/⚠️refusal/🦀️.rs"]
mod sqlite_refusal_tests;

#[cfg(test)]
#[path = "../🔬️shared-wasmtime-engine/🦀️.rs"]
mod shared_wasmtime_engine_tests;
