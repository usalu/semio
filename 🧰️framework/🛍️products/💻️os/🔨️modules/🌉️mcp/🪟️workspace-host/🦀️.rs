//! 🪟️ The headless workspace and its gateway backend.

extern crate semio_framework_os_kernel as store;

#[path = "../🏠️workspace/🛂️gateway-backend/🦀️.rs"]
pub mod gateway_backend;

#[path = "../🏠️workspace/🪪️headless-workspace-state/🦀️.rs"]
pub mod headless_workspace_state;

#[path = "../🏠️workspace/💡️inference-facade/🦀️.rs"]
pub mod inference_facade;

#[path = "../🏠️workspace/🦀️.rs"]
pub mod workspace;

#[cfg(test)]
#[path = "../🏠️workspace/🧪️tests/🔬️long/🦀️.rs"]
pub mod workspace_long;
