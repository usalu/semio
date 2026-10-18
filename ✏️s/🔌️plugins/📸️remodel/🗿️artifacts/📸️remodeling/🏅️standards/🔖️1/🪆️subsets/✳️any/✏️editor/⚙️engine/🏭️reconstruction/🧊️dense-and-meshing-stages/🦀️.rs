//! 🧊️ Dense and meshing stages of the reconstruction engine: the finalisation, dense stereo, volume bounds, fusion and meshing steps with their preparations. See [`super`].

use super::*;

// #region 🔖️Pipeline
/// 🧊️ Coarsest-needed TSDF voxel in the normalised reconstruction frame; see
/// [`ReconstructionEngine::meshing_voxel`].
const MIN_MESHING_VOXEL: f64 = 0.2;

/// 🧊️ Truncation floor in voxels: a band thinner than this leaves the observed sheet full of gaps.
const MIN_TRUNCATION_VOXELS: f64 = 2.0;

/// ✅️ Relative depth agreement two views' pixels need to count as one surface point. PatchMatch at
/// the interactive iteration counts is ~1.5 % noisy in depth, so the batch default of 1 % rejected
/// most good pixels; 3 % is still well inside one meshing voxel at the orbit's viewing distance.
const FUSION_MAX_RELATIVE_DEPTH_DIFF: f32 = 0.03;

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
enum DenseStereoPhase {
    ReferenceLuma,
    SourceLuma,
    PatchMatch,
}

#[derive(semio_framework_value::RetireOwned)]
pub(super) struct DenseStereoPreparation {
    pub(super) phase: DenseStereoPhase,
    reference_frame: usize,
    reference_camera: (remodeling_camera::CameraPose, remodeling_camera::Intrinsics),
    source_frames: Vec<(usize, remodeling_camera::CameraPose)>,
    pub reference_gray: remodeling_image::ImageGray,
    pub source_grays: Vec<(remodeling_image::ImageGray, remodeling_camera::CameraPose, remodeling_camera::Intrinsics)>,
    pub(super) source: usize,
    pub(super) pixel: usize,
    /// 🌫️ The run's PatchMatch configuration with this view's own depth range (see
    /// [`sparse_depth_range`]).
    pub(super) dense: remodeling_dense::PatchMatchConfig,
    pub patch_match: Option<remodeling_dense::PatchMatchPreparation>,
}

#[derive(semio_framework_value::RetireOwned)]
struct TextureViewPreparation {
    pub(super) pose: remodeling_camera::CameraPose,
    pub(super) intrinsics: remodeling_camera::Intrinsics,
    pub image: remodeling_image::ImageRgba8,
    pub(super) cursor: usize,
}

#[derive(semio_framework_value::RetireOwned)]
pub(super) struct MeshingPreparation {
    sparse_cursor: usize,
    dense_cursor: usize,
    pub bounds_min: [f64; 3],
    pub bounds_max: [f64; 3],
    view_cursor: usize,
    pub active_view: Option<TextureViewPreparation>,
    pub views: Vec<semio_s_artifact_remodel_remodeling_mesh_atlas::editor::remodeling::engine::mesh::atlas::TextureView>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
enum FinalizationPhase {
    Snapshot,
    CameraIndex,
    Observations,
    DenseCameras,
    Done,
}

#[derive(semio_framework_value::RetireOwned)]
pub(super) struct FinalizationPreparation {
    pub(super) phase: FinalizationPhase,
    snapshot: Option<semio_s_artifact_remodel_remodeling_sfm::editor::remodeling::engine::sfm::ReconstructionSnapshotPreparation>,
    pub(super) camera_cursor: usize,
    pub(super) camera_index: std::collections::BTreeMap<usize, usize>,
    pub(super) point_cursor: usize,
    pub(super) observation_cursor: usize,
    dense_cursor: usize,
}

impl MeshingPreparation {
    pub(super) fn new() -> Self {
        Self { sparse_cursor: 0, dense_cursor: 0, bounds_min: [f64::INFINITY; 3], bounds_max: [f64::NEG_INFINITY; 3], view_cursor: 0, active_view: None, views: Vec::new() }
    }

