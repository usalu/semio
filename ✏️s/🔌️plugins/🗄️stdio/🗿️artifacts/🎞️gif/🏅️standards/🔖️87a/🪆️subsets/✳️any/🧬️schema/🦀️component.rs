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
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub width: u32,
    #[state(persistent)]
    pub height: u32,
    #[state(persistent)]
    #[serde(default)]
    pub gct: Option<GifColorTable>,
    #[state(persistent)]
    #[serde(default)]
    pub background_color_index: u8,
    #[state(persistent)]
    #[serde(default)]
    pub pixel_aspect_ratio: u8,
    #[state(persistent)]
    #[serde(default)]
    pub images: Vec<GifImage>,
}

impl Default for GifArtifact {
    fn default() -> Self { Self::from_snapshot(GifSnapshot::default()) }
}

impl GifArtifact {
    pub fn to_snapshot(&self) -> GifSnapshot {
        GifSnapshot {
            schema: self.schema.clone(),
            width: self.width,
            height: self.height,
            gct: self.gct.clone(),
            background_color_index: self.background_color_index,
            pixel_aspect_ratio: self.pixel_aspect_ratio,
            images: self.images.clone(),
        }
    }
    pub fn from_snapshot(snapshot: GifSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            width: snapshot.width,
            height: snapshot.height,
            gct: snapshot.gct,
            background_color_index: snapshot.background_color_index,
            pixel_aspect_ratio: snapshot.pixel_aspect_ratio,
            images: snapshot.images,
        }
    }
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
    use crate::artifacts::gif::standards::v87a::subsets::any::schema::{diff::GifDiff, mutations::GifMutation, snapshot::GifSnapshot};

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
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = crate::artifacts::gif::standards::v87a::subsets::any::schema::mutations::apply_gif_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <GifDiff as protocol::MutationDiff<GifSnapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::gif::standards::v87a::subsets::any::schema::snapshot::GifSnapshot;

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
                            diagnostics.push(dsl::Diagnostic::error(
                                "stdio.analyze.text",
                                dsl::TextSpan::at(1, 1),
                                err.to_string(),
                            ));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <GifSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec GifBuilderFacets {
        construction: derived_construction::GifBuilderConstruction,
        analysis: derived_analysis::GifAnalyzerAnalysis,
        composition: super::super::io::derived_composition::GifComposerComposition,
    }
    builder: GifBuilder,
    analyzer: GifAnalyzer,
    composer: GifComposer,
);
//#endregion 🧬️DerivedArtifactFacets
