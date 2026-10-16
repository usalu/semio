//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

//! 🚚️ Repository move: every structural edit the repo CLI and MCP server can make to a checkout —
//! renaming a token in all of its casing variants, moving and deleting files and folders, creating,
//! renaming and deleting sections, extracting a section into its own file and integrating a file
//! into a section — expressed as a pure transformation from a [`Workspace`] to a [`Plan`] of
//! [`Change`]s. Nothing here touches a disk; an [`Executor`] applies a plan, and the in-memory
//! [`Workspace`] is itself one, so every operation is testable without a filesystem.
//! Behaviour twin of `github.com/usalu/semio/repo/move`.

//#endregion 🧲️Header

/// 🗣️ The section grammar this module edits. Re-exported explicitly so a client of the move crate
/// never has to reach past it for the types its own signatures speak in.
pub use semio_framework_repo_languages::{language_for_path, parse_sections, parse_sections_with, Language, Section};

#[path = "🗄️workspace/🦀️.rs"]
mod workspace;
pub use workspace::{clean_relative, Entry, Workspace};
#[path = "🧾️plan/🦀️.rs"]
mod plan;
use plan::fail;
pub use plan::{Change, Message, MoveError, Plan, PlanEvent};
#[path = "🎬️executor/🦀️.rs"]
mod executor;
pub use executor::{execute, Executor, FileSystemExecutor};
#[path = "🔤️casing/🦀️.rs"]
mod casing;
pub use casing::{apply_rename_casings, simple_lower, simple_upper, title_case_token};
#[path = "🗣️language-helpers/🦀️.rs"]
mod language_helpers;
use language_helpers::ensure_trailing_newline;
pub use language_helpers::{extract_imports, extract_package, find_section, format_imports, merge_headers, policy_section_end_match, policy_section_start_match, split_header, unique_strings};
#[path = "🔏️sealed/🦀️.rs"]
mod sealed;
use sealed::{refuse_sealed_move, sealed_within};
pub use sealed::{sealed_document_paths, SEALED_TAXONOMY_PATH};
#[path = "✏️rename/🦀️.rs"]
mod rename;
pub use rename::plan_rename;
#[path = "📁️folders/🦀️.rs"]
mod folders;
pub use folders::{plan_folder_create, plan_folder_delete, plan_folder_move};
#[path = "📄️files/🦀️.rs"]
mod files;
pub use files::{plan_file_create, plan_file_delete, plan_file_move};
#[path = "📚️agents-docs/🦀️.rs"]
mod agents_docs;
pub use agents_docs::{plan_agents_docs_path, plan_agents_docs_removal};
#[path = "📑️sections/🦀️.rs"]
mod sections;
pub use sections::{plan_section_create, plan_section_delete, plan_section_move};
#[path = "📥️integrate/🦀️.rs"]
mod integrate;
pub use integrate::plan_integrate;
#[path = "🧲️extract/🦀️.rs"]
mod extract;
pub use extract::plan_extract;
//#region 🧪️Tests

#[cfg(test)]
#[path = "../../🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#endregion 🧪️Tests
