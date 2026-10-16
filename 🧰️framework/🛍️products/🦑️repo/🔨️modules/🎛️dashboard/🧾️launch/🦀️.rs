//! 🧾️ The launch contract: what a resolved launch is made of and what travels with a running task — the processes in start order,
//! the services that must be ready first, how a group stops, what a task is called and the address its output announces.
//! The registry produces these, the daemon wire carries them; neither depends on the other.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🎮️registry/🦀️.rs
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🌀️daemon/✉️ipc/🦀️.rs

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// 🛑️ Whether stopping one member of a compound stops the others.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Stop { Together, #[default] Independent }

/// 🏷️ What a running task is called; the registry computes it at launch and it travels with the session.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TaskLabel {
    pub verb: String,
    pub owner: Vec<String>,
    pub subject: String,
    pub qualifier: String,
    pub parameters: Vec<(String, String)>,
    pub members: u16,
}

/// 🟢 Ready when the output shows `http://(127.0.0.1|localhost|0.0.0.0):<port>`; the ready URL is that match
/// plus `path`, or with `printed` the whole printed address.
///
/// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🌀️daemon/🟢️ready/🦀️.rs
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Ready {
    pub port: u16,
    pub path: String,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub printed: bool,
}

/// ▶️ One process of a resolved launch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaunchProcess {
    pub command_id: String,
    pub cmd: String,
    pub args: Vec<String>,
    pub cwd: PathBuf,
    pub env: Vec<(String, String)>,
    pub label: TaskLabel,
    pub ready: Option<Ready>,
    pub long_running: bool,
}

/// 🚀️ `(id, chosen parameters, extra arguments)` resolved: the processes in start order, the
/// services that must be ready first and how the group stops.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Launch {
    pub command_id: String,
    pub label: TaskLabel,
    pub processes: Vec<LaunchProcess>,
    pub requires: Vec<Launch>,
    pub group: Option<String>,
    pub stop: Stop,
}
