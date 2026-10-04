//! 📊️ Schema for one Jack results window's ephemeral query output.

use crate::ast::QueryResult;

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_os_kernel::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[artifact(extension = "trinity.jackresultswindowtransient")]
#[dsl(layout = "lines")]
pub struct JackResultsWindowTransient {
    pub query_execution_id: Option<String>,
    pub result: Option<QueryResult>,
    pub query_error: Option<String>,
}
