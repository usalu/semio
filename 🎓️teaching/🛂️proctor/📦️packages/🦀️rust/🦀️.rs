//! 📦️ Package glue — wiring only. Domain lives at the owner `🦀️.rs` files.

#![allow(async_fn_in_trait)]

#[path = "../../🔨️modules/🧩️instance/🦀️.rs"]
pub mod instance;

#[path = "../../🔨️modules/⌨️cli/🦀️.rs"]
pub mod cli;
