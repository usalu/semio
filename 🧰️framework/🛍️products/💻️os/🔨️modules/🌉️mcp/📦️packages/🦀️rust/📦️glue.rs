//! 🌉️ `semio-framework-os-mcp` glue — mounts the `⚠️errors`/`🧬️schema`/`🧭️protocol`/`🚚️transport`
//! facets plus the module root, exactly as `🏃️run`/`🖥️shell`'s own glue files mount theirs.

#[path = "../../⚠️errors/🦀️component.rs"]
pub mod errors;

#[path = "../../🧬️schema/🦀️component.rs"]
pub mod schema;

#[path = "../../🧭️protocol/🦀️component.rs"]
pub mod protocol;

#[path = "../../🚚️transport/🦀️component.rs"]
pub mod transport;

#[path = "../../🦀️component.rs"]
mod root;
pub use root::*;
