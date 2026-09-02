//! 🧬️ BmpArtifact schema — full artifact state.

use crate::artifacts::bmp::schema::snapshot::{BmpPaletteEntry, BmpRowOrder};
use crate::artifacts::bmp::BmpSnapshot;
use schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ Full `stdio.bmp` artifact state — mirrors `BmpSnapshot`'s complete BITMAPINFOHEADER +
/// palette + pixels model field-for-field.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.bmp")]
pub struct BmpArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub header_size: u32,
    #[state(artifact)]
    pub width: u32,
    #[state(artifact)]
    pub height: u32,
    #[state(artifact)]
    pub row_order: BmpRowOrder,
    #[state(artifact)]
    pub planes: u16,
    #[state(artifact)]
    pub bits_per_pixel: u16,
    #[state(artifact)]
    pub compression: u32,
    #[state(artifact)]
    pub image_size: u32,
    #[state(artifact)]
    pub x_pixels_per_meter: i32,
    #[state(artifact)]
    pub y_pixels_per_meter: i32,
    #[state(artifact)]
    pub colors_used: u32,
    #[state(artifact)]
    pub colors_important: u32,
    #[state(artifact)]
    #[value(default)]
    pub palette: Vec<BmpPaletteEntry>,
    #[state(artifact)]
    #[value(default)]
    pub pixels: Vec<u8>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for BmpArtifact {
    fn default() -> Self {
        Self::from_snapshot(BmpSnapshot::default())
    }
}

