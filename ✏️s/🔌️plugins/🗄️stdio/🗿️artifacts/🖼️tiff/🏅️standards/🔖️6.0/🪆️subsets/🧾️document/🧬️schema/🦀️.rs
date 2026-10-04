//! 🧬️ TiffArtifact schema — full artifact state (mirrors `TiffSnapshot` field-for-field; see
//! `png_artifact_schema_descriptor`/`PngArtifact` for the established repo pattern this follows).

use crate::schema::snapshot::{TiffByteOrder, TiffIfd};
use crate::TiffSnapshot;
use framework_schema::ArtifactSchema;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.tiff")]
pub struct TiffArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub byte_order: TiffByteOrder,
    #[state(artifact)]
    #[value(default)]
    pub ifds: Vec<TiffIfd>,
}

impl Default for TiffArtifact {
    fn default() -> Self {
        Self::from_snapshot(TiffSnapshot::default())
    }
}

impl TiffArtifact {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> TiffSnapshot {
        TiffSnapshot { schema: self.schema.clone(), byte_order: self.byte_order, ifds: self.ifds.clone() }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: TiffSnapshot) -> Self {
        Self { schema: snapshot.schema, byte_order: snapshot.byte_order, ifds: snapshot.ifds }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: TiffSnapshot) {
        *self = Self::from_snapshot(snapshot);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn tiff_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.tiff",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::{TiffDiff, TiffMutation, TiffSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.tiff` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct TiffBuilderConstruction {
        snapshot: TiffSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
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
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<TiffSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<TiffSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::schema::mutations::apply_tiff_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <TiffDiff as protocol::MutationDiff<TiffSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
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
    use crate::TiffSnapshot;
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
                    if decoded == SIG_LE || decoded == SIG_BE {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
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
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <TiffSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.binary", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
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
// `blank_tiff_snapshot`/`demo_tiff_snapshot` relocated here verbatim (pure helpers over the
// document type, destination rule 5); `TiffEngine` (zero construction sites) and the dead
// `register`/`register_pilot_languages`/`register_artifact_inferences` cluster (superseded by
// `declaration()` in the artifact root, zero real callers) deleted outright; the real codec
// (`encode_tiff`/`encode_tiff_packbits`/`decode_tiff` + every pure format algorithm) and
// `io_registry` moved to `../🚪️io`; tests moved beside what they now test.
/// 🆕️ A new tiff document: one opaque white pixel in one IFD as the real codec round-trips it — baseline TIFF has no
/// image without `ImageWidth`/`ImageLength` (TIFF 6.0 §8), and a new document must save and reopen as itself.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn blank_tiff_snapshot() -> TiffSnapshot {
    use crate::standards::v6_0::subsets::document::schema::snapshot::*;
    TiffSnapshot { ifds: vec![TiffIfd { entries: vec![
        TiffTag { tag: TAG_IMAGE_WIDTH, values: TiffValues::Long(vec![1]) },TiffTag { tag: TAG_IMAGE_LENGTH, values: TiffValues::Long(vec![1]) },TiffTag { tag: TAG_BITS_PER_SAMPLE, values: TiffValues::Short(vec![8,8,8]) },TiffTag { tag: TAG_COMPRESSION, values: TiffValues::Short(vec![1]) },TiffTag { tag: TAG_PHOTOMETRIC, values: TiffValues::Short(vec![2]) },TiffTag { tag: TAG_SAMPLES_PER_PIXEL, values: TiffValues::Short(vec![3]) },TiffTag { tag: TAG_ROWS_PER_STRIP, values: TiffValues::Long(vec![1]) },
    ], storage: TiffStorage { kind:TiffStorageKind::Strips,offsets_kind:TiffFieldType::Long,byte_counts_kind:TiffFieldType::Long,chunks:vec![vec![255,255,255]] } }], ..TiffSnapshot::default() }
}

/// 📄️ P2-FG2: the demo `stdio.tiff` document — a genuinely non-trivial `TiffSnapshot` exercising
/// a non-solid checkerboard raster plus one carried non-core tag (`Artist`, 315). The single
/// source of truth for `📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio` (`fixture_honesty_law`
/// in `../🚪️io`'s own tests asserts they're literally this snapshot's `print_dsl` output).
///
/// **Deliberately built via a real `encode_tiff`/`decode_tiff` round trip**, not hand-assembled
/// field values: `encode_tiff` always CANONICALIZES the core strip/geometry tags fresh from
/// `pixels` (see `encode_tiff_with`'s own `MultiIfdEncodeScopeNote`) — hand-picking `ImageWidth`/
/// `BitsPerSample`/`Compression`/`PhotometricInterpretation`/`SamplesPerPixel`/`RowsPerStrip`/
/// `StripByteCounts`/`StripOffsets` values here would silently "self-correct" on the very first
/// `print_dsl`/`parse_dsl` round trip and break `fixture_honesty_law`'s `parse_dsl(fixture) ==
/// demo()` identity (same class of trap `png`'s own `demo_png_snapshot()` doc comment documents
/// for its IHDR fields) — running the real codec once here guarantees `demo()` is ALREADY in
/// exactly the canonical shape a second `encode_tiff`/`decode_tiff` pass reproduces byte-for-byte.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_tiff_snapshot() -> TiffSnapshot {
    use crate::standards::v6_0::subsets::document::schema::snapshot::*;
    let (width,height)=(3u32,2u32);let mut rgb=Vec::with_capacity((width*height*3)as usize);for y in 0..height{for x in 0..width{let checker=if(x+y)%2==0{255}else{0};rgb.extend_from_slice(&[checker,((x*37)%256)as u8,((y*53)%256)as u8]);}}
    TiffSnapshot{schema:crate::STDIO_TIFF_DOCUMENT_SCHEMA.into(),byte_order:TiffByteOrder::LittleEndian,ifds:vec![TiffIfd{entries:vec![
        TiffTag{tag:TAG_IMAGE_WIDTH,values:TiffValues::Long(vec![width])},TiffTag{tag:TAG_IMAGE_LENGTH,values:TiffValues::Long(vec![height])},TiffTag{tag:TAG_BITS_PER_SAMPLE,values:TiffValues::Short(vec![8,8,8])},TiffTag{tag:TAG_COMPRESSION,values:TiffValues::Short(vec![1])},TiffTag{tag:TAG_PHOTOMETRIC,values:TiffValues::Short(vec![2])},TiffTag{tag:TAG_SAMPLES_PER_PIXEL,values:TiffValues::Short(vec![3])},TiffTag{tag:TAG_ROWS_PER_STRIP,values:TiffValues::Long(vec![height])},TiffTag{tag:315,values:TiffValues::Ascii(b"stdio.tiff demo\0".to_vec())},
    ],storage:TiffStorage{kind:TiffStorageKind::Strips,offsets_kind:TiffFieldType::Long,byte_counts_kind:TiffFieldType::Long,chunks:vec![rgb]}}]}
}
//#endregion 🔖️DocumentHelpers
