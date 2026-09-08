//! 📊️ Replaces one Jack query execution result in app-local transient state.

use super::{JackTransient, JackTransientMutation};
use crate::ast::QueryResult;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "replace-query-result")]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplaceQueryResult {
    pub execution_id: Option<String>,
    #[dsl(block)]
    pub result: Option<QueryResult>,
    pub error: Option<String>,
}

impl protocol::MutationKind<JackTransient, JackTransientMutation> for ReplaceQueryResult {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "query-result", kind: "replace-query-result", record: "ReplaceQueryResult" };

    fn diff(&self, base: &JackTransient) -> protocol::MutationOutcome<JackTransient> {
        let mut next = base.clone();
        next.query_execution_id.clone_from(&self.execution_id);
        next.result.clone_from(&self.result);
        next.query_error.clone_from(&self.error);
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &JackTransient) -> Vec<JackTransientMutation> {
        vec![Self { execution_id: base.query_execution_id.clone(), result: base.result.clone(), error: base.query_error.clone() }.into()]
    }

    fn label(&self) -> String {
        "Replace Query Result".into()
    }

    fn target(&self) -> Vec<String> {
        vec!["query_execution_id".into(), "result".into(), "query_error".into()]
    }
}
