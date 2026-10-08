//! 🧭️ The details-pane vocabulary of the GIF89a editor: which JSON-pointer edit raises which concrete kind.
//!
//! [`EDIT_RULES`] names every field a single kind sets, inserts or removes; [`special`] answers the gestures whose kinds are computed from the
//! edit: the four geometry fields of a frame (one kind carries all four), a frame's colour table or plain text and a comment or application
//! extension row (replaced in place as the removal of the row followed by the insertion of its new value), and moves of rows.

use super::*;
use semio_framework_value::{DslValue, FromValue, ToValue};
use semio_s_artifact_stdio_contract::editing::{edited_subtree, Carried, EditRules, EntityRule, InsertRule, RemoveRule, Selector, SnapshotEditError, SnapshotEditEvent};

const ROW: &[Selector] = &[Selector::Index("index")];

/// 📚 Every pointer a GIF89a edit resolves to one kind.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[
        EntityRule::new("/width", "set-screen-size", "width").carrying(&[Carried { payload: "height", pointer: "/height" }]),
        EntityRule::new("/height", "set-screen-size", "height").carrying(&[Carried { payload: "width", pointer: "/width" }]),
        EntityRule::new("/gct", "set-global-color-table", "gct"),
        EntityRule::new("/backgroundColorIndex", "set-background-color-index", "index"),
        EntityRule::new("/pixelAspectRatio", "set-pixel-aspect-ratio", "ratio"),
        EntityRule::new("/loopCount", "set-loop-count", "loopCount"),
        EntityRule::new("/frames/*/indices", "set-frame-pixels", "indices").selecting(ROW),
        EntityRule::new("/frames/*/interlace", "set-frame-interlace", "interlace").selecting(ROW),
        EntityRule::new("/frames/*/delayCs", "set-frame-delay", "delayCs").selecting(ROW),
        EntityRule::new("/frames/*/disposal", "set-frame-disposal", "disposal").selecting(ROW),
        EntityRule::new("/frames/*/transparentIndex", "set-frame-transparency", "transparentIndex").selecting(ROW),
        EntityRule::new("/frames/*/userInput", "set-frame-user-input", "userInput").selecting(ROW),
    ],
    inserts: &[InsertRule::new("/frames", "insert-frame", "frame").at("index"), InsertRule::new("/comments", "insert-comment", "text").at("index"), InsertRule::new("/appExtensions", "add-app-extension", "extension").at("index")],
    removes: &[RemoveRule::by_index("/frames", "remove-frame", "index"), RemoveRule::by_index("/comments", "remove-comment", "index"), RemoveRule::by_index("/appExtensions", "remove-app-extension", "index")],
};

fn refusal(path: &str, message: impl Into<String>) -> SnapshotEditError {
    SnapshotEditError::new("snapshot-edit.schema-invalid", path, message)
}

fn pointer_path(event: &SnapshotEditEvent) -> Option<&str> {
    match event {
        SnapshotEditEvent::SetValue { path, .. } | SnapshotEditEvent::InsertValue { path, .. } | SnapshotEditEvent::RemoveValue { path } | SnapshotEditEvent::RenameKey { path, .. } => Some(path),
        SnapshotEditEvent::MoveValue { .. } | SnapshotEditEvent::ReplaceSource { .. } => None,
    }
}

/// 📍 The list a pointer opens with, the row it names and the segments below the row.
fn row_of<'a>(path: &'a str) -> Option<(&'a str, usize, Vec<&'a str>)> {
    let mut segments = path.split('/').skip(1);
    let list = segments.next()?;
    let row = segments.next()?.parse().ok()?;
    Some((list, row, segments.collect()))
}

fn patched<T: FromValue>(tree: &DslValue, list: &str, row: usize, event: &SnapshotEditEvent, path: &str) -> Result<T, SnapshotEditError> {
    let prefix = format!("/{list}/{row}");
    let DslValue::Object(entries) = tree else { return Err(refusal(path, "the document is not a record")) };
    let DslValue::Array(rows) = &entries.iter().find(|(key, _)| key.as_str() == list).ok_or_else(|| refusal(path, format!("the document has no {list}")))?.1 else { return Err(refusal(path, format!("{list} is not a list"))) };
    let current = rows.get(row).ok_or_else(|| SnapshotEditError::new("snapshot-edit.index-out-of-bounds", path, format!("{list} has no row {row}")))?;
    T::from_value(edited_subtree(current, &prefix, event)?).map_err(|error| refusal(path, error.to_string()))
}

