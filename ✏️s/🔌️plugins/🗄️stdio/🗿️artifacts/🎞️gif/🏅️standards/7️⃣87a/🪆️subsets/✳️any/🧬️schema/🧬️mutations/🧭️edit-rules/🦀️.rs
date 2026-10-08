//! 🧭️ The details-pane vocabulary of the GIF87a editor: which JSON-pointer edit raises which concrete kind.
//!
//! [`EDIT_RULES`] names every field a single kind sets, inserts or removes; [`special`] answers the gestures whose kinds are computed from the
//! edit: the four geometry fields of an image (one kind carries all four), an image's colour table (the image replaced in place as the removal
//! of the row followed by the insertion of its new value) and moves of images.

use super::*;
use semio_framework_value::{DslValue, FromValue, ToValue};
use semio_s_artifact_stdio_contract::editing::{edited_subtree, Carried, EditRules, EntityRule, InsertRule, RemoveRule, Selector, SnapshotEditError, SnapshotEditEvent};

const ROW: &[Selector] = &[Selector::Index("index")];

/// 📚 Every pointer a GIF87a edit resolves to one kind.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[
        EntityRule::new("/width", "set-screen-size", "width").carrying(&[Carried { payload: "height", pointer: "/height" }]),
        EntityRule::new("/height", "set-screen-size", "height").carrying(&[Carried { payload: "width", pointer: "/width" }]),
        EntityRule::new("/gct", "set-global-color-table", "gct"),
        EntityRule::new("/backgroundColorIndex", "set-background-color-index", "index"),
        EntityRule::new("/pixelAspectRatio", "set-pixel-aspect-ratio", "ratio"),
        EntityRule::new("/images/*/indices", "set-image-pixels", "indices").selecting(ROW),
        EntityRule::new("/images/*/interlace", "set-image-interlace", "interlace").selecting(ROW),
    ],
    inserts: &[InsertRule::new("/images", "insert-image", "image").at("index")],
    removes: &[RemoveRule::by_index("/images", "remove-image", "index")],
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

/// 📍 The image a pointer names and the segments below it.
fn image_of(path: &str) -> Option<(usize, Vec<&str>)> {
    let mut segments = path.split('/').skip(1);
    (segments.next()? == "images").then_some(())?;
    let row = segments.next()?.parse().ok()?;
    Some((row, segments.collect()))
}

fn patched_image(tree: &DslValue, row: usize, event: &SnapshotEditEvent, path: &str) -> Result<GifImage, SnapshotEditError> {
    let DslValue::Object(entries) = tree else { return Err(refusal(path, "the document is not a record")) };
    let DslValue::Array(rows) = &entries.iter().find(|(key, _)| key.as_str() == "images").ok_or_else(|| refusal(path, "the document has no images"))?.1 else { return Err(refusal(path, "images is not a list")) };
    let current = rows.get(row).ok_or_else(|| SnapshotEditError::new("snapshot-edit.index-out-of-bounds", path, format!("images has no row {row}")))?;
    GifImage::from_value(edited_subtree(current, &format!("/images/{row}"), event)?).map_err(|error| refusal(path, error.to_string()))
}

/// 🎯 The kinds of the gestures [`EDIT_RULES`] cannot carry; `None` hands the edit on to the table.
pub fn special(event: &SnapshotEditEvent, snapshot: &GifSnapshot) -> Result<Option<Vec<GifMutation>>, SnapshotEditError> {
    if let SnapshotEditEvent::MoveValue { from, path } = event {
        let (Some((from_row, from_rest)), Some((row, rest))) = (image_of(from), image_of(path)) else { return Ok(None) };
        if !from_rest.is_empty() || !rest.is_empty() {
            return Ok(None);
        }
        return Ok(Some(if from_row == row { Vec::new() } else { vec![GifMutation::MoveImage(move_image::MoveImage { from: from_row, to: row })] }));
    }
    let Some(path) = pointer_path(event) else { return Ok(None) };
    let Some((row, rest)) = image_of(path) else { return Ok(None) };
    let geometry = matches!(rest.first().copied(), Some("left" | "top" | "width" | "height"));
    let replacement = matches!(rest.first().copied(), Some("lct")) || (rest.is_empty() && matches!(event, SnapshotEditEvent::SetValue { .. }));
    if !geometry && !replacement {
        return Ok(None);
    }
    let image = patched_image(&snapshot.to_value(), row, event, path)?;
    let base = snapshot.images.get(row).ok_or_else(|| SnapshotEditError::new("snapshot-edit.index-out-of-bounds", path, format!("images has no row {row}")))?;
    if *base == image {
        return Ok(Some(Vec::new()));
    }
    Ok(Some(if geometry {
        vec![GifMutation::SetImageGeometry(set_image_geometry::SetImageGeometry { index: row, left: image.left, top: image.top, width: image.width, height: image.height })]
    } else {
        vec![GifMutation::RemoveImage(remove_image::RemoveImage { index: row }), GifMutation::InsertImage(insert_image::InsertImage { index: row, image })]
    }))
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
