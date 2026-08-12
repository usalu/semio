//! 🧬️ Note artifact schema — every field of the artifact with its state class.

use crate::artifacts::note::{NoteBlockNode, NoteImageAsset, NoteTableCell, NoteTextParagraph, NoteTextRun, NOTE_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

//#region 🔖️Artifact
/// 🧬️ Full note artifact state across persistent, shared-ui, local-ui and preview classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.note.note")]
pub struct NoteArtifact {
    #[state(persistent)] pub schema: String,
    #[state(persistent)] pub id: String,
    #[state(persistent)] pub title: Option<String>,
    #[state(persistent)] pub blocks: Vec<NoteBlockNode>,
    #[state(persistent)] pub grid_visible: Option<bool>,
    #[state(persistent)] pub grid_spacing: Option<f64>,
    #[state(persistent)] pub grid_subdivisions: Option<f64>,
    #[state(persistent)] pub grid_opacity: Option<f64>,
    #[state(persistent)] pub snap_enabled: Option<bool>,
    #[state(persistent)] pub snap_grid_spacing: Option<f64>,
    #[state(persistent)] pub pencil_width: Option<f64>,
    #[state(persistent)] pub eraser_radius: Option<f64>,
    #[state(persistent)] pub assets: BTreeMap<String, NoteImageAsset>,
    #[state(shared_ui)] pub selected_block_ids: Vec<String>,
    #[state(shared_ui)] pub active_utility_id: String,
    #[state(local_ui)] pub engagement_input: String,
    #[state(local_ui)] pub camera_x: f64,
    #[state(local_ui)] pub camera_y: f64,
    #[state(local_ui)] pub camera_zoom: f64,
    #[state(local_ui)] pub locale: String,
    #[state(preview)] pub hovered_block_id: Option<String>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for NoteArtifact {
    fn default() -> Self {
        Self::from_snapshot(crate::artifacts::note::NoteSnapshot::default())
    }
}

impl NoteArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::note::NoteSnapshot {
        crate::artifacts::note::NoteSnapshot {
            schema: self.schema.clone(),
            id: self.id.clone(),
            title: self.title.clone(),
            blocks: self.blocks.clone(),
            grid_visible: self.grid_visible,
            grid_spacing: self.grid_spacing,
            grid_subdivisions: self.grid_subdivisions,
            grid_opacity: self.grid_opacity,
            snap_enabled: self.snap_enabled,
            snap_grid_spacing: self.snap_grid_spacing,
            pencil_width: self.pencil_width,
            eraser_radius: self.eraser_radius,
            assets: self.assets.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::note::NoteSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            id: snapshot.id,
            title: snapshot.title,
            blocks: snapshot.blocks,
            grid_visible: snapshot.grid_visible,
            grid_spacing: snapshot.grid_spacing,
            grid_subdivisions: snapshot.grid_subdivisions,
            grid_opacity: snapshot.grid_opacity,
            snap_enabled: snapshot.snap_enabled,
            snap_grid_spacing: snapshot.snap_grid_spacing,
            pencil_width: snapshot.pencil_width,
            eraser_radius: snapshot.eraser_radius,
            assets: snapshot.assets,
            ..Self::default_ui()
        }
    }

    fn default_ui() -> Self {
        Self {
            schema: NOTE_DOCUMENT_SCHEMA.into(),
            id: String::new(),
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
            selected_block_ids: Vec::new(),
            active_utility_id: "selectDirect".into(),
            engagement_input: String::new(),
            camera_x: 0.0,
            camera_y: 0.0,
            camera_zoom: 1.0,
            locale: "en-US".into(),
            hovered_block_id: None,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::note::NoteSnapshot) {
        self.schema = snapshot.schema;
        self.id = snapshot.id;
        self.title = snapshot.title;
        self.blocks = snapshot.blocks;
        self.grid_visible = snapshot.grid_visible;
        self.grid_spacing = snapshot.grid_spacing;
        self.grid_subdivisions = snapshot.grid_subdivisions;
        self.grid_opacity = snapshot.grid_opacity;
        self.snap_enabled = snapshot.snap_enabled;
        self.snap_grid_spacing = snapshot.snap_grid_spacing;
        self.pencil_width = snapshot.pencil_width;
        self.eraser_radius = snapshot.eraser_radius;
        self.assets = snapshot.assets;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.note.note` — twenty handcrafted schema leaves.
pub fn note_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.note.note",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️component.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

//#region 🔖️DocumentHelpers
/// 📄️ The `semio` example document, handcrafted in the `.note` DSL — {@link semio_example_snapshot}/
/// {@link semio_example_json} are the only ways it should be consumed.
const SEMIO_NOTE_EXAMPLE_TEXT: &str = crate::artifacts::note::schema::snapshot::text::SEMIO_NOTE_EXAMPLE_TEXT;

/// 🆔️ Monotonic id generator for freshly created/duplicated/imported blocks.
pub fn create_note_id(prefix: &str) -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(1);
    let serial = NEXT.fetch_add(1, Ordering::Relaxed);
    format!("{prefix}-{serial}")
}

/// 📄️ The `semio` example, parsed once from {@link SEMIO_NOTE_EXAMPLE_TEXT} — the source of truth for
/// every "semio" example call site (`setActiveExample`, tests). Falls back to the empty document if the
/// fixture ever fails to parse, matching the old JSON fixture's failure behavior.
pub fn semio_example_snapshot() -> crate::artifacts::note::NoteSnapshot {
    <crate::artifacts::note::NoteSnapshot as store::ArtifactDsl>::parse_dsl(SEMIO_NOTE_EXAMPLE_TEXT).unwrap_or_else(|_| empty_note_snapshot())
}

/// 📄️ JSON re-serialization of {@link semio_example_snapshot}, for the framework-generic call sites that
/// contractually require JSON text (`PluginApp::render`'s `projection_override_json`, `App::example`'s
/// manifest `document_json`).
pub fn semio_example_json() -> String {
    serde_json::to_string(&semio_example_snapshot()).expect("serialize semio example document")
}

pub fn empty_note_snapshot() -> crate::artifacts::note::NoteSnapshot {
    crate::artifacts::note::NoteSnapshot {
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

pub fn patch_block_field(document: &crate::artifacts::note::NoteSnapshot, block_id: &str, field: &str, value: &Value) -> crate::artifacts::note::NoteSnapshot {
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

//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use semio_framework_plugin::ArtifactBuilder;
    use crate::artifacts::note::{NoteDiff, NoteMutation, NoteSnapshot};

    #[derive(Clone, Debug, Default)]
    pub struct NoteBuilderConstruction {
        snapshot: NoteSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for NoteBuilderConstruction {
        type Snapshot = NoteSnapshot;
        type Mutation = NoteMutation;
        type Diff = NoteDiff;
        fn empty() -> Self { Self { snapshot: NoteSnapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<NoteSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<NoteSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
            self.snapshot = crate::artifacts::note::schema::mutations::apply_note_mutation(&self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <NoteDiff as protocol::MutationDiff<NoteSnapshot>>::apply(&diff, &self.snapshot);
            self
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use semio_framework_plugin::{ArtifactAnalysis, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
    use crate::artifacts::note::NoteSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct NoteParts {
        pub snapshot: Option<NoteSnapshot>,
    }

    pub struct NoteAnalyzerAnalysis;

    impl ArtifactAnalysis for NoteAnalyzerAnalysis {
        type Parts = NoteParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.note", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = NoteParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <NoteSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <NoteSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.binary", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec NoteBuilderFacets {
        construction: derived_construction::NoteBuilderConstruction,
        analysis: derived_analysis::NoteAnalyzerAnalysis,
        composition: super::super::io::derived_composition::NoteComposerComposition,
    }
    builder: NoteBuilder,
    analyzer: NoteAnalyzer,
    composer: NoteComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🧪️ Relocated from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
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
}
//#endregion 🧪️Tests
