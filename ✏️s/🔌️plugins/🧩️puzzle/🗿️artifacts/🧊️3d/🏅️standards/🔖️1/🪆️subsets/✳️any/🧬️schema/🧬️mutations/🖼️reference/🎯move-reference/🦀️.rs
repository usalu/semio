//! 🧩️ Puzzle3d mutation — `MoveReference`: absolute reposition of a reference plane's pinned origin.
use semio_s_artifact_puzzle_3d_diff::Puzzle3dDiff;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Mutation
/// ↔️ `move-reference` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[dsl(keyword = "move-reference")]
pub struct MoveReference {
    pub id: String,
    pub new_origin: [f64; 3],
}

impl<Op> protocol::MutationKind<Puzzle3dSnapshot, Op> for MoveReference
where
    Op: protocol::Mutation<Puzzle3dSnapshot, Diff = Puzzle3dDiff> + From<MoveReference>,
 {

    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "reference", kind: "move-reference", record: "MovedReference" };



    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Move reference \"{}\"", self.id), &format!("Referenz \"{}\" verschieben", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }

}
impl<Op> protocol::ColdMutationKind<Puzzle3dSnapshot, Op> for MoveReference
where
    Op: protocol::ColdMutation<Puzzle3dSnapshot, ReferenceDiff = Puzzle3dDiff> + From<MoveReference>,
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

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn move_reference<Op: From<MoveReference>>(id: String, new_origin: [f64; 3]) -> Op {
    Op::from(MoveReference { id, new_origin })
}