impl BmpArtifact {
    /// 📸️ Persisted subset.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> BmpSnapshot {
        BmpSnapshot {
            schema: self.schema.clone(),
            header_size: self.header_size,
            width: self.width,
            height: self.height,
            row_order: self.row_order,
            planes: self.planes,
            bits_per_pixel: self.bits_per_pixel,
            compression: self.compression,
            image_size: self.image_size,
            x_pixels_per_meter: self.x_pixels_per_meter,
            y_pixels_per_meter: self.y_pixels_per_meter,
            colors_used: self.colors_used,
            colors_important: self.colors_important,
            palette: self.palette.clone(),
            pixels: self.pixels.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: BmpSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            header_size: snapshot.header_size,
            width: snapshot.width,
            height: snapshot.height,
            row_order: snapshot.row_order,
            planes: snapshot.planes,
            bits_per_pixel: snapshot.bits_per_pixel,
            compression: snapshot.compression,
            image_size: snapshot.image_size,
            x_pixels_per_meter: snapshot.x_pixels_per_meter,
            y_pixels_per_meter: snapshot.y_pixels_per_meter,
            colors_used: snapshot.colors_used,
            colors_important: snapshot.colors_important,
            palette: snapshot.palette,
            pixels: snapshot.pixels,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: BmpSnapshot) {
        self.schema = snapshot.schema;
        self.header_size = snapshot.header_size;
        self.width = snapshot.width;
        self.height = snapshot.height;
        self.row_order = snapshot.row_order;
        self.planes = snapshot.planes;
        self.bits_per_pixel = snapshot.bits_per_pixel;
        self.compression = snapshot.compression;
        self.image_size = snapshot.image_size;
        self.x_pixels_per_meter = snapshot.x_pixels_per_meter;
        self.y_pixels_per_meter = snapshot.y_pixels_per_meter;
        self.colors_used = snapshot.colors_used;
        self.colors_important = snapshot.colors_important;
        self.palette = snapshot.palette;
        self.pixels = snapshot.pixels;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.stdio.bmp`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn bmp_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.bmp",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::bmp::{BmpDiff, BmpMutation, BmpSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.bmp` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct BmpBuilderConstruction {
        snapshot: BmpSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for BmpBuilderConstruction {
        type Snapshot = BmpSnapshot;
        type Mutation = BmpMutation;
        type Diff = BmpDiff;
        fn empty() -> Self {
            Self { snapshot: BmpSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<BmpSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<BmpSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::artifacts::bmp::schema::mutations::apply_bmp_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <BmpDiff as protocol::MutationDiff<BmpSnapshot>>::apply(&diff, &self.snapshot)?;
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
    use crate::artifacts::bmp::BmpSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.bmp` parts.
    #[derive(Clone, Debug, Default)]
    pub struct BmpParts {
        pub snapshot: Option<BmpSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.bmp` (v3/✳️any) sources.
    pub struct BmpAnalyzerAnalysis;

    impl ArtifactAnalysis for BmpAnalyzerAnalysis {
        type Parts = BmpParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.bmp", standard: StandardId("v3"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            const SIG: [u8; 2] = *b"BM";
            match source {
                AnalyzeSource::Binary(bytes) => {
                    if bytes.len() >= 2 && bytes[0..2] == SIG {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    // 🔍 stdio.bmp's text envelope is a hex dump of the raw bytes after the
                    // `semio ...` preamble line — decode the first 2 bytes to sniff the real signature.
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
                    if decoded == SIG {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = BmpParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <BmpSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <BmpSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec BmpBuilderFacets {
        construction: BmpBuilderConstruction,
        analysis: BmpAnalyzerAnalysis,
        composition: super::super::io::derived_composition::BmpComposerComposition,
    }
    builder: BmpBuilder,
    analyzer: BmpAnalyzer,
    composer: BmpComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers
// 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
// `empty_bmp_snapshot`/`demo_bmp_snapshot` relocated here verbatim (pure helpers over the
// document type, destination rule 5); `BmpEngine` (zero construction sites) deleted outright;
// the real codec (`encode_bmp`/`decode_bmp` + every pure format algorithm) + the protected
// `register()` cluster (`crate::artifacts::bmp::engine::register()` is one of stdio's 10
// deliberate imperative plugin-root calls — untouched, reached via this standard's own inline
// `engine` barrel) + `io_registry` all moved to `../🚪️io`; tests moved beside what they now test.
/// 🌱 Empty persisted snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_bmp_snapshot() -> BmpSnapshot {
    BmpSnapshot::default()
}

/// 🎬 P2-FG2: canonical demo snapshot — the same value the real `.dsl.semio`/`.pack.semio`
/// fixtures under `📚️examples/🎬️demo/🖼️assets/` are genuine `print_dsl`/`encode_pack` output
/// of (regenerated this wave via a real `encode_bmp`/`print_dsl`/`encode_pack` call, replacing
/// the pre-existing fake "hello" placeholder text). 4x2 24-bit `BI_RGB`, bottom-up, 8 distinct
/// non-solid RGBA pixels (`row_bytes(4, 24) == 12`, already a multiple of 4, so this fixture
/// does NOT exercise row padding — `gradient_checkerboard_24bit_round_trip`'s own 6-wide fixture
/// in `../🚪️io`'s own tests already covers that) — `header_size`/`planes`/`bits_per_pixel`/
/// `compression` are exactly what `encode_bmp` always hardcodes (40/1/24/0, see its own
/// `EncodeScopeNote`), so this snapshot is safe against `encode_bmp`'s own canonicalization (any
/// other value here would silently "self-correct" on the first decode and break
/// `fixture_honesty_law`'s `parse_dsl(fixture) == demo()` identity). No palette (bpp=24 has none).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_bmp_snapshot() -> BmpSnapshot {
    use crate::artifacts::bmp::standards::v_v3::subsets::any::io::row_bytes;
    BmpSnapshot {
        schema: crate::artifacts::bmp::STDIO_BMP_DOCUMENT_SCHEMA.into(),
        header_size: 40,
        width: 4,
        height: 2,
        row_order: BmpRowOrder::BottomUp,
        planes: 1,
        bits_per_pixel: 24,
        compression: 0,
        image_size: row_bytes(4, 24) as u32 * 2,
        x_pixels_per_meter: 2835,
        y_pixels_per_meter: 2835,
        colors_used: 0,
        colors_important: 0,
        palette: Vec::new(),
        pixels: vec![255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 0, 255, 0, 255, 255, 255, 255, 0, 255, 255, 128, 128, 128, 255, 0, 0, 0, 255],
    }
}
//#endregion 🔖️DocumentHelpers
