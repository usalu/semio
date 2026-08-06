//! 🏙️ Block 3D artifact — the document entity the 🧊️3d app edits (constitutional: general). Edits
//! exactly one `ObjectKind`: its identity, representations (meshes at LOD/tags — the semio_compose_rs
//! `type` app's successor), and the `VortexKind` templates placed on its rim.

use crate::core::{BlockAttribute, BlockAuthor, BlockCamera3d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta, BlockRepresentation};
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

/// 🏙️ The block-3d projection: a typed single-`ObjectKind`-definition document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "block.block3d", layout = "lines")]
pub struct Block3dDefinition {
    pub schema: String,
    #[dsl(block)]
    pub object_kind: BlockKindIdentity,
    #[serde(default)]
    #[dsl(table)]
    pub representations: Vec<BlockRepresentation>,
    #[serde(default)]
    #[dsl(table)]
    pub vortex_kinds: Vec<Block3dVortexKind>,
    #[serde(default)]
    #[dsl(table)]
    pub vortices: Vec<Block3dVortexTemplate>,
    #[serde(default)]
    #[dsl(table)]
    pub compatibility: Vec<BlockCompatibilityRule>,
    #[serde(default)]
    #[dsl(table)]
    pub attributes: Vec<BlockAttribute>,
    #[serde(default)]
    #[dsl(table)]
    pub authors: Vec<BlockAuthor>,
    #[dsl(block)]
    #[serde(default)]
    pub camera3d: BlockCamera3d,
    #[dsl(block)]
    #[serde(default)]
    pub meta: BlockMeta,
}

impl Default for Block3dDefinition {
    fn default() -> Self {
        Self {
            schema: BLOCK_3D_SCHEMA.to_string(),
            object_kind: BlockKindIdentity::default(),
            representations: Vec::new(),
            vortex_kinds: Vec::new(),
            vortices: Vec::new(),
            compatibility: Vec::new(),
            attributes: Vec::new(),
            authors: Vec::new(),
            camera3d: BlockCamera3d::default(),
            meta: BlockMeta::default(),
        }
    }
}
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
