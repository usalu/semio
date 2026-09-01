//! 🪜️ `set-shape-representation` — one axis of this conformance class, authored as its own mutation leaf.
//! The class-neutral edit is performed by the shared ladder module; this file names the axis and
//! routes to it, so each rule has ONE implementation and every class calls it.

use crate::artifacts::step::StepSnapshot;
use crate::artifacts::step::standards::v_ap214::engine::ladder::ShapeRepresentationRow;
use crate::artifacts::step::standards::v_ap214::engine::ladder::ClassEdit;
use crate::artifacts::step::standards::v_ap214::subsets::cc3::schema::mutations::{class_diff, class_inverse};
use crate::artifacts::step::standards::v_ap214::subsets::cc3::schema::mutations::{StepCc3Mutation};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetShapeRepresentation {
    pub id: u64,
    pub representation: Option<ShapeRepresentationRow>,
}

impl protocol::MutationKind<StepSnapshot, StepCc3Mutation> for SetShapeRepresentation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "shape-representation", kind: "set-shape-representation", record: "SetShapeRepresentation" };

    fn diff(&self, base: &StepSnapshot) -> protocol::MutationOutcome<<StepCc3Mutation as protocol::Mutation<StepSnapshot>>::Diff> {
        class_diff(base, &ClassEdit::Representation { id: self.id, row: self.representation.clone() })
    }
    fn inverse(&self, base: &StepSnapshot) -> Vec<StepCc3Mutation> {
        class_inverse(base, &ClassEdit::Representation { id: self.id, row: self.representation.clone() })
    }
    fn label(&self) -> String {
        format!("Set shape representation #{}", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.to_string()]
    }
}
//#endregion 🔖️Payload
