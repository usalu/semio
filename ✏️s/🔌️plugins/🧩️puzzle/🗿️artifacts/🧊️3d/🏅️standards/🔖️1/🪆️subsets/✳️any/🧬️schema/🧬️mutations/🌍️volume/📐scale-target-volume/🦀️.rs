//! 🧩️ Puzzle3d mutation — `ScaleTargetVolume`: changes a target volume's freeform pose scale.
use semio_s_artifact_puzzle_3d_diff::Puzzle3dDiff;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Mutation
/// 🔍️ `scale-target-volume` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[dsl(keyword = "scale-target-volume")]
pub struct ScaleTargetVolume {
    pub id: String,
    pub new_scale: Option<semio_s_artifact_puzzle_3d_model::Puzzle3dScale>,
}

impl<Op> protocol::MutationKind<Puzzle3dSnapshot, Op> for ScaleTargetVolume
where
    Op: protocol::Mutation<Puzzle3dSnapshot, Diff = Puzzle3dDiff> + From<ScaleTargetVolume>,
 {

    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "scale", entity: "target-volume", kind: "scale-target-volume", record: "ScaledTargetVolume" };



    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Scale target volume \"{}\"", self.id), &format!("Zielvolumen \"{}\" skalieren", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }

}
impl<Op> protocol::ColdMutationKind<Puzzle3dSnapshot, Op> for ScaleTargetVolume
where
    Op: protocol::ColdMutation<Puzzle3dSnapshot, ReferenceDiff = Puzzle3dDiff> + From<ScaleTargetVolume>,
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
pub fn scale_target_volume<Op: From<ScaleTargetVolume>>(id: String, new_scale: Option<semio_s_artifact_puzzle_3d_model::Puzzle3dScale>) -> Op {
    Op::from(ScaleTargetVolume { id, new_scale })
}
