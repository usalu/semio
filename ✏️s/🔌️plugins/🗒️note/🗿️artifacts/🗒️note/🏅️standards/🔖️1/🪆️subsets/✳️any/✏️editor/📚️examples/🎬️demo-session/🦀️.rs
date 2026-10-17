//! 📚️ Example `demo-session`.

use semio_framework_plugin::ExampleSource;
use semio_framework_ui_locale::LocalizedLabel;

pub const ID: &str = "demo-session";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Demo Session", "Demo-Sitzung")
}
pub const ICON: &str = "play";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🎮️.cmd.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

//#region 🧩️ArtifactItems5
pub mod artifact_items_5 {
use crate::{NoteCamera, NoteSnapshot, NoteTextParagraph, note_text_child_record};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🖋️InkCanvasWire
/// 🖋️ The framework ink-canvas surface's document schema (`InkCanvasHost`'s `InkDocument`, the wgpu
/// `InkDocumentJson`) — the host drops any other schema as "No scene".
pub const INK_CANVAS_DOCUMENT_SCHEMA: &str = "ink.document";

/// 🖼️ Projects persisted Note fields and the local camera onto the framework `ink.document` wire: the
/// schema tag is the surface's own, and every text block's composed `content` record is flattened to
/// the surface's bare `paragraphs` (ticket 26/09/17/NOTE-PLUGIN-END-TO-END — the note schema tag and the
/// nested child record left the canvas empty).
pub fn note_canvas_document_json(document: &NoteSnapshot, camera: &NoteCamera) -> String {
    let mut value = semio_framework_value::ToValue::to_value(document);
    if let semio_framework_value::DslValue::Object(entries) = &mut value {
        for (key, field) in entries.iter_mut() {
            match key.as_str() {
                "schema" => *field = semio_framework_value::DslValue::String(INK_CANVAS_DOCUMENT_SCHEMA.into()),
                "blocks" => ink_wire_blocks_from_note(field),
                _ => {}
            }
        }
        entries.push(("camera".into(), semio_framework_value::ToValue::to_value(camera)));
    }
    semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&value))
}

pub fn entry<'a>(entries: &'a [(String, semio_framework_value::DslValue)], key: &str) -> Option<&'a semio_framework_value::DslValue> {
    entries.iter().find(|(name, _)| name == key).map(|(_, value)| value)
}

pub fn is_kind(entries: &[(String, semio_framework_value::DslValue)], kind: &str) -> bool {
    matches!(entry(entries, "kind"), Some(semio_framework_value::DslValue::String(value)) if value == kind)
}

pub fn ink_wire_blocks_from_note(blocks: &mut semio_framework_value::DslValue) {
    let semio_framework_value::DslValue::Array(items) = blocks else { return };
    for item in items {
        let semio_framework_value::DslValue::Object(entries) = item else { continue };
        if let Some(semio_framework_value::DslValue::String(id)) = entry(entries, "id").cloned() {
            entries.push(("interactionId".into(), semio_framework_value::DslValue::String(format!("note-play-block:{id}"))));
        }
        if is_kind(entries, "text") {
            if let Some(index) = entries.iter().position(|(name, _)| name == "content") {
                let (_, content) = entries.remove(index);
                let paragraphs = match content {
                    semio_framework_value::DslValue::Object(record) => record.into_iter().find(|(name, _)| name == "paragraphs").map(|(_, value)| value),
                    _ => None,
                };
                entries.push(("paragraphs".into(), paragraphs.unwrap_or(semio_framework_value::DslValue::Array(Vec::new()))));
            }
        } else if is_kind(entries, "group") {
            if let Some((_, children)) = entries.iter_mut().find(|(name, _)| name == "children") {
                ink_wire_blocks_from_note(children);
            }
        }
    }
}

/// 🖋️ Inverse of [`note_canvas_document_json`] for one ink-canvas block: a text block's bare
/// `paragraphs` become the composed [`NoteTextChild`] record (content-addressed by block id, so an
/// unchanged block keeps its handle); group children recurse.
pub fn note_block_value_from_ink_wire(block: &mut semio_framework_value::DslValue) -> Result<(), String> {
    let semio_framework_value::DslValue::Object(entries) = block else { return Err("ink block is not a record".into()) };
    if let Some(index) = entries.iter().position(|(name, _)| name == "interactionId") {
        entries.remove(index);
    }
    if is_kind(entries, "text") && entry(entries, "content").is_none() {
        let id = match entry(entries, "id") {
            Some(semio_framework_value::DslValue::String(id)) => id.clone(),
            _ => return Err("ink text block has no id".into()),
        };
        let paragraphs = match entries.iter().position(|(name, _)| name == "paragraphs") {
            Some(index) => <Vec<NoteTextParagraph> as semio_framework_value::FromValue>::from_value(entries.remove(index).1).map_err(|error| error.to_string())?,
            None => Vec::new(),
        };
        entries.push(("content".into(), semio_framework_value::ToValue::to_value(&note_text_child_record(&id, &paragraphs))));
    } else if is_kind(entries, "group") {
        if let Some((_, semio_framework_value::DslValue::Array(children))) = entries.iter_mut().find(|(name, _)| name == "children") {
            for child in children {
                note_block_value_from_ink_wire(child)?;
            }
        }
    }
    Ok(())
}
//#endregion 🖋️InkCanvasWire
}
//#endregion 🧩️ArtifactItems5
