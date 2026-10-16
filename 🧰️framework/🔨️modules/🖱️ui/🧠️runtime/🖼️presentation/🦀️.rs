//! 🖼️ The headless runtime presentation — typed entities with mutation leasing, actual-read dependency tracking, revision-guarded intent routing, and the component tree and producer a presenter builds before anything is reconciled.
//!
//! ```text
//! projections → Entity<T> state → Present → ComponentTree
//! ```
//!
//! It renders nothing and knows about no renderer, so it runs on both sides of the plugin boundary and compiles for
//! `wasm32-wasip2` and `wasm32-unknown-unknown` by construction.

#[path = "../🎛️context/🦀️.rs"]
mod context;
#[path = "../🎯️dispatch/🦀️.rs"]
mod dispatch;
#[path = "../🪪️entity/🦀️.rs"]
mod entity;
#[path = "../🎭️present/🦀️.rs"]
mod present;
#[path = "../🕸️tracking/🦀️.rs"]
mod tracking;

pub use context::*;
pub use dispatch::*;
pub use entity::*;
pub use present::*;
pub use tracking::*;
