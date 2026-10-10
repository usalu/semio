//! 🗑️ `remove-shape-representation` -- deletes one `*_SHAPE_REPRESENTATION`; the instance is restored exactly at its position.

use crate::schema::diff::StepDiff;
use crate::standards::v_ap214::engine::ladder;
use crate::standards::v_ap214::subsets::cc1::schema::mutations::{rejected, restored, StepCc1Mutation, CLASS};
use crate::StepSnapshot;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveShapeRepresentation {
    pub id: u64,
}

impl protocol::MutationKind<StepSnapshot, StepCc1Mutation> for RemoveShapeRepresentation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "shape-representation", kind: "remove-shape-representation", record: "RemovedShapeRepresentation" };

    fn diff(&self, base: &StepSnapshot) -> protocol::MutationOutcome<StepDiff> {
        match ladder::remove_representation_diff(base, self.id) {
            Ok(diff) => protocol::MutationOutcome::new(diff),
            Err(message) => rejected(message),
        }
    }

    fn inverse(&self, base: &StepSnapshot) -> Result<Vec<StepCc1Mutation>, semio_framework_value::ValueError> {
        Ok(restored(ladder::restore_entity_rows(base, self.id)))
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove shape representation #{}", self.id), &format!("Formrepräsentation #{} entfernen", self.id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.to_string()]
    }
}
//#endregion 🔖️Payload
