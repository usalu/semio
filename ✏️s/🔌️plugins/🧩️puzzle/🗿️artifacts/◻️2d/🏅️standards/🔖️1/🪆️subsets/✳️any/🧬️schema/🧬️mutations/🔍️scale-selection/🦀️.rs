//! 🔍️ Puzzle2d mutation — `ScaleSelection`: a parametric similarity scale of a set of nodes and target
//! regions about one pivot. Node positions spread from the pivot while a node's own `scale` stays (a
//! node kind's footprint is the kind's, not the layout's); a target region scales its corner AND its
//! extent — the same rule as the editor's `Puzzle2dTransform::Scale`, with the pivot recorded.

use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use crate::standards::v1::subsets::any::schema::mutations::{puzzle2d_selection_items, puzzle2d_selection_number, Puzzle2dMutation};
use crate::Puzzle2dSnapshot;

//#region 🔖️Mutation
/// 🔍️ `scale-selection` payload — node and target-region ids, the pivot, and the positive factor.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "scale-selection")]
pub struct ScaleSelection {
    pub targets: Vec<String>,
    pub pivot_x: f64,
    pub pivot_y: f64,
    pub factor: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn scale_selection(targets: Vec<String>, pivot_x: f64, pivot_y: f64, factor: f64) -> Puzzle2dMutation {
    Puzzle2dMutation::ScaleSelection(ScaleSelection { targets, pivot_x, pivot_y, factor })
}

impl protocol::MutationKind<Puzzle2dSnapshot, Puzzle2dMutation> for ScaleSelection {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "scale", entity: "selection", kind: "scale-selection", record: "ScaledSelection" };

    fn diff(&self, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let (factor_en, factor_de) = puzzle2d_selection_number(self.factor);
        let (en, de) = puzzle2d_selection_items(self.targets.len());
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Scale {en} by a factor of {factor_en}"), &format!("{de} um den Faktor {factor_de} skalieren"))
    }
    fn target(&self) -> Vec<String> {
        self.targets.clone()
    }
}
//#endregion 🔖️Mutation
