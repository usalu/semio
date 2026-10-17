//! 🏗️ Core of the reconstruction engine: the engine state, its construction and observation and the budgeted advance loop that drives the stages. See [`super`].

use super::*;

// #region 🔖️Pipeline
/// ⚙️ The cooperative staged pipeline: [`advance`](Self::advance) performs one bounded slice of work per
/// call (one image featurized, one pair matched, one PnP registration, one bundle-adjustment solve, one
/// depth map, one TSDF integration batch, one `remodeling_mesh` pipeline stage) and is genuinely resumable —
/// calling it repeatedly with a small budget or once with a huge budget reaches the same [`EngineStatus::Done`]
/// result, only the call count differs.
#[derive(semio_framework_value::RetireOwned)]
pub struct ReconstructionEngine {
    pub params: EngineParams,
    pub frame_source: FrameSource,
    pub(super) stage: EngineStage,
    pub frames: Vec<AcceptedFrame>,
    pub cursor: usize,
    pub feature_preparation: Option<FeaturePreparation>,

    pub keypoints_per_frame: Vec<Vec<remodeling_feature::Keypoint>>,
    pub descriptors_per_frame: Vec<Vec<remodeling_feature::Descriptor256>>,

    pub match_pairs: Vec<(usize, usize)>,
    pub(super) match_pair_i: usize,
    pub(super) match_pair_j: usize,
    pub(super) match_anchor_frame: usize,
    match_pairs_ready: bool,
    pub pair_cursor: usize,
    pub pair_match_preparation: Option<PairMatchPreparation>,
    pub pairwise_matches: Vec<(usize, usize, Vec<remodeling_feature::Match>)>,
    pub track_preparation: Option<TrackPreparation>,
    pub tracks: Option<semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::FeatureTracks>,

    pub sfm: Option<semio_s_artifact_remodel_remodeling_sfm::editor::remodeling::engine::sfm::IncrementalSfm>,
    pub(super) seed_pair_preparation: Option<semio_s_artifact_remodel_remodeling_sfm::editor::remodeling::engine::sfm::SeedPairPreparation>,
    pub(super) registration_preparation: Option<semio_s_artifact_remodel_remodeling_sfm::editor::remodeling::engine::sfm::RegistrationPreparation>,
    pub(super) bundle_preparation: Option<semio_s_artifact_remodel_remodeling_sfm::editor::remodeling::engine::sfm::BundlePreparation>,
    /// 🎯️ A bundle adjustment in flight, [`BUNDLE_TERMS_PER_STEP`] terms per step: the local one
    /// after each registration and the global passes of the bundle stage.
    pub(super) bundle_iterations: Option<semio_s_artifact_remodel_remodeling_sfm::editor::remodeling::engine::sfm::BundleAdjustment>,
    pub(super) finalization_preparation: Option<FinalizationPreparation>,
    pub pose_cursor: usize,
    /// 🧭️ Frames to register after the seed pair, in registration order (outwards from the seed).
    pub(super) registration_order: Vec<usize>,
    pub(super) ba_substep: usize,
    pub reconstruction: Option<semio_s_artifact_remodel_remodeling_sfm::editor::remodeling::engine::sfm::Reconstruction>,
    pub observations: Vec<(usize, usize, [f64; 2])>,
    pub dense_camera_indices: Vec<usize>,

    pub stage_cursor: usize,
    pub depth_maps: Vec<remodeling_dense::DepthMap>,
    pub dense_preparation: Option<DenseStereoPreparation>,
    pub tsdf: Option<remodeling_dense::TsdfVolume>,
    pub(super) tsdf_preparation: Option<remodeling_dense::TsdfIntegrationPreparation>,
    pub(super) fusion_preparation: Option<remodeling_dense::FusionPreparation>,
    pub(super) fusion_views: Vec<(remodeling_camera::CameraPose, remodeling_camera::Intrinsics)>,
    pub(super) fusion_capacity: usize,
    pub fusion_finalized: bool,
    pub dense_cloud: Option<remodeling_dense::PointCloud>,
    /// ✅️ Per dense view, the fusion stage's multi-view consistency bits; the TSDF integrates only
    /// those pixels, so a single view's PatchMatch outliers never reach the volume.
    pub(super) consistency_masks: Vec<Vec<u64>>,

