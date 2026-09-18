//! 🏗️ Remodeling reconstruction tool run (`📋️tool-run-contract.md` §2.4, §3.7): the `ToolRunDefinition`
//! vocabulary and the run and revalidate jobs. The run job ingests the base document's frames, drives the
//! observed [`ReconstructionEngine`] and turns every pipeline decision into trace records and steps:
//! frame verdicts, per-candidate match verdicts (accepted = success, ratio test = warning, cross check =
//! danger), registered cameras and triangulated points as `instance3d` records the moment they appear,
//! pruned points turning danger, depth maps, dense points and the extracted mesh. Its products are
//! provisional `append-content` ops plus one `commit-reconstruction` op; nothing reaches the document before
//! finalize. Source of record: `🔣️.json` beside this file.

use crate::editor::remodeling::engine::images::{BoundedDecodeProgress, BoundedStillDecoder, CompressedChunkRope, ImageRgba8};
use crate::editor::remodeling::engine::reconstruction::{EngineObservation, EngineStage, EngineStatus, FrameAcceptance, ReconstructionEngine, TerminalGeoPreparation};
use crate::editor::remodeling::engine::{build_engine_params, camera_pose_preview, geo as remodeling_geo, watertight_snapshot, RasterPngPreparation, RasterPngProgress};
use crate::mutations::{append_content, commit_reconstruction, AppendContent, CommitReconstruction, ReconstructionAssetCommit, RemodelingMutation};
use crate::{
    mesh_is_within_resolution_envelope, remodeling_content_handle, remodeling_mesh_content_handle, CameraPosePreview, CameraTrajectory, FrameRef, GeoProducts, MeshSource, PackedF32, QcReportSnapshot, RemodelingContentDigest, RemodelingContentKind, RemodelingMesh,
    RemodelingSnapshot, SparseCloud, WatertightReportSnapshot, REMODELING_DURABLE_CHUNK_RAW_BYTES,
};
use semio_framework_job::{Checkpoint, CommitCandidate, InteractiveJob, InteractiveJobCloseStep, JobFault, JobPayloadStream, RetainedJobPayload, StepContext, StepOutcome};
use semio_framework_plugin::LocalizedLabel;
use semio_framework_tool_run::{
    JobKindId, ToolRunCounter, ToolRunCounterDefinition, ToolRunDefinition, ToolRunIdentity, ToolRunProgress, ToolRunReasonDefinition, ToolRunRebasePolicy, ToolRunReconfigurePolicy, ToolRunSettingsReads, ToolRunStageDefinition, ToolRunState, ToolRunStepArg, ToolRunStepKind, ToolRunStepRing,
    ToolRunTickWriter, ToolRunTraceKind, ToolRunTraceSubject, ToolRunVerdict, TOOL_RUN_REASON_CONFLICT,
};
use std::collections::{BTreeSet, VecDeque};
use std::sync::Arc;

//#region 🔖️Contract
pub const RECONSTRUCTION_RUN_JOB_KIND: &str = "remodeling.reconstruction.run";
pub const RECONSTRUCTION_REVALIDATE_JOB_KIND: &str = "remodeling.reconstruction.revalidate";
pub const RECONSTRUCTION_RUN_SCHEMA: &str = "remodeling.reconstruction.run.v1";
/// 🧊️ The trace mesh lane the Model window publishes first, in this order; `instance3d` records index it.
pub const RECONSTRUCTION_TRACE_MESH_LANE: [&str; 2] = ["remodeling-trace-point", "remodeling-trace-camera"];
pub const RECONSTRUCTION_TRACE_POINT_MESH: u32 = 0;
pub const RECONSTRUCTION_TRACE_CAMERA_MESH: u32 = 1;
pub const RECONSTRUCTION_TRACE_POINT_SCALE: f32 = 0.01;
pub const RECONSTRUCTION_TRACE_CAMERA_SCALE: f32 = 0.05;
/// ☁️ Dense points traced per decision, and the most dense points a run shows.
pub const RECONSTRUCTION_DENSE_TRACE_BATCH: usize = 128;
pub const RECONSTRUCTION_DENSE_TRACE_MAX: usize = 65_536;
const TICK_FLUSH_BYTES: usize = 6 * 1024;
const CHECKPOINT_DECISIONS: u64 = 64;
const MAX_STILL_INPUT_BYTES: usize = 1_114_112;
const SHARPNESS_PIXELS_PER_UNIT: usize = 4_096;
const TERMINAL_CAMERA_WORK: usize = 64;
const TERMINAL_POINT_WORK: usize = 256;
const TERMINAL_QUALITY_WORK: usize = 256;
const TERMINAL_GEO_WORK: usize = 256;
const RASTER_CELL_WORK: usize = 4_096;
const REMODELING_RECONSTRUCTION_DSM_ASSET_ID: &str = "reconstruction-dsm";
const REMODELING_RECONSTRUCTION_DTM_ASSET_ID: &str = "reconstruction-dtm";

/// 🧭️ Stages of the run as the panel names them. `Ord` follows the pipeline order, which is what
/// keeps the reported stage monotone.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReconstructionRunStage {
    Ingest,
    Features,
    Matching,
    Poses,
    Bundle,
    Dense,
    Fusion,
    Surface,
    Texturing,
    Products,
}

impl ReconstructionRunStage {
    pub const ALL: [Self; 10] = [Self::Ingest, Self::Features, Self::Matching, Self::Poses, Self::Bundle, Self::Dense, Self::Fusion, Self::Surface, Self::Texturing, Self::Products];

    pub fn index(self) -> u16 {
        self as u16
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Ingest => "ingest",
            Self::Features => "features",
            Self::Matching => "matching",
            Self::Poses => "poses",
            Self::Bundle => "bundle",
            Self::Dense => "dense",
            Self::Fusion => "fusion",
            Self::Surface => "surface",
            Self::Texturing => "texturing",
            Self::Products => "products",
        }
    }

    pub fn label(self) -> LocalizedLabel {
        match self {
            Self::Ingest => LocalizedLabel::native("Ingesting frames", "Bilder werden eingelesen"),
            Self::Features => LocalizedLabel::native("Extracting features", "Merkmale werden extrahiert"),
            Self::Matching => LocalizedLabel::native("Matching features", "Merkmale werden zugeordnet"),
            Self::Poses => LocalizedLabel::native("Estimating camera poses", "Kameraposen werden geschätzt"),
            Self::Bundle => LocalizedLabel::native("Bundle adjusting", "Bündelausgleich"),
            Self::Dense => LocalizedLabel::native("Dense stereo", "Dichtes Stereo"),
            Self::Fusion => LocalizedLabel::native("Fusing the volume", "Volumen wird fusioniert"),
            Self::Surface => LocalizedLabel::native("Extracting the surface", "Oberfläche wird extrahiert"),
            Self::Texturing => LocalizedLabel::native("Texturing", "Texturierung"),
            Self::Products => LocalizedLabel::native("Preparing the result", "Ergebnis wird vorbereitet"),
        }
    }

    /// 🧭️ The run stage an engine stage belongs to.
    pub fn of(stage: EngineStage) -> Self {
        match stage {
            EngineStage::Idle => Self::Ingest,
            EngineStage::ExtractingFeatures => Self::Features,
            EngineStage::MatchingFeatures => Self::Matching,
            EngineStage::EstimatingPoses => Self::Poses,
            EngineStage::BundleAdjusting => Self::Bundle,
            EngineStage::DenseStereo => Self::Dense,
            EngineStage::FusingVolume => Self::Fusion,
            EngineStage::ExtractingSurface | EngineStage::CleaningMesh => Self::Surface,
            EngineStage::Texturing => Self::Texturing,
            EngineStage::Done | EngineStage::Failed => Self::Products,
        }
    }
}

/// 🔢️ Counters every progress snapshot carries, in this order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReconstructionRunCounter {
    FramesAccepted,
    FramesRejected,
    Keypoints,
    MatchesAccepted,
    MatchesRejected,
    Cameras,
    SparsePoints,
    DensePoints,
}

impl ReconstructionRunCounter {
    pub const ALL: [Self; 8] = [Self::FramesAccepted, Self::FramesRejected, Self::Keypoints, Self::MatchesAccepted, Self::MatchesRejected, Self::Cameras, Self::SparsePoints, Self::DensePoints];

