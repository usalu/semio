//! 🧬️ GifArtifact schema — full artifact state.

// 🔀️ S-6: `crate::artifacts::gif::schema` now shims to 89a (canonical) -- 87a's own schema uses
// its own standard-local snapshot type directly rather than the shared root re-export.
use crate::artifacts::gif::standards::v87a::subsets::any::schema::snapshot::{GifColorTable, GifImage, GifSnapshot};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.gif")]
pub struct GifArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub width: u32,
    #[state(artifact)]
    pub height: u32,
    #[state(artifact)]
    #[serde(default)]
    pub gct: Option<GifColorTable>,
    #[state(artifact)]
    #[serde(default)]
    pub background_color_index: u8,
    #[state(artifact)]
    #[serde(default)]
    pub pixel_aspect_ratio: u8,
    #[state(artifact)]
    #[serde(default)]
    pub images: Vec<GifImage>,
}

impl Default for GifArtifact {
    fn default() -> Self {
        Self::from_snapshot(GifSnapshot::default())
    }
}

impl GifArtifact {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> GifSnapshot {
        GifSnapshot { schema: self.schema.clone(), width: self.width, height: self.height, gct: self.gct.clone(), background_color_index: self.background_color_index, pixel_aspect_ratio: self.pixel_aspect_ratio, images: self.images.clone() }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: GifSnapshot) -> Self {
        Self { schema: snapshot.schema, width: snapshot.width, height: snapshot.height, gct: snapshot.gct, background_color_index: snapshot.background_color_index, pixel_aspect_ratio: snapshot.pixel_aspect_ratio, images: snapshot.images }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: GifSnapshot) {
        self.schema = snapshot.schema;
        self.width = snapshot.width;
        self.height = snapshot.height;
        self.gct = snapshot.gct;
        self.background_color_index = snapshot.background_color_index;
        self.pixel_aspect_ratio = snapshot.pixel_aspect_ratio;
        self.images = snapshot.images;
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn gif_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.gif",
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
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::gif::standards::v87a::subsets::any::schema::{diff::GifDiff, mutations::GifMutation, snapshot::GifSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.gif` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct GifBuilderConstruction {
        snapshot: GifSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for GifBuilderConstruction {
        type Snapshot = GifSnapshot;
        type Mutation = GifMutation;
        type Diff = GifDiff;
        fn empty() -> Self {
            Self { snapshot: GifSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<GifSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<GifSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::artifacts::gif::standards::v87a::subsets::any::schema::mutations::apply_gif_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <GifDiff as protocol::MutationDiff<GifSnapshot>>::apply(&diff, &self.snapshot)?;
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
    use crate::artifacts::gif::standards::v87a::subsets::any::schema::snapshot::GifSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.gif` parts.
    #[derive(Clone, Debug, Default)]
    pub struct GifParts {
        pub snapshot: Option<GifSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.gif` (87a/✳️any) sources.
    pub struct GifAnalyzerAnalysis;

    impl ArtifactAnalysis for GifAnalyzerAnalysis {
        type Parts = GifParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gif", standard: StandardId("87a"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            crate::artifacts::gif::standards::v87a::engine::sniff_magic(source, b"GIF87a")
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = GifParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <GifSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <GifSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec GifBuilderFacets {
        construction: GifBuilderConstruction,
        analysis: GifAnalyzerAnalysis,
        composition: super::super::io::derived_composition::GifComposerComposition,
    }
    builder: GifBuilder,
    analyzer: GifAnalyzer,
    composer: GifComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers
// 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
// `empty_gif_snapshot`/`demo_gif_snapshot` relocated here verbatim (pure helpers over the
// document type, destination rule 5); `GifEngine` (zero construction sites) deleted outright;
// the real byte-level LZW/sub-block/color-table/quantize/interlace codec (`pub`, reused verbatim
// by 89a's own engine) + `encode_gif`/`decode_gif` + `sniff_magic` + the protected `register()`
// cluster (`crate::artifacts::gif::engine::register()` is one of stdio's 10 deliberate imperative
// plugin-root calls — untouched, reached via this standard's own inline `engine` barrel) +
// `io_registry` all moved to `../🚪️io`; tests moved beside what they now test.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_gif_snapshot() -> GifSnapshot {
    GifSnapshot::default()
}

/// 🧪️ P2-FG2: real, deterministic demo `GifSnapshot` — a real GCT plus two real images (one
/// with its own LCT, exercising every field a genuine encode/decode round-trip touches) — used
/// by `conformance_laws` (in `../🚪️io`'s own tests) and by the shipped `.dsl.semio`/
/// `.pack.semio` fixtures (`../📚️examples/🎬️demo/🖼️assets/`), matching png's own
/// `demo_png_snapshot()` precedent.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_gif_snapshot() -> GifSnapshot {
    use crate::artifacts::gif::standards::v87a::subsets::any::schema::snapshot::GifRgb;
    let gct = GifColorTable { sorted: false, colors: vec![GifRgb { r: 0, g: 0, b: 0 }, GifRgb { r: 255, g: 255, b: 255 }] };
    let image_a = GifImage { left: 0, top: 0, width: 2, height: 2, interlace: false, lct: None, indices: vec![0, 1, 1, 0] };
    let image_b = GifImage { left: 0, top: 0, width: 2, height: 2, interlace: false, lct: Some(GifColorTable { sorted: true, colors: vec![GifRgb { r: 10, g: 20, b: 30 }, GifRgb { r: 200, g: 100, b: 50 }] }), indices: vec![1, 0, 0, 1] };
    GifSnapshot { schema: crate::artifacts::gif::STDIO_GIF_DOCUMENT_SCHEMA.into(), width: 2, height: 2, gct: Some(gct), background_color_index: 0, pixel_aspect_ratio: 0, images: vec![image_a, image_b] }
}
//#endregion 🔖️DocumentHelpers
