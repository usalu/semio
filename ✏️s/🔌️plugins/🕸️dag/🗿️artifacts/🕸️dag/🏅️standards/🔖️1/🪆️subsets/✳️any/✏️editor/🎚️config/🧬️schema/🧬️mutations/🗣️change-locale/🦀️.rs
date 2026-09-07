//! 🗣️ Change Locale in the DAG config facet.

use super::{DagConfig, DagConfigMutation};

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", deny_unknown_fields))]
#[dsl(keyword = "change-locale")]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeLocale {
    pub value: String
}

impl protocol::MutationKind<DagConfig, DagConfigMutation> for ChangeLocale {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "locale", kind: "change-locale", record: "ChangeLocale" };
    fn diff(&self, base: &DagConfig) -> protocol::MutationOutcome<DagConfig> { protocol::MutationOutcome::new(DagConfig { locale: self.value.clone(), ..base.clone() }) }
    fn inverse(&self, base: &DagConfig) -> Vec<DagConfigMutation> { vec![DagConfigMutation::ChangeLocale(ChangeLocale { value: base.locale.clone() })] }
    fn label(&self) -> String { "Change Locale".into() }
    fn target(&self) -> Vec<String> { vec!["locale".into()] }
}