    pub fn index(self) -> u16 {
        self as u16
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::FramesAccepted => "framesAccepted",
            Self::FramesRejected => "framesRejected",
            Self::Keypoints => "keypoints",
            Self::MatchesAccepted => "matchesAccepted",
            Self::MatchesRejected => "matchesRejected",
            Self::Cameras => "cameras",
            Self::SparsePoints => "sparsePoints",
            Self::DensePoints => "densePoints",
        }
    }

    pub fn label(self) -> LocalizedLabel {
        match self {
            Self::FramesAccepted => LocalizedLabel::native("Frames accepted", "Angenommene Bilder"),
            Self::FramesRejected => LocalizedLabel::native("Frames rejected", "Verworfene Bilder"),
            Self::Keypoints => LocalizedLabel::native("Keypoints", "Schlüsselpunkte"),
            Self::MatchesAccepted => LocalizedLabel::native("Matches accepted", "Angenommene Zuordnungen"),
            Self::MatchesRejected => LocalizedLabel::native("Matches rejected", "Verworfene Zuordnungen"),
            Self::Cameras => LocalizedLabel::native("Cameras registered", "Registrierte Kameras"),
            Self::SparsePoints => LocalizedLabel::native("Sparse points", "Dünne Punkte"),
            Self::DensePoints => LocalizedLabel::native("Dense points", "Dichte Punkte"),
        }
    }
}

/// 🗒️ Trace and step reasons; codes are ordinals.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReconstructionRunReason {
    FrameAccepted,
    FrameSkipped,
    FrameBlurred,
    FrameUnreadable,
    FeaturesExtracted,
    MatchAccepted,
    MatchAmbiguous,
    MatchAsymmetric,
    PairMatched,
    TracksBuilt,
    CameraRegistered,
    CameraRejected,
    PointsTriangulated,
    PointsPruned,
    DepthMapEstimated,
    DenseCloudFused,
    DenseTraceCapped,
    MeshExtracted,
    MeshOutsideEnvelope,
    SparseCapped,
    ContentPrepared,
    ResultReady,
    TooFewFrames,
    PipelineFailed,
    RasterFailed,
    Resuming,
    InputsChanged,
    /// 🧭️ Matches of a pair dropped by the geometric verification (appended last: codes are ordinals).
    PairGeometryRejected,
    /// 🌱️ No adjacent pair of frames could be solved for a relative pose; the run has no cameras.
    SeedPairFailed,
}

impl ReconstructionRunReason {
    pub const ALL: [Self; 29] = [
        Self::FrameAccepted,
        Self::FrameSkipped,
        Self::FrameBlurred,
        Self::FrameUnreadable,
        Self::FeaturesExtracted,
        Self::MatchAccepted,
        Self::MatchAmbiguous,
        Self::MatchAsymmetric,
        Self::PairMatched,
        Self::TracksBuilt,
        Self::CameraRegistered,
        Self::CameraRejected,
        Self::PointsTriangulated,
        Self::PointsPruned,
        Self::DepthMapEstimated,
        Self::DenseCloudFused,
        Self::DenseTraceCapped,
        Self::MeshExtracted,
        Self::MeshOutsideEnvelope,
        Self::SparseCapped,
        Self::ContentPrepared,
        Self::ResultReady,
        Self::TooFewFrames,
        Self::PipelineFailed,
        Self::RasterFailed,
        Self::Resuming,
        Self::InputsChanged,
        Self::PairGeometryRejected,
        Self::SeedPairFailed,
    ];

    pub fn code(self) -> u16 {
        self as u16
    }

    pub fn from_code(code: u16) -> Option<Self> {
        Self::ALL.get(usize::from(code)).copied()
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::FrameAccepted => "frameAccepted",
            Self::FrameSkipped => "frameSkipped",
            Self::FrameBlurred => "frameBlurred",
            Self::FrameUnreadable => "frameUnreadable",
            Self::FeaturesExtracted => "featuresExtracted",
            Self::MatchAccepted => "matchAccepted",
            Self::MatchAmbiguous => "matchAmbiguous",
            Self::MatchAsymmetric => "matchAsymmetric",
            Self::PairMatched => "pairMatched",
            Self::TracksBuilt => "tracksBuilt",
            Self::CameraRegistered => "cameraRegistered",
            Self::CameraRejected => "cameraRejected",
            Self::PointsTriangulated => "pointsTriangulated",
            Self::PointsPruned => "pointsPruned",
            Self::DepthMapEstimated => "depthMapEstimated",
            Self::DenseCloudFused => "denseCloudFused",
            Self::DenseTraceCapped => "denseTraceCapped",
            Self::MeshExtracted => "meshExtracted",
            Self::MeshOutsideEnvelope => "meshOutsideEnvelope",
            Self::SparseCapped => "sparseCapped",
            Self::ContentPrepared => "contentPrepared",
            Self::ResultReady => "resultReady",
            Self::TooFewFrames => "tooFewFrames",
            Self::PipelineFailed => "pipelineFailed",
            Self::RasterFailed => "rasterFailed",
            Self::Resuming => "resuming",
            Self::InputsChanged => "inputsChanged",
            Self::PairGeometryRejected => "pairGeometryRejected",
            Self::SeedPairFailed => "seedPairFailed",
        }
    }

    pub fn verdict(self) -> ToolRunVerdict {
        match self {
            Self::FrameSkipped | Self::FrameBlurred | Self::MatchAmbiguous | Self::CameraRejected | Self::DenseTraceCapped | Self::MeshOutsideEnvelope | Self::SparseCapped | Self::PairGeometryRejected | Self::SeedPairFailed => ToolRunVerdict::Warning,
            Self::FrameUnreadable | Self::MatchAsymmetric | Self::PointsPruned | Self::TooFewFrames | Self::PipelineFailed | Self::RasterFailed | Self::InputsChanged => ToolRunVerdict::Danger,
            _ => ToolRunVerdict::Success,
        }
    }

    pub fn step_kind(self) -> ToolRunStepKind {
        match self.verdict() {
            ToolRunVerdict::Warning => ToolRunStepKind::Warning,
            ToolRunVerdict::Danger => ToolRunStepKind::Danger,
            ToolRunVerdict::Testing => ToolRunStepKind::Info,
            ToolRunVerdict::Success => ToolRunStepKind::Success,
        }
    }

    pub fn template(self) -> LocalizedLabel {
        match self {
            Self::FrameAccepted => LocalizedLabel::native("Frame {0} accepted", "Bild {0} angenommen"),
            Self::FrameSkipped => LocalizedLabel::native("Frame {0} skipped by the sampling stride or frame limit", "Bild {0} durch Abtastschritt oder Bildgrenze übersprungen"),
            Self::FrameBlurred => LocalizedLabel::native("Frame {0} rejected as too blurred", "Bild {0} als zu unscharf verworfen"),
            Self::FrameUnreadable => LocalizedLabel::native("Frame {0} could not be decoded", "Bild {0} konnte nicht dekodiert werden"),
            Self::FeaturesExtracted => LocalizedLabel::native("Frame {0}: {1} keypoints", "Bild {0}: {1} Schlüsselpunkte"),
            Self::MatchAccepted => LocalizedLabel::native("Match between frames {0} and {1} accepted", "Zuordnung zwischen Bild {0} und {1} angenommen"),
            Self::MatchAmbiguous => LocalizedLabel::native("Match between frames {0} and {1} rejected by the ratio test", "Zuordnung zwischen Bild {0} und {1} durch den Verhältnistest verworfen"),
            Self::MatchAsymmetric => LocalizedLabel::native("Match between frames {0} and {1} rejected by the cross check", "Zuordnung zwischen Bild {0} und {1} durch die Kreuzprüfung verworfen"),
            Self::PairMatched => LocalizedLabel::native("Frames {0} and {1}: {2} matches", "Bilder {0} und {1}: {2} Zuordnungen"),
            Self::TracksBuilt => LocalizedLabel::native("{0} feature tracks built", "{0} Merkmalsspuren gebildet"),
            Self::CameraRegistered => LocalizedLabel::native("Camera of frame {0} registered", "Kamera von Bild {0} registriert"),
            Self::CameraRejected => LocalizedLabel::native("Camera of frame {0} could not be registered", "Kamera von Bild {0} konnte nicht registriert werden"),
            Self::PointsTriangulated => LocalizedLabel::native("{0} points triangulated", "{0} Punkte trianguliert"),
            Self::PointsPruned => LocalizedLabel::native("{0} points pruned by their reprojection error", "{0} Punkte wegen ihres Reprojektionsfehlers entfernt"),
            Self::DepthMapEstimated => LocalizedLabel::native("Depth map {0}: {1} valid samples", "Tiefenkarte {0}: {1} gültige Stichproben"),
            Self::DenseCloudFused => LocalizedLabel::native("{0} dense points fused", "{0} dichte Punkte fusioniert"),
            Self::DenseTraceCapped => LocalizedLabel::native("Showing {0} of {1} dense points", "{0} von {1} dichten Punkten werden gezeigt"),
            Self::MeshExtracted => LocalizedLabel::native("Mesh with {0} vertices and {1} triangles extracted", "Netz mit {0} Eckpunkten und {1} Dreiecken extrahiert"),
            Self::MeshOutsideEnvelope => LocalizedLabel::native("Mesh with {0} vertices and {1} triangles exceeds the storable envelope and is not kept", "Netz mit {0} Eckpunkten und {1} Dreiecken überschreitet die speicherbare Grenze und wird nicht behalten"),
            Self::SparseCapped => LocalizedLabel::native("Keeping {0} of {1} sparse points", "{0} von {1} dünnen Punkten werden behalten"),
            Self::ContentPrepared => LocalizedLabel::native("Content of {0} leaves ({1} bytes) prepared", "Inhalt aus {0} Blättern ({1} Bytes) vorbereitet"),
            Self::ResultReady => LocalizedLabel::native("Result ready: {0} provisional changes", "Ergebnis bereit: {0} vorläufige Änderungen"),
            Self::TooFewFrames => LocalizedLabel::native("Only {0} frames accepted; at least 2 are needed", "Nur {0} Bilder angenommen; mindestens 2 werden benötigt"),
            Self::PipelineFailed => LocalizedLabel::native("Reconstruction failed in stage {0}", "Rekonstruktion in Stufe {0} fehlgeschlagen"),
            Self::RasterFailed => LocalizedLabel::native("A terrain raster could not be encoded", "Ein Geländeraster konnte nicht kodiert werden"),
            Self::Resuming => LocalizedLabel::native("Resumed after {0} decisions", "Nach {0} Entscheidungen fortgesetzt"),
            Self::InputsChanged => LocalizedLabel::native("Frames, parameters or calibration changed since the run started; {0} provisional changes withdrawn", "Bilder, Parameter oder Kalibrierung haben sich seit dem Start geändert; {0} vorläufige Änderungen zurückgezogen"),
            Self::PairGeometryRejected => LocalizedLabel::native("Frames {0} and {1}: {2} matches inconsistent with the pair's epipolar geometry dropped", "Bilder {0} und {1}: {2} Zuordnungen ohne epipolare Konsistenz verworfen"),
            Self::SeedPairFailed => LocalizedLabel::native("No adjacent frames share enough consistent matches to solve a first camera pair; the run registers no cameras", "Keine benachbarten Bilder teilen genug konsistente Zuordnungen für ein erstes Kamerapaar; der Lauf registriert keine Kameras"),
        }
    }
}

