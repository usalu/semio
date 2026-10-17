//! 📥️ Frame ingest of the reconstruction engine: the ingest parameters, the frame acceptance with its blur gate and the frame source. See [`super`].

use super::*;

// #region 🔖️Input
pub(super) const MAX_INTERACTIVE_IMAGE_PIXELS: usize = 262_144;

pub(super) const MAX_INTERACTIVE_IMAGE_BYTES: usize = MAX_INTERACTIVE_IMAGE_PIXELS * 4;

/// 🎚️ Frame-ingestion policy shared by [`FrameSource::push_frame`] and [`FrameSource::push_video`]:
/// `stride` keeps every `stride`-th *offered* frame (applied only to the direct [`FrameSource::push_frame`]
/// entry point — [`FrameSource::push_video`] instead relies on the container-level stride already applied
/// by `crate::editor::remodeling::engine::video::extract_frames`, so it isn't double-applied), `max_frames` caps the total accepted
/// count (`0` = unbounded), and `min_sharpness`/`rolling_window` drive the relative blur gate: a frame is
/// rejected when its gradient-based sharpness score falls below `min_sharpness * median(last
/// rolling_window accepted scores)`, once at least 3 accepted frames exist to form a baseline.
#[derive(Clone, Debug, PartialEq, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct IngestParams {
    pub stride: u32,
    pub max_frames: u32,
    pub min_sharpness: f32,
    pub rolling_window: usize,
}

impl Default for IngestParams {
    fn default() -> Self {
        Self { stride: 1, max_frames: 0, min_sharpness: 0.3, rolling_window: BLUR_GATE_ROLLING_WINDOW }
    }
}

/// 🚦️ What [`FrameSource::push_frame`]/[`FrameSource::push_video`] did with one offered frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrameAcceptance {
    Accepted,
    RejectedStride,
    RejectedMaxFrames,
    RejectedBlur,
}

/// 🖼️ One accepted input frame: pixels, true media timestamp, originating stream/camera id (single
/// stream, id `0`, until [`FrameSource`] grows multi-stream support), and the sharpness score that let
/// it through the blur gate.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct AcceptedFrame {
    pub index: u32,
    pub image: remodeling_image::ImageRgba8,
    pub timestamp_ms: f64,
    pub stream_id: u32,
    pub sharpness: f32,
}

/// ⚠️ Errors from this crate's own fallible entry points — currently just video ingestion, re-exporting
/// `crate::editor::remodeling::engine::video::VideoError` so callers get the precise failure (truncated container, unsupported
/// codec, malformed box, …) rather than a lossy wrapper.
#[derive(Clone, Debug, PartialEq)]
pub enum EngineError {
    Video(semio_s_artifact_remodel_remodeling_video::editor::remodeling::engine::video::VideoError),
}

impl std::fmt::Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Video(e) => write!(f, "video ingest error: {e}"),
        }
    }
}

impl std::error::Error for EngineError {}

impl From<semio_s_artifact_remodel_remodeling_video::editor::remodeling::engine::video::VideoError> for EngineError {
    fn from(e: semio_s_artifact_remodel_remodeling_video::editor::remodeling::engine::video::VideoError) -> Self {
        Self::Video(e)
    }
}

/// 📊️ Outcome of [`FrameSource::push_video`]: how many samples the container yielded, how many the
/// ingestion policy accepted vs. rejected (split by reason), and provenance from the probe.
#[derive(Clone, Debug, PartialEq)]
pub struct PushVideoReport {
    pub frames_extracted: u32,
    pub frames_accepted: u32,
    pub frames_rejected_blur: u32,
    pub frames_rejected_sampling: u32,
    pub codec: String,
    pub width: u32,
    pub height: u32,
    pub duration_ms: f64,
}

/// 📏️ How many recent accepted sharpness scores the relative blur gate compares a frame against by default.
pub const BLUR_GATE_ROLLING_WINDOW: usize = 15;

const BLUR_GATE_MIN_SAMPLES: usize = 3;

/// 🧭️ Gradient-energy sharpness proxy (mean squared Scharr gradient magnitude): high for crisp edges,
/// collapsing toward zero for a flat/blurred frame — the signal the relative blur gate thresholds
/// against.
pub fn sharpness_score(image: &remodeling_image::ImageRgba8) -> f32 {
    let gray = remodeling_image::ImageGray::from_rgba8_luma(image);
    let grad = remodeling_image::scharr_gradients(&gray);
    if grad.gx.is_empty() {
        return 0.0;
    }
    let sum_sq: f32 = grad.gx.iter().zip(grad.gy.iter()).map(|(&gx, &gy)| gx * gx + gy * gy).sum();
    sum_sq / grad.gx.len() as f32
}

/// 🚦️ The ONE relative blur gate every ingestion runs (the engine's [`FrameSource`], a streamed import's ticks, the
/// in-process video sampler): once `rolling` holds [`BLUR_GATE_MIN_SAMPLES`] accepted scores, a frame whose `score`
/// falls below `min_sharpness` × their median (the middle element of the sorted copy) is refused; an admitted score
/// joins `rolling`, the oldest leaving past `window`. O(window) per frame, never a re-decode of an earlier frame.
pub fn blur_gate_admits(rolling: &mut Vec<f32>, window: usize, score: f32, min_sharpness: f32) -> bool {
    if rolling.len() >= BLUR_GATE_MIN_SAMPLES {
        let mut sorted = rolling.clone();
        sorted.sort_by(f32::total_cmp);
        if score < min_sharpness * sorted[sorted.len() / 2] {
            return false;
        }
    }
    if rolling.len() >= window.max(1) {
        rolling.remove(0);
    }
    rolling.push(score);
    true
}

