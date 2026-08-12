//! 🏙️ Block 3D artifact — the document entity the 🧊️3d app edits (constitutional: general). Edits
//! exactly one `ObjectKind`: its identity, representations (meshes at LOD/tags — the semio_compose_rs
//! `type` app's successor), and the `VortexKind` templates placed on its rim.


pub use crate::artifacts::block3d::schema::snapshot::Block3dSnapshot;
pub use crate::artifacts::block3d::schema::mutations::Block3dMutation;
pub use crate::artifacts::block3d::schema::diff::Block3dDiff;

use crate::{BlockAttribute, BlockAuthor, BlockCamera3d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta, BlockRepresentation};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use serde::{Deserialize, Serialize};



pub const BLOCK_3D_SCHEMA: &str = "block.3d";

// #region 🔖️Document
/// 🔘️ One vortex-kind catalog row this object kind ships with.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Block3dVortexKind {
    #[dsl(defines = "vortex_kind")]
    pub id: String,
    pub name: String,
    pub label: String,
    pub color: String,
    pub default_cable_kind: String,
}

/// 🌱️ One rim-vortex template — where a vortex of `vortex_kind` sits on the object's surface.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Block3dVortexTemplate {
    pub id: String,
    #[dsl(refs = "vortex_kind")]
    pub vortex_kind: String,
    #[serde(default)]
    #[dsl(coord)]
    pub position: [f64; 3],
    #[serde(default)]
    #[dsl(dir)]
    pub direction: [f64; 3],
    #[serde(default)]
    pub radius: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}


//#region 🔖️WindowView
/// 🪟 Per-window-instance view state (representation subset, layout, active utility).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Block3dWindowView {
    pub window_id: String,
    #[serde(default)]
    pub representation_ids: Vec<String>,
    #[serde(default = "default_arrangement")]
    pub arrangement: String,
    #[serde(default = "default_spacing")]
    pub spacing: f64,
    #[serde(default = "default_active_utility")]
    pub active_utility: String,
}

fn default_arrangement() -> String {
    "overlap".into()
}

fn default_spacing() -> f64 {
    8.0
}

fn default_active_utility() -> String {
    crate::apps::block3d::BLOCK3D_UTILITY_SELECT.into()
}

impl Block3dWindowView {
    /// 🪟 Builds a default view record for one window id.
    pub fn for_window(window_id: impl Into<String>) -> Self {
        Self { window_id: window_id.into(), representation_ids: Vec::new(), arrangement: default_arrangement(), spacing: default_spacing(), active_utility: default_active_utility() }
    }
}

/// 🖌️ Transient brush hover pose in world space (config/preview).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Block3dBrushPreview {
    #[dsl(coord)]
    pub position: [f64; 3],
    #[dsl(dir)]
    pub direction: [f64; 3],
}
//#endregion 🔖️WindowView

//#region 🔖️Snapshot
//#endregion 🔖️Snapshot

// #endregion 🔖️Document

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — the canonical `3d.block` declaration, stitched into
/// `crate::apps::block3d::create_block3d_app`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "3d.block".into(),
        name: "Object Kind".into(),
        source_format: BLOCK_3D_SCHEMA.into(),
        component_kind: "block3d".into(),
        dimension: "3d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
        schema: BLOCK_3D_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec!["stdio.json", "stdio.obj", "stdio.png", "stdio.stl", "stdio.zip"],
        import_stdio_kinds: vec!["stdio.json", "stdio.obj", "stdio.png", "stdio.stl", "stdio.zip"],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artifact_kind_declares_the_3d_block_interchange_kind() {
        let kind = artifact_kind();
        assert_eq!(kind.id, "3d.block");
        assert_eq!(kind.schema, BLOCK_3D_SCHEMA);
        assert_eq!(kind.component_kind, "block3d");
    }
}
//#endregion 🧪️Tests
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
    use crate::artifacts::block3d::standards::v1::engine::io_registry as v1;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("Block3dComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v1::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