/// ⏯️ The reconstruction run declaration. `mutating`: its result is document content. `rebase: revalidate`:
/// an unrelated concurrent edit keeps the provisional result, and finalize re-checks that frames, parameters
/// and calibration still equal the run's inputs. `settings` reads nothing: every run input is document
/// state, so no config or window-config publication reconfigures it. `reconfigure: resume` therefore only
/// governs a rebuilt job, which replays deterministically to its last checkpoint without re-emitting anything.
pub fn reconstruction_run_definition() -> ToolRunDefinition {
    ToolRunDefinition {
        mutating: true,
        rebase: ToolRunRebasePolicy::Revalidate,
        reconfigure: ToolRunReconfigurePolicy::Resume,
        unit: LocalizedLabel::native("decisions", "Entscheidungen"),
        stages: ReconstructionRunStage::ALL.iter().map(|stage| ToolRunStageDefinition { id: stage.id().into(), label: stage.label() }).collect(),
        counters: ReconstructionRunCounter::ALL.iter().map(|counter| ToolRunCounterDefinition { id: counter.id().into(), label: counter.label() }).collect(),
        reasons: ReconstructionRunReason::ALL.iter().map(|reason| ToolRunReasonDefinition { code: reason.code(), id: reason.id().into(), verdict: reason.verdict(), template: reason.template() }).collect(),
        trace: ToolRunTraceKind::Instance3d,
        run_job: JobKindId::new(RECONSTRUCTION_RUN_JOB_KIND),
        revalidate_job: Some(JobKindId::new(RECONSTRUCTION_REVALIDATE_JOB_KIND)),
        settings: ToolRunSettingsReads::default(),
        windows: Vec::new(),
    }
}
//#endregion 🔖️Contract

//#region 🔑️TraceKeys
const KEY_FRAME: u64 = 1 << 56;
const KEY_FEATURES: u64 = 2 << 56;
const KEY_MATCH: u64 = 3 << 56;
const KEY_CAMERA: u64 = 4 << 56;
const KEY_POINT: u64 = 5 << 56;
const KEY_DENSE: u64 = 6 << 56;

/// 🔑️ The trace key of one frame verdict.
pub fn frame_trace_key(frame: u64) -> u64 {
    KEY_FRAME | frame
}

/// 🔑️ The trace key of one match candidate: the query keypoint of `frame_a` against `frame_b`.
pub fn match_trace_key(frame_a: usize, frame_b: usize, query: u32) -> u64 {
    KEY_MATCH | ((frame_a as u64) << 36) | ((frame_b as u64) << 18) | u64::from(query)
}

/// 🔑️ The trace key of one registered or rejected camera.
pub fn camera_trace_key(frame: usize) -> u64 {
    KEY_CAMERA | frame as u64
}

/// 🔑️ The trace key of one sparse point, stable across triangulation and pruning.
pub fn point_trace_key(track: usize) -> u64 {
    KEY_POINT | track as u64
}

fn point_subject(point: [f64; 3]) -> ToolRunTraceSubject {
    ToolRunTraceSubject::Instance3d { mesh: RECONSTRUCTION_TRACE_POINT_MESH, position: point.map(|coordinate| coordinate as f32), rotation: [0.0, 0.0, 0.0, 1.0], scale: RECONSTRUCTION_TRACE_POINT_SCALE }
}
//#endregion 🔑️TraceKeys

//#region 🔖️Checkpoint
/// 📍️ Where a run stands: the base it started from, the digest of its inputs, the decisions taken and the
/// provisional ops it holds. A rebuilt job replays silently to `decisions` when its inputs still match.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReconstructionRunCheckpoint {
    pub base_revision: [u8; 32],
    pub inputs: RemodelingContentDigest,
    pub decisions: u64,
    pub provisional_ops: u32,
}

impl ReconstructionRunCheckpoint {
    pub const BYTES: usize = 84;

    pub fn encode(self) -> [u8; Self::BYTES] {
        let mut bytes = [0u8; Self::BYTES];
        bytes[..32].copy_from_slice(&self.base_revision);
        let (words, length) = self.inputs.parts();
        for (index, word) in words.iter().enumerate() {
            bytes[32 + index * 8..40 + index * 8].copy_from_slice(&word.to_le_bytes());
        }
        bytes[64..72].copy_from_slice(&length.to_le_bytes());
        bytes[72..80].copy_from_slice(&self.decisions.to_le_bytes());
        bytes[80..84].copy_from_slice(&self.provisional_ops.to_le_bytes());
        bytes
    }

    pub fn decode(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != Self::BYTES {
            return None;
        }
        let word = |at: usize| u64::from_le_bytes(bytes[at..at + 8].try_into().expect("eight checkpoint bytes"));
        Some(Self {
            base_revision: bytes[..32].try_into().ok()?,
            inputs: RemodelingContentDigest::from_parts([word(32), word(40), word(48), word(56)], word(64)),
            decisions: word(72),
            provisional_ops: u32::from_le_bytes(bytes[80..84].try_into().ok()?),
        })
    }
}

/// #️⃣️ The digest of everything a reconstruction reads: frame references and the content ids of their
/// assets, reconstruction parameters and calibration. GCPs, results and view state are not inputs.
pub fn reconstruction_inputs(snapshot: &RemodelingSnapshot) -> RemodelingContentDigest {
    let mut digest = RemodelingContentDigest::default();
    for stream in &snapshot.streams {
        digest.record(stream.id.as_bytes());
        for frame in &stream.frames {
            digest.record(&u64::from(frame.index).to_le_bytes());
            digest.record(&frame.timestamp_ms.to_le_bytes());
            digest.record(snapshot.assets.get(&frame.asset_id).map_or(frame.asset_id.as_bytes(), |handle| handle.child_id.as_bytes()));
        }
    }
    digest.record(dsl::json::to_json_string(&dsl::ToValue::to_value(&snapshot.params)).as_bytes());
    digest.record(dsl::json::to_json_string(&dsl::ToValue::to_value(&snapshot.calibration)).as_bytes());
    digest
}
//#endregion 🔖️Checkpoint

