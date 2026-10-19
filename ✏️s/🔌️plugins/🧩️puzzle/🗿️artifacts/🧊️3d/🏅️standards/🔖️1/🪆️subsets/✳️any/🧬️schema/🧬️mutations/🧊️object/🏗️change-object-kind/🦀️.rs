//! 🧩️ Puzzle3d mutation — `ChangeObjectKind`: changes an object's `object_kind` catalog reference.
use semio_s_artifact_puzzle_3d_diff::Puzzle3dDiff;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Mutation
/// 📦️ `change-object-kind` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-object-kind")]
pub struct ChangeObjectKind {
    pub id: String,
    pub new_object_kind: Option<String>,
}

impl<Op> protocol::MutationKind<Puzzle3dSnapshot, Op> for ChangeObjectKind
where
    Op: protocol::Mutation<Puzzle3dSnapshot, Diff = Puzzle3dDiff> + From<ChangeObjectKind>,
 {

    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "object", kind: "change-object-kind", record: "ChangedObjectKind" };



    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change object \"{}\" kind", self.id), &format!("Art von Objekt \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }

}
impl<Op> protocol::ColdMutationKind<Puzzle3dSnapshot, Op> for ChangeObjectKind
where
    Op: protocol::ColdMutation<Puzzle3dSnapshot, ReferenceDiff = Puzzle3dDiff> + From<ChangeObjectKind>,
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
pub fn change_object_kind<Op: From<ChangeObjectKind>>(id: String, new_object_kind: Option<String>) -> Op {
    Op::from(ChangeObjectKind { id, new_object_kind })
}
