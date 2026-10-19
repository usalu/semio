//! 🧩️ Puzzle3d mutation — `CreateReference`: brings a new id-keyed reference into existence.
use crate::delete_reference::mutation::DeleteReference;
use semio_s_artifact_puzzle_3d_diff::Puzzle3dDiff;
use {semio_s_artifact_puzzle_3d_model::Puzzle3dReference, semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot};

//#region 🔖️Mutation
/// ➕️ `create-reference` payload — full initial payload at an optional FINAL-state `index` (`None` appends). A
/// duplicate `reference.id` is a no-op.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[dsl(keyword = "create-reference")]
pub struct CreateReference {
    #[dsl(block)]
    pub reference: Puzzle3dReference,
    pub index: Option<usize>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_reference<Op: From<CreateReference>>(reference: Puzzle3dReference, index: Option<usize>) -> Op {
    Op::from(CreateReference { reference, index })
}

impl<Op> protocol::MutationKind<Puzzle3dSnapshot, Op> for CreateReference
where
    Op: protocol::Mutation<Puzzle3dSnapshot, Diff = Puzzle3dDiff> + From<CreateReference> + From<DeleteReference>,
 {

    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "reference", kind: "create-reference", record: "CreatedReference" };



    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create reference \"{}\"", self.reference.id), &format!("Referenz \"{}\" erstellen", self.reference.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.reference.id.clone()]
    }

}
impl<Op> protocol::ColdMutationKind<Puzzle3dSnapshot, Op> for CreateReference
where
    Op: protocol::ColdMutation<Puzzle3dSnapshot, ReferenceDiff = Puzzle3dDiff> + From<CreateReference> + From<DeleteReference>,
 {
fn diff(&self, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
        super::diff::diff(self, base)
    }
fn inverse(&self, base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
