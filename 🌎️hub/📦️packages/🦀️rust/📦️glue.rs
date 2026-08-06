//! 🌎️ `semio-hub` library surface — re-exports the backend-agnostic `HubDirectory` identity/tenancy
//! seam (+ sqlite/postgres/neo4j backends, each behind its own Cargo feature) for anything that
//! reasonably wants directory logic without the axum server (`bin.rs`'s `os-hub` binary target).
//! Contains no logic of its own — see `📇️directory/🦀️component.rs` for the trait/model and
//! `📇️directory/{🪶️sqlite,🐘️postgres,🌐️neo4j}/🦀️component.rs` for each backend.

#[path = "../../📇️directory/🦀️component.rs"]
pub mod directory;
