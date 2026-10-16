//! 🔐️ Hub sign-in and space-browser contract twins: pure state, wire shapes and labels, no renderer and no transport.
//!
//! 🧭️ Native and wasm32. The two module files stay in their element trees (`🔐️HubSignIn`, `🏘️SpaceBrowser`); this crate root mounts them as the public modules `hub_sign_in` and `space_browser`.

#[path = "../../../🧱️elements/🔐️HubSignIn/🎯️targets/🧊️wgpu/🦀️.rs"]
pub mod hub_sign_in;

#[path = "../../../🧱️elements/🏘️SpaceBrowser/🎯️targets/🧊️wgpu/🦀️.rs"]
pub mod space_browser;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
