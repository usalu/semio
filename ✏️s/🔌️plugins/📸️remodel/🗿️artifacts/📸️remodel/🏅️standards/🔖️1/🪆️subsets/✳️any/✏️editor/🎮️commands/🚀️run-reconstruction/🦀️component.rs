//! 🚀️ Remodel reconstruction as a generation-tagged, bounded continuation.

use crate::artifacts::remodel::mutations::{commit_reconstruction, create_asset, replace_job, CommitReconstruction, ReconstructionAssetCommit};
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::schema::next_remodel_id;
use crate::artifacts::remodel::{
    CameraPosePreview, CameraTrajectory, FrameRef, GeoProducts, ImageAsset, MeshSource, PackedF32, QcReportSnapshot, ReconstructionJob, ReconstructionStage, RemodelMesh, RemodelSnapshot, SparseCloud, WatertightReportSnapshot,
};
use crate::editor::remodel::config::{RemodelConfig, RemodelConfigMutation};
use crate::editor::remodel::engine::images::{BoundedDecodeProgress, BoundedStillDecoder, CompressedChunkRope};
use crate::editor::remodel::engine::{build_engine_params, camera_pose_preview, reconstruction as remodel_engine, watertight_snapshot, RasterPngPreparation, RasterPngProgress};
use base64::Engine as _;
use semio_framework::kernel::{Effect, UiDirtyScope};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, RequestId};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};

//#region 🔖️Constants
pub const ADVANCE_RECONSTRUCTION_ACTION_ID: &str = "advanceReconstruction";
const RECONSTRUCTION_STEP_BUDGET: usize = 1;
const MAX_RECONSTRUCTION_TICKS: u32 = 200_000;
const MAX_LIVE_SESSIONS: usize = 32;
const MAX_STILL_INPUT_BYTES: usize = 1_114_112;
const PREVIEW_CAMERA_LIMIT: usize = 64;
const PREVIEW_POINT_LIMIT: usize = 256;
const TERMINAL_CAMERA_WORK: usize = 64;
const TERMINAL_POINT_WORK: usize = 256;
const TERMINAL_QUALITY_WORK: usize = 256;
const TERMINAL_GEO_WORK: usize = 256;
const MESH_CHUNK_BYTES: usize = 4_096;
//#endregion 🔖️Constants

//#region 🔖️Session
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RequestedStage {
    Full,
    ExtractingFeatures,
    MatchingFeatures,
    EstimatingPoses,
    BundleAdjusting,
    DenseStereo,
    FusingVolume,
    ExtractingSurface,
    CleaningMesh,
    Texturing,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TerminalPhase {
    Sparse,
    Quality,
    Mesh,
    Geo,
    Dsm,
    Dtm,
    Commit,
}

struct TerminalPreparation {
    phase: TerminalPhase,
    camera_cursor: usize,
    point_cursor: usize,
    quality_cursor: usize,
    quality_squared_error_sum: f64,
    quality_observation_count: usize,
    quality_point_indices: BTreeSet<usize>,
    trajectory: Vec<CameraPosePreview>,
    preview_cameras: Vec<CameraPosePreview>,
    sparse_content: ContentPreparation,
    preview_points_base64: String,
    sparse: Option<SparseCloud>,
    qc: Option<QcReportSnapshot>,
    mesh_result: Option<Box<RemodelMesh>>,
    geo_result: Option<GeoProducts>,
    assets: Vec<ReconstructionAssetCommit>,
    watertight: Option<WatertightReportSnapshot>,
    mesh: Option<MeshPreparation>,
    geo_preparation: Option<remodel_engine::TerminalGeoPreparation>,
    geo: Option<remodel_engine::GeoProducts>,
    pending_dtm: Option<crate::editor::remodel::engine::geo::Raster>,
    raster_asset: Option<RasterAssetPreparation>,
    completed_asset_staging_ids: Vec<String>,
}

struct ContentPreparation {
    staging_id: String,
    digest: [u64; 4],
    digest_len: u64,
    chunk_count: u64,
}

impl ContentPreparation {
    fn new(staging_id: String) -> Self {
        Self { staging_id, digest: [0x6c62272e07bb0142, 0x62b821756295c58d, 0x9e3779b185ebca87, 0xc2b2ae3d27d4eb4f], digest_len: 0, chunk_count: 0 }
    }

    fn record(&mut self, bytes: &[u8]) -> Result<u64, Fault> {
        for byte in bytes {
            self.digest_len = self.digest_len.checked_add(1).ok_or_else(|| Fault::from("sparse content digest length overflow").with_retryable(true))?;
            self.digest[0] = (self.digest[0] ^ u64::from(*byte)).wrapping_mul(0x00000100000001b3);
            self.digest[1] = (self.digest[1] ^ self.digest[0].rotate_left(17) ^ self.digest_len).wrapping_mul(0x9e3779b185ebca87);
            self.digest[2] = (self.digest[2] ^ self.digest[1].rotate_left(29) ^ u64::from(*byte)).wrapping_mul(0xc2b2ae3d27d4eb4f);
            self.digest[3] = (self.digest[3] ^ self.digest[2].rotate_left(41) ^ self.digest_len.rotate_left(7)).wrapping_mul(0x165667b19e3779f9);
        }
        let index = self.chunk_count;
        self.chunk_count = self.chunk_count.checked_add(1).ok_or_else(|| Fault::from("sparse content chunk count overflow").with_retryable(true))?;
        Ok(index)
    }

    fn content_id(&self) -> String {
        format!("remodel-asset-{:016x}{:016x}{:016x}{:016x}-{:016x}", self.digest[0], self.digest[1], self.digest[2], self.digest[3], self.digest_len)
    }

    fn handle(&self) -> String {
        crate::artifacts::remodel::remodel_asset_content_handle(&self.content_id(), &self.staging_id, self.chunk_count)
    }
}

struct RasterAssetPreparation {
    encoder: RasterPngPreparation,
    staging_id: String,
    asset_id: String,
}

enum RasterAssetProgress {
    Working,
    Mutation(RemodelMutation),
    Complete(ReconstructionAssetCommit),
    Failed,
}

impl RasterAssetPreparation {
    fn new(raster: crate::editor::remodel::engine::geo::Raster, staging_id: String, asset_id: String) -> Self {
        Self { encoder: RasterPngPreparation::new(raster), staging_id, asset_id }
    }

    fn advance(&mut self) -> RasterAssetProgress {
        match self.encoder.advance(4_096) {
            RasterPngProgress::Working => RasterAssetProgress::Working,
            RasterPngProgress::Chunk(bytes) => {
                let Some(index) = self.encoder.chunk_count().checked_sub(1) else { return RasterAssetProgress::Failed };
                RasterAssetProgress::Mutation(create_asset(
                    crate::artifacts::remodel::remodel_asset_stage_key(&self.staging_id, crate::artifacts::remodel::RemodelAssetContentKind::Raster, index),
                    ImageAsset { mime: "application/vnd.semio.asset-chunk".into(), data: base64::engine::general_purpose::STANDARD.encode(bytes), width: 0, height: 0 },
                ))
            }
            RasterPngProgress::Complete => RasterAssetProgress::Complete(self.commit_asset(&self.encoder.content_id())),
            RasterPngProgress::Failed => RasterAssetProgress::Failed,
        }
    }

    fn commit_asset(&self, content_id: &str) -> ReconstructionAssetCommit {
        ReconstructionAssetCommit {
            id: self.asset_id.clone(),
            asset: ImageAsset { mime: "image/png".into(), data: crate::artifacts::remodel::remodel_asset_content_handle(content_id, &self.staging_id, self.encoder.chunk_count()), width: self.encoder.width(), height: self.encoder.height() },
        }
    }
}

struct MeshPreparation {
    mesh: semio_framework::MeshData,
    staging_id: String,
    field: u8,
    cursor: usize,
    chunk_count: u64,
    digest: [u64; 4],
    digest_len: u64,
}

impl MeshPreparation {
    fn new(mesh: semio_framework::MeshData, staging_id: String) -> Self {
        Self { mesh, staging_id, field: 0, cursor: 0, chunk_count: 0, digest: [0x6c62272e07bb0142, 0x62b821756295c58d, 0x9e3779b185ebca87, 0xc2b2ae3d27d4eb4f], digest_len: 0 }
    }

    fn update_digest(&mut self, bytes: &[u8]) -> Result<(), Fault> {
        for byte in bytes {
            self.digest_len = self.digest_len.checked_add(1).ok_or_else(|| Fault::from("mesh content digest length overflow").with_retryable(true))?;
            self.digest[0] = (self.digest[0] ^ u64::from(*byte)).wrapping_mul(0x00000100000001b3);
            self.digest[1] = (self.digest[1] ^ self.digest[0].rotate_left(17) ^ self.digest_len).wrapping_mul(0x9e3779b185ebca87);
            self.digest[2] = (self.digest[2] ^ self.digest[1].rotate_left(29) ^ u64::from(*byte)).wrapping_mul(0xc2b2ae3d27d4eb4f);
            self.digest[3] = (self.digest[3] ^ self.digest[2].rotate_left(41) ^ self.digest_len.rotate_left(7)).wrapping_mul(0x165667b19e3779f9);
        }
        Ok(())
    }

    fn content_id(&self) -> String {
        format!("remodel-mesh-{:016x}{:016x}{:016x}{:016x}-{:016x}", self.digest[0], self.digest[1], self.digest[2], self.digest[3], self.digest_len)
    }

    fn next_chunk(&mut self) -> Result<Option<Vec<u8>>, Fault> {
        while self.field <= 11 {
            let mut bytes = Vec::with_capacity(MESH_CHUNK_BYTES);
            bytes.push(self.field);
            let complete = match self.field {
                0 => append_f32_mesh_chunk(&self.mesh.positions, &mut self.cursor, &mut bytes)?,
                1 => append_f32_mesh_chunk(&self.mesh.normals, &mut self.cursor, &mut bytes)?,
                2 => append_f32_mesh_chunk(&self.mesh.colors, &mut self.cursor, &mut bytes)?,
                3 => append_u32_mesh_chunk(&self.mesh.indices, &mut self.cursor, &mut bytes)?,
                4 => append_f32_mesh_chunk(&self.mesh.uvs, &mut self.cursor, &mut bytes)?,
                5 => append_u32_mesh_chunk(&self.mesh.face_ids, &mut self.cursor, &mut bytes)?,
                6 => append_u32_mesh_chunk(&self.mesh.vertex_ids, &mut self.cursor, &mut bytes)?,
                7 => append_f32_mesh_chunk(&self.mesh.edge_positions, &mut self.cursor, &mut bytes)?,
                8 => append_u32_mesh_chunk(&self.mesh.edge_ids, &mut self.cursor, &mut bytes)?,
                9 => append_f32_mesh_chunk(&self.mesh.edge_uvs, &mut self.cursor, &mut bytes)?,
                10 => append_u8_mesh_chunk(&self.mesh.edge_is_seam, &mut self.cursor, &mut bytes)?,
                11 => append_text_mesh_chunk(self.mesh.paint_texture_base64.as_deref().unwrap_or(""), &mut self.cursor, &mut bytes)?,
                _ => true,
            };
            if complete {
                self.field = self.field.checked_add(1).ok_or_else(|| Fault::from("mesh field cursor overflow").with_retryable(true))?;
                self.cursor = 0;
            }
            if bytes.len() > 1 {
                self.update_digest(&bytes)?;
                self.chunk_count = self.chunk_count.checked_add(1).ok_or_else(|| Fault::from("mesh content chunk count overflow").with_retryable(true))?;
                return Ok(Some(bytes));
            }
        }
        Ok(None)
    }
}

fn append_f32_mesh_chunk(values: &[f32], cursor: &mut usize, output: &mut Vec<u8>) -> Result<bool, Fault> {
    let end = cursor.checked_add((MESH_CHUNK_BYTES - 1) / 4).ok_or_else(|| Fault::from("mesh f32 cursor overflow").with_retryable(true))?.min(values.len());
    output.extend(values[*cursor..end].iter().flat_map(|value| value.to_le_bytes()));
    *cursor = end;
    Ok(end == values.len())
}

fn append_u32_mesh_chunk(values: &[u32], cursor: &mut usize, output: &mut Vec<u8>) -> Result<bool, Fault> {
    let end = cursor.checked_add((MESH_CHUNK_BYTES - 1) / 4).ok_or_else(|| Fault::from("mesh u32 cursor overflow").with_retryable(true))?.min(values.len());
    output.extend(values[*cursor..end].iter().flat_map(|value| value.to_le_bytes()));
    *cursor = end;
    Ok(end == values.len())
}

fn append_u8_mesh_chunk(values: &[u8], cursor: &mut usize, output: &mut Vec<u8>) -> Result<bool, Fault> {
    let end = cursor.checked_add(MESH_CHUNK_BYTES - 1).ok_or_else(|| Fault::from("mesh u8 cursor overflow").with_retryable(true))?.min(values.len());
    output.extend_from_slice(&values[*cursor..end]);
    *cursor = end;
    Ok(end == values.len())
}

fn append_text_mesh_chunk(value: &str, cursor: &mut usize, output: &mut Vec<u8>) -> Result<bool, Fault> {
    let mut end = cursor.checked_add(MESH_CHUNK_BYTES - 1).ok_or_else(|| Fault::from("mesh text cursor overflow").with_retryable(true))?.min(value.len());
    while end > *cursor && !value.is_char_boundary(end) {
        end -= 1;
    }
    output.extend_from_slice(&value.as_bytes()[*cursor..end]);
    *cursor = end;
    Ok(end == value.len())
}

struct ReconstructionSession {
    job_id: String,
    artifact_authority: String,
    engine: remodel_engine::ReconstructionEngine,
    texture_size: u32,
    gcp_count: usize,
    requested_stage: RequestedStage,
    stream_index: u32,
    frame_index: u32,
    ingestion: Option<FrameIngestion>,
    tick: u32,
    terminal: Option<TerminalPreparation>,
}

struct FrameIngestion {
    _asset_identity: String,
    mime: String,
    frame_index: u32,
    timestamp_ms: f64,
    compressed: CompressedChunkRope,
    decoder: Option<BoundedStillDecoder>,
    decoded_image: Option<crate::editor::remodel::engine::images::ImageRgba8>,
    sharpness_cursor: usize,
    sharpness_sum: f64,
}

fn frame_ingestion(scene: &RemodelSnapshot, frame: &FrameRef) -> Option<FrameIngestion> {
    let source = crate::artifacts::remodel::remodel_asset_chunk_source(scene, &frame.asset_id)?;
    let compressed = CompressedChunkRope::from_leaves(source.leaves, MAX_STILL_INPUT_BYTES).ok()?;
    Some(FrameIngestion { _asset_identity: source.identity, mime: source.mime, frame_index: frame.index, timestamp_ms: frame.timestamp_ms, compressed, decoder: None, decoded_image: None, sharpness_cursor: 0, sharpness_sum: 0.0 })
}

fn rgba_luma(image: &crate::editor::remodel::engine::images::ImageRgba8, index: usize) -> f64 {
    let offset = index * 4;
    (0.299 * f64::from(image.data[offset]) + 0.587 * f64::from(image.data[offset + 1]) + 0.114 * f64::from(image.data[offset + 2])) / 255.0
}

fn advance_frame_sharpness(ingestion: &mut FrameIngestion) -> bool {
    let image = ingestion.decoded_image.as_ref().expect("decoded image admission");
    let pixels = image.width as usize * image.height as usize;
    let end = ingestion.sharpness_cursor.saturating_add(4_096).min(pixels);
    let width = image.width as usize;
    for index in ingestion.sharpness_cursor..end {
        let x = index % width;
        let y = index / width;
        let mirror = |coordinate: i64, length: usize| -> usize {
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
        };
        let sample = |dx: i64, dy: i64| rgba_luma(image, mirror(y as i64 + dy, image.height as usize) * width + mirror(x as i64 + dx, width));
        let gx = (3.0 * (sample(1, -1) - sample(-1, -1)) + 10.0 * (sample(1, 0) - sample(-1, 0)) + 3.0 * (sample(1, 1) - sample(-1, 1))) / 32.0;
        let gy = (3.0 * (sample(-1, 1) - sample(-1, -1)) + 10.0 * (sample(0, 1) - sample(0, -1)) + 3.0 * (sample(1, 1) - sample(1, -1))) / 32.0;
        ingestion.sharpness_sum += gx * gx + gy * gy;
    }
    ingestion.sharpness_cursor = end;
    end == pixels
}

#[derive(Default)]
struct ReconstructionSessions {
    admitted: BTreeSet<u64>,
    live: BTreeMap<u64, ReconstructionSession>,
}

impl ReconstructionSessions {
    fn admit(&mut self, generation: u64, job_id: &str) -> Result<Vec<ReconstructionSession>, ()> {
        let superseded = self.live.iter().filter_map(|(candidate_generation, session)| (session.job_id == job_id).then_some(*candidate_generation)).collect::<Vec<_>>();
        let mut removed = Vec::with_capacity(superseded.len());
        for candidate_generation in superseded {
            if let Some(session) = self.live.remove(&candidate_generation) {
                removed.push(session);
            }
            self.admitted.remove(&candidate_generation);
        }
        if self.admitted.len() >= MAX_LIVE_SESSIONS {
            return Err(());
        }
        self.admitted.insert(generation);
        Ok(removed)
    }
}

static RECONSTRUCTION_SESSIONS: OnceLock<Mutex<ReconstructionSessions>> = OnceLock::new();
static NEXT_RECONSTRUCTION_GENERATION: AtomicU64 = AtomicU64::new(1);
static NEXT_RECONSTRUCTION_REQUEST: AtomicU64 = AtomicU64::new(10_000);

fn sessions() -> &'static Mutex<ReconstructionSessions> {
    RECONSTRUCTION_SESSIONS.get_or_init(|| Mutex::new(ReconstructionSessions::default()))
}

