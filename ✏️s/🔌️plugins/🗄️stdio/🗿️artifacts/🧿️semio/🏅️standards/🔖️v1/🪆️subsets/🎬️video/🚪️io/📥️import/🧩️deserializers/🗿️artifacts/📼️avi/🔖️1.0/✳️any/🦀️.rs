//! 📥️ Deserialize `s.stdio.avi` (1.0/✳️any) into `s.stdio.semio` (v1/video) — direct reshape of
//! RIFF/AVI stream metadata. Only `vids` (`fccType == "vids"`) streams become `SemioVideoStream`s
//! (an honest boundary matching `video`'s own kind vocabulary of Video/Audio/Subtitle -- `auds`
//! streams map to `Audio`, everything else is dropped with a doc-noted reason below, never
//! fabricated). AVI has no per-sample timestamp -- `pts` is synthesized as the running index times
//! `strh.scale` (AVI's own per-frame duration unit, `rate`/`scale` gives frames/sec), matching the
//! spec's own constant-frame-duration assumption for `dwLength`.
//!
//! Honest, documented lossy fields:
//! - `AviMainHeader` (global flags/buffer sizes) has no video-subset counterpart; dropped.
//! - `AviStreamFormat::BitmapInfo`/`WaveFormat` structured fields collapse to a codec name string
//!   (`compression` for `vids`, `"pcm"`/`format_tag` for `auds`) -- the rest is dropped.
//! - `AviChunk.fourcc` (e.g. `"00dc"`) is dropped -- `SemioVideoSample` has no per-sample tag slot.

use crate::standards::v1::subsets::video::schema::snapshot::{SemioRational, SemioVideoSample, SemioVideoSnapshot, SemioVideoStream, SemioVideoStreamKind, STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};
use semio_s_artifact_stdio_avi::standards::v1_0::subsets::any::schema::snapshot::AviStreamFormat;
use semio_s_artifact_stdio_avi::AviSnapshot;

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.avi", standard: StandardId("1.0"), subset: SubsetId("*") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("video") };

pub struct SemioVideoFromAvi;

impl ArtifactDeserializer for SemioVideoFromAvi {
    type From = AviSnapshot;
    type Into = SemioVideoSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let streams = from
            .streams
            .iter()
            .filter_map(|s| {
                let kind = match s.strh.fcc_type.as_str() {
                    "vids" => SemioVideoStreamKind::Video,
                    "auds" => SemioVideoStreamKind::Audio,
                    _ => return None, // 📦️ AVI also permits "txts"/"mids" streams -- out of `video`'s own Video/Audio/Subtitle+opaque-sample vocabulary honestly dropped, never fabricated as one of the three.
                };
                let (codec, width, height) = match &s.strf {
                    AviStreamFormat::BitmapInfo { compression, width, height, .. } => (compression.clone(), (*width).max(0) as u32, (*height).unsigned_abs()),
                    AviStreamFormat::WaveFormat { format_tag, .. } => (format!("wav-tag-{format_tag}"), 0, 0),
                    AviStreamFormat::Raw { .. } => (s.strh.fcc_handler.clone(), 0, 0),
                };
                let scale = s.strh.scale.max(1) as i64;
                let rate_num = s.strh.rate.max(1) as i64;
                let samples = s.chunks.iter().enumerate().map(|(i, chunk)| SemioVideoSample { pts: (i as i64 * scale).max(0) as u64, key: chunk.keyframe, data: chunk.data.clone() }).collect();
                Some(SemioVideoStream { kind, codec, width, height, rate: SemioRational { num: rate_num, den: scale }, samples })
            })
            .collect();
        Ok(SemioVideoSnapshot { schema: STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA.into(), streams })
    }
}

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
