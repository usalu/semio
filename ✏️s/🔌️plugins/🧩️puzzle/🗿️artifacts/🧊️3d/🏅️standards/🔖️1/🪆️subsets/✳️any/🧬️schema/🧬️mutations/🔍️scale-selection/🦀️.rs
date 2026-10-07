//! 🔍️ Puzzle3d mutation — `ScaleSelection`: a relative, parametric per-axis scaling of a set of objects
//! and target volumes, each about its own origin. The gesture's own inputs (which ids, which factors) are
//! the payload, so editing the scaling in history re-derives every scale from whatever base it replays on.
use crate::standards::v1::subsets::any::schema::diff::Puzzle3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::{puzzle3d_selection_items,puzzle3d_selection_triple,Puzzle3dMutation};

use crate::Puzzle3dSnapshot;

//#region 🔖️Mutation
/// 🔍️ `scale-selection` payload — object and target-volume ids and the per-axis factors every one of
/// their scales is multiplied by.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "scale-selection")]
pub struct ScaleSelection {
    pub targets: Vec<String>,
    pub factors: [f64; 3],
}

impl protocol::MutationKind<Puzzle3dSnapshot, Puzzle3dMutation> for ScaleSelection {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "scale", entity: "selection", kind: "scale-selection", record: "ScaledSelection" };

    fn diff(&self, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle3dSnapshot) -> Result<Vec<Puzzle3dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let (factors_en, factors_de) = puzzle3d_selection_triple(self.factors);
        let (en, de) = puzzle3d_selection_items(self.targets.len());
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Scale {en} by {factors_en}"), &format!("{de} um {factors_de} skalieren"))
    }
    fn target(&self) -> Vec<String> {
        self.targets.clone()
    }
}
//#endregion 🔖️Mutation

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn scale_selection(targets: Vec<String>, factors: [f64; 3]) -> Puzzle3dMutation {
    Puzzle3dMutation::ScaleSelection(ScaleSelection { targets, factors })
}
