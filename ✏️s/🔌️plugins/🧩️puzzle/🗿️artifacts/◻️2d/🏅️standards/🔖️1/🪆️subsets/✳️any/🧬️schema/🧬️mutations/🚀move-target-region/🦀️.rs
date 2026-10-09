//! 🚀 Puzzle2d mutation — `MoveTargetRegion`: absolute reposition of a region's minimum corner.

use semio_framework_value::paged::PagedUtf8;

use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use crate::Puzzle2dSnapshot;

//#region 🔖️Mutation
/// 🚀 `move-target-region` payload — absolute reposition of a region's minimum corner.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "move-target-region")]
pub struct MoveTargetRegion {
    pub id: PagedUtf8<{ usize::MAX }>,
    pub new_x: f64,
    pub new_y: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn move_target_region(id: PagedUtf8<{ usize::MAX }>, new_x: f64, new_y: f64) -> Puzzle2dMutation {
    Puzzle2dMutation::MoveTargetRegion(MoveTargetRegion { id, new_x, new_y })
}

impl protocol::MutationKind<Puzzle2dSnapshot, Puzzle2dMutation> for MoveTargetRegion {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "target region", kind: "move-target-region", record: "MovedTargetRegion" };

    fn diff(&self, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle2dSnapshot) -> Result<Vec<Puzzle2dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Move target region \"{}\"", self.id), &format!("Zielregion \"{}\" verschieben", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.to_string_owner()]
    }
}
//#endregion 🔖️Mutation
