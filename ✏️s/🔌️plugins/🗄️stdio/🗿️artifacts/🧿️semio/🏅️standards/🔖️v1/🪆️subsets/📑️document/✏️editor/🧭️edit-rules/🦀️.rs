//! 🧭️ The document editor's edit rules: which snapshot pointer raises which ONE concrete document mutation kind. An edit no kind expresses is refused, never approximated.

use crate::editor::semio_base::edit_plumbing::{build, complete_by, ent, edited, event_path, fault, field, ins, item, keyed, named, position, record, rem, resolve, segments, unsupported, Entries, Reshape};
use crate::standards::v1::subsets::document::schema::mutations::SemioDocumentMutation;
use crate::standards::v1::subsets::document::schema::snapshot::SemioDocumentSnapshot;
use semio_framework_plugin::Fault;
use semio_framework_value::{DslValue, ToValue};
use semio_s_artifact_stdio_contract::editing::{EditRules, Selector, SnapshotEditEvent};

const ID: Selector = named("id", "id");

/// 📚 Every pointer outside the block tree a document editor edit resolves, one rule per kind that sets, inserts or removes through it.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[
        ent("/styles/*/name", "set-style-name", &[ID], "name"),
        ent("/styles/*/basedOn", "set-style-based-on", &[ID], "based_on"),
        ent("/images/*/mime", "set-image-bytes", &[ID], "mime"),
        ent("/images/*/bytes", "set-image-bytes", &[ID], "bytes"),
    ],
    inserts: &[ins("/styles", "insert-style", &[], Some("at"), "style"), ins("/images", "insert-image", &[], Some("at"), "image")],
    removes: &[rem("/styles", "remove-style", &[], keyed("id", "id")), rem("/images", "remove-image", &[], keyed("id", "id"))],
};

const RESHAPES: &[(&str, Reshape)] = &[("set-image-bytes", image_bytes)];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn image_bytes(tree: &DslValue, entries: &mut Entries) -> Result<(), String> {
    complete_by(tree, entries, "images", "id", &["mime", "bytes"])
}

