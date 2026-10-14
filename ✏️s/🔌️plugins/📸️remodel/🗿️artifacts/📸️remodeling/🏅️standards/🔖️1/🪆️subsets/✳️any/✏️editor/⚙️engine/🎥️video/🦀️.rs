//! 🎞️ Owned video provider admission, frame sampling and baseline AVC pixel decoding.

use semio_s_artifact_remodel_remodeling_images::editor::remodeling::engine::images as remodeling_image;
use semio_s_artifact_remodel_remodeling_video_h264::editor::remodeling::engine::video::h264::H264Decoder;
use semio_s_artifact_remodel_remodeling_video_h264::editor::remodeling::engine::video::h264::H264Error;


// #region 🔖️Bytes

/// 🧭️ Four-character box/chunk code (ISO-BMFF box types, RIFF FourCCs); compared and hashed by raw bytes.
#[derive(Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct FourCc(pub [u8; 4]);

impl FourCc {
    /// 🧭️ Builds a `FourCc` from an ASCII byte-string literal, e.g. `FourCc::new(b"moov")`.
    pub fn new(bytes: &[u8; 4]) -> Self {
        Self(*bytes)
    }
}

impl std::fmt::Debug for FourCc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.iter().all(|&b| (0x20..0x7F).contains(&b)) {
            write!(f, "FourCc({:?})", std::str::from_utf8(&self.0).unwrap_or("?"))
        } else {
            write!(f, "FourCc({:02x}{:02x}{:02x}{:02x})", self.0[0], self.0[1], self.0[2], self.0[3])
        }
    }
}

impl std::fmt::Display for FourCc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.iter().all(|&b| (0x20..0x7F).contains(&b)) {
            write!(f, "{}", std::str::from_utf8(&self.0).unwrap_or("????"))
        } else {
            write!(f, "{:02x}{:02x}{:02x}{:02x}", self.0[0], self.0[1], self.0[2], self.0[3])
        }
    }
}

/// ⚠️ Error type for every fallible operation in this crate: demux (now stdio's, wrapped verbatim
/// as `Container`), probe, decode. Malformed or truncated input always yields one of these —
/// decoders in this crate never panic on attacker-controlled bytes.
#[derive(Clone, Debug, PartialEq)]
pub enum VideoError {
    Truncated,
    /// 🕳️ Wraps a `String` error from stdio's `mp4::engine::decode_mp4`/`avi::engine::decode_avi`
    /// verbatim — replaces the old hand-rolled box/chunk parser's `BadBox(&'static str)` now that
    /// every container-level parse error genuinely originates from stdio (a dynamic message, not a
    /// fixed set of named box-parsing failures this file no longer implements).
    Container(String),
    ContainerAdmission(VideoContainerAdmissionErrorV1),
    InvalidProbe,
    NoVideoTrack,
    UnsupportedCodec(FourCc),
    Jpeg(remodeling_image::ImageError),
    H264(H264Error),
}

impl std::fmt::Display for VideoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Truncated => write!(f, "video container truncated"),
            Self::Container(msg) => write!(f, "video container error: {msg}"),
            Self::ContainerAdmission(error) => write!(f, "video container admission: {error}"),
            Self::InvalidProbe => f.write_str("invalid video container provider record"),
            Self::NoVideoTrack => write!(f, "container has no video track"),
            Self::UnsupportedCodec(fourcc) => write!(f, "unsupported video codec: {fourcc}"),
            Self::Jpeg(e) => write!(f, "jpeg error: {e}"),
            Self::H264(e) => write!(f, "h264 error: {e}"),
        }
    }
}

impl std::error::Error for VideoError {}

impl From<remodeling_image::ImageError> for VideoError {
    fn from(e: remodeling_image::ImageError) -> Self {
        Self::Jpeg(e)
    }
}

impl From<H264Error> for VideoError {
    fn from(e: H264Error) -> Self {
        Self::H264(e)
    }
}

/// 🎞️ Video codec identified from a container's sample description; `Unknown` carries the raw fourcc for
/// diagnostics even when this crate cannot decode it (routing the caller to a host decoder).
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[serde(tag = "kind", content = "fourcc", rename_all = "kebab-case", deny_unknown_fields)]
pub enum VideoCodec {
    Avc,
    Hevc,
    Vp9,
    Av1,
    Mjpeg,
    Unknown(FourCc),
}

fn fourcc_from_str(s: &str) -> FourCc {
    let mut out = [b' '; 4];
    for (i, b) in s.as_bytes().iter().take(4).enumerate() {
        out[i] = *b;
    }
    FourCc(out)
}

