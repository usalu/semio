//! 🌱️ Native Energy row birth and deletion pipelines compose the actual original typed sources of the native mutation base.
extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;
extern crate semio_s_artifact_energy_model_diff as diff;
use semio_s_artifact_energy_model_native_mutations::{leaf,reference,row,scalar,owned,text};

#[cfg(test)]#[global_allocator]
static ORIGINAL_NATIVE_LIFECYCLE_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;

#[cfg(test)]extern crate semio_s_plugin_energy_engine_model as model;
#[cfg(test)]extern crate semio_s_plugin_energy_engine_schedule as schedule;

#[path="🌱️construction/🦀️.rs"]pub mod construction;

#[path="✂️deletion/🦀️.rs"]pub mod deletion;
