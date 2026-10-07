//! 🔍️ Puzzle5d mutation — `ScaleSelection3d`: a relative, parametric per-axis scaling of a set of parts and target volumes, each about
//! its own world origin. The gesture's own inputs (which ids, which factors) are the payload, so editing the scaling in
//! history re-derives every scale from whatever base it replays on.
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::standards::v1::subsets::any::schema::mutations::{puzzle5d_selection_items,puzzle5d_selection_triple,Puzzle5dMutation};

use crate::Puzzle5dSnapshot;

//#region 🔖️Mutation
/// 🔍️ `scale-selection3d` payload — part and target-volume ids and the per-axis factors every one of their scales is multiplied by.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "scale-selection3d")]
pub struct ScaleSelection3d {
    pub targets: Vec<String>,
    pub factors: [f64; 3],
}

impl protocol::MutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for ScaleSelection3d {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "scale", entity: "selection", kind: "scale-selection3d", record: "ScaledSelection3d" };

    fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Result<Vec<Puzzle5dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let (factors_en, factors_de) = puzzle5d_selection_triple(self.factors);
        let (en, de) = puzzle5d_selection_items(self.targets.len());
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Scale {en} by {factors_en}"), &format!("{de} um {factors_de} skalieren"))
    }
    fn target(&self) -> Vec<String> {
        self.targets.clone()
    }
}
//#endregion 🔖️Mutation

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn scale_selection_3d(targets: Vec<String>, factors: [f64; 3]) -> Puzzle5dMutation {
    Puzzle5dMutation::ScaleSelection3d(ScaleSelection3d { targets, factors })
}
