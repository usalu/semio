//! ⚙️ Plugin component activation and the plugin artifact channel.

extern crate semio_os_kernel as store;

#[cfg(test)]
use semio_framework_value_testing_allocator as _;

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
