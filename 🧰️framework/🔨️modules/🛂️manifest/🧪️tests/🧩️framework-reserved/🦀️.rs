//! 🧩️ The framework hosts every reserved feature contribution.
use super::*;

#[test]
fn the_framework_hosts_the_history_edit_and_tool_run_contributions() {
    assert!(FRAMEWORK_RESERVED_CONTRIBUTIONS.iter().any(|contribution| contribution.id == history_edit::RESERVED.id), "the framework hosts the history-edit contribution");
    assert!(FRAMEWORK_RESERVED_CONTRIBUTIONS.iter().any(|contribution| contribution.id == tool_run::RESERVED.id), "the framework hosts the tool run contribution");
}