    pub mesh_pipeline: Option<semio_s_artifact_remodel_remodeling_mesh::editor::remodeling::engine::mesh::MeshPipeline>,
    pub meshing_preparation: Option<MeshingPreparation>,
    pub mesh_data: Option<semio_framework::MeshData>,
    pub(super) watertight_report: Option<semio_s_artifact_remodel_remodeling_mesh_repair::editor::remodeling::engine::mesh::repair::WatertightReport>,

    failure: Option<String>,
    pub recorded: Option<Vec<EngineObservation>>,
}

impl ReconstructionEngine {
    /// 🆕️ A fresh engine in [`EngineStage::Idle`], with an internal empty [`FrameSource`] under
    /// `params.ingest`. Push frames via [`push_frame`](Self::push_frame)/[`push_video`](Self::push_video)
    /// before the first [`advance`](Self::advance) call.
    pub fn new(params: &EngineParams) -> Self {
        Self {
            params: params.clone(),
            frame_source: FrameSource::new(params.ingest.clone()),
            stage: EngineStage::Idle,
            frames: Vec::new(),
            cursor: 0,
            feature_preparation: None,
            keypoints_per_frame: Vec::new(),
            descriptors_per_frame: Vec::new(),
            match_pairs: Vec::new(),
            match_pair_i: 0,
            match_pair_j: 1,
            match_anchor_frame: 0,
            match_pairs_ready: false,
            pair_cursor: 0,
            pair_match_preparation: None,
            pairwise_matches: Vec::new(),
            track_preparation: None,
            tracks: None,
            sfm: None,
            seed_pair_preparation: None,
            registration_preparation: None,
            bundle_preparation: None,
            bundle_iterations: None,
            finalization_preparation: None,
            pose_cursor: 0,
            registration_order: Vec::new(),
            ba_substep: 0,
            reconstruction: None,
            observations: Vec::new(),
            dense_camera_indices: Vec::new(),
            stage_cursor: 0,
            depth_maps: Vec::new(),
            dense_preparation: None,
            tsdf: None,
            tsdf_preparation: None,
            fusion_preparation: None,
            fusion_views: Vec::new(),
            fusion_capacity: 0,
            fusion_finalized: false,
            dense_cloud: None,
            consistency_masks: Vec::new(),
            mesh_pipeline: None,
            meshing_preparation: None,
            mesh_data: None,
            watertight_report: None,
            failure: None,
            recorded: None,
        }
    }

    /// 🔭️ Starts recording [`EngineObservation`]s; an unobserved engine records nothing.
    pub fn observe(&mut self) {
        self.recorded.get_or_insert_with(Vec::new);
    }

    /// 📤️ Moves every recorded observation into `into`, oldest first.
    pub fn drain_observations(&mut self, into: &mut Vec<EngineObservation>) {
        if let (Some(sfm), Some(observations)) = (self.sfm.as_mut(), self.recorded.as_mut()) {
            sfm.drain_point_events(|track, point, kept| observations.push(if kept { EngineObservation::PointTriangulated { track, point } } else { EngineObservation::PointPruned { track, point } }));
        }
        if let Some(observations) = self.recorded.as_mut() {
            into.append(observations);
        }
    }

    pub(super) fn record(&mut self, observation: EngineObservation) {
        if let Some(observations) = self.recorded.as_mut() {
            observations.push(observation);
        }
    }

    /// 📏️ `(completed, total)` units of the current stage; `total` is absent where the stage has no
    /// countable extent.
    pub fn stage_progress(&self) -> (u64, Option<u64>) {
        let count = |value: usize| value as u64;
        match self.stage {
            EngineStage::ExtractingFeatures => (count(self.cursor), Some(count(self.frames.len()))),
            EngineStage::MatchingFeatures if self.match_pairs_ready => (count(self.pair_cursor), Some(count(self.match_pairs.len()))),
            EngineStage::EstimatingPoses => (count(self.pose_cursor.min(self.registration_order.len())), Some(count(self.registration_order.len()))),
            EngineStage::DenseStereo | EngineStage::FusingVolume => (count(self.stage_cursor.min(self.dense_camera_indices.len())), Some(count(self.dense_camera_indices.len()))),
            _ => (0, None),
        }
    }

