//! 🧬️ Transparent TiffMutation aggregate.
use crate::schema::diff::{differing_runs, same_block_shape, TiffDiff};
use crate::schema::snapshot::TiffTag;
use crate::TiffSnapshot;

pub use crate::schema::operations::apply_tiff_mutation;

//#region Owners
pub use super::insert_ifd::InsertIfdMutation;
pub use super::remove_ifd::RemoveIfdMutation;
pub use super::remove_tag::RemoveTagMutation;
pub use super::replace_samples::ReplaceSamplesMutation;
pub use super::replace_tag::ReplaceTagMutation;
pub use super::paint_region::PaintRegionMutation;
//#endregion Owners

//#region Aggregate
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[value(tag = "mutation", content = "payload", rename_all = "kebab-case")]
#[mutations(snapshot = TiffSnapshot, diff = TiffDiff, schema = "s.stdio.tiff")]
pub enum TiffMutation {
    InsertIfd(InsertIfdMutation),
    RemoveIfd(RemoveIfdMutation),
    ReplaceTag(ReplaceTagMutation),
    RemoveTag(RemoveTagMutation),
    PaintRegion(PaintRegionMutation),
    ReplaceSamples(ReplaceSamplesMutation),
}

//#endregion Aggregate

//#region Net
/// 🧮️ The tag leaves that carry one directory's entries from `before` to `after`: removals of vanished tags, then replacements of new or changed tags.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn net_tags(ifd_index: usize, before: &[TiffTag], after: &[TiffTag]) -> Vec<TiffMutation> {
    let removed = before.iter().filter(|old| after.iter().all(|new| new.tag != old.tag)).map(|old| TiffMutation::RemoveTag(RemoveTagMutation { ifd_index, tag: old.tag }));
    let replaced = after.iter().filter(|new| before.iter().find(|old| old.tag == new.tag).is_none_or(|old| old.values != new.values)).map(|new| TiffMutation::ReplaceTag(ReplaceTagMutation { ifd_index, tag: new.tag, values: new.values.clone() }));
    removed.chain(replaced).collect()
}

/// 🧮️ The leaves that carry `base` to exactly `next`: per surviving directory its tag edits and sample runs (or a remove/insert pair at the same index when
/// the block geometry itself changed), then the surplus directories removed last first and the missing ones inserted. A changed schema stamp answers nothing.
pub fn net_mutations(base: &TiffSnapshot, next: &TiffSnapshot) -> Vec<TiffMutation> {
    if base.schema != next.schema {
        return Vec::new();
    }
    let shared = base.ifds.len().min(next.ifds.len());
    let mut leaves = Vec::new();
    for (index, (before, after)) in base.ifds.iter().zip(&next.ifds).enumerate() {
        match same_block_shape(&before.blocks, &after.blocks) {
            true => {
                leaves.extend(net_tags(index, &before.entries, &after.entries));
                let runs = before.blocks.iter().zip(&after.blocks).enumerate().flat_map(|(block, (old, new))| differing_runs(block, 0, &old.samples, &new.samples));
                leaves.extend(runs.map(|run| TiffMutation::ReplaceSamples(ReplaceSamplesMutation { ifd_index: index, block: run.block, offset: run.offset, samples: run.samples })));
            }
            false => {
                leaves.push(TiffMutation::RemoveIfd(RemoveIfdMutation { index }));
                leaves.push(TiffMutation::InsertIfd(InsertIfdMutation { index, ifd: after.clone() }));
            }
        }
    }
    leaves.extend((shared..base.ifds.len()).rev().map(|index| TiffMutation::RemoveIfd(RemoveIfdMutation { index })));
    leaves.extend(next.ifds.iter().enumerate().skip(shared).map(|(index, ifd)| TiffMutation::InsertIfd(InsertIfdMutation { index, ifd: ifd.clone() })));
    leaves
}
//#endregion Net
