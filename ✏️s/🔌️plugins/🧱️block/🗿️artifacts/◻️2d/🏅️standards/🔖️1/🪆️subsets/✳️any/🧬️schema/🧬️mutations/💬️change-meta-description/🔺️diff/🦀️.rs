//! 🔺️ Diff for `ChangeMetaDescription`.

use crate::Block2dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block2dDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeMetaDescription, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    if payload.new_description == base.meta.description {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Meta description is unchanged.");
    }
    protocol::MutationOutcome::new(Block2dDiff { meta: Some(semio_s_plugin_block::BlockMetaPatch { description: Some(payload.new_description.clone()) }), ..Default::default() })
}
//#endregion 🔖️Diff
