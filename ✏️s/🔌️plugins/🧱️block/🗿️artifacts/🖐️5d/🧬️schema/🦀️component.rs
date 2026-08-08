//! 🧬️ Block5d artifact schema — every field with its state class.

use crate::artifacts::block5d::{Block5dGripKind, Block5dGripTemplate, Block5dPart2d, Block5dPart3d, Block5dSnapshot, BLOCK_5D_SCHEMA};
use crate::{BlockAttribute, BlockAuthor, BlockCamera2d, BlockCamera3d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta, BlockRepresentation};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full block5d artifact state across persistent, shared-ui and local-ui classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.block.block5d")]
pub struct Block5dArtifact {
    #[state(persistent)] pub schema: String,
    #[state(persistent)] pub part_kind: BlockKindIdentity,
    #[state(persistent)] pub part_2d: Block5dPart2d,
    #[state(persistent)] pub part_3d: Block5dPart3d,
    #[state(persistent)] pub representations: Vec<BlockRepresentation>,
    #[state(persistent)] pub grip_kinds: Vec<Block5dGripKind>,
    #[state(persistent)] pub grips: Vec<Block5dGripTemplate>,
    #[state(persistent)] pub compatibility: Vec<BlockCompatibilityRule>,
    #[state(persistent)] pub attributes: Vec<BlockAttribute>,
    #[state(persistent)] pub authors: Vec<BlockAuthor>,
    #[state(persistent)] pub camera2d: BlockCamera2d,
    #[state(persistent)] pub camera3d: BlockCamera3d,
    #[state(persistent)] pub meta: BlockMeta,
    #[state(shared_ui)] pub selected_ids: Vec<String>,
    #[state(local_ui)] pub locale: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for Block5dArtifact {
    fn default() -> Self {
        Self::from_snapshot(Block5dSnapshot::default())
    }
}

impl Block5dArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> Block5dSnapshot {
        Block5dSnapshot {
            schema: self.schema.clone(),
            part_kind: self.part_kind.clone(),
            part_2d: self.part_2d.clone(),
            part_3d: self.part_3d.clone(),
            representations: self.representations.clone(),
            grip_kinds: self.grip_kinds.clone(),
            grips: self.grips.clone(),
            compatibility: self.compatibility.clone(),
            attributes: self.attributes.clone(),
            authors: self.authors.clone(),
            camera2d: self.camera2d.clone(),
            camera3d: self.camera3d.clone(),
            meta: self.meta.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: Block5dSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            part_kind: snapshot.part_kind,
            part_2d: snapshot.part_2d,
            part_3d: snapshot.part_3d,
            representations: snapshot.representations,
            grip_kinds: snapshot.grip_kinds,
            grips: snapshot.grips,
            compatibility: snapshot.compatibility,
            attributes: snapshot.attributes,
            authors: snapshot.authors,
            camera2d: snapshot.camera2d,
            camera3d: snapshot.camera3d,
            meta: snapshot.meta,
            selected_ids: Vec::new(),
            locale: "en-US".into(),
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: Block5dSnapshot) {
        self.schema = snapshot.schema;
        self.part_kind = snapshot.part_kind;
        self.part_2d = snapshot.part_2d;
        self.part_3d = snapshot.part_3d;
        self.representations = snapshot.representations;
        self.grip_kinds = snapshot.grip_kinds;
        self.grips = snapshot.grips;
        self.compatibility = snapshot.compatibility;
        self.attributes = snapshot.attributes;
        self.authors = snapshot.authors;
        self.camera2d = snapshot.camera2d;
        self.camera3d = snapshot.camera3d;
        self.meta = snapshot.meta;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.block.block5d` — fifteen handcrafted schema leaves.
pub fn block5d_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.block.block5d",
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