    fn include(&mut self, point: [f64; 3]) {
        for (axis, coordinate) in point.into_iter().enumerate() {
            self.bounds_min[axis] = self.bounds_min[axis].min(coordinate);
            self.bounds_max[axis] = self.bounds_max[axis].max(coordinate);
        }
    }
}

impl ReconstructionEngine {
    /// 📦️ Moves the final sparse snapshot, observation table, and dense-camera index through bounded
    /// item cursors. No camera/point/observation collection is materialized in one continuation.
    pub(super) fn step_finalize_reconstruction(&mut self) -> bool {
        const ITEMS_PER_STEP: usize = 64;
        if self.finalization_preparation.is_none() {
            let snapshot = self.sfm.as_ref().map(semio_s_artifact_remodel_remodeling_sfm::editor::remodeling::engine::sfm::IncrementalSfm::begin_reconstruction_snapshot);
            self.finalization_preparation = Some(FinalizationPreparation { phase: FinalizationPhase::Snapshot, snapshot, camera_cursor: 0, camera_index: std::collections::BTreeMap::new(), point_cursor: 0, observation_cursor: 0, dense_cursor: 0 });
        }
        let preparation = self.finalization_preparation.as_mut().expect("finalization preparation");
        match preparation.phase {
            FinalizationPhase::Snapshot => {
                let Some(snapshot) = preparation.snapshot.as_mut() else {
                    preparation.phase = FinalizationPhase::Done;
                    return true;
                };
                let sfm = self.sfm.as_ref().expect("snapshot SfM");
                if sfm.advance_reconstruction_snapshot(snapshot, ITEMS_PER_STEP) {
                    let snapshot = preparation.snapshot.take().expect("completed snapshot");
                    let mut reconstruction = semio_s_artifact_remodel_remodeling_sfm::editor::remodeling::engine::sfm::IncrementalSfm::finish_reconstruction_snapshot(snapshot);
                    normalize_reconstruction_frame(&mut reconstruction);
                    self.reconstruction = Some(reconstruction);
                    preparation.phase = FinalizationPhase::CameraIndex;
                }
            }
            FinalizationPhase::CameraIndex => {
                let reconstruction = self.reconstruction.as_ref().expect("final reconstruction");
                let end = preparation.camera_cursor.saturating_add(ITEMS_PER_STEP).min(reconstruction.cameras.len());
                for index in preparation.camera_cursor..end {
                    preparation.camera_index.insert(reconstruction.cameras[index].0, index);
                }
                preparation.camera_cursor = end;
                if end == reconstruction.cameras.len() {
                    preparation.phase = FinalizationPhase::Observations;
                }
            }
            FinalizationPhase::Observations => {
                let reconstruction = self.reconstruction.as_ref().expect("final reconstruction");
                let tracks = self.tracks.as_ref().expect("feature tracks");
                let mut remaining = ITEMS_PER_STEP;
                while preparation.point_cursor < reconstruction.point_track_ids.len() && remaining > 0 {
                    let track_id = reconstruction.point_track_ids[preparation.point_cursor];
                    let Some(track) = tracks.tracks.get(track_id) else {
                        preparation.point_cursor += 1;
                        preparation.observation_cursor = 0;
                        continue;
                    };
                    if preparation.observation_cursor >= track.len() {
                        preparation.point_cursor += 1;
                        preparation.observation_cursor = 0;
                        continue;
                    }
                    let (frame, keypoint) = track[preparation.observation_cursor];
                    preparation.observation_cursor += 1;
                    remaining -= 1;
                    let Some(&camera) = preparation.camera_index.get(&frame) else { continue };
                    let Some(keypoints) = self.keypoints_per_frame.get(frame) else { continue };
                    let Some(keypoint) = keypoints.get(keypoint as usize) else { continue };
                    self.observations.push((camera, preparation.point_cursor, [f64::from(keypoint.x), f64::from(keypoint.y)]));
                }
                if preparation.point_cursor == reconstruction.point_track_ids.len() {
                    preparation.phase = FinalizationPhase::DenseCameras;
                }
            }
            FinalizationPhase::DenseCameras => {
                let cameras = self.reconstruction.as_ref().map_or(0, |reconstruction| reconstruction.cameras.len());
                let dense_count = if self.params.max_dense_cameras == 0 { cameras.min(12) } else { cameras.min(self.params.max_dense_cameras).min(12) };
                if preparation.dense_cursor < dense_count {
                    let index = if dense_count <= 1 { 0 } else { preparation.dense_cursor * cameras.saturating_sub(1) / dense_count.saturating_sub(1) };
                    self.dense_camera_indices.push(index);
                    preparation.dense_cursor += 1;
                } else {
                    self.depth_maps = Vec::with_capacity(self.dense_camera_indices.len());
                    self.stage_cursor = 0;
                    preparation.phase = FinalizationPhase::Done;
                }
            }
            FinalizationPhase::Done => return true,
        }
        preparation.phase == FinalizationPhase::Done
    }

