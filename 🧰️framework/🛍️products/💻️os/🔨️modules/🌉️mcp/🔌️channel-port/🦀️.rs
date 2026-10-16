//! 🔌️ The artifact channel port, mutation records and the policy engine.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as store;

#[path = "../🔀️dispatch/🔌️channel-port/🦀️.rs"]
pub mod channel_port;

#[path = "../🛡️policy/🦀️.rs"]
pub mod policy;
