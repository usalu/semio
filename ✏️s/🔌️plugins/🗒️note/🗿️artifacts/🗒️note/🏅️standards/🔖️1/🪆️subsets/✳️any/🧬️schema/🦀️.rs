//! 🧬️ Note artifact schema — every field of the artifact with its state class.

use crate::{NoteTableCell, NoteTextParagraph, NoteTextRun};
use framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Artifact
/// 🧬️ note document artifact state.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.note.note")]
pub struct NoteArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub id: String,
    #[state(artifact)]
    pub title: Option<String>,
    #[state(artifact)]
    pub blocks: Vec<NoteBlockNode>,
    #[state(artifact)]
    pub grid_visible: Option<bool>,
    #[state(artifact)]
    pub grid_spacing: Option<f64>,
    #[state(artifact)]
    pub grid_subdivisions: Option<f64>,
    #[state(artifact)]
    pub grid_opacity: Option<f64>,
    #[state(artifact)]
    pub snap_enabled: Option<bool>,
    #[state(artifact)]
    pub snap_grid_spacing: Option<f64>,
    #[state(artifact)]
    pub pencil_width: Option<f64>,
    #[state(artifact)]
    pub eraser_radius: Option<f64>,
    #[state(artifact)]
    pub assets: BTreeMap<String, NoteImageAsset>,
    #[state(artifact)]
    #[link_slot(roles("any"))]
    pub linked_artifact: Option<store::ArtifactLink>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for NoteArtifact {
    fn default() -> Self {
        Self::from_snapshot(crate::NoteSnapshot::default())
    }
}

