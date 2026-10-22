//! 📐 Block5d mutation — `ChangePartKindUnit`: the part kind's optional `unit`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;
use semio_s_artifact_block_5d_diff::Block5dDiff;

//#region 🔖️Mutation
/// 📐 `change-part-kind-unit` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-part-kind-unit")]
pub struct ChangePartKindUnit {
    pub new_unit: Option<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_part_kind_unit<Op: From<crate::change_part_kind_unit::ChangePartKindUnit>>(new_unit: Option<String>) -> Op {
    Op::from(ChangePartKindUnit { new_unit })
}

impl<Op> protocol::MutationKind<Block5dSnapshot, Op> for ChangePartKindUnit where Op: protocol::Mutation<Block5dSnapshot, Diff = semio_s_artifact_block_5d_diff::Block5dDiff> + From<crate::change_part_kind_unit::ChangePartKindUnit> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "part-kind", kind: "change-part-kind-unit", record: "ChangedPartKindUnit" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change part kind unit to {:?}", self.new_unit), &format!("Bauteilarteinheit auf {:?} ändern", self.new_unit))
    }
}

impl<Op> protocol::ColdMutationKind<Block5dSnapshot, Op> for ChangePartKindUnit where Op: protocol::ColdMutation<Block5dSnapshot, ReferenceDiff = semio_s_artifact_block_5d_diff::Block5dDiff> + From<crate::change_part_kind_unit::ChangePartKindUnit> {
    fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        crate::change_part_kind_unit::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_part_kind_unit::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
