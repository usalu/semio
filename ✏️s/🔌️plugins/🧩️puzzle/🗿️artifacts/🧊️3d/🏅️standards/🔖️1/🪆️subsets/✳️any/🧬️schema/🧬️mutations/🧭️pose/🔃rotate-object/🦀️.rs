//! 🧩️ Puzzle3d mutation — `RotateObject`: changes an object's orientation quaternion.
use semio_s_artifact_puzzle_3d_diff::Puzzle3dDiff;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Mutation
/// 📐️ `rotate-object` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[dsl(keyword = "rotate-object")]
pub struct RotateObject {
    pub id: String,
    pub new_orientation: Option<[f64; 4]>,
}

impl<Op> protocol::MutationKind<Puzzle3dSnapshot, Op> for RotateObject
where
    Op: protocol::Mutation<Puzzle3dSnapshot, Diff = Puzzle3dDiff> + From<RotateObject>,
 {

    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rotate", entity: "object", kind: "rotate-object", record: "RotatedObject" };



    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rotate object \"{}\"", self.id), &format!("Objekt \"{}\" drehen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }

}
impl<Op> protocol::ColdMutationKind<Puzzle3dSnapshot, Op> for RotateObject
where
    Op: protocol::ColdMutation<Puzzle3dSnapshot, ReferenceDiff = Puzzle3dDiff> + From<RotateObject>,
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
pub fn rotate_object<Op: From<RotateObject>>(id: String, new_orientation: Option<[f64; 4]>) -> Op {
    Op::from(RotateObject { id, new_orientation })
}
