//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

//! 🗄️ The production repo GraphQL context: `FsRepoContext` answers every aggregate from the repository on disk
//! through the domain crate that owns it, with the provider-backed goal management and the coordinator event
//! emitter it needs. The grammar, schema and executor stay in `semio-framework-repo-graphql`, whose `RepoContext`
//! port this crate implements. Twin of `NewRepoContext` in `🔗️graphql/📦️packages/🐹️go/🩻️context/🐹️.go`.

//#endregion 🧲️Header

use semio_framework_repo_codebase::{Codebase, CodebaseContext, Scope as CodebaseScope};
use semio_framework_repo_goals::{FsGoalStore, GoalError, GoalStore, Goals, NullManagement};
use semio_framework_repo_graphql::{parse_milestone_number, ContextError, RepoContext};
use semio_framework_repo_identity::{emoji_text, entity, flat};
use semio_framework_repo_model::{
    AnalyzeMetrics, AnalyzeResult, Breach, BreachPriority, Bundle, Checkpoint, Contributor, ContributorAddInput, Definition, Draft, DraftCreateInput, File, FilterInput, FixResult, Folder, Goal,
    GoalChangeInput, GoalCloseInput, GoalCreateInput, GoalDeleteInput, GoalManagementData, GoalReopenInput, GoalStatus, InteractionResource, Policy, PriorityCount, Section, StatuteMeta, Technology,
    Ticket, TicketChangeInput, TicketCloseInput, TicketDeleteInput, TicketOpenInput, TicketReopenInput, TicketStatus, Todo, TodoChangeInput, TodoCreateInput,
};
use semio_framework_repo_providers::{ManagementLabel, ManagementMilestone, ManagementProvider, ManagementProviders, McpClientKind, VersionControlProvider};
use semio_framework_repo_statutes::{SourceFile, SourceSet};
use semio_framework_repo_tickets::{CoordinatorEventSink, FileTicketStore, PlanRoots, SystemClock as TicketSystemClock, TicketChangeRequest, TicketCloseRequest, TicketId, TicketLayout, TicketOpenRequest, TicketReopenRequest, TicketService};
use semio_framework_repo_todos::{FsDraftStore, FsTodoTree, Todos};
use serde_json::{json, Value as Json};

#[path = "📡️emitter-and-management/🦀️.rs"]
mod emitter_and_management;
pub use emitter_and_management::*;

#[path = "🧭️analyze-scope/🦀️.rs"]
mod analyze_scope;
pub use analyze_scope::*;

#[path = "🏠️context-core/🦀️.rs"]
mod context_core;
pub use context_core::*;

#[path = "🎯️goal-and-file-support/🦀️.rs"]
mod goal_and_file_support;

#[path = "🔧️refactor-operations/🦀️.rs"]
mod refactor_operations;

#[path = "🔄️management-sync/🦀️.rs"]
mod management_sync;
use management_sync::*;

#[path = "🔌️repo-context-port/🦀️.rs"]
mod repo_context_port;

//#region 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

//#endregion 🧪️Tests
