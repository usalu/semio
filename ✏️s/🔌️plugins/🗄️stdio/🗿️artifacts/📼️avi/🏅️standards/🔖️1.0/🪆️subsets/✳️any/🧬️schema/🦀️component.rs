//! 🧬️ AviArtifact schema — full artifact state, mirrors `AviSnapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows). 🚧 scaffolded by W1b.

use crate::artifacts::avi::standards::v1_0::subsets::any::schema::snapshot::{AviMainHeader, AviSnapshot, AviStream, RiffChunk};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.avi")]
pub struct AviArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub main_header: AviMainHeader,
    #[state(persistent)]
    #[serde(default)]
    pub streams: Vec<AviStream>,
    #[state(persistent)]
    pub idx1_present: bool,
    #[state(persistent)]
    #[serde(default)]
    pub unknown_chunks: Vec<RiffChunk>,
}

impl AviArtifact {
    pub fn to_snapshot(&self) -> AviSnapshot {
        AviSnapshot { schema: self.schema.clone(), main_header: self.main_header.clone(), streams: self.streams.clone(), idx1_present: self.idx1_present, unknown_chunks: self.unknown_chunks.clone() }
    }
    pub fn from_snapshot(snapshot: AviSnapshot) -> Self {
        Self { schema: snapshot.schema, main_header: snapshot.main_header, streams: snapshot.streams, idx1_present: snapshot.idx1_present, unknown_chunks: snapshot.unknown_chunks }
    }
    pub fn set_snapshot(&mut self, snapshot: AviSnapshot) {
        self.schema = snapshot.schema;
        self.main_header = snapshot.main_header;
        self.streams = snapshot.streams;
        self.idx1_present = snapshot.idx1_present;
        self.unknown_chunks = snapshot.unknown_chunks;
    }
}

pub fn avi_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.avi",
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
    use crate::artifacts::avi::standards::v1_0::subsets::any::schema::diff::AviDiff;
    use crate::artifacts::avi::standards::v1_0::subsets::any::schema::mutations::{AviMutation, apply_avi_mutation};
    use crate::artifacts::avi::standards::v1_0::subsets::any::schema::snapshot::AviSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct AviBuilderConstruction { snapshot: AviSnapshot }

    impl ArtifactBuilder for AviBuilderConstruction {
        type Snapshot = AviSnapshot;
        type Mutation = AviMutation;
        type Diff = AviDiff;
        fn empty() -> Self { Self { snapshot: AviSnapshot::default() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<AviSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<AviSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = apply_avi_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <AviDiff as protocol::MutationDiff<AviSnapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::avi::standards::v1_0::subsets::any::schema::snapshot::{AviSnapshot, STDIO_AVI_DOCUMENT_SCHEMA};
    use crate::artifacts::avi::standards::v1_0::subsets::any::io as io;

    #[derive(Clone, Debug, Default)]
    pub struct AviParts { pub snapshot: Option<AviSnapshot> }

    pub struct AviAnalyzerAnalysis;

    impl ArtifactAnalysis for AviAnalyzerAnalysis {
        type Parts = AviParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.avi", standard: StandardId("1.0"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    if io::sniff_real_bytes(bytes) {
                        return IoConfidence::High;
                    }
                    let marker = STDIO_AVI_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) { IoConfidence::High } else { IoConfidence::Low }
                }
                AnalyzeSource::Text(text) => {
                    if io::sniff_real_bytes(text.as_bytes()) || text.contains(STDIO_AVI_DOCUMENT_SCHEMA) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = AviParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <AviSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <AviSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec AviBuilderFacets {
        construction: derived_construction::AviBuilderConstruction,
        analysis: derived_analysis::AviAnalyzerAnalysis,
        composition: super::super::io::derived_composition::AviComposerComposition,
    }
    builder: AviBuilder,
    analyzer: AviAnalyzer,
    composer: AviComposer,
);
//#endregion 🧬️DerivedArtifactFacets
