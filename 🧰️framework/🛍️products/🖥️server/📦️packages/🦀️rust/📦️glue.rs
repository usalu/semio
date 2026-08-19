//! 📦️ Package glue — wiring only. Domain lives at the owner `🦀️component.rs` files.

#[path = "../../🔨️modules/🧬️contract/🦀️component.rs"]
pub mod contract;

#[path = "../../🔨️modules/🗄️storage/🦀️component.rs"]
pub mod storage;

#[path = "../../🔨️modules/🛡️policy/🦀️component.rs"]
pub mod policy;

#[path = "../../🔨️modules/🎭️authority/🦀️component.rs"]
pub mod authority;

#[path = "../../🔨️modules/📡️gateway/🦀️component.rs"]
pub mod gateway;

#[path = "../../🦀️component.rs"]
mod component;
pub use component::*;
