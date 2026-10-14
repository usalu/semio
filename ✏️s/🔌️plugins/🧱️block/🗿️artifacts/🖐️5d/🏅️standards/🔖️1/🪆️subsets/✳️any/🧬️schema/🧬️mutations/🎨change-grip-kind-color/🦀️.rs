//! 🎨 Block5d mutation — `ChangeGripKindColor`: a grip-kind catalog row's `color`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;
use semio_s_artifact_block_5d_diff::Block5dDiff;

//#region 🔖️Mutation
/// 🎨 `change-grip-kind-color` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-grip-kind-color")]
pub struct ChangeGripKindColor {
    pub id: String,
    pub new_color: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_grip_kind_color<Op: From<crate::change_grip_kind_color::ChangeGripKindColor>>(id: String, new_color: String) -> Op {
    Op::from(ChangeGripKindColor { id, new_color })
}

impl<Op> protocol::MutationKind<Block5dSnapshot, Op> for ChangeGripKindColor where Op: protocol::Mutation<Block5dSnapshot, Diff = semio_s_artifact_block_5d_diff::Block5dDiff> + From<crate::change_grip_kind_color::ChangeGripKindColor> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "grip-kind", kind: "change-grip-kind-color", record: "ChangedGripKindColor" };

    fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        crate::change_grip_kind_color::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_grip_kind_color::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change grip kind \"{}\" color to \"{}\"", self.id, self.new_color), &format!("Farbe von Griffart \"{}\" auf \"{}\" ändern", self.id, self.new_color))
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
