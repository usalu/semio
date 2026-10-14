//! 🧱 Block5d mutation — `CreateRepresentation`: a new representation (mesh at a LOD/tag combination).

use semio_s_plugin_block::BlockRepresentation;
use semio_s_artifact_block_5d_model::Block5dSnapshot;
use semio_s_artifact_block_5d_diff::Block5dDiff;

//#region 🔖️Mutation
/// 🧱 `create-representation` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
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
pub fn create_representation<Op: From<crate::create_representation::CreateRepresentation>>(representation: BlockRepresentation) -> Op {
    Op::from(CreateRepresentation { representation, index: None })
}

/// 📍️ Builder — like [`create_representation`] but inserts the row at `index`.
pub fn create_representation_at<Op: From<crate::create_representation::CreateRepresentation>>(representation: BlockRepresentation, index: u32) -> Op {
    Op::from(CreateRepresentation { representation, index: Some(index) })
}

impl<Op> protocol::MutationKind<Block5dSnapshot, Op> for CreateRepresentation where Op: protocol::Mutation<Block5dSnapshot, Diff = semio_s_artifact_block_5d_diff::Block5dDiff> + From<crate::delete_representation::DeleteRepresentation> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "representation", kind: "create-representation", record: "CreatedRepresentation" };

    fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        crate::create_representation::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::create_representation::inverse::inverse(self, base)?
    
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

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
