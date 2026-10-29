//! 🫧️ Native Energy singleton, root, network, output and action pipelines compose the actual original typed sources of the native mutation base.
extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;
extern crate semio_s_artifact_energy_model_diff as diff;
extern crate semio_s_plugin_energy_engine_model as model;
use semio_s_artifact_energy_model_native_mutations::{leaf,row,scalar};

#[cfg(test)]#[global_allocator]
static ORIGINAL_NATIVE_SINGLETONS_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;

#[cfg(test)]extern crate semio_s_plugin_energy_engine_calendar as calendar;

#[path="🔹️single/🦀️.rs"]pub mod single;

#[path="🔗️root/🦀️.rs"]pub mod root;

#[path="📊️outputs/🦀️.rs"]pub mod outputs;

#[path="🫧️network/🦀️.rs"]pub mod network;

#[path="🎯️action/🦀️.rs"]pub mod action;
