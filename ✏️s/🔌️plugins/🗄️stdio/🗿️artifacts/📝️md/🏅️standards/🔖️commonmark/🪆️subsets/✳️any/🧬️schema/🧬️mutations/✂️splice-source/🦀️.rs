//! ✂️ `splice-source` — authored as its own mutation leaf: the change set an editor made to the CommonMark source text, carried verbatim.
//! It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;
use crate::schema::diff::{MdBlockAdded, MdBlockModified, MdBlocksDiff};
use crate::standards::v_commonmark::subsets::any::io::export::serializers::render_markdown_blocks;

//#region 🔖️Payload
/// ✂️ One range of the edited source: `delete` Unicode scalars at `offset` of the document's CommonMark source (its blocks rendered,
/// blank-line separated) are replaced by `insert`.
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct SourceSplice {
    pub offset: u32,
    pub delete: u32,
    pub insert: String,
}

/// ✂️ The ranges of one edit, ascending and disjoint, in the coordinates of the source they were taken from.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SpliceSource {
    pub(crate) splices: Vec<SourceSplice>,
}
//#endregion 🔖️Payload

//#region ⚙️Semantics
/// 🧮️ The rows of the top-level block sequence one edit means: base indices to remove and to replace, final indices to add.
struct Rows {
    removed: Vec<usize>,
    modified: Vec<(usize, MdBlock)>,
    added: Vec<(usize, MdBlock)>,
}

impl Rows {
    fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}

/// 🪟️ The base blocks `first..=last` one run of ranges rewrites (their neighbours included, so a block that merges with the edited
/// one is reparsed with it) and the blocks the edited source of that run parses to.
struct Window {
    first: usize,
    last: usize,
    blocks: Vec<MdBlock>,
}

impl SpliceSource {
    /// 🧭️ The rows the ranges mean on `base`, or the refusal reason. The edited source is the base source with the ranges applied; only the
    /// blocks the ranges touch (and their neighbours) are reparsed, every other block stays as `base` holds it.
    fn plan(&self, base: &MdSnapshot) -> Result<Rows, String> {
        const SEPARATOR: usize = 2;
        let pieces: Vec<Vec<char>> = base.blocks.iter().map(|block| render_markdown_blocks(std::slice::from_ref(block)).chars().collect()).collect();
        let source: Vec<char> = base.to_text().chars().collect();
        let mut joined: Vec<char> = Vec::with_capacity(source.len());
        for (index, piece) in pieces.iter().enumerate() {
            if index > 0 {
                joined.extend(['\n', '\n']);
            }
            joined.extend(piece);
        }
        if joined != source {
            return Err("the document source is not addressable by ranges: its blocks do not render as blank-line separated pieces".to_string());
        }
        let mut ranges = Vec::with_capacity(self.splices.len());
        let mut previous_end = 0usize;
        for splice in &self.splices {
            let (offset, delete) = (splice.offset as usize, splice.delete as usize);
            if offset < previous_end || offset + delete > source.len() {
                return Err("the splices must ascend, never overlap and stay inside the source".to_string());
            }
            previous_end = offset + delete;
            ranges.push((offset, delete, splice.insert.chars().collect::<Vec<char>>()));
        }
        if ranges.is_empty() {
            return Ok(Rows { removed: Vec::new(), modified: Vec::new(), added: Vec::new() });
        }
        let count = pieces.len();
        if count == 0 {
            let text: String = ranges.iter().flat_map(|(_, _, insert)| insert.iter().copied()).collect();
            let added = MdSnapshot::from_text(&text).blocks.into_iter().enumerate().collect();
            return Ok(Rows { removed: Vec::new(), modified: Vec::new(), added });
        }
        let mut starts = Vec::with_capacity(count);
        let mut cursor = 0usize;
        for piece in &pieces {
            starts.push(cursor);
            cursor += piece.len() + SEPARATOR;
        }
        let end_of = |index: usize| starts[index] + pieces[index].len();
        let span = |position: usize| {
            let block = starts.partition_point(|start| *start <= position) - 1;
            (block, if position > end_of(block) && block + 1 < count { block + 1 } else { block })
        };
        let mut groups: Vec<(usize, usize, usize, usize)> = Vec::new();
        let mut index = 0;
        while index < ranges.len() {
            let first = span(ranges[index].0).0;
            let mut last = span(ranges[index].0).1.max(span(ranges[index].0 + ranges[index].1).1);
            let mut group_end = index;
            while group_end + 1 < ranges.len() && span(ranges[group_end + 1].0).0 <= last {
                group_end += 1;
                last = last.max(span(ranges[group_end].0).1).max(span(ranges[group_end].0 + ranges[group_end].1).1);
            }
            let (first, last) = (first.saturating_sub(1), (last + 1).min(count - 1));
            match groups.last_mut() {
                Some(previous) if first <= previous.1 => {
                    previous.1 = previous.1.max(last);
                    previous.3 = group_end;
                }
                _ => groups.push((first, last, index, group_end)),
            }
            index = group_end + 1;
        }
        let mut windows = Vec::with_capacity(groups.len());
        for (first, last, from, to) in groups {
            let mut text: Vec<char> = Vec::new();
            let mut position = starts[first];
            for (offset, delete, insert) in &ranges[from..=to] {
                text.extend_from_slice(&source[position..*offset]);
                text.extend_from_slice(insert);
                position = offset + delete;
            }
            text.extend_from_slice(&source[position..end_of(last)]);
            windows.push(Window { first, last, blocks: MdSnapshot::from_text(&text.iter().collect::<String>()).blocks });
        }
        let (mut removed, mut modified, mut added) = (Vec::new(), Vec::new(), Vec::new());
        let mut shift: isize = 0;
        for window in windows {
            let old_count = window.last - window.first + 1;
            let paired = old_count.min(window.blocks.len());
            let start = usize::try_from(isize::try_from(window.first).map_err(|error| error.to_string())? + shift).map_err(|error| error.to_string())?;
            for (offset, block) in window.blocks.iter().enumerate() {
                if offset < old_count {
                    if base.blocks[window.first + offset] != *block {
                        modified.push((window.first + offset, block.clone()));
                    }
                } else {
                    added.push((start + offset, block.clone()));
                }
            }
            removed.extend((paired..old_count).map(|offset| window.first + offset));
            shift += isize::try_from(window.blocks.len()).map_err(|error| error.to_string())? - isize::try_from(old_count).map_err(|error| error.to_string())?;
        }
        Ok(Rows { removed, modified, added })
    }
}

