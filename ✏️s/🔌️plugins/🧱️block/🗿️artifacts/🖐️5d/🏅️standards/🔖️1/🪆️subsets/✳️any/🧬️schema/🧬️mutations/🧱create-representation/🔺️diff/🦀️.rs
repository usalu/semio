//! 🔺️ Diff for `CreateRepresentation`.

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block5dDiff;
use semio_s_plugin_block::{BlockRepresentationsDelta};

//#region 🔖️Diff
pub fn diff(payload: &super::CreateRepresentation, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if base.representations.iter().any(|item| item.id == payload.representation.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("{} \"{}\" already exists", "representation", payload.representation.id), vec![payload.representation.id.clone()]);
    }
    protocol::MutationOutcome::new(Block5dDiff { representations: Some(BlockRepresentationsDelta { added: vec![payload.representation.clone()], reordered: semio_s_plugin_block::block_insert_order(base.representations.iter().map(|item| item.id.as_str()), &payload.representation.id, payload.index), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
