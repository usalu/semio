//! 🧬️ Raster artifact schema — every field of the artifact with its state class.

use crate::artifacts::raster::{RasterImageAsset, RasterLayerNode, RasterViewportSize, RASTER_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Artifact
/// 🧬️ Full raster artifact state across persistent, shared-ui, local-ui and preview classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.raster.raster")]
pub struct RasterArtifact {
    #[state(persistent)] pub schema: String,
    #[state(persistent)] pub id: String,
    #[state(persistent)] pub title: Option<String>,
    #[state(persistent)] pub layers: Vec<RasterLayerNode>,
    #[state(persistent)] pub assets: BTreeMap<String, RasterImageAsset>,
    #[state(shared_ui)] pub selected_ids: Vec<String>,
    #[state(shared_ui)] pub active_utility_id: String,
    #[state(local_ui)] pub brush_size: f64,
    #[state(local_ui)] pub brush_opacity: f64,
    #[state(local_ui)] pub composite_viewport: Option<RasterViewportSize>,
    #[state(local_ui)] pub camera_x: f64,
    #[state(local_ui)] pub camera_y: f64,
    #[state(local_ui)] pub camera_zoom: f64,
    #[state(local_ui)] pub locale: String,
    #[state(preview)] pub hovered_id: Option<String>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for RasterArtifact {
    fn default() -> Self {
        Self {
            schema: RASTER_DOCUMENT_SCHEMA.into(),
            id: String::new(),
            title: None,
            layers: Vec::new(),
            assets: BTreeMap::new(),
            selected_ids: Vec::new(),
            active_utility_id: "selectMarquee".into(),
            brush_size: 24.0,
            brush_opacity: 1.0,
            composite_viewport: None,
            camera_x: 0.0,
            camera_y: 0.0,
            camera_zoom: 1.0,
            locale: "en-US".into(),
            hovered_id: None,
        }
    }
}

impl RasterArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::raster::RasterSnapshot {
        crate::artifacts::raster::RasterSnapshot {
            schema: self.schema.clone(),
            id: self.id.clone(),
            title: self.title.clone(),
            layers: self.layers.clone(),
            assets: self.assets.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::raster::RasterSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            id: snapshot.id,
            title: snapshot.title,
            layers: snapshot.layers,
            assets: snapshot.assets,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::raster::RasterSnapshot) {
        self.schema = snapshot.schema;
        self.id = snapshot.id;
        self.title = snapshot.title;
        self.layers = snapshot.layers;
        self.assets = snapshot.assets;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.raster.raster` — twenty handcrafted schema leaves.
pub fn raster_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.raster.raster",
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
    use crate::artifacts::raster::{RasterDiff, RasterMutation, RasterSnapshot};

    #[derive(Clone, Debug, Default)]
    pub struct RasterBuilderConstruction {
        snapshot: RasterSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for RasterBuilderConstruction {
        type Snapshot = RasterSnapshot;
        type Mutation = RasterMutation;
        type Diff = RasterDiff;
        fn empty() -> Self { Self { snapshot: RasterSnapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<RasterSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<RasterSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
            self.snapshot = crate::artifacts::raster::schema::mutations::apply_raster_mutation(&self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <RasterDiff as protocol::MutationDiff<RasterSnapshot>>::apply(&diff, &self.snapshot);
            self
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use semio_framework_plugin::{ArtifactAnalysis, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
    use crate::artifacts::raster::RasterSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct RasterParts {
        pub snapshot: Option<RasterSnapshot>,
    }

    pub struct RasterAnalyzerAnalysis;

    impl ArtifactAnalysis for RasterAnalyzerAnalysis {
        type Parts = RasterParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.raster", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = RasterParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <RasterSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <RasterSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.binary", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec RasterBuilderFacets {
        construction: derived_construction::RasterBuilderConstruction,
        analysis: derived_analysis::RasterAnalyzerAnalysis,
        composition: super::super::io::derived_composition::RasterComposerComposition,
    }
    builder: RasterBuilder,
    analyzer: RasterAnalyzer,
    composer: RasterComposer,
);
//#endregion 🧬️DerivedArtifactFacets