    /// 🔬️ The raw matching inputs an independent match oracle recomputes verdicts from: descriptor words
    /// per frame, the scheduled pairs, the ratio and whether the cross check runs.
    #[cfg(test)]
    pub fn match_oracle_inputs(&self) -> (Vec<Vec<[u64; 4]>>, Vec<(usize, usize)>, f32, bool) {
        (self.descriptors_per_frame.iter().map(|frame| frame.iter().map(|descriptor| descriptor.0).collect()).collect(), self.match_pairs.clone(), self.params.match_ratio, self.params.match_mutual)
    }

    /// ☁️ The fused dense cloud positions once [`EngineStage::FusingVolume`] finished, else empty.
    pub fn dense_positions(&self) -> &[[f64; 3]] {
        self.dense_cloud.as_ref().map_or(&[], |cloud| cloud.positions.as_slice())
    }

    /// 📥️ Delegates to the internal [`FrameSource::push_frame`].
    #[cfg(test)]
    pub fn push_frame(&mut self, index: u32, image: remodeling_image::ImageRgba8, timestamp_ms: f64) -> FrameAcceptance {
        self.frame_source.push_frame(index, image, timestamp_ms)
    }

    /// ⏱️ Commits a frame after its gradient score was computed by the outer resumable ingestion cursor.
    pub fn push_frame_with_sharpness(&mut self, index: u32, image: remodeling_image::ImageRgba8, timestamp_ms: f64, sharpness: f32) -> FrameAcceptance {
        self.frame_source.accept_with_sharpness(index, image, timestamp_ms, true, sharpness)
    }

    /// 🎞️ Delegates to the internal [`FrameSource::push_video`].
    #[cfg(test)]
    pub fn push_video(&mut self, bytes: &[u8], opts: &semio_s_artifact_remodel_remodeling_video::editor::remodeling::engine::video::VideoIngestOptions) -> Result<PushVideoReport, EngineError> {
        self.frame_source.push_video(bytes, opts)
    }

    /// 🔍️ The internal frame source, for inspecting accepted frames/counts without driving the pipeline.
    pub fn frame_source(&self) -> &FrameSource {
        &self.frame_source
    }

    /// 🚦️ Current stage.
    pub fn stage(&self) -> EngineStage {
        self.stage
    }

    /// 📸️ Snapshots [`FrameSource::frames`] into the engine's own working set; fails if fewer than 2
    /// frames were accepted (the minimum an [`crate::editor::remodeling::engine::sfm::IncrementalSfm`] two-view init needs).
    pub fn start(&mut self) -> Result<(), String> {
        self.frames = std::mem::take(&mut self.frame_source.frames);
        if self.frames.len() < 2 {
            return Err(format!("reconstruction requires at least 2 accepted frames, got {}", self.frames.len()));
        }
        if self.frames.len() > 64 {
            return Err(format!("reconstruction admits at most 64 frames per interactive job, got {}", self.frames.len()));
        }
        for frame in &self.frames {
            let pixels = frame.image.width as usize * frame.image.height as usize;
            if pixels > MAX_INTERACTIVE_IMAGE_PIXELS || frame.image.data.len() != pixels.saturating_mul(4) || frame.image.data.len() > MAX_INTERACTIVE_IMAGE_BYTES {
                return Err(format!("interactive reconstruction image envelope exceeded by frame {}: {}x{} / {} bytes", frame.index, frame.image.width, frame.image.height, frame.image.data.len()));
            }
        }
        self.cursor = 0;
        Ok(())
    }
}

impl ReconstructionEngine {
    /// 📈️ Coarse `[0, 1]` progress from the current stage's ordinal alone (no intra-stage fraction — the
    /// per-stage cursors have wildly different, not-necessarily-comparable totals).
    pub(super) fn progress(&self) -> f32 {
        stage_ordinal(self.stage) as f32 / 10.0
    }

