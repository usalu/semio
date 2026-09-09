//! 📊️ Schema for one Jack results window's ephemeral query output.

use crate::ast::QueryResult;

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[dsl(extension = "trinity.jackresultswindowtransient")]
#[dsl(layout = "lines")]
pub struct JackResultsWindowTransient {
    pub query_execution_id: Option<String>,
    pub result: Option<QueryResult>,
    pub query_error: Option<String>,
}
