//! 🧵️ The daemon wire: what both ends of the workspace daemon agree on and the plumbing under the supervisor — the frames and the
//! workspace paths, the transport, the readiness of output, its replay from log files, the journal of sessions, the lease of a
//! workspace and the connection a view holds. The supervisor, the control surface and the terminal view build on it.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🌀️daemon/🔣️.json

#[path = "../✉️ipc/🦀️.rs"]
pub mod ipc;

#[path = "../🟢️ready/🦀️.rs"]
pub mod ready;

#[path = "../📼️replay/🦀️.rs"]
pub mod replay;

#[path = "../🚚️transport/🦀️.rs"]
pub mod transport;

#[path = "../📜️journal/🦀️.rs"]
pub mod journal;

#[path = "../🔐️lease/🦀️.rs"]
pub mod lease;

#[path = "../../📎️connection/🦀️.rs"]
pub mod client;
