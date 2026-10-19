//! 🧩️ Puzzle5d mutation — `ScalePart3d`: changes a part's 3D-projection freeform scale.
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::Puzzle5dSnapshot;

//#region 🔖️Mutation
/// 🔍️ `scale-part3d` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "scale-part3d")]
pub struct ScalePart3d {
    pub id: String,
    pub new_scale: Option<crate::standards::v1::subsets::any::schema::document_records::Puzzle5dScale>,
}

impl<Op> protocol::MutationKind<Puzzle5dSnapshot, Op> for ScalePart3d
where
    Op: protocol::Mutation<Puzzle5dSnapshot, Diff = Puzzle5dDiff> + From<ScalePart3d>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "scale", entity: "part", kind: "scale-part3d", record: "ScaledPart3d" };



    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Scale part \"{}\" (3d)", self.id), &format!("Bauteil \"{}\" (3D) skalieren", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Puzzle5dSnapshot, Op> for ScalePart3d
where
    Op: protocol::ColdMutation<Puzzle5dSnapshot, ReferenceDiff = Puzzle5dDiff> + From<ScalePart3d>,
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

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn scale_part_3d<Op>(id: String, new_scale: Option<crate::standards::v1::subsets::any::schema::document_records::Puzzle5dScale>) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::scale_part_3d::ScalePart3d>,
{
    Op::from(ScalePart3d { id, new_scale })
}
