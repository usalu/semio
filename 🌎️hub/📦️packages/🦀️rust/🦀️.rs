//! 🌎️ `semio-hub` library surface — re-exports the backend-agnostic `HubDirectory` identity/tenancy
//! seam (+ sqlite/postgres/neo4j backends, each behind its own Cargo feature) for anything that
//! reasonably wants directory logic without the axum server (`bin.rs`'s `os-hub` binary target).
//! Contains no logic of its own — see `📇️directory/🦀️.rs` for the trait/model and
//! `📇️directory/{🪶️sqlite,🐘️postgres,🌐️neo4j}/🦀️.rs` for each backend.

// 🚫️async: R7 — `HubDirectory` is a public trait with `async fn` methods; callers cannot assume
// `Send` from the signature alone, but R3 answers that structurally (every dyn seam is now the
// concrete `HubDirectories` enum, so `Send` is derived at each call site from its variants, never
// from a bound on the trait). Never take rustc's suggested `-> impl Future + Send` fix.
#![allow(async_fn_in_trait)]

#[path = "../../📇️directory/🦀️.rs"]
pub mod directory;

#[path = "../../🗿️artifact-authority/🦀️.rs"]
pub mod artifact_authority;

#[path = "../../🛰️lag-rebootstrap/🦀️.rs"]
pub mod lag_rebootstrap;

#[path = "../../🚀️local-bootstrap/🦀️.rs"]
pub mod local_bootstrap;

#[path = "../../💡️inference/🦀️.rs"]
pub mod inference;
