//! ↩️ `restore-entities` -- writes entities to exact absolute values (or removes them) at exact positions in one atomic step -- the undo verb of every conformance repair, which a class cannot express through its own filtered verbs.

use crate::schema::diff::StepDiff;
use crate::standards::v_ap214::engine::ladder;
use crate::standards::v_ap214::engine::ladder::EntityRestore;
use crate::standards::v_ap214::subsets::cc2::schema::mutations::{rejected, restored, StepCc2Mutation};
use crate::StepSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RestoreEntities {
    pub entities: Vec<EntityRestore>,
}

impl protocol::MutationKind<StepSnapshot, StepCc2Mutation> for RestoreEntities {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "entities", kind: "restore-entities", record: "RestoredEntities" };

    fn diff(&self, base: &StepSnapshot) -> protocol::MutationOutcome<StepDiff> {
        match ladder::restore_diff(base, &self.entities) {
            Ok(diff) => protocol::MutationOutcome::new(diff),
            Err(message) => rejected(message),
        }
    }

    fn inverse(&self, base: &StepSnapshot) -> Result<Vec<StepCc2Mutation>, semio_framework_value::ValueError> {
        Ok(restored(self.entities.iter().flat_map(|row| ladder::restore_entity_rows(base, row.id)).collect()))
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Restore entities", "Entitäten wiederherstellen")
    }

    fn target(&self) -> Vec<String> {
        self.entities.iter().map(|row| row.id.to_string()).collect()
    }
}
//#endregion 🔖️Payload
