//! ✂️ Direct splice-text mutation owner: the change set an editor made to the text body, carried verbatim.
//#region 🔖️Payload
use crate::schema::diff::{TxtDiff, TxtLineAdded, TxtLineModified, TxtLinesDiff};
use crate::schema::mutation_support::{native_shape_error, native_snapshot_error, native_text_error};
use crate::TxtSnapshot;

/// ✂️ One range of the edited body: `delete` scalars at `offset` (Unicode scalar values of the body as the line ending joins it)
/// are replaced by `insert`.
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct TextSplice {
    pub offset: u32,
    pub delete: u32,
    pub insert: String,
}

/// ✂️ The ranges of one edit, ascending and disjoint, all in the coordinates of the body they were taken from.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct SpliceTextMutation {
    pub splices: Vec<TextSplice>,
}

pub type SpliceTextPayload = SpliceTextMutation;
//#endregion 🔖️Payload

//#region ⚙️Semantics
/// 🪟️ The base lines one run of splices rewrites and the lines that replace them.
struct Window {
    first: usize,
    last: usize,
    lines: Vec<String>,
}

/// ✂️ Splits `text` on the scalar sequence `separator` (always at least one piece).
fn split_scalars(text: &[char], separator: &[char]) -> Vec<String> {
    let mut pieces = Vec::new();
    let mut start = 0;
    let mut cursor = 0;
    while cursor + separator.len() <= text.len() {
        if text[cursor..cursor + separator.len()] == *separator {
            pieces.push(text[start..cursor].iter().collect());
            cursor += separator.len();
            start = cursor;
        } else {
            cursor += 1;
        }
    }
    pieces.push(text[start..].iter().collect());
    pieces
}

impl SpliceTextMutation {
    /// 🧭️ The sparse diff the ranges mean on `base`: the lines they touch are rewritten from the payload and the base body, every
    /// other line is untouched. The refusal reason when the result would leave the native shape.
    fn plan(&self, base: &TxtSnapshot) -> Result<TxtDiff, String> {
        if let Some(reason) = native_snapshot_error(base) {
            return Err(reason);
        }
        let separator: Vec<char> = base.line_ending.as_str().chars().collect();
        let body: Vec<char> = base.to_body().chars().collect();
        let line_count = base.lines.len();
        let virtual_count = (line_count + usize::from(base.trailing_newline)).max(1);
        let length_of = |index: usize| base.lines.get(index).map_or(0, |line| line.chars().count());
        let mut starts = Vec::with_capacity(virtual_count);
        let mut cursor = 0usize;
        for index in 0..virtual_count {
            starts.push(cursor);
            cursor += length_of(index) + separator.len();
        }
        let end_of = |index: usize| starts[index] + length_of(index);
        let span = |position: usize| {
            let line = starts.partition_point(|start| *start <= position) - 1;
            (line, if position > end_of(line) { line + 1 } else { line })
        };
        let mut ranges = Vec::with_capacity(self.splices.len());
        let mut previous_end = 0usize;
        for splice in &self.splices {
            let (offset, delete) = (splice.offset as usize, splice.delete as usize);
            if offset < previous_end || offset + delete > body.len() {
                return Err("the splices must ascend, never overlap and stay inside the text".to_string());
            }
            previous_end = offset + delete;
            ranges.push((offset, delete, splice.insert.chars().collect::<Vec<char>>()));
        }
        let mut windows: Vec<Window> = Vec::new();
        let mut index = 0;
        while index < ranges.len() {
            let first = span(ranges[index].0).0;
            let mut last = span(ranges[index].0).1.max(span(ranges[index].0 + ranges[index].1).1);
            let mut group_end = index;
            while group_end + 1 < ranges.len() && span(ranges[group_end + 1].0).0 <= last {
                group_end += 1;
                last = last.max(span(ranges[group_end].0).1).max(span(ranges[group_end].0 + ranges[group_end].1).1);
            }
            let region_end = end_of(last);
            let mut text: Vec<char> = Vec::new();
            let mut position = starts[first];
            for (offset, delete, insert) in &ranges[index..=group_end] {
                text.extend_from_slice(&body[position..*offset]);
                text.extend_from_slice(insert);
                position = offset + delete;
            }
            text.extend_from_slice(&body[position..region_end]);
            windows.push(Window { first, last, lines: split_scalars(&text, &separator) });
            index = group_end + 1;
        }
        if windows.is_empty() {
            return Ok(TxtDiff::default());
        }
        let replaced: usize = windows.iter().map(|window| window.last - window.first + 1).sum();
        let produced: usize = windows.iter().map(|window| window.lines.len()).sum();
        let after_count = virtual_count - replaced + produced;
        let after_line = |position: usize| -> &str {
            let mut shift: isize = 0;
            for window in &windows {
                let start = usize::try_from(isize::try_from(window.first).unwrap_or(isize::MAX) + shift).unwrap_or(usize::MAX);
                if position < start {
                    break;
                }
                if position < start + window.lines.len() {
                    return window.lines[position - start].as_str();
                }
                shift += isize::try_from(window.lines.len()).unwrap_or(isize::MAX) - isize::try_from(window.last - window.first + 1).unwrap_or(isize::MAX);
            }
            let old = usize::try_from(isize::try_from(position).unwrap_or(isize::MAX) - shift).unwrap_or(usize::MAX);
            base.lines.get(old).map_or("", String::as_str)
        };
        let last_text = after_line(after_count - 1);
        let empty_after = after_count == 1 && last_text.is_empty();
        let trailing_after = last_text.is_empty() && after_count >= 2;
        let lines_after = if empty_after { 0 } else if trailing_after { after_count - 1 } else { after_count };
        let (mut removed, mut modified, mut added) = (Vec::new(), Vec::new(), Vec::new());
        let mut shift: isize = 0;
        for window in &windows {
            let is_tail = window.last == virtual_count - 1;
            let keep = if is_tail && (trailing_after || empty_after) { window.lines.len() - 1 } else { window.lines.len() };
            let old_count = (window.first..=window.last).filter(|line| *line < line_count).count();
            for (offset, text) in window.lines[..keep].iter().enumerate() {
                let final_index = usize::try_from(isize::try_from(window.first).map_err(|error| error.to_string())? + shift).map_err(|error| error.to_string())? + offset;
                if let Some(reason) = native_text_error(text, base.line_ending, final_index + 1 < lines_after || trailing_after) {
                    return Err(reason);
                }
                if offset < old_count {
                    if base.lines[window.first + offset] != *text {
                        modified.push(TxtLineModified { index: window.first + offset, text: text.clone() });
                    }
                } else {
                    added.push(TxtLineAdded { index: final_index, text: text.clone() });
                }
            }
            removed.extend((keep..old_count).map(|offset| window.first + offset));
            shift += isize::try_from(keep).map_err(|error| error.to_string())? - isize::try_from(old_count).map_err(|error| error.to_string())?;
        }
        let last_line_is_empty = lines_after > 0 && after_line(lines_after - 1).is_empty();
        if let Some(reason) = native_shape_error(lines_after, last_line_is_empty, trailing_after, base.line_ending) {
            return Err(reason);
        }
        let lines = (!removed.is_empty() || !modified.is_empty() || !added.is_empty()).then_some(TxtLinesDiff { removed, modified, added });
        Ok(TxtDiff { trailing_newline: (trailing_after != base.trailing_newline).then_some(trailing_after), line_ending: None, lines })
    }

