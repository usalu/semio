//! 🧳️ Artifact, ui, context resource and prompt tools.

extern crate semio_framework_os_kernel as store;

#[path = "../🗿️artifact/🦀️.rs"]
pub mod artifact;

#[cfg(test)]
#[path = "../🗿️artifact/🧪️tests/🔬️quick/🦀️.rs"]
pub mod artifact_quick;

#[path = "../🧠️context/🦀️.rs"]
pub mod context;

#[cfg(test)]
#[path = "../🧠️context/🧪️tests/🔬️quick/🦀️.rs"]
pub mod context_quick;

#[path = "../💬️prompts/🦀️.rs"]
pub mod prompts;

#[cfg(test)]
#[path = "../💬️prompts/🧪️tests/🔬️quick/🦀️.rs"]
pub mod prompts_quick;

#[path = "../🖥️ui/🦀️.rs"]
pub mod ui;

#[cfg(test)]
#[path = "../🖥️ui/🧪️tests/🔬️quick/🦀️.rs"]
pub mod ui_quick;
