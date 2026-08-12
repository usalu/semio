//! 🧬️ SemioMeshArtifact schema — full artifact state, mirrors `SemioMeshSnapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows).

use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::{SemioMeshSnapshot, SemioMesh, SemioMaterial, SemioTexture};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.mesh")]
pub struct SemioMeshArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub meshes: Vec<SemioMesh>,
    #[state(persistent)]
    #[serde(default)]
    pub materials: Vec<SemioMaterial>,
    #[state(persistent)]
    #[serde(default)]
    pub textures: Vec<SemioTexture>,
}

impl Default for SemioMeshArtifact {
    fn default() -> Self { Self::from_snapshot(SemioMeshSnapshot::default()) }
}

impl SemioMeshArtifact {
    pub fn to_snapshot(&self) -> SemioMeshSnapshot {
        SemioMeshSnapshot {
            schema: self.schema.clone(),
            meshes: self.meshes.clone(),
            materials: self.materials.clone(),
            textures: self.textures.clone(),
        }
    }
    pub fn from_snapshot(snapshot: SemioMeshSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            meshes: snapshot.meshes,
            materials: snapshot.materials,
            textures: snapshot.textures,
        }
    }
    pub fn set_snapshot(&mut self, snapshot: SemioMeshSnapshot) {
        self.schema = snapshot.schema;
        self.meshes = snapshot.meshes;
        self.materials = snapshot.materials;
        self.textures = snapshot.textures;
    }
}

pub fn semio_mesh_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.semio.mesh",
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
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use semio_framework_plugin::ArtifactBuilder;
    use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
    use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
    use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct SemioMeshBuilderConstruction { snapshot: SemioMeshSnapshot }

    impl ArtifactBuilder for SemioMeshBuilderConstruction {
        type Snapshot = SemioMeshSnapshot;
        type Mutation = SemioMeshMutation;
        type Diff = SemioMeshDiff;
        fn empty() -> Self { Self { snapshot: SemioMeshSnapshot::default() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<SemioMeshSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SemioMeshSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = <Self::Mutation as protocol::Mutation<SemioMeshSnapshot>>::diff(&mutation, &self.snapshot);
            self.snapshot = <Self::Diff as protocol::MutationDiff<SemioMeshSnapshot>>::apply(&diff, &self.snapshot);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <SemioMeshDiff as protocol::MutationDiff<SemioMeshSnapshot>>::apply(&diff, &self.snapshot);
            self
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { Ok(self.snapshot) }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use semio_framework_plugin::{ArtifactAnalysis, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
    use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::{SemioMeshSnapshot, STDIO_SEMIOMESH_DOCUMENT_SCHEMA};

    #[derive(Clone, Debug, Default)]
    pub struct SemioMeshParts { pub snapshot: Option<SemioMeshSnapshot> }

    pub struct SemioMeshAnalyzerAnalysis;

    impl ArtifactAnalysis for SemioMeshAnalyzerAnalysis {
        type Parts = SemioMeshParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("mesh") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    let marker = STDIO_SEMIOMESH_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) { IoConfidence::High } else { IoConfidence::Low }
                }
                AnalyzeSource::Text(text) => {
                    if text.contains(STDIO_SEMIOMESH_DOCUMENT_SCHEMA) { IoConfidence::High } else { IoConfidence::Low }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = SemioMeshParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <SemioMeshSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <SemioMeshSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.binary", dsl::TextSpan::at(1, 1), err.to_string()));
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
    pub spec SemioMeshBuilderFacets {
        construction: derived_construction::SemioMeshBuilderConstruction,
        analysis: derived_analysis::SemioMeshAnalyzerAnalysis,
        composition: super::super::io::derived_composition::SemioMeshComposerComposition,
    }
    builder: SemioMeshBuilder,
    analyzer: SemioMeshAnalyzer,
    composer: SemioMeshComposer,
);
//#endregion 🧬️DerivedArtifactFacets
