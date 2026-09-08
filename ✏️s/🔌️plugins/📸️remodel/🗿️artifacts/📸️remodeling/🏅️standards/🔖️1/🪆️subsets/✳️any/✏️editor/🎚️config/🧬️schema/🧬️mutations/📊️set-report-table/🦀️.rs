//! 📊️ Set Report Table in the remodeling config channel.

use super::{RemodelingConfig, RemodelingConfigMutation, ReplaceConfig};

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-report-table")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetReportTable {
    pub table: String,
}

impl protocol::MutationKind<RemodelingConfig, RemodelingConfigMutation> for SetReportTable {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "report-table", kind: "set-report-table", record: "SetReportTable" };
    fn diff(&self, base: &RemodelingConfig) -> protocol::MutationOutcome<RemodelingConfig> {
        let mut next = base.clone();
        next.report_table = self.table.clone();
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &RemodelingConfig) -> Vec<RemodelingConfigMutation> { vec![RemodelingConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })] }
    fn label(&self) -> String { "Set Report Table".into() }
    fn target(&self) -> Vec<String> { vec!["report-table".into()] }
}