struct Located<'a> {
    trail: Vec<DslValue>,
    list: &'a [DslValue],
    at: usize,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn rows(value: Option<&DslValue>) -> &[DslValue] {
    match value {
        Some(DslValue::Array(rows)) => rows,
        _ => &[],
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn count(value: usize) -> DslValue {
    DslValue::uint(value as u64)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn tagged(kind: &str, members: impl IntoIterator<Item = (&'static str, DslValue)>) -> DslValue {
    record([("kind", DslValue::String(kind.to_string()))].into_iter().chain(members))
}

/// 🪆️ The container step a pointer takes into block `index`, the block list below it, and the pointer segments the step consumed.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn nested<'a>(block: &'a DslValue, index: usize, rest: &[String]) -> Option<(DslValue, &'a [DslValue], usize)> {
    let number = |at: usize| rest.get(at).and_then(|segment| segment.parse::<usize>().ok());
    let Some(DslValue::String(kind)) = field(block, "kind") else { return None };
    match kind.as_str() {
        "quote" if rest.len() > 1 && rest[0] == "blocks" => Some((tagged("quote", [("blockIndex", count(index))]), rows(field(block, "blocks")), 1)),
        "list" if rest.len() > 3 && rest[0] == "items" && rest[2] == "blocks" => {
            let entry = number(1)?;
            let children = rows(field(item(field(block, "items")?, entry)?, "blocks"));
            Some((tagged("listItem", [("blockIndex", count(index)), ("item", count(entry))]), children, 3))
        }
        "table" if rest.len() > 5 && rest[0] == "rows" && rest[2] == "cells" && rest[4] == "blocks" => {
            let (row, cell) = (number(1)?, number(3)?);
            let children = rows(field(item(field(item(field(block, "rows")?, row)?, "cells")?, cell)?, "blocks"));
            Some((tagged("tableCell", [("blockIndex", count(index)), ("row", count(row)), ("cell", count(cell))]), children, 5))
        }
        _ => None,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn locate<'a>(tree: &'a DslValue, segs: &[String], path: &str) -> Result<Located<'a>, Fault> {
    let mut list = rows(field(tree, "blocks"));
    let mut trail = Vec::new();
    let mut at = 1;
    loop {
        let segment = segs.get(at).ok_or_else(|| unsupported(path, "the edit addresses no block"))?;
        if let Some((step, children, consumed)) = segment.parse::<usize>().ok().and_then(|index| list.get(index).and_then(|block| nested(block, index, &segs[at + 1..]))) {
            trail.push(step);
            list = children;
            at += 1 + consumed;
            continue;
        }
        return Ok(Located { trail, list, at });
    }
}

/// 🧱️ Edits of the block tree resolve to the block kind they address: a whole block is inserted, removed or replaced, a heading level, list order, paragraph style, run text, run style or image block is set; every other edit inside a block replaces that block. Everything else goes through [`EDIT_RULES`].
pub(crate) fn special(event: &SnapshotEditEvent, snapshot: &SemioDocumentSnapshot) -> Result<Option<Vec<SemioDocumentMutation>>, Fault> {
    let path = event_path(event);
    let segs = segments(path)?;
    if segs.first().map(String::as_str) != Some("blocks") {
        return resolve::<SemioDocumentSnapshot, SemioDocumentMutation>(&EDIT_RULES, RESHAPES, snapshot, event).map(Some);
    }
    let tree = snapshot.to_value();
    let located = locate(&tree, &segs, path)?;
    let segment = &segs[located.at];
    let slot = |index: usize| record([("segments", DslValue::Array(located.trail.clone())), ("index", count(index))]);
    let make = |kind: &str, members: Vec<(&str, DslValue)>| build::<SemioDocumentSnapshot, SemioDocumentMutation>(&tree, &[], kind, members.into_iter().map(|(name, value)| (name.to_string(), value)).collect(), path).map(Some);
    if located.at + 1 == segs.len() {
        return match event {
            SnapshotEditEvent::InsertValue { value, .. } => make("insert-block", vec![("path", slot(position(segment, Some(located.list.len()), path)?)), ("block", value.clone())]),
            SnapshotEditEvent::RemoveValue { .. } => make("remove-block", vec![("path", slot(position(segment, None, path)?))]),
            SnapshotEditEvent::SetValue { value, .. } => {
                let index = position(segment, None, path)?;
                if located.list.get(index) == Some(value) {
                    return Ok(Some(Vec::new()));
                }
                make("set-block-content", vec![("path", slot(index)), ("block", value.clone())])
            }
            _ => Err(unsupported(path, "no kind moves or renames a block")),
        };
    }
    let index = position(segment, None, path)?;
    let block = located.list.get(index).ok_or_else(|| fault("snapshot-edit.index-out-of-bounds", path, format!("no block {index}")))?;
    if let SnapshotEditEvent::MoveValue { from, .. } = event {
        if segments(from)?.get(..=located.at) != segs.get(..=located.at) {
            return Err(unsupported(path, "no kind moves a row out of its block"));
        }
    }
    let depth = located.at + 1;
    let next = edited(block, event, depth)?;
    if &next == block {
        return Ok(Some(Vec::new()));
    }
    let replace = || make("set-block-content", vec![("path", slot(index)), ("block", next.clone())]);
    let SnapshotEditEvent::SetValue { value, .. } = event else { return replace() };
    let rest: Vec<&str> = segs[depth..].iter().map(String::as_str).collect();
    let run = |at: &str| at.parse::<usize>().map_err(|_| fault("snapshot-edit.index-invalid", path, format!("'{at}' is not a run position")));
    match rest.as_slice() {
        ["level"] => make("set-heading-level", vec![("path", slot(index)), ("level", value.clone())]),
        ["ordered"] => make("set-list-ordered", vec![("path", slot(index)), ("ordered", value.clone())]),
        ["style_id"] => make("set-paragraph-style", vec![("path", slot(index)), ("style_id", value.clone())]),
        ["runs", at, "text"] => make("set-run-text", vec![("path", slot(index)), ("run_index", count(run(at)?)), ("text", value.clone())]),
        ["runs", at, "style", ..] => {
            let style = field(&next, "runs").and_then(|runs| item(runs, run(at).ok()?)).and_then(|entry| field(entry, "style")).cloned().ok_or_else(|| fault("snapshot-edit.path-missing", path, format!("no run {at}")))?;
            make("set-run-style", vec![("path", slot(index)), ("run_index", count(run(at)?)), ("style", style)])
        }
        ["alt" | "image_id" | "width" | "height"] => {
            let member = |name: &str| field(&next, name).cloned().unwrap_or(DslValue::Null);
            make("set-image-block", vec![("path", slot(index)), ("image_id", member("image_id")), ("alt", member("alt")), ("width", member("width")), ("height", member("height"))])
        }
        _ => replace(),
    }
}
