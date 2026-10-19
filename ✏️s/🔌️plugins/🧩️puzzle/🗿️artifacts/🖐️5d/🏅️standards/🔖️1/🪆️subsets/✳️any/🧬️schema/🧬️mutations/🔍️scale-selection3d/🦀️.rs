//! its own world origin. The gesture's own inputs (which ids, which factors) are the payload, so editing the scaling in
//! 🔍️ Puzzle5d mutation — `ScaleSelection3d`: a relative, parametric per-axis scaling of a set of parts and target volumes, each about
//! history re-derives every scale from whatever base it replays on.
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::standards::v1::subsets::any::schema::mutations::{puzzle5d_selection_items, puzzle5d_selection_triple};

use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;

use crate::Puzzle5dSnapshot;

//#region 🔖️Mutation
/// 🔍️ `scale-selection3d` payload — part and target-volume ids and the per-axis factors every one of their scales is multiplied by.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "scale-selection3d")]
pub struct ScaleSelection3d {
    pub targets: Vec<String>,
    pub factors: [f64; 3],
}

impl<Op> protocol::MutationKind<Puzzle5dSnapshot, Op> for ScaleSelection3d
where
    Op: protocol::Mutation<Puzzle5dSnapshot, Diff = Puzzle5dDiff> + From<ScaleSelection3d> + From<crate::standards::v1::subsets::any::schema::mutations::scale_part_3d::ScalePart3d> + From<crate::standards::v1::subsets::any::schema::mutations::scale_target_volume::ScaleTargetVolume>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "scale", entity: "selection", kind: "scale-selection3d", record: "ScaledSelection3d" };



    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let (factors_en, factors_de) = puzzle5d_selection_triple(self.factors);
        let (en, de) = puzzle5d_selection_items(self.targets.len());
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Scale {en} by {factors_en}"), &format!("{de} um {factors_de} skalieren"))
    }
    fn target(&self) -> Vec<String> {
        self.targets.clone()
    }
}

impl protocol::ColdMutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for ScaleSelection3d {


}

impl protocol::ColdMutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for ScaleSelection3d {
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
pub fn scale_selection_3d<Op>(targets: Vec<String>, factors: [f64; 3]) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::scale_selection_3d::ScaleSelection3d>,
{
    Op::from(ScaleSelection3d { targets, factors })
}
