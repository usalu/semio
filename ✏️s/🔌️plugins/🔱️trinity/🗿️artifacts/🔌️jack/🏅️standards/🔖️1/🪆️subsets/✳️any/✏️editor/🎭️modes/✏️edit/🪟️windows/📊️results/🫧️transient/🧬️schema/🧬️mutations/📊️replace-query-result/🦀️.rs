//! 📊️ Replaces output in the addressed Jack results window.

use super::{JackResultsWindowTransient, JackResultsWindowTransientMutation};
use crate::ast::QueryResult;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "replace-query-result")]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplaceQueryResult {
    pub execution_id: Option<String>,
    pub result: Option<QueryResult>,
    pub error: Option<String>,
}

impl protocol::MutationKind<JackResultsWindowTransient, JackResultsWindowTransientMutation> for ReplaceQueryResult {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "results-window-query-output", kind: "replace-query-result", record: "ReplaceQueryResult" };
    fn diff(&self, base: &JackResultsWindowTransient) -> protocol::MutationOutcome<JackResultsWindowTransient> {
        let mut next = base.clone();
        next.query_execution_id.clone_from(&self.execution_id);
        next.result.clone_from(&self.result);
        next.query_error.clone_from(&self.error);
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &JackResultsWindowTransient) -> Vec<JackResultsWindowTransientMutation> {
        vec![Self { execution_id: base.query_execution_id.clone(), result: base.result.clone(), error: base.query_error.clone() }.into()]
    }
    fn label(&self) -> String { "Replace Results Window Query Output".into() }
    fn target(&self) -> Vec<String> { vec!["query_execution_id".into(), "result".into(), "query_error".into()] }
}