//#region 🔖️Ingestion
struct FrameIngestion {
    frame: FrameRef,
    decoder: Option<BoundedStillDecoder>,
    image: Option<ImageRgba8>,
    sharpness_cursor: usize,
    sharpness_sum: f64,
}

fn frame_ingestion(scene: &RemodelingSnapshot, frame: &FrameRef) -> Option<FrameIngestion> {
    let source = crate::remodeling_asset_chunk_source(scene, &frame.asset_id)?;
    let compressed = CompressedChunkRope::from_leaves(source.leaves, MAX_STILL_INPUT_BYTES).ok()?;
    Some(FrameIngestion { frame: frame.clone(), decoder: Some(BoundedStillDecoder::new(&source.mime, compressed)), image: None, sharpness_cursor: 0, sharpness_sum: 0.0 })
}

fn rgba_luma(image: &ImageRgba8, index: usize) -> f64 {
    let offset = index * 4;
    (0.299 * f64::from(image.data[offset]) + 0.587 * f64::from(image.data[offset + 1]) + 0.114 * f64::from(image.data[offset + 2])) / 255.0
}

fn mirror(coordinate: i64, length: usize) -> usize {
    if length <= 1 {
        return 0;
    }
    let period = 2 * length as i64 - 2;
    let wrapped = coordinate.rem_euclid(period);
    if wrapped < length as i64 {
        wrapped as usize
    } else {
        (period - wrapped) as usize
    }
}

/// 🔍️ Folds one bounded window of Scharr gradient energy; `true` once every pixel was scored.
fn advance_frame_sharpness(ingestion: &mut FrameIngestion) -> bool {
    let Some(image) = ingestion.image.as_ref() else { return true };
    let pixels = image.width as usize * image.height as usize;
    let end = ingestion.sharpness_cursor.saturating_add(SHARPNESS_PIXELS_PER_UNIT).min(pixels);
    let width = image.width as usize;
    for index in ingestion.sharpness_cursor..end {
        let (x, y) = (index % width, index / width);
        let sample = |dx: i64, dy: i64| rgba_luma(image, mirror(y as i64 + dy, image.height as usize) * width + mirror(x as i64 + dx, width));
        let gx = (3.0 * (sample(1, -1) - sample(-1, -1)) + 10.0 * (sample(1, 0) - sample(-1, 0)) + 3.0 * (sample(1, 1) - sample(-1, 1))) / 32.0;
        let gy = (3.0 * (sample(-1, 1) - sample(-1, -1)) + 10.0 * (sample(0, 1) - sample(0, -1)) + 3.0 * (sample(1, 1) - sample(1, -1))) / 32.0;
        ingestion.sharpness_sum += gx * gx + gy * gy;
    }
    ingestion.sharpness_cursor = end;
    end == pixels
}
//#endregion 🔖️Ingestion

//#region 🔖️Products
/// 🧱️ Raw leaves of one durable content entry under construction, content-addressed once complete.
struct ContentLeaves {
    kind: RemodelingContentKind,
    leaves: Vec<Vec<u8>>,
    digest: RemodelingContentDigest,
    bytes: usize,
}

impl ContentLeaves {
    fn new(kind: RemodelingContentKind) -> Self {
        Self { kind, leaves: Vec::new(), digest: RemodelingContentDigest::default(), bytes: 0 }
    }

    fn push(&mut self, leaf: Vec<u8>) {
        self.digest.record(&leaf);
        self.bytes += leaf.len();
        self.leaves.push(leaf);
    }

    /// 📦️ One `append-content` op per leaf (a leaf is the largest unit one tick page carries), and the id.
    fn into_ops(self, mime: Option<String>, width: u32, height: u32) -> (String, u64, Vec<RemodelingMutation>) {
        let content_id = self.digest.content_id(self.kind);
        let count = self.leaves.len() as u64;
        let ops = self
            .leaves
            .into_iter()
            .enumerate()
            .map(|(index, leaf)| append_content(AppendContent { content_id: content_id.clone(), kind: self.kind, mime: mime.clone(), width, height, first: index as u64, chunks: vec![base64_codec::base64_standard_encode(leaf)] }))
            .collect();
        (content_id, count, ops)
    }
}

