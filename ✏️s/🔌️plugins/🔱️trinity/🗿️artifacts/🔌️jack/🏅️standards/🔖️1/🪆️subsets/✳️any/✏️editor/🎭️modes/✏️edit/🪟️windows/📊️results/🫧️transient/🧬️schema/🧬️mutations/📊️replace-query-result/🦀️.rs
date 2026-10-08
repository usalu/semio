//! 📊️ Replaces output in the addressed Jack results window.

use super::{JackResultsWindowTransient, JackResultsWindowTransientDiff, JackResultsWindowTransientMutation};
use crate::ast::QueryResult;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "replace-query-result")]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplaceQueryResult {
    pub execution_id: Option<String>,
    pub result: Option<QueryResult>,
    pub error: Option<String>,
}

impl protocol::MutationKind<JackResultsWindowTransient, JackResultsWindowTransientMutation> for ReplaceQueryResult {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "results-window-query-output", kind: "replace-query-result", record: "ReplaceQueryResult" };
    fn diff(&self, base: &JackResultsWindowTransient) -> protocol::MutationOutcome<JackResultsWindowTransientDiff> {
        let diff = JackResultsWindowTransientDiff {
            query_execution_id: (self.execution_id != base.query_execution_id).then(|| self.execution_id.clone()),
            result: (self.result != base.result).then(|| self.result.clone()),
            query_error: (self.error != base.query_error).then(|| self.error.clone()),
        };
        if protocol::DiffAlgebra::<JackResultsWindowTransient>::is_empty(&diff) {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "The results window already holds this query output.");
        }
        protocol::MutationOutcome::new(diff)
    }
    fn inverse(&self, base: &JackResultsWindowTransient) -> Result<Vec<JackResultsWindowTransientMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Self { execution_id: base.query_execution_id.clone(), result: base.result.clone(), error: base.query_error.clone() }.into()]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Replace Results Window Query Output", "Abfrageausgabe des Ergebnisfensters ersetzen")
    }
    fn target(&self) -> Vec<String> { vec!["query_execution_id".into(), "result".into(), "query_error".into()] }
}