    /// 🌫️ One registered camera's `remodeling_dense::patchmatch_mvs` depth map against its nearest
    /// registered neighbors; returns whether more cameras remain.
    pub(super) fn step_dense_stereo(&mut self) -> bool {
        const LUMA_PIXELS_PER_STEP: usize = 2_048;
        // ⏱️ One PatchMatch pixel evaluates `(2r + 1)²` reference samples against every source view
        // per hypothesis; the admitted worst case (radius 4, eight sources = 648 samples) is held to
        // one pixel per step, and a lighter configuration takes proportionally more pixels.
        const PATCH_SAMPLES_PER_STEP: usize = 648;
        let patch_side = 2 * self.params.dense.window_radius.clamp(1, 4) as usize + 1;
        let patch_sources = self.params.dense_source_views.clamp(1, 8);
        let patch_pixels_per_step = (PATCH_SAMPLES_PER_STEP / (patch_side * patch_side * patch_sources)).max(1);
        let n_dense = self.dense_camera_indices.len();
        if n_dense == 0 {
            return false;
        }
        let slot = self.stage_cursor;
        if slot >= n_dense {
            return false;
        }
        if self.dense_preparation.is_none() {
            let ci = self.dense_camera_indices[slot];
            let reconstruction = self.reconstruction.as_ref().expect("dense requires reconstruction");
            let (reference_frame, pose) = reconstruction.cameras[ci];
            let intrinsics = reconstruction.intrinsics;
            let source_frames = neighbor_camera_indices(ci, reconstruction.cameras.len(), self.params.dense_source_views.min(8)).into_iter().map(|neighbor| reconstruction.cameras[neighbor]).collect();
            let reference = &self.frames[reference_frame].image;
            let (depth_min, depth_max) = sparse_depth_range(&reconstruction.points, &pose, &self.params.dense);
            let dense = remodeling_dense::PatchMatchConfig { depth_min, depth_max, ..self.params.dense };
            self.dense_preparation = Some(DenseStereoPreparation {
                phase: DenseStereoPhase::ReferenceLuma,
                reference_frame,
                reference_camera: (pose, intrinsics),
                source_frames,
                reference_gray: remodeling_image::ImageGray { width: reference.width, height: reference.height, data: Vec::with_capacity(reference.width as usize * reference.height as usize) },
                source_grays: Vec::new(),
                source: 0,
                pixel: 0,
                dense,
                patch_match: None,
            });
        }
        let preparation = self.dense_preparation.as_mut().expect("dense preparation");
        match preparation.phase {
            DenseStereoPhase::ReferenceLuma => {
                if append_luma_slice(&self.frames[preparation.reference_frame].image, &mut preparation.reference_gray, &mut preparation.pixel, LUMA_PIXELS_PER_STEP) {
                    preparation.pixel = 0;
                    preparation.phase = DenseStereoPhase::SourceLuma;
                }
            }
            DenseStereoPhase::SourceLuma => {
                if preparation.source == preparation.source_frames.len() {
                    preparation.phase = DenseStereoPhase::PatchMatch;
                } else {
                    let (frame, pose) = preparation.source_frames[preparation.source];
                    if preparation.source_grays.len() == preparation.source {
                        let image = &self.frames[frame].image;
                        preparation.source_grays.push((remodeling_image::ImageGray { width: image.width, height: image.height, data: Vec::with_capacity(image.width as usize * image.height as usize) }, pose, preparation.reference_camera.1));
                    }
                    let complete = append_luma_slice(&self.frames[frame].image, &mut preparation.source_grays[preparation.source].0, &mut preparation.pixel, LUMA_PIXELS_PER_STEP);
                    if complete {
                        preparation.source += 1;
                        preparation.pixel = 0;
                    }
                }
            }
            DenseStereoPhase::PatchMatch => {
                let patch_match = preparation.patch_match.get_or_insert_with(|| remodeling_dense::PatchMatchPreparation::new(preparation.reference_gray.width, preparation.reference_gray.height));
                if patch_match.advance(&preparation.reference_gray, &preparation.reference_camera, &preparation.source_grays, &preparation.dense, patch_pixels_per_step) {
                    let complete = self.dense_preparation.take().expect("completed dense preparation");
                    let map = complete.patch_match.expect("completed patch match").finish().expect("finished depth map");
                    self.record(EngineObservation::DepthMapEstimated { view: slot, samples: map.depth.iter().filter(|depth| depth.is_finite() && **depth > 0.0).count() });
                    self.fusion_capacity = self.fusion_capacity.saturating_add(map.depth.len());
                    self.depth_maps.push(map);
                    self.stage_cursor += 1;
                }
            }
        }
        self.stage_cursor < n_dense
    }

