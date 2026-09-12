//! 🚀️ Remodeling reconstruction as a generation-tagged, bounded continuation.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::editor::remodeling::engine::images::{BoundedDecodeProgress, BoundedStillDecoder, CompressedChunkRope};
use crate::editor::remodeling::engine::{build_engine_params, camera_pose_preview, reconstruction as remodeling_engine, watertight_snapshot, RasterPngPreparation, RasterPngProgress};
use crate::mutations::{commit_reconstruction, create_asset, replace_job, CommitReconstruction, CreateAsset, ReconstructionAssetCommit};
use crate::op::RemodelingMutation;
use crate::schema::next_remodeling_id;
use crate::{CameraPosePreview, CameraTrajectory, FrameRef, GeoProducts, ImageAsset, MeshSource, PackedF32, QcReportSnapshot, ReconstructionJob, ReconstructionStage, RemodelingMesh, RemodelingSnapshot, SparseCloud, WatertightReportSnapshot};
use semio_framework::kernel::{Effect, UiDirtyScope};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, RequestId};
use semio_framework_value_derive::{FromValue, ToValue};
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
    mesh_result: Option<Box<RemodelingMesh>>,
    geo_result: Option<GeoProducts>,
    assets: Vec<ReconstructionAssetCommit>,
    watertight: Option<WatertightReportSnapshot>,
    mesh: Option<MeshPreparation>,
    geo_preparation: Option<remodeling_engine::TerminalGeoPreparation>,
    geo: Option<remodeling_engine::GeoProducts>,
    pending_dtm: Option<crate::editor::remodeling::engine::geo::Raster>,
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
        format!("remodeling-asset-{:016x}{:016x}{:016x}{:016x}-{:016x}", self.digest[0], self.digest[1], self.digest[2], self.digest[3], self.digest_len)
    }

    fn handle(&self) -> String {
        crate::remodeling_asset_content_handle(&self.content_id(), &self.staging_id, self.chunk_count)
    }
}

struct RasterAssetPreparation {
    encoder: RasterPngPreparation,
    staging_id: String,
    asset_id: String,
}

enum RasterAssetProgress {
    Working,
    CreateAsset(CreateAsset),
    Complete(ReconstructionAssetCommit),
    Failed,
}

impl RasterAssetPreparation {
    fn new(raster: crate::editor::remodeling::engine::geo::Raster, staging_id: String, asset_id: String) -> Self {
        Self { encoder: RasterPngPreparation::new(raster), staging_id, asset_id }
    }

    fn advance(&mut self) -> RasterAssetProgress {
        match self.encoder.advance(4_096) {
            RasterPngProgress::Working => RasterAssetProgress::Working,
            RasterPngProgress::Chunk(bytes) => {
                let Some(index) = self.encoder.chunk_count().checked_sub(1) else { return RasterAssetProgress::Failed };
                RasterAssetProgress::CreateAsset(CreateAsset {
                    key: crate::remodeling_asset_stage_key(&self.staging_id, crate::RemodelingAssetContentKind::Raster, index),
                    asset: ImageAsset { mime: "application/vnd.semio.asset-chunk".into(), data: base64_codec::base64_standard_encode(bytes), width: 0, height: 0 },
                })
            }
            RasterPngProgress::Complete => RasterAssetProgress::Complete(self.commit_asset(&self.encoder.content_id())),
            RasterPngProgress::Failed => RasterAssetProgress::Failed,
        }
    }

