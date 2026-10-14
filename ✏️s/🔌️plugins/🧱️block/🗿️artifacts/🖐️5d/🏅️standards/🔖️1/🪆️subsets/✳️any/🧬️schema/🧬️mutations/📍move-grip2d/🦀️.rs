//! 📍 Block5d mutation — `MoveGrip2d`: a grip's 2D polar position (`angle`+`radius2d` together).

use semio_s_artifact_block_5d_model::Block5dSnapshot;
use semio_s_artifact_block_5d_diff::Block5dDiff;

//#region 🔖️Mutation
/// 📍 `move-grip-2d` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "move-grip-2d")]
pub struct MoveGrip2d {
    pub id: String,
    pub new_angle: f64,
    pub new_radius_2d: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn move_grip_2d<Op: From<crate::move_grip_2d::MoveGrip2d>>(id: String, new_angle: f64, new_radius_2d: f64) -> Op {
    Op::from(MoveGrip2d { id, new_angle, new_radius_2d })
}

impl<Op> protocol::MutationKind<Block5dSnapshot, Op> for MoveGrip2d where Op: protocol::Mutation<Block5dSnapshot, Diff = semio_s_artifact_block_5d_diff::Block5dDiff> + From<crate::move_grip_2d::MoveGrip2d> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "grip", kind: "move-grip2d", record: "MovedGrip2d" };

    fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        crate::move_grip_2d::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::move_grip_2d::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Move grip \"{}\" (2D)", self.id), &format!("Griff \"{}\" (2D) verschieben", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