    /// 🧮️ One fuel-bounded slice of the sparse+dense extrema walk that fixes the meshing lattice (and
    /// with it the TSDF integration box); `true` once both clouds are walked. The cursors live in
    /// [`MeshingPreparation`], so the fusion stage and [`Self::step_begin_meshing`] share one walk
    /// rather than each performing their own.
    fn step_meshing_bounds(&mut self) -> bool {
        const POINTS_PER_STEP: usize = 2_048;
        let preparation = self.meshing_preparation.get_or_insert_with(MeshingPreparation::new);
        if let Some(reconstruction) = &self.reconstruction {
            if preparation.sparse_cursor < reconstruction.points.len() {
                let end = preparation.sparse_cursor.saturating_add(POINTS_PER_STEP).min(reconstruction.points.len());
                for &point in &reconstruction.points[preparation.sparse_cursor..end] {
                    preparation.include(point);
                }
                preparation.sparse_cursor = end;
                return false;
            }
        }
        if let Some(cloud) = &self.dense_cloud {
            if preparation.dense_cursor < cloud.positions.len() {
                let end = preparation.dense_cursor.saturating_add(POINTS_PER_STEP).min(cloud.positions.len());
                for &point in &cloud.positions[preparation.dense_cursor..end] {
                    preparation.include(point);
                }
                preparation.dense_cursor = end;
                return false;
            }
        }
        true
    }

