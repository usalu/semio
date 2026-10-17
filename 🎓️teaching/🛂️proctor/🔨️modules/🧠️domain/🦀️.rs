//! 🧠️ The proctor's domain: the learner and handle deciders, the read-model projections and the query handler. Wiring only; the domain lives at the owner `🦀️.rs` files.

#![allow(async_fn_in_trait)]

#[path = "../🎭️actors/🦀️.rs"]
pub mod actors;

#[path = "../🔭️projections/🦀️.rs"]
pub mod projections;

#[path = "../❓️queries/🦀️.rs"]
pub mod queries;
