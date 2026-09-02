//! 🤝️ Block plugin — record types shared by all three artifacts' document entities (non-constitutional
//! cross-artifact kernel; see the constitutional-split recipe's "shared code used by ≥2 artifacts" rule).
//! Dimension-specific nouns (handle/vortex/grip kinds and their placement templates) stay per-artifact —
//! only the identity/metadata/compatibility/representation/camera shapes common to every dimension live
//! here, reached as `crate::*` from every `🗿️artifacts/<a>` node.

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp};

//#region 🗃️Apps
/// 🗃️ Closed runtime app fleet for the block editor and viewer surfaces.
semio_framework_dispatch_macros::dyn_enum_close! {
    pub enum BlockApps: PluginApp {
        Block2dEditor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::block2d::Block2dPlayApp>>),
        Block2dViewer(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::block2d::Block2dViewer>>),
        Block3dEditor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::block3d::Block3dPlayApp>>),
        Block3dViewer(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::block3d::Block3dViewer>>),
        Block5dEditor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::block5d::Block5dPlayApp>>),
        Block5dViewer(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::block5d::Block5dViewer>>),
    }
}
//#endregion 🗃️Apps

//#region 🔖️Identity
/// 🪪️ The single kind definition a block document edits — name/label/variant/description/icon/unit
/// apply uniformly whether the document is a `NodeKind` (2d), `ObjectKind` (3d) or `PartKind` (5d).
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct BlockKindIdentity {
    pub id: String,
    pub name: String,
    pub label: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    pub variant: Option<String>,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    pub description: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    pub icon: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    pub unit: Option<String>,
}
//#endregion 🔖️Identity

//#region 🔖️Metadata
/// 🏷️ One free-form key/value attribute on a kind (optionally naming the attribute definition it
/// instantiates).
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct BlockAttribute {
    pub key: String,
    pub value: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    pub definition: Option<String>,
}

/// 👤️ One author credited on a kind.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct BlockAuthor {
    pub id: String,
    pub name: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    pub email: Option<String>,
}

/// 🔗️ One allowed (or, unidirectional, one-way-allowed) compatibility pair between two handle/vortex/
/// grip kind ids — the `id` lets ops remove a specific row without re-keying on `(source, target)`.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct BlockCompatibilityRule {
    pub id: String,
    pub source: String,
    pub target: String,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    pub bidirectional: bool,
}

/// 🧱️ One representation (mesh at a LOD/tag combination) a kind ships with — semio_compose_rs's "Representation".
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct BlockRepresentation {
    pub id: String,
    pub name: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    pub mesh_url: Option<String>,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    pub tags: Vec<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    pub lod: Option<String>,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    pub description: String,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[dsl(table)]
    pub attributes: Vec<BlockAttribute>,
}
//#endregion 🔖️Metadata

//#region 🔖️Cameras
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct BlockCamera2d {
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    pub x: f64,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    pub y: f64,
    #[value(default = "block_one_f64")]
    #[cfg_attr(test, serde(default = "block_one_f64"))]
    pub zoom: f64,
}