fn take_session(generation: u64) -> Option<ReconstructionSession> {
    sessions().lock().expect("remodel reconstruction sessions lock").live.remove(&generation)
}

fn admit_session(generation: u64, job_id: &str) -> bool {
    let removed = {
        let mut sessions = sessions().lock().expect("remodel reconstruction sessions lock");
        sessions.admit(generation, job_id)
    };
    let Ok(removed) = removed else { return false };
    for session in &removed {
        discard_session_staging(session);
    }
    true
}

fn put_session(generation: u64, session: ReconstructionSession) {
    let mut sessions = sessions().lock().expect("remodel reconstruction sessions lock");
    if sessions.admitted.contains(&generation) {
        sessions.live.insert(generation, session);
    }
}

fn cancel_session(generation: u64) {
    let removed = {
        let mut sessions = sessions().lock().expect("remodel reconstruction sessions lock");
        let removed = sessions.live.remove(&generation);
        sessions.admitted.remove(&generation);
        removed
    };
    if let Some(session) = removed.as_ref() {
        discard_session_staging(session);
    }
}

fn complete_session(generation: u64) {
    cancel_session(generation);
}

fn discard_terminal_staging(terminal: &TerminalPreparation) {
    if let Some(staging_id) = terminal.mesh.as_ref().map(|mesh| mesh.staging_id.as_str()) {
        crate::artifacts::remodel::discard_staged_remodel_mesh(staging_id);
    }
    if let Some(staging_id) = terminal.raster_asset.as_ref().map(|asset| asset.staging_id.as_str()) {
        crate::artifacts::remodel::discard_staged_remodel_asset(staging_id);
    }
    for staging_id in &terminal.completed_asset_staging_ids {
        crate::artifacts::remodel::discard_staged_remodel_asset(staging_id);
    }
    if terminal.sparse_content.chunk_count > 0 {
        crate::artifacts::remodel::discard_staged_remodel_asset(&terminal.sparse_content.staging_id);
    }
}

fn discard_session_staging(session: &ReconstructionSession) {
    if let Some(terminal) = &session.terminal {
        discard_terminal_staging(terminal);
    }
}

/// 🛑️ Cancels every worker-portable continuation for the current document generation and
/// emits replayable bounded cleanup for any privately staged mesh chunks.
pub fn cancel_current_reconstruction(scene: &RemodelSnapshot) -> Emit<RemodelMutation, RemodelConfigMutation> {
    let cancelled_sessions = {
        let mut sessions = sessions().lock().expect("remodel reconstruction sessions lock");
        let cancelled = sessions.live.iter().filter_map(|(generation, session)| (session.job_id == scene.job.id).then_some(*generation)).collect::<Vec<_>>();
        let mut removed = Vec::with_capacity(cancelled.len());
        for generation in cancelled {
            let Some(session) = sessions.live.remove(&generation) else { continue };
            sessions.admitted.remove(&generation);
            removed.push(session);
        }
        removed
    };
    let mut job = scene.job.clone();
    job.cancel_requested = true;
    let artifact_mutations = vec![replace_job(job)];
    for session in &cancelled_sessions {
        discard_session_staging(session);
    }
    Emit { artifact_mutations, coalesce_key: Some(format!("reconstruction-cancel:{}", scene.job.id)), ui_scope: UiDirtyScope::Full, ..Default::default() }
}