    /// 🧊️ The fusion stage, in the one order that keeps the volume small: first the cross-view
    /// `fuse_depth_maps` aggregate for the QC/geo cloud (it reads every depth map and never touches the
    /// volume), then the extrema walk that fixes the meshing lattice, and only then the per-camera TSDF
    /// integration — restricted to that lattice, releasing each depth map as it is consumed. Returns
    /// whether more work remains in this stage.
    ///
    /// 🧠️ Why the integration is bounded rather than global: PatchMatch hypothesises depths anywhere in
    /// `[dense.depth_min, dense.depth_max]` (a 0.1–100 m range for uncalibrated input), so an
    /// unconstrained integration scatters one 4 KiB `8³` block per touched neighbourhood across a
    /// hundred-metre frustum — measured at ~300 MiB and still climbing on a ten-view 320x240 run, which
    /// is what used to abort the guest's allocator mid-stage. `remodeling_mesh`'s surface extraction only
    /// ever samples the `compute_voxel_bounds_from_extrema` lattice (itself clamped to 32 cells per axis)
    /// plus one voxel past each face, so every voxel it can read is still integrated bit-identically
    /// while nothing is allocated beyond the margin.
    pub(super) fn step_fusing_volume(&mut self) -> bool {
        const TSDF_SAMPLES_PER_STEP: usize = 256;
        const FUSION_COMPARISONS_PER_STEP: usize = 256;
        /// 🧱️ Voxels of slack around the meshing lattice: the surface net reads one voxel past each
        /// face through its cube corners, two keeps that provably inside the integrated region.
        const TSDF_INTEGRATION_MARGIN_VOXELS: i32 = 2;
        let n_dense = self.dense_camera_indices.len();
        if !self.fusion_finalized {
            let recon = self.reconstruction.as_ref().expect("fusion requires reconstruction");
            if self.fusion_views.len() < n_dense {
                let camera = self.dense_camera_indices[self.fusion_views.len()];
                self.fusion_views.push((recon.cameras[camera].1, recon.intrinsics));
                return true;
            }
            let config = self.fusion_config();
            let preparation = self.fusion_preparation.get_or_insert_with(|| remodeling_dense::FusionPreparation::new(self.fusion_capacity));
            if preparation.advance(&self.fusion_views, &self.depth_maps, &config, FUSION_COMPARISONS_PER_STEP) {
                if let Some((cloud, masks)) = self.fusion_preparation.take().and_then(remodeling_dense::FusionPreparation::finish_with_consistency) {
                    self.dense_cloud = Some(cloud);
                    self.consistency_masks = masks;
                }
                self.fusion_finalized = true;
                let points = self.dense_positions().len();
                self.record(EngineObservation::DenseCloudFused { points });
            }
            return true;
        }
        if !self.step_meshing_bounds() {
            return true;
        }
        // 🧊️ Always ensures a (possibly still-empty) TSDF exists once this stage starts, even when
        // `n_dense == 0` (a degenerate but legitimate outcome — every registered camera got pruned by
        // bundle adjustment): without this, `begin_meshing` used to find `self.tsdf` still `None` and
        // silently skip building a `MeshPipeline`, later surfacing as the confusing, wiring-looking
        // `"mesh pipeline not initialized"` failure instead of an honest empty-reconstruction outcome.
        if self.tsdf.is_none() {
            let preparation = self.meshing_preparation.as_ref().expect("walked meshing preparation");
            let (voxel_size, truncation) = self.meshing_voxel();
            let (lattice_min, lattice_max) = compute_voxel_bounds_from_extrema(preparation.bounds_min, preparation.bounds_max, voxel_size);
            self.tsdf = Some(
                remodeling_dense::TsdfVolume::new(voxel_size, truncation)
                    .with_voxel_bounds(lattice_min.map(|coordinate| coordinate.saturating_sub(TSDF_INTEGRATION_MARGIN_VOXELS)), lattice_max.map(|coordinate| coordinate.saturating_add(TSDF_INTEGRATION_MARGIN_VOXELS))),
            );
        }
        if self.stage_cursor < n_dense {
            let slot = self.stage_cursor;
            let ci = self.dense_camera_indices[slot];
            let (pose, intrinsics) = {
                let recon = self.reconstruction.as_ref().expect("fusion requires reconstruction");
                let (_, pose) = recon.cameras[ci];
                (pose, recon.intrinsics)
            };
            let preparation = self.tsdf_preparation.get_or_insert_with(remodeling_dense::TsdfIntegrationPreparation::new);
            let mask = self.consistency_masks.get(slot).map(Vec::as_slice);
            if preparation.advance_masked(self.tsdf.as_mut().expect("just ensured"), &self.depth_maps[slot], mask, &(pose, intrinsics), true, TSDF_SAMPLES_PER_STEP) {
                self.tsdf_preparation = None;
                // 🧹️ This map's three per-pixel buffers (~1.5 MiB for a 320x240 view, ~5 MiB at the
                // admitted 512x512 envelope) are dead the moment it is integrated: fusion already read
                // every map above and nothing downstream reads them again. The slot itself stays so the
                // stage's cursors and counts keep their meaning.
                self.depth_maps[slot] = remodeling_dense::DepthMap::new(0, 0);
                if let Some(mask) = self.consistency_masks.get_mut(slot) {
                    *mask = Vec::new();
                }
                self.stage_cursor += 1;
            }
            return true;
        }
        false
    }

