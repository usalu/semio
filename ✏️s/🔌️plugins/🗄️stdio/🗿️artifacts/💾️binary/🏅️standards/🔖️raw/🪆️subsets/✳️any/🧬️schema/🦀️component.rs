//! 🧬️ BinaryArtifact schema — full artifact state.

use crate::artifacts::binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full `stdio.binary` artifact state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.binary")]
pub struct BinaryArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    pub bytes: Vec<u8>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for BinaryArtifact {
    fn default() -> Self {
        Self::from_snapshot(BinarySnapshot::default())
    }
}

impl BinaryArtifact {
    /// 📸️ Persisted subset.
    pub async fn to_snapshot(&self) -> BinarySnapshot {
        BinarySnapshot { schema: self.schema.clone(), bytes: self.bytes.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    pub async fn from_snapshot(snapshot: BinarySnapshot) -> Self {
        Self { schema: snapshot.schema, bytes: snapshot.bytes }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub async fn set_snapshot(&mut self, snapshot: BinarySnapshot) {
        self.schema = snapshot.schema;
        self.bytes = snapshot.bytes;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️DocumentHelpers
/// 🦑 Dissolved out of the former `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-
/// MACHINES) — mirrors `png`'s own `empty_png_snapshot`/`demo_png_snapshot` placement beside the
/// artifact struct (binary has no format codec of its own to sit beside — the hex `ArtifactDsl`/
/// `ArtifactPack` impls already live in `📸️snapshot/🦀️component.rs`, untouched by this move).
/// 🌱 Empty persisted snapshot.
pub async fn empty_binary_snapshot() -> BinarySnapshot {
    BinarySnapshot::default()
}

/// 📄️ The demo `stdio.binary` document -- `bytes = b"hello"`, matching the companion real-format
/// fixture asset (`📚️examples/🎬️demo/🖼️assets/🎒️example.bin`, which is literally the raw bytes
/// `hello`). The single source of truth for `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`/
/// `🎒️example.pack.semio` (both are literally this snapshot's `print_dsl`/`encode_pack` output,
/// asserted equal by `fixture_honesty_law` in `💡️inferences/🦀️component.rs`).
pub async fn demo_binary_snapshot() -> BinarySnapshot {
    BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes: b"hello".to_vec() }
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.stdio.binary`.
pub async fn binary_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.binary",
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
    use crate::artifacts::binary::{BinaryDiff, BinaryMutation, BinarySnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.binary` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct BinaryBuilderConstruction {
        snapshot: BinarySnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for BinaryBuilderConstruction {
        type Snapshot = BinarySnapshot;
        type Mutation = BinaryMutation;
        type Diff = BinaryDiff;
        async fn empty() -> Self {
            Self { snapshot: BinarySnapshot::default(), diagnostics: Vec::new() }
        }
        async fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        async fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<BinarySnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        async fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<BinarySnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        async fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::artifacts::binary::schema::mutations::apply_binary_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        async fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <BinaryDiff as protocol::MutationDiff<BinarySnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }
        async fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
    //#endregion 🔖️Builder
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::binary::BinarySnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.binary` parts.
    #[derive(Clone, Debug, Default)]
    pub struct BinaryParts {
        pub snapshot: Option<BinarySnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.binary` (raw/✳️any) sources.
    pub struct BinaryAnalyzerAnalysis;

    impl ArtifactAnalysis for BinaryAnalyzerAnalysis {
        type Parts = BinaryParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };

        async fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            // 👃️ Any byte sequence is a valid stdio.binary payload -- terminal format, always High.
            IoConfidence::High
        }

        async fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = BinaryParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <BinarySnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <BinarySnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    //#endregion 🔖️Analyzer
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec BinaryBuilderFacets {
        construction: BinaryBuilderConstruction,
        analysis: BinaryAnalyzerAnalysis,
        composition: super::super::io::derived_composition::BinaryComposerComposition,
    }
    builder: BinaryBuilder,
    analyzer: BinaryAnalyzer,
    composer: BinaryComposer,
);
//#endregion 🧬️DerivedArtifactFacets
