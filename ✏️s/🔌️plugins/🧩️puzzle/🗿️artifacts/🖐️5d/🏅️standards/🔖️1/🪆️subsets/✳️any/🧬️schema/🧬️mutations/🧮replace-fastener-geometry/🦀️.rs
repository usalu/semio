//! 🧩️ Puzzle5d mutation — `ReplaceFastenerGeometry`: whole-value swap of a fastener's pose-solver connection pose.
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::Puzzle5dSnapshot;

//#region 🔖️Mutation
/// 🔁️ `replace-fastener-geometry` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "replace-fastener-geometry")]
pub struct ReplaceFastenerGeometry {
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

impl<Op> protocol::MutationKind<Puzzle5dSnapshot, Op> for ReplaceFastenerGeometry
where
    Op: protocol::Mutation<Puzzle5dSnapshot, Diff = Puzzle5dDiff> + From<ReplaceFastenerGeometry>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "fastener", kind: "replace-fastener-geometry", record: "ReplacedFastenerGeometry" };



    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Replace fastener \"{}\" geometry", self.id), &format!("Geometrie von Verbindungsmittel \"{}\" ersetzen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Puzzle5dSnapshot, Op> for ReplaceFastenerGeometry
where
    Op: protocol::ColdMutation<Puzzle5dSnapshot, ReferenceDiff = Puzzle5dDiff> + From<ReplaceFastenerGeometry>,
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
pub fn replace_fastener_geometry<Op>(geometry: ReplaceFastenerGeometry) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::replace_fastener_geometry::ReplaceFastenerGeometry>,
{
    Op::from(geometry)
}
