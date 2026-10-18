//! 🧬️ Block5d artifact schema — every field with its state class.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_block_5d_model::{Block5dSnapshot, Block5dPart2d, Block5dPart3d, Block5dGripKind, Block5dGripTemplate};

use ::semio_framework_schema::ArtifactSchema;

#[path = "♻️retirement/🦀️.rs"]
pub mod retirement;

//#region 🔖️Artifact
/// 🧬️ block5d document artifact state.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[artifact_schema(id = "s.block.block5d")]
pub struct Block5dArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub part_kind: BlockKindIdentity,
    #[state(artifact)]
    pub part_2d: Block5dPart2d,
    #[state(artifact)]
    pub part_3d: Block5dPart3d,
    #[state(artifact)]
    pub representations: Vec<BlockRepresentation>,
    #[state(artifact)]
    pub grip_kinds: Vec<Block5dGripKind>,
    #[state(artifact)]
    pub grips: Vec<Block5dGripTemplate>,
    #[state(artifact)]
    pub compatibility: Vec<BlockCompatibilityRule>,
    #[state(artifact)]
    pub attributes: Vec<BlockAttribute>,
    #[state(artifact)]
    pub authors: Vec<BlockAuthor>,
    #[state(artifact)]
    pub camera2d: BlockCamera2d,
    #[state(artifact)]
    pub camera3d: BlockCamera3d,
    #[state(artifact)]
    pub meta: BlockMeta,
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

    /// 🧬️ Builds the document artifact from its snapshot.
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
/// 🧬️ Descriptor for `s.block.block5d` — twenty handcrafted schema leaves.
pub fn block5d_artifact_schema_descriptor() -> ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
    ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.block.block5d",
        artifact: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
        snapshot: semio_s_artifact_block_5d_model::SNAPSHOT_LEAVES,
        diff: semio_s_artifact_block_5d_diff::DIFF_LEAVES,
        mutations: semio_s_artifact_block_5d_mutations::MUTATION_LEAVES,
    }
}
//#endregion 🔖️Descriptor

//#region 🔖️DocumentHelpers

/// 🪪️ Finds the smallest `"{prefix}{n}"` id not already present in `existing`.
pub fn next_id<'a>(existing: impl Iterator<Item = &'a str>, prefix: &str) -> String {
    let ids: std::collections::HashSet<&str> = existing.collect();
    let mut i = ids.len();
    loop {
        let candidate = format!("{prefix}{i}");
        if !ids.iter().any(|id| *id == candidate) {
            return candidate;
        }
        i += 1;
    }
}
//#endregion 🔖️DocumentHelpers

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use semio_s_plugin_block::BlockKindIdentity;

pub use semio_s_plugin_block::BlockRepresentation;

pub use semio_s_plugin_block::BlockCompatibilityRule;
pub use semio_s_plugin_block::BlockAttribute;
pub use semio_s_plugin_block::BlockAuthor;
pub use semio_s_plugin_block::BlockCamera2d;
pub use semio_s_plugin_block::BlockCamera3d;
pub use semio_s_plugin_block::BlockMeta;
//#endregion 🔁️Re-exports
