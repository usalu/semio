//! 🧬️ Block5d snapshot schema — artifact-lane fields only.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_plugin_block::{BlockAttribute, BlockAuthor, BlockCamera2d, BlockCamera3d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta, BlockRepresentation};
use ::semio_framework_schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted block5d document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(id = "block.block5d", layout = "lines")]
#[artifact_schema(id = "s.block.block5d")]
pub struct Block5dSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[dsl(block)]
    #[state(artifact)]
    pub part_kind: BlockKindIdentity,
    #[dsl(block)]
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[state(artifact)]
    pub part_2d: Block5dPart2d,
    #[dsl(block)]
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[state(artifact)]
    pub part_3d: Block5dPart3d,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[dsl(table)]
    #[state(artifact)]
    pub representations: Vec<BlockRepresentation>,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[dsl(table)]
    #[state(artifact)]
    pub grip_kinds: Vec<Block5dGripKind>,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[dsl(table)]
    #[state(artifact)]
    pub grips: Vec<Block5dGripTemplate>,
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
    pub camera3d: BlockCamera3d,
    #[dsl(block)]
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[state(artifact)]
    pub meta: BlockMeta,
}
//#endregion 🔖️Snapshot

impl Default for Block5dSnapshot {
    fn default() -> Self {
        Self {
            schema: BLOCK_5D_SCHEMA.to_string(),
            part_kind: BlockKindIdentity::default(),
            part_2d: Block5dPart2d::default(),
            part_3d: Block5dPart3d::default(),
            representations: Vec::new(),
            grip_kinds: Vec::new(),
            grips: Vec::new(),
            compatibility: Vec::new(),
            attributes: Vec::new(),
            authors: Vec::new(),
            camera2d: BlockCamera2d::default(),
            camera3d: BlockCamera3d::default(),
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

pub const BLOCK_5D_SCHEMA: &str = "block.5d";

/// 🎯️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET: the one `Dialect` coordinate every
/// surface (`✏️editor`, `👁️viewer`) of the `✳️any` subset binds `ArtifactEditor::DIALECT`/
/// `ArtifactViewer::DIALECT` to — `"s.block.block5d"` matches this file's own `definition()`'s
/// `"s.block5d.schema.artifact"` row descriptor, standard `"1"` and subset `"*"` match this file's
/// own `🏅️standards/🔖️1/🪆️subsets/✳️any` location. Lives at the ARTIFACT level (not under
/// `editor`/`viewer`) so a viewer file can read it without ever importing through the sibling editor
/// module.
pub const BLOCK5D_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.block.block5d", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };

/// 🔵️ The part's 2D-projection presentation (board node).
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Block5dPart2d {
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

/// 🧱️ The part's 3D-projection presentation (world object) — pose defaults only; the mesh itself
/// comes from `representations`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Block5dPart3d {
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    pub orientation: Option<[f64; 4]>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    pub scale: Option<[f64; 3]>,
}

/// 🔘️ One grip-kind catalog row this part kind ships with.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Block5dGripKind {
    #[dsl(defines = "grip_kind")]
    pub id: String,
    pub name: String,
    pub label: String,
    pub color: String,
    pub default_rope_kind: String,
}

/// 🌱️ One rim-grip template, unified across both projections — flat scalar fields (no nested 2d/3d
/// sub-records) to dodge the pack table-column bug noted on `Block5dPart3d` above.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Block5dGripTemplate {
    pub id: String,
    #[dsl(refs = "grip_kind")]
    pub grip_kind: String,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[dsl(angle = "rad")]
    pub angle: f64,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    pub radius_2d: f64,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[dsl(coord)]
    pub position: [f64; 3],
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[dsl(dir)]
    pub direction: [f64; 3],
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    pub radius_3d: f64,
}

/// 🗂️ This artifact's `ArtifactKindSpec` — the canonical `5d.block` declaration, stitched into
/// `crate::editor::block5d::create_block5d_app`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "5d.block".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Part Kind", "Bauteiltyp"),
        source_format: BLOCK_5D_SCHEMA.into(),
        component_kind: "block5d".into(),
        dimension: "5d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
        schema: BLOCK_5D_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.json".into(), "stdio.txt".into(), "stdio.zip".into()],
        import_stdio_kinds: vec!["stdio.json".into(), "stdio.txt".into(), "stdio.zip".into()],
    }
}

extern crate self as semio_s_artifact_block_5d_model;
extern crate semio_framework_pack as pack;
#[path = "../../🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
pub mod binary_snapshot;
#[path = "../../🚪️io/📝️text/📸️snapshot/🦀️.rs"]
pub mod text_snapshot;
#[path = "../../🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs"]
pub mod sqlite_snapshot;
