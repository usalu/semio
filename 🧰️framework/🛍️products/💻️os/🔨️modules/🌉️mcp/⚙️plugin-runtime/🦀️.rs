//! ⚙️ Plugin component activation and the plugin artifact channel.

extern crate semio_framework_os_kernel as store;

#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;

#[cfg(not(target_arch = "wasm32"))]
#[path = "../🏠️workspace/🧩️artifact-app-frames/🦀️.rs"]
pub mod artifact_app_frames;

#[path = "../🏠️workspace/🧊️compiled-components/🦀️.rs"]
pub mod compiled_components;

#[path = "../🏠️workspace/🗂️guest-document-codec/🦀️.rs"]
pub mod guest_document_codec;

#[cfg(test)]
#[path = "../🏠️workspace/🪶️sqlite/🧪️tests/guest-input-slot.rs"]
pub mod guest_document_codec_guest_sqlite_original;

#[cfg(not(target_arch = "wasm32"))]
#[path = "../🏠️workspace/📨️plugin-artifact-channel/🦀️.rs"]
pub mod plugin_artifact_channel;

#[path = "../🏠️workspace/🔖️plugin-binding-and-activation/🦀️.rs"]
pub mod plugin_binding_and_activation;
