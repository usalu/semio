//! 🧬️ SemioBrepArtifact schema — full artifact state, mirrors `SemioBrepSnapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows). 🚧 scaffolded by W1b.

use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{
    BrepEdge, BrepFace, BrepLoop, BrepShell, BrepSolid, BrepVertex, SemioBrepSnapshot,
};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.brep")]
pub struct SemioBrepArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub vertices: Vec<BrepVertex>,
    #[state(persistent)]
    #[serde(default)]
    pub edges: Vec<BrepEdge>,
    #[state(persistent)]
    #[serde(default)]
    pub loops: Vec<BrepLoop>,
    #[state(persistent)]
    #[serde(default)]
    pub faces: Vec<BrepFace>,
    #[state(persistent)]
    #[serde(default)]
    pub shells: Vec<BrepShell>,
    #[state(persistent)]
    #[serde(default)]
    pub solids: Vec<BrepSolid>,
}

impl Default for SemioBrepArtifact {
    fn default() -> Self { Self::from_snapshot(SemioBrepSnapshot::default()) }
}

impl SemioBrepArtifact {
    pub fn to_snapshot(&self) -> SemioBrepSnapshot {
        SemioBrepSnapshot {
            schema: self.schema.clone(),
            vertices: self.vertices.clone(),
            edges: self.edges.clone(),
            loops: self.loops.clone(),
            faces: self.faces.clone(),
            shells: self.shells.clone(),
            solids: self.solids.clone(),
        }
    }
    pub fn from_snapshot(snapshot: SemioBrepSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            vertices: snapshot.vertices,
            edges: snapshot.edges,
            loops: snapshot.loops,
            faces: snapshot.faces,
            shells: snapshot.shells,
            solids: snapshot.solids,
        }
    }
    pub fn set_snapshot(&mut self, snapshot: SemioBrepSnapshot) {
        *self = Self::from_snapshot(snapshot);
    }
}

pub fn semio_brep_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.semio.brep",
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
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::SemioBrepDiff;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::{SemioBrepMutation, apply_semio_brep_mutation};
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct SemioBrepBuilderConstruction { snapshot: SemioBrepSnapshot }

    impl ArtifactBuilder for SemioBrepBuilderConstruction {
        type Snapshot = SemioBrepSnapshot;
        type Mutation = SemioBrepMutation;
        type Diff = SemioBrepDiff;
        fn empty() -> Self { Self { snapshot: SemioBrepSnapshot::default() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<SemioBrepSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SemioBrepSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = apply_semio_brep_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <SemioBrepDiff as protocol::MutationDiff<SemioBrepSnapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{SemioBrepSnapshot, STDIO_SEMIOBREP_DOCUMENT_SCHEMA};

    #[derive(Clone, Debug, Default)]
    pub struct SemioBrepParts { pub snapshot: Option<SemioBrepSnapshot> }

    pub struct SemioBrepAnalyzerAnalysis;

    impl ArtifactAnalysis for SemioBrepAnalyzerAnalysis {
        type Parts = SemioBrepParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("brep") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    let marker = STDIO_SEMIOBREP_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) { IoConfidence::High } else { IoConfidence::Low }
                }
                AnalyzeSource::Text(text) => {
                    if text.contains(STDIO_SEMIOBREP_DOCUMENT_SCHEMA) { IoConfidence::High } else { IoConfidence::Low }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = SemioBrepParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <SemioBrepSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <SemioBrepSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec SemioBrepBuilderFacets {
        construction: derived_construction::SemioBrepBuilderConstruction,
        analysis: derived_analysis::SemioBrepAnalyzerAnalysis,
        composition: super::super::io::derived_composition::SemioBrepComposerComposition,
    }
    builder: SemioBrepBuilder,
    analyzer: SemioBrepAnalyzer,
    composer: SemioBrepComposer,
);
//#endregion 🧬️DerivedArtifactFacets