    /// 🧊️ The TSDF voxel size and truncation the meshing stage uses: the configured ones, floored
    /// at [`MIN_MESHING_VOXEL`] and [`MIN_TRUNCATION_VOXELS`] voxels. The reconstruction is meshed in
    /// its normalised frame (points' RMS radius one unit, see `normalize_reconstruction_frame`), so a
    /// configured size is a fraction of the scene there, not millimetres; and the interactive surface
    /// is capped at 512 elements, which a closed surface of unit RMS radius (area ~13–16) only fits
    /// at about a fifth of a unit per net cell. Finer voxels are not more detail: the extraction then
    /// reads every second or fourth sample of a volume whose observed band has more gaps per
    /// sample, and on the synthetic orbit a 0.1 lattice covered half the cube where 0.2 covered 90 %.
    /// ✅️ Cross-view agreement the fusion stage demands of a depth pixel before the TSDF takes it:
    /// [`FUSION_MAX_RELATIVE_DEPTH_DIFF`] and the document's `min-view-consistency` views (counting
    /// the pixel's own). Two views are not enough: a background pixel beside the object's silhouette
    /// takes the object's depth in PatchMatch (its patch holds object texture), two neighbouring views
    /// fatten the silhouette alike and agree, and the pair fused a fin out of the orbit's cube.
    pub fn fusion_config(&self) -> remodeling_dense::FusionConfig {
        remodeling_dense::FusionConfig { max_relative_depth_diff: FUSION_MAX_RELATIVE_DEPTH_DIFF, min_consistent_views: self.params.dense_source_views.max(2), ..remodeling_dense::FusionConfig::default() }
    }

    pub fn meshing_voxel(&self) -> (f64, f64) {
        let voxel_size = self.params.tsdf_voxel_size.max(MIN_MESHING_VOXEL);
        (voxel_size, self.params.tsdf_truncation.max(MIN_TRUNCATION_VOXELS * voxel_size))
    }

