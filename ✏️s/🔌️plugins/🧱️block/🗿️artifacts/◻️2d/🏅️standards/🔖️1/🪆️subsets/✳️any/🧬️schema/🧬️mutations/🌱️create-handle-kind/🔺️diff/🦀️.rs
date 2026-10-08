//! 🔺️ Diff for `CreateHandleKind`.

use crate::Block2dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::{Block2dDiff, Block2dHandleKindsDelta};

//#region 🔖️Diff
pub fn diff(payload: &super::CreateHandleKind, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    if base.handle_kinds.iter().any(|item| item.id == payload.handle_kind.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("{} \"{}\" already exists", "handle-kind", payload.handle_kind.id), vec![payload.handle_kind.id.clone()]);
    }
    protocol::MutationOutcome::new(Block2dDiff { handle_kinds: Some(Block2dHandleKindsDelta { added: vec![payload.handle_kind.clone()], reordered: semio_s_plugin_block::block_insert_order(base.handle_kinds.iter().map(|item| item.id.as_str()), &payload.handle_kind.id, payload.index), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
