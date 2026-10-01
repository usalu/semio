//! 📼️ AVI capability with owned video records and encoded media bytes.
use super::super::{codec_from_fourcc_str, remodeling_image, SampleInfo, VideoContainerProviderV1, VideoError, VideoProbe, VideoTimeBaseV1};
use semio_s_artifact_stdio_avi::{
    standards::v1_0::{
        subsets::any::io as avi_engine,
        subsets::any::schema::snapshot::{AviChunk, AviMainHeader, AviSnapshot, AviStream, AviStreamFormat, AviStreamHeader},
    },
    STDIO_AVI_DOCUMENT_SCHEMA,
};

/// 🔌️ Binds actual RIFF/AVI recognition and decoding to the neutral provider port.
pub fn provider_v1() -> VideoContainerProviderV1<'static> {
    VideoContainerProviderV1 { id: "avi", matches: |bytes| bytes.len() >= 12 && &bytes[..4] == b"RIFF" && &bytes[8..12] == b"AVI ", probe }
}

/// 🔍️ Projects the first decoded video stream into owned timing and sample records.
pub fn probe(bytes: &[u8]) -> Result<VideoProbe, VideoError> {
    let snapshot = avi_engine::decode_avi(bytes).map_err(VideoError::Container)?;
    let stream = snapshot.streams.iter().find(|s| s.strh.fcc_type == "vids").ok_or(VideoError::NoVideoTrack)?;
    let compression = match &stream.strf {
        AviStreamFormat::BitmapInfo { compression, .. } => compression.clone(),
        _ => String::new(),
    };
    let codec = codec_from_fourcc_str(&compression);
    let fps = if stream.strh.scale > 0 {
        f64::from(stream.strh.rate) / f64::from(stream.strh.scale)
    } else if snapshot.main_header.micro_sec_per_frame > 0 {
        1_000_000.0 / f64::from(snapshot.main_header.micro_sec_per_frame)
    } else {
        0.0
    };
    let samples: Vec<SampleInfo> = stream.chunks.iter().enumerate().map(|(i, chunk)| SampleInfo { data: chunk.data.clone(), timestamp_ms: if fps > 0.0 { i as f64 * 1000.0 / fps } else { 0.0 } }).collect();
    let time_base = if stream.strh.scale > 0 && stream.strh.rate > 0 {
        Some(VideoTimeBaseV1 { numerator: stream.strh.scale, denominator: stream.strh.rate })
    } else if snapshot.main_header.micro_sec_per_frame > 0 {
        Some(VideoTimeBaseV1 { numerator: snapshot.main_header.micro_sec_per_frame, denominator: 1_000_000 })
    } else { None };
    Ok(VideoProbe { container: "avi".into(), width: snapshot.main_header.width, height: snapshot.main_header.height, time_base, fps, duration_ms: if fps > 0.0 { samples.len() as f64 * 1000.0 / fps } else { 0.0 }, frame_count: samples.len() as u32, codec, samples, avc_config: None })
}

/// ✍️ Encodes JPEG frames into a real AVI video stream.
pub fn write_mjpeg(frames: &[Vec<u8>], fps: f64) -> Vec<u8> {
    let (width, height) = frames.first().and_then(|f| remodeling_image::decode_jpeg(f).ok()).map_or((0, 0), |img| (img.width, img.height));
    let micro_sec_per_frame = if fps > 0.0 { (1_000_000.0 / fps).round() as u32 } else { 1_000_000 };
    let rate = if fps > 0.0 { (fps * 1000.0).round() as u32 } else { 1000 };
    let chunks: Vec<AviChunk> = frames.iter().map(|data| AviChunk { fourcc: "00dc".into(), data: data.clone(), keyframe: true }).collect();
    let stream = AviStream {
        strh: AviStreamHeader {
            fcc_type: "vids".into(),
            fcc_handler: "MJPG".into(),
            flags: 0,
            priority: 0,
            language: 0,
            initial_frames: 0,
            scale: 1000,
            rate,
            start: 0,
            length: frames.len() as u32,
            suggested_buffer_size: 0,
            quality: 0,
            sample_size: 0,
            rc_frame_left: 0,
            rc_frame_top: 0,
            rc_frame_right: width as i32,
            rc_frame_bottom: height as i32,
            rc_frame_width: 16,
            strh_extra: Vec::new(),
        },
        strf: AviStreamFormat::BitmapInfo { size: 40, width: width as i32, height: height as i32, planes: 1, bit_count: 24, compression: "MJPG".into(), size_image: 0, x_pels_per_meter: 0, y_pels_per_meter: 0, colors_used: 0, colors_important: 0 },
        chunks,
        strl_extra: Vec::new(),
    };
    let snapshot = AviSnapshot {
        schema: STDIO_AVI_DOCUMENT_SCHEMA.into(),
        main_header: AviMainHeader {
            micro_sec_per_frame,
            max_bytes_per_sec: 0,
            padding_granularity: 0,
            flags: 0x10,
            total_frames: frames.len() as u32,
            initial_frames: 0,
            streams: 1,
            suggested_buffer_size: 0,
            width,
            height,
            reserved: vec![0, 0, 0, 0],
        },
        streams: vec![stream],
        idx1_present: true,
        unknown_chunks: Vec::new(),
        hdrl_extra: Vec::new(),
    };
    avi_engine::encode_avi(&snapshot)
}
