//! 🧩️ Trinity Jack app — document entities (constitutional: general).
//!
//! 📌️ Deviation from the constitutional-split recipe: Jack's document (`GraphFixture`) and its
//! `DocumentDsl`/`DocumentPack` impls are owned by the shared `trinity_ram` crate (used directly by
//! both the `jack` and `rewrite` apps), not defined locally. This crate re-exports it so every
//! constitutional slot exists as a real crate per the recipe's shape, without duplicating the type.

pub use trinity_ram::{GraphFixture, TRINITY_GRAPH_SCHEMA};
