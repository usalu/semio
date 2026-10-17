//! 🧱️ The proctor's base: the SQLite storage roles, the quiz catalog, the process configuration, the crowd tally and the presence rules. Wiring only; the domain lives at the owner `🦀️.rs` files.

#![allow(async_fn_in_trait)]

#[path = "../🗄️storage/🦀️.rs"]
pub mod storage;

#[path = "../📚️catalog/🦀️.rs"]
pub mod catalog;

#[path = "../🎚️config/🦀️.rs"]
pub mod config;

#[path = "../👥️presence/🦀️.rs"]
pub mod presence;

#[path = "../👪️crowd/🦀️.rs"]
pub mod crowd;