fn replace_frame(index: usize, frame: GifFrame) -> Vec<GifMutation> {
    vec![GifMutation::RemoveFrame(remove_frame::RemoveFrame { index }), GifMutation::InsertFrame(insert_frame::InsertFrame { index, frame })]
}

/// 🎯 The kinds of the gestures [`EDIT_RULES`] cannot carry; `None` hands the edit on to the table.
pub fn special(event: &SnapshotEditEvent, snapshot: &GifSnapshot) -> Result<Option<Vec<GifMutation>>, SnapshotEditError> {
    if let SnapshotEditEvent::MoveValue { from, path } = event {
        let (Some((from_list, from_row, from_rest)), Some((list, row, rest))) = (row_of(from), row_of(path)) else { return Ok(None) };
        if from_list != list || !from_rest.is_empty() || !rest.is_empty() {
            return Ok(None);
        }
        if from_row == row {
            return Ok(Some(Vec::new()));
        }
        return Ok(match list {
            "frames" => Some(vec![GifMutation::MoveFrame(move_frame::MoveFrame { from: from_row, to: row })]),
            "comments" => snapshot.comments.get(from_row).map(|text| {
                vec![GifMutation::RemoveComment(remove_comment::RemoveComment { index: from_row }), GifMutation::InsertComment(insert_comment::InsertComment { index: row, text: text.clone() })]
            }),
            "appExtensions" => snapshot.app_extensions.get(from_row).map(|extension| {
                vec![GifMutation::RemoveAppExtension(remove_app_extension::RemoveAppExtension { index: from_row }), GifMutation::AddAppExtension(add_app_extension::AddAppExtension { index: row, extension: extension.clone() })]
            }),
            _ => None,
        });
    }
    let Some(path) = pointer_path(event) else { return Ok(None) };
    let Some((list, row, rest)) = row_of(path) else { return Ok(None) };
    let tree = snapshot.to_value();
    let whole_row_set = rest.is_empty() && matches!(event, SnapshotEditEvent::SetValue { .. });
    match (list, rest.first().copied()) {
        ("frames", first) => {
            let geometry = matches!(first, Some("left" | "top" | "width" | "height"));
            if !geometry && !matches!(first, Some("lct" | "plainText")) && !(first.is_none() && whole_row_set) {
                return Ok(None);
            }
            let frame: GifFrame = patched(&tree, list, row, event, path)?;
            let base = snapshot.frames.get(row).ok_or_else(|| SnapshotEditError::new("snapshot-edit.index-out-of-bounds", path, format!("frames has no row {row}")))?;
            if *base == frame {
                return Ok(Some(Vec::new()));
            }
            Ok(Some(if geometry {
                vec![GifMutation::SetFrameGeometry(set_frame_geometry::SetFrameGeometry { index: row, left: frame.left, top: frame.top, width: frame.width, height: frame.height })]
            } else {
                replace_frame(row, frame)
            }))
        }
        ("comments", None) if whole_row_set => {
            let text: String = patched(&tree, list, row, event, path)?;
            Ok(Some(if snapshot.comments.get(row) == Some(&text) {
                Vec::new()
            } else {
                vec![GifMutation::RemoveComment(remove_comment::RemoveComment { index: row }), GifMutation::InsertComment(insert_comment::InsertComment { index: row, text })]
            }))
        }
        ("appExtensions", _) if !rest.is_empty() || whole_row_set => {
            let extension: GifAppExtension = patched(&tree, list, row, event, path)?;
            Ok(Some(if snapshot.app_extensions.get(row) == Some(&extension) {
                Vec::new()
            } else {
                vec![GifMutation::RemoveAppExtension(remove_app_extension::RemoveAppExtension { index: row }), GifMutation::AddAppExtension(add_app_extension::AddAppExtension { index: row, extension })]
            }))
        }
        _ => Ok(None),
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
