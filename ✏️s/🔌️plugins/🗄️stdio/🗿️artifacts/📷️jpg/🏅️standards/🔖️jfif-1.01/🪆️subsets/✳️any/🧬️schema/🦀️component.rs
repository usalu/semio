//! 🧬️ JpgArtifact schema — full artifact state.

use crate::artifacts::jpg::JpgSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

/// 🎪️ Reduced UI-editable view: identity + the raster the user is directly manipulating. Ticket
/// 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION killed the shared
/// `RasterImage` wrapper (jpg/png/tiff each copy-pasted it) — `width`/`height`/`pixels` are
/// first-class fields here, matching `JpgSnapshot`'s own shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.jpg")]
pub struct JpgArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub width: u32,
    #[state(persistent)]
    #[serde(default)]
    pub height: u32,
    #[state(persistent)]
    #[serde(default)]
    pub pixels: Vec<u8>,
}

impl Default for JpgArtifact {
    fn default() -> Self { Self::from_snapshot(JpgSnapshot::default()) }
}

impl JpgArtifact {
    pub fn to_snapshot(&self) -> JpgSnapshot {
        // 🎪️ `JpgArtifact` is the reduced UI-editable view (schema+raster only) — it never
        // carries frame/table data, so `frame`/`sof_marker`/`arithmetic`/`quant_tables`/
        // `huffman_tables`/etc. fall back to `JpgSnapshot::default()`'s "no decoded frame" state.
        JpgSnapshot { schema: self.schema.clone(), width: self.width, height: self.height, pixels: self.pixels.clone(), ..JpgSnapshot::default() }
    }
    pub fn from_snapshot(snapshot: JpgSnapshot) -> Self {
        Self { schema: snapshot.schema, width: snapshot.width, height: snapshot.height, pixels: snapshot.pixels }
    }
    pub fn set_snapshot(&mut self, snapshot: JpgSnapshot) {
        self.schema = snapshot.schema;
        self.width = snapshot.width;
        self.height = snapshot.height;
        self.pixels = snapshot.pixels;
    }
}

pub fn jpg_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.jpg",
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
    use crate::artifacts::jpg::{JpgDiff, JpgMutation, JpgSnapshot};

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.jpg` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct JpgBuilderConstruction {
        snapshot: JpgSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for JpgBuilderConstruction {
        type Snapshot = JpgSnapshot;
        type Mutation = JpgMutation;
        type Diff = JpgDiff;
        fn empty() -> Self {
            Self { snapshot: JpgSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<JpgSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<JpgSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = crate::artifacts::jpg::schema::mutations::apply_jpg_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <JpgDiff as protocol::MutationDiff<JpgSnapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::jpg::JpgSnapshot;

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.jpg` parts.
    #[derive(Clone, Debug, Default)]
    pub struct JpgParts {
        pub snapshot: Option<JpgSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.jpg` (jfif-1.01/✳️any) sources.
    pub struct JpgAnalyzerAnalysis;

    impl ArtifactAnalysis for JpgAnalyzerAnalysis {
        type Parts = JpgParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.jpg", standard: StandardId("jfif-1.01"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            const SIG: [u8; 2] = [0xFF, 0xD8]; // SOI
            match source {
                AnalyzeSource::Binary(bytes) => {
                    if bytes.len() >= 2 && bytes[0..2] == SIG { IoConfidence::High } else { IoConfidence::Low }
                }
                AnalyzeSource::Text(text) => {
                    // 🔍 stdio.jpg's text envelope is a hex dump of the raw bytes after the
                    // `semio ...` preamble line — decode the first 2 bytes to sniff the real SOI marker.
                    let body = match store::semio_format::split_text_preamble(text) {
                        Ok((_, rest)) => rest,
                        Err(_) => text,
                    };
                    let hex: String = body.chars().filter(|c| !c.is_whitespace()).take(4).collect();
                    if hex.len() < 4 {
                        return IoConfidence::Low;
                    }
                    let mut decoded = [0u8; 2];
                    for (i, byte) in decoded.iter_mut().enumerate() {
                        match u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16) {
                            Ok(b) => *byte = b,
                            Err(_) => return IoConfidence::Low,
                        }
                    }
                    if decoded == SIG { IoConfidence::High } else { IoConfidence::Low }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = JpgParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <JpgSnapshot as store::ArtifactDsl>::parse_dsl(text) {
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
                    AnalyzeSource::Binary(bytes) => match <JpgSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec JpgBuilderFacets {
        construction: derived_construction::JpgBuilderConstruction,
        analysis: derived_analysis::JpgAnalyzerAnalysis,
        composition: super::super::io::derived_composition::JpgComposerComposition,
    }
    builder: JpgBuilder,
    analyzer: JpgAnalyzer,
    composer: JpgComposer,
);
//#endregion 🧬️DerivedArtifactFacets