impl NoteArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::NoteSnapshot {
        crate::NoteSnapshot {
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
            linked_artifact: self.linked_artifact.clone(),
        }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: crate::NoteSnapshot) -> Self {
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
            linked_artifact: snapshot.linked_artifact,

        }
    }


    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::NoteSnapshot) {
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
        self.linked_artifact = snapshot.linked_artifact;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.note.note` — twenty handcrafted schema leaves.
pub fn note_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.note.note",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

//#region 🔖️DocumentHelpers
/// 📄️ The `semio` example document, handcrafted in the `.note` DSL — {@link semio_example_snapshot}/
/// {@link semio_example_json} are the only ways it should be consumed.

/// 🆔️ Durable identifier cursor owned by one exact app operation or importer child.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct NoteIdOwner {
    pub scope: String,
    pub next_serial: u64,
}

impl NoteIdOwner {
    pub fn new(scope: impl Into<String>, next_serial: u64) -> Self {
        Self { scope: scope.into(), next_serial }
    }

    pub fn for_document_child(document: &crate::NoteSnapshot, child: &str) -> Self {
        Self::new(format!("{}-{child}", document.id), document.blocks.len() as u64)
    }

    pub fn allocate(&mut self, prefix: &str) -> String {
        let serial = self.next_serial;
        self.next_serial = self.next_serial.checked_add(1).expect("note identifier owner exhausted its serial range");
        format!("{prefix}-{}-{serial}", note_id_scope_tag(&self.scope))
    }
}

fn note_id_scope_tag(scope: &str) -> String {
    let digest = scope.as_bytes().iter().fold(0xcbf2_9ce4_8422_2325_u64, |state, byte| (state ^ u64::from(*byte)).wrapping_mul(0x100_0000_01b3));
    format!("{digest:016x}")
}

/// 🆔️ Allocates from the caller-owned operation/child cursor; no plugin-process owner exists.
pub fn create_note_id(owner: &mut NoteIdOwner, prefix: &str) -> String {
    owner.allocate(prefix)
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

/// 📏️ Number of children in `parent`'s container (`None` = the document root); `None` when `parent` names no group.
pub fn container_len(blocks: &[NoteBlockNode], parent: Option<&str>) -> Option<usize> {
    match parent {
        None => Some(blocks.len()),
        Some(id) => match find_block(blocks, id)? {
            NoteBlockNode::Group { children, .. } => Some(children.len()),
            _ => None,
        },
    }
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

pub fn create_block_by_kind(owner: &mut NoteIdOwner, kind: &str, x: f64, y: f64) -> NoteBlockNode {
    let id = create_note_id(owner, kind);
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
        _ => {
            let paragraphs = vec![NoteTextParagraph { runs: vec![NoteTextRun { text: String::new(), bold: None, italic: None, underline: None, link: None }] }];
            NoteBlockNode::Text {
                content: crate::note_text_child_record(&id, &paragraphs),
                id,
                name: "Text".into(),
                x,
                y,
                width: 280.0,
                height: 120.0,
                rotation: 0.0,
                visible: true,
                locked: false,
                font_size: 18.0,
                font_weight: "normal".into(),
                align: "left".into(),
            }
        }
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

pub fn reid_block_tree(owner: &mut NoteIdOwner, block: &mut NoteBlockNode, rename_top: bool) {
    let kind = block_kind(block).to_string();
    // 🧬️ A duplicated Text block must never keep its source's composed `content` child handle — two
    // distinct block ids sharing one content-addressed child slot would violate the "a child slot is
    // owned by exactly one parent" invariant composition assumes. Copy the source record's durable
    // paragraphs before the id changes, then remint them under the new id assigned below.
    let recovered_paragraphs = if let NoteBlockNode::Text { content, .. } = &*block { Some(crate::note_block_text(content)) } else { None };
    match block {
        NoteBlockNode::Text { id, name, .. } | NoteBlockNode::Image { id, name, .. } | NoteBlockNode::Table { id, name, .. } | NoteBlockNode::Math { id, name, .. } | NoteBlockNode::Ink { id, name, .. } | NoteBlockNode::Group { id, name, .. } => {
            *id = create_note_id(owner, &kind);
            if rename_top {
                *name = format!("{name} copy");
            }
        }
    }
    if let (NoteBlockNode::Text { id, content, .. }, Some(paragraphs)) = (&mut *block, recovered_paragraphs) {
        *content = crate::note_text_child_record(id, &paragraphs);
    }
    if let NoteBlockNode::Group { children, .. } = block {
        for child in children.iter_mut() {
            reid_block_tree(owner, child, false);
        }
    }
}

pub fn clone_block(owner: &mut NoteIdOwner, block: &NoteBlockNode) -> NoteBlockNode {
    let mut cloned = block.clone();
    reid_block_tree(owner, &mut cloned, true);
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

//#endregion 🔖️DocumentHelpers

//#region 🏗️Construction
/// 🏗️ Ordinary `Mutation`/`MutationDiff` algebra is all this subset needs (no custom analysis/
/// composition logic beyond it) — the old hand-rolled `NoteBuilderConstruction` (which only
/// differed from this in swallowing a failed `apply` into a diagnostics `Vec` instead of erroring)
/// is retired in favor of the SDK's generic replacement (ticket
/// 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM, `📓️recipe-subset.md` §4d/step 6).
pub type Construction = semio_framework_plugin::app::SnapshotBuilder<crate::NoteSnapshot, crate::NoteMutation>;
//#endregion 🏗️Construction

//#region 🧪️Tests

//#endregion 🧪️Tests

//#region 🔖️Helpers
/// 🔢️ A history label's number, `(en, de)`: two decimals at most, trailing zeros trimmed, a German decimal comma.
pub fn note_label_number(value: f64) -> (String, String) {
    let rounded = (value * 100.0).round() / 100.0;
    let en = format!("{:.2}", if rounded == 0.0 { 0.0 } else { rounded }).trim_end_matches('0').trim_end_matches('.').to_string();
    let de = en.replace('.', ",");
    (en, de)
}

/// 🎚️ A document-setting history label from the setting's `(en, de)` name: "Change grid spacing to 20" / "Rasterabstand auf
/// 20 ändern" for a value, "Reset grid spacing" / "Rasterabstand zurücksetzen" for `None` (the default applies again).
pub fn note_setting_label(setting: (&str, &str), value: Option<(String, String)>) -> semio_framework_ui_locale::LocalizedLabel {
    match value {
        Some((en, de)) => semio_framework_ui_locale::LocalizedLabel::native(&format!("Change {} to {en}", setting.0), &format!("{} auf {de} ändern", setting.1)),
        None => semio_framework_ui_locale::LocalizedLabel::native(&format!("Reset {}", setting.0), &format!("{} zurücksetzen", setting.1)),
    }
}
//#endregion 🔖️Helpers

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::NoteBlockNode;
pub use crate::NoteImageAsset;
//#endregion 🔁️Re-exports

//#region 🧩️ArtifactItems4
pub mod artifact_items_4 {
use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_stdio_semio_text_model::schema::snapshot::SemioTextMark;
use semio_s_artifact_stdio_semio_text_model::schema::snapshot::SemioTextMarkKind;
use semio_s_artifact_stdio_semio_text_model::schema::snapshot::SemioTextRun;
use semio_s_artifact_stdio_semio_text_model::schema::snapshot::SemioTextSnapshot;
use semio_s_artifact_stdio_semio_text_model::schema::snapshot::STDIO_SEMIOTEXT_DOCUMENT_SCHEMA;
use serde::{Deserialize, Serialize};
pub use crate::standards::v1::subsets::any::schema;

//#region 🔖️Domain
pub const NOTE_DOCUMENT_SCHEMA: &str = "note.document";

/// 🎥️ Camera pose — persisted by the exact composite-window configuration owner, never in
/// `NoteSnapshot` or app-wide configuration, so two open canvases retain independent poses.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct NoteCamera {
    #[serde(default)]
    #[value(default)]
    pub x: f64,
    #[serde(default)]
    #[value(default)]
    pub y: f64,
    #[serde(default = "default_zoom")]
    #[value(default = "default_zoom")]
    pub zoom: f64,
}

impl Default for NoteCamera {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

pub fn default_zoom() -> f64 {
    1.0
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum NoteBlockNode {
    #[value(rename = "text", rename_all = "camelCase")]
    Text {
        id: String,
        name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        #[value(default)]
        rotation: f64,
        #[value(default = "default_true")]
        visible: bool,
        #[value(default)]
        locked: bool,
        /// ✏️ Composed content — ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`note→C:text`):
        /// replaces the former inline `paragraphs: Vec<NoteTextParagraph>` (which duplicated stdio's
        /// `s.stdio.semio.text` run/mark shape) with a content-addressed handle onto that composed
        /// subset. See this file's `🔖️TextBridge`/`🔖️TextChildren` regions for the
        /// converter and durable child-record accessor.
        content: NoteTextChild,
        font_size: f64,
        font_weight: String,
        align: String,
    },
    #[value(rename = "image", rename_all = "camelCase")]
    Image {
        id: String,
        name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        #[value(default)]
        rotation: f64,
        #[value(default = "default_true")]
        visible: bool,
        #[value(default)]
        locked: bool,
        image_key: String,
    },
    #[value(rename = "table", rename_all = "camelCase")]
    Table {
        id: String,
        name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        #[value(default)]
        rotation: f64,
        #[value(default = "default_true")]
        visible: bool,
        #[value(default)]
        locked: bool,
        columns: Vec<String>,
        rows: Vec<Vec<NoteTableCell>>,
    },
    #[value(rename = "math", rename_all = "camelCase")]
    Math {
        id: String,
        name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        #[value(default)]
        rotation: f64,
        #[value(default = "default_true")]
        visible: bool,
        #[value(default)]
        locked: bool,
        #[dsl(lang = "tex")]
        tex: String,
        display_mode: bool,
    },
    #[value(rename = "stroke", rename_all = "camelCase")]
    #[dsl(key = "stroke")]
    Ink {
        id: String,
        name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        #[value(default)]
        rotation: f64,
        #[value(default = "default_true")]
        visible: bool,
        #[value(default)]
        locked: bool,
        points: Vec<[f64; 2]>,
        stroke_width: f64,
        color: [f64; 4],
    },
    #[value(rename = "group", rename_all = "camelCase")]
    Group {
        id: String,
        name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        #[value(default)]
        rotation: f64,
        #[value(default = "default_true")]
        visible: bool,
        #[value(default)]
        locked: bool,
        #[dsl(statements, block)]
        children: Vec<NoteBlockNode>,
    },
}

//#region 🔖️ComposedTypes
/// 🕸️ Snapshot-owned text child record. The handle preserves composition identity while the bounded
/// paragraph records are durable authority that survives reopen and worker migration.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct NoteTextChild {
    pub handle: store::os_store::artifact_child::ArtifactChild<SemioTextSnapshot>,
    #[value(default)]
    pub paragraphs: Vec<NoteTextParagraph>,
}
//#endregion 🔖️ComposedTypes

//#region 🔖️TextBridge
/// 🌉 REAL bidirectional converter between one `NoteBlockNode::Text`'s paragraph/run working
/// representation and the composed child's own `SemioTextSnapshot` (a flat run list — no paragraph
/// grouping). Paragraph boundaries are honestly lossy in one edge case, documented here rather than
/// silently dropped: a boundary is represented as a marks-free, language-free run whose `content` is
/// exactly `"\n"`; this collapses "zero paragraphs" and "one paragraph containing zero runs" onto the
/// same single empty paragraph on the way back (both are legitimate "no text yet" states,
/// indistinguishable once flattened — [`paragraphs_from_text_snapshot`] always emits a trailing
/// paragraph for whatever ran since the last separator, even an empty one). `NoteTextRun.underline`
/// has no equivalent mark in stdio's closed
/// bold/italic/code/link vocabulary and is dropped on the way in — real, honestly-lossy, not fabricated.
pub fn text_snapshot_from_paragraphs(paragraphs: &[NoteTextParagraph]) -> SemioTextSnapshot {
    let mut runs = Vec::new();
    for (index, paragraph) in paragraphs.iter().enumerate() {
        if index > 0 {
            runs.push(SemioTextRun { language: String::new(), content: "\n".into(), marks: Vec::new() });
        }
        for run in &paragraph.runs {
            let mut marks = Vec::new();
            if run.bold == Some(true) {
                marks.push(SemioTextMark { kind: SemioTextMarkKind::Bold, href: String::new() });
            }
            if run.italic == Some(true) {
                marks.push(SemioTextMark { kind: SemioTextMarkKind::Italic, href: String::new() });
            }
            if let Some(href) = &run.link {
                marks.push(SemioTextMark { kind: SemioTextMarkKind::Link, href: href.clone() });
            }
            runs.push(SemioTextRun { language: String::new(), content: run.text.clone(), marks });
        }
    }
    SemioTextSnapshot { schema: STDIO_SEMIOTEXT_DOCUMENT_SCHEMA.into(), runs }
}

/// 🌉 Inverse of [`text_snapshot_from_paragraphs`] — splits the flat run list back into paragraphs on
/// every marks-free/language-free `"\n"` separator run. See that function's doc comment for the one
/// honestly-lossy edge case (empty paragraph list vs. one paragraph with zero runs).
pub fn paragraphs_from_text_snapshot(snapshot: &SemioTextSnapshot) -> Vec<NoteTextParagraph> {
    let mut paragraphs = Vec::new();
    let mut current = Vec::new();
    for run in &snapshot.runs {
        if run.content == "\n" && run.marks.is_empty() && run.language.is_empty() {
            paragraphs.push(NoteTextParagraph { runs: std::mem::take(&mut current) });
            continue;
        }
        let bold = run.marks.iter().any(|mark| mark.kind == SemioTextMarkKind::Bold).then_some(true);
        let italic = run.marks.iter().any(|mark| mark.kind == SemioTextMarkKind::Italic).then_some(true);
        let link = run.marks.iter().find(|mark| mark.kind == SemioTextMarkKind::Link).map(|mark| mark.href.clone());
        current.push(NoteTextRun { text: run.content.clone(), bold, italic, underline: None, link });
    }
    paragraphs.push(NoteTextParagraph { runs: current });
    paragraphs
}

/// 🕸️ Deterministic content-addressed CHILD handle for one text block's composed content — same
/// `(child_id, target)` for identical `(block_id, paragraphs)`, a different pair once either changes;
/// mirrors writer's `document_child_handle`/cad's `cad_model_child_handle`, keyed by `block_id` (not
/// content alone) so two distinct blocks never collide on the same child slot.
pub fn note_text_child_handle(block_id: &str, paragraphs: &[NoteTextParagraph]) -> NoteTextChild {
    let content_json = serde_json::to_string(paragraphs).unwrap_or_default();
    let child_id = store::content_id("note-text", format!("{block_id}\u{1f}{content_json}").as_bytes());
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "text".into() };
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: format!("{block_id}-text"), dialect };
    NoteTextChild { handle: store::os_store::artifact_child::ArtifactChild::new(child_id, target), paragraphs: paragraphs.to_vec() }
}
//#endregion 🔖️TextBridge

//#region 🔖️TextChildren
/// 🔎 Reads the durable paragraphs owned by the text-child record.
pub fn note_block_text(handle: &NoteTextChild) -> Vec<NoteTextParagraph> {
    handle.paragraphs.clone()
}

/// 🏗️ Mints a new content-addressed handle and stores its paragraphs in the same snapshot-owned
/// record used by mutation-diff, fixture, and converter builders.
pub fn note_text_child_record(block_id: &str, paragraphs: &[NoteTextParagraph]) -> NoteTextChild {
    note_text_child_handle(block_id, paragraphs)
}
//#endregion 🔖️TextChildren

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "r")]
pub struct NoteTextRun {
    #[dsl(positional)]
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub underline: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "p")]
pub struct NoteTextParagraph {
    pub runs: Vec<NoteTextRun>,
}

pub fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct NoteTableCell {
    #[dsl(positional)]
    pub content: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct NoteImageAsset {
    pub mime: String,
    pub data: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
}

pub use crate::schema::diff::NoteDiff;
pub use crate::schema::mutations::NoteMutation;
pub use crate::schema::snapshot::NoteSnapshot;

//#endregion 🔖️Domain
}
//#endregion 🧩️ArtifactItems4
