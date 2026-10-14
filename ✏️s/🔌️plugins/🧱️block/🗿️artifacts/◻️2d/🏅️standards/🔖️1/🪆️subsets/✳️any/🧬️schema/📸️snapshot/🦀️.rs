//! 🧬️ Block2d snapshot schema — artifact-lane fields only.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_plugin_block::{BlockAttribute, BlockAuthor, BlockCamera2d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta};
use ::semio_framework_schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted block2d document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(id = "block.block2d", layout = "lines")]
#[artifact_schema(id = "s.block.block2d")]
pub struct Block2dSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[dsl(block)]
    #[state(artifact)]
    pub node_kind: BlockKindIdentity,
    #[dsl(block)]
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[state(artifact)]
    pub presentation: Block2dPresentation,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[dsl(table)]
    #[state(artifact)]
    pub handle_kinds: Vec<Block2dHandleKind>,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[dsl(table)]
    #[state(artifact)]
    pub handles: Vec<Block2dHandleTemplate>,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[dsl(table)]
    #[state(artifact)]
    pub compatibility: Vec<BlockCompatibilityRule>,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[dsl(table)]
    #[state(artifact)]
    pub attributes: Vec<BlockAttribute>,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[dsl(table)]
    #[state(artifact)]
    pub authors: Vec<BlockAuthor>,
    #[dsl(block)]
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[state(artifact)]
    pub camera2d: BlockCamera2d,
    #[dsl(block)]
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[state(artifact)]
    pub meta: BlockMeta,
}
//#endregion 🔖️Snapshot

impl Default for Block2dSnapshot {
    fn default() -> Self {
        Self {
            schema: BLOCK_2D_SCHEMA.to_string(),
            node_kind: BlockKindIdentity::default(),
            presentation: Block2dPresentation::default(),
            handle_kinds: Vec::new(),
            handles: Vec::new(),
            compatibility: Vec::new(),
            attributes: Vec::new(),
            authors: Vec::new(),
            camera2d: BlockCamera2d::default(),
            meta: BlockMeta::default(),
        }
    }
}

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `snapshot` schema facet.
pub const SNAPSHOT_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") };
//#endregion 🍃️FacetLeaves

use semio_framework_plugin::ArtifactKindSpec;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::OsMediaCapability;

pub const BLOCK_2D_SCHEMA: &str = "block.2d";

/// 🔵️ The node's own rim presentation — mirrors `Puzzle2dNode`'s shape fields, minus placement (a
/// kind definition has no x/y — those belong to the puzzle assembly, not the definition).
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Block2dPresentation {
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    pub shape: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    pub radius: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    pub width: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    pub height: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    pub color: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    pub icon_kind: Option<String>,
}

/// 🔘️ One handle-kind catalog row this node kind ships with.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Block2dHandleKind {
    #[dsl(defines = "handle_kind")]
    pub id: String,
    pub name: String,
    pub label: String,
    pub color: String,
    pub default_wire_kind: String,
}

/// 🌱️ One rim-handle template — where a handle of `handle_kind` sits on the node's rim.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Block2dHandleTemplate {
    pub id: String,
    #[dsl(refs = "handle_kind")]
    pub handle_kind: String,
    #[dsl(angle = "rad")]
    pub angle: f64,
    pub radius: f64,
}

/// 🎯️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET: the one `Dialect` coordinate every
/// surface (`✏️editor`, `👁️viewer`) of the `✳️any` subset binds `ArtifactEditor::DIALECT`/
/// `ArtifactViewer::DIALECT` to — `"s.block.block2d"` matches this file's own `definition()` row
/// (`"s.block2d.schema.artifact"`, descriptor `"s.block.block2d"`), standard `"1"` and subset `"*"`
/// match this file's own `🏅️standards/🔖️1/🪆️subsets/✳️any` location. Lives at the artifact level
/// (not under `editor`/`viewer`) so `policyViewerPurityBreaches` never sees a viewer file importing
/// through the sibling editor module just to read this constant.
pub const BLOCK2D_DIALECT: semio_framework_artifact_reference::Dialect = semio_framework_artifact_reference::Dialect { artifact_kind: "s.block.block2d", standard: semio_framework_artifact_reference::StandardId("1"), subset: semio_framework_artifact_reference::SubsetId::ANY };

/// 🗂️ This artifact's `ArtifactKindSpec` — the canonical `2d.block` declaration, stitched into
/// `crate::editor::block2d::create_block2d_app`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "2d.block".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Node Kind", "Knotentyp"),
        source_format: BLOCK_2D_SCHEMA.into(),
        component_kind: "block2d".into(),
        dimension: "2d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
        schema: BLOCK_2D_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.json".into(), "stdio.txt".into(), "stdio.zip".into()],
        import_stdio_kinds: vec!["stdio.json".into(), "stdio.txt".into(), "stdio.zip".into()],
    }
}
