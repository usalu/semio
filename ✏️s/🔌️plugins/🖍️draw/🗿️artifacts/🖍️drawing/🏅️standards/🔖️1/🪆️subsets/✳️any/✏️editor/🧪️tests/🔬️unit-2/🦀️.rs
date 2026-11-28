//! 🔬️ Drawing editor unit laws of the shared unit context: selection, nodes, commands, envelopes, pointers and the clipboard runtime.

#[path = "../🔬️unit/🧬️prelude/🦀️.rs"]
pub mod prelude;
pub use prelude::context;
use prelude::*;

#[path = "../🔬️unit/🧩️harness/🦀️.rs"]
mod harness;
use harness::*;

#[path="../../📋️clipboard/🧪️tests/🪟️runtime/🦀️.rs"]
mod clipboard_runtime;

#[path = "../🔬️unit/🎛️selection-laws/🦀️.rs"]
mod selection_laws;
#[path = "../🔬️unit/🔷️node-laws/🦀️.rs"]
mod node_laws;
#[path = "../🔬️unit/⌨️command-laws/🦀️.rs"]
mod command_laws;
#[path = "../🔬️unit/✉️envelope-laws/🦀️.rs"]
mod envelope_laws;
#[path = "../🔬️unit/👆️pointer-laws/🦀️.rs"]
mod pointer_laws;
