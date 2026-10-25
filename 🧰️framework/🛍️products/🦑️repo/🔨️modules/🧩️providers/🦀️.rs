//! 🧩️ Composable provider integrations for the semio repository domain: issue/milestone management
//! (GitHub through the `gh` CLI), version control (git), sandbox (devcontainer), the eight agent-client
//! providers and the registry that hands them out.
//!
//! Every external process goes through [`ProcessRunner`], so each provider is exercised end to end
//! without `gh` or `git` on the machine — see `🧪️tests/🐙️github-management-transcripts`.
//!
//! `#![allow(async_fn_in_trait)]` is the per-crate responsibility every `#[dyn_enum]` declaring crate
//! carries; see `🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust/🦀️.rs`.
#![allow(async_fn_in_trait)]

use dispatch_macros::{dyn_enum, dyn_enum_close};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::cell::RefCell;

//#region 🔖️Children

#[path = "📐️provider-model/🦀️.rs"]
mod provider_model;
pub use provider_model::*;

#[path = "🏃️process-runner/🦀️.rs"]
mod process_runner;
pub use process_runner::*;

#[macro_use]
#[path = "🔭️provider-interfaces/🦀️.rs"]
mod provider_interfaces;
pub use provider_interfaces::*;

#[path = "🐙️github-management/🦀️.rs"]
mod github_management;
pub use github_management::*;

#[path = "🌿️version-control-and-sandbox/🦀️.rs"]
mod version_control_and_sandbox;
pub use version_control_and_sandbox::*;

#[path = "🎆️agent-client-providers/🦀️.rs"]
mod agent_client_providers;
pub use agent_client_providers::*;

#[path = "🎖️registry-and-git-hooks/🦀️.rs"]
mod registry_and_git_hooks;
pub use registry_and_git_hooks::*;

//#endregion 🔖️Children

// #region 🧪️Tests
#[cfg(test)]
#[path = "./🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🧪️Tests