/// 🧊️ Field-tagged mesh leaves: `[field, values…]` per ≤ 4 KiB leaf, fields in ascending order.
fn mesh_leaves(mesh: &semio_framework::MeshData) -> ContentLeaves {
    let mut content = ContentLeaves::new(RemodelingContentKind::Mesh);
    let value_bytes = (REMODELING_DURABLE_CHUNK_RAW_BYTES - 1) / 4 * 4;
    let fields: [(u8, Vec<u8>); 12] = [
        (0, mesh.positions.iter().flat_map(|value| value.to_le_bytes()).collect()),
        (1, mesh.normals.iter().flat_map(|value| value.to_le_bytes()).collect()),
        (2, mesh.colors.iter().flat_map(|value| value.to_le_bytes()).collect()),
        (3, mesh.indices.iter().flat_map(|value| value.to_le_bytes()).collect()),
        (4, mesh.uvs.iter().flat_map(|value| value.to_le_bytes()).collect()),
        (5, mesh.face_ids.iter().flat_map(|value| value.to_le_bytes()).collect()),
        (6, mesh.vertex_ids.iter().flat_map(|value| value.to_le_bytes()).collect()),
        (7, mesh.edge_positions.iter().flat_map(|value| value.to_le_bytes()).collect()),
        (8, mesh.edge_ids.iter().flat_map(|value| value.to_le_bytes()).collect()),
        (9, mesh.edge_uvs.iter().flat_map(|value| value.to_le_bytes()).collect()),
        (10, mesh.edge_is_seam.clone()),
        (11, mesh.paint_texture_base64.clone().unwrap_or_default().into_bytes()),
    ];
    for (field, bytes) in fields {
        let step = if field >= 10 { REMODELING_DURABLE_CHUNK_RAW_BYTES - 1 } else { value_bytes };
        for window in bytes.chunks(step) {
            let mut leaf = Vec::with_capacity(window.len() + 1);
            leaf.push(field);
            leaf.extend_from_slice(window);
            content.push(leaf);
        }
    }
    content
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ProductPhase {
    Sparse,
    Quality,
    Mesh,
    Geo,
    Dsm,
    Dtm,
    Commit,
    Done,
}

/// 🏁️ What one bounded product unit yielded.
enum ProductYield {
    Working,
    Op(RemodelingMutation),
    Step(ReconstructionRunReason, Vec<u64>),
    Failed(ReconstructionRunReason),
    Done,
}

/// 🏭️ The product preparation after the engine finished: sparse leaves, QC, mesh leaves, DSM/DTM rasters,
/// then the one `commit-reconstruction` op.
struct ProductPreparation {
    phase: ProductPhase,
    camera_cursor: usize,
    point_cursor: usize,
    points_total: usize,
    quality_cursor: usize,
    quality_squared_error_sum: f64,
    quality_observations: usize,
    quality_points: BTreeSet<usize>,
    trajectory: Vec<CameraPosePreview>,
    sparse_leaves: ContentLeaves,
    sparse: Option<SparseCloud>,
    watertight: Option<WatertightReportSnapshot>,
    qc: Option<QcReportSnapshot>,
    mesh: Option<Box<RemodelingMesh>>,
    geo_preparation: Option<TerminalGeoPreparation>,
    pending_dtm: Option<remodeling_geo::Raster>,
    raster: Option<(RasterPngPreparation, ContentLeaves, &'static str)>,
    assets: Vec<ReconstructionAssetCommit>,
    geo: Option<GeoProducts>,
    pending: VecDeque<ProductYield>,
    gcp_count: usize,
}

impl ProductPreparation {
    fn new(gcp_count: usize) -> Self {
        Self {
            phase: ProductPhase::Sparse,
            camera_cursor: 0,
            point_cursor: 0,
            points_total: 0,
            quality_cursor: 0,
            quality_squared_error_sum: 0.0,
            quality_observations: 0,
            quality_points: BTreeSet::new(),
            trajectory: Vec::new(),
            sparse_leaves: ContentLeaves::new(RemodelingContentKind::Sparse),
            sparse: None,
            watertight: None,
            qc: None,
            mesh: None,
            geo_preparation: None,
            pending_dtm: None,
            raster: None,
            assets: Vec::new(),
            geo: None,
            pending: VecDeque::new(),
            gcp_count,
        }
    }

    fn queue_content(&mut self, content: ContentLeaves, mime: Option<String>, width: u32, height: u32) -> (String, u64) {
        let (leaves, bytes) = (content.leaves.len() as u64, content.bytes as u64);
        let (content_id, count, ops) = content.into_ops(mime, width, height);
        self.pending.push_back(ProductYield::Step(ReconstructionRunReason::ContentPrepared, vec![leaves, bytes]));
        self.pending.extend(ops.into_iter().map(ProductYield::Op));
        (content_id, count)
    }

    fn advance(&mut self, engine: &mut ReconstructionEngine) -> ProductYield {
        if let Some(pending) = self.pending.pop_front() {
            return pending;
        }
        match self.phase {
            ProductPhase::Sparse => {
                let chunk = engine.terminal_sparse_chunk(self.camera_cursor, self.point_cursor, TERMINAL_CAMERA_WORK, TERMINAL_POINT_WORK);
                for (offset, pose) in chunk.camera_poses.iter().enumerate() {
                    self.trajectory.push(camera_pose_preview((self.camera_cursor + offset) as u32, pose));
                }
                self.points_total += chunk.packed_points.len() / 3;
                let room = (RemodelingContentKind::Sparse.max_bytes() - self.sparse_leaves.bytes) / 12 * 3;
                let kept = &chunk.packed_points[..chunk.packed_points.len().min(room)];
                if !kept.is_empty() {
                    self.sparse_leaves.push(kept.iter().flat_map(|value| value.to_le_bytes()).collect());
                }
                self.camera_cursor = chunk.next_camera;
                self.point_cursor = chunk.next_point;
                if chunk.complete {
                    self.phase = ProductPhase::Quality;
                    let kept_points = self.sparse_leaves.bytes / 12;
                    if kept_points < self.points_total {
                        self.pending.push_back(ProductYield::Step(ReconstructionRunReason::SparseCapped, vec![kept_points as u64, self.points_total as u64]));
                    }
                    let content = std::mem::replace(&mut self.sparse_leaves, ContentLeaves::new(RemodelingContentKind::Sparse));
                    if !content.leaves.is_empty() {
                        let (content_id, count) = self.queue_content(content, None, 0, 0);
                        self.sparse = Some(SparseCloud { points: PackedF32(remodeling_content_handle(&content_id, count)), colors: None });
                    }
                }
                ProductYield::Working
            }
            ProductPhase::Quality => {
                let chunk = engine.terminal_quality_chunk(self.quality_cursor, TERMINAL_QUALITY_WORK);
                self.quality_cursor = chunk.next_observation;
                self.quality_squared_error_sum += chunk.squared_error_sum;
                self.quality_observations += chunk.observation_count;
                self.quality_points.extend(chunk.point_indices);
                if chunk.complete {
                    self.watertight = engine.terminal_watertight_report().as_ref().map(watertight_snapshot);
                    let accepted = engine.frame_source().accepted_count();
                    let mut warnings = Vec::new();
                    if self.watertight.as_ref().is_some_and(|report| !report.is_watertight) {
                        warnings.push("Mesh is not watertight.".into());
                    }
                    if self.gcp_count > 0 {
                        warnings.push("Ground control points are set but checkpoint RMSE is not yet computed.".into());
                    }
                    self.qc = Some(QcReportSnapshot {
                        reprojection_rms_px: if self.quality_observations == 0 { 0.0 } else { (self.quality_squared_error_sum / self.quality_observations as f64).sqrt() },
                        gcp_checkpoint_rmse: None,
                        watertight: self.watertight.clone(),
                        mean_track_length: if self.quality_points.is_empty() { 0.0 } else { self.quality_observations as f32 / self.quality_points.len() as f32 },
                        registered_frame_ratio: if accepted == 0 { 0.0 } else { self.trajectory.len() as f32 / accepted as f32 },
                        dense_coverage_ratio: 0.0,
                        warnings,
                    });
                    self.phase = ProductPhase::Mesh;
                }
                ProductYield::Working
            }
            ProductPhase::Mesh => {
                self.phase = ProductPhase::Geo;
                let Some(mesh) = engine.take_mesh().filter(|mesh| !mesh.indices.is_empty()) else { return ProductYield::Working };
                let (vertices, triangles) = ((mesh.positions.len() / 3) as u64, (mesh.indices.len() / 3) as u64);
                if !mesh_is_within_resolution_envelope(&mesh) {
                    return ProductYield::Step(ReconstructionRunReason::MeshOutsideEnvelope, vec![vertices, triangles]);
                }
                let (content_id, count) = self.queue_content(mesh_leaves(&mesh), None, 0, 0);
                self.mesh = Some(Box::new(RemodelingMesh { mesh: remodeling_mesh_content_handle(&content_id, count), source: MeshSource::Reconstructed, texture_asset_id: None, watertight: self.watertight.clone() }));
                ProductYield::Working
            }
            ProductPhase::Geo => {
                match self.geo_preparation.as_mut() {
                    None => {
                        self.geo_preparation = engine.begin_terminal_geo();
                        if self.geo_preparation.is_none() {
                            self.phase = ProductPhase::Commit;
                        }
                    }
                    Some(preparation) => {
                        if engine.advance_terminal_geo(preparation, TERMINAL_GEO_WORK) {
                            let products = self.geo_preparation.take().and_then(ReconstructionEngine::finish_terminal_geo);
                            match products {
                                Some(products) => {
                                    self.pending_dtm = Some(products.dtm);
                                    self.raster = Some((RasterPngPreparation::new(products.dsm), ContentLeaves::new(RemodelingContentKind::Image), REMODELING_RECONSTRUCTION_DSM_ASSET_ID));
                                    self.phase = ProductPhase::Dsm;
                                }
                                None => self.phase = ProductPhase::Commit,
                            }
                        }
                    }
                }
                ProductYield::Working
            }
            ProductPhase::Dsm | ProductPhase::Dtm => {
                let Some((encoder, content, _)) = self.raster.as_mut() else { return ProductYield::Failed(ReconstructionRunReason::RasterFailed) };
                match encoder.advance(RASTER_CELL_WORK) {
                    RasterPngProgress::Working => ProductYield::Working,
                    RasterPngProgress::Chunk(bytes) => {
                        content.push(bytes);
                        ProductYield::Working
                    }
                    RasterPngProgress::Failed => ProductYield::Failed(ReconstructionRunReason::RasterFailed),
                    RasterPngProgress::Complete => {
                        let (encoder, content, asset_id) = self.raster.take().expect("completed raster");
                        let (content_id, _) = self.queue_content(content, Some("image/png".into()), encoder.width(), encoder.height());
                        self.assets.push(ReconstructionAssetCommit { id: asset_id.into(), content_id: Some(content_id) });
                        if self.phase == ProductPhase::Dsm {
                            let dtm = self.pending_dtm.take().expect("DTM retained through DSM encoding");
                            self.raster = Some((RasterPngPreparation::new(dtm), ContentLeaves::new(RemodelingContentKind::Image), REMODELING_RECONSTRUCTION_DTM_ASSET_ID));
                            self.phase = ProductPhase::Dtm;
                        } else {
                            self.geo = Some(GeoProducts { dsm_asset_id: Some(REMODELING_RECONSTRUCTION_DSM_ASSET_ID.into()), dtm_asset_id: Some(REMODELING_RECONSTRUCTION_DTM_ASSET_ID.into()), ortho_asset_id: None });
                            self.phase = ProductPhase::Commit;
                        }
                        ProductYield::Working
                    }
                }
            }
            ProductPhase::Commit => {
                self.phase = ProductPhase::Done;
                let trajectory = (!self.trajectory.is_empty()).then(|| CameraTrajectory { poses: std::mem::take(&mut self.trajectory) });
                if self.sparse.is_none() && trajectory.is_none() && self.mesh.is_none() {
                    // 🌱️ A run that registered no camera has nothing to publish: no edit, no
                    // placeholder replaced, the trace explains why (`seedPairFailed`).
                    return ProductYield::Done;
                }
                ProductYield::Op(commit_reconstruction(CommitReconstruction { sparse: self.sparse.take(), trajectory, mesh: self.mesh.take(), geo: self.geo.take(), qc: self.qc.take(), assets: std::mem::take(&mut self.assets) }))
            }
            ProductPhase::Done => ProductYield::Done,
        }
    }
}
//#endregion 🔖️Products

//#region 🧵️RunJob
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Owed {
    Checkpoint,
    Complete,
    Fault,
}

enum RunPhase {
    Ingest { stream: usize, frame: usize, ingestion: Option<FrameIngestion> },
    Pipeline,
    DenseTrace { cursor: usize },
    Products(Box<ProductPreparation>),
    Settled,
}

/// ⏯️ The reconstruction `runJob`. One unit of fuel is one decision that produced visible evidence; engine
/// slices without evidence are transitions bounded only by the deadline. A rebuilt job whose checkpoint
/// matches its inputs replays silently to the checkpoint's decision count.
pub struct ReconstructionRunJob {
    identity: ToolRunIdentity,
    writer: ToolRunTickWriter,
    scene: Option<Arc<RemodelingSnapshot>>,
    inputs: RemodelingContentDigest,
    engine: Option<Box<ReconstructionEngine>>,
    phase: RunPhase,
    observations: Vec<EngineObservation>,
    counters: [u64; 8],
    frames_total: u64,
    frames_decided: u64,
    gcp_count: usize,
    decisions: u64,
    replay_to: u64,
    ops_emitted: u32,
    checkpoint_at: u64,
    stage: ReconstructionRunStage,
    owed: VecDeque<Owed>,
    closing: bool,
}

impl ReconstructionRunJob {
    /// 🌱️ A run over `scene`; `checkpoint` resumes it when its inputs still match, `provisional` is the
    /// ledger's provisional op count the writer continues from.
    pub fn new(identity: ToolRunIdentity, scene: Arc<RemodelingSnapshot>, checkpoint: Option<&[u8]>, provisional: u32) -> Self {
        let inputs = reconstruction_inputs(&scene);
        let mut engine = Box::new(ReconstructionEngine::new(&build_engine_params(&scene.params, &scene.calibration)));
        engine.observe();
        let mut writer = ToolRunTickWriter::with_provisional_base(identity, provisional);
        let resume = checkpoint.and_then(ReconstructionRunCheckpoint::decode).filter(|checkpoint| checkpoint.inputs == inputs && checkpoint.provisional_ops <= provisional);
        match resume {
            Some(checkpoint) => writer.retract_to(checkpoint.provisional_ops),
            None if checkpoint.is_some() || provisional > 0 => {
                writer.clear_trace();
                writer.retract_to(0);
            }
            None => {}
        }
        let frames_total = scene.streams.iter().map(|stream| stream.frames.len() as u64).sum();
        let gcp_count = scene.gcps.len();
        Self {
            identity,
            writer,
            scene: Some(scene),
            inputs,
            engine: Some(engine),
            phase: RunPhase::Ingest { stream: 0, frame: 0, ingestion: None },
            observations: Vec::new(),
            counters: [0; 8],
            frames_total,
            frames_decided: 0,
            gcp_count,
            decisions: 0,
            replay_to: resume.map_or(0, |checkpoint| checkpoint.decisions),
            ops_emitted: 0,
            checkpoint_at: 0,
            stage: ReconstructionRunStage::Ingest,
            owed: VecDeque::from(if resume.is_some() { Vec::new() } else { vec![Owed::Checkpoint] }),
            closing: false,
        }
    }

    /// 📍️ The checkpoint this job reports now.
    pub fn checkpoint(&self) -> ReconstructionRunCheckpoint {
        ReconstructionRunCheckpoint { base_revision: self.identity.base_revision, inputs: self.inputs, decisions: self.decisions, provisional_ops: self.ops_emitted }
    }

    /// 🔢️ Counter values in [`ReconstructionRunCounter::ALL`] order.
    pub fn counters(&self) -> [u64; 8] {
        self.counters
    }

    fn live(&self) -> bool {
        self.decisions >= self.replay_to
    }

    fn count(&mut self, counter: ReconstructionRunCounter, delta: i64) {
        let slot = &mut self.counters[usize::from(counter.index())];
        *slot = slot.saturating_add_signed(delta);
    }

    fn step(&mut self, reason: ReconstructionRunReason, args: &[u64]) {
        if self.live() {
            let args: Vec<ToolRunStepArg> = args.iter().map(|value| ToolRunStepArg::Unsigned(*value)).collect();
            let _ = self.writer.step(reason.step_kind(), self.stage.index(), reason.code(), None, &args);
        }
    }

    fn upsert(&mut self, key: u64, reason: ReconstructionRunReason, subject: ToolRunTraceSubject) {
        if self.live() {
            self.writer.upsert(key, reason.verdict(), reason.code(), subject);
        }
    }

    fn append(&mut self, op: &RemodelingMutation) -> Result<(), &'static str> {
        self.ops_emitted += 1;
        if self.ops_emitted > self.writer.provisional_len() {
            let bytes = protocol::OpBinary::encode_op(op).map_err(|_| "remodeling.reconstruction.op-encode")?;
            self.writer.append_op(bytes).map_err(|_| "remodeling.reconstruction.provisional-cap")?;
        }
        Ok(())
    }

    /// 🎯️ Completes one decision: consumes one unit of fuel once live, and owes a checkpoint every
    /// `CHECKPOINT_DECISIONS` decisions.
    fn decide(&mut self, cx: &mut StepContext<'_>) {
        let was_live = self.live();
        self.decisions += 1;
        if was_live {
            cx.consume_fuel(1);
            if self.decisions - self.checkpoint_at >= CHECKPOINT_DECISIONS {
                self.checkpoint_at = self.decisions;
                self.owed.push_back(Owed::Checkpoint);
            }
        } else if self.live() {
            self.step(ReconstructionRunReason::Resuming, &[self.decisions]);
        }
    }

    fn stage_progress(&self) -> (u64, Option<u64>) {
        match &self.phase {
            RunPhase::Ingest { .. } => (self.frames_decided, Some(self.frames_total)),
            RunPhase::Pipeline => self.engine.as_ref().map_or((0, None), |engine| engine.stage_progress()),
            RunPhase::DenseTrace { cursor } => (*cursor as u64, self.engine.as_ref().map(|engine| engine.dense_positions().len().min(RECONSTRUCTION_DENSE_TRACE_MAX) as u64)),
            RunPhase::Products(_) | RunPhase::Settled => (u64::from(self.ops_emitted), None),
        }
    }

    fn progress(&self, state: ToolRunState) -> ToolRunProgress {
        let (completed, total) = self.stage_progress();
        ToolRunProgress {
            identity: self.identity,
            sequence: 0,
            state,
            stage: self.stage.index(),
            completed: total.map_or(completed, |total| completed.min(total)),
            total,
            counters: ReconstructionRunCounter::ALL.iter().zip(self.counters).map(|(counter, value)| ToolRunCounter { counter: counter.index(), value }).collect(),
            units_per_second: 0.0,
            conflicts: 0,
            steps: ToolRunStepRing::new(),
        }
    }

    /// 🔭️ Turns every engine observation of the last slice into trace records, steps and counters;
    /// `true` when the slice produced visible evidence.
    fn observe(&mut self) -> bool {
        let mut observations = std::mem::take(&mut self.observations);
        if let Some(engine) = self.engine.as_mut() {
            engine.drain_observations(&mut observations);
        }
        let evidence = !observations.is_empty();
        let (mut triangulated, mut pruned) = (0u64, 0u64);
        for observation in observations.drain(..) {
            match observation {
                EngineObservation::FeaturesExtracted { frame, keypoints } => {
                    self.count(ReconstructionRunCounter::Keypoints, keypoints as i64);
                    self.upsert(KEY_FEATURES | frame as u64, ReconstructionRunReason::FeaturesExtracted, ToolRunTraceSubject::Entity { entity: frame as u64 });
                    self.step(ReconstructionRunReason::FeaturesExtracted, &[frame as u64, keypoints as u64]);
                }
                EngineObservation::MatchAccepted { frame_a, frame_b, query, .. } => {
                    self.count(ReconstructionRunCounter::MatchesAccepted, 1);
                    self.upsert(match_trace_key(frame_a, frame_b, query), ReconstructionRunReason::MatchAccepted, ToolRunTraceSubject::Entity { entity: match_trace_key(frame_a, frame_b, query) & !KEY_MATCH });
                }
                EngineObservation::MatchRatioRejected { frame_a, frame_b, query, .. } => {
                    self.count(ReconstructionRunCounter::MatchesRejected, 1);
                    self.upsert(match_trace_key(frame_a, frame_b, query), ReconstructionRunReason::MatchAmbiguous, ToolRunTraceSubject::Entity { entity: match_trace_key(frame_a, frame_b, query) & !KEY_MATCH });
                }
                EngineObservation::MatchCrossCheckRejected { frame_a, frame_b, query, .. } => {
                    self.count(ReconstructionRunCounter::MatchesRejected, 1);
                    self.upsert(match_trace_key(frame_a, frame_b, query), ReconstructionRunReason::MatchAsymmetric, ToolRunTraceSubject::Entity { entity: match_trace_key(frame_a, frame_b, query) & !KEY_MATCH });
                }
                EngineObservation::PairMatched { frame_a, frame_b, matches } => self.step(ReconstructionRunReason::PairMatched, &[frame_a as u64, frame_b as u64, matches as u64]),
                EngineObservation::PairGeometryRejected { frame_a, frame_b, dropped } => {
                    self.count(ReconstructionRunCounter::MatchesRejected, dropped as i64);
                    self.step(ReconstructionRunReason::PairGeometryRejected, &[frame_a as u64, frame_b as u64, dropped as u64]);
                }
                EngineObservation::TracksBuilt { tracks } => self.step(ReconstructionRunReason::TracksBuilt, &[tracks as u64]),
                EngineObservation::CameraRegistered { frame, pose } => {
                    self.count(ReconstructionRunCounter::Cameras, 1);
                    let preview = camera_pose_preview(frame as u32, &pose);
                    let [w, x, y, z] = preview.rotation_wxyz;
                    self.upsert(camera_trace_key(frame), ReconstructionRunReason::CameraRegistered, ToolRunTraceSubject::Instance3d { mesh: RECONSTRUCTION_TRACE_CAMERA_MESH, position: preview.translation, rotation: [x, y, z, w], scale: RECONSTRUCTION_TRACE_CAMERA_SCALE });
                    self.step(ReconstructionRunReason::CameraRegistered, &[frame as u64]);
                }
                EngineObservation::CameraRejected { frame } => {
                    self.upsert(camera_trace_key(frame), ReconstructionRunReason::CameraRejected, ToolRunTraceSubject::Entity { entity: frame as u64 });
                    self.step(ReconstructionRunReason::CameraRejected, &[frame as u64]);
                }
                EngineObservation::SeedPairFailed { .. } => self.step(ReconstructionRunReason::SeedPairFailed, &[]),
                EngineObservation::PointTriangulated { track, point } => {
                    triangulated += 1;
                    self.count(ReconstructionRunCounter::SparsePoints, 1);
                    self.upsert(point_trace_key(track), ReconstructionRunReason::PointsTriangulated, point_subject(point));
                }
                EngineObservation::PointPruned { track, point } => {
                    pruned += 1;
                    self.count(ReconstructionRunCounter::SparsePoints, -1);
                    self.upsert(point_trace_key(track), ReconstructionRunReason::PointsPruned, point_subject(point));
                }
                EngineObservation::DepthMapEstimated { view, samples } => self.step(ReconstructionRunReason::DepthMapEstimated, &[view as u64, samples as u64]),
                EngineObservation::DenseCloudFused { points } => {
                    self.step(ReconstructionRunReason::DenseCloudFused, &[points as u64]);
                    if points > RECONSTRUCTION_DENSE_TRACE_MAX {
                        self.step(ReconstructionRunReason::DenseTraceCapped, &[RECONSTRUCTION_DENSE_TRACE_MAX as u64, points as u64]);
                    }
                }
                EngineObservation::MeshExtracted { vertices, triangles } => self.step(ReconstructionRunReason::MeshExtracted, &[vertices as u64, triangles as u64]),
            }
        }
        self.observations = observations;
        if triangulated > 0 {
            self.step(ReconstructionRunReason::PointsTriangulated, &[triangulated]);
        }
        if pruned > 0 {
            self.step(ReconstructionRunReason::PointsPruned, &[pruned]);
        }
        evidence
    }

    /// 📥️ One bounded ingestion slice; `Some(reason)` once a frame verdict was taken.
    fn ingest(&mut self) -> Result<Option<(u64, ReconstructionRunReason)>, ()> {
        let RunPhase::Ingest { stream, frame, ingestion } = &mut self.phase else { return Err(()) };
        let scene = self.scene.as_ref().ok_or(())?;
        let Some(current) = ingestion.as_mut() else {
            let Some(stream_ref) = scene.streams.get(*stream) else {
                self.scene = None;
                self.phase = RunPhase::Pipeline;
                return Ok(None);
            };
            let Some(frame_ref) = stream_ref.frames.get(*frame) else {
                *stream += 1;
                *frame = 0;
                return Ok(None);
            };
            let index = u64::from(frame_ref.index);
            *frame += 1;
            return Ok(match frame_ingestion(scene, frame_ref) {
                Some(next) => {
                    *ingestion = Some(next);
                    None
                }
                None => Some((index, ReconstructionRunReason::FrameUnreadable)),
            });
        };
        let index = u64::from(current.frame.index);
        if let Some(decoder) = current.decoder.as_mut() {
            match decoder.advance() {
                BoundedDecodeProgress::Working => return Ok(None),
                BoundedDecodeProgress::Complete(image) => {
                    current.decoder = None;
                    current.image = Some(image);
                    return Ok(None);
                }
                BoundedDecodeProgress::Failed(_) => {
                    *ingestion = None;
                    return Ok(Some((index, ReconstructionRunReason::FrameUnreadable)));
                }
            }
        }
        if !advance_frame_sharpness(current) {
            return Ok(None);
        }
        let done = ingestion.take().expect("scored frame");
        let image = done.image.expect("decoded frame");
        let pixels = image.width as usize * image.height as usize;
        let sharpness = if pixels == 0 { 0.0 } else { (done.sharpness_sum / pixels as f64) as f32 };
        let engine = self.engine.as_mut().ok_or(())?;
        Ok(Some((
            index,
            match engine.push_frame_with_sharpness(done.frame.index, image, done.frame.timestamp_ms, sharpness) {
                FrameAcceptance::Accepted => ReconstructionRunReason::FrameAccepted,
                FrameAcceptance::RejectedStride | FrameAcceptance::RejectedMaxFrames => ReconstructionRunReason::FrameSkipped,
                FrameAcceptance::RejectedBlur => ReconstructionRunReason::FrameBlurred,
            },
        )))
    }

    fn fault(&mut self, reason: ReconstructionRunReason, args: &[u64]) {
        self.step(reason, args);
        self.phase = RunPhase::Settled;
        self.owed.push_back(Owed::Fault);
    }

    /// 🦶️ One bounded unit of the current phase; `true` when it completed a decision.
    fn advance(&mut self) -> Result<bool, &'static str> {
        match self.phase {
            RunPhase::Ingest { .. } => self.advance_ingest(),
            RunPhase::Pipeline => self.advance_pipeline(),
            RunPhase::DenseTrace { cursor } => self.advance_dense(cursor),
            RunPhase::Products(_) => self.advance_products(),
            RunPhase::Settled => Ok(false),
        }
    }

    fn advance_ingest(&mut self) -> Result<bool, &'static str> {
        match self.ingest() {
            Ok(Some((frame, reason))) => {
                self.frames_decided += 1;
                let counter = if reason == ReconstructionRunReason::FrameAccepted { ReconstructionRunCounter::FramesAccepted } else { ReconstructionRunCounter::FramesRejected };
                self.count(counter, 1);
                self.upsert(frame_trace_key(frame), reason, ToolRunTraceSubject::Entity { entity: frame });
                self.step(reason, &[frame]);
                Ok(true)
            }
            Ok(None) => Ok(false),
            Err(()) => Err("remodeling.reconstruction.ingest"),
        }
    }

    fn advance_pipeline(&mut self) -> Result<bool, &'static str> {
        let engine = self.engine.as_mut().ok_or("remodeling.reconstruction.engine")?;
        let status = engine.advance(1);
        if engine.stage() != EngineStage::Failed {
            self.stage = ReconstructionRunStage::of(engine.stage());
        }
        let evidence = self.observe();
        match status {
            EngineStatus::Working { .. } => Ok(evidence),
            EngineStatus::Done => {
                // 📈️ The dense trace replays the fused cloud AFTER meshing finished, so naming its
                // own stage would walk the reported stage backwards (Surface -> Fusion); the run's
                // visible stage only ever moves forward.
                self.stage = self.stage.max(ReconstructionRunStage::Fusion);
                self.phase = RunPhase::DenseTrace { cursor: 0 };
                Ok(evidence)
            }
            EngineStatus::Failed(_) => {
                let accepted = self.counters[usize::from(ReconstructionRunCounter::FramesAccepted.index())];
                if accepted < 2 {
                    self.fault(ReconstructionRunReason::TooFewFrames, &[accepted]);
                } else {
                    self.fault(ReconstructionRunReason::PipelineFailed, &[u64::from(self.stage.index()) + 1]);
                }
                Ok(true)
            }
        }
    }

    fn advance_dense(&mut self, cursor: usize) -> Result<bool, &'static str> {
        let engine = self.engine.as_ref().ok_or("remodeling.reconstruction.engine")?;
        let total = engine.dense_positions().len().min(RECONSTRUCTION_DENSE_TRACE_MAX);
        if cursor >= total {
            self.stage = ReconstructionRunStage::Products;
            self.phase = RunPhase::Products(Box::new(ProductPreparation::new(self.gcp_count)));
            return Ok(false);
        }
        let end = (cursor + RECONSTRUCTION_DENSE_TRACE_BATCH).min(total);
        let batch: Vec<[f64; 3]> = engine.dense_positions()[cursor..end].to_vec();
        self.phase = RunPhase::DenseTrace { cursor: end };
        for (offset, point) in batch.into_iter().enumerate() {
            self.count(ReconstructionRunCounter::DensePoints, 1);
            self.upsert(KEY_DENSE | (cursor + offset) as u64, ReconstructionRunReason::DenseCloudFused, point_subject(point));
        }
        Ok(true)
    }

    fn advance_products(&mut self) -> Result<bool, &'static str> {
        let (RunPhase::Products(preparation), Some(engine)) = (&mut self.phase, self.engine.as_mut()) else { return Err("remodeling.reconstruction.products") };
        match preparation.advance(engine) {
            ProductYield::Working => Ok(false),
            ProductYield::Step(reason, args) => {
                self.step(reason, &args);
                Ok(false)
            }
            ProductYield::Op(op) => {
                self.append(&op)?;
                self.owed.push_back(Owed::Checkpoint);
                Ok(true)
            }
            ProductYield::Failed(reason) => {
                self.fault(reason, &[]);
                Ok(true)
            }
            ProductYield::Done => {
                self.step(ReconstructionRunReason::ResultReady, &[u64::from(self.ops_emitted)]);
                self.phase = RunPhase::Settled;
                self.owed.push_back(Owed::Checkpoint);
                self.owed.push_back(Owed::Complete);
                Ok(true)
            }
        }
    }

    fn flush(&mut self, cx: &mut StepContext<'_>, state: ToolRunState) -> Option<StepOutcome> {
        if self.writer.is_empty() || !self.live() {
            return None;
        }
        self.writer.progress(self.progress(state));
        let bytes = self.writer.finish()?.encode().ok()?;
        Some(match cx.payload_from_bytes(JobPayloadStream::Preview, &bytes) {
            Ok(payload) => StepOutcome::PreviewReady(payload),
            Err(rejected) => {
                drop(rejected.into_source());
                fault_outcome()
            }
        })
    }

    fn settle(&mut self, cx: &mut StepContext<'_>, owed: Owed) -> StepOutcome {
        match owed {
            Owed::Checkpoint => match cx.payload_from_bytes(JobPayloadStream::CheckpointState, &self.checkpoint().encode()) {
                Ok(state) => StepOutcome::CheckpointReady(Checkpoint { state, applied_progress: self.decisions }),
                Err(rejected) => {
                    drop(rejected.into_source());
                    StepOutcome::Yield
                }
            },
            Owed::Complete => StepOutcome::Complete(CommitCandidate { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output: RetainedJobPayload::empty(JobPayloadStream::CommitOutput) }),
            Owed::Fault => fault_outcome(),
        }
    }
}

