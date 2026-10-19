//! 📐 Puzzle5d mutation — `ScaleTargetVolume`: changes a target volume's freeform pose scale.
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::Puzzle5dSnapshot;

//#region 🔖️Mutation
/// 📐 `scale-target-volume` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "scale-target-volume")]
pub struct ScaleTargetVolume {
    pub id: String,
    pub new_scale: Option<crate::standards::v1::subsets::any::schema::document_records::Puzzle5dScale>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn scale_target_volume<Op>(id: String, new_scale: Option<crate::standards::v1::subsets::any::schema::document_records::Puzzle5dScale>) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::scale_target_volume::ScaleTargetVolume>,
{
    Op::from(ScaleTargetVolume { id, new_scale })
}

impl<Op> protocol::MutationKind<Puzzle5dSnapshot, Op> for ScaleTargetVolume
where
    Op: protocol::Mutation<Puzzle5dSnapshot, Diff = Puzzle5dDiff> + From<ScaleTargetVolume>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "scale", entity: "target-volume", kind: "scale-target-volume", record: "ScaledTargetVolume" };



    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Scale target volume \"{}\"", self.id), &format!("Zielvolumen \"{}\" skalieren", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Puzzle5dSnapshot, Op> for ScaleTargetVolume
where
    Op: protocol::ColdMutation<Puzzle5dSnapshot, ReferenceDiff = Puzzle5dDiff> + From<ScaleTargetVolume>,
{
    fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
