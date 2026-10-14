//! 🔀️ Block5d mutation — `ChangePartKindVariant`: the part kind's optional `variant`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;
use semio_s_artifact_block_5d_diff::Block5dDiff;

//#region 🔖️Mutation
/// 🔀️ `change-part-kind-variant` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-part-kind-variant")]
pub struct ChangePartKindVariant {
    pub new_variant: Option<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_part_kind_variant<Op: From<crate::change_part_kind_variant::ChangePartKindVariant>>(new_variant: Option<String>) -> Op {
    Op::from(ChangePartKindVariant { new_variant })
}

impl<Op> protocol::MutationKind<Block5dSnapshot, Op> for ChangePartKindVariant where Op: protocol::Mutation<Block5dSnapshot, Diff = semio_s_artifact_block_5d_diff::Block5dDiff> + From<crate::change_part_kind_variant::ChangePartKindVariant> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "part-kind", kind: "change-part-kind-variant", record: "ChangedPartKindVariant" };

    fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        crate::change_part_kind_variant::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_part_kind_variant::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change part kind variant to {:?}", self.new_variant), &format!("Bauteilartvariante auf {:?} ändern", self.new_variant))
    }
}
//#endregion 🔖️Mutation

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
