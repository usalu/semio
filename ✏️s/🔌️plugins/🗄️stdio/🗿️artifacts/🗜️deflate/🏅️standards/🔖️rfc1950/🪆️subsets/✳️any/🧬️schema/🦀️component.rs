//! 🧬️ DeflateArtifact schema — full artifact state.

use crate::artifacts::deflate::schema::snapshot::DeflateLevelHint;
use crate::artifacts::deflate::{DeflateSnapshot, STDIO_DEFLATE_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full `stdio.deflate` artifact state — mirrors `DeflateSnapshot`'s typed RFC1950 fields.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.deflate")]
pub struct DeflateArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    pub compression_method: u8,
    #[state(artifact)]
    #[serde(default)]
    pub window_bits: u8,
    #[state(artifact)]
    #[serde(default)]
    pub compression_level_hint: DeflateLevelHint,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dict_id: Option<u32>,
    #[state(artifact)]
    #[serde(default)]
    pub payload: Vec<u8>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for DeflateArtifact {
    fn default() -> Self {
        Self::from_snapshot(DeflateSnapshot::default())
    }
}

impl DeflateArtifact {
    /// 📸️ Persisted subset.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> DeflateSnapshot {
        DeflateSnapshot { schema: self.schema.clone(), compression_method: self.compression_method, window_bits: self.window_bits, compression_level_hint: self.compression_level_hint, dict_id: self.dict_id, payload: self.payload.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: DeflateSnapshot) -> Self {
        Self { schema: snapshot.schema, compression_method: snapshot.compression_method, window_bits: snapshot.window_bits, compression_level_hint: snapshot.compression_level_hint, dict_id: snapshot.dict_id, payload: snapshot.payload }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: DeflateSnapshot) {
        self.schema = snapshot.schema;
        self.compression_method = snapshot.compression_method;
        self.window_bits = snapshot.window_bits;
        self.compression_level_hint = snapshot.compression_level_hint;
        self.dict_id = snapshot.dict_id;
        self.payload = snapshot.payload;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️DocumentHelpers
/// 🦑 Dissolved out of the former `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-
/// MACHINES) — mirrors `png`'s own `empty_png_snapshot`/`demo_png_snapshot` placement beside the
/// artifact struct.
/// 🌱 Empty persisted snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_deflate_snapshot() -> DeflateSnapshot {
    DeflateSnapshot::default()
}

/// 📄️ The demo `stdio.deflate` document — a genuine, non-empty RFC1950 container: a real
/// preset-dictionary id (exercises the FDICT-gated `dict_id` field) plus repetitive text payload
/// (round-trips through this artifact's own `deflate_raw`/`inflate_raw` in `🚪️io/🦀️component.rs`).
/// Single source of truth for `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`/`🗜️example.zz`/
/// `🎒️example.pack.semio` (all three are literally this snapshot's `print_dsl`/
/// `encode_deflate_snapshot`/`encode_pack` output, asserted equal by `fixture_honesty_law` in
/// `💡️inferences/🦀️component.rs`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_deflate_snapshot() -> DeflateSnapshot {
    DeflateSnapshot {
        schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(),
        compression_method: 8,
        window_bits: 7,
        compression_level_hint: DeflateLevelHint::Default,
        dict_id: Some(0x1234_5678),
        payload: b"the quick brown fox jumps over the lazy dog".to_vec(),
    }
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.stdio.deflate`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deflate_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.deflate",
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
    use crate::artifacts::deflate::{DeflateDiff, DeflateMutation, DeflateSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.deflate` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct DeflateBuilderConstruction {
        snapshot: DeflateSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for DeflateBuilderConstruction {
        type Snapshot = DeflateSnapshot;
        type Mutation = DeflateMutation;
        type Diff = DeflateDiff;
        async fn empty() -> Self {
            Self { snapshot: DeflateSnapshot::default(), diagnostics: Vec::new() }
        }
        async fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        async fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<DeflateSnapshot as store::ArtifactDsl>::parse_dsl(text)?).await)
        }
        async fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<DeflateSnapshot as store::ArtifactPack>::decode_pack(bytes)?).await)
        }
        async fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::artifacts::deflate::schema::mutations::apply_deflate_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        async fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <DeflateDiff as protocol::MutationDiff<DeflateSnapshot>>::apply(&diff, &self.snapshot)?;
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
    use crate::artifacts::deflate::DeflateSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.deflate` parts.
    #[derive(Clone, Debug, Default)]
    pub struct DeflateParts {
        pub snapshot: Option<DeflateSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.deflate` (rfc1950/✳️any) sources.
    pub struct DeflateAnalyzerAnalysis;

    impl ArtifactAnalysis for DeflateAnalyzerAnalysis {
        type Parts = DeflateParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.deflate", standard: StandardId("rfc1950"), subset: SubsetId("*") };

        async fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        async fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = DeflateParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <DeflateSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <DeflateSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec DeflateBuilderFacets {
        construction: DeflateBuilderConstruction,
        analysis: DeflateAnalyzerAnalysis,
        composition: super::super::io::derived_composition::DeflateComposerComposition,
    }
    builder: DeflateBuilder,
    analyzer: DeflateAnalyzer,
    composer: DeflateComposer,
);
//#endregion 🧬️DerivedArtifactFacets
