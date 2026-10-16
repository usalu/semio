//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

//#endregion 🧲️Header

//! 🪝️ Lifecycle hooks of the `🦑️repo` product: an IDE hands a native payload to the repo binary,
//! this module normalises it into a neutral [`HookEvent`], decides whether the tool invocation is
//! allowed, projects the event-specific record, formats it the way the calling IDE reads it, and
//! records it in the session log.
//!
//! Layering (`📓️go-region-dependency-graph.md`, fixes 5 and 7): the per-IDE native event tables and
//! the output formatting live in `🧩️providers`; this module reaches DOWN into them through
//! [`HookFormatter`] and [`EditorProvider`] and providers never reaches back. `McpClientKind` is
//! taken from `🧩️providers` too, so nothing here imports `🔌️mcp` at L5.
//!
//! Everything that would otherwise touch a clock, a process or the filesystem is a port with an
//! inert default: [`HookEnvironment`] (checkpoint identity, commit message, formatter),
//! [`TestFileResolver`] (command → test file paths, owned by `🏃️test-runner`) and [`SessionStore`]
//! (where `session.json` lives). A caller that supplies none of them still gets the full
//! normalisation, blocking, plan and formatting behaviour, deterministically.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🪝️hooks/🧬️schema/🔣️.json

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

//#region 🔁️Reexports
/// 🔁️ The JSON reader hook payloads arrive in. Re-exported because a hook payload is JSON by
/// definition: a client of this crate — a test adapter, the CLI, the MCP server — must be able to
/// build and read one without declaring a dependency this crate already carries.
pub use serde_json;
/// 🔁️ The recorded plan shapes this module folds an incoming plan into, declared by `📐️model`.
pub use semio_framework_repo_model::{TicketAgentPlan, TicketAgentPlanStep};
/// 🔁️ The event vocabulary, the result shape, the editor providers that format it and the process
/// runner micro-commit is delegated through — all declared by `🧩️providers`, re-exported so no
/// consumer has to name a module one level down to speak about a hook. `📋️plan.md` §3: an exported
/// api never forces a client to reach past the module it is talking to.
pub use semio_framework_repo_providers::{format_plain_hook_output, get_editor_provider, resolve_claude_compatible_event, EditorProvider, EditorProviders, HookEvent, HookFormatter, HookResult, McpClientKind, ProcessExchange, ProcessOutcome, ProcessRequest, ProcessRunner, ProcessTranscript, RecordedProcessRunner, SystemProcessRunner, ToolKind, ALL_HOOK_EVENTS};
/// 🔁️ The logging switches this module obeys, declared by `🏠️workspace` which owns `📋️config.toml`.
pub use semio_framework_repo_workspace::{load_repo_config, parse_repo_config, repo_meta_dir_for_root, LoggingConfig, RepoConfig};
//#endregion 🔁️Reexports

//#region 🔖️Children

#[path = "🎯️hook-model/🦀️.rs"]
mod hook_model;
pub use hook_model::*;

#[path = "🧭️payload-reading/🦀️.rs"]
mod payload_reading;
pub use payload_reading::*;

#[path = "🛡️tool-policy/🦀️.rs"]
mod tool_policy;
pub use tool_policy::*;

#[path = "🗺️plan-steps/🦀️.rs"]
mod plan_steps;
pub use plan_steps::*;

#[path = "🔶️dispatch-and-formatting/🦀️.rs"]
mod dispatch_and_formatting;
pub use dispatch_and_formatting::*;

#[path = "📓️session-logging/🦀️.rs"]
mod session_logging;
pub use session_logging::*;

#[path = "💽️system-ports/🦀️.rs"]
mod system_ports;
pub use system_ports::*;

//#endregion 🔖️Children

//#region 🧪️Tests
#[cfg(test)]
#[path = "../../🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
