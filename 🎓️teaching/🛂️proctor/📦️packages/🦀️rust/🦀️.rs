//! 📦️ Package glue — wiring only. Domain lives at the owner `🦀️.rs` files.

#![allow(async_fn_in_trait)]

#[path = "../../🔨️modules/🗄️storage/🦀️.rs"]
pub mod storage;

#[path = "../../🔨️modules/📚️catalog/🦀️.rs"]
pub mod catalog;

#[path = "../../🔨️modules/🎚️config/🦀️.rs"]
pub mod config;

#[path = "../../🔨️modules/👥️presence/🦀️.rs"]
pub mod presence;

#[path = "../../🔨️modules/🎭️actors/🦀️.rs"]
pub mod actors;

#[path = "../../🔨️modules/👪️crowd/🦀️.rs"]
pub mod crowd;

#[path = "../../🔨️modules/🔭️projections/🦀️.rs"]
pub mod projections;

#[path = "../../🔨️modules/❓️queries/🦀️.rs"]
pub mod queries;

#[path = "../../🔨️modules/🧩️instance/🦀️.rs"]
pub mod instance;

#[path = "../../🔨️modules/⌨️cli/🦀️.rs"]
pub mod cli;
