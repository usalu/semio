//! ✍️ Text editor scene document, delivery and interaction of an engine surface.
//!
//! The editor scene document, the typing delivery driver, the explicit draft settings and the public text editor pointer, key, wheel and
//! action API; the delivery state itself lives with the surface registry.

#[path = "../../../🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/✍️text-editor/🦀️.rs"]
pub mod text_editor;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
