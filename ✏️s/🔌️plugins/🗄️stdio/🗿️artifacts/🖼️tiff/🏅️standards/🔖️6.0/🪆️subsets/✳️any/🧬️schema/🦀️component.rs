//! 🧬️ TiffArtifact schema — full artifact state (mirrors `TiffSnapshot` field-for-field; see
//! `png_artifact_schema_descriptor`/`PngArtifact` for the established repo pattern this follows).

use crate::artifacts::tiff::schema::snapshot::{TiffByteOrder, TiffIfd};
use crate::artifacts::tiff::TiffSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.tiff")]
pub struct TiffArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub byte_order: TiffByteOrder,
    #[state(persistent)]
    #[serde(default)]
    pub ifds: Vec<TiffIfd>,
    #[state(persistent)]
    #[serde(default)]
    pub pixels: Vec<u8>,
}

impl Default for TiffArtifact {
    fn default() -> Self { Self::from_snapshot(TiffSnapshot::default()) }
}

impl TiffArtifact {
    pub fn to_snapshot(&self) -> TiffSnapshot {
        TiffSnapshot { schema: self.schema.clone(), byte_order: self.byte_order, ifds: self.ifds.clone(), pixels: self.pixels.clone() }
    }
    pub fn from_snapshot(snapshot: TiffSnapshot) -> Self {
        Self { schema: snapshot.schema, byte_order: snapshot.byte_order, ifds: snapshot.ifds, pixels: snapshot.pixels }
    }
    pub fn set_snapshot(&mut self, snapshot: TiffSnapshot) {
        *self = Self::from_snapshot(snapshot);
    }
}

pub fn tiff_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.tiff",
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
    use crate::artifacts::tiff::{TiffDiff, TiffMutation, TiffSnapshot};

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.tiff` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct TiffBuilderConstruction {
        snapshot: TiffSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for TiffBuilderConstruction {
        type Snapshot = TiffSnapshot;
        type Mutation = TiffMutation;
        type Diff = TiffDiff;
        fn empty() -> Self {
            Self { snapshot: TiffSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<TiffSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<TiffSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = crate::artifacts::tiff::schema::mutations::apply_tiff_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <TiffDiff as protocol::MutationDiff<TiffSnapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::tiff::TiffSnapshot;

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.tiff` parts.
    #[derive(Clone, Debug, Default)]
    pub struct TiffParts {
        pub snapshot: Option<TiffSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.tiff` (6.0/✳️any) sources.
    pub struct TiffAnalyzerAnalysis;

    impl ArtifactAnalysis for TiffAnalyzerAnalysis {
        type Parts = TiffParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.tiff", standard: StandardId("6.0"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            const SIG_LE: [u8; 4] = [0x49, 0x49, 0x2A, 0x00]; // "II*\0" little-endian
            const SIG_BE: [u8; 4] = [0x4D, 0x4D, 0x00, 0x2A]; // "MM\0*" big-endian
            match source {
                AnalyzeSource::Binary(bytes) => {
                    if bytes.len() >= 4 && (bytes[0..4] == SIG_LE || bytes[0..4] == SIG_BE) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    // 🔍 stdio.tiff's text envelope is a hex dump of the raw bytes after the
                    // `semio ...` preamble line — decode the first 4 bytes to sniff the real signature.
                    let body = match store::semio_format::split_text_preamble(text) {
                        Ok((_, rest)) => rest,
                        Err(_) => text,
                    };
                    let hex: String = body.chars().filter(|c| !c.is_whitespace()).take(8).collect();
                    if hex.len() < 8 {
                        return IoConfidence::Low;
                    }
                    let mut decoded = [0u8; 4];
                    for (i, byte) in decoded.iter_mut().enumerate() {
                        match u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16) {
                            Ok(b) => *byte = b,
                            Err(_) => return IoConfidence::Low,
                        }
                    }
                    if decoded == SIG_LE || decoded == SIG_BE { IoConfidence::High } else { IoConfidence::Low }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = TiffParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <TiffSnapshot as store::ArtifactDsl>::parse_dsl(text) {
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
                    AnalyzeSource::Binary(bytes) => match <TiffSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec TiffBuilderFacets {
        construction: derived_construction::TiffBuilderConstruction,
        analysis: derived_analysis::TiffAnalyzerAnalysis,
        composition: super::super::io::derived_composition::TiffComposerComposition,
    }
    builder: TiffBuilder,
    analyzer: TiffAnalyzer,
    composer: TiffComposer,
);
//#endregion 🧬️DerivedArtifactFacets