impl Default for BlockCamera2d {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct BlockCamera3d {
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[dsl(coord)]
    pub position: [f64; 3],
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[dsl(coord)]
    pub target: [f64; 3],
    #[value(default = "block_one_f64")]
    #[cfg_attr(test, serde(default = "block_one_f64"))]
    pub zoom: f64,
}

impl Default for BlockCamera3d {
    fn default() -> Self {
        Self { position: [0.0, 0.0, 0.0], target: [0.0, 0.0, 0.0], zoom: 1.0 }
    }
}

async fn block_one_f64() -> f64 {
    1.0
}
//#endregion 🔖️Cameras

//#region 🔖️Meta
/// 📝️ Free-text description carried alongside a block document (distinct from the kind's own
/// `BlockKindIdentity::description`, which describes the kind; this describes the editing session).
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct BlockMeta {
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    pub description: String,
}
//#endregion 🔖️Meta

//#region 🔌️Registration
/// 🔌️ Builds the block plugin surface for host registration. Atomic cutover (ticket
/// 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME, `descriptor-prep`, following `🔱️trinity`'s
/// `fleet-trinity-recipe`): `.declare_artifact(…)` (new declaration tree) replaces
/// `.artifact(declaration())`/`.editor::<>()`/`.viewer::<>()` for ALL THREE owned artifacts outright
/// — the old channel is NOT kept alongside it. `require_declared_capability_or_record`'s exact
/// sorted-claims equality check (the old channel's `try_build()` path) is why
/// `.artifact(block2d::declaration())` etc. failed assembly (`"no declared <kind> capability owns the
/// runtime claims"`); the new channel never runs that check. `.editor_mutation_roster()`/
/// `.viewer_mutation_roster()` stay: orthogonal, still-supported opt-ins the new declaration tree's
/// `SurfaceDeclaration.mutation_roster` does not yet wire live — not a second registration of the
/// artifact/schema/io itself. `.activation(…)`/`.execution(…)`/`.requests(…)` (ticket
/// 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M6-remaining, `📓️design-abi.md` §3/§6) are this
/// crate's migration proof: one `OnArtifactKind` event per owned kind, read live from each
/// dimension's own `artifact_kind().id`, `Isolated` execution, one `documents.write` ask covering
/// all three editors' persisted mutations.
pub fn plugin() -> Result<Plugin<BlockApps>, semio_framework_plugin::PluginAssemblyError> {
    Plugin::<BlockApps>::builder("block")
        .label("Block")
        .version("0.1.0")
        .declare_artifact(crate::artifacts::block2d::artifact())
        .declare_artifact(crate::artifacts::block3d::artifact())
        .declare_artifact(crate::artifacts::block5d::artifact())
        .editor_mutation_roster::<crate::editor::block2d::Block2dPlayApp>()
        .viewer_mutation_roster::<crate::viewer::block2d::Block2dViewer>()
        .editor_mutation_roster::<crate::editor::block3d::Block3dPlayApp>()
        .viewer_mutation_roster::<crate::viewer::block3d::Block3dViewer>()
        .editor_mutation_roster::<crate::editor::block5d::Block5dPlayApp>()
        .viewer_mutation_roster::<crate::viewer::block5d::Block5dViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::block2d::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::block3d::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::block5d::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist block2d/block3d/block5d edits to the open document".into(), optional: false })
        .try_build()
}
//#endregion 🔌️Registration

//#region 🧪️SurfaceTests
/// 👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.5: proves each subset's
/// viewer never mutates the document/draft store at runtime and that every editor/viewer pair shares
/// the same `Dialect` — real framework testkit functions (W0-F gap closure), not local stand-ins.
#[cfg(test)]
mod surface_tests {
    #[semio_framework_async_macros::async_test]
    async fn block2d_viewer_never_mutates() {
        semio_framework_plugin::testkit::assert_viewer_never_mutates::<crate::viewer::block2d::Block2dViewer>();
    }

    #[semio_framework_async_macros::async_test]
    async fn block2d_editor_and_viewer_share_dialect() {
        semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<crate::editor::block2d::Block2dPlayApp, crate::viewer::block2d::Block2dViewer>();
    }

    #[semio_framework_async_macros::async_test]
    async fn block3d_viewer_never_mutates() {
        semio_framework_plugin::testkit::assert_viewer_never_mutates::<crate::viewer::block3d::Block3dViewer>();
    }

    #[semio_framework_async_macros::async_test]
    async fn block3d_editor_and_viewer_share_dialect() {
        semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<crate::editor::block3d::Block3dPlayApp, crate::viewer::block3d::Block3dViewer>();
    }

    #[semio_framework_async_macros::async_test]
    async fn block5d_viewer_never_mutates() {
        semio_framework_plugin::testkit::assert_viewer_never_mutates::<crate::viewer::block5d::Block5dViewer>();
    }

    #[semio_framework_async_macros::async_test]
    async fn block5d_editor_and_viewer_share_dialect() {
        semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<crate::editor::block5d::Block5dPlayApp, crate::viewer::block5d::Block5dViewer>();
    }
}
//#endregion 🧪️SurfaceTests
