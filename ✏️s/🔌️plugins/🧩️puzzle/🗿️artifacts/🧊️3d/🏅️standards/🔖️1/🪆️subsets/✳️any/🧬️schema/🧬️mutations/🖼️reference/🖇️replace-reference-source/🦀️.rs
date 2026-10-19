//! 🧩️ Puzzle3d mutation — `ReplaceReferenceSource`: whole-value swap of a reference's media source (url + media kind together).
use semio_s_artifact_puzzle_3d_diff::Puzzle3dDiff;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Mutation
/// 🔁️ `replace-reference-source` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[dsl(keyword = "replace-reference-source")]
pub struct ReplaceReferenceSource {
    pub id: String,
    pub new_source: semio_s_artifact_puzzle_3d_model::Puzzle3dReferenceSource,
}

impl<Op> protocol::MutationKind<Puzzle3dSnapshot, Op> for ReplaceReferenceSource
where
    Op: protocol::Mutation<Puzzle3dSnapshot, Diff = Puzzle3dDiff> + From<ReplaceReferenceSource>,
 {

    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "reference", kind: "replace-reference-source", record: "ReplacedReferenceSource" };



    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Replace reference \"{}\" source", self.id), &format!("Quelle von Referenz \"{}\" ersetzen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }

}
impl<Op> protocol::ColdMutationKind<Puzzle3dSnapshot, Op> for ReplaceReferenceSource
where
    Op: protocol::ColdMutation<Puzzle3dSnapshot, ReferenceDiff = Puzzle3dDiff> + From<ReplaceReferenceSource>,
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
pub fn replace_reference_source<Op: From<ReplaceReferenceSource>>(id: String, new_source: semio_s_artifact_puzzle_3d_model::Puzzle3dReferenceSource) -> Op {
    Op::from(ReplaceReferenceSource { id, new_source })
}
