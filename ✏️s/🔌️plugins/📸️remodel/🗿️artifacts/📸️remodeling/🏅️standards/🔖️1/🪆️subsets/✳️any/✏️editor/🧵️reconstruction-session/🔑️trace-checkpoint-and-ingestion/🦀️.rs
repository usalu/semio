//! 🔑️ Trace keys, checkpoint and ingestion of the reconstruction session: the per-stage trace keys, the run checkpoint, the run inputs and the bounded frame ingestion with its sharpness. See [`super`].

use super::*;

//#region 🔑️TraceKeys
const KEY_FRAME: u64 = 1 << 56;

pub(super) const KEY_FEATURES: u64 = 2 << 56;

pub(super) const KEY_MATCH: u64 = 3 << 56;

const KEY_CAMERA: u64 = 4 << 56;

const KEY_POINT: u64 = 5 << 56;

pub(super) const KEY_DENSE: u64 = 6 << 56;

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

pub(super) fn point_subject(point: [f64; 3]) -> ToolRunTraceSubject {
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
    digest.record(semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&snapshot.params)).as_bytes());
    digest.record(semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&snapshot.calibration)).as_bytes());
    digest
}
//#endregion 🔖️Checkpoint

//#region 🔖️Ingestion
#[derive(semio_framework_value::RetireOwned)]
pub(super) struct FrameIngestion {
    pub(super) frame: FrameRef,
    pub(super) decoder: Option<BoundedStillDecoder>,
    pub(super) image: Option<ImageRgba8>,
    sharpness_cursor: usize,
    pub(super) sharpness_sum: f64,
}

pub(super) fn frame_ingestion(scene: &RemodelingSnapshot, frame: &FrameRef) -> Option<FrameIngestion> {
    let source = semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::remodeling_asset_chunk_source(scene, &frame.asset_id)?;
    let compressed = CompressedChunkRope::from_leaves(source.leaves, MAX_STILL_INPUT_BYTES).ok()?;
    Some(FrameIngestion { frame: frame.clone(), decoder: Some(BoundedStillDecoder::new(&source.mime, compressed)), image: None, sharpness_cursor: 0, sharpness_sum: 0.0 })
}

fn rgba_luma(image: &ImageRgba8, index: usize) -> f64 {
    let offset = index * 4;
    (0.299 * f64::from(image.data[offset]) + 0.587 * f64::from(image.data[offset + 1]) + 0.114 * f64::from(image.data[offset + 2])) / 255.0
}

pub(super) fn mirror(coordinate: i64, length: usize) -> usize {
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
pub(super) fn advance_frame_sharpness(ingestion: &mut FrameIngestion) -> bool {
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