    /// ⚙️ Advances the pipeline through at most `step_budget` bounded units of work (never fewer than 1),
    /// crossing stage boundaries within the same call whenever a stage finishes with budget still left —
    /// the same style `crate::editor::remodeling::engine::mesh::mesh_pipeline_step` uses internally. Genuinely resumable: calling
    /// this repeatedly with a small budget or once with `usize::MAX` reaches the same terminal
    /// [`EngineStatus`], only the call count differs.
    pub fn advance(&mut self, step_budget: usize) -> EngineStatus {
        for _ in 0..step_budget.max(1) {
            match self.stage {
                EngineStage::Idle => {
                    if let Err(msg) = self.start() {
                        self.stage = EngineStage::Failed;
                        self.failure = Some(msg.clone());
                        return EngineStatus::Failed(msg);
                    }
                    self.stage = EngineStage::ExtractingFeatures;
                }
                EngineStage::ExtractingFeatures => {
                    if !self.step_extracting_features() {
                        self.stage = EngineStage::MatchingFeatures;
                    }
                }
                EngineStage::MatchingFeatures => {
                    if !self.match_pairs_ready {
                        self.match_pairs_ready = self.step_build_match_pairs(64);
                    } else if !self.step_matching_features() {
                        if let Some(tracks) = self.step_build_tracks() {
                            self.record(EngineObservation::TracksBuilt { tracks: tracks.tracks.len() });
                            self.tracks = Some(tracks);
                            self.stage = EngineStage::EstimatingPoses;
                        }
                    }
                }
                EngineStage::EstimatingPoses => match self.step_estimating_poses() {
                    Ok(true) => {}
                    Ok(false) => {
                        self.ba_substep = 0;
                        self.stage = EngineStage::BundleAdjusting;
                    }
                    Err(msg) => {
                        self.stage = EngineStage::Failed;
                        self.failure = Some(msg.clone());
                        return EngineStatus::Failed(msg);
                    }
                },
                EngineStage::BundleAdjusting => {
                    if !self.step_bundle_adjusting() && self.step_finalize_reconstruction() {
                        self.stage = EngineStage::DenseStereo;
                    }
                }
                EngineStage::DenseStereo => {
                    if !self.step_dense_stereo() {
                        self.stage_cursor = 0;
                        self.stage = EngineStage::FusingVolume;
                    }
                }
                EngineStage::FusingVolume => {
                    if !self.step_fusing_volume() && self.step_begin_meshing() {
                        self.stage = EngineStage::ExtractingSurface;
                    }
                }
                EngineStage::ExtractingSurface | EngineStage::CleaningMesh | EngineStage::Texturing => match self.step_meshing() {
                    MeshStepOutcome::Working(stage) => self.stage = stage,
                    MeshStepOutcome::Done => {
                        self.mesh_data = self.mesh_pipeline.as_ref().and_then(semio_s_artifact_remodel_remodeling_mesh::editor::remodeling::engine::mesh::MeshPipeline::result).cloned();
                        self.watertight_report = self.mesh_pipeline.as_ref().and_then(semio_s_artifact_remodel_remodeling_mesh::editor::remodeling::engine::mesh::MeshPipeline::report).cloned();
                        let (vertices, triangles) = self.mesh_data.as_ref().map_or((0, 0), |mesh| (mesh.positions.len() / 3, mesh.indices.len() / 3));
                        self.record(EngineObservation::MeshExtracted { vertices, triangles });
                        self.stage = EngineStage::Done;
                        return EngineStatus::Done;
                    }
                    MeshStepOutcome::Failed(msg) => {
                        self.stage = EngineStage::Failed;
                        self.failure = Some(msg.clone());
                        return EngineStatus::Failed(msg);
                    }
                },
                EngineStage::Done => return EngineStatus::Done,
                EngineStage::Failed => return EngineStatus::Failed(self.failure.clone().unwrap_or_default()),
            }
        }
        match self.stage {
            EngineStage::Done => EngineStatus::Done,
            EngineStage::Failed => EngineStatus::Failed(self.failure.clone().unwrap_or_default()),
            stage => EngineStatus::Working { stage, progress: self.progress() },
        }
    }
}
// #endregion 🔖️Pipeline
