//! 🧬️ Block5d artifact schema — every field with its state class.

use crate::{Block5dSnapshot};

use ::semio_framework_schema::ArtifactSchema;

#[path = "♻️retirement/🦀️.rs"]
pub mod retirement;

//#region 🔖️Artifact
/// 🧬️ block5d document artifact state.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, ArtifactSchema)]
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
        snapshot: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

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

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::BlockKindIdentity;
pub use crate::Block5dPart2d;
pub use crate::Block5dPart3d;
pub use crate::BlockRepresentation;
pub use crate::Block5dGripKind;
pub use crate::Block5dGripTemplate;
pub use crate::BlockCompatibilityRule;
pub use crate::BlockAttribute;
pub use crate::BlockAuthor;
pub use crate::BlockCamera2d;
pub use crate::BlockCamera3d;
pub use crate::BlockMeta;
//#endregion 🔁️Re-exports
