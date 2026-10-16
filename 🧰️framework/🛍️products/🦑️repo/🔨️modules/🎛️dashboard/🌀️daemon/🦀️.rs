//! 🌀️ The dashboard daemon: the process that owns the developer processes of a workspace independently of the views attached to it.
//! One folder per concern. The wire both ends agree on and the plumbing under it (the frames, the transport, the readiness and
//! replay of output, the journal, the lease and the connection a view holds) is the `dashboard_wire` crate; this crate is what
//! only the daemon process and its controllers need — the views it serves, the launch groups, the supervisor, the control surface
//! without a terminal and the lifecycle — and the `semio daemon start|serve|stop|status|attach` verb is the `⌨️command` folder.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🌀️daemon/🔣️.json

use dashboard_wire::{client, ipc, journal, lease, ready, replay, transport};

#[path = "👥️clients/🦀️.rs"]
pub mod clients;

#[path = "🧩️groups/🦀️.rs"]
pub mod groups;

#[path = "🧠️supervisor/🦀️.rs"]
pub mod supervisor;

#[path = "🕹️control/🦀️.rs"]
pub mod control;

#[path = "🚦️lifecycle/🦀️.rs"]
pub mod lifecycle;

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🧊️integration/🦀️.rs"]
mod integration;
// #endregion 🔖️Tests