impl RequestedStage {
    fn parse(value: &str) -> Option<Self> {
        match value {
            "full" | "done" => Some(Self::Full),
            "extracting-features" => Some(Self::ExtractingFeatures),
            "matching-features" => Some(Self::MatchingFeatures),
            "estimating-poses" => Some(Self::EstimatingPoses),
            "bundle-adjusting" => Some(Self::BundleAdjusting),
            "dense-stereo" => Some(Self::DenseStereo),
            "fusing-volume" => Some(Self::FusingVolume),
            "extracting-surface" => Some(Self::ExtractingSurface),
            "cleaning-mesh" => Some(Self::CleaningMesh),
            "texturing" => Some(Self::Texturing),
            _ => None,
        }
    }

    fn wire(self) -> &'static str {
        match self {
            Self::Full => "full",
            Self::ExtractingFeatures => "extracting-features",
            Self::MatchingFeatures => "matching-features",
            Self::EstimatingPoses => "estimating-poses",
            Self::BundleAdjusting => "bundle-adjusting",
            Self::DenseStereo => "dense-stereo",
            Self::FusingVolume => "fusing-volume",
            Self::ExtractingSurface => "extracting-surface",
            Self::CleaningMesh => "cleaning-mesh",
            Self::Texturing => "texturing",
        }
    }

    fn engine(self) -> remodel_engine::EngineStage {
        match self {
            Self::Full => remodel_engine::EngineStage::Done,
            Self::ExtractingFeatures => remodel_engine::EngineStage::ExtractingFeatures,
            Self::MatchingFeatures => remodel_engine::EngineStage::MatchingFeatures,
            Self::EstimatingPoses => remodel_engine::EngineStage::EstimatingPoses,
            Self::BundleAdjusting => remodel_engine::EngineStage::BundleAdjusting,
            Self::DenseStereo => remodel_engine::EngineStage::DenseStereo,
            Self::FusingVolume => remodel_engine::EngineStage::FusingVolume,
            Self::ExtractingSurface => remodel_engine::EngineStage::ExtractingSurface,
            Self::CleaningMesh => remodel_engine::EngineStage::CleaningMesh,
            Self::Texturing => remodel_engine::EngineStage::Texturing,
        }
    }

    fn needs_terminal_products(self) -> bool {
        matches!(self, Self::Full | Self::Texturing)
    }
}

fn engine_stage_rank(stage: remodel_engine::EngineStage) -> usize {
    match stage {
        remodel_engine::EngineStage::Idle => 0,
        remodel_engine::EngineStage::ExtractingFeatures => 1,
        remodel_engine::EngineStage::MatchingFeatures => 2,
        remodel_engine::EngineStage::EstimatingPoses => 3,
        remodel_engine::EngineStage::BundleAdjusting => 4,
        remodel_engine::EngineStage::DenseStereo => 5,
        remodel_engine::EngineStage::FusingVolume => 6,
        remodel_engine::EngineStage::ExtractingSurface => 7,
        remodel_engine::EngineStage::CleaningMesh => 8,
        remodel_engine::EngineStage::Texturing => 9,
        remodel_engine::EngineStage::Done | remodel_engine::EngineStage::Failed => 10,
    }
}

fn requested_stage_complete(requested: RequestedStage, current: remodel_engine::EngineStage) -> bool {
    requested != RequestedStage::Full && engine_stage_rank(current) > engine_stage_rank(requested.engine())
}

fn terminal_phase_wire(phase: TerminalPhase) -> &'static str {
    match phase {
        TerminalPhase::Sparse => "terminal-sparse",
        TerminalPhase::Quality => "terminal-quality",
        TerminalPhase::Mesh => "terminal-mesh",
        TerminalPhase::Geo => "terminal-geo",
        TerminalPhase::Dsm => "terminal-dsm",
        TerminalPhase::Dtm => "terminal-dtm",
        TerminalPhase::Commit => "terminal-commit",
    }
}

fn checkpoint(generation: u64, session: &ReconstructionSession) -> AdvanceReconstruction {
    let (phase, terminal_cursor) = session.terminal.as_ref().map_or(("pipeline", 0), |terminal| {
        let cursor = match terminal.phase {
            TerminalPhase::Sparse => terminal.camera_cursor.max(terminal.point_cursor) as u64,
            TerminalPhase::Quality => terminal.quality_cursor as u64,
            TerminalPhase::Geo => terminal.geo_preparation.as_ref().map_or(0, |preparation| preparation.cursor as u64),
            _ => 0,
        };
        (terminal_phase_wire(terminal.phase), cursor)
    });
    AdvanceReconstruction {
        generation,
        job_id: session.job_id.clone(),
        requested_stage: session.requested_stage.wire().into(),
        phase: phase.into(),
        stream_index: session.stream_index,
        frame_index: session.frame_index,
        terminal_cursor,
        tick: session.tick,
    }
}
//#endregion 🔖️Session

//#region 🔖️Continuation
fn queue(payload: &AdvanceReconstruction) -> Effect {
    Effect::DispatchAction {
        req: RequestId(NEXT_RECONSTRUCTION_REQUEST.fetch_add(1, Ordering::Relaxed)),
        action: ADVANCE_RECONSTRUCTION_ACTION_ID.into(),
        args: semio_framework::optional_json_to_dsl(Some(json!({
            "generation": payload.generation,
            "jobId": payload.job_id,
            "requestedStage": payload.requested_stage,
            "phase": payload.phase,
            "streamIndex": payload.stream_index,
            "frameIndex": payload.frame_index,
            "terminalCursor": payload.terminal_cursor,
            "tick": payload.tick,
        }))),
        delay_ms: 0,
    }
}

fn emit_step(job: ReconstructionJob, generation: u64, next: Option<AdvanceReconstruction>) -> Emit<RemodelMutation, RemodelConfigMutation> {
    Emit { artifact_mutations: vec![replace_job(job)], coalesce_key: Some(format!("reconstruction:{generation}")), effects: next.as_ref().map(queue).into_iter().collect(), ui_scope: UiDirtyScope::Full, ..Default::default() }
}

fn next_frame_cursor(scene: &RemodelSnapshot, stream_index: u32, frame_index: u32) -> Result<(u32, u32), Fault> {
    let stream_cursor = usize::try_from(stream_index).map_err(|_| Fault::from("stream cursor conversion overflow").with_retryable(true))?;
    let Some(stream) = scene.streams.get(stream_cursor) else { return Ok((stream_index, frame_index)) };
    let next_frame = frame_index.checked_add(1).ok_or_else(|| Fault::from("frame cursor overflow").with_retryable(true))?;
    let frame_count = u32::try_from(stream.frames.len()).map_err(|_| Fault::from("frame count overflow").with_retryable(true))?;
    if next_frame < frame_count {
        Ok((stream_index, next_frame))
    } else {
        Ok((stream_index.checked_add(1).ok_or_else(|| Fault::from("stream cursor overflow").with_retryable(true))?, 0))
    }
}

fn preview_job(job_id: String, stage: ReconstructionStage, progress: f32, stage_cursor: u32, engine: &remodel_engine::ReconstructionEngine) -> ReconstructionJob {
    let preview = engine.sparse_preview_bounded(PREVIEW_CAMERA_LIMIT, PREVIEW_POINT_LIMIT);
    let mut camera_poses_preview = Vec::with_capacity(preview.camera_poses.len());
    for (index, pose) in preview.camera_poses.iter().enumerate() {
        camera_poses_preview.push(camera_pose_preview(index as u32, pose));
    }
    ReconstructionJob { id: job_id, stage, progress_0_1: progress, cancel_requested: false, stage_cursor, started_at_ms: None, error: None, camera_poses_preview, sparse_point_cloud_preview: PackedF32::from_f32_slice(&preview.packed_points) }
}

fn reconstruction_stage(stage: remodel_engine::EngineStage) -> ReconstructionStage {
    match stage {
        remodel_engine::EngineStage::Idle => ReconstructionStage::Ingesting,
        remodel_engine::EngineStage::ExtractingFeatures => ReconstructionStage::ExtractingFeatures,
        remodel_engine::EngineStage::MatchingFeatures => ReconstructionStage::MatchingFeatures,
        remodel_engine::EngineStage::EstimatingPoses => ReconstructionStage::EstimatingPoses,
        remodel_engine::EngineStage::BundleAdjusting => ReconstructionStage::BundleAdjusting,
        remodel_engine::EngineStage::DenseStereo => ReconstructionStage::DenseStereo,
        remodel_engine::EngineStage::FusingVolume => ReconstructionStage::FusingVolume,
        remodel_engine::EngineStage::ExtractingSurface => ReconstructionStage::ExtractingSurface,
        remodel_engine::EngineStage::CleaningMesh => ReconstructionStage::CleaningMesh,
        remodel_engine::EngineStage::Texturing => ReconstructionStage::Texturing,
        remodel_engine::EngineStage::Done => ReconstructionStage::Done,
        remodel_engine::EngineStage::Failed => ReconstructionStage::Failed,
    }
}

fn terminal_preparation(generation: u64, artifact_authority: &str) -> TerminalPreparation {
    TerminalPreparation {
        phase: TerminalPhase::Sparse,
        camera_cursor: 0,
        point_cursor: 0,
        quality_cursor: 0,
        quality_squared_error_sum: 0.0,
        quality_observation_count: 0,
        quality_point_indices: BTreeSet::new(),
        trajectory: Vec::new(),
        preview_cameras: Vec::new(),
        sparse_content: ContentPreparation::new(format!("{artifact_authority}:sparse:{generation}")),
        preview_points_base64: String::new(),
        sparse: None,
        qc: None,
        mesh_result: None,
        geo_result: None,
        assets: Vec::new(),
        watertight: None,
        mesh: None,
        geo_preparation: None,
        geo: None,
        pending_dtm: None,
        raster_asset: None,
        completed_asset_staging_ids: Vec::new(),
    }
}
//#endregion 🔖️Continuation

