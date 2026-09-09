//! 📥️ Deserialize `s.stdio.mp4` (isobmff/✳️any) into `s.stdio.semio` (v1/video) — a direct
//! reshape of ISO-BMFF track/sample metadata onto video's container-typed, payload-opaque stream
//! model (master plan: "close to a direct reshape of typed metadata, not a pixel/codec-level
//! operation"). Every `Mp4Track` decodes to one `SemioVideoStream` of `kind: Video` (this codec
//! only ever types video-handler tracks — see `Mp4Track`'s own doc comment — audio/other tracks
//! are rejected by the native MP4 deserializer before this reshape).
//!
//! Honest, documented lossy fields (real, unavoidable — never fabricated):
//! - `Mp4Codec{sps,pps,nal_length_size}` collapses to the plain codec name string `"avc1"` —
//!   `SemioVideoStream.codec` has no slot for structured codec-config bytes.
//! - `Mp4Sample.cts_offset` (composition-time offset) is folded into the derived `pts` (`pts = dts
//!   + cts_offset`, `dts` = running sum of prior `duration`s) rather than kept as its own field —
//!   `SemioVideoSample` has no separate decode/presentation timestamp pair.
//! - `Mp4Snapshot.ftyp` has no video-subset counterpart and is dropped.

use crate::standards::v1::subsets::video::schema::snapshot::{SemioRational, SemioVideoSample, SemioVideoSnapshot, SemioVideoStream, SemioVideoStreamKind, STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};
use semio_s_artifact_stdio_mp4::Mp4Snapshot;

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.mp4", standard: StandardId("isobmff"), subset: SubsetId("*") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("video") };

pub struct SemioVideoFromMp4;

impl ArtifactDeserializer for SemioVideoFromMp4 {
    type From = Mp4Snapshot;
    type Into = SemioVideoSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let streams = from
            .tracks
            .iter()
            .map(|track| {
                let codec = "avc1".to_string();
                let mut dts: i64 = 0;
                let samples = track
                    .samples
                    .iter()
                    .map(|sample| {
                        let pts = (dts + sample.cts_offset as i64).max(0) as u64;
                        dts += sample.duration as i64;
                        SemioVideoSample { pts, key: sample.sync, data: sample.data.clone() }
                    })
                    .collect();
                let den = track.samples.first().map_or(1, |s| s.duration as i64).max(1);
                SemioVideoStream { kind: SemioVideoStreamKind::Video, codec, width: track.width, height: track.height, rate: SemioRational { num: track.timescale as i64, den }, samples }
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
