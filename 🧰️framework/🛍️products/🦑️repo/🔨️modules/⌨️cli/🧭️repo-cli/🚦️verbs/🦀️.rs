//! 🚦️ One handler per verb, each a thin front for the domain crate that owns the aggregate.

//#region 🧭️RepoCli
//#region 🚦️Verbs

#[path = "🧰️verb-support/🦀️.rs"]
mod verb_support;
use verb_support::{client_enum, extract_client, extract_effort, extract_llm, flag_or_arg, missing_client, missing_llm, print_lines, ticket_path};
pub use verb_support::resolve_section_name;
#[path = "🔌️gateway-verbs/🦀️.rs"]
mod gateway_verbs;
pub use gateway_verbs::{graphql, hook, mcp, test};
#[path = "🎫️ticket-verbs/🦀️.rs"]
mod ticket_verbs;
pub use ticket_verbs::{ticket_change, ticket_close, ticket_files, ticket_list, ticket_open, ticket_purge_artifacts, ticket_reopen, ticket_show};
#[path = "🎯️planning-verbs/🦀️.rs"]
mod planning_verbs;
pub use planning_verbs::{contributor_add, contributor_list, contributor_remove, goal_change, goal_close, goal_list, goal_open, goal_reopen, goal_tree, todo_change, todo_create, todo_delete, todo_list, todo_search};
#[path = "📁️artifact-verbs/🦀️.rs"]
mod artifact_verbs;
pub use artifact_verbs::{
    extract, file_create, file_delete, file_list, file_move, folder_create, folder_delete, folder_list, folder_move, integrate, move_artifact, rename, section_create, section_delete, section_extract, section_integrate, section_list, section_move,
    sync, sync_management,
};
#[path = "🔎️query-verbs/🦀️.rs"]
mod query_verbs;
pub use query_verbs::{export, list, query, search, tree_goal, tree_monorepo, tree_statute, tree_territory};
#[path = "📊️code-metric-verbs/🦀️.rs"]
mod code_metric_verbs;
pub use code_metric_verbs::{benchmark, bundle_list, bundle_tree, loc, mermaid_contributors, mermaid_languages, mermaid_technologies, technology, technology_list, technology_tree};
#[path = "⚖️statute-verbs/🦀️.rs"]
mod statute_verbs;
pub use statute_verbs::{analyze, autofix, statute_list, statute_tree};
#[path = "🧩️system-verbs/🦀️.rs"]
mod system_verbs;
pub use system_verbs::{auth_status, auth_whoami, configure, entity_emojis, micro_commit, update};
#[path = "📜️history-verbs/🦀️.rs"]
mod history_verbs;
pub use history_verbs::{checkpoint_list, definition_list, draft_create, draft_delete, draft_list, interaction_list, interaction_tree};
//#endregion 🚦️Verbs
//#endregion 🧭️RepoCli
