//! 🔄️ Replace Presence in the DAG presence facet.

use super::{DagPresence, DagPresenceMutation};

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", deny_unknown_fields))]
#[dsl(keyword = "replace-presence")]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplacePresence {
    #[dsl(block)]
    pub presence: DagPresence,
}

impl protocol::MutationKind<DagPresence, DagPresenceMutation> for ReplacePresence {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "presence", kind: "replace-presence", record: "ReplacePresence" };
    fn diff(&self, _base: &DagPresence) -> protocol::MutationOutcome<DagPresence> {
        protocol::MutationOutcome::new(self.presence.clone())
    }
    fn inverse(&self, base: &DagPresence) -> Vec<DagPresenceMutation> {
        vec![DagPresenceMutation::ReplacePresence(ReplacePresence { presence: base.clone() })]
    }
    fn label(&self) -> String {
        "Replace Presence".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["presence".into()]
    }
}
