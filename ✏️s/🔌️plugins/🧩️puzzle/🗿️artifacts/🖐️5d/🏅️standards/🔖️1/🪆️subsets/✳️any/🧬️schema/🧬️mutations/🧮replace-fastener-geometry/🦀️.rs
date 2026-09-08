//! Puzzle5d mutation — `ReplaceFastenerGeometry`: whole-value swap of a fastener's pose-solver connection pose.
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::Puzzle5dSnapshot;

//#region 🔖️Mutation
/// `replace-fastener-geometry` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
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

impl protocol::MutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for ReplaceFastenerGeometry {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "fastener", kind: "replace-fastener-geometry", record: "ReplacedFastenerGeometry" };

    fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace fastener \"{}\" geometry", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_fastener_geometry(geometry: ReplaceFastenerGeometry) -> Puzzle5dMutation {
    Puzzle5dMutation::ReplaceFastenerGeometry(geometry)
}
