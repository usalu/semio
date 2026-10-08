//! 🔺️ Diff for `AddRepresentationAttribute`.

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block5dDiff;
use semio_s_plugin_block::{BlockRepresentationsDelta, BlockRepresentationsPatchEntry, BlockRepresentationPatch};

//#region 🔖️Diff
pub fn diff(payload: &super::AddRepresentationAttribute, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    let Some(existing) = base.representations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "representation-attribute", payload.id), vec![payload.id.clone()]);
    };
    if existing.attributes.iter().any(|attribute| attribute.key == payload.attribute.key) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("representation attribute \"{}\" already present", payload.attribute.key));
    }
    let patch = BlockRepresentationPatch { attributes_added: vec![payload.attribute.clone()], ..Default::default() };
    protocol::MutationOutcome::new(Block5dDiff { representations: Some(BlockRepresentationsDelta { patched: vec![BlockRepresentationsPatchEntry { id: payload.id.clone(), patch }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