    /// 🏗️ Advances the shared extrema walk, texture-view copying and pipeline creation with finite
    /// point/byte fuel.
    pub(super) fn step_begin_meshing(&mut self) -> bool {
        const IMAGE_BYTES_PER_STEP: usize = 4_096;
        if self.tsdf.is_none() {
            return true;
        }
        if !self.step_meshing_bounds() {
            return false;
        }
        let preparation = self.meshing_preparation.as_mut().expect("walked meshing preparation");
        if self.params.texture_enabled {
            if let Some(reconstruction) = &self.reconstruction {
                if preparation.view_cursor < self.dense_camera_indices.len() {
                    let camera = self.dense_camera_indices[preparation.view_cursor];
                    let (frame, pose) = reconstruction.cameras[camera];
                    if preparation.active_view.is_none() {
                        let source = &self.frames[frame].image;
                        preparation.active_view =
                            Some(TextureViewPreparation { pose, intrinsics: reconstruction.intrinsics, image: remodeling_image::ImageRgba8 { width: source.width, height: source.height, data: Vec::with_capacity(source.data.len()) }, cursor: 0 });
                    }
                    let active = preparation.active_view.as_mut().expect("active texture view");
                    let source = &self.frames[frame].image.data;
                    let end = active.cursor.saturating_add(IMAGE_BYTES_PER_STEP).min(source.len());
                    active.image.data.extend_from_slice(&source[active.cursor..end]);
                    active.cursor = end;
                    if end == source.len() {
                        let complete = preparation.active_view.take().expect("completed texture view");
                        preparation.views.push(semio_s_artifact_remodel_remodeling_mesh_atlas::editor::remodeling::engine::mesh::atlas::TextureView { pose: complete.pose, intrinsics: complete.intrinsics, image: complete.image });
                        preparation.view_cursor += 1;
                    }
                    return false;
                }
            }
        }
        let preparation = self.meshing_preparation.take().expect("completed meshing preparation");
        let (bounds_min, bounds_max) = compute_voxel_bounds_from_extrema(preparation.bounds_min, preparation.bounds_max, self.meshing_voxel().0);
        let volume = self.tsdf.take().expect("meshing volume");
        self.mesh_pipeline = Some(semio_s_artifact_remodel_remodeling_mesh::editor::remodeling::engine::mesh::MeshPipeline::new_bounded(volume, 0.0, bounds_min, bounds_max, self.params.mesh.clone()).with_observed_only_surface().with_views(preparation.views));
        // 🧹️ Input pixels are dead here: the texture views above hold their own copies and the mesh
        // stages read only the volume and those views. Ten 320x240 frames are 3 MiB, the admitted
        // envelope (64 frames of 512x512) is 64 MiB — all of it retained through meshing otherwise.
        // Width/height go with the buffer so `data.len() == width * height * 4` still holds.
        for frame in &mut self.frames {
            frame.image = remodeling_image::ImageRgba8 { width: 0, height: 0, data: Vec::new() };
        }
        // 🧹️ The keypoints the terminal observation table needs were already copied into the
        // finalized `observations`, so they are dead by the mesh handoff. The descriptors stay:
        // at 32 bytes each they are under a megabyte for the admitted envelope, and the match
        // oracle (`match_oracle_inputs`) reads them after the run.
        self.keypoints_per_frame = Vec::new();
        true
    }

    /// 🕸️ Drives one cursor-bounded internal mesh unit per call and maps its stage name back onto
    /// [`EngineStage`].
    pub(super) fn step_meshing(&mut self) -> MeshStepOutcome {
        let Some(pipeline) = self.mesh_pipeline.as_mut() else {
            return MeshStepOutcome::Failed("mesh pipeline not initialized".to_string());
        };
        match semio_s_artifact_remodel_remodeling_mesh::editor::remodeling::engine::mesh::mesh_pipeline_step(pipeline, 1) {
            semio_s_artifact_remodel_remodeling_mesh::editor::remodeling::engine::mesh::MeshPipelineStatus::Working { stage, .. } => MeshStepOutcome::Working(mesh_stage_to_engine_stage(stage)),
            semio_s_artifact_remodel_remodeling_mesh::editor::remodeling::engine::mesh::MeshPipelineStatus::Done => MeshStepOutcome::Done,
            semio_s_artifact_remodel_remodeling_mesh::editor::remodeling::engine::mesh::MeshPipelineStatus::Failed(msg) => MeshStepOutcome::Failed(msg),
        }
    }
}

/// 🕸️ Internal result of one `mesh_pipeline_step` call, translated to engine vocabulary.
pub(super) enum MeshStepOutcome {
    Working(EngineStage),
    Done,
    Failed(String),
}
// #endregion 🔖️Pipeline