//#region 🔖️Run
/// 🌱️ Starts a fresh generation and schedules ingestion; it performs no pipeline work itself.
pub fn begin_reconstruction(doc: &ArtifactView<'_, RemodelSnapshot>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    begin_requested_reconstruction(doc, RequestedStage::Full)
}

/// 🎯️ Starts a fresh dependency-prefix generation ending at the requested pipeline stage.
pub fn begin_stage_reconstruction(doc: &ArtifactView<'_, RemodelSnapshot>, requested_stage: &str) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    let Some(requested_stage) = RequestedStage::parse(requested_stage) else { return Ok(Emit::default()) };
    begin_requested_reconstruction(doc, requested_stage)
}

fn begin_requested_reconstruction(doc: &ArtifactView<'_, RemodelSnapshot>, requested_stage: RequestedStage) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    let scene = doc.snapshot;
    if scene.streams.iter().all(|stream| stream.frames.is_empty()) {
        return Ok(Emit::default());
    }
    let generation = NEXT_RECONSTRUCTION_GENERATION.fetch_add(1, Ordering::Relaxed);
    let job_id = next_remodel_id("job");
    let engine_params = build_engine_params(&scene.params);
    let session = ReconstructionSession {
        job_id: job_id.clone(),
        artifact_authority: doc.operation_optional().map_or_else(
            || format!("document={}:app=test:operation={job_id}:generation={generation}", scene.id),
            |operation| format!("document={}:app={}:operation={}:generation={}", operation.parent_document_id, operation.app_instance_id, operation.operation_id, operation.generation),
        ),
        engine: remodel_engine::ReconstructionEngine::new(&engine_params),
        texture_size: scene.params.mesh.texture_size,
        gcp_count: scene.gcps.len(),
        requested_stage,
        stream_index: 0,
        frame_index: 0,
        ingestion: None,
        tick: 0,
        terminal: None,
    };
    let next = checkpoint(generation, &session);
    let mut job = ReconstructionJob {
        id: job_id.clone(),
        stage: ReconstructionStage::Ingesting,
        progress_0_1: 0.0,
        cancel_requested: false,
        stage_cursor: 0,
        started_at_ms: None,
        error: None,
        camera_poses_preview: Vec::new(),
        sparse_point_cloud_preview: PackedF32::default(),
    };
    if !admit_session(generation, &scene.job.id) {
        job.stage = ReconstructionStage::Failed;
        job.error = Some(format!("Interactive reconstruction capacity is {MAX_LIVE_SESSIONS} active jobs; cancel one before retrying."));
        return Ok(emit_step(job, generation, None));
    }
    put_session(generation, session);
    Ok(emit_step(job, generation, Some(next)))
}

fn packed_chunk_base64(values: &[f32]) -> String {
    let bytes: Vec<u8> = values.iter().flat_map(|value| value.to_le_bytes()).collect();
    base64::engine::general_purpose::STANDARD.encode(bytes)
}

fn terminal_progress(phase: TerminalPhase) -> f32 {
    match phase {
        TerminalPhase::Sparse => 0.91,
        TerminalPhase::Quality => 0.93,
        TerminalPhase::Mesh => 0.95,
        TerminalPhase::Geo => 0.96,
        TerminalPhase::Dsm => 0.97,
        TerminalPhase::Dtm => 0.98,
        TerminalPhase::Commit => 0.99,
    }
}

fn yield_terminal(generation: u64, session: ReconstructionSession, mutation: Option<RemodelMutation>) -> Emit<RemodelMutation, RemodelConfigMutation> {
    let terminal = session.terminal.as_ref().expect("terminal preparation present");
    let job = ReconstructionJob {
        id: session.job_id.clone(),
        stage: ReconstructionStage::ReportingQc,
        progress_0_1: terminal_progress(terminal.phase),
        cancel_requested: false,
        stage_cursor: session.tick,
        started_at_ms: None,
        error: None,
        camera_poses_preview: terminal.preview_cameras.clone(),
        sparse_point_cloud_preview: PackedF32(terminal.preview_points_base64.clone()),
    };
    let next = checkpoint(generation, &session);
    put_session(generation, session);
    let artifact_mutations = vec![mutation.unwrap_or_else(|| replace_job(job))];
    Emit { artifact_mutations, coalesce_key: Some(format!("reconstruction:{generation}")), effects: vec![queue(&next)], ui_scope: UiDirtyScope::Full, ..Default::default() }
}

fn advance_terminal(generation: u64, mut session: ReconstructionSession) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    let mut terminal = session.terminal.take().expect("terminal preparation present");
    let mut step_mutation = None;
    match terminal.phase {
        TerminalPhase::Sparse => {
            let chunk = session.engine.terminal_sparse_chunk(terminal.camera_cursor, terminal.point_cursor, TERMINAL_CAMERA_WORK, TERMINAL_POINT_WORK);
            for (offset, pose) in chunk.camera_poses.iter().enumerate() {
                let preview = camera_pose_preview((terminal.camera_cursor + offset) as u32, pose);
                if terminal.preview_cameras.len() < PREVIEW_CAMERA_LIMIT {
                    terminal.preview_cameras.push(preview.clone());
                }
                terminal.trajectory.push(preview);
            }
            let bytes = chunk.packed_points.iter().flat_map(|value| value.to_le_bytes()).collect::<Vec<_>>();
            if !bytes.is_empty() {
                let index = terminal.sparse_content.record(&bytes)?;
                step_mutation = Some(create_asset(
                    crate::artifacts::remodel::remodel_asset_stage_key(&terminal.sparse_content.staging_id, crate::artifacts::remodel::RemodelAssetContentKind::Sparse, index),
                    ImageAsset { mime: "application/vnd.semio.asset-chunk".into(), data: base64::engine::general_purpose::STANDARD.encode(bytes), width: 0, height: 0 },
                ));
            }
            if terminal.point_cursor < PREVIEW_POINT_LIMIT {
                let remaining = PREVIEW_POINT_LIMIT - terminal.point_cursor;
                let component_limit = remaining.saturating_mul(3).min(chunk.packed_points.len());
                terminal.preview_points_base64.push_str(&packed_chunk_base64(&chunk.packed_points[..component_limit]));
            }
            terminal.camera_cursor = chunk.next_camera;
            terminal.point_cursor = chunk.next_point;
            if chunk.complete {
                let points = if terminal.sparse_content.chunk_count == 0 { PackedF32::default() } else { PackedF32(terminal.sparse_content.handle()) };
                terminal.sparse = Some(SparseCloud { points, colors: None });
                if !terminal.trajectory.is_empty() {
                    terminal.trajectory = std::mem::take(&mut terminal.trajectory);
                }
                terminal.phase = TerminalPhase::Quality;
            }
        }
        TerminalPhase::Quality => {
            let chunk = session.engine.terminal_quality_chunk(terminal.quality_cursor, TERMINAL_QUALITY_WORK);
            terminal.quality_cursor = chunk.next_observation;
            terminal.quality_squared_error_sum += chunk.squared_error_sum;
            terminal.quality_observation_count = terminal.quality_observation_count.checked_add(chunk.observation_count).ok_or_else(|| Fault::from("quality observation count overflow").with_retryable(true))?;
            terminal.quality_point_indices.extend(chunk.point_indices);
            if chunk.complete {
                terminal.watertight = match session.engine.terminal_watertight_report().as_ref() {
                    Some(report) => Some(watertight_snapshot(report)),
                    None => None,
                };
                let accepted_count = session.engine.frame_source().accepted_count();
                let mut warnings = Vec::new();
                if terminal.watertight.as_ref().is_some_and(|report| !report.is_watertight) {
                    warnings.push("Mesh is not watertight.".into());
                }
                if session.gcp_count > 0 {
                    warnings.push("Ground control points are set but checkpoint RMSE is not yet computed.".into());
                }
                let reprojection_rms_px = if terminal.quality_observation_count == 0 { 0.0 } else { (terminal.quality_squared_error_sum / terminal.quality_observation_count as f64).sqrt() };
                let mean_track_length = if terminal.quality_point_indices.is_empty() { 0.0 } else { terminal.quality_observation_count as f32 / terminal.quality_point_indices.len() as f32 };
                let qc = QcReportSnapshot {
                    reprojection_rms_px,
                    gcp_checkpoint_rmse: None,
                    watertight: terminal.watertight.clone(),
                    mean_track_length,
                    registered_frame_ratio: if accepted_count == 0 { 0.0 } else { terminal.camera_cursor as f32 / accepted_count as f32 },
                    dense_coverage_ratio: 0.0,
                    warnings,
                };
                terminal.qc = Some(qc);
                terminal.phase = TerminalPhase::Mesh;
            }
        }
        TerminalPhase::Mesh => {
            if terminal.mesh.is_none() {
                if let Some(mesh) = session.engine.take_mesh() {
                    terminal.mesh = Some(MeshPreparation::new(mesh, format!("{}:mesh:{generation}", session.artifact_authority)));
                } else {
                    terminal.phase = TerminalPhase::Geo;
                }
            }
            if let Some(preparation) = terminal.mesh.as_mut() {
                if let Some(chunk) = preparation.next_chunk()? {
                    let index = preparation.chunk_count.checked_sub(1).ok_or_else(|| Fault::from("mesh chunk index underflow"))?;
                    step_mutation = Some(create_asset(
                        crate::artifacts::remodel::remodel_mesh_stage_asset_key(&preparation.staging_id, index),
                        ImageAsset { mime: "application/vnd.semio.mesh-chunk".into(), data: base64::engine::general_purpose::STANDARD.encode(chunk), width: 0, height: 0 },
                    ));
                } else {
                    let content_id = preparation.content_id();
                    let handle = crate::artifacts::remodel::staged_remodel_mesh_handle(&content_id, &preparation.staging_id);
                    terminal.mesh_result = Some(Box::new(RemodelMesh { mesh: handle, source: MeshSource::Reconstructed, texture_asset_id: None, watertight: terminal.watertight.take() }));
                    terminal.phase = TerminalPhase::Geo;
                }
            }
        }
        TerminalPhase::Geo => {
            if terminal.geo_preparation.is_none() {
                terminal.geo_preparation = session.engine.begin_terminal_geo();
                if terminal.geo_preparation.is_none() {
                    terminal.phase = TerminalPhase::Commit;
                }
            } else if session.engine.advance_terminal_geo(terminal.geo_preparation.as_mut().expect("terminal geo preparation"), TERMINAL_GEO_WORK) {
                terminal.geo = remodel_engine::ReconstructionEngine::finish_terminal_geo(terminal.geo_preparation.take().expect("completed terminal geo preparation"));
                terminal.phase = if terminal.geo.is_some() { TerminalPhase::Dsm } else { TerminalPhase::Commit };
            }
        }
        TerminalPhase::Dsm => {
            if terminal.raster_asset.is_none() {
                let geo = terminal.geo.take().expect("geo products retained through raster phases");
                terminal.pending_dtm = Some(geo.dtm);
                terminal.raster_asset = Some(RasterAssetPreparation::new(geo.dsm, format!("{}:dsm:{generation}", session.artifact_authority), format!("geo-dsm-{}", session.job_id)));
            }
            match terminal.raster_asset.as_mut().expect("DSM preparation").advance() {
                RasterAssetProgress::Working => {}
                RasterAssetProgress::Mutation(mutation) => step_mutation = Some(mutation),
                RasterAssetProgress::Complete(asset) => {
                    terminal.completed_asset_staging_ids.push(terminal.raster_asset.as_ref().expect("completed DSM staging").staging_id.clone());
                    terminal.assets.push(asset);
                    terminal.raster_asset = None;
                    terminal.phase = TerminalPhase::Dtm;
                }
                RasterAssetProgress::Failed => {
                    discard_terminal_staging(&terminal);
                    complete_session(generation);
                    return Ok(Emit::default());
                }
            }
        }
        TerminalPhase::Dtm => {
            let dsm_id = format!("geo-dsm-{}", session.job_id);
            let dtm_id = format!("geo-dtm-{}", session.job_id);
            if terminal.raster_asset.is_none() {
                let dtm = terminal.pending_dtm.take().expect("DTM retained through DSM encoding");
                terminal.raster_asset = Some(RasterAssetPreparation::new(dtm, format!("{}:dtm:{generation}", session.artifact_authority), dtm_id.clone()));
            }
            match terminal.raster_asset.as_mut().expect("DTM preparation").advance() {
                RasterAssetProgress::Working => {}
                RasterAssetProgress::Mutation(mutation) => step_mutation = Some(mutation),
                RasterAssetProgress::Complete(asset) => {
                    terminal.completed_asset_staging_ids.push(terminal.raster_asset.as_ref().expect("completed DTM staging").staging_id.clone());
                    terminal.assets.push(asset);
                    terminal.geo_result = Some(GeoProducts { dsm_asset_id: Some(dsm_id), dtm_asset_id: Some(dtm_id), ortho_asset_id: None });
                    terminal.raster_asset = None;
                    terminal.phase = TerminalPhase::Commit;
                }
                RasterAssetProgress::Failed => {
                    discard_terminal_staging(&terminal);
                    complete_session(generation);
                    return Ok(Emit::default());
                }
            }
        }
        TerminalPhase::Commit => {
            let job = ReconstructionJob {
                id: session.job_id,
                stage: ReconstructionStage::Done,
                progress_0_1: 1.0,
                cancel_requested: false,
                stage_cursor: session.tick,
                started_at_ms: None,
                error: None,
                camera_poses_preview: terminal.preview_cameras,
                sparse_point_cloud_preview: PackedF32(terminal.preview_points_base64),
            };
            let trajectory = (!terminal.trajectory.is_empty()).then_some(CameraTrajectory { poses: terminal.trajectory });
            let mutation = commit_reconstruction(CommitReconstruction { job, sparse: terminal.sparse, trajectory, mesh: terminal.mesh_result, geo: terminal.geo_result, qc: terminal.qc, assets: terminal.assets });
            complete_session(generation);
            return Ok(Emit { artifact_mutations: vec![mutation], coalesce_key: Some(format!("reconstruction:{generation}")), ui_scope: UiDirtyScope::Full, ..Default::default() });
        }
    }
    session.tick = session.tick.saturating_add(1);
    session.terminal = Some(terminal);
    Ok(yield_terminal(generation, session, step_mutation))
}