impl protocol::MutationKind<MdSnapshot, MdMutation> for SpliceSource {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "source", kind: "splice-source", record: "SpliceSource" };

    fn diff(&self, base: &MdSnapshot) -> protocol::MutationOutcome<<MdMutation as Mutation<MdSnapshot>>::Diff> {
        match self.plan(base) {
            Err(reason) => protocol::MutationOutcome::fatal("mutation.invariant", reason, Vec::<String>::new()),
            Ok(rows) if rows.is_empty() => protocol::MutationOutcome::new(MdDiff::default()),
            Ok(rows) => protocol::MutationOutcome::new(MdDiff {
                blocks: Some(MdBlocksDiff {
                    removed: rows.removed,
                    modified: rows.modified.into_iter().map(|(index, block)| MdBlockModified { index, diff: MdBlockDiff::Replace { block } }).collect(),
                    added: rows.added.into_iter().map(|(index, item)| MdBlockAdded { index, item }).collect(),
                }),
            }),
        }
    }

    /// ↩️ The blocks the ranges rewrote, put back as absolute block setters read from `base`: the added blocks go (last first), the removed
    /// blocks return at their base index (first first), the replaced blocks take their base value. Listed so the last row replays first.
    fn inverse(&self, base: &MdSnapshot) -> Result<Vec<MdMutation>, semio_framework_value::ValueError> {
        let Ok(rows) = self.plan(base) else { return Ok(Vec::new()) };
        let mut undo: Vec<MdMutation> = Vec::new();
        undo.extend(rows.added.iter().rev().map(|(index, _)| MdMutation::RemoveBlock(remove_block::RemoveBlock { path: Vec::new(), index: *index })));
        undo.extend(rows.removed.iter().map(|index| MdMutation::InsertBlock(insert_block::InsertBlock { path: Vec::new(), index: *index, block: base.blocks[*index].clone() })));
        undo.extend(rows.modified.iter().map(|(index, _)| MdMutation::ReplaceBlock(replace_block::ReplaceBlock { path: Vec::new(), index: *index, block: base.blocks[*index].clone() })));
        undo.reverse();
        Ok(undo)
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Edit source", "Quelltext bearbeiten")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion ⚙️Semantics
