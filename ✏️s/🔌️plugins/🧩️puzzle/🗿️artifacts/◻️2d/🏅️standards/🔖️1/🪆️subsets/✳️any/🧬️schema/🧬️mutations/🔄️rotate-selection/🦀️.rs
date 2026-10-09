//! 🔄️ Puzzle2d mutation — `RotateSelection`: a parametric rotation of a set of nodes about one pivot.
//! Positions orbit the pivot and every handle angle turns with its node, so edges keep their geometry —
//! the same rule as the editor's `Puzzle2dTransform::Rotate`, with the pivot recorded instead of derived.

use semio_framework_value::{list::PagedList, paged::PagedUtf8};

use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use crate::standards::v1::subsets::any::schema::mutations::{puzzle2d_selection_items,puzzle2d_selection_number,Puzzle2dMutation};

use crate::Puzzle2dSnapshot;

//#region 🔖️Mutation
/// 🔄️ `rotate-selection` payload — node ids, the pivot, and the counter-clockwise angle in radians.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "rotate-selection")]
pub struct RotateSelection {
    pub targets: PagedList<PagedUtf8<{ usize::MAX }>, { usize::MAX }>,
    pub pivot_x: f64,
    pub pivot_y: f64,
    #[dsl(angle = "rad")]
    pub angle: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn rotate_selection(targets: PagedList<PagedUtf8<{ usize::MAX }>, { usize::MAX }>, pivot_x: f64, pivot_y: f64, angle: f64) -> Puzzle2dMutation {
    Puzzle2dMutation::RotateSelection(RotateSelection { targets, pivot_x, pivot_y, angle })
}

impl protocol::MutationKind<Puzzle2dSnapshot, Puzzle2dMutation> for RotateSelection {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rotate", entity: "selection", kind: "rotate-selection", record: "RotatedSelection" };

    fn diff(&self, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle2dSnapshot) -> Result<Vec<Puzzle2dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let (degrees_en, degrees_de) = puzzle2d_selection_number(self.angle.to_degrees());
        let (en, de) = puzzle2d_selection_items(self.targets.len());
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rotate {en} by {degrees_en}°"), &format!("{de} um {degrees_de}° drehen"))
    }
    fn target(&self) -> Vec<String> {
        self.targets.iter().map(PagedUtf8::to_string_owner).collect()
    }
}
//#endregion 🔖️Mutation