/// ⏱️ Advances one ingestion cursor, one engine unit, or one terminal preparation phase.
pub fn advance_reconstruction(payload: &AdvanceReconstruction, doc: &ArtifactView<'_, RemodelSnapshot>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    let scene = doc.snapshot;
    if scene.job.id != payload.job_id || scene.job.cancel_requested {
        cancel_session(payload.generation);
        return Ok(Emit::default());
    }
    let Some(mut session) = take_session(payload.generation) else { return Ok(Emit::default()) };
    if session.job_id != payload.job_id || &checkpoint(payload.generation, &session) != payload {
        discard_session_staging(&session);
        complete_session(payload.generation);
        return Ok(Emit::default());
    }
    if session.tick >= MAX_RECONSTRUCTION_TICKS {
        discard_session_staging(&session);
        complete_session(payload.generation);
        let job = ReconstructionJob { stage: ReconstructionStage::Failed, error: Some("reconstruction exceeded its bounded continuation limit".into()), ..scene.job.clone() };
        return Ok(emit_step(job, payload.generation, None));
    }
    if session.terminal.is_some() {
        return advance_terminal(payload.generation, session);
    }
    if let Some(mut ingestion) = session.ingestion.take() {
        let mut complete = false;
        if ingestion.decoded_image.is_some() {
            if advance_frame_sharpness(&mut ingestion) {
                let pixels = ingestion.decoded_image.as_ref().map_or(0, |image| image.width as usize * image.height as usize);
                let sharpness = if pixels == 0 { 0.0 } else { (ingestion.sharpness_sum / pixels as f64) as f32 };
                let image = ingestion.decoded_image.take().expect("completed frame admission");
                session.engine.push_frame_with_sharpness(ingestion.frame_index, image, ingestion.timestamp_ms, sharpness);
                complete = true;
            }
        } else if let Some(decoder) = ingestion.decoder.as_mut() {
            match decoder.advance() {
                BoundedDecodeProgress::Working => {}
                BoundedDecodeProgress::Complete(image) => {
                    ingestion.decoder = None;
                    ingestion.decoded_image = Some(image);
                }
                BoundedDecodeProgress::Failed(_) => complete = true,
            }
        } else {
            ingestion.decoder = Some(BoundedStillDecoder::new(&ingestion.mime, std::mem::take(&mut ingestion.compressed)));
        }
        if complete {
            (session.stream_index, session.frame_index) = next_frame_cursor(scene, session.stream_index, session.frame_index)?;
        } else {
            session.ingestion = Some(ingestion);
        }
        session.tick = session.tick.saturating_add(1);
        let job = ReconstructionJob { stage: ReconstructionStage::Ingesting, stage_cursor: session.tick, ..scene.job.clone() };
        let next = checkpoint(payload.generation, &session);
        put_session(payload.generation, session);
        return Ok(emit_step(job, payload.generation, Some(next)));
    }
    if let Some(stream) = scene.streams.get(session.stream_index as usize) {
        if let Some(frame_ref) = stream.frames.get(session.frame_index as usize) {
            if let Some(ingestion) = frame_ingestion(scene, frame_ref) {
                session.ingestion = Some(ingestion);
            } else {
                (session.stream_index, session.frame_index) = next_frame_cursor(scene, session.stream_index, session.frame_index)?;
            }
        } else {
            (session.stream_index, session.frame_index) = next_frame_cursor(scene, session.stream_index, session.frame_index)?;
        }
        session.tick = session.tick.saturating_add(1);
        let job = ReconstructionJob { stage: ReconstructionStage::Ingesting, stage_cursor: session.tick, ..scene.job.clone() };
        let next = checkpoint(payload.generation, &session);
        put_session(payload.generation, session);
        return Ok(emit_step(job, payload.generation, Some(next)));
    }
    let status = session.engine.advance(RECONSTRUCTION_STEP_BUDGET);
    session.tick = session.tick.saturating_add(1);
    match status {
        remodel_engine::EngineStatus::Working { stage, progress: _ } if requested_stage_complete(session.requested_stage, stage) => {
            complete_session(payload.generation);
            let job = ReconstructionJob { stage: ReconstructionStage::Done, progress_0_1: 1.0, stage_cursor: session.tick, ..scene.job.clone() };
            Ok(emit_step(job, payload.generation, None))
        }
        remodel_engine::EngineStatus::Working { stage, progress } => {
            let job = preview_job(payload.job_id.clone(), reconstruction_stage(stage), progress, session.tick, &session.engine);
            let next = checkpoint(payload.generation, &session);
            put_session(payload.generation, session);
            Ok(emit_step(job, payload.generation, Some(next)))
        }
        remodel_engine::EngineStatus::Done if session.requested_stage.needs_terminal_products() => {
            session.terminal = Some(terminal_preparation(payload.generation, &session.artifact_authority));
            Ok(yield_terminal(payload.generation, session, None))
        }
        remodel_engine::EngineStatus::Done => {
            complete_session(payload.generation);
            let job = ReconstructionJob { stage: ReconstructionStage::Done, progress_0_1: 1.0, stage_cursor: session.tick, ..scene.job.clone() };
            Ok(emit_step(job, payload.generation, None))
        }
        remodel_engine::EngineStatus::Failed(message) => {
            complete_session(payload.generation);
            let job = ReconstructionJob { stage: ReconstructionStage::Failed, error: Some(message), stage_cursor: session.tick, ..scene.job.clone() };
            Ok(emit_step(job, payload.generation, None))
        }
    }
}
//#endregion 🔖️Run

//#region 🔖️Payloads
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "run-reconstruction")]
pub struct RunReconstruction {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "advance-reconstruction")]
pub struct AdvanceReconstruction {
    pub generation: u64,
    pub job_id: String,
    pub requested_stage: String,
    pub phase: String,
    pub stream_index: u32,
    pub frame_index: u32,
    pub terminal_cursor: u64,
    pub tick: u32,
}
//#endregion 🔖️Payloads

pub async fn handle(_payload: &RunReconstruction, doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    begin_reconstruction(doc)
}

