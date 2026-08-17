//! 🧬️ PngArtifact schema — full artifact state (mirrors `PngSnapshot` field-for-field; see
//! `zip_artifact_schema_descriptor`/`ZipArtifact` for the established repo pattern this follows).

use crate::artifacts::png::schema::snapshot::{PngBackground, PngChromaticities, PngChunk, PngChunkMarker, PngColorType, PngPhysicalDims, PngRgb, PngSrgbIntent, PngTextChunk, PngTextKind, PngTimestamp, PngTransparency};
use crate::artifacts::png::PngSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.png")]
pub struct PngArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub width: u32,
    #[state(artifact)]
    pub height: u32,
    #[state(artifact)]
    pub bit_depth: u8,
    #[state(artifact)]
    pub color_type: PngColorType,
    #[state(artifact)]
    pub interlace: bool,
    #[state(artifact)]
    #[serde(default)]
    pub plte: Option<Vec<PngRgb>>,
    #[state(artifact)]
    #[serde(default)]
    pub trns: Option<PngTransparency>,
    #[state(artifact)]
    #[serde(default)]
    pub gama: Option<u32>,
    #[state(artifact)]
    #[serde(default)]
    pub chrm: Option<PngChromaticities>,
    #[state(artifact)]
    #[serde(default)]
    pub srgb: Option<PngSrgbIntent>,
    #[state(artifact)]
    #[serde(default)]
    pub phys: Option<PngPhysicalDims>,
    #[state(artifact)]
    #[serde(default)]
    pub time: Option<PngTimestamp>,
    #[state(artifact)]
    #[serde(default)]
    pub bkgd: Option<PngBackground>,
    #[state(artifact)]
    #[serde(default)]
    pub text_chunks: Vec<PngTextChunk>,
    #[state(artifact)]
    #[serde(default)]
    pub pixels: Vec<u8>,
    #[state(artifact)]
    #[serde(default)]
    pub chunk_order: Vec<PngChunkMarker>,
    #[state(artifact)]
    #[serde(default)]
    pub unknown_chunks: Vec<PngChunk>,
}

impl Default for PngArtifact {
    fn default() -> Self {
        Self::from_snapshot(PngSnapshot::default())
    }
}

impl PngArtifact {
    pub fn to_snapshot(&self) -> PngSnapshot {
        PngSnapshot {
            schema: self.schema.clone(),
            width: self.width,
            height: self.height,
            bit_depth: self.bit_depth,
            color_type: self.color_type,
            interlace: self.interlace,
            plte: self.plte.clone(),
            trns: self.trns.clone(),
            gama: self.gama,
            chrm: self.chrm,
            srgb: self.srgb,
            phys: self.phys,
            time: self.time,
            bkgd: self.bkgd.clone(),
            text_chunks: self.text_chunks.clone(),
            pixels: self.pixels.clone(),
            chunk_order: self.chunk_order.clone(),
            unknown_chunks: self.unknown_chunks.clone(),
        }
    }
    pub fn from_snapshot(snapshot: PngSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            width: snapshot.width,
            height: snapshot.height,
            bit_depth: snapshot.bit_depth,
            color_type: snapshot.color_type,
            interlace: snapshot.interlace,
            plte: snapshot.plte,
            trns: snapshot.trns,
            gama: snapshot.gama,
            chrm: snapshot.chrm,
            srgb: snapshot.srgb,
            phys: snapshot.phys,
            time: snapshot.time,
            bkgd: snapshot.bkgd,
            text_chunks: snapshot.text_chunks,
            pixels: snapshot.pixels,
            chunk_order: snapshot.chunk_order,
            unknown_chunks: snapshot.unknown_chunks,
        }
    }
    pub fn set_snapshot(&mut self, snapshot: PngSnapshot) {
        *self = Self::from_snapshot(snapshot);
    }
}

//#region 🔖️DemoFixtures
/// 🕳️ Relocated verbatim from `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES, rule 5: pure helpers over document
/// types live in `🧬️schema/`).
pub fn empty_png_snapshot() -> PngSnapshot {
    PngSnapshot::default()
}

