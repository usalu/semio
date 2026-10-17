//! 🔺️ Diff for `AddBlock`.

#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_base::schema::triples::NamedTripleDiff;
#[allow(unused_imports)]
use crate::schema::diff::SemioCadDiff;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioCadSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::support::find_block;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::AddBlock, base: &SemioCadSnapshot) -> protocol::MutationOutcome<SemioCadDiff> {
    if find_block(base, &payload.block.name).is_some() {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Block \"{}\" already exists.", payload.block.name), [payload.block.name.clone()]);
    }
    let super::AddBlock { block, at } = payload;
    protocol::MutationOutcome::new(SemioCadDiff { layers: None, blocks: Some(NamedTripleDiff { removed: Vec::new(), modified: Vec::new(), added: vec![semio_s_artifact_stdio_semio_base::schema::triples::NamedAdded { index: at.map_or(base.blocks.len(), |at| at.min(base.blocks.len())), item: block.clone() }] }), entities: None })
}
//#endregion 🔖️Diff