pub async fn handle_advance(payload: &AdvanceReconstruction, doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    advance_reconstruction(payload, doc)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::remodel::testkit::{app_with_registry, RemodelApp};
    use crate::editor::remodel::RemodelPlayApp;
    use protocol::OpText as _;
    use semio_framework_plugin::testkit::meta;
    use semio_framework_plugin::{ArtifactEditor, InvocationResult, PluginApp};

    fn test_session(job_id: &str, requested_stage: RequestedStage) -> ReconstructionSession {
        ReconstructionSession {
            job_id: job_id.into(),
            artifact_authority: format!("document=test:app=test:operation={job_id}:generation=test"),
            engine: remodel_engine::ReconstructionEngine::new(&remodel_engine::EngineParams::default()),
            texture_size: 1,
            gcp_count: 0,
            requested_stage,
            stream_index: 0,
            frame_index: 0,
            ingestion: None,
            tick: 0,
            terminal: None,
        }
    }

    fn store_session(generation: u64, session: ReconstructionSession) {
        assert!(admit_session(generation, &session.job_id));
        put_session(generation, session);
    }

    fn forget_all_remodel_process_state() {
        *sessions().lock().expect("remodel reconstruction sessions lock") = ReconstructionSessions::default();
        crate::artifacts::remodel::forget_all_remodel_content_for_test();
    }

    async fn dispatch_public_action(app: &mut RemodelApp, action: &str, args: Option<serde_json::Value>) -> InvocationResult {
        let command = <RemodelPlayApp as ArtifactEditor>::command_from_action(action, args.as_ref()).await.expect("public Remodel action bridge");
        app.dispatch_typed(command, &meta("local")).await.expect("public ActionBus worker dispatch")
    }

    async fn dispatch_continuation(app: &mut RemodelApp, effect: Effect) -> InvocationResult {
        let Effect::DispatchAction { action, args, .. } = effect else { panic!("reconstruction continuation action") };
        assert_eq!(action, ADVANCE_RECONSTRUCTION_ACTION_ID);
        let args = args.map(|value| semio_framework::from_dsl_value::<serde_json::Value>(value).expect("continuation args"));
        dispatch_public_action(app, &action, args).await
    }

    async fn drive_public_reconstruction(app: &mut RemodelApp, start_action: &str) -> (RemodelSnapshot, Vec<ReconstructionStage>) {
        let start_args = matches!(start_action, "runStage" | "retryStage").then(|| json!({ "stage": "texturing" }));
        let mut result = dispatch_public_action(app, start_action, start_args).await;
        let mut stages = Vec::new();
        for _ in 0..MAX_RECONSTRUCTION_TICKS {
            assert_eq!(result.mutations.len(), 1, "every active reconstruction handler turn emits exactly one durable mutation");
            let snapshot = app.snapshot().await.expect("worker-applied Remodel snapshot");
            if !stages.contains(&snapshot.job.stage) {
                stages.push(snapshot.job.stage);
            }
            let next = result.requested_effects.into_iter().find(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == ADVANCE_RECONSTRUCTION_ACTION_ID));
            let Some(next) = next else { return (snapshot, stages) };
            result = dispatch_continuation(app, next).await;
        }
        panic!("public reconstruction did not reach a terminal stage")
    }

    fn typed_rows(ops: &str) -> Vec<String> {
        ops.lines().filter_map(|line| line.strip_prefix("  ").map(str::to_string)).collect()
    }

    fn assert_durable_chunk_ceiling(mutation: &RemodelMutation) {
        let RemodelMutation::CreateAsset(payload) = mutation else { return };
        let chunked = crate::artifacts::remodel::remodel_asset_stage_parts(&payload.key).is_some() || crate::artifacts::remodel::remodel_mesh_stage_asset_parts(&payload.key).is_some();
        if chunked {
            let bytes = base64::engine::general_purpose::STANDARD.decode(&payload.asset.data).expect("typed durable chunk base64");
            assert!(bytes.len() <= MESH_CHUNK_BYTES, "every shared durable asset/mesh row is at most 4 KiB raw");
        }
    }

    fn durable_input_assets(snapshot: &RemodelSnapshot) -> Vec<(String, ImageAsset)> {
        snapshot
            .streams
            .iter()
            .flat_map(|stream| &stream.frames)
            .map(|frame| {
                let asset = crate::artifacts::remodel::remodel_asset(snapshot, &frame.asset_id).expect("durable input asset");
                (frame.asset_id.clone(), asset)
            })
            .collect()
    }

    async fn public_app_with_inputs(document: &str) -> RemodelApp {
        let mut app = app_with_registry().await;
        for index in 0..4 {
            let payload = crate::editor::remodel::commands::import_frame_payload::checker_data_url(24, 24, 3).await;
            dispatch_public_action(&mut app, "importFramePayload", Some(json!({ "payload": payload, "name": format!("{document}-frame-{index}.png"), "index": index }))).await;
        }
        app
    }

    fn continuation_identity(effect: &Effect) -> (u64, String) {
        let Effect::DispatchAction { args: Some(args), .. } = effect else { panic!("continuation identity") };
        let value = semio_framework::from_dsl_value::<serde_json::Value>(args.clone()).expect("continuation identity args");
        (value["generation"].as_u64().expect("generation"), value["jobId"].as_str().expect("job id").to_string())
    }

    #[semio_framework_async_macros::async_test]
    async fn one_continuation_advances_at_most_one_engine_unit() {
        assert_eq!(RECONSTRUCTION_STEP_BUDGET, 1);
        let payload = AdvanceReconstruction { generation: 7, job_id: "job-7".into(), requested_stage: "matching-features".into(), phase: "pipeline".into(), stream_index: 2, frame_index: 3, terminal_cursor: 0, tick: 11 };
        let Effect::DispatchAction { args, .. } = queue(&payload) else { panic!("continuation effect") };
        let value = semio_framework::from_dsl_value::<serde_json::Value>(args.expect("args")).expect("decode args");
        assert_eq!(value["generation"], 7);
        assert_eq!(value["requestedStage"], "matching-features");
        assert_eq!(value["tick"], 11);
    }

    #[semio_framework_async_macros::async_test]
    async fn snapshot_frame_ingestion_decodes_owned_leaves_without_whole_asset_reassembly() {
        let app = public_app_with_inputs("snapshot-leaves").await;
        let snapshot = app.snapshot().await.expect("snapshot with durable input");
        let frame = snapshot.streams.first().and_then(|stream| stream.frames.first()).expect("durable frame");
        let expected_identity = snapshot.assets.get(&frame.asset_id).expect("durable asset handle").child_id.clone();
        let mut ingestion = frame_ingestion(&snapshot, frame).expect("active snapshot-to-ingestion path");
        assert_eq!(ingestion._asset_identity, expected_identity);
        assert!(ingestion.compressed.leaf_lengths().iter().all(|length| *length <= MESH_CHUNK_BYTES));
        let input_len = ingestion.compressed.len();
        let observed_reads = ingestion.compressed.clone();
        ingestion.decoder = Some(BoundedStillDecoder::new(&ingestion.mime, std::mem::take(&mut ingestion.compressed)));
        loop {
            match ingestion.decoder.as_mut().expect("active decoder").advance() {
                BoundedDecodeProgress::Working => {}
                BoundedDecodeProgress::Complete(image) => {
                    assert_eq!((image.width, image.height), (24, 24));
                    break;
                }
                BoundedDecodeProgress::Failed(error) => panic!("snapshot-owned leaves failed to decode: {error}"),
            }
        }
        let metrics = observed_reads.read_metrics();
        assert!(metrics.sequential_bytes > 0 && metrics.sequential_bytes <= input_len, "active PNG ingestion permits no duplicate full read");
        assert!(metrics.sequential_reads <= input_len.saturating_add(2));
        assert!(metrics.largest_sequential_read <= MESH_CHUNK_BYTES);
        assert!(metrics.random_byte_reads <= input_len.min(32));
        assert!(metrics.largest_random_read <= 1);
    }

    #[test]
    fn distinct_stage_requests_stop_after_distinct_dependency_prefixes() {
        let matching = RequestedStage::parse("matching-features").expect("matching stage");
        let dense = RequestedStage::parse("dense-stereo").expect("dense stage");
        assert!(requested_stage_complete(matching, remodel_engine::EngineStage::EstimatingPoses));
        assert!(!requested_stage_complete(dense, remodel_engine::EngineStage::EstimatingPoses));
        assert!(requested_stage_complete(dense, remodel_engine::EngineStage::FusingVolume));
        assert_eq!(matching.wire(), "matching-features");
        assert_eq!(dense.wire(), "dense-stereo");
    }

    #[test]
    fn terminal_phase_and_preview_work_are_finite() {
        assert_eq!(TERMINAL_CAMERA_WORK, 64);
        assert_eq!(TERMINAL_POINT_WORK, 256);
        assert_eq!(TERMINAL_QUALITY_WORK, 256);
        assert_eq!(TERMINAL_GEO_WORK, 256);
        assert_eq!(PREVIEW_CAMERA_LIMIT, 64);
        assert_eq!(PREVIEW_POINT_LIMIT, 256);
        assert_eq!(terminal_phase_wire(TerminalPhase::Sparse), "terminal-sparse");
        assert_eq!(terminal_phase_wire(TerminalPhase::Commit), "terminal-commit");
    }

    #[semio_framework_async_macros::async_test]
    async fn public_action_bus_workers_replay_every_start_action_from_genesis_after_total_process_loss() {
        for start_action in ["runReconstruction", "runStage", "retryStage"] {
            forget_all_remodel_process_state();
            let mut app = public_app_with_inputs(start_action).await;

            let (terminal, stages) = drive_public_reconstruction(&mut app, start_action).await;
            assert_eq!(terminal.job.stage, ReconstructionStage::Done, "{start_action} reaches the real terminal commit");
            for required in [
                ReconstructionStage::Ingesting,
                ReconstructionStage::ExtractingFeatures,
                ReconstructionStage::MatchingFeatures,
                ReconstructionStage::EstimatingPoses,
                ReconstructionStage::BundleAdjusting,
                ReconstructionStage::DenseStereo,
                ReconstructionStage::FusingVolume,
                ReconstructionStage::ExtractingSurface,
                ReconstructionStage::CleaningMesh,
                ReconstructionStage::Texturing,
                ReconstructionStage::ReportingQc,
                ReconstructionStage::Done,
            ] {
                assert!(stages.contains(&required), "{start_action} missed {required:?}");
            }

            let terminal_handle = terminal.results.mesh.mesh.clone();
            let terminal_mesh = crate::artifacts::remodel::resolve_bounded_remodel_mesh(&terminal.durable_artifacts, &terminal_handle).expect("terminal bounded mesh");
            let terminal_sparse = terminal.results.sparse.as_ref().map(|sparse| sparse.points.to_f32_vec_from(&terminal.durable_artifacts)).unwrap_or_default();
            assert!(!terminal_sparse.is_empty(), "terminal sparse content is committed through the compact event");
            assert!(!terminal.durable_artifacts.is_empty());
            for artifact in terminal.durable_artifacts.values() {
                assert!(!artifact.chunks.is_empty());
                for chunk in &artifact.chunks {
                    let leaf = base64::engine::general_purpose::STANDARD.decode(chunk).expect("durable leaf encoding");
                    assert!(leaf.len() <= 4 * 1024, "durable state never hides a whole unbounded payload");
                }
            }
            let terminal_inputs = durable_input_assets(&terminal);
            assert_eq!(terminal_inputs.len(), 4);
            let (_, _, terminal_chunk_count) = crate::artifacts::remodel::replayable_remodel_mesh_handle_parts(&terminal_handle).expect("terminal durable mesh handle");
            assert!(terminal_chunk_count > 0);

            let files = app.document_text().await.expect("public typed Remodel op log");
            let rows = typed_rows(&files.ops);
            assert!(!rows.is_empty());
            for row in &rows {
                let mutation: RemodelMutation = protocol::OpText::parse_op(row).await.expect("typed Remodel OpText row");
                assert_durable_chunk_ceiling(&mutation);
            }

            forget_all_remodel_process_state();
            assert_eq!(crate::artifacts::remodel::resolve_bounded_remodel_mesh(&terminal.durable_artifacts, &terminal_handle), Some(terminal_mesh.clone()));
            assert_eq!(terminal.results.sparse.as_ref().expect("terminal sparse handle").points.to_f32_vec_from(&terminal.durable_artifacts), terminal_sparse);
            let mut replayed = app_with_registry().await;
            for row in &rows {
                replayed.ingest_operations_text(row).await.expect("one typed row replayed from genesis");
            }
            let replayed_snapshot = replayed.snapshot().await.expect("replayed snapshot");
            assert_eq!(replayed_snapshot.results.mesh.mesh, terminal_handle);
            assert_eq!(replayed_snapshot.results.sparse.as_ref().expect("replayed sparse").points.to_f32_vec_from(&replayed_snapshot.durable_artifacts), terminal_sparse);
            assert_eq!(durable_input_assets(&replayed_snapshot), terminal_inputs);
            assert_eq!(crate::artifacts::remodel::resolve_bounded_remodel_mesh(&replayed_snapshot.durable_artifacts, &replayed_snapshot.results.mesh.mesh), Some(terminal_mesh.clone()));

            replayed.handle_action("commitCheckpoint", None, &meta("local")).await.expect("checkpoint replayed terminal document");
            let checkpoint = replayed.document_pack().await.expect("checkpointed terminal pack");
            forget_all_remodel_process_state();
            let mut restored = app_with_registry().await;
            restored.load_document_pack(&checkpoint).await.expect("restore checkpointed terminal document");
            let restored_snapshot = restored.snapshot().await.expect("restored terminal snapshot");
            assert_eq!(restored_snapshot.results.mesh.mesh, terminal_handle);
            assert_eq!(restored_snapshot.results.sparse.as_ref().expect("restored sparse").points.to_f32_vec_from(&restored_snapshot.durable_artifacts), terminal_sparse);
            assert_eq!(durable_input_assets(&restored_snapshot), terminal_inputs);
            assert_eq!(crate::artifacts::remodel::resolve_bounded_remodel_mesh(&restored_snapshot.durable_artifacts, &restored_snapshot.results.mesh.mesh), Some(terminal_mesh));
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn public_workers_isolate_two_documents_and_reject_cancelled_stale_aba_continuations() {
        forget_all_remodel_process_state();
        let mut document_a = public_app_with_inputs("document-a").await;
        let mut document_b = public_app_with_inputs("document-b").await;

        let start_a = dispatch_public_action(&mut document_a, "runReconstruction", None).await;
        let old_a = start_a.requested_effects.into_iter().find(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == ADVANCE_RECONSTRUCTION_ACTION_ID)).expect("document A continuation");
        let (old_generation, old_job) = continuation_identity(&old_a);

        let (terminal_b, _) = drive_public_reconstruction(&mut document_b, "runReconstruction").await;
        assert_eq!(terminal_b.job.stage, ReconstructionStage::Done, "document B completes while document A remains admitted");

        dispatch_public_action(&mut document_a, "cancelReconstruction", None).await;
        let cancelled_a = document_a.snapshot().await.expect("cancelled document A");
        assert!(cancelled_a.job.cancel_requested);
        let restart_a = dispatch_public_action(&mut document_a, "runReconstruction", None).await;
        let new_a = restart_a.requested_effects.into_iter().find(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == ADVANCE_RECONSTRUCTION_ACTION_ID)).expect("document A replacement continuation");
        let (new_generation, new_job) = continuation_identity(&new_a);
        assert_ne!(new_generation, old_generation, "generation identity is never reused after cancellation");
        assert_ne!(new_job, old_job, "job identity is never reused after cancellation");

        let stale = dispatch_continuation(&mut document_a, old_a).await;
        assert!(stale.mutations.is_empty());
        assert!(stale.requested_effects.is_empty());
        let live_a = document_a.snapshot().await.expect("live document A replacement");
        assert_eq!(live_a.job.id, new_job, "stale ABA delivery cannot overwrite the replacement job");
        assert_eq!(terminal_b, document_b.snapshot().await.expect("document B remains terminal"));
        dispatch_public_action(&mut document_a, "cancelReconstruction", None).await;
    }

    #[test]
    fn shared_durable_chunk_admission_accepts_4k_and_rejects_overflow_and_malformed_rows() {
        forget_all_remodel_process_state();
        let asset_max = vec![0x5a; MESH_CHUNK_BYTES];
        let asset_over = vec![0x5a; MESH_CHUNK_BYTES + 1];
        assert!(crate::artifacts::remodel::stage_remodel_asset_chunk("asset-max", crate::artifacts::remodel::RemodelAssetContentKind::Raster, 0, &base64::engine::general_purpose::STANDARD.encode(&asset_max)).is_ok());
        assert!(crate::artifacts::remodel::stage_remodel_asset_chunk("asset-over", crate::artifacts::remodel::RemodelAssetContentKind::Raster, 0, &base64::engine::general_purpose::STANDARD.encode(&asset_over)).is_err());
        assert!(crate::artifacts::remodel::stage_remodel_asset_chunk("asset-malformed", crate::artifacts::remodel::RemodelAssetContentKind::Sparse, 0, "%%%").is_err());
        assert!(crate::artifacts::remodel::stage_remodel_asset_chunk("asset-index-overflow", crate::artifacts::remodel::RemodelAssetContentKind::Sparse, u64::MAX, &base64::engine::general_purpose::STANDARD.encode([1])).is_err());

        let mut mesh_max = vec![11];
        mesh_max.resize(MESH_CHUNK_BYTES, 1);
        let mut mesh_over = mesh_max.clone();
        mesh_over.push(1);
        assert!(crate::artifacts::remodel::stage_remodel_mesh_chunk("mesh-max", 0, &base64::engine::general_purpose::STANDARD.encode(&mesh_max)).is_ok());
        assert!(crate::artifacts::remodel::stage_remodel_mesh_chunk("mesh-over", 0, &base64::engine::general_purpose::STANDARD.encode(&mesh_over)).is_err());
        assert!(crate::artifacts::remodel::stage_remodel_mesh_chunk("mesh-malformed", 0, "%%%").is_err());
        assert!(crate::artifacts::remodel::stage_remodel_mesh_chunk("mesh-index-overflow", u64::MAX, &base64::engine::general_purpose::STANDARD.encode([10, 1])).is_err());
        forget_all_remodel_process_state();
    }

    #[test]
    fn aggregate_staging_rejects_overflow_malformed_field_order_and_513th_elements_with_cleanup() {
        forget_all_remodel_process_state();
        let full = base64::engine::general_purpose::STANDARD.encode(vec![1; MESH_CHUNK_BYTES]);
        let tail = base64::engine::general_purpose::STANDARD.encode(vec![1; 2_048]);
        assert!(crate::artifacts::remodel::stage_remodel_asset_chunk("sparse-overflow", crate::artifacts::remodel::RemodelAssetContentKind::Sparse, 0, &full).is_ok());
        assert!(crate::artifacts::remodel::stage_remodel_asset_chunk("sparse-overflow", crate::artifacts::remodel::RemodelAssetContentKind::Sparse, 1, &tail).is_ok());
        assert!(crate::artifacts::remodel::stage_remodel_asset_chunk("sparse-overflow", crate::artifacts::remodel::RemodelAssetContentKind::Sparse, 2, &base64::engine::general_purpose::STANDARD.encode([1])).is_err());
        assert_eq!(crate::artifacts::remodel::staged_remodel_asset_chunk_count("sparse-overflow"), 0);

        assert!(crate::artifacts::remodel::stage_remodel_asset_chunk("kind-mismatch", crate::artifacts::remodel::RemodelAssetContentKind::Sparse, 0, &tail).is_ok());
        assert_eq!(crate::artifacts::remodel::stage_remodel_asset_chunk("kind-mismatch", crate::artifacts::remodel::RemodelAssetContentKind::Raster, 1, &tail), Err(crate::artifacts::remodel::RemodelStagingFault::Invalid));
        assert_eq!(crate::artifacts::remodel::staged_remodel_asset_chunk_count("kind-mismatch"), 0);

        let indices = base64::engine::general_purpose::STANDARD.encode([3, 0, 0, 0, 0]);
        let positions = base64::engine::general_purpose::STANDARD.encode([0, 0, 0, 0, 0]);
        assert!(crate::artifacts::remodel::stage_remodel_mesh_chunk("field-order", 0, &indices).is_ok());
        assert!(crate::artifacts::remodel::stage_remodel_mesh_chunk("field-order", 1, &positions).is_err());
        assert_eq!(crate::artifacts::remodel::staged_remodel_mesh_chunk_count("field-order"), 0);
        assert_eq!(crate::artifacts::remodel::stage_remodel_mesh_chunk("component-count", 0, &base64::engine::general_purpose::STANDARD.encode([0, 1])), Err(crate::artifacts::remodel::RemodelStagingFault::Invalid));

        for (staging_id, field) in [("vertex-513", 0u8), ("triangle-513", 3u8)] {
            let mut values = Vec::with_capacity(513 * 3 * 4);
            for value in 0..513 * 3 {
                values.extend_from_slice(&(value as u32).to_le_bytes());
            }
            for (index, chunk) in values.chunks(MESH_CHUNK_BYTES - 4).enumerate() {
                let mut framed = vec![field];
                framed.extend_from_slice(chunk);
                let result = crate::artifacts::remodel::stage_remodel_mesh_chunk(staging_id, index as u64, &base64::engine::general_purpose::STANDARD.encode(framed));
                if index == 1 {
                    assert!(result.is_err(), "513th semantic element is rejected before retention");
                } else {
                    assert!(result.is_ok());
                }
            }
            assert_eq!(crate::artifacts::remodel::staged_remodel_mesh_chunk_count(staging_id), 0);
        }
        forget_all_remodel_process_state();
    }

    #[test]
    fn staging_busy_and_preparation_accounting_overflow_are_typed_and_early() {
        forget_all_remodel_process_state();
        let one = base64::engine::general_purpose::STANDARD.encode([1]);
        for index in 0..32 {
            assert!(crate::artifacts::remodel::stage_remodel_asset_chunk(&format!("busy-{index}"), crate::artifacts::remodel::RemodelAssetContentKind::Sparse, 0, &one).is_ok());
        }
        assert_eq!(crate::artifacts::remodel::stage_remodel_asset_chunk("busy-overflow", crate::artifacts::remodel::RemodelAssetContentKind::Sparse, 0, &one), Err(crate::artifacts::remodel::RemodelStagingFault::Busy));

        let mut content_digest = ContentPreparation::new("digest-overflow".into());
        content_digest.digest_len = u64::MAX;
        assert!(content_digest.record(&[1]).is_err());
        let mut content_chunks = ContentPreparation::new("count-overflow".into());
        content_chunks.chunk_count = u64::MAX;
        assert!(content_chunks.record(&[]).is_err());

        let mesh = semio_framework::MeshData { positions: vec![0.0], ..Default::default() };
        let mut mesh_digest = MeshPreparation::new(mesh.clone(), "mesh-digest-overflow".into());
        mesh_digest.digest_len = u64::MAX;
        assert!(mesh_digest.next_chunk().is_err());
        let mut mesh_chunks = MeshPreparation::new(mesh, "mesh-count-overflow".into());
        mesh_chunks.chunk_count = u64::MAX;
        assert!(mesh_chunks.next_chunk().is_err());
        forget_all_remodel_process_state();
    }

    #[semio_framework_async_macros::async_test]
    async fn stale_generation_is_cancelled_before_any_mutation_or_continuation() {
        let mut old = test_session("old", RequestedStage::MatchingFeatures);
        old.terminal = Some(terminal_preparation(41, &old.artifact_authority));
        store_session(41, old);
        store_session(42, test_session("live", RequestedStage::DenseStereo));
        let mut scene = crate::artifacts::remodel::default_remodel_scene();
        scene.job.id = "live".into();
        let history = semio_framework_plugin::HistoryView::empty().await;
        let view = ArtifactView::new(&scene, &history).await;
        let stale =
            advance_reconstruction(&AdvanceReconstruction { generation: 41, job_id: "old".into(), requested_stage: "matching-features".into(), phase: "terminal-sparse".into(), stream_index: 0, frame_index: 0, terminal_cursor: 0, tick: 0 }, &view)
                .expect("stale step");
        assert!(stale.artifact_mutations.is_empty());
        assert!(stale.effects.is_empty());
        assert!(take_session(41).is_none());
        assert!(take_session(42).is_some());
        complete_session(42);
    }

    #[test]
    fn maximum_envelope_mesh_chunks_are_bounded_replayable_and_resolve_across_threads() {
        let mesh = semio_framework::MeshData { positions: (0..512 * 3).map(|index| index as f32 * 0.001).collect(), indices: (0..512 * 3).map(|index| (index % 512) as u32).collect(), ..Default::default() };
        let expected = mesh.clone();
        let mut preparation = MeshPreparation::new(mesh, "cross-thread-stage".into());
        while let Some(chunk) = preparation.next_chunk().expect("checked mesh chunk accounting") {
            assert!(chunk.len() <= MESH_CHUNK_BYTES);
            let index = preparation.chunk_count.checked_sub(1).expect("emitted mesh chunk has a checked index");
            let encoded = base64::engine::general_purpose::STANDARD.encode(chunk);
            let started = std::time::Instant::now();
            assert!(std::thread::spawn(move || crate::artifacts::remodel::stage_remodel_mesh_chunk("cross-thread-stage", index, &encoded)).join().expect("worker stage").is_ok());
            assert!(started.elapsed() < std::time::Duration::from_millis(8), "full worker-hop mesh stage exceeded 8 ms");
        }
        let content_id = preparation.content_id();
        let chunk_count = preparation.chunk_count;
        let started = std::time::Instant::now();
        let durable = crate::artifacts::remodel::durable_staged_remodel_mesh("cross-thread-stage").expect("bounded staged mesh materializes");
        let mut durable_store = crate::artifacts::remodel::RemodelDurableArtifactStore::default();
        durable_store.insert(content_id.clone(), durable);
        let handle = crate::artifacts::remodel::replayable_remodel_mesh_handle(&content_id, "cross-thread-stage", chunk_count);
        crate::artifacts::remodel::discard_staged_remodel_mesh("cross-thread-stage");
        assert!(started.elapsed() < std::time::Duration::from_millis(8), "compact snapshot-owned mesh publication exceeded 8 ms");
        let resolved = std::thread::spawn(move || crate::artifacts::remodel::resolve_bounded_remodel_mesh(&durable_store, &handle)).join().expect("worker resolve").expect("durable mesh");
        assert_eq!(resolved, expected);
    }

    #[semio_framework_async_macros::async_test]
    async fn cancellation_and_stale_delivery_are_isolated_between_documents() {
        let generation_a = 8_100_001;
        let generation_b = 8_100_002;
        let mut scene_a = crate::artifacts::remodel::default_remodel_scene();
        scene_a.job.id = "document-a-job".into();
        let mut scene_b = crate::artifacts::remodel::default_remodel_scene();
        scene_b.job.id = "document-b-job".into();
        let session_a = test_session(&scene_a.job.id, RequestedStage::Full);
        let session_b = test_session(&scene_b.job.id, RequestedStage::DenseStereo);
        let stale_payload_a = checkpoint(generation_a, &session_a);
        store_session(generation_a, session_a);
        store_session(generation_b, session_b);

        let cancel = cancel_current_reconstruction(&scene_a);
        assert!(!cancel.artifact_mutations.is_empty());
        assert!(take_session(generation_a).is_none());
        let retained_b = take_session(generation_b).expect("cancelling document A preserves document B");
        put_session(generation_b, retained_b);

        let history = semio_framework_plugin::HistoryView::empty().await;
        let view_a = ArtifactView::new(&scene_a, &history).await;
        let config = RemodelConfig::default();
        let stale = handle_advance(&stale_payload_a, &view_a, &ConfigView { snapshot: &config }).await.expect("stale handler delivery");
        assert!(stale.artifact_mutations.is_empty());
        assert!(stale.effects.is_empty());
        assert!(take_session(generation_b).is_some(), "stale document A delivery cannot cancel document B");
        complete_session(generation_b);
    }

    #[semio_framework_async_macros::async_test]
    async fn user_cancel_drops_generation_and_private_mesh_staging() {
        let mut scene = crate::artifacts::remodel::default_remodel_scene();
        scene.job.id = "cancel-job".into();
        let mut session = test_session("cancel-job", RequestedStage::Full);
        let mut terminal = terminal_preparation(92, "document=test:app=test:operation=cancel:92");
        terminal.mesh = Some(MeshPreparation::new(semio_framework::MeshData::default(), "cancel-stage".into()));
        session.terminal = Some(terminal);
        crate::artifacts::remodel::stage_remodel_mesh_chunk("cancel-stage", 0, &base64::engine::general_purpose::STANDARD.encode([10, 1])).expect("cancel fixture staged");
        store_session(92, session);

        let emit = cancel_current_reconstruction(&scene);
        assert!(take_session(92).is_none());
        assert_eq!(crate::artifacts::remodel::staged_remodel_mesh_chunk_count("cancel-stage"), 0);
        assert!(emit.effects.is_empty());
        assert_eq!(emit.artifact_mutations.len(), 1);
    }

    #[test]
    fn cancellation_during_compressed_streaming_drops_the_rope_without_decode_or_publication() {
        let mut scene = crate::artifacts::remodel::default_remodel_scene();
        scene.job.id = "stream-cancel-job".into();
        let leaf = std::sync::Arc::<[u8]>::from([0x89, b'P', b'N', b'G']);
        let mut compressed = CompressedChunkRope::default();
        compressed.push(leaf.clone(), MAX_STILL_INPUT_BYTES).expect("streaming leaf admitted");
        let mut session = test_session(&scene.job.id, RequestedStage::Full);
        session.ingestion = Some(FrameIngestion { _asset_identity: "cancel-input".into(), mime: "image/png".into(), frame_index: 0, timestamp_ms: 0.0, compressed, decoder: None, decoded_image: None, sharpness_cursor: 0, sharpness_sum: 0.0 });
        assert_eq!(std::sync::Arc::strong_count(&leaf), 2);
        store_session(93, session);
        let emit = cancel_current_reconstruction(&scene);
        assert_eq!(emit.artifact_mutations.len(), 1);
        assert!(emit.effects.is_empty());
        assert!(take_session(93).is_none());
        assert_eq!(std::sync::Arc::strong_count(&leaf), 1, "cancellation releases the persistent compressed rope before decode");
    }

    #[semio_framework_async_macros::async_test]
    async fn admission_never_evicts_an_active_worker_owned_generation() {
        let mut registry = ReconstructionSessions::default();
        for generation in 1..=MAX_LIVE_SESSIONS as u64 {
            assert!(registry.admit(generation, &format!("job-{generation}")).is_ok());
            registry.live.insert(generation, test_session(&format!("job-{generation}"), RequestedStage::Full));
        }
        let worker_owned = registry.live.remove(&1).expect("worker owns admitted session");
        assert!(registry.admit(MAX_LIVE_SESSIONS as u64 + 1, "overflow").is_err());
        assert!(registry.admitted.contains(&1));
        assert!(registry.live.contains_key(&2));
        registry.live.insert(1, worker_owned);
        assert_eq!(registry.live.len(), MAX_LIVE_SESSIONS);
    }
}
//#endregion 🧪️Tests
