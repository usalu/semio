//! 🔺️ Diff for `AddCompatibilityRule`.

use crate::Block2dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block2dDiff;
use semio_s_plugin_block::BlockCompatibilityDelta;

//#region 🔖️Diff
pub fn diff(payload: &super::AddCompatibilityRule, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    if base.compatibility.iter().any(|item| item.id == payload.rule.id) {
        return protocol::MutationOutcome::new(Block2dDiff::default())
            .absorb_messages([protocol::MutationMessage::warning("mutation.no-op", format!("{} \"{}\" already present", "compatibility-rule", payload.rule.id)).at(vec![payload.rule.id.clone()])]);
    }
    protocol::MutationOutcome::new(Block2dDiff { compatibility: Some(BlockCompatibilityDelta { added: vec![payload.rule.clone()], reordered: semio_s_plugin_block::block_insert_order(base.compatibility.iter().map(|item| item.id.as_str()), &payload.rule.id, payload.index), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