/// 🏷️ Codec, dimensions and duration from the owned provider record.
#[cfg(test)]
fn describe_probe(probe: &semio_s_artifact_remodel_remodeling_video::editor::remodeling::engine::video::VideoProbe) -> (String, u32, u32, f64) {
    (format!("{:?}", probe.codec), probe.width, probe.height, probe.duration_ms)
}

/// 📥️ Accumulates accepted input frames for one reconstruction: the real ingestion point where
/// stride/max-frame sampling and the gradient-score blur-rejection gate actually run, for both
/// direct-frame ([`push_frame`](Self::push_frame)) and video ([`push_video`](Self::push_video)) input.
#[derive(semio_framework_value::RetireOwned)]
pub struct FrameSource {
    pub(super) ingest: IngestParams,
    stream_id: u32,
    offered: u32,
    pub(super) frames: Vec<AcceptedFrame>,
    rolling_scores: Vec<f32>,
}

impl FrameSource {
    /// 🆕️ An empty frame source under the given ingestion policy.
    pub fn new(ingest: IngestParams) -> Self {
        Self { ingest, stream_id: 0, offered: 0, frames: Vec::new(), rolling_scores: Vec::new() }
    }

    /// 🔍️ Every frame accepted so far, in ingestion order.
    pub fn frames(&self) -> &[AcceptedFrame] {
        &self.frames
    }

    /// 🔢️ How many frames have been accepted so far.
    pub fn accepted_count(&self) -> usize {
        self.frames.len()
    }

    /// 📥️ Offers one directly-provided frame (e.g. an imported image sequence): applies this source's
    /// `stride`/`max_frames` sampling, then the relative blur gate.
    #[cfg(test)]
    pub fn push_frame(&mut self, index: u32, image: remodeling_image::ImageRgba8, timestamp_ms: f64) -> FrameAcceptance {
        self.accept(index, image, timestamp_ms, true)
    }

    /// 🎞️ Probes `bytes` as a video container, lazily decodes sampled frames via
    /// `crate::editor::remodeling::engine::video::extract_frames` (container-level stride/max-frames/downscale already applied per
    /// `opts`), and offers each decoded frame through the same blur gate as [`push_frame`](Self::push_frame)
    /// (without re-applying this source's own stride counter, since the container already sampled).
    #[cfg(test)]
    pub fn push_video(&mut self, bytes: &[u8], opts: &semio_s_artifact_remodel_remodeling_video::editor::remodeling::engine::video::VideoIngestOptions) -> Result<PushVideoReport, EngineError> {
        let probe = semio_s_artifact_remodel_remodeling_video::editor::remodeling::engine::video::probe(bytes)?;
        let (codec, width, height, duration_ms) = describe_probe(&probe);
        let iter = semio_s_artifact_remodel_remodeling_video::editor::remodeling::engine::video::extract_frames(bytes, opts)?;
        let mut report = PushVideoReport { frames_extracted: 0, frames_accepted: 0, frames_rejected_blur: 0, frames_rejected_sampling: 0, codec, width, height, duration_ms };
        for extracted in iter {
            let extracted = extracted?;
            report.frames_extracted += 1;
            match self.accept(extracted.index, extracted.image, extracted.timestamp_ms, false) {
                FrameAcceptance::Accepted => report.frames_accepted += 1,
                FrameAcceptance::RejectedBlur => report.frames_rejected_blur += 1,
                FrameAcceptance::RejectedStride | FrameAcceptance::RejectedMaxFrames => report.frames_rejected_sampling += 1,
            }
        }
        Ok(report)
    }

    /// 🚦️ Shared gate: optional stride counting, then `max_frames`, then the relative blur threshold
    /// against the rolling median of recently accepted scores.
    #[cfg(test)]
    fn accept(&mut self, index: u32, image: remodeling_image::ImageRgba8, timestamp_ms: f64, apply_stride: bool) -> FrameAcceptance {
        let score = sharpness_score(&image);
        self.accept_with_sharpness(index, image, timestamp_ms, apply_stride, score)
    }

    pub(super) fn accept_with_sharpness(&mut self, index: u32, image: remodeling_image::ImageRgba8, timestamp_ms: f64, apply_stride: bool, score: f32) -> FrameAcceptance {
        let offered = self.offered;
        self.offered += 1;
        if apply_stride {
            let stride = self.ingest.stride.max(1);
            if !offered.is_multiple_of(stride) {
                return FrameAcceptance::RejectedStride;
            }
        }
        if self.ingest.max_frames != 0 && self.frames.len() as u32 >= self.ingest.max_frames {
            return FrameAcceptance::RejectedMaxFrames;
        }
        if !blur_gate_admits(&mut self.rolling_scores, self.ingest.rolling_window, score, self.ingest.min_sharpness) {
            return FrameAcceptance::RejectedBlur;
        }
        self.frames.push(AcceptedFrame { index, image, timestamp_ms, stream_id: self.stream_id, sharpness: score });
        FrameAcceptance::Accepted
    }
}
// #endregion 🔖️Input
