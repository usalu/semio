//! 📤️ Serialize `s.stdio.semio` (v1/video) into `s.stdio.avi` (1.0/✳️any) — one `AviStream` per
//! `SemioVideoStream`, `strh.scale = rate.den`/`strh.rate = rate.num` (AVI's own frames-per-second
//! fraction, matching this pair's deserializer exactly), `strh.fcc_type` from `kind`
//! (`Video`→`"vids"`, everything else→`"auds"` -- AVI's own two-stream-kind vocabulary is coarser
//! than `video`'s three-way `SemioVideoStreamKind`, so `Subtitle` streams honestly fold to `"auds"`
//! rather than being dropped, since AVI has no dedicated subtitle `fccType` and dropping would lose
//! the samples entirely; documented here rather than silently done). `AviMainHeader` is synthesized
//! from the FIRST stream's dimensions/frame count (a real, honest, documented simplification -- AVI
//! has exactly one global header per file, `video` has per-stream dimensions, a genuine cardinality
//! mismatch when there is more than one stream).

use crate::standards::v1::subsets::video::schema::snapshot::{SemioVideoSnapshot, SemioVideoStreamKind};
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};
use semio_s_artifact_stdio_avi::standards::v1_0::subsets::any::schema::snapshot::{AviChunk, AviMainHeader, AviStream, AviStreamFormat, AviStreamHeader};
use semio_s_artifact_stdio_avi::AviSnapshot;

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("video") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.avi", standard: StandardId("1.0"), subset: SubsetId("*") };

pub struct SemioVideoToAvi;

impl ArtifactSerializer for SemioVideoToAvi {
    type From = SemioVideoSnapshot;
    type Into = AviSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let streams: Vec<AviStream> = from
            .streams
            .iter()
            .map(|s| {
                let fcc_type = if s.kind == SemioVideoStreamKind::Video { "vids" } else { "auds" };
                let scale = s.rate.den.max(1) as u32;
                let rate = s.rate.num.max(1) as u32;
                let strh = AviStreamHeader {
                    fcc_type: fcc_type.into(),
                    fcc_handler: s.codec.clone(),
                    flags: 0,
                    priority: 0,
                    language: 0,
                    initial_frames: 0,
                    scale,
                    rate,
                    start: 0,
                    length: s.samples.len() as u32,
                    suggested_buffer_size: 0,
                    quality: -1,
                    sample_size: 0,
                    rc_frame_left: 0,
                    rc_frame_top: 0,
                    rc_frame_right: s.width as i32,
                    rc_frame_bottom: s.height as i32,
                    rc_frame_width: 16,
                    strh_extra: Vec::new(),
                };
                let strf = if fcc_type == "vids" {
                    AviStreamFormat::BitmapInfo {
                        size: 40,
                        width: s.width as i32,
                        height: s.height as i32,
                        planes: 1,
                        bit_count: 24,
                        compression: s.codec.clone(),
                        size_image: 0,
                        x_pels_per_meter: 0,
                        y_pels_per_meter: 0,
                        colors_used: 0,
                        colors_important: 0,
                    }
                } else {
                    AviStreamFormat::Raw { data: Vec::new() }
                };
                let chunks = s.samples.iter().map(|sample| AviChunk { fourcc: if fcc_type == "vids" { "00dc".into() } else { "01wb".into() }, data: sample.data.clone(), keyframe: sample.key }).collect();
                AviStream { strh, strf, chunks, strl_extra: Vec::new() }
            })
            .collect();
        let first = from.streams.first();
        let main_header = AviMainHeader {
            micro_sec_per_frame: first.map_or(0, |s| if s.rate.num > 0 { (1_000_000 * s.rate.den / s.rate.num).max(0) as u32 } else { 0 }),
            max_bytes_per_sec: 0,
            padding_granularity: 0,
            flags: 0x10,
            total_frames: first.map_or(0, |s| s.samples.len() as u32),
            initial_frames: 0,
            streams: streams.len() as u32,
            suggested_buffer_size: 0,
            width: first.map_or(0, |s| s.width),
            height: first.map_or(0, |s| s.height),
            reserved: vec![0, 0, 0, 0],
        };
        Ok(AviSnapshot { schema: "stdio.avi".into(), main_header, streams, idx1_present: true, unknown_chunks: Vec::new(), hdrl_extra: Vec::new() })
    }
}

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
