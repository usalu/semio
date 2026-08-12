//! 🧬️ Remodel artifact schema — every field of the artifact with its state class.

use crate::artifacts::remodel::{
    CalibrationState, GroundControlPoint, ImageAsset, MediaStream, ReconstructionJob,
    ReconstructionParams, ReconstructionResults, RemodelSnapshot,
};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Artifact
/// 🧬️ Full remodel artifact state across persistent, shared-ui and local-ui classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.remodel.remodel")]
pub struct RemodelArtifact {
    #[state(persistent)] pub schema: String,
    #[state(persistent)] pub id: String,
    #[state(persistent)] pub streams: Vec<MediaStream>,
    #[state(persistent)] pub assets: BTreeMap<String, ImageAsset>,
    #[state(persistent)] pub calibration: CalibrationState,
    #[state(persistent)] pub params: ReconstructionParams,
    #[state(persistent)] pub gcps: Vec<GroundControlPoint>,
    #[state(persistent)] pub job: ReconstructionJob,
    #[state(persistent)] pub results: ReconstructionResults,
    #[state(shared_ui)] pub selection: RemodelUiSelection,
    #[state(shared_ui)] pub active_utility_id: String,
    #[state(shared_ui)] pub report_table: String,
    #[state(shared_ui)] pub frame_cursor: RemodelUiFrameCursor,
    #[state(local_ui)] pub camera: RemodelUiCamera,
    #[state(local_ui)] pub layers: RemodelUiLayers,
    #[state(local_ui)] pub locale: String,
}
//#endregion 🔖️Artifact

//#region 🔖️UiHelpers
/// 🎥️ Artifact-owned orbit camera (mirror of app config camera).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct RemodelUiCamera {
    pub position: [f64; 3],
    pub target: [f64; 3],
    pub fov: f64,
}

impl Default for RemodelUiCamera {
    fn default() -> Self {
        Self { position: [4.0, -4.0, 3.0], target: [0.0, 0.0, 0.0], fov: 45.0 }
    }
}

/// 🖱️ Artifact-owned selection (mirror of app config selection).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct RemodelUiSelection {
    pub mode: String,
    pub ids: Vec<String>,
}

/// 👁️ Artifact-owned layer visibility (mirror of app config layers).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct RemodelUiLayers {
    pub mesh: bool,
    pub dense: bool,
    pub sparse: bool,
    pub cameras: bool,
    pub gcps: bool,
}

impl Default for RemodelUiLayers {
    fn default() -> Self {
        Self { mesh: true, dense: true, sparse: true, cameras: true, gcps: true }
    }
}

/// 🎞️ Artifact-owned frame cursor (mirror of app config frame cursor).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct RemodelUiFrameCursor {
    pub stream_id: Option<String>,
    pub frame_index: u32,
}
//#endregion 🔖️UiHelpers

//#region 🔖️Conversions
impl Default for RemodelArtifact {
    fn default() -> Self {
        Self::from_snapshot(RemodelSnapshot::default())
    }
}

impl RemodelArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> RemodelSnapshot {
        RemodelSnapshot {
            schema: self.schema.clone(),
            id: self.id.clone(),
            streams: self.streams.clone(),
            assets: self.assets.clone(),
            calibration: self.calibration.clone(),
            params: self.params.clone(),
            gcps: self.gcps.clone(),
            job: self.job.clone(),
            results: self.results.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: RemodelSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            id: snapshot.id,
            streams: snapshot.streams,
            assets: snapshot.assets,
            calibration: snapshot.calibration,
            params: snapshot.params,
            gcps: snapshot.gcps,
            job: snapshot.job,
            results: snapshot.results,
            selection: RemodelUiSelection::default(),
            active_utility_id: "select".into(),
            report_table: "frames".into(),
            frame_cursor: RemodelUiFrameCursor::default(),
            camera: RemodelUiCamera::default(),
            layers: RemodelUiLayers::default(),
            locale: "en-US".into(),
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: RemodelSnapshot) {
        self.schema = snapshot.schema;
        self.id = snapshot.id;
        self.streams = snapshot.streams;
        self.assets = snapshot.assets;
        self.calibration = snapshot.calibration;
        self.params = snapshot.params;
        self.gcps = snapshot.gcps;
        self.job = snapshot.job;
        self.results = snapshot.results;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.remodel.remodel` — twenty handcrafted schema leaves.
pub fn remodel_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.remodel.remodel",
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
    use crate::artifacts::remodel::schema::diff::RemodelDiff;
    use crate::artifacts::remodel::schema::mutations::RemodelMutation;
    use crate::artifacts::remodel::schema::snapshot::RemodelSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct RemodelBuilderConstruction {
        snapshot: RemodelSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for RemodelBuilderConstruction {
        type Snapshot = RemodelSnapshot;
        type Mutation = RemodelMutation;
        type Diff = RemodelDiff;
        fn empty() -> Self { Self { snapshot: RemodelSnapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<RemodelSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<RemodelSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let d = <RemodelMutation as protocol::Mutation<RemodelSnapshot>>::diff(&mutation, &self.snapshot);
            self.snapshot = protocol::MutationDiff::apply(&d, &self.snapshot);
            (self, d)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <RemodelDiff as protocol::MutationDiff<RemodelSnapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::remodel::RemodelSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct RemodelParts {
        pub snapshot: Option<RemodelSnapshot>,
    }

    pub struct RemodelAnalyzerAnalysis;

    impl ArtifactAnalysis for RemodelAnalyzerAnalysis {
        type Parts = RemodelParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.remodel", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = RemodelParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <RemodelSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <RemodelSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec RemodelBuilderFacets {
        construction: derived_construction::RemodelBuilderConstruction,
        analysis: derived_analysis::RemodelAnalyzerAnalysis,
        composition: super::super::io::derived_composition::RemodelComposerComposition,
    }
    builder: RemodelBuilder,
    analyzer: RemodelAnalyzer,
    composer: RemodelComposer,
);
//#endregion 🧬️DerivedArtifactFacets
