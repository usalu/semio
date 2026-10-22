//! ⬇️ `demote-shape-representation` -- rewrites an over-rung representation onto this class's ceiling type; the prior instance is restored exactly.

use semio_s_artifact_stdio_step_snapshot::standards::v_ap214::subsets::base::schema::diff::StepDiff;
use semio_s_artifact_stdio_step_ladder::standards::v_ap214::subsets::base::io::ladder;
use crate::standards::v_ap214::subsets::cc5::schema::MAX_RUNG;
use crate::standards::v_ap214::subsets::cc5::schema::mutations::{rejected, restored, StepCc5Mutation, CLASS};
use semio_s_artifact_stdio_step_snapshot::standards::v_ap214::subsets::base::schema::snapshot::StepSnapshot;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct DemoteShapeRepresentation {
    pub id: u64,
}

impl protocol::MutationKind<StepSnapshot, StepCc5Mutation> for DemoteShapeRepresentation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "shape-representation", kind: "demote-shape-representation", record: "DemotedShapeRepresentation" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Demote shape representation #{} onto this class's ceiling", self.id), &format!("Formrepräsentation #{} auf die Obergrenze dieser Klasse herabstufen", self.id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.to_string()]
    }
}

impl protocol::ColdMutationKind<StepSnapshot, StepCc5Mutation> for DemoteShapeRepresentation {
    fn diff(&self, base: &StepSnapshot) -> protocol::MutationOutcome<StepDiff> {
        match ladder::demotion_diff(base, CLASS, MAX_RUNG, self.id) {
            Ok(diff) => protocol::MutationOutcome::new(diff),
            Err(message) => rejected(message),
        }
    }
    fn inverse(&self, base: &StepSnapshot) -> Result<Vec<StepCc5Mutation>, semio_framework_value::ValueError> {
        Ok(restored(ladder::restore_entity_rows(base, self.id)))
    }
}
//#endregion 🔖️Payload
