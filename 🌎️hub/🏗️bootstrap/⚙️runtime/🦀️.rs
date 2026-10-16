//! ⚙️ Hub runtime foundations: startup supervisors and controls, readiness, transport middleware, lifecycle and the shared HTTP support that carry no `HubState`.

#[path = "../🧰️http-support/🦀️.rs"]
pub mod support;

#[path = "../👥️presence/🦀️.rs"]
pub mod presence;

#[path = "../🛑️lifecycle/🦀️.rs"]
pub mod lifecycle;

#[path = "../🩺️readiness/🦀️.rs"]
pub mod readiness;

#[path = "../🚀️startup/🦀️.rs"]
pub mod startup;

#[path = "../🚦️transport/🦀️.rs"]
pub mod transport;

