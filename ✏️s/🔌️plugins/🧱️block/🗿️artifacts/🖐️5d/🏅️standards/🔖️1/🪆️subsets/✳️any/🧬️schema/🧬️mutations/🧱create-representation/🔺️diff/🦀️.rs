//! 🔺️ Diff for `CreateRepresentation`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;
use semio_s_artifact_block_5d_diff::Block5dDiff;
use semio_s_plugin_block::{BlockRepresentationsDelta};

//#region 🔖️Diff
pub fn diff(payload: &crate::create_representation::CreateRepresentation, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if base.representations.iter().any(|item| item.id == payload.representation.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("{} \"{}\" already exists", "representation", payload.representation.id), vec![payload.representation.id.clone()]);
    }
    protocol::MutationOutcome::new(Block5dDiff { representations: BlockRepresentationsDelta::insertion(semio_s_plugin_block::block_insert_index(base.representations.len(), payload.index), payload.representation.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
