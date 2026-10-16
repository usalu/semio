//! 🧪️ The law library every committed mutation vector of this subset is checked with.
//!
//! Source of truth is every quintet under `✳️any/🧪️tests/🧫️fixtures/🧬️mutations/<leaf>/<case>/` (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`): `📸️snapshot/⬅️before`, `📸️snapshot/➡️after`, `🦠️mutation`, `🎯️outcome` and
//! `🔺️diff` (or its empty `🚫️.absent` marker). Each implementation case `🧬️mutations/<leaf>/🧪️tests/<case>/🦀️.rs` names its
//! own vector; the vector is read at test runtime, so no fixture byte enters a compilation unit, and its declared
//! `status` selects the law set — `applied`, `no-op` or `rejected`.

#[path = "🧫️committed-vector/🦀️.rs"]
mod committed_vector;
#[path = "⚖️vector-laws/🦀️.rs"]
mod vector_laws;

pub use committed_vector::Vector;
pub use vector_laws::{assert_case, assert_inverse_sum};
