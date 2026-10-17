//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

//#endregion 🧲️Header

//! 🎯️ Goals of the `🦑️repo` product: the `GOAL/SUBGOAL` identifier scheme, the `🎯️goal.json`
//! document codec, the goal/ticket tree and the open/change/close/reopen/delete lifecycle.
//!
//! Every effect is a port. A goal document is read and written through [`GoalStore`], the
//! management provider that owns milestones and issues is [`ManagementPort`], and the coordinator
//! is `semio_framework_repo_events::Emitter`. [`MemoryGoalStore`], [`RecordingManagement`] and
//! [`MemoryEmitter`] are the in-memory implementations the language-agnostic tests run against, so
//! nothing in this crate needs a repository on disk, a `gh` binary or a network to be judged.
//!
//! The wire contract is `🧬️schema/🔣️.json`; the fixtures under `🧫️fixtures` are real documents
//! taken from this repository's own `🎯️goals` tree.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use semio_framework_repo_events::Emitter;
use semio_framework_repo_identity as identity;
use semio_framework_repo_model as model;

pub use model::{Goal, GoalChangeInput, GoalCloseInput, GoalCreateInput, GoalDates, GoalDeleteInput, GoalManagementData, GoalNode, GoalReopenInput, GoalStatus, TicketNode};

#[path = "🚨️errors/🦀️.rs"]
mod errors;
pub use errors::*;

#[path = "🪪️id-scheme/🦀️.rs"]
mod id_scheme;
pub use id_scheme::*;

#[path = "🗄️store-and-codec/🦀️.rs"]
mod store_and_codec;
pub use store_and_codec::*;

#[path = "🧩️management-and-emission/🦀️.rs"]
mod management_and_emission;
pub use management_and_emission::*;

#[path = "🏛️goals-aggregate/🦀️.rs"]
mod goals_aggregate;
pub use goals_aggregate::*;

#[path = "✨️goal-create/🦀️.rs"]
mod goal_create;

#[path = "✏️goal-change-and-close/🦀️.rs"]
mod goal_change_and_close;

#[path = "🔄️goal-reopen-delete-and-sync/🦀️.rs"]
mod goal_reopen_delete_and_sync;

#[path = "🌱️tree-seeds/🦀️.rs"]
mod tree_seeds;
pub use tree_seeds::*;

#[path = "🌳️tree-build/🦀️.rs"]
mod tree_build;
pub use tree_build::*;

#[path = "🖨️tree-render/🦀️.rs"]
mod tree_render;
pub use tree_render::*;
