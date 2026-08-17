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

#[path = "../../🎫️handles/🦀️component.rs"]
pub mod handles;

#[path = "../../📒️audit/🦀️component.rs"]
pub mod audit;

#[path = "../../🧵️bridge/🦀️component.rs"]
pub mod bridge;

#[path = "../../🗂️catalog/🦀️component.rs"]
pub mod catalog;

#[path = "../../🔎️search/🦀️component.rs"]
pub mod search;

#[path = "../../🧠️context/🦀️component.rs"]
pub mod context;

#[path = "../../🧪️conformance/🦀️component.rs"]
pub mod conformance;

#[path = "../../🧫️fixtures/🦀️component.rs"]
pub mod fixtures;

#[path = "../../🦀️component.rs"]
mod root;
pub use root::*;
