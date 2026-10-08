//! 🔺️ Diff for `CreateRepresentation`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block3dDiff;
use semio_s_plugin_block::{BlockRepresentationsDelta};

//#region 🔖️Diff
pub fn diff(payload: &super::CreateRepresentation, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if base.representations.iter().any(|item| item.id == payload.representation.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("{} \"{}\" already exists", "representation", payload.representation.id), vec![payload.representation.id.clone()]);
    }
    protocol::MutationOutcome::new(Block3dDiff { representations: Some(BlockRepresentationsDelta { added: vec![payload.representation.clone()], reordered: semio_s_plugin_block::block_insert_order(base.representations.iter().map(|item| item.id.as_str()), &payload.representation.id, payload.index), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