/// 📄️ P2-P2: the demo `stdio.png` document — a genuinely non-trivial `PngSnapshot` exercising
/// PLTE, every typed ancillary chunk (gAMA/cHRM/sRGB/pHYs/tIME/bKGD), one text chunk, and one
/// verbatim-retained unknown ancillary chunk, all in a real relative chunk order. The single
/// source of truth for `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`/`🎒️example.pack.semio`
/// (both are literally this snapshot's `print_dsl`/`encode_pack` output, asserted equal by
/// `fixture_honesty_law` in `💡️inferences/🦀️component.rs`).
///
/// **Deliberately safe against `encode_png`'s own canonicalization** (see `encode_png`'s own
/// `🚫️EncodeScopeNote` in `🚪️io/🦀️component.rs`): `bit_depth`/`color_type`/`interlace` are set to
/// EXACTLY what `encode_png` always hardcodes into the real IHDR bytes regardless of the
/// snapshot's own field values (`8`/`Rgba`/`false`) — any OTHER value here would silently
/// "self-correct" on the first decode and break `fixture_honesty_law`'s
/// `parse_dsl(fixture) == demo()` identity. `trns` is deliberately `None` (a `tRNS` chunk decoded
/// under `color_type == 6` is spec-mandated to be IGNORED — `decode_png`'s own `_ => {}` arm — so
/// no non-`None` value here could ever round-trip either); `bkgd` uses the `Rgb` variant
/// specifically (the ONLY variant whose own 6-byte wire shape matches what `color_type == 6`
/// decodes, `2|6 => 6 bytes`).
pub fn demo_png_snapshot() -> PngSnapshot {
    let (w, h) = (3u32, 3u32);
    let mut pixels = Vec::with_capacity((w * h * 4) as usize);
    for y in 0..h {
        for x in 0..w {
            let checker = if (x + y) % 2 == 0 { 255u8 } else { 0u8 };
            pixels.extend_from_slice(&[checker, ((x * 37) % 256) as u8, ((y * 53) % 256) as u8, 255]);
        }
    }
    PngSnapshot {
        schema: crate::artifacts::png::STDIO_PNG_DOCUMENT_SCHEMA.into(),
        width: w,
        height: h,
        bit_depth: 8,
        color_type: PngColorType::Rgba,
        interlace: false,
        plte: Some(vec![PngRgb { r: 255, g: 0, b: 0 }, PngRgb { r: 0, g: 255, b: 0 }, PngRgb { r: 0, g: 0, b: 255 }]),
        trns: None,
        gama: Some(45455),
        chrm: Some(PngChromaticities { white_x: 31270, white_y: 32900, red_x: 64000, red_y: 33000, green_x: 30000, green_y: 60000, blue_x: 15000, blue_y: 6000 }),
        srgb: Some(PngSrgbIntent::Perceptual),
        phys: Some(PngPhysicalDims { ppu_x: 2835, ppu_y: 2835, unit_is_meter: true }),
        time: Some(PngTimestamp { year: 2024, month: 6, day: 15, hour: 12, minute: 30, second: 0 }),
        bkgd: Some(PngBackground::Rgb { r: 255, g: 255, b: 255 }),
        text_chunks: vec![PngTextChunk { keyword: "Title".into(), value: "semio demo".into(), compressed: false, kind: PngTextKind::Text, language_tag: String::new(), translated_keyword: String::new() }],
        pixels,
        chunk_order: vec![
            PngChunkMarker::Ihdr,
            PngChunkMarker::Plte,
            PngChunkMarker::Gama,
            PngChunkMarker::Chrm,
            PngChunkMarker::Srgb,
            PngChunkMarker::Phys,
            PngChunkMarker::Time,
            PngChunkMarker::Bkgd,
            PngChunkMarker::Text { index: 0 },
            PngChunkMarker::Unknown { index: 0 },
            PngChunkMarker::Idat,
            PngChunkMarker::Iend,
        ],
        unknown_chunks: vec![PngChunk { kind: *b"prIV", data: vec![9, 9, 9] }],
    }
}
//#endregion 🔖️DemoFixtures

pub fn png_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.png",
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
    use crate::artifacts::png::{PngDiff, PngMutation, PngSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.png` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct PngBuilderConstruction {
        snapshot: PngSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for PngBuilderConstruction {
        type Snapshot = PngSnapshot;
        type Mutation = PngMutation;
        type Diff = PngDiff;
        fn empty() -> Self {
            Self { snapshot: PngSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<PngSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<PngSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::artifacts::png::schema::mutations::apply_png_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <PngDiff as protocol::MutationDiff<PngSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
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
    use crate::artifacts::png::PngSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.png` parts.
    #[derive(Clone, Debug, Default)]
    pub struct PngParts {
        pub snapshot: Option<PngSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.png` (1.2/✳️any) sources.
    pub struct PngAnalyzerAnalysis;

    impl ArtifactAnalysis for PngAnalyzerAnalysis {
        type Parts = PngParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            const SIG: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
            match source {
                AnalyzeSource::Binary(bytes) => {
                    if bytes.len() >= 8 && bytes[0..8] == SIG {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    // 🔍 stdio.png's text envelope is a hex dump of the raw bytes after the
                    // `semio ...` preamble line — decode the first 8 bytes to sniff the real signature.
                    let body = match store::semio_format::split_text_preamble(text) {
                        Ok((_, rest)) => rest,
                        Err(_) => text,
                    };
                    let hex: String = body.chars().filter(|c| !c.is_whitespace()).take(16).collect();
                    if hex.len() < 16 {
                        return IoConfidence::Low;
                    }
                    let mut decoded = [0u8; 8];
                    for (i, byte) in decoded.iter_mut().enumerate() {
                        match u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16) {
                            Ok(b) => *byte = b,
                            Err(_) => return IoConfidence::Low,
                        }
                    }
                    if decoded == SIG {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = PngParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <PngSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <PngSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec PngBuilderFacets {
        construction: PngBuilderConstruction,
        analysis: PngAnalyzerAnalysis,
        composition: super::super::io::derived_composition::PngComposerComposition,
    }
    builder: PngBuilder,
    analyzer: PngAnalyzer,
    composer: PngComposer,
);
//#endregion 🧬️DerivedArtifactFacets