    fn commit_asset(&self, content_id: &str) -> ReconstructionAssetCommit {
        ReconstructionAssetCommit {
            id: self.asset_id.clone(),
            asset: ImageAsset { mime: "image/png".into(), data: crate::remodeling_asset_content_handle(content_id, &self.staging_id, self.encoder.chunk_count()), width: self.encoder.width(), height: self.encoder.height() },
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
        format!("remodeling-mesh-{:016x}{:016x}{:016x}{:016x}-{:016x}", self.digest[0], self.digest[1], self.digest[2], self.digest[3], self.digest_len)
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
    engine: remodeling_engine::ReconstructionEngine,
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
    decoded_image: Option<crate::editor::remodeling::engine::images::ImageRgba8>,
    sharpness_cursor: usize,
    sharpness_sum: f64,
}

fn frame_ingestion(scene: &RemodelingSnapshot, frame: &FrameRef) -> Option<FrameIngestion> {
    let source = crate::remodeling_asset_chunk_source(scene, &frame.asset_id)?;
    let compressed = CompressedChunkRope::from_leaves(source.leaves, MAX_STILL_INPUT_BYTES).ok()?;
    Some(FrameIngestion { _asset_identity: source.identity, mime: source.mime, frame_index: frame.index, timestamp_ms: frame.timestamp_ms, compressed, decoder: None, decoded_image: None, sharpness_cursor: 0, sharpness_sum: 0.0 })
}

fn rgba_luma(image: &crate::editor::remodeling::engine::images::ImageRgba8, index: usize) -> f64 {
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
    sessions().lock().expect("remodeling reconstruction sessions lock").live.remove(&generation)
}

fn admit_session(generation: u64, job_id: &str) -> bool {
    let removed = {
        let mut sessions = sessions().lock().expect("remodeling reconstruction sessions lock");
        sessions.admit(generation, job_id)
    };
    let Ok(removed) = removed else { return false };
    for session in &removed {
        discard_session_staging(session);
    }
    true
}

fn put_session(generation: u64, session: ReconstructionSession) {
    let mut sessions = sessions().lock().expect("remodeling reconstruction sessions lock");
    if sessions.admitted.contains(&generation) {
        sessions.live.insert(generation, session);
    }
}

fn cancel_session(generation: u64) {
    let removed = {
        let mut sessions = sessions().lock().expect("remodeling reconstruction sessions lock");
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
        crate::discard_staged_remodeling_mesh(staging_id);
    }
    if let Some(staging_id) = terminal.raster_asset.as_ref().map(|asset| asset.staging_id.as_str()) {
        crate::discard_staged_remodeling_asset(staging_id);
    }
    for staging_id in &terminal.completed_asset_staging_ids {
        crate::discard_staged_remodeling_asset(staging_id);
    }
    if terminal.sparse_content.chunk_count > 0 {
        crate::discard_staged_remodeling_asset(&terminal.sparse_content.staging_id);
    }
}

fn discard_session_staging(session: &ReconstructionSession) {
    if let Some(terminal) = &session.terminal {
        discard_terminal_staging(terminal);
    }
}

/// 🛑️ Cancels every worker-portable continuation for the current document generation and
/// emits replayable bounded cleanup for any privately staged mesh chunks.
pub fn cancel_current_reconstruction(scene: &RemodelingSnapshot) -> Emit<RemodelingMutation, NoConfigMutation> {
    let cancelled_sessions = {
        let mut sessions = sessions().lock().expect("remodeling reconstruction sessions lock");
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

    fn engine(self) -> remodeling_engine::EngineStage {
        match self {
            Self::Full => remodeling_engine::EngineStage::Done,
            Self::ExtractingFeatures => remodeling_engine::EngineStage::ExtractingFeatures,
            Self::MatchingFeatures => remodeling_engine::EngineStage::MatchingFeatures,
            Self::EstimatingPoses => remodeling_engine::EngineStage::EstimatingPoses,
            Self::BundleAdjusting => remodeling_engine::EngineStage::BundleAdjusting,
            Self::DenseStereo => remodeling_engine::EngineStage::DenseStereo,
            Self::FusingVolume => remodeling_engine::EngineStage::FusingVolume,
            Self::ExtractingSurface => remodeling_engine::EngineStage::ExtractingSurface,
            Self::CleaningMesh => remodeling_engine::EngineStage::CleaningMesh,
            Self::Texturing => remodeling_engine::EngineStage::Texturing,
        }
    }

    fn needs_terminal_products(self) -> bool {
        matches!(self, Self::Full | Self::Texturing)
    }
}

fn engine_stage_rank(stage: remodeling_engine::EngineStage) -> usize {
    match stage {
        remodeling_engine::EngineStage::Idle => 0,
        remodeling_engine::EngineStage::ExtractingFeatures => 1,
        remodeling_engine::EngineStage::MatchingFeatures => 2,
        remodeling_engine::EngineStage::EstimatingPoses => 3,
        remodeling_engine::EngineStage::BundleAdjusting => 4,
        remodeling_engine::EngineStage::DenseStereo => 5,
        remodeling_engine::EngineStage::FusingVolume => 6,
        remodeling_engine::EngineStage::ExtractingSurface => 7,
        remodeling_engine::EngineStage::CleaningMesh => 8,
        remodeling_engine::EngineStage::Texturing => 9,
        remodeling_engine::EngineStage::Done | remodeling_engine::EngineStage::Failed => 10,
    }
}

fn requested_stage_complete(requested: RequestedStage, current: remodeling_engine::EngineStage) -> bool {
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

fn emit_step(job: ReconstructionJob, generation: u64, next: Option<&AdvanceReconstruction>) -> Emit<RemodelingMutation, NoConfigMutation> {
    Emit { artifact_mutations: vec![replace_job(job)], coalesce_key: Some(format!("reconstruction:{generation}")), effects: next.map(queue).into_iter().collect(), ui_scope: UiDirtyScope::Full, ..Default::default() }
}

fn next_frame_cursor(scene: &RemodelingSnapshot, stream_index: u32, frame_index: u32) -> Result<(u32, u32), Fault> {
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

fn preview_job(job_id: String, stage: ReconstructionStage, progress: f32, stage_cursor: u32, engine: &remodeling_engine::ReconstructionEngine) -> ReconstructionJob {
    let preview = engine.sparse_preview_bounded(PREVIEW_CAMERA_LIMIT, PREVIEW_POINT_LIMIT);
    let mut camera_poses_preview = Vec::with_capacity(preview.camera_poses.len());
    for (index, pose) in preview.camera_poses.iter().enumerate() {
        camera_poses_preview.push(camera_pose_preview(index as u32, pose));
    }
    ReconstructionJob { id: job_id, stage, progress_0_1: progress, cancel_requested: false, stage_cursor, started_at_ms: None, error: None, camera_poses_preview, sparse_point_cloud_preview: PackedF32::from_f32_slice(&preview.packed_points) }
}

fn reconstruction_stage(stage: remodeling_engine::EngineStage) -> ReconstructionStage {
    match stage {
        remodeling_engine::EngineStage::Idle => ReconstructionStage::Ingesting,
        remodeling_engine::EngineStage::ExtractingFeatures => ReconstructionStage::ExtractingFeatures,
        remodeling_engine::EngineStage::MatchingFeatures => ReconstructionStage::MatchingFeatures,
        remodeling_engine::EngineStage::EstimatingPoses => ReconstructionStage::EstimatingPoses,
        remodeling_engine::EngineStage::BundleAdjusting => ReconstructionStage::BundleAdjusting,
        remodeling_engine::EngineStage::DenseStereo => ReconstructionStage::DenseStereo,
        remodeling_engine::EngineStage::FusingVolume => ReconstructionStage::FusingVolume,
        remodeling_engine::EngineStage::ExtractingSurface => ReconstructionStage::ExtractingSurface,
        remodeling_engine::EngineStage::CleaningMesh => ReconstructionStage::CleaningMesh,
        remodeling_engine::EngineStage::Texturing => ReconstructionStage::Texturing,
        remodeling_engine::EngineStage::Done => ReconstructionStage::Done,
        remodeling_engine::EngineStage::Failed => ReconstructionStage::Failed,
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
pub fn begin_reconstruction(doc: &ArtifactView<'_, RemodelingSnapshot>) -> Emit<RemodelingMutation, NoConfigMutation> {
    begin_requested_reconstruction(doc, RequestedStage::Full)
}

/// 🎯️ Starts a fresh dependency-prefix generation ending at the requested pipeline stage.
pub fn begin_stage_reconstruction(doc: &ArtifactView<'_, RemodelingSnapshot>, requested_stage: &str) -> Emit<RemodelingMutation, NoConfigMutation> {
    let Some(requested_stage) = RequestedStage::parse(requested_stage) else { return Emit::default() };
    begin_requested_reconstruction(doc, requested_stage)
}

fn begin_requested_reconstruction(doc: &ArtifactView<'_, RemodelingSnapshot>, requested_stage: RequestedStage) -> Emit<RemodelingMutation, NoConfigMutation> {
    let scene = doc.snapshot;
    if scene.streams.iter().all(|stream| stream.frames.is_empty()) {
        return Emit::default();
    }
    let generation = NEXT_RECONSTRUCTION_GENERATION.fetch_add(1, Ordering::Relaxed);
    let job_id = next_remodeling_id("job");
    let engine_params = build_engine_params(&scene.params, &scene.calibration);
    let session = ReconstructionSession {
        job_id: job_id.clone(),
        artifact_authority: doc.operation_optional().map_or_else(
            || format!("document={}:app=test:operation={job_id}:generation={generation}", scene.id),
            |operation| format!("document={}:app={}:operation={}:generation={}", operation.parent_document_id, operation.app_instance_id, operation.operation_id, operation.generation),
        ),
        engine: remodeling_engine::ReconstructionEngine::new(&engine_params),
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
        id: job_id,
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
        return emit_step(job, generation, None);
    }
    put_session(generation, session);
    emit_step(job, generation, Some(&next))
}

fn packed_chunk_base64(values: &[f32]) -> String {
    let bytes: Vec<u8> = values.iter().flat_map(|value| value.to_le_bytes()).collect();
    base64_codec::base64_standard_encode(bytes)
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

fn yield_terminal(generation: u64, session: ReconstructionSession, mutation: Option<RemodelingMutation>) -> Emit<RemodelingMutation, NoConfigMutation> {
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

fn advance_terminal(generation: u64, mut session: ReconstructionSession) -> Result<Emit<RemodelingMutation, NoConfigMutation>, Fault> {
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
                    crate::remodeling_asset_stage_key(&terminal.sparse_content.staging_id, crate::RemodelingAssetContentKind::Sparse, index),
                    ImageAsset { mime: "application/vnd.semio.asset-chunk".into(), data: base64_codec::base64_standard_encode(bytes), width: 0, height: 0 },
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
                terminal.watertight = session.engine.terminal_watertight_report().as_ref().map(watertight_snapshot);
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
                        crate::remodeling_mesh_stage_asset_key(&preparation.staging_id, index),
                        ImageAsset { mime: "application/vnd.semio.mesh-chunk".into(), data: base64_codec::base64_standard_encode(chunk), width: 0, height: 0 },
                    ));
                } else {
                    let content_id = preparation.content_id();
                    let handle = crate::staged_remodeling_mesh_handle(&content_id, &preparation.staging_id);
                    terminal.mesh_result = Some(Box::new(RemodelingMesh { mesh: handle, source: MeshSource::Reconstructed, texture_asset_id: None, watertight: terminal.watertight.take() }));
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
                terminal.geo = remodeling_engine::ReconstructionEngine::finish_terminal_geo(terminal.geo_preparation.take().expect("completed terminal geo preparation"));
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
                RasterAssetProgress::CreateAsset(mutation) => step_mutation = Some(RemodelingMutation::CreateAsset(mutation)),
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
                RasterAssetProgress::CreateAsset(mutation) => step_mutation = Some(RemodelingMutation::CreateAsset(mutation)),
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
pub fn advance_reconstruction(payload: &AdvanceReconstruction, doc: &ArtifactView<'_, RemodelingSnapshot>) -> Result<Emit<RemodelingMutation, NoConfigMutation>, Fault> {
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
        return Ok(emit_step(job, payload.generation, Some(&next)));
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
        return Ok(emit_step(job, payload.generation, Some(&next)));
    }
    let status = session.engine.advance(RECONSTRUCTION_STEP_BUDGET);
    session.tick = session.tick.saturating_add(1);
    match status {
        remodeling_engine::EngineStatus::Working { stage, progress: _ } if requested_stage_complete(session.requested_stage, stage) => {
            complete_session(payload.generation);
            let job = ReconstructionJob { stage: ReconstructionStage::Done, progress_0_1: 1.0, stage_cursor: session.tick, ..scene.job.clone() };
            Ok(emit_step(job, payload.generation, None))
        }
        remodeling_engine::EngineStatus::Working { stage, progress } => {
            let job = preview_job(payload.job_id.clone(), reconstruction_stage(stage), progress, session.tick, &session.engine);
            let next = checkpoint(payload.generation, &session);
            put_session(payload.generation, session);
            Ok(emit_step(job, payload.generation, Some(&next)))
        }
        remodeling_engine::EngineStatus::Done if session.requested_stage.needs_terminal_products() => {
            session.terminal = Some(terminal_preparation(payload.generation, &session.artifact_authority));
            Ok(yield_terminal(payload.generation, session, None))
        }
        remodeling_engine::EngineStatus::Done => {
            complete_session(payload.generation);
            let job = ReconstructionJob { stage: ReconstructionStage::Done, progress_0_1: 1.0, stage_cursor: session.tick, ..scene.job.clone() };
            Ok(emit_step(job, payload.generation, None))
        }
        remodeling_engine::EngineStatus::Failed(message) => {
            complete_session(payload.generation);
            let job = ReconstructionJob { stage: ReconstructionStage::Failed, error: Some(message), stage_cursor: session.tick, ..scene.job.clone() };
            Ok(emit_step(job, payload.generation, None))
        }
    }
}
//#endregion 🔖️Run

//#region 🔖️Payloads
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "run-reconstruction")]
pub struct RunReconstruction {}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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

pub fn handle(_payload: &RunReconstruction, doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<RemodelingMutation, NoConfigMutation>, Fault> {
    Ok(begin_reconstruction(doc))
}

pub fn handle_advance(payload: &AdvanceReconstruction, doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<RemodelingMutation, NoConfigMutation>, Fault> {
    advance_reconstruction(payload, doc)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
