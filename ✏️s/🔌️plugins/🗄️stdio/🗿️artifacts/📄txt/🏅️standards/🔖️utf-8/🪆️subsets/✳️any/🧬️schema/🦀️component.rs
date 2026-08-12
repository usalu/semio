//! 🧬️ TxtArtifact schema — full artifact state.

use crate::artifacts::txt::schema::snapshot::LineEnding;
use crate::artifacts::txt::TxtSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full `stdio.txt` artifact state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.txt")]
pub struct TxtArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub lines: Vec<String>,
    #[state(persistent)]
    #[serde(default)]
    pub trailing_newline: bool,
    #[state(persistent)]
    #[serde(default)]
    pub line_ending: LineEnding,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for TxtArtifact {
    fn default() -> Self {
        Self::from_snapshot(TxtSnapshot::default())
    }
}

impl TxtArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> TxtSnapshot {
        TxtSnapshot {
            schema: self.schema.clone(),
            lines: self.lines.clone(),
            trailing_newline: self.trailing_newline,
            line_ending: self.line_ending,
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    pub fn from_snapshot(snapshot: TxtSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            lines: snapshot.lines,
            trailing_newline: snapshot.trailing_newline,
            line_ending: snapshot.line_ending,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: TxtSnapshot) {
        self.schema = snapshot.schema;
        self.lines = snapshot.lines;
        self.trailing_newline = snapshot.trailing_newline;
        self.line_ending = snapshot.line_ending;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.stdio.txt`.
pub fn txt_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.txt",
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
    use crate::artifacts::txt::{TxtDiff, TxtMutation, TxtSnapshot};

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.txt` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct TxtBuilderConstruction {
        snapshot: TxtSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for TxtBuilderConstruction {
        type Snapshot = TxtSnapshot;
        type Mutation = TxtMutation;
        type Diff = TxtDiff;
        fn empty() -> Self {
            Self { snapshot: TxtSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<TxtSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<TxtSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = crate::artifacts::txt::schema::mutations::apply_txt_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <TxtDiff as protocol::MutationDiff<TxtSnapshot>>::apply(&diff, &self.snapshot);
            self
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
        }
    }
    //#endregion 🔖️Builder
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use semio_framework_plugin::{ArtifactAnalysis, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
    use crate::artifacts::txt::TxtSnapshot;

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.txt` parts.
    #[derive(Clone, Debug, Default)]
    pub struct TxtParts {
        pub snapshot: Option<TxtSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.txt` (utf-8/✳️any) sources.
    pub struct TxtAnalyzerAnalysis;

    /// 🔍 `stdio.txt` accepts anything that is real, valid UTF-8 — a `Text` source is
    /// trivially valid by construction (`High`); a `Binary` source is inspected for actual
    /// UTF-8 validity and the presence of NUL bytes (the standard "probably not text"
    /// signal binary sniffers use).
    fn classify_bytes(bytes: &[u8]) -> IoConfidence {
        match std::str::from_utf8(bytes) {
            Ok(_) if !bytes.contains(&0) => IoConfidence::High,
            Ok(_) => IoConfidence::Medium,
            Err(_) => IoConfidence::Low,
        }
    }

    impl ArtifactAnalysis for TxtAnalyzerAnalysis {
        type Parts = TxtParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Text(_) => IoConfidence::High,
                AnalyzeSource::Binary(bytes) => match store::semio_format::unwrap_binary(bytes) {
                    Ok((_, inner)) => classify_bytes(&inner),
                    Err(_) => classify_bytes(bytes),
                },
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = TxtParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <TxtSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error(
                                "stdio.analyze.text",
                                dsl::TextSpan::at(1, 1),
                                err.to_string(),
                            ));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <TxtSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error(
                                "stdio.analyze.binary",
                                dsl::TextSpan::at(1, 1),
                                err.to_string(),
                            ));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
    //#endregion 🔖️Analyzer

    //#region 🧪️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn sniff_text_source_is_high() {
            assert_eq!(TxtAnalyzerAnalysis::sniff(&AnalyzeSource::Text("anything at all")), IoConfidence::High);
        }

        #[test]
        fn sniff_binary_with_nul_bytes_is_low_or_medium_not_high() {
            let bytes: &[u8] = b"\x00\x01\x02binary garbage\x00";
            assert_ne!(TxtAnalyzerAnalysis::sniff(&AnalyzeSource::Binary(bytes)), IoConfidence::High);
        }

        #[test]
        fn sniff_invalid_utf8_binary_is_low() {
            let bytes: &[u8] = &[0xff, 0xfe, 0xfd];
            assert_eq!(TxtAnalyzerAnalysis::sniff(&AnalyzeSource::Binary(bytes)), IoConfidence::Low);
        }
    }
    //#endregion 🧪️Tests
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec TxtBuilderFacets {
        construction: derived_construction::TxtBuilderConstruction,
        analysis: derived_analysis::TxtAnalyzerAnalysis,
        composition: super::super::io::derived_composition::TxtComposerComposition,
    }
    builder: TxtBuilder,
    analyzer: TxtAnalyzer,
    composer: TxtComposer,
);
//#endregion 🧬️DerivedArtifactFacets
