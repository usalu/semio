//! 🎥️ ISO-BMFF capability with owned video records and encoded media bytes.
use super::super::{codec_from_fourcc_str, remodeling_image, sps_nal_dimensions, AvcDecoderConfigurationV1, SampleInfo, VideoContainerProviderV1, VideoError, VideoProbe, VideoTimeBaseV1};
use semio_s_artifact_stdio_mp4::{
    standards::isobmff::{
        subsets::any::io as mp4_engine,
        subsets::any::schema::snapshot::{Mp4Codec, Mp4CodecFormat, Mp4Ftyp, Mp4Sample, Mp4Snapshot, Mp4Track},
    },
    STDIO_MP4_DOCUMENT_SCHEMA,
};

/// 🔌️ Binds actual ISO-BMFF recognition and decoding to the neutral provider port.
pub fn provider_v1() -> VideoContainerProviderV1<'static> {
    VideoContainerProviderV1 { id: "mp4", matches: mp4_engine::sniff_real_bytes, probe }
}

/// 🔍️ Projects the first decoded video track into owned timing and sample records.
pub fn probe(bytes: &[u8]) -> Result<VideoProbe, VideoError> {
    let snapshot = mp4_engine::decode_mp4(bytes).map_err(VideoError::Container)?;
    let track = snapshot.tracks.first().ok_or(VideoError::NoVideoTrack)?;
    let codec = codec_from_fourcc_str(track.codec.format.fourcc());
    let avc_config = track.codec.format.is_avc().then(|| AvcDecoderConfigurationV1 { sps: track.codec.sps.clone(), pps: track.codec.pps.clone(), nal_length_size: track.codec.nal_length_size });
    let timescale = track.timescale.max(1);
    let mut dts_accum: u64 = 0;
    let mut samples = Vec::with_capacity(track.samples.len());
    for sample in &track.samples {
        let pts_ticks = dts_accum as i64 + i64::from(sample.cts_offset);
        let timestamp_ms = pts_ticks as f64 * 1000.0 / f64::from(timescale);
        samples.push(SampleInfo { data: sample.data.clone(), timestamp_ms });
        dts_accum += u64::from(sample.duration);
    }
    let duration_ms = dts_accum as f64 * 1000.0 / f64::from(timescale);
    Ok(VideoProbe { container: "mp4".into(), width: track.width, height: track.height, time_base: Some(VideoTimeBaseV1 { numerator: 1, denominator: timescale }), fps: if duration_ms > 0.0 { samples.len() as f64 * 1000.0 / duration_ms } else { 0.0 }, duration_ms, frame_count: samples.len() as u32, codec, samples, avc_config })
}

/// ✍️ Encodes JPEG frames into a real ISO-BMFF video track.
pub fn write_mjpeg(frames: &[Vec<u8>], fps: f64) -> Vec<u8> {
    let (width, height) = frames.first().and_then(|f| remodeling_image::decode_jpeg(f).ok()).map_or((0, 0), |img| (img.width, img.height));
    let delta = if fps > 0.0 { (1000.0 / fps).round() as u32 } else { 1000 }.max(1);
    let samples: Vec<_> = frames.iter().map(|data| Mp4Sample { data: data.clone(), duration: delta, cts_offset: 0, sync: true }).collect();
    let track = Mp4Track { track_id: 1, timescale: 1000, codec: Mp4Codec::jpeg(Mp4CodecFormat::Jpeg), width, height, metadata: Default::default(), chunk_sample_counts: vec![samples.len() as u32], samples };
    let snapshot = Mp4Snapshot { schema: STDIO_MP4_DOCUMENT_SCHEMA.into(), ftyp: Mp4Ftyp { major_brand: "isom".into(), minor_version: 512, compatible_brands: vec!["isom".into(), "mp41".into()] }, movie: Default::default(), tracks: vec![track] };
    mp4_engine::encode_mp4(&snapshot)
}

/// ✍️ Encodes AVC access units with their actual parameter sets.
pub fn write_avc(nal_samples: &[Vec<u8>], sps_nal: &[u8], pps_nal: &[u8], fps: f64) -> Vec<u8> {
    let (width, height) = sps_nal_dimensions(sps_nal);
    let delta = if fps > 0.0 { (1000.0 / fps).round() as u32 } else { 1000 }.max(1);
    let samples: Vec<_> = nal_samples.iter().map(|data| Mp4Sample { data: data.clone(), duration: delta, cts_offset: 0, sync: true }).collect();
    let track = Mp4Track {
        track_id: 1,
        timescale: 1000,
        codec: Mp4Codec::avc(vec![sps_nal.to_vec()], vec![pps_nal.to_vec()], 4, None),
        width,
        height,
        metadata: Default::default(),
        chunk_sample_counts: vec![samples.len() as u32],
        samples,
    };
    let snapshot =
        Mp4Snapshot { schema: STDIO_MP4_DOCUMENT_SCHEMA.into(), ftyp: Mp4Ftyp { major_brand: "isom".into(), minor_version: 512, compatible_brands: vec!["isom".into(), "avc1".into(), "mp41".into()] }, movie: Default::default(), tracks: vec![track] };
    mp4_engine::encode_mp4(&snapshot)
}
