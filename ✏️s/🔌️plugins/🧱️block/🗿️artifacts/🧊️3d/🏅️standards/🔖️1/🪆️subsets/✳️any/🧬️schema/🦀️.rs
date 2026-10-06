//! 🧬️ Block3d artifact schema — every field with its state class.


use crate::{Block3dSnapshot, Block3dVortexKindExtra};
use ::semio_framework_schema::ArtifactSchema;
use semio_s_artifact_stdio_semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

#[path = "♻️retirement/🦀️.rs"]
pub mod retirement;

//#region 🔖️Artifact
/// 🧬️ block3d document artifact state.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.block.block3d")]
pub struct Block3dArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub object_kind: BlockKindIdentity,
    #[state(artifact)]
    pub representations: Vec<BlockRepresentation>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub catalog: store::ArtifactChild<SemioKitSnapshot>,
    #[state(artifact)]
    pub vortex_kind_extra: Vec<Block3dVortexKindExtra>,
    #[state(artifact)]
    pub vortices: Vec<Block3dVortexTemplate>,
    #[state(artifact)]
    pub compatibility: Vec<BlockCompatibilityRule>,
    #[state(artifact)]
    pub attributes: Vec<BlockAttribute>,
    #[state(artifact)]
    pub authors: Vec<BlockAuthor>,
    #[state(artifact)]
    pub camera3d: BlockCamera3d,
    #[state(artifact)]
    pub meta: BlockMeta,
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
            catalog: self.catalog.clone(),
            vortex_kind_extra: self.vortex_kind_extra.clone(),
            vortices: self.vortices.clone(),
            compatibility: self.compatibility.clone(),
            attributes: self.attributes.clone(),
            authors: self.authors.clone(),
            camera3d: self.camera3d.clone(),
            meta: self.meta.clone(),
        }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: Block3dSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            object_kind: snapshot.object_kind,
            representations: snapshot.representations,
            catalog: snapshot.catalog,
            vortex_kind_extra: snapshot.vortex_kind_extra,
            vortices: snapshot.vortices,
            compatibility: snapshot.compatibility,
            attributes: snapshot.attributes,
            authors: snapshot.authors,
            camera3d: snapshot.camera3d,
            meta: snapshot.meta,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: Block3dSnapshot) {
        self.schema = snapshot.schema;
        self.object_kind = snapshot.object_kind;
        self.representations = snapshot.representations;
        self.catalog = snapshot.catalog;
        self.vortex_kind_extra = snapshot.vortex_kind_extra;
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
/// 🧬️ Descriptor for `s.block.block3d` — twenty handcrafted schema leaves.
pub fn block3d_artifact_schema_descriptor() -> ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
    ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.block.block3d",
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
/// 📸️ A fresh, empty `Block3dSnapshot` (all fields at their `Default`).
pub fn empty_block3d_snapshot() -> Block3dSnapshot {
    Block3dSnapshot::default()
}

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
pub use crate::BlockRepresentation;
pub use crate::Block3dVortexTemplate;
pub use crate::BlockCompatibilityRule;
pub use crate::BlockAttribute;
pub use crate::BlockAuthor;
pub use crate::BlockCamera3d;
pub use crate::BlockMeta;
pub use crate::Block3dWindowView;
//#endregion 🔁️Re-exports
