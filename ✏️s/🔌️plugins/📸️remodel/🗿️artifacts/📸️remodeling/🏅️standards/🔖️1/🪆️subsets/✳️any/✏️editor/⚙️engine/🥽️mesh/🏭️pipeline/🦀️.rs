//! 🏭️ Pipeline of the mesh surface extraction: the pipeline parameters and status, the stage order and the bounded step that drives the stages to a watertight result or an explicit failure. See [`super`].

use super::*;

// #region 🔖️Pipeline
#[derive(Clone, Debug, PartialEq, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct MeshParams {
    pub guarantee_watertight: bool,
    pub hole_fill_max_boundary_verts: usize,
    pub self_intersection_check: bool,
    pub target_triangles: usize,
    pub taubin_lambda: f64,
    pub taubin_mu: f64,
    pub taubin_iterations: usize,
    pub min_component_faces: usize,
    pub min_component_bbox_fraction: f64,
    pub chart_angle_deg: f64,
    pub atlas_size: u32,
}

impl Default for MeshParams {
    fn default() -> Self {
        Self {
            guarantee_watertight: true,
            hole_fill_max_boundary_verts: 512,
            self_intersection_check: false,
            target_triangles: usize::MAX,
            taubin_lambda: 0.5,
            taubin_mu: -0.53,
            taubin_iterations: 5,
            min_component_faces: 8,
            min_component_bbox_fraction: 0.02,
            chart_angle_deg: 60.0,
            atlas_size: 512,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MeshPipelineStatus {
    Working { stage: &'static str, progress: f32 },
    Done,
    Failed(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Stage {
    Mc,
    Clean,
    Repair,
    OrientConsistency,
    HoleFill,
    Validate1,
    Close,
    Revalidate,
    OrientOutward,
    Taubin,
    Qem,
    Validate2,
    Lscm,
    TextureBake,
    Interchange,
    Done,
}

const STAGE_ORDER: [Stage; 16] = [
    Stage::Mc,
    Stage::Clean,
    Stage::Repair,
    Stage::OrientConsistency,
    Stage::HoleFill,
    Stage::Validate1,
    Stage::Close,
    Stage::Revalidate,
    Stage::OrientOutward,
    Stage::Taubin,
    Stage::Qem,
    Stage::Validate2,
    Stage::Lscm,
    Stage::TextureBake,
    Stage::Interchange,
    Stage::Done,
];

/// 🏗️ Engine-chunkable driver over the full watertight-mesh-out pipeline: MC → Clean → Repair →
/// Orient(consistency) → HoleFill → Validate → `[Close if triggered → re-Validate, must pass]` →
/// Orient(outward) → Taubin → QEM → Validate(light) → LSCM → Texture bake → `to_mesh_data`.
#[derive(semio_framework_value::RetireOwned)]
pub struct MeshPipeline {
    pub(super) mesh: TriMesh,
    tsdf: Option<remodeling_dense::TsdfVolume>,
    extraction: Option<TsdfExtractionPreparation>,
    pub(super) params: MeshParams,
    stage_index: usize,
    close_used: bool,
    tsdf_voxel_size: Option<f64>,
    pub(super) report: Option<WatertightReport>,
    failed: Option<String>,
    pub(super) uvs: Option<Vec<[f32; 2]>>,
    pub(super) texture: Option<remodeling_image::ImageRgba8>,
    pub(super) views: Vec<TextureView>,
    pub(super) result: Option<semio_framework::MeshData>,
    interchange: Option<InterchangePreparation>,
    pub(super) interactive: bool,
    cleaning: Option<BoundedCleanPreparation>,
    repairing: Option<BoundedRepairPreparation>,
    orienting: Option<BoundedOrientPreparation>,
    hole_filling: Option<BoundedHoleFillPreparation>,
    validating: Option<BoundedValidationPreparation>,
    outward: Option<BoundedOutwardPreparation>,
    smoothing: Option<BoundedTaubinPreparation>,
    simplifying: Option<BoundedSimplifyPreparation>,
    unwrapping: Option<BoundedUnwrapPreparation>,
    texturing: Option<BoundedTexturePreparation>,
}

impl MeshPipeline {
    #[cfg(test)]
    pub fn new(vol: &remodeling_dense::TsdfVolume, iso: f64, bounds_min: [i32; 3], bounds_max: [i32; 3], params: MeshParams) -> Self {
        let mesh = extract_tsdf(vol, iso, bounds_min, bounds_max);
        Self {
            mesh,
            tsdf: None,
            extraction: None,
            params,
            stage_index: 0,
            close_used: false,
            tsdf_voxel_size: Some(vol.voxel_size),
            report: None,
            failed: None,
            uvs: None,
            texture: None,
            views: Vec::new(),
            result: None,
            interchange: None,
            interactive: false,
            cleaning: None,
            repairing: None,
            orienting: None,
            hole_filling: None,
            validating: None,
            outward: None,
            smoothing: None,
            simplifying: None,
            unwrapping: None,
            texturing: None,
        }
    }

    pub fn new_bounded(vol: remodeling_dense::TsdfVolume, iso: f64, bounds_min: [i32; 3], bounds_max: [i32; 3], mut params: MeshParams) -> Self {
        let voxel_size = vol.voxel_size;
        params.atlas_size = params.atlas_size.min(64);
        params.taubin_iterations = params.taubin_iterations.min(4);
        params.hole_fill_max_boundary_verts = params.hole_fill_max_boundary_verts.min(128);
        Self {
            mesh: TriMesh::new(),
            tsdf: Some(vol),
            extraction: Some(TsdfExtractionPreparation::new(iso, bounds_min, bounds_max)),
            params,
            stage_index: 0,
            close_used: false,
            tsdf_voxel_size: Some(voxel_size),
            report: None,
            failed: None,
            uvs: None,
            texture: None,
            views: Vec::new(),
            result: None,
            interchange: None,
            interactive: true,
            cleaning: None,
            repairing: None,
            orienting: None,
            hole_filling: None,
            validating: None,
            outward: None,
            smoothing: None,
            simplifying: None,
            unwrapping: None,
            texturing: None,
        }
    }

    #[cfg(test)]
    pub fn from_mesh(mesh: TriMesh, params: MeshParams) -> Self {
        Self {
            mesh,
            tsdf: None,
            extraction: None,
            params,
            stage_index: 1,
            close_used: false,
            tsdf_voxel_size: None,
            report: None,
            failed: None,
            uvs: None,
            texture: None,
            views: Vec::new(),
            result: None,
            interchange: None,
            interactive: false,
            cleaning: None,
            repairing: None,
            orienting: None,
            hole_filling: None,
            validating: None,
            outward: None,
            smoothing: None,
            simplifying: None,
            unwrapping: None,
            texturing: None,
        }
    }

    /// 👁️ Extract only the observed surface and let the hole fill cap it; see
    /// [`TsdfExtractionPreparation::observed_only`].
    pub fn with_observed_only_surface(mut self) -> Self {
        self.extraction = self.extraction.map(TsdfExtractionPreparation::observed_only);
        self
    }

    pub fn with_views(mut self, views: Vec<TextureView>) -> Self {
        self.views = views;
        self
    }

    pub fn report(&self) -> Option<&WatertightReport> {
        self.report.as_ref()
    }

    pub fn result(&self) -> Option<&semio_framework::MeshData> {
        self.result.as_ref()
    }

    pub fn mesh(&self) -> &TriMesh {
        &self.mesh
    }
}

/// 🧩️ Whether the post-repair mesh still needs the guaranteed [`close_voxel`] fallback.
/// [`WatertightReport::is_watertight`] alone is blind to fragmentation: [`fill_holes`] dispatches
/// per boundary *loop*, so a mesh made of hundreds/thousands of small disconnected islands (a
/// badly under-registered SfM/TSDF reconstruction, not a hand-crafted single-shape defect) can
/// have every one of its tiny per-island holes legitimately ear-fanned/DP-triangulated shut —
/// closed, 2-manifold, consistently oriented, and yet still just a swarm of sealed confetti
/// rather than the one coherent solid this pipeline promises. `connected_components > 1` is the
/// fragmentation signal `is_watertight` can't see on its own, so it gates the guarantee here too.
fn needs_close_fallback(report: &WatertightReport) -> bool {
    !report.is_watertight || report.connected_components > 1
}

/// ⚙️ Advances the pipeline through at most `budget` stage dispatches. Interactive pipelines
/// retain explicit extraction, topology, validation, smoothing, unwrap, texture, and interchange
/// cursors; a dispatch consumes only the fixed work allowance of the active cursor.
pub fn mesh_pipeline_step(state: &mut MeshPipeline, budget: usize) -> MeshPipelineStatus {
    for _ in 0..budget.max(1) {
        if state.interactive && (state.mesh.positions.len() > 512 || state.mesh.triangles.len() > 512) {
            let message = format!("interactive mesh envelope exceeded: {} vertices, {} triangles", state.mesh.positions.len(), state.mesh.triangles.len());
            state.failed = Some(message.clone());
            return MeshPipelineStatus::Failed(message);
        }
        if state.stage_index >= STAGE_ORDER.len() {
            return MeshPipelineStatus::Done;
        }
        let stage = STAGE_ORDER[state.stage_index];
        match stage {
            Stage::Mc => {
                if let (Some(extraction), Some(volume)) = (state.extraction.as_mut(), state.tsdf.as_ref()) {
                    let complete = extraction.advance(volume, TSDF_EXTRACTION_POINTS_PER_DISPATCH);
                    if extraction.exceeded() {
                        match extraction.coarsened() {
                            Some(coarser) => {
                                state.extraction = Some(coarser);
                                return MeshPipelineStatus::Working { stage: "marching_cubes", progress: 0.0 };
                            }
                            None => {
                                let message = "interactive mesh envelope exceeded during bounded TSDF extraction".to_string();
                                state.failed = Some(message.clone());
                                return MeshPipelineStatus::Failed(message);
                            }
                        }
                    }
                    if !complete {
                        return MeshPipelineStatus::Working { stage: "marching_cubes", progress: 0.0 };
                    }
                    state.mesh = state.extraction.take().and_then(TsdfExtractionPreparation::finish).expect("completed TSDF extraction");
                    state.tsdf = None;
                }
            }
            Stage::Clean => {
                if state.interactive {
                    let preparation = state.cleaning.get_or_insert_with(|| BoundedCleanPreparation::new(&state.mesh, state.params.min_component_faces, state.params.min_component_bbox_fraction, state.params.guarantee_watertight));
                    if !preparation.advance(&state.mesh, 64) {
                        return MeshPipelineStatus::Working { stage: "clean", progress: state.stage_index as f32 / STAGE_ORDER.len() as f32 };
                    }
                    state.mesh = state.cleaning.take().expect("completed cleanup").finish();
                } else {
                    clean_mesh(&mut state.mesh, state.params.min_component_faces, state.params.min_component_bbox_fraction);
                }
            }
            Stage::Repair => {
                if state.interactive {
                    let preparation = state.repairing.get_or_insert_with(|| BoundedRepairPreparation::new(&state.mesh));
                    if !preparation.advance(&state.mesh, 64) {
                        return MeshPipelineStatus::Working { stage: "repair", progress: state.stage_index as f32 / STAGE_ORDER.len() as f32 };
                    }
                    state.repairing.take().expect("completed repair").finish(&mut state.mesh);
                } else {
                    repair_non_manifold(&mut state.mesh);
                }
            }
            Stage::OrientConsistency => {
                if state.interactive {
                    let preparation = state.orienting.get_or_insert_with(BoundedOrientPreparation::new);
                    if !preparation.advance(&mut state.mesh, 64) {
                        return MeshPipelineStatus::Working { stage: "orient_consistency", progress: state.stage_index as f32 / STAGE_ORDER.len() as f32 };
                    }
                    state.orienting = None;
                } else {
                    let _ = orient_consistently(&mut state.mesh);
                }
            }
            Stage::HoleFill => {
                if state.interactive {
                    let preparation = state.hole_filling.get_or_insert_with(|| BoundedHoleFillPreparation::new(state.params.hole_fill_max_boundary_verts));
                    let complete = preparation.advance(&mut state.mesh, 64);
                    if preparation.exceeded() {
                        let message = "interactive mesh envelope exceeded during bounded hole filling".to_string();
                        state.failed = Some(message.clone());
                        return MeshPipelineStatus::Failed(message);
                    }
                    if !complete {
                        return MeshPipelineStatus::Working { stage: "hole_fill", progress: state.stage_index as f32 / STAGE_ORDER.len() as f32 };
                    }
                    state.hole_filling = None;
                } else {
                    fill_holes(&mut state.mesh, &HoleFillParams { max_boundary_verts: state.params.hole_fill_max_boundary_verts });
                }
            }
            Stage::Validate1 => {
                let report = if state.interactive {
                    let preparation = state.validating.get_or_insert_with(|| BoundedValidationPreparation::new(false));
                    if !preparation.advance(&state.mesh, 64) {
                        return MeshPipelineStatus::Working { stage: "validate", progress: state.stage_index as f32 / STAGE_ORDER.len() as f32 };
                    }
                    state.validating.take().expect("completed validation").finish(&state.mesh)
                } else {
                    validate_watertight(&state.mesh, false)
                };
                let needs_close = state.params.guarantee_watertight && needs_close_fallback(&report);
                state.report = Some(report);
                if !needs_close {
                    state.stage_index = Stage::OrientOutward as usize;
                    continue;
                }
            }
            Stage::Close => {
                if state.interactive {
                    let seen = state.report.as_ref().map_or_else(String::new, |report| {
                        format!(
                            " (validated {} vertices / {} triangles: {} boundary edges in {} loops, {} non-manifold edges, {} non-manifold vertices, {} components, oriented={})",
                            report.vertex_count, report.triangle_count, report.boundary_edge_count, report.boundary_loop_count, report.non_manifold_edge_count, report.non_manifold_vertex_count, report.connected_components, report.consistently_oriented
                        )
                    });
                    let message = format!("interactive mesh requires a bounded watertight input; voxel-close fallback is not admitted{seen}");
                    state.failed = Some(message.clone());
                    return MeshPipelineStatus::Failed(message);
                }
                let (lo, hi) = state.mesh.bbox();
                let diag = norm3(sub3(hi, lo)).max(1e-9);
                let voxel = match state.tsdf_voxel_size {
                    Some(tsdf_voxel) => (2.0 * tsdf_voxel).max(diag / 512.0),
                    None => diag / 64.0,
                };
                state.mesh = close_voxel(&state.mesh, voxel.max(1e-9));
                state.close_used = true;
            }
            Stage::Revalidate => {
                let mut report = if state.interactive {
                    let check = state.params.self_intersection_check;
                    let preparation = state.validating.get_or_insert_with(|| BoundedValidationPreparation::new(check));
                    if !preparation.advance(&state.mesh, 64) {
                        return MeshPipelineStatus::Working { stage: "revalidate", progress: state.stage_index as f32 / STAGE_ORDER.len() as f32 };
                    }
                    state.validating.take().expect("completed revalidation").finish(&state.mesh)
                } else {
                    validate_watertight(&state.mesh, state.params.self_intersection_check)
                };
                report.closed_fallback_used = state.close_used;
                if state.params.guarantee_watertight && !report.is_watertight {
                    state.report = Some(report);
                    state.failed = Some("close_voxel fallback did not yield a watertight mesh".to_string());
                    return MeshPipelineStatus::Failed(state.failed.clone().expect("just set"));
                }
                state.report = Some(report);
            }
            Stage::OrientOutward => {
                if state.interactive {
                    let preparation = state.outward.get_or_insert_with(BoundedOutwardPreparation::new);
                    if !preparation.advance(&mut state.mesh, 64) {
                        return MeshPipelineStatus::Working { stage: "orient_outward", progress: state.stage_index as f32 / STAGE_ORDER.len() as f32 };
                    }
                    state.outward = None;
                } else {
                    orient_outward(&mut state.mesh);
                }
                if let Some(report) = &mut state.report {
                    report.closed_fallback_used = state.close_used;
                    report.signed_volume = if state.interactive { report.signed_volume.abs() } else { state.mesh.signed_volume() };
                }
            }
            Stage::Taubin => {
                if state.interactive {
                    let preparation = state.smoothing.get_or_insert_with(|| BoundedTaubinPreparation::new(state.params.taubin_iterations, state.params.taubin_lambda, state.params.taubin_mu));
                    if !preparation.advance(&mut state.mesh, 64) {
                        return MeshPipelineStatus::Working { stage: "taubin", progress: state.stage_index as f32 / STAGE_ORDER.len() as f32 };
                    }
                    state.smoothing = None;
                } else if state.params.taubin_iterations > 0 {
                    taubin_smooth(&mut state.mesh, state.params.taubin_lambda, state.params.taubin_mu, state.params.taubin_iterations);
                }
            }
            Stage::Qem => {
                if state.interactive && state.params.target_triangles < state.mesh.triangles.len() {
                    let preparation = state.simplifying.get_or_insert_with(|| BoundedSimplifyPreparation::new(&state.mesh, state.params.target_triangles));
                    if !preparation.advance(&state.mesh, 64) {
                        return MeshPipelineStatus::Working { stage: "simplify", progress: state.stage_index as f32 / STAGE_ORDER.len() as f32 };
                    }
                    state.simplifying.take().expect("completed simplification").finish(&mut state.mesh);
                } else if state.params.target_triangles < state.mesh.triangles.len() {
                    simplify_qem(&mut state.mesh, state.params.target_triangles, &SimplifyParams::default());
                }
            }
            Stage::Validate2 => {
                let mut report = if state.interactive {
                    let check = state.params.self_intersection_check;
                    let preparation = state.validating.get_or_insert_with(|| BoundedValidationPreparation::new(check));
                    if !preparation.advance(&state.mesh, 64) {
                        return MeshPipelineStatus::Working { stage: "validate_light", progress: state.stage_index as f32 / STAGE_ORDER.len() as f32 };
                    }
                    state.validating.take().expect("completed light validation").finish(&state.mesh)
                } else {
                    validate_watertight(&state.mesh, state.params.self_intersection_check)
                };
                report.closed_fallback_used = state.close_used;
                if state.interactive && state.params.guarantee_watertight && !report.is_watertight {
                    let message = "bounded mesh postprocess did not preserve a watertight surface".to_string();
                    state.report = Some(report);
                    state.failed = Some(message.clone());
                    return MeshPipelineStatus::Failed(message);
                }
                state.report = Some(report);
            }
            Stage::Lscm => {
                if state.interactive {
                    let preparation = state.unwrapping.get_or_insert_with(|| BoundedUnwrapPreparation::new(&state.mesh));
                    if !preparation.advance(&state.mesh, 64) {
                        return MeshPipelineStatus::Working { stage: "unwrap", progress: state.stage_index as f32 / STAGE_ORDER.len() as f32 };
                    }
                    state.uvs = state.unwrapping.take().map(|preparation| preparation.uvs);
                } else {
                    let charts = segment_charts(&state.mesh, state.params.chart_angle_deg);
                    state.uvs = Some(unwrap_mesh(&mut state.mesh, &charts));
                }
            }
            Stage::TextureBake => {
                if !state.views.is_empty() {
                    if let Some(uvs) = &state.uvs {
                        if state.interactive {
                            let preparation = state.texturing.get_or_insert_with(|| BoundedTexturePreparation::new(state.params.atlas_size));
                            if !preparation.advance(&state.mesh, uvs, &state.views, 64) {
                                return MeshPipelineStatus::Working { stage: "texture_bake", progress: state.stage_index as f32 / STAGE_ORDER.len() as f32 };
                            }
                            state.texture = state.texturing.take().map(BoundedTexturePreparation::finish);
                        } else {
                            state.texture = Some(bake_texture(&state.mesh, uvs, state.params.atlas_size, &state.views));
                        }
                    }
                }
            }
            Stage::Interchange => {
                let preparation = state.interchange.get_or_insert_with(|| InterchangePreparation::new(&state.mesh));
                if state.interactive {
                    if !preparation.advance(&state.mesh, state.uvs.as_deref(), state.texture.as_ref(), 64) {
                        return MeshPipelineStatus::Working { stage: "interchange", progress: state.stage_index as f32 / STAGE_ORDER.len() as f32 };
                    }
                } else {
                    while !preparation.advance(&state.mesh, state.uvs.as_deref(), state.texture.as_ref(), 64) {}
                }
                state.result = state.interchange.take().map(|preparation| preparation.mesh_data);
            }
            Stage::Done => {}
        }
        state.stage_index += 1;
        if state.stage_index >= STAGE_ORDER.len() {
            return MeshPipelineStatus::Done;
        }
    }
    let progress = state.stage_index as f32 / STAGE_ORDER.len() as f32;
    MeshPipelineStatus::Working { stage: stage_name(STAGE_ORDER[state.stage_index]), progress }
}

fn stage_name(stage: Stage) -> &'static str {
    match stage {
        Stage::Mc => "marching_cubes",
        Stage::Clean => "clean",
        Stage::Repair => "repair",
        Stage::OrientConsistency => "orient_consistency",
        Stage::HoleFill => "hole_fill",
        Stage::Validate1 => "validate",
        Stage::Close => "close",
        Stage::Revalidate => "revalidate",
        Stage::OrientOutward => "orient_outward",
        Stage::Taubin => "taubin",
        Stage::Qem => "simplify",
        Stage::Validate2 => "validate_light",
        Stage::Lscm => "unwrap",
        Stage::TextureBake => "texture_bake",
        Stage::Interchange => "interchange",
        Stage::Done => "done",
    }
}
// #endregion 🔖️Pipeline
