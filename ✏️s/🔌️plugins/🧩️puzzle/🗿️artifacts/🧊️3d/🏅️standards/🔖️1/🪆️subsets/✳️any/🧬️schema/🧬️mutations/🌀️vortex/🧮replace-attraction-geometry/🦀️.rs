//! 🧩️ Puzzle3d mutation — `ReplaceAttractionGeometry`: whole-value swap of an attraction's pose-solver connection pose.
use semio_s_artifact_puzzle_3d_diff::Puzzle3dDiff;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Mutation
/// 🔁️ `replace-attraction-geometry` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[dsl(keyword = "replace-attraction-geometry")]
pub struct ReplaceAttractionGeometry {
    pub id: String,
    pub new_gap: f64,
    pub new_shift: f64,
    pub new_rise: f64,
    pub new_rotation: f64,
    pub new_turn: f64,
    pub new_tilt: f64,
    pub new_x: f64,
    pub new_y: f64,
}

impl<Op> protocol::MutationKind<Puzzle3dSnapshot, Op> for ReplaceAttractionGeometry
where
    Op: protocol::Mutation<Puzzle3dSnapshot, Diff = Puzzle3dDiff> + From<ReplaceAttractionGeometry>,
 {

    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "attraction", kind: "replace-attraction-geometry", record: "ReplacedAttractionGeometry" };



    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Replace attraction \"{}\" geometry", self.id), &format!("Geometrie von Anziehung \"{}\" ersetzen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }

}
impl<Op> protocol::ColdMutationKind<Puzzle3dSnapshot, Op> for ReplaceAttractionGeometry
where
    Op: protocol::ColdMutation<Puzzle3dSnapshot, ReferenceDiff = Puzzle3dDiff> + From<ReplaceAttractionGeometry>,
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
pub fn replace_attraction_geometry<Op: From<ReplaceAttractionGeometry>>(geometry: ReplaceAttractionGeometry) -> Op {
    Op::from(geometry)
}
