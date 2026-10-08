//! 📐️ The complete original planning state declares every typed field for controlled retirement.
use semio_framework_value::{artifact_retire_leaf,artifact_retire_struct};
artifact_retire_leaf!(super::ReplayProgress);
artifact_retire_struct!(super::HistoryReadPlan {target,drafts,positions,first_input,last_draft,order,counts,cursor,located_target,located_draft,located_inputs,operations,supersessions,copied,copying_drafts,report,progress,finished});
#[cfg(test)]
#[path="../🧪️tests/🦀️.rs"]
mod tests;
