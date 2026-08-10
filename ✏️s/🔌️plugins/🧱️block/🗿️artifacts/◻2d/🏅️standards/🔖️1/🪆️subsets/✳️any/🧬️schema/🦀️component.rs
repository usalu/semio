//! 🧬️ Block2d artifact schema — every field with its state class.

use crate::artifacts::block2d::{Block2dHandleKind, Block2dHandleTemplate, Block2dPresentation, Block2dSnapshot, BLOCK_2D_SCHEMA};
use crate::{BlockAttribute, BlockAuthor, BlockCamera2d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full block2d artifact state across persistent, shared-ui and local-ui classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.block.block2d")]
pub struct Block2dArtifact {
    #[state(persistent)] pub schema: String,
    #[state(persistent)] pub node_kind: BlockKindIdentity,
    #[state(persistent)] pub presentation: Block2dPresentation,
    #[state(persistent)] pub handle_kinds: Vec<Block2dHandleKind>,
    #[state(persistent)] pub handles: Vec<Block2dHandleTemplate>,
    #[state(persistent)] pub compatibility: Vec<BlockCompatibilityRule>,
    #[state(persistent)] pub attributes: Vec<BlockAttribute>,
    #[state(persistent)] pub authors: Vec<BlockAuthor>,
    #[state(persistent)] pub camera2d: BlockCamera2d,
    #[state(persistent)] pub meta: BlockMeta,
    #[state(shared_ui)] pub selected_ids: Vec<String>,
    #[state(local_ui)] pub locale: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for Block2dArtifact {
    fn default() -> Self {
        Self::from_snapshot(Block2dSnapshot::default())
    }
}

impl Block2dArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> Block2dSnapshot {
        Block2dSnapshot {
            schema: self.schema.clone(),
            node_kind: self.node_kind.clone(),
            presentation: self.presentation.clone(),
            handle_kinds: self.handle_kinds.clone(),
            handles: self.handles.clone(),
            compatibility: self.compatibility.clone(),
            attributes: self.attributes.clone(),
            authors: self.authors.clone(),
            camera2d: self.camera2d.clone(),
            meta: self.meta.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: Block2dSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            node_kind: snapshot.node_kind,
            presentation: snapshot.presentation,
            handle_kinds: snapshot.handle_kinds,
            handles: snapshot.handles,
            compatibility: snapshot.compatibility,
            attributes: snapshot.attributes,
            authors: snapshot.authors,
            camera2d: snapshot.camera2d,
            meta: snapshot.meta,
            selected_ids: Vec::new(),
            locale: "en-US".into(),
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: Block2dSnapshot) {
        self.schema = snapshot.schema;
        self.node_kind = snapshot.node_kind;
        self.presentation = snapshot.presentation;
        self.handle_kinds = snapshot.handle_kinds;
        self.handles = snapshot.handles;
        self.compatibility = snapshot.compatibility;
        self.attributes = snapshot.attributes;
        self.authors = snapshot.authors;
        self.camera2d = snapshot.camera2d;
        self.meta = snapshot.meta;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.block.block2d` — fifteen handcrafted schema leaves.
pub fn block2d_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.block.block2d",
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
    }
}
//#endregion 🔖️Descriptor
