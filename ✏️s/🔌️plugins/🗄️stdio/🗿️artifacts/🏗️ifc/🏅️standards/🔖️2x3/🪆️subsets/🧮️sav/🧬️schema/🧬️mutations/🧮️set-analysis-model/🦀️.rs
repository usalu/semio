//! 🧮️ `set-analysis-model` -- sets or clears one `IfcStructuralAnalysisModel`; a cleared model is restored at the position it stood.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetAnalysisModel {
    pub id: u64,
    pub model: Option<SavAnalysisModel>,
    pub index: Option<usize>,
}

impl protocol::MutationKind<Ifc2x3Snapshot, Ifc2x3SavMutation> for SetAnalysisModel {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "analysis-model", kind: "set-analysis-model", record: "SetAnalysisModel" };

    fn diff(&self, base: &Ifc2x3Snapshot) -> protocol::MutationOutcome<Ifc2x3Diff> {
        let Self { id, model, index } = self;
        let instance = match model {
            None => None,
            Some(row) => {

                Some(mvd::simple_instance(*id, ANALYSIS_MODEL, analysis_model_args(row)))
            }
        };
        match mvd::entity_diff(base, *id, &[ANALYSIS_MODEL], instance, *index) {
            Ok(diff) => protocol::MutationOutcome::new(diff),
            Err(message) => rejected(message),
        }
    }

    fn inverse(&self, base: &Ifc2x3Snapshot) -> Result<Vec<Ifc2x3SavMutation>, semio_framework_value::ValueError> {
        let Self { id, model, .. } = self;
        Ok(match mvd::standing(base, *id, &[ANALYSIS_MODEL]) {
            mvd::Standing::Foreign => Vec::new(),
            mvd::Standing::Absent if model.is_some() => vec![Ifc2x3SavMutation::SetAnalysisModel(SetAnalysisModel { id: *id, model: None, index: None })],
            mvd::Standing::Absent => Vec::new(),
            mvd::Standing::Present { index } => match analysis_model_row(base, *id) {
                Some(row) => vec![Ifc2x3SavMutation::SetAnalysisModel(SetAnalysisModel { id: *id, model: Some(row), index: Some(index) })],
                None => Vec::new(),
            },
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set analysis model", "Analysemodell setzen")
    }

    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
