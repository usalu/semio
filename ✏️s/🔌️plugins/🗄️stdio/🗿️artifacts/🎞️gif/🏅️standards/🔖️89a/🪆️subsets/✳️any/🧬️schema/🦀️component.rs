//! 🧬️ GifArtifact schema (89a) — full artifact state, mirrors `GifSnapshot`'s frame/GCE/loop model.

use crate::artifacts::gif::standards::v89a::subsets::any::schema::snapshot::{GifAppExtension, GifColorTable, GifFrame, GifSnapshot};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.gif.89a")]
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
    pub loop_count: Option<u16>,
    #[state(artifact)]
    #[serde(default)]
    pub frames: Vec<GifFrame>,
    #[state(artifact)]
    #[serde(default)]
    pub comments: Vec<String>,
    #[state(artifact)]
    #[serde(default)]
    pub app_extensions: Vec<GifAppExtension>,
}

impl Default for GifArtifact {
    fn default() -> Self {
        Self::from_snapshot(GifSnapshot::default())
    }
}

impl GifArtifact {
    pub async fn to_snapshot(&self) -> GifSnapshot {
        GifSnapshot {
            schema: self.schema.clone(),
            width: self.width,
            height: self.height,
            gct: self.gct.clone(),
            background_color_index: self.background_color_index,
            pixel_aspect_ratio: self.pixel_aspect_ratio,
            loop_count: self.loop_count,
            frames: self.frames.clone(),
            comments: self.comments.clone(),
            app_extensions: self.app_extensions.clone(),
        }
    }
    pub async fn from_snapshot(snapshot: GifSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            width: snapshot.width,
            height: snapshot.height,
            gct: snapshot.gct,
            background_color_index: snapshot.background_color_index,
            pixel_aspect_ratio: snapshot.pixel_aspect_ratio,
            loop_count: snapshot.loop_count,
            frames: snapshot.frames,
            comments: snapshot.comments,
            app_extensions: snapshot.app_extensions,
        }
    }
    pub async fn set_snapshot(&mut self, snapshot: GifSnapshot) {
        self.schema = snapshot.schema;
        self.width = snapshot.width;
        self.height = snapshot.height;
        self.gct = snapshot.gct;
        self.background_color_index = snapshot.background_color_index;
        self.pixel_aspect_ratio = snapshot.pixel_aspect_ratio;
        self.loop_count = snapshot.loop_count;
        self.frames = snapshot.frames;
        self.comments = snapshot.comments;
        self.app_extensions = snapshot.app_extensions;
    }
}

pub async fn gif_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.gif.89a",
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
    use crate::artifacts::gif::standards::v89a::subsets::any::schema::diff::GifDiff;
    use crate::artifacts::gif::standards::v89a::subsets::any::schema::mutations::GifMutation;
    use crate::artifacts::gif::standards::v89a::subsets::any::schema::snapshot::{GifAppExtension, GifColorTable, GifFrame, GifSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    #[derive(Clone, Debug, Default)]
    pub struct GifBuilderConstruction {
        snapshot: GifSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    //#region 🔖️TypedConstructors
    impl GifBuilderConstruction {
        /// 🏗️ Starts a fresh 89a document at the given logical screen size.
        pub async fn new(width: u32, height: u32) -> Self {
            Self { snapshot: GifSnapshot { width, height, ..GifSnapshot::default() }, diagnostics: Vec::new() }
        }
        /// 🏗️ Appends one animation frame, in order.
        pub async fn add_frame(mut self, frame: GifFrame) -> Self {
            self.snapshot.frames.push(frame);
            self
        }
        /// 🏗️ Sets the NETSCAPE2.0 loop count (`None` = no loop extension, plays once).
        pub async fn set_loop_count(mut self, loop_count: Option<u16>) -> Self {
            self.snapshot.loop_count = loop_count;
            self
        }
        /// 🏗️ Sets the Global Color Table.
        pub async fn set_global_color_table(mut self, gct: Option<GifColorTable>) -> Self {
            self.snapshot.gct = gct;
            self
        }
        /// 🏗️ Sets the logical screen's background color index.
        pub async fn set_background_color_index(mut self, index: u8) -> Self {
            self.snapshot.background_color_index = index;
            self
        }
        /// 🏗️ Sets the logical screen's pixel aspect ratio byte.
        pub async fn set_pixel_aspect_ratio(mut self, ratio: u8) -> Self {
            self.snapshot.pixel_aspect_ratio = ratio;
            self
        }
        /// 🏗️ Appends one comment extension.
        pub async fn add_comment(mut self, text: String) -> Self {
            self.snapshot.comments.push(text);
            self
        }
        /// 🏗️ Appends one non-NETSCAPE application extension verbatim.
        pub async fn add_app_extension(mut self, extension: GifAppExtension) -> Self {
            self.snapshot.app_extensions.push(extension);
            self
        }
    }
    //#endregion 🔖️TypedConstructors

    impl ArtifactBuilder for GifBuilderConstruction {
        type Snapshot = GifSnapshot;
        type Mutation = GifMutation;
        type Diff = GifDiff;
        async fn empty() -> Self {
            Self { snapshot: GifSnapshot::default(), diagnostics: Vec::new() }
        }
        async fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        async fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<GifSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        async fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<GifSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        async fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::artifacts::gif::standards::v89a::subsets::any::schema::mutations::apply_gif_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        async fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <GifDiff as protocol::MutationDiff<GifSnapshot>>::apply(&diff, &self.snapshot)?;
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
    use crate::artifacts::gif::standards::v89a::subsets::any::schema::snapshot::GifSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.gif` parts.
    #[derive(Clone, Debug, Default)]
    pub struct GifParts {
        pub snapshot: Option<GifSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.gif` (89a/✳️any) sources.
    pub struct GifAnalyzerAnalysis;

    impl ArtifactAnalysis for GifAnalyzerAnalysis {
        type Parts = GifParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gif", standard: StandardId("89a"), subset: SubsetId("*") };

        async fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            crate::artifacts::gif::standards::v87a::engine::sniff_magic(source, b"GIF89a")
        }

        async fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
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
// the real GIF89a codec (multi-frame animation, Graphic Control Extension, NETSCAPE2.0 loop —
// reusing 87a's own `pub` byte-level LZW/sub-block/color-table/quantize/interlace helpers
// verbatim) + the protected `register()` cluster (`crate::artifacts::gif::engine::register()`'s
// own local override explicitly calls BOTH `standards::v87a::engine::register()` AND
// `standards::v89a::engine::register()` — untouched) + `io_registry` all moved to `../🚪️io`;
// tests moved beside what they now test.
pub async fn empty_gif_snapshot() -> GifSnapshot {
    GifSnapshot::default()
}

/// 🧪️ P2-FG2: real, deterministic demo `GifSnapshot` for `conformance_laws` (in `../🚪️io`'s own
/// tests) and the shipped `.dsl.semio`/`.pack.semio` fixtures (`../📚️examples/🎬️demo/🖼️assets/`)
/// — per the ticket's own instruction, this reuses the REAL `dancing.gif` fixture
/// (`crate::artifacts::gif::examples::dancing::decoded_snapshot()`, 54 frames, 800×800,
/// per-frame LCTs, NETSCAPE2.0 loop) decoded via the real 89a codec, for byte-real
/// conformance — not a synthetic stand-in.
pub async fn demo_gif_snapshot() -> GifSnapshot {
    crate::artifacts::gif::examples::dancing::decoded_snapshot()
}
//#endregion 🔖️DocumentHelpers
