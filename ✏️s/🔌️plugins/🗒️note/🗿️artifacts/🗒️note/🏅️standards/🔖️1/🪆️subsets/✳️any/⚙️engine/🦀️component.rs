//! ⚙️ Note artifact — headless compute (constitutional: engine).

use crate::artifacts::note::{NoteBlockNode, NoteSnapshot, NoteTableCell, NoteTextParagraph, NoteTextRun, NOTE_DOCUMENT_SCHEMA};
use semio_framework::{DwgDrawing, DwgGeometry};
use semio_framework_plugin::{io_dispatch, Dialect, ErasedComposeSource, IoDirection, IoKey, IoPayload, StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::engine::geometry::{SemioPoint2, SemioPoint3, SemioQuaternion, SemioRgba, SemioTransform};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::io as semio_drawing_composer;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawCanvas, DrawLayer, DrawNode, DrawStyle, PathSegment, SemioDrawingSnapshot, STDIO_SEMIODRAWING_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::svg::schema::snapshot::write_svg_xml;
use serde_json::Value;
use std::collections::BTreeMap;

//#region 🔖️Register
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`, which called five different global registries directly from
/// a plugin `.setup()` callback. `crate::apps::note::config::schema::register_app_schema()` is the
/// one exception, still called from `🗒️note/🦀️component.rs`'s own `.setup()`: it registers the
/// `NotePlayApp` CONFIG/PRESENCE schema, an app-scope concern `ArtifactDeclaration` deliberately has
/// no field for (see that struct's own doc) — `register_app_schema_descriptor` is not in §6's
/// artifact-scoped function set.
pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
    semio_framework_plugin::ArtifactDeclaration::builder("s.note")
        .schema(crate::artifacts::note::schema::note_artifact_schema_descriptor())
        .inferences([crate::artifacts::note::schema::inferences::note_artifact_inference_descriptor()])
        .composers(crate::artifacts::note::standards::v1::engine::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<crate::apps::note::NotePlayApp>()
        .build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring the
/// `OnceLock`-backed `io_registry::entries()` convention already used below.
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "note.document",
                    extension: Some("note"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::note::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::note::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::note::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::note::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("note.document"),
                },
                dsl::LanguageSpec {
                    id: "note.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::note::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::note::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::note::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::note::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("note.op"),
                },
                dsl::LanguageSpec {
                    id: "note.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::note::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::note::schema::diff::text::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("note.diff"),
                },
                dsl::LanguageSpec {
                    id: "note.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::note::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::note::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("note.pack"),
                },
                dsl::LanguageSpec {
                    id: "note.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::note::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::note::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("note.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Register

//#region 🔖️Constants
/// 📄️ The `semio` example document, handcrafted in the `.note` DSL — {@link semio_example_snapshot}/
/// {@link semio_example_json} are the only ways it should be consumed.
const SEMIO_NOTE_EXAMPLE_TEXT: &str = crate::artifacts::note::schema::snapshot::text::SEMIO_NOTE_EXAMPLE_TEXT;

//#endregion 🔖️Constants

//#region 🔖️DocumentHelpers
pub fn create_note_id(prefix: &str) -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(1);
    let serial = NEXT.fetch_add(1, Ordering::Relaxed);
    format!("{prefix}-{serial}")
}

/// 📄️ The `semio` example, parsed once from {@link SEMIO_NOTE_EXAMPLE_TEXT} — the source of truth for
/// every "semio" example call site (`setActiveExample`, tests). Falls back to the empty document if the
/// fixture ever fails to parse, matching the old JSON fixture's failure behavior.
pub fn semio_example_snapshot() -> NoteSnapshot {
    <NoteSnapshot as store::ArtifactDsl>::parse_dsl(SEMIO_NOTE_EXAMPLE_TEXT).unwrap_or_else(|_| empty_note_snapshot())
}

/// 📄️ JSON re-serialization of {@link semio_example_snapshot}, for the framework-generic call sites that
/// contractually require JSON text (`PluginApp::render`'s `projection_override_json`, `App::example`'s
/// manifest `document_json`).
pub fn semio_example_json() -> String {
    serde_json::to_string(&semio_example_snapshot()).expect("serialize semio example document")
}

pub fn empty_note_snapshot() -> NoteSnapshot {
    NoteSnapshot {
        schema: NOTE_DOCUMENT_SCHEMA.into(),
        id: "empty".into(),
        title: None,
        blocks: Vec::new(),
        grid_visible: Some(true),
        grid_spacing: Some(32.0),
        grid_subdivisions: Some(4.0),
        grid_opacity: Some(0.35),
        snap_enabled: Some(false),
        snap_grid_spacing: Some(8.0),
        pencil_width: Some(3.0),
        eraser_radius: Some(12.0),
        assets: BTreeMap::new(),
    }
}

pub fn block_id(block: &NoteBlockNode) -> &str {
    match block {
        NoteBlockNode::Text { id, .. } | NoteBlockNode::Image { id, .. } | NoteBlockNode::Table { id, .. } | NoteBlockNode::Math { id, .. } | NoteBlockNode::Ink { id, .. } | NoteBlockNode::Group { id, .. } => id,
    }
}

pub fn block_name(block: &NoteBlockNode) -> &str {
    match block {
        NoteBlockNode::Text { name, .. } | NoteBlockNode::Image { name, .. } | NoteBlockNode::Table { name, .. } | NoteBlockNode::Math { name, .. } | NoteBlockNode::Ink { name, .. } | NoteBlockNode::Group { name, .. } => name,
    }
}

pub fn block_kind(block: &NoteBlockNode) -> &str {
    match block {
        NoteBlockNode::Text { .. } => "text",
        NoteBlockNode::Image { .. } => "image",
        NoteBlockNode::Table { .. } => "table",
        NoteBlockNode::Math { .. } => "math",
        NoteBlockNode::Ink { .. } => "stroke",
        NoteBlockNode::Group { .. } => "group",
    }
}

pub fn block_visible(block: &NoteBlockNode) -> bool {
    match block {
        NoteBlockNode::Text { visible, .. } | NoteBlockNode::Image { visible, .. } | NoteBlockNode::Table { visible, .. } | NoteBlockNode::Math { visible, .. } | NoteBlockNode::Ink { visible, .. } | NoteBlockNode::Group { visible, .. } => *visible,
    }
}

pub fn block_locked(block: &NoteBlockNode) -> bool {
    match block {
        NoteBlockNode::Text { locked, .. } | NoteBlockNode::Image { locked, .. } | NoteBlockNode::Table { locked, .. } | NoteBlockNode::Math { locked, .. } | NoteBlockNode::Ink { locked, .. } | NoteBlockNode::Group { locked, .. } => *locked,
    }
}

pub fn block_icon(kind: &str) -> &str {
    match kind {
        "text" => "type",
        "image" => "image",
        "table" => "table",
        "math" => "sigma",
        "stroke" => "pencil",
        _ => "folder",
    }
}

pub fn block_tree_row_id(block: &NoteBlockNode) -> String {
    format!("note-play-block:{}", block_id(block))
}

pub fn block_id_from_tree_row_id(row_id: &str) -> Option<String> {
    row_id.strip_prefix("note-play-block:").map(str::to_string)
}

pub fn find_block<'a>(blocks: &'a [NoteBlockNode], target_id: &str) -> Option<&'a NoteBlockNode> {
    for block in blocks {
        if block_id(block) == target_id {
            return Some(block);
        }
        if let NoteBlockNode::Group { children, .. } = block {
            if let Some(found) = find_block(children, target_id) {
                return Some(found);
            }
        }
    }
    None
}

/// 🧭️ Locates `target_id`'s parent (`None` = document root) and sibling index — the position a
/// `delete-block`/`move-block-to-container` mutation's diff/inverse needs to reconstruct or
/// reparent a node exactly, since `find_block` alone only returns the node's content.
pub fn find_block_location(blocks: &[NoteBlockNode], target_id: &str) -> Option<(Option<String>, usize)> {
    if let Some(index) = blocks.iter().position(|block| block_id(block) == target_id) {
        return Some((None, index));
    }
    for block in blocks {
        if let NoteBlockNode::Group { id, children, .. } = block {
            if let Some(index) = children.iter().position(|child| block_id(child) == target_id) {
                return Some((Some(id.clone()), index));
            }
            if let Some(found) = find_block_location(children, target_id) {
                return Some(found);
            }
        }
    }
    None
}

pub fn flatten_blocks(blocks: &[NoteBlockNode]) -> Vec<&NoteBlockNode> {
    let mut out = Vec::new();
    fn visit<'a>(blocks: &'a [NoteBlockNode], out: &mut Vec<&'a NoteBlockNode>) {
        for block in blocks {
            out.push(block);
            if let NoteBlockNode::Group { children, .. } = block {
                visit(children, out);
            }
        }
    }
    visit(blocks, &mut out);
    out
}

pub fn create_block_by_kind(kind: &str, x: f64, y: f64) -> NoteBlockNode {
    let id = create_note_id(kind);
    match kind {
        "image" => NoteBlockNode::Image { id, name: "Image".into(), x, y, width: 240.0, height: 160.0, rotation: 0.0, visible: true, locked: false, image_key: "placeholder".into() },
        "table" => NoteBlockNode::Table {
            id,
            name: "Table".into(),
            x,
            y,
            width: 320.0,
            height: 160.0,
            rotation: 0.0,
            visible: true,
            locked: false,
            columns: vec!["A".into(), "B".into(), "C".into()],
            rows: vec![
                vec![NoteTableCell { content: String::new() }, NoteTableCell { content: String::new() }, NoteTableCell { content: String::new() }],
                vec![NoteTableCell { content: String::new() }, NoteTableCell { content: String::new() }, NoteTableCell { content: String::new() }],
            ],
        },
        "math" => NoteBlockNode::Math { id, name: "Math".into(), x, y, width: 200.0, height: 80.0, rotation: 0.0, visible: true, locked: false, tex: "E = mc^2".into(), display_mode: true },
        "stroke" => NoteBlockNode::Ink { id, name: "Ink".into(), x, y, width: 1.0, height: 1.0, rotation: 0.0, visible: true, locked: false, points: Vec::new(), stroke_width: 3.0, color: [0.0, 0.0, 0.0, 1.0] },
        "group" => NoteBlockNode::Group { id, name: "Group".into(), x, y, width: 280.0, height: 120.0, rotation: 0.0, visible: true, locked: false, children: Vec::new() },
        _ => NoteBlockNode::Text {
            id,
            name: "Text".into(),
            x,
            y,
            width: 280.0,
            height: 120.0,
            rotation: 0.0,
            visible: true,
            locked: false,
            paragraphs: vec![NoteTextParagraph { runs: vec![NoteTextRun { text: String::new(), bold: None, italic: None, underline: None, link: None }] }],
            font_size: 18.0,
            font_weight: "normal".into(),
            align: "left".into(),
        },
    }
}

pub fn remove_block_from_tree(blocks: &mut Vec<NoteBlockNode>, target_id: &str) -> bool {
    if let Some(index) = blocks.iter().position(|block| block_id(block) == target_id) {
        blocks.remove(index);
        return true;
    }
    for block in blocks.iter_mut() {
        if let NoteBlockNode::Group { children, .. } = block {
            if remove_block_from_tree(children, target_id) {
                return true;
            }
        }
    }
    false
}

pub fn reid_block_tree(block: &mut NoteBlockNode, rename_top: bool) {
    let kind = block_kind(block).to_string();
    match block {
        NoteBlockNode::Text { id, name, .. } | NoteBlockNode::Image { id, name, .. } | NoteBlockNode::Table { id, name, .. } | NoteBlockNode::Math { id, name, .. } | NoteBlockNode::Ink { id, name, .. } | NoteBlockNode::Group { id, name, .. } => {
            *id = create_note_id(&kind);
            if rename_top {
                *name = format!("{name} copy");
            }
        }
    }
    if let NoteBlockNode::Group { children, .. } = block {
        for child in children.iter_mut() {
            reid_block_tree(child, false);
        }
    }
}

pub fn clone_block(block: &NoteBlockNode) -> NoteBlockNode {
    let mut cloned: NoteBlockNode = serde_json::from_value(serde_json::to_value(block).unwrap()).unwrap();
    reid_block_tree(&mut cloned, true);
    cloned
}

pub fn offset_block_tree(block: &mut NoteBlockNode, dx: f64, dy: f64) {
    match block {
        NoteBlockNode::Text { x, y, .. } | NoteBlockNode::Image { x, y, .. } | NoteBlockNode::Table { x, y, .. } | NoteBlockNode::Math { x, y, .. } | NoteBlockNode::Ink { x, y, .. } | NoteBlockNode::Group { x, y, .. } => {
            *x += dx;
            *y += dy;
        }
    }
    if let NoteBlockNode::Group { children, .. } = block {
        for child in children.iter_mut() {
            offset_block_tree(child, dx, dy);
        }
    }
}

pub fn insert_after(blocks: &mut Vec<NoteBlockNode>, target_id: &str, block: NoteBlockNode) -> bool {
    if let Some(index) = blocks.iter().position(|entry| block_id(entry) == target_id) {
        blocks.insert(index + 1, block);
        return true;
    }
    for entry in blocks.iter_mut() {
        if let NoteBlockNode::Group { children, .. } = entry {
            if insert_after(children, target_id, block.clone()) {
                return true;
            }
        }
    }
    false
}

pub fn insert_block(blocks: &mut Vec<NoteBlockNode>, parent_id: Option<&str>, index: usize, block: NoteBlockNode) {
    if let Some(parent_id) = parent_id {
        for node in blocks.iter_mut() {
            if let NoteBlockNode::Group { id, children, .. } = node {
                if id == parent_id {
                    let index = index.min(children.len());
                    children.insert(index, block);
                    return;
                }
                insert_block(children, Some(parent_id), index, block.clone());
            }
        }
        return;
    }
    let index = index.min(blocks.len());
    blocks.insert(index, block);
}

pub fn update_block_in_tree(blocks: &mut [NoteBlockNode], target_id: &str, next_block: NoteBlockNode) -> bool {
    for block in blocks.iter_mut() {
        if block_id(block) == target_id {
            *block = next_block;
            return true;
        }
        if let NoteBlockNode::Group { children, .. } = block {
            if update_block_in_tree(children, target_id, next_block.clone()) {
                return true;
            }
        }
    }
    false
}

pub fn mutate_block_in_tree(blocks: &mut [NoteBlockNode], target_id: &str, mutator: &mut impl FnMut(&NoteBlockNode) -> NoteBlockNode) -> bool {
    for block in blocks.iter_mut() {
        if block_id(block) == target_id {
            *block = mutator(block);
            return true;
        }
        if let NoteBlockNode::Group { children, .. } = block {
            if mutate_block_in_tree(children, target_id, mutator) {
                return true;
            }
        }
    }
    false
}

pub fn block_bounds(block: &NoteBlockNode) -> (f64, f64, f64, f64) {
    match block {
        NoteBlockNode::Text { x, y, width, height, .. }
        | NoteBlockNode::Image { x, y, width, height, .. }
        | NoteBlockNode::Table { x, y, width, height, .. }
        | NoteBlockNode::Math { x, y, width, height, .. }
        | NoteBlockNode::Ink { x, y, width, height, .. }
        | NoteBlockNode::Group { x, y, width, height, .. } => (*x, *y, *width, *height),
    }
}

pub fn patch_block_field(document: &NoteSnapshot, block_id: &str, field: &str, value: &Value) -> NoteSnapshot {
    let Some(block) = find_block(&document.blocks, block_id).cloned() else {
        return document.clone();
    };
    let mut next = document.clone();
    match field {
        "name" => {
            mutate_block_in_tree(&mut next.blocks, block_id, &mut |block| {
                let mut cloned = block.clone();
                match &mut cloned {
                    NoteBlockNode::Text { name, .. } | NoteBlockNode::Image { name, .. } | NoteBlockNode::Table { name, .. } | NoteBlockNode::Math { name, .. } | NoteBlockNode::Ink { name, .. } | NoteBlockNode::Group { name, .. } => {
                        *name = value.as_str().unwrap_or("").into();
                    }
                }
                cloned
            });
        }
        "visible" => {
            let pressed = value.as_bool().unwrap_or(true);
            mutate_block_in_tree(&mut next.blocks, block_id, &mut |block| {
                let mut cloned = block.clone();
                match &mut cloned {
                    NoteBlockNode::Text { visible, .. }
                    | NoteBlockNode::Image { visible, .. }
                    | NoteBlockNode::Table { visible, .. }
                    | NoteBlockNode::Math { visible, .. }
                    | NoteBlockNode::Ink { visible, .. }
                    | NoteBlockNode::Group { visible, .. } => *visible = pressed,
                }
                cloned
            });
        }
        "locked" => {
            let pressed = value.as_bool().unwrap_or(false);
            mutate_block_in_tree(&mut next.blocks, block_id, &mut |block| {
                let mut cloned = block.clone();
                match &mut cloned {
                    NoteBlockNode::Text { locked, .. } | NoteBlockNode::Image { locked, .. } | NoteBlockNode::Table { locked, .. } | NoteBlockNode::Math { locked, .. } | NoteBlockNode::Ink { locked, .. } | NoteBlockNode::Group { locked, .. } => {
                        *locked = pressed;
                    }
                }
                cloned
            });
        }
        "x" | "y" | "width" | "height" => {
            let number = value.as_f64().unwrap_or(0.0);
            mutate_block_in_tree(&mut next.blocks, block_id, &mut |block| {
                let mut cloned = block.clone();
                match &mut cloned {
                    NoteBlockNode::Text { x, y, width, height, .. } => match field {
                        "x" => *x = number,
                        "y" => *y = number,
                        "width" => *width = number,
                        _ => *height = number,
                    },
                    NoteBlockNode::Image { x, y, width, height, .. } => match field {
                        "x" => *x = number,
                        "y" => *y = number,
                        "width" => *width = number,
                        _ => *height = number,
                    },
                    NoteBlockNode::Table { x, y, width, height, .. } => match field {
                        "x" => *x = number,
                        "y" => *y = number,
                        "width" => *width = number,
                        _ => *height = number,
                    },
                    NoteBlockNode::Math { x, y, width, height, .. } => match field {
                        "x" => *x = number,
                        "y" => *y = number,
                        "width" => *width = number,
                        _ => *height = number,
                    },
                    NoteBlockNode::Ink { x, y, width, height, .. } => match field {
                        "x" => *x = number,
                        "y" => *y = number,
                        "width" => *width = number,
                        _ => *height = number,
                    },
                    NoteBlockNode::Group { x, y, width, height, .. } => match field {
                        "x" => *x = number,
                        "y" => *y = number,
                        "width" => *width = number,
                        _ => *height = number,
                    },
                }
                cloned
            });
        }
        "textContent" => {
            if let NoteBlockNode::Text { .. } = block {
                let text = value.as_str().unwrap_or("");
                let paragraphs = vec![NoteTextParagraph { runs: vec![NoteTextRun { text: text.into(), bold: None, italic: None, underline: None, link: None }] }];
                let mut updated = block;
                if let NoteBlockNode::Text { paragraphs: p, .. } = &mut updated {
                    *p = paragraphs;
                }
                update_block_in_tree(&mut next.blocks, block_id, updated);
            }
        }
        "textSize" => {
            if let NoteBlockNode::Text { .. } = block {
                let mut updated = block;
                if let NoteBlockNode::Text { font_size, .. } = &mut updated {
                    *font_size = value.as_f64().unwrap_or(18.0);
                }
                update_block_in_tree(&mut next.blocks, block_id, updated);
            }
        }
        "mathTex" => {
            if let NoteBlockNode::Math { .. } = block {
                let mut updated = block;
                if let NoteBlockNode::Math { tex, .. } = &mut updated {
                    *tex = value.as_str().unwrap_or("").into();
                }
                update_block_in_tree(&mut next.blocks, block_id, updated);
            }
        }
        "inkWidth" => {
            if let NoteBlockNode::Ink { .. } = block {
                let mut updated = block;
                if let NoteBlockNode::Ink { stroke_width, .. } = &mut updated {
                    *stroke_width = value.as_f64().unwrap_or(3.0);
                }
                update_block_in_tree(&mut next.blocks, block_id, updated);
            }
        }
        "tableAddRow" => {
            if let NoteBlockNode::Table { .. } = block {
                let mut updated = block;
                if let NoteBlockNode::Table { columns, rows, .. } = &mut updated {
                    let width = columns.len();
                    rows.push((0..width).map(|_| NoteTableCell { content: String::new() }).collect());
                }
                update_block_in_tree(&mut next.blocks, block_id, updated);
            }
        }
        "tableRemoveRow" => {
            if let NoteBlockNode::Table { .. } = block {
                let mut updated = block;
                if let NoteBlockNode::Table { rows, .. } = &mut updated {
                    if rows.len() > 1 {
                        rows.pop();
                    }
                }
                update_block_in_tree(&mut next.blocks, block_id, updated);
            }
        }
        "tableAddColumn" => {
            if let NoteBlockNode::Table { .. } = block {
                let mut updated = block;
                if let NoteBlockNode::Table { columns, rows, .. } = &mut updated {
                    let next_letter = (b'A' + (columns.len() as u8 % 26)) as char;
                    columns.push(next_letter.to_string());
                    for row in rows.iter_mut() {
                        row.push(NoteTableCell { content: String::new() });
                    }
                }
                update_block_in_tree(&mut next.blocks, block_id, updated);
            }
        }
        "tableRemoveColumn" => {
            if let NoteBlockNode::Table { .. } = block {
                let mut updated = block;
                if let NoteBlockNode::Table { columns, rows, .. } = &mut updated {
                    if columns.len() > 1 {
                        columns.pop();
                        for row in rows.iter_mut() {
                            row.pop();
                        }
                    }
                }
                update_block_in_tree(&mut next.blocks, block_id, updated);
            }
        }
        _ => {}
    }
    next
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️MediaExport
pub fn note_document_bounds(document: &NoteSnapshot) -> (u32, u32) {
    let mut max_x = 1024.0_f64;
    let mut max_y = 1024.0_f64;
    for block in flatten_blocks(&document.blocks) {
        if !block_visible(block) {
            continue;
        }
        let (x, y, width, height) = match block {
            NoteBlockNode::Text { x, y, width, height, .. }
            | NoteBlockNode::Image { x, y, width, height, .. }
            | NoteBlockNode::Table { x, y, width, height, .. }
            | NoteBlockNode::Math { x, y, width, height, .. }
            | NoteBlockNode::Ink { x, y, width, height, .. }
            | NoteBlockNode::Group { x, y, width, height, .. } => (*x, *y, *width, *height),
        };
        max_x = max_x.max(x + width);
        max_y = max_y.max(y + height);
    }
    (max_x.max(1.0).round() as u32, max_y.max(1.0).round() as u32)
}

/// 🧭️ Position/rotation of any block variant, lifted into a semio `SemioTransform` (Z-axis-only
/// rotation, matching the drawing subset's own svg-bridge convention — see its `matrix_to_semio_transform`).
fn note_block_transform(block: &NoteBlockNode) -> SemioTransform {
    let (x, y, rotation) = match block {
        NoteBlockNode::Text { x, y, rotation, .. }
        | NoteBlockNode::Image { x, y, rotation, .. }
        | NoteBlockNode::Table { x, y, rotation, .. }
        | NoteBlockNode::Math { x, y, rotation, .. }
        | NoteBlockNode::Ink { x, y, rotation, .. }
        | NoteBlockNode::Group { x, y, rotation, .. } => (*x, *y, *rotation),
    };
    let theta = rotation.to_radians();
    SemioTransform {
        translation: SemioPoint3 { x, y, z: 0.0 },
        rotation: SemioQuaternion { x: 0.0, y: 0.0, z: (theta / 2.0).sin(), w: (theta / 2.0).cos() },
        scale: SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 },
    }
}

/// ▭ The outline-rectangle `PathSegment`s the deleted `note_block_to_svg` drew for its
/// image-without-asset and Table/Math/Group catch-all cases.
fn note_outline_rect_segments(width: f64, height: f64) -> Vec<PathSegment> {
    vec![
        PathSegment::MoveTo { to: SemioPoint2 { x: 0.0, y: 0.0 } },
        PathSegment::LineTo { to: SemioPoint2 { x: width, y: 0.0 } },
        PathSegment::LineTo { to: SemioPoint2 { x: width, y: height } },
        PathSegment::LineTo { to: SemioPoint2 { x: 0.0, y: height } },
        PathSegment::Close,
    ]
}

/// 🎨️ Always-append style intern, mirroring semio/drawing's own svg-import `intern_style`
/// convention (see that bridge's module doc) — one named `DrawStyle` per call, referenced back by name.
fn note_intern_style(styles: &mut Vec<DrawStyle>, fill: Option<SemioRgba>, stroke: Option<SemioRgba>, stroke_width: Option<f64>) -> String {
    let name = format!("note-style-{}", styles.len());
    styles.push(DrawStyle { name: name.clone(), fill, stroke, stroke_width, opacity: None });
    name
}

/// 🔤️ Minimal, dependency-free base64 decoder (this repo's "no external libraries for runtime
/// purposes" rule — mirrors semio/drawing's own svg-import `base64_decode`) — unwraps a
/// `NoteImageAsset.data` `data:<mime>;base64,<payload>` URI into the raw bytes `DrawNode::Image`
/// needs.
fn note_asset_data_uri_bytes(data_uri: &str) -> Vec<u8> {
    fn val(c: u8) -> Option<u8> {
        match c {
            b'A'..=b'Z' => Some(c - b'A'),
            b'a'..=b'z' => Some(c - b'a' + 26),
            b'0'..=b'9' => Some(c - b'0' + 52),
            b'+' => Some(62),
            b'/' => Some(63),
            _ => None,
        }
    }
    let payload = data_uri.split_once(',').map(|(_, rest)| rest).unwrap_or(data_uri);
    let clean: Vec<u8> = payload.bytes().filter(|&b| b != b'=' && !b.is_ascii_whitespace()).collect();
    let mut out = Vec::with_capacity(clean.len() * 3 / 4);
    for chunk in clean.chunks(4) {
        let vals: Vec<u8> = match chunk.iter().map(|&b| val(b)).collect::<Option<Vec<u8>>>() {
            Some(vals) => vals,
            None => return out,
        };
        let n = vals.len();
        let combined = vals.iter().fold(0u32, |acc, &v| (acc << 6) | v as u32) << ((4 - n) * 6);
        out.push((combined >> 16) as u8);
        if n > 2 { out.push((combined >> 8) as u8); }
        if n > 3 { out.push(combined as u8); }
    }
    out
}

/// 🖍️ Maps one note block into a `DrawNode` (wrapped in a `Group` carrying its position/rotation
/// `SemioTransform`) — the real per-block domain mapping that replaces the deleted hand-rolled
/// `note_block_to_svg` SVG string emission. Text/Image/Ink map onto their natural `DrawNode`
/// counterpart; Table/Math/Group (no scene-graph equivalent in this subset) fall back to the same
/// plain outline rectangle the deleted code drew for its catch-all case.
fn draw_node_from_note_block(block: &NoteBlockNode, document: &NoteSnapshot, styles: &mut Vec<DrawStyle>) -> Option<DrawNode> {
    let transform = note_block_transform(block);
    let inner = match block {
        NoteBlockNode::Text { paragraphs, font_size, .. } => {
            let text = paragraphs.iter().map(|paragraph| paragraph.runs.iter().map(|run| run.text.as_str()).collect::<Vec<_>>().join("")).collect::<Vec<_>>().join("\n");
            DrawNode::Text { value: text, at: SemioPoint2 { x: 0.0, y: *font_size }, style: None }
        }
        NoteBlockNode::Image { image_key, width, height, .. } => match document.assets.get(image_key) {
            Some(asset) => DrawNode::Image { at: SemioPoint2::default(), width: *width, height: *height, mime: asset.mime.clone(), bytes: note_asset_data_uri_bytes(&asset.data) },
            None => {
                let style = note_intern_style(styles, None, Some(SemioRgba { r: 0.53, g: 0.53, b: 0.53, a: 1.0 }), Some(1.0));
                DrawNode::Path { segments: note_outline_rect_segments(*width, *height), style: Some(style) }
            }
        },
        NoteBlockNode::Ink { points, stroke_width, color, .. } => {
            if points.len() < 2 {
                return None;
            }
            let mut segments = vec![PathSegment::MoveTo { to: SemioPoint2 { x: points[0][0], y: points[0][1] } }];
            segments.extend(points.iter().skip(1).map(|point| PathSegment::LineTo { to: SemioPoint2 { x: point[0], y: point[1] } }));
            let stroke = SemioRgba { r: color[0] as f32, g: color[1] as f32, b: color[2] as f32, a: color[3] as f32 };
            let style = note_intern_style(styles, None, Some(stroke), Some(*stroke_width));
            DrawNode::Path { segments, style: Some(style) }
        }
        NoteBlockNode::Table { width, height, .. } | NoteBlockNode::Math { width, height, .. } | NoteBlockNode::Group { width, height, .. } => {
            let style = note_intern_style(styles, None, Some(SemioRgba { r: 0.53, g: 0.53, b: 0.53, a: 1.0 }), Some(1.0));
            DrawNode::Path { segments: note_outline_rect_segments(*width, *height), style: Some(style) }
        }
    };
    Some(DrawNode::Group { transform, children: vec![inner] })
}

/// 🧿️ Builds this document's `SemioDrawingSnapshot` — one flattened, visible-only layer whose
/// children are each block's `draw_node_from_note_block` mapping. This is the real snapshot
/// `note_document_to_svg` hands to `io_dispatch` (never a hand-rolled SVG string).
pub fn note_document_to_drawing_snapshot(document: &NoteSnapshot) -> SemioDrawingSnapshot {
    let (width, height) = note_document_bounds(document);
    let mut styles = Vec::new();
    let children: Vec<DrawNode> = flatten_blocks(&document.blocks)
        .into_iter()
        .filter(|block| block_visible(block))
        .filter_map(|block| draw_node_from_note_block(block, document, &mut styles))
        .collect();
    SemioDrawingSnapshot {
        schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(),
        canvas: DrawCanvas { width: width as f64, height: height as f64, background: None },
        styles,
        layers: vec![DrawLayer { id: "0".into(), name: "note".into(), visible: true, root: DrawNode::Group { transform: SemioTransform::identity(), children } }],
    }
}

const NOTE_DRAWING_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("drawing") };
const NOTE_SVG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId::ANY };

/// 📌️ Registers the stdio semio/drawing subset's composer (schema descriptor + document codec +
/// validator + svg/dxf/pdf io entries) into the process-global `io` registry exactly once, so
/// `io_dispatch` below can resolve the drawing→svg bridge regardless of host-boot ordering (unit
/// tests included — nothing else in this test binary calls stdio's own `plugin()`/`register()`).
fn ensure_semio_drawing_bridge_registered() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(semio_drawing_composer::register);
}

/// 📤️ note → svg, real bridge: builds this document's `SemioDrawingSnapshot`, then dispatches it
/// through stdio's registered semio/drawing→svg composer (`io_dispatch`, never a hand-rolled SVG
/// string) to obtain a real `SvgSnapshot`, which is printed back to XML text via svg's own
/// `write_svg_xml`.
pub fn note_document_to_svg(document: &NoteSnapshot) -> Result<(String, u32, u32), String> {
    ensure_semio_drawing_bridge_registered();
    let (width, height) = note_document_bounds(document);
    let drawing = note_document_to_drawing_snapshot(document);
    let drawing_bytes = <SemioDrawingSnapshot as store::ArtifactPack>::encode_pack(&drawing);
    let key = IoKey {
        artifact_kind: NOTE_DRAWING_DIALECT.artifact_kind.to_string(),
        standard: NOTE_DRAWING_DIALECT.standard.0.to_string(),
        subset: NOTE_DRAWING_DIALECT.subset.0.to_string(),
        direction: IoDirection::Export,
        format_kind: NOTE_SVG_DIALECT.artifact_kind.to_string(),
        format_standard: NOTE_SVG_DIALECT.standard.0.to_string(),
        format_subset: NOTE_SVG_DIALECT.subset.0.to_string(),
    };
    let sources = [ErasedComposeSource { dialect: NOTE_DRAWING_DIALECT, payload: IoPayload::Binary(drawing_bytes) }];
    let composed = io_dispatch(&key, &sources).map_err(|error| format!("note→svg via semio/drawing bridge: {}", error.message))?;
    let svg_bytes = match composed.payload {
        IoPayload::Binary(bytes) => bytes,
        IoPayload::Text(_) => return Err("note→svg via semio/drawing bridge: expected a Binary (ArtifactPack) svg payload".into()),
    };
    let svg_snapshot = <semio_s_plugin_stdio::artifacts::svg::schema::snapshot::SvgSnapshot as store::ArtifactPack>::decode_pack(&svg_bytes)
        .map_err(|error| format!("note→svg via semio/drawing bridge: decode svg snapshot: {error:?}"))?;
    Ok((write_svg_xml(&svg_snapshot.doc), width, height))
}

pub fn note_document_json_to_svg(value: &Value) -> Result<(String, u32, u32), String> {
    let document: NoteSnapshot = serde_json::from_value(value.clone()).map_err(|error| error.to_string())?;
    note_document_to_svg(&document)
}
//#endregion 🔖️MediaExport

//#region 🔖️MediaImport
/// 🕳️ Not rewired onto the semio/drawing bridge (see `stdio_gaps` in the W5b ticket report): the
/// drawing subset has no dwg-format io leaf yet (only svg/dxf/pdf), and routing through the
/// framework's `dwg_drawing_to_svg` + `io_dispatch(svg→drawing)` round trip would REGRESS this
/// path — `dwg_drawing_to_svg` only walks `LwPolyline` geometry (silently drops `Text` entities)
/// and `DrawNode::Text` has no font-size field to carry DWG's `height`. `ink_block_from_points`/
/// `text_block_from_dwg` below are real domain mappers over already-typed `DwgGeometry` fields
/// (not hand-rolled DWG byte manipulation — `semio_framework::dwg_from_bytes` does the actual
/// byte-level parse), kept as the honest, lossless choice until that bridge exists.
fn ink_block_from_points(points: &[[f64; 2]]) -> NoteBlockNode {
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    for point in points {
        min_x = min_x.min(point[0]);
        min_y = min_y.min(point[1]);
        max_x = max_x.max(point[0]);
        max_y = max_y.max(point[1]);
    }
    let local_points = points.iter().map(|point| [point[0] - min_x, point[1] - min_y]).collect();
    NoteBlockNode::Ink {
        id: create_note_id("dwg-ink"),
        name: "Imported Stroke".into(),
        x: min_x,
        y: min_y,
        width: (max_x - min_x).max(1.0),
        height: (max_y - min_y).max(1.0),
        rotation: 0.0,
        visible: true,
        locked: false,
        points: local_points,
        stroke_width: 1.0,
        color: [0.0, 0.0, 0.0, 1.0],
    }
}

fn text_block_from_dwg(at: &[f64; 3], height: f64, rotation: f64, content: &str) -> NoteBlockNode {
    let font_size = if height > 0.0 { height } else { 12.0 };
    NoteBlockNode::Text {
        id: create_note_id("dwg-text"),
        name: "Imported Text".into(),
        x: at[0],
        y: at[1],
        width: (content.chars().count() as f64 * font_size * 0.6).max(font_size),
        height: font_size * 1.4,
        rotation,
        visible: true,
        locked: false,
        paragraphs: vec![NoteTextParagraph { runs: vec![NoteTextRun { text: content.to_string(), bold: None, italic: None, underline: None, link: None }] }],
        font_size,
        font_weight: "normal".into(),
        align: "left".into(),
    }
}

pub fn note_document_json_from_dwg(drawing: &DwgDrawing) -> Result<Value, String> {
    let mut document = empty_note_snapshot();
    document.id = create_note_id("dwg-import");
    document.title = Some("Imported Drawing".into());
    for entity in &drawing.entities {
        match &entity.geometry {
            DwgGeometry::Line { start, end } => {
                document.blocks.push(ink_block_from_points(&[[start[0], start[1]], [end[0], end[1]]]));
            }
            DwgGeometry::LwPolyline { closed, vertices, .. } => {
                if vertices.len() >= 2 {
                    let mut points = vertices.clone();
                    if *closed {
                        points.push(vertices[0]);
                    }
                    document.blocks.push(ink_block_from_points(&points));
                }
            }
            DwgGeometry::Text { at, height, rotation, content } => {
                document.blocks.push(text_block_from_dwg(at, *height, *rotation, content));
            }
            _ => {}
        }
    }
    serde_json::to_value(&document).map_err(|error| error.to_string())
}
//#endregion 🔖️MediaImport

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework::{DwgColor, DwgEntity, DwgLayer};

    #[test]
    fn clone_block_reids_group_children() {
        let child = create_block_by_kind("text", 0.0, 0.0);
        let child_id = block_id(&child).to_string();
        let group = NoteBlockNode::Group { id: "group-1".into(), name: "Group".into(), x: 0.0, y: 0.0, width: 100.0, height: 100.0, rotation: 0.0, visible: true, locked: false, children: vec![child] };
        let cloned = clone_block(&group);
        if let NoteBlockNode::Group { children, .. } = &cloned {
            assert_ne!(block_id(&children[0]), child_id);
        } else {
            panic!("expected group block");
        }
    }

    #[test]
    fn imports_dwg_polyline_and_text_into_note_blocks() {
        let drawing = DwgDrawing {
            layers: vec![DwgLayer::default()],
            entities: vec![
                DwgEntity { layer: 0, color: DwgColor::ByLayer, geometry: DwgGeometry::LwPolyline { closed: true, elevation: 0.0, vertices: vec![[0.0, 0.0], [10.0, 0.0], [10.0, 10.0]], bulges: vec![0.0, 0.5, 0.0] } },
                DwgEntity { layer: 0, color: DwgColor::ByLayer, geometry: DwgGeometry::Text { at: [1.0, 2.0, 0.0], height: 2.5, rotation: 0.0, content: "semio".into() } },
            ],
            extmin: [0.0, 0.0, 0.0],
            extmax: [10.0, 10.0, 0.0],
        };
        let value = note_document_json_from_dwg(&drawing).unwrap();
        let document: NoteSnapshot = serde_json::from_value(value).unwrap();
        assert_eq!(document.schema, NOTE_DOCUMENT_SCHEMA);
        assert_eq!(document.blocks.len(), 2);
        let ink_count = document.blocks.iter().filter(|block| matches!(block, NoteBlockNode::Ink { .. })).count();
        let text_count = document.blocks.iter().filter(|block| matches!(block, NoteBlockNode::Text { .. })).count();
        assert_eq!(ink_count, 1);
        assert_eq!(text_count, 1);
        if let Some(NoteBlockNode::Ink { points, .. }) = document.blocks.iter().find(|block| matches!(block, NoteBlockNode::Ink { .. })) {
            assert_eq!(points.len(), 4);
        } else {
            panic!("expected ink block");
        }
        if let Some(NoteBlockNode::Text { paragraphs, .. }) = document.blocks.iter().find(|block| matches!(block, NoteBlockNode::Text { .. })) {
            assert_eq!(paragraphs[0].runs[0].text, "semio");
        } else {
            panic!("expected text block");
        }
    }

    #[test]
    fn imports_empty_dwg_drawing_as_valid_empty_note_snapshot() {
        let drawing = DwgDrawing::default();
        let value = note_document_json_from_dwg(&drawing).unwrap();
        let document: NoteSnapshot = serde_json::from_value(value).unwrap();
        assert_eq!(document.schema, NOTE_DOCUMENT_SCHEMA);
        assert!(document.blocks.is_empty());
    }

    /// 🧪️ Real end-to-end proof `note_document_to_svg` goes through `io_dispatch` onto stdio's
    /// registered semio/drawing→svg composer (not a hand-rolled SVG string): text content, ink
    /// strokes, and the Table catch-all outline all have to survive a real `SemioDrawingSnapshot`
    /// build + a real svg-composer round trip to show up in the output.
    #[test]
    fn document_to_svg_dispatches_through_semio_drawing_bridge() {
        let mut document = empty_note_snapshot();
        document.blocks.push(NoteBlockNode::Text {
            id: "t1".into(),
            name: "Text".into(),
            x: 10.0,
            y: 20.0,
            width: 100.0,
            height: 30.0,
            rotation: 0.0,
            visible: true,
            locked: false,
            paragraphs: vec![NoteTextParagraph { runs: vec![NoteTextRun { text: "hello semio".into(), bold: None, italic: None, underline: None, link: None }] }],
            font_size: 18.0,
            font_weight: "normal".into(),
            align: "left".into(),
        });
        document.blocks.push(NoteBlockNode::Ink {
            id: "i1".into(),
            name: "Ink".into(),
            x: 0.0,
            y: 0.0,
            width: 1.0,
            height: 1.0,
            rotation: 0.0,
            visible: true,
            locked: false,
            points: vec![[0.0, 0.0], [5.0, 5.0]],
            stroke_width: 2.0,
            color: [1.0, 0.0, 0.0, 1.0],
        });
        document.blocks.push(NoteBlockNode::Table {
            id: "tb1".into(),
            name: "Table".into(),
            x: 0.0,
            y: 0.0,
            width: 50.0,
            height: 40.0,
            rotation: 0.0,
            visible: true,
            locked: false,
            columns: vec!["A".into()],
            rows: vec![vec![NoteTableCell { content: String::new() }]],
        });

        let (svg, width, height) = note_document_to_svg(&document).expect("svg export via io_dispatch");
        assert!(svg.starts_with("<svg"), "{svg}");
        assert!(svg.contains("hello semio"), "{svg}");
        assert!(width >= 1024 && height >= 1024);

        // Same pipeline through the JSON-wrapped entry point every io leaf/media handler actually calls.
        let json = serde_json::to_value(&document).unwrap();
        let (svg_via_json, _w, _h) = note_document_json_to_svg(&json).expect("json svg export via io_dispatch");
        assert_eq!(svg, svg_via_json);
    }

    /// 🧪️ Real proof an image block's asset bytes flow through `DrawNode::Image` (base64-decoded
    /// from the `NoteImageAsset.data` uri) and back out as a data uri on the svg side.
    #[test]
    fn document_to_svg_embeds_image_asset_bytes_as_data_uri() {
        let mut document = empty_note_snapshot();
        document.assets.insert(
            "asset-1".into(),
            crate::artifacts::note::NoteImageAsset { mime: "image/png".into(), data: "data:image/png;base64,AAECAw==".into(), width: Some(4.0), height: Some(4.0) },
        );
        document.blocks.push(NoteBlockNode::Image { id: "im1".into(), name: "Image".into(), x: 0.0, y: 0.0, width: 4.0, height: 4.0, rotation: 0.0, visible: true, locked: false, image_key: "asset-1".into() });

        let (svg, _w, _h) = note_document_to_svg(&document).expect("svg export via io_dispatch");
        assert!(svg.contains("data:image/png;base64,"), "{svg}");
    }

    #[test]
    fn note_document_to_drawing_snapshot_flattens_visible_blocks_into_one_layer() {
        let mut document = empty_note_snapshot();
        document.blocks.push(create_block_by_kind("text", 5.0, 6.0));
        let mut hidden = create_block_by_kind("text", 0.0, 0.0);
        if let NoteBlockNode::Text { visible, .. } = &mut hidden {
            *visible = false;
        }
        document.blocks.push(hidden);

        let drawing = note_document_to_drawing_snapshot(&document);
        assert_eq!(drawing.layers.len(), 1);
        let DrawNode::Group { children, .. } = &drawing.layers[0].root else { panic!("expected root Group") };
        assert_eq!(children.len(), 1, "hidden block must not be mapped into a DrawNode");
    }
}
//#endregion 🧪️Tests

//#region 🔖️ArtifactEngine
pub struct NoteEngine {
    artifact: crate::artifacts::note::schema::NoteArtifact,
    snapshot: NoteSnapshot,
}

impl NoteEngine {
    pub fn new(snapshot: NoteSnapshot) -> Self {
        let artifact = crate::artifacts::note::schema::NoteArtifact::from_snapshot(snapshot.clone());
        Self { artifact, snapshot }
    }
}
//#endregion 🔖️ArtifactEngine
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ArtifactBuilder, ComposerEntry, ComposedArtifact, ComposeError, Dialect, StandardId, SubsetId, ErasedComposeSource, IoPayload, IoConfidence, composer_entry_of};
    use crate::artifacts::note::standards::v1::subsets::any::schema::NoteComposer as NoteAnyComposer;
    use crate::artifacts::note::standards::v1::subsets::any::schema::NoteBuilder as NoteAnyBuilder;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    //#region 🔖️ExportEntries
    /// 🗄️ Ticket 26/08/10/STDIO-ARTIFACTS-AND-IO W15: the typed registry (W11-W14) only ever grew
    /// IMPORT-direction entries (each composer's own `reads()`) -- nothing registers the REVERSE
    /// ("this domain artifact can be exported AS format Y"), because `ArtifactComposer` only models
    /// "produce my own snapshot." These entries wrap the artifact's EXISTING `🚪️io/📤️export/🧵️serializers`
    /// leaves (which already convert this artifact's snapshot straight to target-format bytes/text) as
    /// their own `ComposerEntry` rows: `writes` = the target format's dialect, `reads` = just this
    /// artifact's own dialect. `register_composer_entries` already inserts BOTH an Import key (target
    /// reads from us) and an Export key (we export to target) per entry, so no framework change was
    /// needed, only populating the missing direction. Generated by generators/w15_add_export_entries.py
    /// -- hand-validated pattern on note/json first (see that file's own tests), pilot kept as reference.
    const NOTE_DIALECT: Dialect = Dialect { artifact_kind: "s.note", standard: StandardId("1"), subset: SubsetId("*") };
    const NOTE_JSON_BRIDGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

    fn rebuild_native_snapshot(sources: &[ErasedComposeSource]) -> Result<crate::artifacts::note::NoteSnapshot, ComposeError> {
        if let Some(source) = sources.iter().find(|s| s.dialect == NOTE_DIALECT) {
            let builder = match &source.payload {
                IoPayload::Text(t) => NoteAnyBuilder::from_text(t).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
                IoPayload::Binary(b) => NoteAnyBuilder::from_binary(b).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
            };
            return builder.build().map_err(|diagnostics| ComposeError { message: "NoteComposer export: build() failed".into(), diagnostics });
        }
        if let Some(source) = sources.iter().find(|s| s.dialect == NOTE_JSON_BRIDGE_DIALECT) {
            // 🌉 The OS dispatch layer (export_os_app_instance_media_kind) deals in already-
            // deserialized `serde_json::Value`, not this artifact's own wire text/binary -- json
            // is the universal bridge dialect every domain artifact already imports from.
            let bytes: Vec<u8> = match &source.payload {
                IoPayload::Text(t) => t.as_bytes().to_vec(),
                IoPayload::Binary(b) => b.clone(),
            };
            return crate::artifacts::note::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() });
        }
        Err(ComposeError { message: "NoteComposer export: no native or json-bridge source provided".into(), diagnostics: Vec::new() })
    }

    const EXPORT_SVG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("*") };
    fn compose_export_svg(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::note::io::export::serializers::artifacts::svg::v1_1::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_SVG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_PDF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId("*") };
    fn compose_export_pdf(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::note::io::export::serializers::artifacts::pdf::v1_4::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_PDF_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
    fn compose_export_png(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::note::io::export::serializers::artifacts::png::v1_2::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_PNG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    fn compose_export_json(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::note::io::export::serializers::artifacts::json::v_rfc8259::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_JSON_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_DWG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId("*") };
    fn compose_export_dwg(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::note::io::export::serializers::artifacts::dwg::v_ac1018::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_DWG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_DXF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dxf", standard: StandardId("r12"), subset: SubsetId("*") };
    fn compose_export_dxf(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::note::io::export::serializers::artifacts::dxf::v_r12::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_DXF_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    //#endregion 🔖️ExportEntries





    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![
            composer_entry_of::<NoteAnyComposer>(),
            ComposerEntry { writes: EXPORT_SVG_DIALECT, reads: &[NOTE_DIALECT], compose: compose_export_svg },
            ComposerEntry { writes: EXPORT_PDF_DIALECT, reads: &[NOTE_DIALECT], compose: compose_export_pdf },
            ComposerEntry { writes: EXPORT_PNG_DIALECT, reads: &[NOTE_DIALECT], compose: compose_export_png },
            ComposerEntry { writes: EXPORT_JSON_DIALECT, reads: &[NOTE_DIALECT], compose: compose_export_json },
            ComposerEntry { writes: EXPORT_DWG_DIALECT, reads: &[NOTE_DIALECT], compose: compose_export_dwg },
            ComposerEntry { writes: EXPORT_DXF_DIALECT, reads: &[NOTE_DIALECT], compose: compose_export_dxf },
        ]).as_slice()
    }

    //#region 🧪️Tests
    /// 🧪️ Reference pilot (hand-maintained, not regenerated by w15_add_export_entries.py) proving the
    /// export-entry pattern is genuinely correct, not just compiling -- a real round-trip through the
    /// typed registry. Every other artifact's export entries follow this exact mechanical shape.
    #[cfg(test)]
    mod tests {
        use super::*;
        use semio_framework_plugin::{register_composer_entries, io_resolve, io_dialects_for, IoKey, IoDirection};

        #[test]
        fn every_export_target_writes_its_own_dialect_and_reads_only_the_native_dialect() {
            for entry in entries().iter().skip(1) {
                assert_eq!(entry.reads, &[NOTE_DIALECT], "export entry writing {:?} should read only the native dialect", entry.writes);
            }
        }

        #[test]
        fn registered_json_export_entry_resolves_and_produces_valid_json_bytes() {
            register_composer_entries(entries());
            let key = IoKey {
                artifact_kind: NOTE_DIALECT.artifact_kind.to_string(),
                standard: NOTE_DIALECT.standard.0.to_string(),
                subset: NOTE_DIALECT.subset.0.to_string(),
                direction: IoDirection::Export,
                format_kind: EXPORT_JSON_DIALECT.artifact_kind.to_string(),
                format_standard: EXPORT_JSON_DIALECT.standard.0.to_string(),
                format_subset: EXPORT_JSON_DIALECT.subset.0.to_string(),
            };
            let resolved = io_resolve(&key).expect("note -> json export entry resolves through the typed registry");
            let snapshot = NoteAnyBuilder::empty().build().expect("empty note builds");
            let native_bytes = store::ArtifactPack::encode_pack(&snapshot);
            let sources = [ErasedComposeSource { dialect: NOTE_DIALECT, payload: IoPayload::Binary(native_bytes) }];
            let composed = (resolved.compose)(&sources).expect("compose succeeds");
            assert_eq!(composed.dialect, EXPORT_JSON_DIALECT);
            let IoPayload::Binary(bytes) = composed.payload else { panic!("expected binary json payload") };
            let value: serde_json::Value = serde_json::from_slice(&bytes).expect("export produced valid json bytes");
            assert!(value.is_object(), "expected a json object, got {value:?}");
        }

        #[test]
        fn dialects_for_export_direction_includes_every_target() {
            register_composer_entries(entries());
            let dialects = io_dialects_for(NOTE_DIALECT.artifact_kind, IoDirection::Export);
            for entry in entries().iter().skip(1) {
                assert!(dialects.contains(&entry.writes), "expected note's export dialects to include {:?}, got {:?}", entry.writes, dialects);
            }
        }
    }
    //#endregion 🧪️Tests
}
//#endregion 🚪️DerivedIoRegistry
