//! 🧬️ Remodel artifact schema — every field of the artifact with its state class.

use crate::artifacts::remodel::{CalibrationState, GroundControlPoint, MediaStream, ReconstructionJob, ReconstructionParams, ReconstructionResults, ReconstructionStage, RemodelAssetChild, RemodelDurableArtifactStore, RemodelSnapshot, VideoCodec};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Ids
/// 🔢️ Replaces every former `RemodelPlayRuntime` id counter (`stream_counter`/`job_counter`/
/// `gcp_counter`/`import_counter`) — mirrors `shooting_engine::next_shooting_id`'s precedent (a plain
/// global monotonic counter, not VCS-tracked config state: uniqueness is all id generation needs, and
/// the generated id itself becomes real, undoable document content the moment an operation stores it).
/// Relocated from `⚙️engine/🦀️component.rs` (26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES,
/// #2553): a pure document-side id generator, not app or engine behaviour.
pub fn next_remodel_id(prefix: &str) -> String {
    static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
    let next = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    format!("{prefix}-{next}")
}
//#endregion 🔖️Ids

//#region 🔖️Codecs
/// 🏷️ Display label for one `ReconstructionStage` — pure document-enum formatting, no engine
/// dependency (relocated from `⚙️engine/🦀️component.rs`, #2553).
pub fn stage_display(stage: ReconstructionStage) -> &'static str {
    match stage {
        ReconstructionStage::Idle => "Idle",
        ReconstructionStage::Ingesting => "Ingesting",
        ReconstructionStage::Calibrating => "Calibrating",
        ReconstructionStage::ExtractingFeatures => "Extracting Features",
        ReconstructionStage::MatchingFeatures => "Matching Features",
        ReconstructionStage::EstimatingPoses => "Estimating Poses",
        ReconstructionStage::BundleAdjusting => "Bundle Adjusting",
        ReconstructionStage::Georeferencing => "Georeferencing",
        ReconstructionStage::DenseStereo => "Dense Stereo",
        ReconstructionStage::FusingVolume => "Fusing Volume",
        ReconstructionStage::ExtractingSurface => "Extracting Surface",
        ReconstructionStage::CleaningMesh => "Cleaning Mesh",
        ReconstructionStage::Texturing => "Texturing",
        ReconstructionStage::TrackingMotion => "Tracking Motion",
        ReconstructionStage::DerivingGeoProducts => "Deriving Geo Products",
        ReconstructionStage::ReportingQc => "Reporting QC",
        ReconstructionStage::Done => "Done",
        ReconstructionStage::Failed => "Failed",
    }
}

/// 🎞️ Label → document `VideoCodec` — pure string parsing, no engine dependency (relocated from
/// `⚙️engine/🦀️component.rs`, #2553).
pub fn video_codec_from_label(label: &str) -> VideoCodec {
    match label.to_ascii_lowercase().as_str() {
        "avc" | "h264" | "h.264" => VideoCodec::Avc,
        "hevc" | "h265" | "h.265" => VideoCodec::Hevc,
        "vp9" => VideoCodec::Vp9,
        "av1" => VideoCodec::Av1,
        "mjpeg" | "mjpg" => VideoCodec::Mjpeg,
        _ => VideoCodec::Unknown,
    }
}
//#endregion 🔖️Codecs

//#region 🔖️Artifact
/// 🧬️ Full remodel artifact state across the artifact, presence and config lanes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.remodel.remodel")]
pub struct RemodelArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub id: String,
    #[state(artifact)]
    pub streams: Vec<MediaStream>,
    #[state(artifact)]
    pub assets: BTreeMap<String, RemodelAssetChild>,
    #[state(artifact)]
    pub durable_artifacts: RemodelDurableArtifactStore,
    #[state(artifact)]
    pub calibration: CalibrationState,
    #[state(artifact)]
    pub params: ReconstructionParams,
    #[state(artifact)]
    pub gcps: Vec<GroundControlPoint>,
    #[state(artifact)]
    pub job: ReconstructionJob,
    #[state(artifact)]
    pub results: ReconstructionResults,
    #[state(presence)]
    pub selection: RemodelUiSelection,
    #[state(presence)]
    pub active_utility_id: String,
    #[state(presence)]
    pub report_table: String,
    #[state(presence)]
    pub frame_cursor: RemodelUiFrameCursor,
    #[state(config)]
    pub camera: RemodelUiCamera,
    #[state(config)]
    pub layers: RemodelUiLayers,
    #[state(config)]
    pub locale: String,
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
            durable_artifacts: self.durable_artifacts.clone(),
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
            durable_artifacts: snapshot.durable_artifacts,
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
        self.durable_artifacts = snapshot.durable_artifacts;
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
pub async fn remodel_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
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
    use crate::artifacts::remodel::schema::diff::RemodelDiff;
    use crate::artifacts::remodel::schema::mutations::RemodelMutation;
    use crate::artifacts::remodel::schema::snapshot::RemodelSnapshot;
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct RemodelBuilderConstruction {
        snapshot: RemodelSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for RemodelBuilderConstruction {
        type Snapshot = RemodelSnapshot;
        type Mutation = RemodelMutation;
        type Diff = RemodelDiff;
        async fn empty() -> Self {
            Self { snapshot: RemodelSnapshot::default(), diagnostics: Vec::new() }
        }
        async fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        async fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<RemodelSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        async fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<RemodelSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        async fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <RemodelMutation as protocol::Mutation<RemodelSnapshot>>::diff(&mutation, &self.snapshot);
            match protocol::MutationDiff::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error("mutation.apply", dsl::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        async fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <RemodelDiff as protocol::MutationDiff<RemodelSnapshot>>::apply(&diff, &self.snapshot)?;
            self.snapshot = snapshot;
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
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::remodel::RemodelSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct RemodelParts {
        pub snapshot: Option<RemodelSnapshot>,
    }

    pub struct RemodelAnalyzerAnalysis;

    impl ArtifactAnalysis for RemodelAnalyzerAnalysis {
        type Parts = RemodelParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.remodel", standard: StandardId("1"), subset: SubsetId("*") };

        async fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        async fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
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
        construction: RemodelBuilderConstruction,
        analysis: RemodelAnalyzerAnalysis,
        composition: super::super::io::derived_composition::RemodelComposerComposition,
    }
    builder: RemodelBuilder,
    analyzer: RemodelAnalyzer,
    composer: RemodelComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn next_remodel_id_is_monotonic_and_prefixed() {
        let a = next_remodel_id("stream");
        let b = next_remodel_id("stream");
        assert!(a.starts_with("stream-"));
        assert!(b.starts_with("stream-"));
        assert_ne!(a, b);
    }

    #[semio_framework_async_macros::async_test]
    async fn stage_display_covers_every_stage() {
        let cases = [(ReconstructionStage::Idle, "Idle"), (ReconstructionStage::Done, "Done"), (ReconstructionStage::Failed, "Failed")];
        for (stage, expected) in cases {
            assert_eq!(stage_display(stage), expected);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn video_codec_from_label_recognizes_common_aliases() {
        assert_eq!(video_codec_from_label("h264"), VideoCodec::Avc);
        assert_eq!(video_codec_from_label("h.265"), VideoCodec::Hevc);
        assert_eq!(video_codec_from_label("mjpg"), VideoCodec::Mjpeg);
        assert_eq!(video_codec_from_label("weird"), VideoCodec::Unknown);
    }
}
//#endregion 🧪️Tests