    /// ↩️ The ranges that put the deleted scalars back: each takes out what its forward range inserted and restores the scalars the
    /// forward range deleted, read from the base body, at its place in the edited body.
    fn undo(&self, body: &[char]) -> Vec<TextSplice> {
        let mut shift: i64 = 0;
        self.splices
            .iter()
            .map(|splice| {
                let (offset, delete) = (splice.offset as usize, splice.delete as usize);
                let inserted = splice.insert.chars().count();
                let restored = TextSplice { offset: u32::try_from(i64::from(splice.offset) + shift).unwrap_or(u32::MAX), delete: u32::try_from(inserted).unwrap_or(u32::MAX), insert: body[offset..offset + delete].iter().collect() };
                shift += i64::try_from(inserted).unwrap_or(i64::MAX) - i64::from(splice.delete);
                restored
            })
            .collect()
    }
}

impl protocol::MutationKind<TxtSnapshot, super::TxtMutation> for SpliceTextMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "splice", entity: "text", kind: "splice-text", record: "SpliceText" };

    fn diff(&self, base: &TxtSnapshot) -> protocol::MutationOutcome<TxtDiff> {
        match self.plan(base) {
            Err(reason) => protocol::MutationOutcome::fatal("mutation.invariant", reason, Vec::<String>::new()),
            Ok(diff) => protocol::MutationOutcome::new(diff),
        }
    }

    fn inverse(&self, base: &TxtSnapshot) -> Result<Vec<super::TxtMutation>, semio_framework_value::ValueError> {
        Ok(match self.plan(base) {
            Ok(diff) if diff != TxtDiff::default() => vec![super::TxtMutation::SpliceText(Self { splices: self.undo(&base.to_body().chars().collect::<Vec<char>>()) })],
            _ => Vec::new(),
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Edit Text", "Text bearbeiten")
    }
    fn target(&self) -> Vec<String> {
        vec!["splice-text".to_string()]
    }
}
//#endregion ⚙️Semantics

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
