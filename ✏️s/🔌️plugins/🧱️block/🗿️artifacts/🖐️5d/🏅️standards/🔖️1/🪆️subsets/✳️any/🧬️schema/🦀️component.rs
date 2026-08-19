//! 🧬️ Block5d artifact schema — every field with its state class.

use crate::artifacts::block5d::{Block5dGripKind, Block5dGripTemplate, Block5dPart2d, Block5dPart3d, Block5dSnapshot};
use crate::{BlockAttribute, BlockAuthor, BlockCamera2d, BlockCamera3d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta, BlockRepresentation};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full block5d artifact state across the artifact, presence and config lanes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.block.block5d")]
pub struct Block5dArtifact {
    #[state(artifact)] pub schema: String,
    #[state(artifact)] pub part_kind: BlockKindIdentity,
    #[state(artifact)] pub part_2d: Block5dPart2d,
    #[state(artifact)] pub part_3d: Block5dPart3d,
    #[state(artifact)] pub representations: Vec<BlockRepresentation>,
    #[state(artifact)] pub grip_kinds: Vec<Block5dGripKind>,
    #[state(artifact)] pub grips: Vec<Block5dGripTemplate>,
    #[state(artifact)] pub compatibility: Vec<BlockCompatibilityRule>,
    #[state(artifact)] pub attributes: Vec<BlockAttribute>,
    #[state(artifact)] pub authors: Vec<BlockAuthor>,
    #[state(artifact)] pub camera2d: BlockCamera2d,
    #[state(artifact)] pub camera3d: BlockCamera3d,
    #[state(artifact)] pub meta: BlockMeta,
    #[state(presence)] pub selected_ids: Vec<String>,
    #[state(config)] pub locale: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for Block5dArtifact {
    async fn default() -> Self {
        Self::from_snapshot(Block5dSnapshot::default())
    }
}

impl Block5dArtifact {
    /// 📸️ Persisted subset.
    pub async fn to_snapshot(&self) -> Block5dSnapshot {
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
    pub async fn from_snapshot(snapshot: Block5dSnapshot) -> Self {
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
    pub async fn set_snapshot(&mut self, snapshot: Block5dSnapshot) {
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
pub async fn block5d_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
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
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️component.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use semio_framework_plugin::ArtifactBuilder;
    use crate::artifacts::block5d::{Block5dDiff, Block5dMutation, Block5dSnapshot};

    #[derive(Clone, Debug, Default)]
    pub struct Block5dBuilderConstruction {
        snapshot: Block5dSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for Block5dBuilderConstruction {
        type Snapshot = Block5dSnapshot;
        type Mutation = Block5dMutation;
        type Diff = Block5dDiff;
        async fn empty() -> Self { Self { snapshot: Block5dSnapshot::default(), diagnostics: Vec::new() } }
        async fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        async fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<Block5dSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        async fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Block5dSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        async fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
            match <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error(
                    "mutation.apply",
                    dsl::TextSpan::at(1, 1),
                    error.to_string(),
                )),
            }
            (self, outcome)
        }
        async fn absorb(
            mut self,
            diff: Self::Diff,
        ) -> protocol::MutationApplyResult<Self> {
            let snapshot = <Block5dDiff as protocol::MutationDiff<Block5dSnapshot>>::apply(&diff, &self.snapshot)?;
            self.snapshot = snapshot;
            Ok(self)
        }
        async fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use semio_framework_plugin::{ArtifactAnalysis, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
    use crate::artifacts::block5d::Block5dSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct Block5dParts {
        pub snapshot: Option<Block5dSnapshot>,
    }

    pub struct Block5dAnalyzerAnalysis;

    impl ArtifactAnalysis for Block5dAnalyzerAnalysis {
        type Parts = Block5dParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.block5d", standard: StandardId("1"), subset: SubsetId("*") };

        async fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        async fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Block5dParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <Block5dSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Block5dSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.binary", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec Block5dBuilderFacets {
        construction: Block5dBuilderConstruction,
        analysis: Block5dAnalyzerAnalysis,
        composition: super::super::io::derived_composition::Block5dComposerComposition,
    }
    builder: Block5dBuilder,
    analyzer: Block5dAnalyzer,
    composer: Block5dComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers
/// 📸️ A fresh, empty `Block5dSnapshot` (all fields at their `Default`).
pub async fn empty_block5d_snapshot() -> Block5dSnapshot {
    Block5dSnapshot::default()
}

/// 🪪️ Finds the smallest `"{prefix}{n}"` id not already present in `existing`.
pub async fn next_id<'a>(existing: impl Iterator<Item = &'a str>, prefix: &str) -> String {
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
mod tests {
    use super::*;

    #[test]
    async fn empty_definition_matches_default() {
        assert_eq!(empty_block5d_snapshot(), Block5dSnapshot::default());
    }
}
//#endregion 🧪️Tests
