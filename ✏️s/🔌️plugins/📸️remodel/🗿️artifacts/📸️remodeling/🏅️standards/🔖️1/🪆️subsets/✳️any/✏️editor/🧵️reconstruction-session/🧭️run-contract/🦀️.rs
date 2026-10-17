//! 🧭️ Run contract of the reconstruction session: the job kinds and trace lanes, the stage, counter and reason vocabulary and the tool-run definition. See [`super`].

use super::*;

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

pub(super) const TICK_FLUSH_BYTES: usize = 6 * 1024;

pub(super) const CHECKPOINT_DECISIONS: u64 = 64;

pub(super) const MAX_STILL_INPUT_BYTES: usize = 1_114_112;

pub(super) const SHARPNESS_PIXELS_PER_UNIT: usize = 4_096;

pub(super) const TERMINAL_CAMERA_WORK: usize = 64;

pub(super) const TERMINAL_POINT_WORK: usize = 256;

pub(super) const TERMINAL_QUALITY_WORK: usize = 256;

pub(super) const TERMINAL_GEO_WORK: usize = 256;

pub(super) const RASTER_CELL_WORK: usize = 4_096;

pub(super) const REMODELING_RECONSTRUCTION_DSM_ASSET_ID: &str = "reconstruction-dsm";

pub(super) const REMODELING_RECONSTRUCTION_DTM_ASSET_ID: &str = "reconstruction-dtm";

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
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
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
    ToolRunDefinition { receiving: semio_framework_tool_run::ToolRunReceivingLimits::REPORTS,
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
        member: None,
    }
}
//#endregion 🔖️Contract
