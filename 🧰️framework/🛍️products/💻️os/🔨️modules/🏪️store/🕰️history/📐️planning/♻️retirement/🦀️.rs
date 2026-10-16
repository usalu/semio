//! 📐️ The complete original planning state declares every typed field for controlled retirement.
use semio_framework_value::{artifact_retire_leaf,artifact_retire_struct};
artifact_retire_leaf!(semio_framework_os_store_replay::ReplayProgress);
artifact_retire_struct!(crate::HistoryReadPlan {target,drafts,positions,first_input,last_draft,order,counts,cursor,located_target,located_draft,located_inputs,operations,supersessions,copied,copying_drafts,report,progress,finished});

