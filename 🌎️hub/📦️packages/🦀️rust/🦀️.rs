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

// 🔐️ Credential sign-in, session minting policy and the route rate limiter — the `hub.auth` scope.
#[path = "../../🔐️auth/🦀️.rs"]
pub mod auth;

#[path = "../../🗿️artifact-authority/🦀️.rs"]
pub mod artifact_authority;

#[path = "../../🛰️lag-rebootstrap/🦀️.rs"]
pub mod lag_rebootstrap;

#[path = "../../🚀️local-bootstrap/🦀️.rs"]
pub mod local_bootstrap;

#[path = "../../💡️inference/🦀️.rs"]
pub mod inference;

// 🗄️ Hub as instance #1 of the `semio-framework-server` product: `HubInstance` plus the four
// durable storage roles behind it.
#[path = "../../🗄️stores/🦀️.rs"]
pub mod stores;

// 🗂️ Also under `integration-fixtures`: that feature exists to serve the crate's OTHER targets
// (`🗿️artifact-authority/🔏️trusted-catalog`'s `trusted_catalog_fixture`), which builds its profiles
// under this same artifact root, and a `#[cfg(test)]` module cannot serve them.
#[cfg(any(test, feature = "integration-fixtures"))]
#[path = "../../🧪️tests/🗂️artifact-root/🦀️.rs"]
pub(crate) mod test_artifact_root;

#[cfg(test)]
#[path = "../../🔐️auth/🧪️tests/🧭️credential-source-order/🔮️oracles/🦀️.rs"]
mod credential_source_order_oracle;

#[cfg(test)]
#[path = "../../📇️directory/🔐️authorization/🔌️socket-grant/🧪️tests/🔮️oracles/🦀️.rs"]
mod socket_grant_oracle;
