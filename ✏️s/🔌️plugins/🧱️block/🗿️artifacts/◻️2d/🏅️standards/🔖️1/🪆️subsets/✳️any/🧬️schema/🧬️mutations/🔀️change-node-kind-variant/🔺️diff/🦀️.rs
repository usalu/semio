//! 🔺️ Diff for `ChangeNodeKindVariant`.

use crate::Block2dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block2dDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeNodeKindVariant, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    if payload.new_variant == base.node_kind.variant {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Node kind variant is unchanged.");
    }
    protocol::MutationOutcome::new(Block2dDiff { node_kind: Some(semio_s_plugin_block::BlockKindIdentityPatch { variant: Some(semio_s_plugin_block::BlockOptionalText { value: payload.new_variant.clone() }), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
