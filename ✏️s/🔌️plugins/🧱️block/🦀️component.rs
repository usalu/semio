//! 🤝️ Block plugin — record types shared by all three artifacts' document entities (non-constitutional
//! cross-artifact kernel; see the constitutional-split recipe's "shared code used by ≥2 artifacts" rule).
//! Dimension-specific nouns (handle/vortex/grip kinds and their placement templates) stay per-artifact —
//! only the identity/metadata/compatibility/representation/camera shapes common to every dimension live
//! here, reached as `crate::*` from every `🗿️artifacts/<a>` node.

use semio_framework_plugin::Plugin;
use serde::{Deserialize, Serialize};

//#region 🔖️Identity
/// 🪪️ The single kind definition a block document edits — name/label/variant/description/icon/unit
/// apply uniformly whether the document is a `NodeKind` (2d), `ObjectKind` (3d) or `PartKind` (5d).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct BlockKindIdentity {
    pub id: String,
    pub name: String,
    pub label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}
//#endregion 🔖️Identity

//#region 🔖️Metadata
/// 🏷️ One free-form key/value attribute on a kind (optionally naming the attribute definition it
/// instantiates).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct BlockAttribute {
    pub key: String,
    pub value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
}

/// 👤️ One author credited on a kind.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct BlockAuthor {
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

/// 🔗️ One allowed (or, unidirectional, one-way-allowed) compatibility pair between two handle/vortex/
/// grip kind ids — the `id` lets ops remove a specific row without re-keying on `(source, target)`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct BlockCompatibilityRule {
    pub id: String,
    pub source: String,
    pub target: String,
    #[serde(default)]
    pub bidirectional: bool,
}

/// 🧱️ One representation (mesh at a LOD/tag combination) a kind ships with — semio_compose_rs's "Representation".
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct BlockRepresentation {
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mesh_url: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lod: Option<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    #[dsl(table)]
    pub attributes: Vec<BlockAttribute>,
}
//#endregion 🔖️Metadata

//#region 🔖️Cameras
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct BlockCamera2d {
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    #[serde(default = "block_one_f64")]
    pub zoom: f64,
}

impl Default for BlockCamera2d {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct BlockCamera3d {
    #[serde(default)]
    #[dsl(coord)]
    pub position: [f64; 3],
    #[serde(default)]
    #[dsl(coord)]
    pub target: [f64; 3],
    #[serde(default = "block_one_f64")]
    pub zoom: f64,
}

impl Default for BlockCamera3d {
    fn default() -> Self {
        Self { position: [0.0, 0.0, 0.0], target: [0.0, 0.0, 0.0], zoom: 1.0 }
    }
}

fn block_one_f64() -> f64 {
    1.0
}
//#endregion 🔖️Cameras

//#region 🔖️Meta
/// 📝️ Free-text description carried alongside a block document (distinct from the kind's own
/// `BlockKindIdentity::description`, which describes the kind; this describes the editing session).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct BlockMeta {
    #[serde(default)]
    pub description: String,
}
//#endregion 🔖️Meta

//#region 🔌️Registration
/// 🔌️ Builds the block plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1/W1d) replaces the old
/// `.setup(register_block_exports)` escape hatch for all three artifacts — block2d's own
/// registration surface (`crate::artifacts::block2d::declaration()`) landed once its concurrent
/// `⚙️engine`-dissolution restructure settled. Every app's CONFIG/PRESENCE schema now registers via
/// `ArtifactApp::app_schema()` (ticket W1c) instead — an app-scope concern `ArtifactDeclaration`
/// has no field for by design (see that struct's doc) — so `.setup()` is gone from this plugin
/// entirely.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("block")
        .label("Block")
        .version("0.1.0")
        .artifact(crate::artifacts::block2d::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .artifact(crate::artifacts::block3d::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .artifact(crate::artifacts::block5d::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .editor::<crate::editor::block2d::Block2dPlayApp>(crate::editor::block2d::create_block2d_app())
        .viewer::<crate::viewer::block2d::Block2dViewer>(crate::viewer::block2d::create_block2d_viewer())
        .editor::<crate::editor::block3d::Block3dPlayApp>(crate::editor::block3d::create_block3d_app())
        .viewer::<crate::viewer::block3d::Block3dViewer>(crate::viewer::block3d::create_block3d_viewer())
        .editor::<crate::editor::block5d::Block5dPlayApp>(crate::editor::block5d::create_block5d_app())
        .viewer::<crate::viewer::block5d::Block5dViewer>(crate::viewer::block5d::create_block5d_viewer())
        .try_build()
}
//#endregion 🔌️Registration

//#region 🧪️SurfaceTests
/// 👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.5: proves each subset's
/// viewer never mutates the document/draft store at runtime and that every editor/viewer pair shares
/// the same `Dialect` — real framework testkit functions (W0-F gap closure), not local stand-ins.
#[cfg(test)]
mod surface_tests {
    #[test]
    fn block2d_viewer_never_mutates() {
        semio_framework_plugin::testkit::assert_viewer_never_mutates::<crate::viewer::block2d::Block2dViewer>();
    }

    #[test]
    fn block2d_editor_and_viewer_share_dialect() {
        semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<crate::editor::block2d::Block2dPlayApp, crate::viewer::block2d::Block2dViewer>();
    }

    #[test]
    fn block3d_viewer_never_mutates() {
        semio_framework_plugin::testkit::assert_viewer_never_mutates::<crate::viewer::block3d::Block3dViewer>();
    }

    #[test]
    fn block3d_editor_and_viewer_share_dialect() {
        semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<crate::editor::block3d::Block3dPlayApp, crate::viewer::block3d::Block3dViewer>();
    }

    #[test]
    fn block5d_viewer_never_mutates() {
        semio_framework_plugin::testkit::assert_viewer_never_mutates::<crate::viewer::block5d::Block5dViewer>();
    }

    #[test]
    fn block5d_editor_and_viewer_share_dialect() {
        semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<crate::editor::block5d::Block5dPlayApp, crate::viewer::block5d::Block5dViewer>();
    }
}
//#endregion 🧪️SurfaceTests
