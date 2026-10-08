//! 🧱 Block3d mutation — `CreateRepresentation`: a new representation (mesh at a LOD/tag combination).

use crate::BlockRepresentation;
use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Block3dMutation;

//#region 🔖️Mutation
/// 🧱 `create-representation` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "create-representation")]
pub struct CreateRepresentation {
    #[dsl(block)]
    pub representation: BlockRepresentation,
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_representation(representation: BlockRepresentation) -> Block3dMutation {
    Block3dMutation::CreateRepresentation(CreateRepresentation { representation, index: None })
}

/// 📍️ Builder — like [`create_representation`] but inserts the row at `index`.
pub fn create_representation_at(representation: BlockRepresentation, index: u32) -> Block3dMutation {
    Block3dMutation::CreateRepresentation(CreateRepresentation { representation, index: Some(index) })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for CreateRepresentation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "representation", kind: "create-representation", record: "CreatedRepresentation" };

    fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block3dSnapshot) -> Result<Vec<Block3dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create representation \"{}\"", self.representation.id), &format!("Repräsentation \"{}\" erstellen", self.representation.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.representation.id.clone()]
    }
}
//#endregion 🔖️Mutation