/// 🏷️ Classifies a container-reported fourcc string into [`VideoCodec`] — shared by both container
/// families (stdio's `Mp4CodecFormat::fourcc` sample entry and `AviStreamFormat::BitmapInfo::compression`).
fn codec_from_fourcc_str(fourcc: &str) -> VideoCodec {
    match fourcc.to_ascii_lowercase().as_str() {
        "avc1" | "avc3" => VideoCodec::Avc,
        "hvc1" | "hev1" => VideoCodec::Hevc,
        "vp09" => VideoCodec::Vp9,
        "av01" => VideoCodec::Av1,
        "mjpg" | "jpeg" | "mjpa" => VideoCodec::Mjpeg,
        _ => VideoCodec::Unknown(fourcc_from_str(fourcc)),
    }
}

// #endregion 🔖️Bytes

// #region 🔖️Container

/// 🎞️ Owned sample payload and presentation timestamp.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SampleInfo {
    pub data: Vec<u8>,
    pub timestamp_ms: f64,
}

/// ⏱️ Seconds per source clock tick.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VideoTimeBaseV1 {
    pub numerator: u32,
    pub denominator: u32,
}

/// 🎞️ Owned AVC sequence parameters, picture parameters and NAL length width.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AvcDecoderConfigurationV1 {
    pub sps: Vec<Vec<u8>>,
    pub pps: Vec<Vec<u8>>,
    pub nal_length_size: u8,
}

/// 🔍️ Container-independent video metadata and access units from an admitted provider.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VideoProbe {
    pub container: String,
    pub width: u32,
    pub height: u32,
    #[serde(deserialize_with = "deserialize_required_video_option_v1")]
    pub time_base: Option<VideoTimeBaseV1>,
    pub fps: f64,
    pub duration_ms: f64,
    pub frame_count: u32,
    pub codec: VideoCodec,
    pub samples: Vec<SampleInfo>,
    #[serde(deserialize_with = "deserialize_required_video_option_v1")]
    pub avc_config: Option<AvcDecoderConfigurationV1>,
}

fn deserialize_required_video_option_v1<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where D: serde::Deserializer<'de>, T: serde::Deserialize<'de> {
    serde::Deserialize::deserialize(deserializer)
}

/// 🔌️ Owned container recognition and sample projection callbacks.
#[derive(Clone, Copy)]
pub struct VideoContainerProviderV1<'a> {
    pub id: &'a str,
    pub matches: fn(&[u8]) -> bool,
    pub probe: fn(&[u8]) -> Result<VideoProbe, VideoError>,
}

/// 🚦️ Refusal of absent, ambiguous or malformed capability inventories.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VideoContainerAdmissionErrorV1 {
    MissingProvider,
    AmbiguousProvider,
    InvalidInventory,
}

impl std::fmt::Display for VideoContainerAdmissionErrorV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::MissingProvider => "no video container provider matches",
            Self::AmbiguousProvider => "multiple video container providers match",
            Self::InvalidInventory => "invalid video container provider inventory",
        })
    }
}

impl std::error::Error for VideoContainerAdmissionErrorV1 {}

fn valid_video_provider_id_v1(id: &str) -> bool {
    !id.is_empty() && id.len() <= 64 && id.as_bytes()[0].is_ascii_lowercase()
        && id.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'-'))
}