fn fault_outcome() -> StepOutcome {
    StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) })
}

impl InteractiveJob for ReconstructionRunJob {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        if self.closing || cx.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        let mut advanced = false;
        loop {
            let terminal = self.owed.iter().any(|owed| matches!(owed, Owed::Complete | Owed::Fault));
            if !self.owed.is_empty() && (self.live() || terminal) {
                let state = if self.owed.contains(&Owed::Complete) { ToolRunState::Complete } else { ToolRunState::Running };
                if let Some(preview) = self.flush(cx, state) {
                    return preview;
                }
                let owed = self.owed.pop_front().expect("owed outcome");
                return self.settle(cx, owed);
            }
            if !self.live() {
                self.owed.clear();
            }
            if advanced && (cx.deadline_exceeded() || (self.live() && (cx.fuel_exhausted() || self.writer.pending_bytes() >= TICK_FLUSH_BYTES))) {
                return self.flush(cx, ToolRunState::Running).unwrap_or(StepOutcome::Yield);
            }
            advanced = true;
            match self.advance() {
                Ok(true) => self.decide(cx),
                Ok(false) => {}
                Err(_) => {
                    self.phase = RunPhase::Settled;
                    self.owed.push_back(Owed::Fault);
                }
            }
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> InteractiveJobCloseStep {
        self.closing = true;
        if maximum_items == 0 {
            return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if !matches!(self.phase, RunPhase::Settled) {
            self.phase = RunPhase::Settled;
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if self.engine.take().is_some() || self.scene.take().is_some() || !self.observations.is_empty() || !self.owed.is_empty() {
            self.observations.clear();
            self.owed.clear();
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && matches!(self.phase, RunPhase::Settled) && self.engine.is_none() && self.scene.is_none() && self.observations.is_empty() && self.owed.is_empty()
    }
}
//#endregion 🧵️RunJob

//#region 🔍️RevalidateJob
/// ✅️ The finalize `revalidateJob`: when the head's reconstruction inputs no longer equal the run's, every
/// provisional op is withdrawn with one danger step; otherwise the provisional result stands.
pub struct ReconstructionRevalidateJob {
    identity: ToolRunIdentity,
    head: Option<Arc<RemodelingSnapshot>>,
    run_inputs: Option<RemodelingContentDigest>,
    provisional: u32,
    owed: Option<Owed>,
    closing: bool,
}

impl ReconstructionRevalidateJob {
    pub fn new(identity: ToolRunIdentity, head: Arc<RemodelingSnapshot>, checkpoint: Option<&[u8]>, provisional: u32) -> Self {
        Self { identity, head: Some(head), run_inputs: checkpoint.and_then(ReconstructionRunCheckpoint::decode).map(|checkpoint| checkpoint.inputs), provisional, owed: None, closing: false }
    }
}

impl InteractiveJob for ReconstructionRevalidateJob {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        if self.closing || cx.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if let Some(owed) = self.owed.take() {
            return match owed {
                Owed::Complete => StepOutcome::Complete(CommitCandidate { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output: RetainedJobPayload::empty(JobPayloadStream::CommitOutput) }),
                Owed::Checkpoint | Owed::Fault => fault_outcome(),
            };
        }
        let Some(head) = self.head.take() else { return fault_outcome() };
        let unchanged = self.run_inputs == Some(reconstruction_inputs(&head));
        let mut writer = ToolRunTickWriter::with_provisional_base(self.identity, self.provisional);
        if !unchanged {
            writer.retract_to(0);
            let _ = writer.step(ToolRunStepKind::Danger, ReconstructionRunStage::Products.index(), ReconstructionRunReason::InputsChanged.code(), None, &[ToolRunStepArg::Unsigned(u64::from(self.provisional))]);
            let _ = writer.step(ToolRunStepKind::Danger, ReconstructionRunStage::Products.index(), TOOL_RUN_REASON_CONFLICT, None, &[ToolRunStepArg::Unsigned(u64::from(self.provisional))]);
        }
        cx.consume_fuel(1);
        writer.progress(ToolRunProgress {
            identity: self.identity,
            sequence: 0,
            state: ToolRunState::Finalizing,
            stage: ReconstructionRunStage::Products.index(),
            completed: 1,
            total: Some(1),
            counters: Vec::new(),
            units_per_second: 0.0,
            conflicts: if unchanged { 0 } else { self.provisional },
            steps: ToolRunStepRing::new(),
        });
        self.owed = Some(Owed::Complete);
        match writer.finish().and_then(|tick| tick.encode().ok()).map(|bytes| cx.payload_from_bytes(JobPayloadStream::Preview, &bytes)) {
            Some(Ok(payload)) => StepOutcome::PreviewReady(payload),
            Some(Err(rejected)) => {
                drop(rejected.into_source());
                fault_outcome()
            }
            None => fault_outcome(),
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, _maximum_items: usize, _maximum_bytes: usize) -> InteractiveJobCloseStep {
        self.closing = true;
        self.head = None;
        self.owed = None;
        InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.head.is_none() && self.owed.is_none()
    }
}
//#endregion 🔍️RevalidateJob

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
