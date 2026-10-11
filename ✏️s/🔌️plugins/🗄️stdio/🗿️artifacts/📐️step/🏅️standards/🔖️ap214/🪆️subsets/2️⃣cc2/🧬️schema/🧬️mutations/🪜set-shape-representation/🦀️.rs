//! 🪜 `set-shape-representation` -- writes (at `index` when new) or deletes one `*_SHAPE_REPRESENTATION` this class admits; the instance is restored exactly at its position.

use crate::schema::diff::StepDiff;
use crate::standards::v_ap214::engine::ladder;
use crate::standards::v_ap214::engine::ladder::ShapeRepresentationRow;
use crate::standards::v_ap214::subsets::cc2::schema::MAX_RUNG;
use crate::standards::v_ap214::subsets::cc2::schema::mutations::{rejected, restored, StepCc2Mutation, CLASS};
use crate::StepSnapshot;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetShapeRepresentation {
    pub id: u64,
    pub representation: Option<ShapeRepresentationRow>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
}

impl protocol::MutationKind<StepSnapshot, StepCc2Mutation> for SetShapeRepresentation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "shape-representation", kind: "set-shape-representation", record: "SetShapeRepresentation" };

    fn diff(&self, base: &StepSnapshot) -> protocol::MutationOutcome<StepDiff> {
        let result = match &self.representation {
            None => ladder::remove_representation_diff(base, self.id),
            Some(row) => ladder::representation_diff(base, CLASS, MAX_RUNG, self.id, row, self.index),
        };
        match result {
            Ok(diff) => protocol::MutationOutcome::new(diff),
            Err(message) => rejected(message),
        }
    }

    fn inverse(&self, base: &StepSnapshot) -> Result<Vec<StepCc2Mutation>, semio_framework_value::ValueError> {
        Ok(restored(ladder::restore_entity_rows(base, self.id)))
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set shape representation #{}", self.id), &format!("Formrepräsentation #{} setzen", self.id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.to_string()]
    }
}
//#endregion 🔖️Payload
