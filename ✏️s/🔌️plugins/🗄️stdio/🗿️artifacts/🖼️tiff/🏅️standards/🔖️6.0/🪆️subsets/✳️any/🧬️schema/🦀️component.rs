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
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub byte_order: TiffByteOrder,
    #[state(artifact)]
    #[serde(default)]
    pub ifds: Vec<TiffIfd>,
    #[state(artifact)]
    #[serde(default)]
    pub pixels: Vec<u8>,
}

impl Default for TiffArtifact {
    fn default() -> Self {
        Self::from_snapshot(TiffSnapshot::default())
    }
}

impl TiffArtifact {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> TiffSnapshot {
        TiffSnapshot { schema: self.schema.clone(), byte_order: self.byte_order, ifds: self.ifds.clone(), pixels: self.pixels.clone() }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: TiffSnapshot) -> Self {
        Self { schema: snapshot.schema, byte_order: snapshot.byte_order, ifds: snapshot.ifds, pixels: snapshot.pixels }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: TiffSnapshot) {
        *self = Self::from_snapshot(snapshot);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
    use crate::artifacts::tiff::{TiffDiff, TiffMutation, TiffSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

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
        async fn empty() -> Self {
            Self { snapshot: TiffSnapshot::default(), diagnostics: Vec::new() }
        }
        async fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        async fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<TiffSnapshot as store::ArtifactDsl>::parse_dsl(text)?).await)
        }
        async fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<TiffSnapshot as store::ArtifactPack>::decode_pack(bytes)?).await)
        }
        async fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::artifacts::tiff::schema::mutations::apply_tiff_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        async fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <TiffDiff as protocol::MutationDiff<TiffSnapshot>>::apply(&diff, &self.snapshot)?;
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
    use crate::artifacts::tiff::TiffSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

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

        async fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
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
                    if decoded == SIG_LE || decoded == SIG_BE {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
            }
        }

        async fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = TiffParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <TiffSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <TiffSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec TiffBuilderFacets {
        construction: TiffBuilderConstruction,
        analysis: TiffAnalyzerAnalysis,
        composition: super::super::io::derived_composition::TiffComposerComposition,
    }
    builder: TiffBuilder,
    analyzer: TiffAnalyzer,
    composer: TiffComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers
// 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
// `empty_tiff_snapshot`/`demo_tiff_snapshot` relocated here verbatim (pure helpers over the
// document type, destination rule 5); `TiffEngine` (zero construction sites) and the dead
// `register`/`register_pilot_languages`/`register_artifact_inferences` cluster (superseded by
// `declaration()` in the artifact root, zero real callers) deleted outright; the real codec
// (`encode_tiff`/`encode_tiff_packbits`/`decode_tiff` + every pure format algorithm) and
// `io_registry` moved to `../🚪️io`; tests moved beside what they now test.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_tiff_snapshot() -> TiffSnapshot {
    TiffSnapshot::default()
}

/// 📄️ P2-FG2: the demo `stdio.tiff` document — a genuinely non-trivial `TiffSnapshot` exercising
/// a non-solid checkerboard raster plus one carried non-core tag (`Artist`, 315). The single
/// source of truth for `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` (`fixture_honesty_law`
/// in `../🚪️io`'s own tests asserts they're literally this snapshot's `print_dsl` output).
///
/// **Deliberately built via a real `encode_tiff`/`decode_tiff` round trip**, not hand-assembled
/// field values: `encode_tiff` always CANONICALIZES the core strip/geometry tags fresh from
/// `pixels` (see `encode_tiff_with`'s own `EncodeScopeNote`) — hand-picking `ImageWidth`/
/// `BitsPerSample`/`Compression`/`PhotometricInterpretation`/`SamplesPerPixel`/`RowsPerStrip`/
/// `StripByteCounts`/`StripOffsets` values here would silently "self-correct" on the very first
/// `print_dsl`/`parse_dsl` round trip and break `fixture_honesty_law`'s `parse_dsl(fixture) ==
/// demo()` identity (same class of trap `png`'s own `demo_png_snapshot()` doc comment documents
/// for its IHDR fields) — running the real codec once here guarantees `demo()` is ALREADY in
/// exactly the canonical shape a second `encode_tiff`/`decode_tiff` pass reproduces byte-for-byte.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_tiff_snapshot() -> TiffSnapshot {
    use crate::artifacts::tiff::standards::v6_0::subsets::any::io::{decode_tiff, encode_tiff};
    use crate::artifacts::tiff::standards::v6_0::subsets::any::schema::snapshot::{TiffByteOrder, TiffFieldType, TiffIfd, TiffTag, TiffValues};
    use crate::artifacts::tiff::standards::v6_0::subsets::any::schema::snapshot::{TAG_IMAGE_LENGTH, TAG_IMAGE_WIDTH};
    use crate::artifacts::tiff::TiffSnapshot;
    use crate::artifacts::tiff::STDIO_TIFF_DOCUMENT_SCHEMA;
    let (w, h) = (3u32, 2u32);
    let mut pixels = Vec::with_capacity((w * h * 4) as usize);
    for y in 0..h {
        for x in 0..w {
            let checker = if (x + y) % 2 == 0 { 255u8 } else { 0u8 };
            pixels.extend_from_slice(&[checker, ((x * 37) % 256) as u8, ((y * 53) % 256) as u8, 255]);
        }
    }
    let seed = TiffSnapshot {
        schema: STDIO_TIFF_DOCUMENT_SCHEMA.into(),
        byte_order: TiffByteOrder::LittleEndian,
        ifds: vec![TiffIfd {
            entries: vec![
                TiffTag { tag: TAG_IMAGE_WIDTH, kind: TiffFieldType::Long, values: TiffValues::Long(vec![w]) },
                TiffTag { tag: TAG_IMAGE_LENGTH, kind: TiffFieldType::Long, values: TiffValues::Long(vec![h]) },
                TiffTag { tag: 315, kind: TiffFieldType::Ascii, values: TiffValues::Ascii("stdio.tiff demo".into()) },
            ],
        }],
        pixels,
    };
    let encoded = encode_tiff(&seed).expect("demo_tiff_snapshot: encode must succeed");
    decode_tiff(&encoded).expect("demo_tiff_snapshot: decode must succeed")
}
//#endregion 🔖️DocumentHelpers
