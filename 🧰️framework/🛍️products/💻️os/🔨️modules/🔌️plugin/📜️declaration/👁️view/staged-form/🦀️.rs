//! 📝️ Original source-owned ordinary controls share exact typed bindings across renderer families.
#[path="payload.rs"]
mod payload;
pub use payload::*;
#[path="control.rs"]
mod control;
pub use control::*;
#[path="status.rs"]
mod status;
pub use status::*;
#[path="body.rs"]
mod body;
pub use body::*;
#[path="binding.rs"]
mod binding;
pub use binding::*;
#[path="output.rs"]
mod output;
pub use output::*;
