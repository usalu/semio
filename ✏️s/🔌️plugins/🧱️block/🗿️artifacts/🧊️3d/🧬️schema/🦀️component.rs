//! 🧬️ Block3d artifact schema — every field with its state class.

use crate::artifacts::block3d::{Block3dVortexKind, Block3dVortexTemplate, Block3dSnapshot, BLOCK_3D_SCHEMA};
use crate::artifacts::block3d::{Block3dBrushPreview, Block3dWindowView};
use crate::{BlockAttribute, BlockAuthor, BlockCamera3d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta, BlockRepresentation};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full block3d artifact state across persistent, shared-ui, local-ui and preview classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.block.block3d")]
pub struct Block3dArtifact {
    #[state(persistent)] pub schema: String,
    #[state(persistent)] pub object_kind: BlockKindIdentity,
    #[state(persistent)] pub representations: Vec<BlockRepresentation>,
    #[state(persistent)] pub vortex_kinds: Vec<Block3dVortexKind>,
    #[state(persistent)] pub vortices: Vec<Block3dVortexTemplate>,
    #[state(persistent)] pub compatibility: Vec<BlockCompatibilityRule>,
    #[state(persistent)] pub attributes: Vec<BlockAttribute>,
    #[state(persistent)] pub authors: Vec<BlockAuthor>,
    #[state(persistent)] pub camera3d: BlockCamera3d,
    #[state(persistent)] pub meta: BlockMeta,
    #[state(shared_ui)] pub selected_ids: Vec<String>,
    #[state(shared_ui)] pub active_representation_id: Option<String>,
    #[state(shared_ui)] pub wanted_tags: Vec<String>,
    #[state(local_ui)] pub locale: String,
    #[state(local_ui)] pub windows: Vec<Block3dWindowView>,
    #[state(local_ui)] pub brush_vortex_kind_id: Option<String>,
    #[state(local_ui)] pub brush_radius: f64,
    #[state(local_ui)] pub brush_flip: bool,
    #[state(preview)] pub brush_preview: Option<Block3dBrushPreview>,
    #[state(local_ui)] pub camera: Option<BlockCamera3d>,
    #[state(preview)] pub hovered_vortex_full_id: Option<String>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for Block3dArtifact {
    fn default() -> Self {
        Self::from_snapshot(Block3dSnapshot::default())
    }
}

impl Block3dArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> Block3dSnapshot {
        Block3dSnapshot {
            schema: self.schema.clone(),
            object_kind: self.object_kind.clone(),
            representations: self.representations.clone(),
            vortex_kinds: self.vortex_kinds.clone(),
            vortices: self.vortices.clone(),
            compatibility: self.compatibility.clone(),
            attributes: self.attributes.clone(),
            authors: self.authors.clone(),
            camera3d: self.camera3d.clone(),
            meta: self.meta.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: Block3dSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            object_kind: snapshot.object_kind,
            representations: snapshot.representations,
            vortex_kinds: snapshot.vortex_kinds,
            vortices: snapshot.vortices,
            compatibility: snapshot.compatibility,
            attributes: snapshot.attributes,
            authors: snapshot.authors,
            camera3d: snapshot.camera3d,
            meta: snapshot.meta,
            selected_ids: Vec::new(),
            active_representation_id: None,
            wanted_tags: Vec::new(),
            locale: "en-US".into(),
            windows: Vec::new(),
            brush_vortex_kind_id: None,
            brush_radius: 0.25,
            brush_flip: false,
            brush_preview: None,
            camera: None,
            hovered_vortex_full_id: None,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: Block3dSnapshot) {
        self.schema = snapshot.schema;
        self.object_kind = snapshot.object_kind;
        self.representations = snapshot.representations;
        self.vortex_kinds = snapshot.vortex_kinds;
        self.vortices = snapshot.vortices;
        self.compatibility = snapshot.compatibility;
        self.attributes = snapshot.attributes;
        self.authors = snapshot.authors;
        self.camera3d = snapshot.camera3d;
        self.meta = snapshot.meta;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.block.block3d` — fifteen handcrafted schema leaves.
pub fn block3d_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.block.block3d",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("../📸️snapshot/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../📸️snapshot/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../📸️snapshot/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../📸️snapshot/🧬️schema/🔣️component.json"),
            proto: include_str!("../📸️snapshot/🧬️schema/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("../🔺️diff/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../🔺️diff/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../🔺️diff/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../🔺️diff/🧬️schema/🔣️component.json"),
            proto: include_str!("../🔺️diff/🧬️schema/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