/// 🚦️ Selects exactly one provider after validating the entire bounded inventory.
pub fn select_video_container_provider_v1(providers: &[VideoContainerProviderV1<'_>], bytes: &[u8]) -> Result<usize, VideoContainerAdmissionErrorV1> {
    if providers.len() > 64 || providers.iter().enumerate().any(|(index, provider)| !valid_video_provider_id_v1(provider.id) || providers[..index].iter().any(|prior| prior.id == provider.id)) {
        return Err(VideoContainerAdmissionErrorV1::InvalidInventory);
    }
    let mut selected = None;
    for (index, provider) in providers.iter().enumerate() {
        if (provider.matches)(bytes) {
            if selected.is_some() {
                return Err(VideoContainerAdmissionErrorV1::AmbiguousProvider);
            }
            selected = Some(index);
        }
    }
    selected.ok_or(VideoContainerAdmissionErrorV1::MissingProvider)
}

/// 🛂️ Admits an owned provider record without allocating a second sample buffer.
pub fn admit_video_probe_v1(provider_id: &str, probe: &VideoProbe) -> bool {
    valid_video_provider_id_v1(provider_id) && probe.container == provider_id
        && probe.samples.len() <= 1_048_576 && probe.frame_count as usize == probe.samples.len()
        && probe.fps.is_finite() && probe.fps >= 0.0 && probe.duration_ms.is_finite() && probe.duration_ms >= 0.0
        && probe.time_base.is_none_or(|base| base.numerator > 0 && base.denominator > 0)
        && probe.samples.iter().all(|sample| sample.timestamp_ms.is_finite() && sample.data.len() <= 16_777_216)
        && probe.avc_config.as_ref().is_none_or(|configuration| (1..=4).contains(&configuration.nal_length_size)
            && configuration.sps.len() <= 65_535 && configuration.pps.len() <= 65_535
            && configuration.sps.iter().chain(&configuration.pps).all(|nal| nal.len() <= 65_535))
}

/// 🔍️ Invokes the uniquely selected callback and admits its first-party record.
pub fn probe_with_providers_v1(providers: &[VideoContainerProviderV1<'_>], bytes: &[u8]) -> Result<VideoProbe, VideoError> {
    let provider = &providers[select_video_container_provider_v1(providers, bytes).map_err(VideoError::ContainerAdmission)?];
    let probe = (provider.probe)(bytes)?;
    if !admit_video_probe_v1(provider.id, &probe) {
        return Err(VideoError::InvalidProbe);
    }
    Ok(probe)
}

/// 🔌️ Capabilities selected by this artifact's authored composition.
#[path = "🔌️providers/🦀️.rs"]
pub mod container_providers;

// #endregion 🔖️Container

// #region 🔖️Extract

/// 🎚️ Frame-sampling knobs for [`extract_frames`]: `stride` keeps every `stride`-th sample (`0` treated as
/// `1`), `max_frames` caps the total sampled count (`0` means unbounded), `max_long_edge_px` downscales any
/// frame whose longer side exceeds it (`0` means no downscaling).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VideoIngestOptions {
    pub stride: u32,
    pub max_frames: u32,
    pub max_long_edge_px: u32,
}

/// 🔍️ Probes with the artifact owner's currently linked capabilities.
pub fn probe(bytes: &[u8]) -> Result<VideoProbe, VideoError> {
    probe_with_providers_v1(&container_providers::inventory_v1(), bytes)
}

/// 🏷️ A human/log-facing fourcc for a codec, for [`VideoError::UnsupportedCodec`] diagnostics.
fn codec_fourcc_hint(codec: VideoCodec) -> FourCc {
    match codec {
        VideoCodec::Avc => FourCc(*b"avc1"),
        VideoCodec::Hevc => FourCc(*b"hvc1"),
        VideoCodec::Vp9 => FourCc(*b"vp09"),
        VideoCodec::Av1 => FourCc(*b"av01"),
        VideoCodec::Mjpeg => FourCc(*b"mjpg"),
        VideoCodec::Unknown(fourcc) => fourcc,
    }
}

/// 🖼️ One sampled, fully decoded frame: its sample index (into the container's original sample table,
/// *before* stride/max_frames selection), true media timestamp, and image.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtractedFrame {
    pub index: u32,
    pub timestamp_ms: f64,
    pub image: remodeling_image::ImageRgba8,
}

/// 🔻️ Box-filtered (bilinear-sampled) downscale so the longer side is at most `max_long_edge_px`; a no-operation
/// when already within budget or `max_long_edge_px == 0`. Built on [`remodeling_image::ImageRgba8::sample_rgb`]
/// rather than a new resize primitive in `remodeling_image` itself.
fn resize_to_max_long_edge(img: remodeling_image::ImageRgba8, max_long_edge_px: u32) -> remodeling_image::ImageRgba8 {
    if max_long_edge_px == 0 || img.width == 0 || img.height == 0 {
        return img;
    }
    let long_edge = img.width.max(img.height);
    if long_edge <= max_long_edge_px {
        return img;
    }
    let scale = f64::from(max_long_edge_px) / f64::from(long_edge);
    let new_w = ((f64::from(img.width) * scale).round() as u32).max(1);
    let new_h = ((f64::from(img.height) * scale).round() as u32).max(1);
    let mut out = remodeling_image::ImageRgba8::new(new_w, new_h);
    for y in 0..new_h {
        for x in 0..new_w {
            let sx = ((f64::from(x) + 0.5) / scale - 0.5).max(0.0) as f32;
            let sy = ((f64::from(y) + 0.5) / scale - 0.5).max(0.0) as f32;
            let [r, g, b] = img.sample_rgb(sx, sy);
            let idx = ((y * new_w + x) * 4) as usize;
            out.data[idx] = (r * 255.0).round().clamp(0.0, 255.0) as u8;
            out.data[idx + 1] = (g * 255.0).round().clamp(0.0, 255.0) as u8;
            out.data[idx + 2] = (b * 255.0).round().clamp(0.0, 255.0) as u8;
            out.data[idx + 3] = 255;
        }
    }
    out
}

