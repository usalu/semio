//! ↩️ `restore-entities` -- writes entities to exact absolute values (or removes them) at exact positions in one atomic step -- the undo verb of every conformance repair, which a class cannot express through its own filtered verbs.

use semio_s_artifact_stdio_step_snapshot::standards::v_ap214::subsets::base::schema::diff::StepDiff;
use semio_s_artifact_stdio_step_ladder::standards::v_ap214::subsets::base::io::ladder;
use semio_s_artifact_stdio_step_ladder::standards::v_ap214::subsets::base::io::ladder::EntityRestore;
use crate::standards::v_ap214::subsets::cc5::schema::mutations::{rejected, restored, StepCc5Mutation};
use semio_s_artifact_stdio_step_snapshot::standards::v_ap214::subsets::base::schema::snapshot::StepSnapshot;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct RestoreEntities {
    pub entities: Vec<EntityRestore>,
}

impl protocol::MutationKind<StepSnapshot, StepCc5Mutation> for RestoreEntities {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "entities", kind: "restore-entities", record: "RestoredEntities" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Restore entities", "Entitäten wiederherstellen")
    }

    fn target(&self) -> Vec<String> {
        self.entities.iter().map(|row| row.id.to_string()).collect()
    }
}

impl protocol::ColdMutationKind<StepSnapshot, StepCc5Mutation> for RestoreEntities {
    fn diff(&self, base: &StepSnapshot) -> protocol::MutationOutcome<StepDiff> {
        match ladder::restore_diff(base, &self.entities) {
            Ok(diff) => protocol::MutationOutcome::new(diff),
            Err(message) => rejected(message),
        }
    }
    fn inverse(&self, base: &StepSnapshot) -> Result<Vec<StepCc5Mutation>, semio_framework_value::ValueError> {
        Ok(restored(self.entities.iter().flat_map(|row| ladder::restore_entity_rows(base, row.id)).collect()))
    }
}
//#endregion 🔖️Payload
