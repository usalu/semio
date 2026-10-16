//! 🧩️ Extension bundle, inference gateway, describe and extension retirement of the plugin SDK.

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as pack;

#[path = "../💡️inference/🦀️.rs"]
pub mod artifact_inference;
#[path = "📨️invoke/⚠️failure/🦀️.rs"]
pub mod extension_invocation_failure;
#[path = "../🛂️describe/🦀️.rs"]
pub mod describe;
#[path = "../💡️inference/🚪️gateway/🦀️.rs"]
pub mod inference_gateway;
#[path = "🛬️lifted/🏃️runtime/🦀️.rs"]
pub mod extension_runtime_lift;

#[path = "🛬️lifted/🌳️tree/🦀️.rs"]
pub mod lifted_tree;
pub use lifted_tree::{commit_artifact_declarations, preflight_artifact_declarations};