/// 🎞️ Which selected sample indices (post-`stride`/`max_frames`) to output.
fn select_sample_indices(total: usize, opts: &VideoIngestOptions) -> Vec<usize> {
    let stride = opts.stride.max(1) as usize;
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < total && (opts.max_frames == 0 || out.len() < opts.max_frames as usize) {
        out.push(i);
        i += stride;
    }
    out
}

/// 🧬️ SPS/PPS as separate NAL lists (stdio's `Mp4Codec` shape) flattened back into
/// [`H264Decoder::new`]'s expected `(u16 length, NAL)*` format.
fn flatten_sps_pps(sps_list: &[Vec<u8>], pps_list: &[Vec<u8>]) -> Vec<u8> {
    let mut out = Vec::new();
    for nal in sps_list.iter().chain(pps_list.iter()) {
        out.extend_from_slice(&(nal.len() as u16).to_be_bytes());
        out.extend_from_slice(nal);
    }
    out
}

/// 🐌️ Lazy frame decoder over a probed sample table: one decode per [`Iterator::next`], chunkable by callers.
/// MJPEG frames are independently decodable, so unselected samples are never even opened; baseline AVC's
/// P-frame chain means every sample up to (and including) each selected target must still be decoded — only
/// samples *trailing after* the last ever-needed one are skipped. Owns its samples' bytes directly (stdio's
/// `decode_mp4`/`decode_avi` already extract them) — no source-buffer lifetime to carry, unlike this file's
/// pre-extraction `FrameIter`.
pub struct FrameIter {
    samples: Vec<SampleInfo>,
    decoder: Option<H264Decoder>,
    opts: VideoIngestOptions,
    selected: Vec<usize>,
    cursor: usize,
    sample_idx: usize,
}

impl FrameIter {
    fn new(samples: Vec<SampleInfo>, decoder: Option<H264Decoder>, opts: VideoIngestOptions) -> Self {
        let selected = select_sample_indices(samples.len(), &opts);
        Self { samples, decoder, opts, selected, cursor: 0, sample_idx: 0 }
    }
}

impl Iterator for FrameIter {
    type Item = Result<ExtractedFrame, VideoError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.selected.len() {
            return None;
        }
        let target = self.selected[self.cursor];
        match &mut self.decoder {
            None => {
                let image = match remodeling_image::decode_jpeg(&self.samples[target].data) {
                    Ok(i) => i,
                    Err(e) => return Some(Err(e.into())),
                };
                let timestamp_ms = self.samples[target].timestamp_ms;
                self.cursor += 1;
                Some(Ok(ExtractedFrame { index: target as u32, timestamp_ms, image: resize_to_max_long_edge(image, self.opts.max_long_edge_px) }))
            }
            Some(decoder) => {
                while self.sample_idx <= target {
                    let idx = self.sample_idx;
                    let decoded = match decoder.decode_sample(&self.samples[idx].data) {
                        Ok(v) => v,
                        Err(e) => return Some(Err(e.into())),
                    };
                    let Some(image) = decoded else {
                        self.sample_idx += 1;
                        continue;
                    };
                    if idx == target {
                        let timestamp_ms = self.samples[idx].timestamp_ms;
                        self.sample_idx += 1;
                        self.cursor += 1;
                        return Some(Ok(ExtractedFrame { index: idx as u32, timestamp_ms, image: resize_to_max_long_edge(image, self.opts.max_long_edge_px) }));
                    }
                    self.sample_idx += 1;
                }
                None
            }
        }
    }
}

/// 📥️ Extracts sampled frames using the artifact owner's linked capabilities.
pub fn extract_frames(bytes: &[u8], opts: &VideoIngestOptions) -> Result<FrameIter, VideoError> {
    extract_frames_with_providers_v1(&container_providers::inventory_v1(), bytes, opts)
}

/// 📥️ Decodes admitted provider samples with owned MJPEG and baseline AVC decoders.
pub fn extract_frames_with_providers_v1(providers: &[VideoContainerProviderV1<'_>], bytes: &[u8], opts: &VideoIngestOptions) -> Result<FrameIter, VideoError> {
    let info = probe_with_providers_v1(providers, bytes)?;
    match info.codec {
        VideoCodec::Mjpeg => Ok(FrameIter::new(info.samples, None, *opts)),
        VideoCodec::Avc => {
            let configuration = info.avc_config.ok_or(VideoError::UnsupportedCodec(FourCc(*b"avc1")))?;
            let decoder = H264Decoder::new(&flatten_sps_pps(&configuration.sps, &configuration.pps))?.with_nal_length_size(configuration.nal_length_size);
            Ok(FrameIter::new(info.samples, Some(decoder), *opts))
        }
        other => Err(VideoError::UnsupportedCodec(codec_fourcc_hint(other))),
    }
}

// #endregion 🔖️Extract

// #region 🔖️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

// #endregion 🔖️Tests
