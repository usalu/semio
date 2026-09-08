
use super::*;

fn lcg(state: &mut u64) -> u8 {
    *state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    (*state >> 56) as u8
}

fn fill_deterministic(state: &mut u64, len: usize) -> Vec<u8> {
    (0..len).map(|_| lcg(state)).collect()
}

fn synth_rgba(width: u32, height: u32, seed: u64) -> remodeling_image::ImageRgba8 {
    let mut state = seed;
    let mut img = remodeling_image::ImageRgba8::new(width, height);
    for px in img.data.chunks_mut(4) {
        px[0] = lcg(&mut state);
        px[1] = lcg(&mut state);
        px[2] = lcg(&mut state);
        px[3] = 255;
    }
    img
}

// #region 🔖️CoreTests
#[test]
fn fourcc_debug_and_display_render_printable_ascii() {
    let fcc = FourCc::new(b"avc1");
    assert_eq!(format!("{fcc:?}"), "FourCc(\"avc1\")");
    assert_eq!(format!("{fcc}"), "avc1");
}

#[test]
fn fourcc_debug_and_display_render_non_ascii_as_hex() {
    let fcc = FourCc([0x00, 0x01, 0xFF, 0x80]);
    assert_eq!(format!("{fcc:?}"), "FourCc(0001ff80)");
    assert_eq!(format!("{fcc}"), "0001ff80");
}

#[test]
fn video_error_display_messages() {
    assert_eq!(VideoError::Truncated.to_string(), "video container truncated");
    assert_eq!(VideoError::Container("x".into()).to_string(), "video container error: x");
    assert_eq!(VideoError::NoVideoTrack.to_string(), "container has no video track");
    assert_eq!(VideoError::UnsupportedCodec(FourCc(*b"xvid")).to_string(), "unsupported video codec: xvid");
    assert_eq!(VideoError::H264(H264Error::NoSps).to_string(), "h264 error: h264 slice references an unparsed sps");
}

#[test]
fn h264_error_display_messages() {
    assert_eq!(H264Error::Truncated.to_string(), "h264 bitstream truncated");
    assert_eq!(H264Error::Malformed("y").to_string(), "malformed h264 bitstream: y");
    assert_eq!(H264Error::Unsupported("z").to_string(), "unsupported h264 feature: z");
    assert_eq!(H264Error::NoSps.to_string(), "h264 slice references an unparsed sps");
    assert_eq!(H264Error::NoPps.to_string(), "h264 slice references an unparsed pps");
}
// #endregion 🔖️CoreTests

// #region 🔖️ContainerTests
/// 🔬 The real integration point with stdio: mux via `write_mp4_mjpeg` (→ stdio's real
/// `encode_mp4`), probe via `probe_mp4` (→ stdio's real `decode_mp4`), and check every sample's
/// recovered timestamp — proves the adapter's DTS-accumulation formula matches stdio's own
/// `duration`/`cts_offset` semantics, not just that types line up.
#[test]
fn write_mp4_mjpeg_probe_round_trip_reports_exact_frames() {
    let frames: Vec<Vec<u8>> = (0..5).map(|i| remodeling_image::encode_jpeg(&synth_rgba(16, 16, 100 + i), 90)).collect();
    let mp4 = write_mp4_mjpeg(&frames, 10.0);
    let info = probe_mp4(&mp4).expect("probes");
    assert_eq!(info.frame_count, 5);
    assert_eq!(info.codec, VideoCodec::Mjpeg);
    assert_eq!(info.width, 16);
    assert_eq!(info.height, 16);
    for (i, s) in info.samples.iter().enumerate() {
        assert!((s.timestamp_ms - i as f64 * 100.0).abs() < 1.0, "sample {i} timestamp {}", s.timestamp_ms);
    }
}

#[test]
fn mp4_probe_detects_avc1_codec_from_avcc_sample_entry() {
    let (sps_nal, pps_nal) = h264_enc_sps_pps_nals(1, 1);
    let mp4 = write_mp4_avc(&[h264_enc_i_pcm_sample(1, 1, 0, &[0; 256], &[0; 64], &[0; 64])], &sps_nal, &pps_nal, 5.0);
    let info = probe_mp4(&mp4).expect("probes");
    assert_eq!(info.codec, VideoCodec::Avc);
    assert_eq!(info.width, 16);
    assert_eq!(info.height, 16);
    assert!(info.avc_config.is_some());
}

/// 🔬 `probe_mp4` surfaces stdio's own decode error verbatim through `VideoError::Container`,
/// rather than swallowing or misclassifying it.
#[test]
fn mp4_probe_wraps_stdio_decode_errors_as_container() {
    assert!(matches!(probe_mp4(&[]), Err(VideoError::Container(_))));
    assert!(matches!(probe_mp4(b"not an mp4 at all"), Err(VideoError::Container(_))));
}

#[test]
fn mp4_probe_reports_no_video_track_when_none_present() {
    let snapshot = Mp4Snapshot { schema: STDIO_MP4_DOCUMENT_SCHEMA.into(), ftyp: Mp4Ftyp { major_brand: "isom".into(), minor_version: 0, compatible_brands: vec!["isom".into()] }, movie: Default::default(), tracks: vec![] };
    let bytes = mp4_engine::encode_mp4(&snapshot);
    assert!(matches!(probe_mp4(&bytes), Err(VideoError::NoVideoTrack)));
}

#[test]
fn write_avi_mjpg_probe_round_trip_reports_exact_frames() {
    let frames: Vec<Vec<u8>> = (0..4).map(|i| remodeling_image::encode_jpeg(&synth_rgba(8, 8, 300 + i), 85)).collect();
    let avi = write_avi_mjpg(&frames, 8.0);
    let info = probe_avi(&avi).expect("probes");
    assert_eq!(info.frame_count, 4);
    assert_eq!(info.codec, VideoCodec::Mjpeg);
    assert_eq!(info.width, 8);
    assert_eq!(info.height, 8);
    assert!((info.fps - 8.0).abs() < 1e-6);
    for (i, s) in info.samples.iter().enumerate() {
        assert!((s.timestamp_ms - i as f64 * 125.0).abs() < 1.0);
    }
}

#[test]
fn avi_probe_rejects_non_riff_bytes() {
    assert!(matches!(probe_avi(b"not an avi at all!!"), Err(VideoError::Container(_))));
}

#[test]
fn avi_probe_reports_no_video_track_when_only_audio_present() {
    let snapshot = AviSnapshot {
        schema: STDIO_AVI_DOCUMENT_SCHEMA.into(),
        main_header: AviMainHeader { micro_sec_per_frame: 0, max_bytes_per_sec: 0, padding_granularity: 0, flags: 0, total_frames: 0, initial_frames: 0, streams: 1, suggested_buffer_size: 0, width: 0, height: 0, reserved: vec![0, 0, 0, 0] },
        streams: vec![AviStream {
            strh: AviStreamHeader {
                fcc_type: "auds".into(),
                fcc_handler: "NONE".into(),
                flags: 0,
                priority: 0,
                language: 0,
                initial_frames: 0,
                scale: 1,
                rate: 44100,
                start: 0,
                length: 0,
                suggested_buffer_size: 0,
                quality: 0,
                sample_size: 2,
                rc_frame_left: 0,
                rc_frame_top: 0,
                rc_frame_right: 0,
                rc_frame_bottom: 0,
                rc_frame_width: 16,
                strh_extra: Vec::new(),
            },
            strf: AviStreamFormat::WaveFormat { format_tag: 1, channels: 1, samples_per_sec: 44100, avg_bytes_per_sec: 88200, block_align: 2, bits_per_sample: 16, extra: vec![] },
            chunks: vec![],
            strl_extra: Vec::new(),
        }],
        idx1_present: false,
        unknown_chunks: vec![],
        hdrl_extra: Vec::new(),
    };
    let bytes = avi_engine::encode_avi(&snapshot);
    assert!(matches!(probe_avi(&bytes), Err(VideoError::NoVideoTrack)));
}

#[test]
fn probe_dispatches_by_riff_magic() {
    let frames: Vec<Vec<u8>> = (0..2).map(|i| remodeling_image::encode_jpeg(&synth_rgba(4, 4, 900 + i), 80)).collect();
    let avi = write_avi_mjpg(&frames, 5.0);
    assert!(matches!(probe(&avi), Ok(VideoProbe::Avi(_))));
    let mp4 = write_mp4_mjpeg(&frames, 5.0);
    assert!(matches!(probe(&mp4), Ok(VideoProbe::Mp4(_))));
}

#[test]
fn codec_fourcc_hint_maps_each_codec_variant() {
    assert_eq!(codec_fourcc_hint(VideoCodec::Avc), FourCc(*b"avc1"));
    assert_eq!(codec_fourcc_hint(VideoCodec::Hevc), FourCc(*b"hvc1"));
    assert_eq!(codec_fourcc_hint(VideoCodec::Vp9), FourCc(*b"vp09"));
    assert_eq!(codec_fourcc_hint(VideoCodec::Av1), FourCc(*b"av01"));
    assert_eq!(codec_fourcc_hint(VideoCodec::Mjpeg), FourCc(*b"mjpg"));
    assert_eq!(codec_fourcc_hint(VideoCodec::Unknown(FourCc(*b"zzzz"))), FourCc(*b"zzzz"));
}
// #endregion 🔖️ContainerTests

// #region 🔖️ExtractTests
#[test]
fn extract_frames_mjpeg_applies_stride_and_max_frames_exactly() {
    let frames: Vec<Vec<u8>> = (0..10).map(|i| remodeling_image::encode_jpeg(&synth_rgba(8, 8, 500 + i), 85)).collect();
    let mp4 = write_mp4_mjpeg(&frames, 10.0);
    let opts = VideoIngestOptions { stride: 3, max_frames: 2, max_long_edge_px: 0 };
    let extracted: Vec<ExtractedFrame> = extract_frames(&mp4, &opts).expect("extracts").map(|f| f.expect("frame decodes")).collect();
    assert_eq!(extracted.len(), 2);
    assert_eq!(extracted[0].index, 0);
    assert_eq!(extracted[1].index, 3);
}

#[test]
fn extract_frames_mjpeg_lazily_skips_undecoded_frames() {
    let mut frames: Vec<Vec<u8>> = (0..6).map(|i| remodeling_image::encode_jpeg(&synth_rgba(8, 8, 600 + i), 85)).collect();
    for i in [1usize, 2, 4, 5] {
        frames[i] = vec![0xDE, 0xAD, 0xBE, 0xEF];
    }
    let mp4 = write_mp4_mjpeg(&frames, 10.0);
    let opts = VideoIngestOptions { stride: 3, max_frames: 0, max_long_edge_px: 0 };
    let extracted: Result<Vec<ExtractedFrame>, VideoError> = extract_frames(&mp4, &opts).expect("extracts").collect();
    let extracted = extracted.expect("only sync-selected frames (0, 3) are ever decoded, both real jpegs");
    assert_eq!(extracted.len(), 2);
    assert_eq!(extracted[0].index, 0);
    assert_eq!(extracted[1].index, 3);
}

#[test]
fn extract_frames_applies_max_long_edge_downscale() {
    let frames = vec![remodeling_image::encode_jpeg(&synth_rgba(32, 16, 700), 90)];
    let mp4 = write_mp4_mjpeg(&frames, 5.0);
    let opts = VideoIngestOptions { stride: 1, max_frames: 0, max_long_edge_px: 16 };
    let extracted: Vec<ExtractedFrame> = extract_frames(&mp4, &opts).expect("extracts").map(|f| f.expect("decodes")).collect();
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].image.width, 16);
    assert_eq!(extracted[0].image.height, 8);
}

#[test]
fn extract_frames_rejects_unsupported_codec_with_provenance() {
    let track =
        Mp4Track { track_id: 1, timescale: 1000, codec: Mp4Codec::default(), width: 4, height: 4, metadata: Default::default(), chunk_sample_counts: vec![1], samples: vec![Mp4Sample { data: vec![0; 10], duration: 100, cts_offset: 0, sync: true }] };
    let snapshot = Mp4Snapshot { schema: STDIO_MP4_DOCUMENT_SCHEMA.into(), ftyp: Mp4Ftyp { major_brand: "isom".into(), minor_version: 0, compatible_brands: vec!["isom".into()] }, movie: Default::default(), tracks: vec![track] };
    let bytes = mp4_engine::encode_mp4(&snapshot);
    let info = probe_mp4(&bytes).expect("hvc1 still probes for provenance");
    assert_eq!(info.codec, VideoCodec::Hevc);
    let opts = VideoIngestOptions { stride: 1, max_frames: 0, max_long_edge_px: 0 };
    assert!(matches!(extract_frames(&bytes, &opts), Err(VideoError::UnsupportedCodec(_))));
}

#[test]
fn extract_frames_mjpeg_propagates_jpeg_decode_error() {
    let frames = vec![vec![0xDE, 0xAD, 0xBE, 0xEF]];
    let mp4 = write_mp4_mjpeg(&frames, 5.0);
    let opts = VideoIngestOptions { stride: 1, max_frames: 0, max_long_edge_px: 0 };
    let mut iter = extract_frames(&mp4, &opts).expect("extracts");
    assert!(matches!(iter.next(), Some(Err(VideoError::Jpeg(_)))));
}

#[test]
fn extract_frames_avi_rejects_unsupported_codec_with_provenance() {
    let snapshot = AviSnapshot {
        schema: STDIO_AVI_DOCUMENT_SCHEMA.into(),
        main_header: AviMainHeader { micro_sec_per_frame: 166_667, max_bytes_per_sec: 0, padding_granularity: 0, flags: 0x10, total_frames: 0, initial_frames: 0, streams: 1, suggested_buffer_size: 0, width: 8, height: 8, reserved: vec![0, 0, 0, 0] },
        streams: vec![AviStream {
            strh: AviStreamHeader {
                fcc_type: "vids".into(),
                fcc_handler: "XVID".into(),
                flags: 0,
                priority: 0,
                language: 0,
                initial_frames: 0,
                scale: 1000,
                rate: 6000,
                start: 0,
                length: 0,
                suggested_buffer_size: 0,
                quality: 0,
                sample_size: 0,
                rc_frame_left: 0,
                rc_frame_top: 0,
                rc_frame_right: 8,
                rc_frame_bottom: 8,
                rc_frame_width: 16,
                strh_extra: Vec::new(),
            },
            strf: AviStreamFormat::BitmapInfo { size: 40, width: 8, height: 8, planes: 1, bit_count: 24, compression: "XVID".into(), size_image: 0, x_pels_per_meter: 0, y_pels_per_meter: 0, colors_used: 0, colors_important: 0 },
            chunks: vec![],
            strl_extra: Vec::new(),
        }],
        idx1_present: false,
        unknown_chunks: vec![],
        hdrl_extra: Vec::new(),
    };
    let bytes = avi_engine::encode_avi(&snapshot);
    let info = probe_avi(&bytes).expect("XVID still probes for provenance");
    assert_eq!(info.codec, VideoCodec::Unknown(FourCc(*b"XVID")));
    let opts = VideoIngestOptions { stride: 1, max_frames: 0, max_long_edge_px: 0 };
    assert!(matches!(extract_frames(&bytes, &opts), Err(VideoError::UnsupportedCodec(_))));
}

#[test]
fn resize_to_max_long_edge_noop_when_budget_zero_or_already_small() {
    let same = resize_to_max_long_edge(synth_rgba(10, 5, 1), 0);
    assert_eq!((same.width, same.height), (10, 5));
    let same2 = resize_to_max_long_edge(synth_rgba(10, 5, 2), 20);
    assert_eq!((same2.width, same2.height), (10, 5));
}

#[test]
fn resize_to_max_long_edge_downscales_preserving_aspect_ratio() {
    let out = resize_to_max_long_edge(synth_rgba(40, 20, 3), 20);
    assert_eq!((out.width, out.height), (20, 10));
}

#[test]
fn select_sample_indices_treats_stride_zero_as_one_and_respects_max_frames() {
    let opts = VideoIngestOptions { stride: 0, max_frames: 3, max_long_edge_px: 0 };
    assert_eq!(select_sample_indices(10, &opts), vec![0, 1, 2]);
}

#[test]
fn select_sample_indices_max_frames_zero_is_unbounded() {
    let opts = VideoIngestOptions { stride: 4, max_frames: 0, max_long_edge_px: 0 };
    assert_eq!(select_sample_indices(10, &opts), vec![0, 4, 8]);
}
// #endregion 🔖️ExtractTests

// #region 🔖️H264Tests
fn pcm_frame(mb_w: u32, mb_h: u32, seed: u64) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let mut state = seed;
    let luma = fill_deterministic(&mut state, (mb_w * 16 * mb_h * 16) as usize);
    let cb = fill_deterministic(&mut state, (mb_w * 8 * mb_h * 8) as usize);
    let cr = fill_deterministic(&mut state, (mb_w * 8 * mb_h * 8) as usize);
    (luma, cb, cr)
}

#[test]
fn h264_i_pcm_single_frame_decodes_bit_exactly() {
    let (mb_w, mb_h) = (2, 2);
    let (luma, cb, cr) = pcm_frame(mb_w, mb_h, 42);
    let sps_pps = h264_enc_sps_pps(mb_w, mb_h);
    let mut dec = H264Decoder::new(&sps_pps).expect("sps/pps parse");
    let nal = h264_enc_i_pcm_sample(mb_w, mb_h, 0, &luma, &cb, &cr);
    let image = dec.decode_sample(&nal).expect("decodes").expect("immediate output");
    let expected = ycbcr420_to_rgba(&luma, (mb_w * 16) as usize, &cb, &cr, (mb_w * 8) as usize, mb_w * 16, mb_h * 16);
    assert_eq!(image, expected);
}

#[test]
fn h264_p_skip_chain_propagates_the_i_pcm_frame_unchanged() {
    let (mb_w, mb_h) = (2, 2);
    let (luma, cb, cr) = pcm_frame(mb_w, mb_h, 7);
    let sps_pps = h264_enc_sps_pps(mb_w, mb_h);
    let mut dec = H264Decoder::new(&sps_pps).expect("sps/pps parse");
    let idr = dec.decode_sample(&h264_enc_i_pcm_sample(mb_w, mb_h, 0, &luma, &cb, &cr)).expect("decodes").expect("output");
    let mut prev = idr;
    for frame_num in 1..6u32 {
        let sample = h264_enc_p_skip_sample(mb_w, mb_h, frame_num);
        let image = dec.decode_sample(&sample).expect("p_skip decodes").expect("output");
        assert_eq!(image, prev, "frame {frame_num} should exactly equal the previous decoded frame");
        prev = image;
    }
}

#[test]
fn h264_truncated_nal_errors_not_panics() {
    let (mb_w, mb_h) = (1, 1);
    let (luma, cb, cr) = pcm_frame(mb_w, mb_h, 99);
    let sps_pps = h264_enc_sps_pps(mb_w, mb_h);
    let full = h264_enc_i_pcm_sample(mb_w, mb_h, 0, &luma, &cb, &cr);
    for len in 0..full.len() {
        let mut dec = H264Decoder::new(&sps_pps).expect("sps/pps parse");
        let _ = dec.decode_sample(&full[..len]);
    }
    let mut dec = H264Decoder::new(&sps_pps).expect("sps/pps parse");
    assert!(dec.decode_sample(&full).is_ok());
}

#[test]
fn h264_garbage_bytes_error_not_panic() {
    let (mb_w, mb_h) = (1, 1);
    let sps_pps = h264_enc_sps_pps(mb_w, mb_h);
    let mut state = 12345u64;
    for _ in 0..40 {
        let garbage = fill_deterministic(&mut state, 40);
        let mut dec = H264Decoder::new(&sps_pps).expect("sps/pps parse");
        let _ = dec.decode_sample(&garbage);
    }
}

#[test]
fn h264_new_rejects_truncated_or_missing_sps_pps() {
    assert!(matches!(H264Decoder::new(&[]), Err(H264Error::NoSps)));
    assert!(matches!(H264Decoder::new(&[0, 100]), Err(H264Error::Truncated)));
    assert!(matches!(H264Decoder::new(&[0, 1, 2]), Err(H264Error::NoSps)));
}

#[test]
fn h264_cabac_pps_is_unsupported() {
    let (sps, _) = h264_enc_sps_pps_nals(1, 1);
    let mut pps_bits = BitWriter::default();
    pps_bits.put_ue(0);
    pps_bits.put_ue(0);
    pps_bits.put_u(1, 1);
    pps_bits.put_u(0, 1);
    pps_bits.put_ue(0);
    pps_bits.put_ue(0);
    pps_bits.put_ue(0);
    pps_bits.put_u(0, 1);
    pps_bits.put_u(0, 2);
    pps_bits.put_se(0);
    pps_bits.put_se(0);
    pps_bits.put_se(0);
    pps_bits.put_u(1, 1);
    pps_bits.put_u(0, 1);
    pps_bits.put_u(0, 1);
    pps_bits.rbsp_trailing();
    let pps_nal = write_nal(3, 8, &pps_bits.bytes);
    let mut nals = Vec::new();
    nals.extend_from_slice(&(sps.len() as u16).to_be_bytes());
    nals.extend_from_slice(&sps);
    nals.extend_from_slice(&(pps_nal.len() as u16).to_be_bytes());
    nals.extend_from_slice(&pps_nal);
    assert!(matches!(H264Decoder::new(&nals), Err(H264Error::Unsupported(_))));
}

#[test]
fn h264_b_slice_is_unsupported() {
    let (mb_w, mb_h) = (1, 1);
    let sps_pps = h264_enc_sps_pps(mb_w, mb_h);
    let mut dec = H264Decoder::new(&sps_pps).expect("sps/pps parse");
    let mut s = BitWriter::default();
    s.put_ue(0);
    s.put_ue(1);
    s.put_ue(0);
    s.put_u(1, 8);
    s.put_u(0, 1);
    s.put_u(0, 1);
    s.put_u(0, 1);
    s.put_u(0, 1);
    s.put_se(0);
    s.put_ue(1);
    s.rbsp_trailing();
    let sample = avcc_frame(&write_nal(2, 1, &s.bytes));
    assert!(matches!(dec.decode_sample(&sample), Err(H264Error::Unsupported(_))));
}

#[test]
fn h264_decode_sample_rejects_nonzero_first_mb_in_slice() {
    let (mb_w, mb_h) = (2, 1);
    let sps_pps = h264_enc_sps_pps(mb_w, mb_h);
    let mut dec = H264Decoder::new(&sps_pps).expect("sps/pps parse");
    let mut s = BitWriter::default();
    s.put_ue(1);
    s.put_ue(5);
    s.put_ue(0);
    s.put_u(0, 8);
    s.put_u(0, 1);
    s.put_u(0, 1);
    s.put_u(0, 1);
    s.put_se(0);
    s.put_ue(1);
    s.rbsp_trailing();
    let sample = avcc_frame(&write_nal(2, 1, &s.bytes));
    assert!(matches!(dec.decode_sample(&sample), Err(H264Error::Unsupported(_))));
}

#[test]
fn h264_decode_sample_rejects_multiple_slice_nals_in_one_access_unit() {
    let (mb_w, mb_h) = (1, 1);
    let sps_pps = h264_enc_sps_pps(mb_w, mb_h);
    let mut dec = H264Decoder::new(&sps_pps).expect("sps/pps parse");
    let (luma, cb, cr) = pcm_frame(mb_w, mb_h, 1);
    let idr = h264_enc_i_pcm_sample(mb_w, mb_h, 0, &luma, &cb, &cr);
    let mut doubled = idr.clone();
    doubled.extend_from_slice(&idr);
    assert!(matches!(dec.decode_sample(&doubled), Err(H264Error::Unsupported(_))));
}

#[test]
fn h264_new_rejects_non_baseline_profile() {
    let mut sps = BitWriter::default();
    sps.put_u(77, 8);
    sps.rbsp_trailing();
    let nal = write_nal(3, 7, &sps.bytes);
    let blob = [(nal.len() as u16).to_be_bytes().to_vec(), nal].concat();
    assert!(matches!(H264Decoder::new(&blob), Err(H264Error::Unsupported(_))));
}

#[test]
fn h264_new_rejects_pic_order_cnt_type_one() {
    let mut sps = BitWriter::default();
    sps.put_u(66, 8);
    sps.put_u(0, 8);
    sps.put_u(30, 8);
    sps.put_ue(0);
    sps.put_ue(4);
    sps.put_ue(1);
    sps.rbsp_trailing();
    let nal = write_nal(3, 7, &sps.bytes);
    let blob = [(nal.len() as u16).to_be_bytes().to_vec(), nal].concat();
    assert!(matches!(H264Decoder::new(&blob), Err(H264Error::Unsupported(_))));
}

#[test]
fn h264_new_rejects_pic_order_cnt_type_out_of_range() {
    let mut sps = BitWriter::default();
    sps.put_u(66, 8);
    sps.put_u(0, 8);
    sps.put_u(30, 8);
    sps.put_ue(0);
    sps.put_ue(4);
    sps.put_ue(3);
    sps.rbsp_trailing();
    let nal = write_nal(3, 7, &sps.bytes);
    let blob = [(nal.len() as u16).to_be_bytes().to_vec(), nal].concat();
    assert!(matches!(H264Decoder::new(&blob), Err(H264Error::Malformed(_))));
}

#[test]
fn h264_new_rejects_interlaced_sps() {
    let mut sps = BitWriter::default();
    sps.put_u(66, 8);
    sps.put_u(0, 8);
    sps.put_u(30, 8);
    sps.put_ue(0);
    sps.put_ue(4);
    sps.put_ue(2);
    sps.put_ue(0);
    sps.put_u(0, 1);
    sps.put_ue(1);
    sps.put_ue(1);
    sps.put_u(0, 1);
    sps.rbsp_trailing();
    let nal = write_nal(3, 7, &sps.bytes);
    let blob = [(nal.len() as u16).to_be_bytes().to_vec(), nal].concat();
    assert!(matches!(H264Decoder::new(&blob), Err(H264Error::Unsupported(_))));
}

#[test]
fn h264_new_rejects_multiple_slice_groups_pps() {
    let (sps, _) = h264_enc_sps_pps_nals(1, 1);
    let mut pps_bits = BitWriter::default();
    pps_bits.put_ue(0);
    pps_bits.put_ue(0);
    pps_bits.put_u(0, 1);
    pps_bits.put_u(0, 1);
    pps_bits.put_ue(1);
    pps_bits.rbsp_trailing();
    let pps_nal = write_nal(3, 8, &pps_bits.bytes);
    let mut nals = Vec::new();
    nals.extend_from_slice(&(sps.len() as u16).to_be_bytes());
    nals.extend_from_slice(&sps);
    nals.extend_from_slice(&(pps_nal.len() as u16).to_be_bytes());
    nals.extend_from_slice(&pps_nal);
    assert!(matches!(H264Decoder::new(&nals), Err(H264Error::Unsupported(_))));
}

#[test]
fn h264_new_rejects_transform_8x8_mode_pps() {
    let (sps, _) = h264_enc_sps_pps_nals(1, 1);
    let mut pps_bits = BitWriter::default();
    pps_bits.put_ue(0);
    pps_bits.put_ue(0);
    pps_bits.put_u(0, 1);
    pps_bits.put_u(0, 1);
    pps_bits.put_ue(0);
    pps_bits.put_ue(0);
    pps_bits.put_ue(0);
    pps_bits.put_u(0, 1);
    pps_bits.put_u(0, 2);
    pps_bits.put_se(0);
    pps_bits.put_se(0);
    pps_bits.put_se(0);
    pps_bits.put_u(0, 1);
    pps_bits.put_u(0, 1);
    pps_bits.put_u(0, 1);
    pps_bits.put_u(1, 1);
    pps_bits.rbsp_trailing();
    let pps_nal = write_nal(3, 8, &pps_bits.bytes);
    let mut nals = Vec::new();
    nals.extend_from_slice(&(sps.len() as u16).to_be_bytes());
    nals.extend_from_slice(&sps);
    nals.extend_from_slice(&(pps_nal.len() as u16).to_be_bytes());
    nals.extend_from_slice(&pps_nal);
    assert!(matches!(H264Decoder::new(&nals), Err(H264Error::Unsupported(_))));
}

/// 🏗️ Like [`h264_enc_i_pcm_sample`] but with a configurable `disable_deblocking_filter_idc`/offsets, to
/// exercise the in-loop deblocking filter path that this crate's own encoder never turns on.
#[allow(clippy::too_many_arguments)]
fn i_pcm_sample_with_deblocking(mb_w: u32, mb_h: u32, frame_num: u32, luma: &[u8], cb: &[u8], cr: &[u8], disable_idc: u32, alpha_off_div2: i32, beta_off_div2: i32) -> Vec<u8> {
    let mut s = BitWriter::default();
    s.put_ue(0);
    s.put_ue(7);
    s.put_ue(0);
    s.put_u(frame_num, 8);
    s.put_ue(0);
    s.put_u(0, 1);
    s.put_u(0, 1);
    s.put_se(0);
    s.put_ue(disable_idc);
    if disable_idc != 1 {
        s.put_se(alpha_off_div2);
        s.put_se(beta_off_div2);
    }
    for n in 0..(mb_w * mb_h) {
        s.put_ue(25);
        s.zero_align();
        let (mb_x, mb_y) = (n % mb_w, n / mb_w);
        let lw = (mb_w * 16) as usize;
        for r in 0..16usize {
            for c in 0..16usize {
                s.put_u(u32::from(luma[(mb_y as usize * 16 + r) * lw + mb_x as usize * 16 + c]), 8);
            }
        }
        let cw = (mb_w * 8) as usize;
        for r in 0..8usize {
            for c in 0..8usize {
                s.put_u(u32::from(cb[(mb_y as usize * 8 + r) * cw + mb_x as usize * 8 + c]), 8);
            }
        }
        for r in 0..8usize {
            for c in 0..8usize {
                s.put_u(u32::from(cr[(mb_y as usize * 8 + r) * cw + mb_x as usize * 8 + c]), 8);
            }
        }
    }
    s.rbsp_trailing();
    avcc_frame(&write_nal(3, 5, &s.bytes))
}

#[test]
fn h264_i_pcm_with_deblocking_enabled_flat_picture_stays_flat() {
    let (mb_w, mb_h) = (2, 2);
    let luma = vec![128u8; (mb_w * 16 * mb_h * 16) as usize];
    let cb = vec![128u8; (mb_w * 8 * mb_h * 8) as usize];
    let cr = cb.clone();
    let sps_pps = h264_enc_sps_pps(mb_w, mb_h);
    let mut dec = H264Decoder::new(&sps_pps).expect("sps/pps parse");
    let nal = i_pcm_sample_with_deblocking(mb_w, mb_h, 0, &luma, &cb, &cr, 0, 0, 0);
    let image = dec.decode_sample(&nal).expect("decodes").expect("output");
    let expected = ycbcr420_to_rgba(&luma, (mb_w * 16) as usize, &cb, &cr, (mb_w * 8) as usize, mb_w * 16, mb_h * 16);
    assert_eq!(image, expected, "deblocking a perfectly flat picture is a no-op by construction");
}

#[test]
fn split_annexb_nals_splits_multiple_start_coded_nals() {
    let mut stream = Vec::new();
    stream.extend_from_slice(&[0, 0, 0, 1]);
    stream.extend_from_slice(&[0x67, 0xAA, 0xBB]);
    stream.extend_from_slice(&[0, 0, 1]);
    stream.extend_from_slice(&[0x68, 0xCC]);
    let nals = split_annexb_nals(&stream);
    assert_eq!(nals.len(), 2);
    assert_eq!(nals[0], [0x67, 0xAA, 0xBB]);
    assert_eq!(nals[1], [0x68, 0xCC]);
}

#[test]
fn bitreader_ue_se_roundtrip_via_bitwriter() {
    for v in [0u32, 1, 2, 5, 100, 1000] {
        let mut w = BitWriter::default();
        w.put_ue(v);
        w.rbsp_trailing();
        let mut r = BitReader::new(&w.bytes);
        assert_eq!(r.ue().unwrap(), v);
    }
    for v in [-500i32, -1, 0, 1, 500] {
        let mut w = BitWriter::default();
        w.put_se(v);
        w.rbsp_trailing();
        let mut r = BitReader::new(&w.bytes);
        assert_eq!(r.se().unwrap(), v);
    }
}

#[test]
fn bitreader_ue_rejects_overlong_leading_zero_run() {
    let data = [0x00u8, 0x00, 0x00, 0x00, 0x80];
    let mut r = BitReader::new(&data);
    assert!(matches!(r.ue(), Err(H264Error::Malformed(_))));
}

#[test]
fn bitreader_more_rbsp_data_detects_remaining_bits() {
    let empty = BitReader::new(&[]);
    assert!(!empty.more_rbsp_data());
    let only_stop_bit = BitReader::new(&[0x80]);
    assert!(!only_stop_bit.more_rbsp_data());
    let mut with_more = BitReader::new(&[0xFF, 0x80]);
    assert!(with_more.more_rbsp_data());
    with_more.u(8).unwrap();
    assert!(!with_more.more_rbsp_data());
}

// #region 🔖️H264PixelMathTests
#[test]
fn predict_intra4x4_uniform_neighbors_yield_uniform_output_for_all_modes() {
    let n = Intra4Neighbors { top: Some([100; 4]), left: Some([100; 4]), top_right: [100; 4], corner: 100 };
    for mode in 0..=8u8 {
        let out = predict_intra4x4(mode, &n).unwrap();
        assert_eq!(out, [100; 16], "mode {mode} should reproduce a flat neighborhood exactly");
    }
}

#[test]
fn predict_intra4x4_vertical_and_horizontal_require_their_neighbor() {
    let no_top = Intra4Neighbors { top: None, left: Some([1; 4]), top_right: [1; 4], corner: 1 };
    assert!(matches!(predict_intra4x4(0, &no_top), Err(H264Error::Malformed(_))));
    let no_left = Intra4Neighbors { top: Some([1; 4]), left: None, top_right: [1; 4], corner: 1 };
    assert!(matches!(predict_intra4x4(1, &no_left), Err(H264Error::Malformed(_))));
}

#[test]
fn predict_intra4x4_rejects_out_of_range_mode() {
    let n = Intra4Neighbors { top: Some([0; 4]), left: Some([0; 4]), top_right: [0; 4], corner: 0 };
    assert!(matches!(predict_intra4x4(9, &n), Err(H264Error::Malformed(_))));
}

#[test]
fn predict_intra4x4_dc_mode_averages_available_neighbors() {
    let both = Intra4Neighbors { top: Some([4, 4, 4, 4]), left: Some([12, 12, 12, 12]), top_right: [0; 4], corner: 0 };
    assert_eq!(predict_intra4x4(2, &both).unwrap(), [8; 16]);
    let top_only = Intra4Neighbors { top: Some([4, 4, 4, 4]), left: None, top_right: [0; 4], corner: 0 };
    assert_eq!(predict_intra4x4(2, &top_only).unwrap(), [4; 16]);
    let left_only = Intra4Neighbors { top: None, left: Some([12, 12, 12, 12]), top_right: [0; 4], corner: 0 };
    assert_eq!(predict_intra4x4(2, &left_only).unwrap(), [12; 16]);
    let neither = Intra4Neighbors { top: None, left: None, top_right: [0; 4], corner: 0 };
    assert_eq!(predict_intra4x4(2, &neither).unwrap(), [128; 16]);
}

#[test]
fn dc_pred_all_neighbor_availability_branches() {
    let top = [10i32, 20, 30, 40];
    let left = [1i32, 2, 3, 4];
    assert_eq!(dc_pred(Some(&top), Some(&left), 4), 14);
    assert_eq!(dc_pred(Some(&top), None, 4), 25);
    assert_eq!(dc_pred(None, Some(&left), 4), 3);
    assert_eq!(dc_pred(None, None, 4), 128);
}

#[test]
fn plane_pred_flat_neighbors_are_a_noop() {
    let top16 = vec![100i32; 16];
    let left16 = vec![100i32; 16];
    assert_eq!(plane_pred(&top16, &left16, 100, 16, 5, 32, 6), vec![100i32; 256]);
    let top8 = vec![100i32; 8];
    let left8 = vec![100i32; 8];
    assert_eq!(plane_pred(&top8, &left8, 100, 8, 17, 16, 5), vec![100i32; 64]);
}

#[test]
fn clip_u8_clamps_to_byte_range() {
    assert_eq!(clip_u8(-10), 0);
    assert_eq!(clip_u8(300), 255);
    assert_eq!(clip_u8(128), 128);
}

#[test]
fn norm_adjust_selects_scale_by_position_parity() {
    assert_eq!(norm_adjust(0, 0, 0), 10);
    assert_eq!(norm_adjust(0, 1, 1), 13);
    assert_eq!(norm_adjust(0, 0, 1), 16);
}

#[test]
fn dequant4x4_applies_shift_and_rounding_branches() {
    let coeffs = [1i32; 16];
    let low_qp = dequant4x4(&coeffs, 0);
    assert_eq!((low_qp[0], low_qp[1], low_qp[5]), (1, 1, 1));
    let high_qp = dequant4x4(&coeffs, 24);
    assert_eq!((high_qp[0], high_qp[1], high_qp[5]), (10, 16, 13));
}

#[test]
fn idct4x4_zero_input_is_zero() {
    assert_eq!(idct4x4(&[0; 16]), [0; 16]);
}

#[test]
fn idct4x4_dc_only_produces_uniform_output() {
    let mut d = [0i32; 16];
    d[0] = 64;
    assert_eq!(idct4x4(&d), [1; 16]);
}

#[test]
fn hadamard4_1d_basic_butterfly() {
    assert_eq!(hadamard4_1d([1, 2, 3, 4]), [10, -4, 0, -2]);
}

#[test]
fn transform_luma16x16_dc_zero_input_is_zero() {
    assert_eq!(transform_luma16x16_dc(&[0; 16], 10), [0; 16]);
}

#[test]
fn transform_chroma_dc_computes_expected_values() {
    assert_eq!(transform_chroma_dc(&[4, 0, 0, 0], 0), [1, 1, 1, 1]);
}

#[test]
fn chroma_qp_maps_table_and_clamps_offset() {
    assert_eq!(chroma_qp(0, 0), 0);
    assert_eq!(chroma_qp(30, 0), 29);
    assert_eq!(chroma_qp(51, 0), 39);
    assert_eq!(chroma_qp(51, 20), 39);
}

#[test]
fn read_te_variants_by_max_val() {
    assert_eq!(read_te(&mut BitReader::new(&[]), 0).unwrap(), 0);
    let mut inverted_one = BitReader::new(&[0b1000_0000]);
    assert_eq!(read_te(&mut inverted_one, 1).unwrap(), 0);
    let mut inverted_zero = BitReader::new(&[0b0000_0000]);
    assert_eq!(read_te(&mut inverted_zero, 1).unwrap(), 1);
}

#[test]
fn block4x4_grid_pos_covers_full_4x4_grid_bijectively() {
    let mut seen = std::collections::HashSet::new();
    for n in 0..16 {
        let pos = block4x4_grid_pos(n);
        assert!(pos.0 < 4 && pos.1 < 4);
        assert!(seen.insert(pos));
    }
    assert_eq!(seen.len(), 16);
}

#[test]
fn chroma_quad_pos_maps_known_indices() {
    assert_eq!(chroma_quad_pos(0), (0, 0));
    assert_eq!(chroma_quad_pos(1), (1, 0));
    assert_eq!(chroma_quad_pos(2), (0, 1));
    assert_eq!(chroma_quad_pos(3), (1, 1));
}

#[test]
fn boundary_strength_covers_all_branches() {
    assert_eq!(boundary_strength(true, true, false, 0, 0, [0, 0], 0, [0, 0], 0), 4);
    assert_eq!(boundary_strength(false, true, true, 0, 0, [0, 0], 0, [0, 0], 0), 3);
    assert_eq!(boundary_strength(false, false, false, 1, 0, [0, 0], 0, [0, 0], 0), 2);
    assert_eq!(boundary_strength(false, false, false, 0, 0, [0, 0], 0, [0, 0], 1), 1);
    assert_eq!(boundary_strength(false, false, false, 0, 0, [0, 0], 0, [4, 0], 0), 1);
    assert_eq!(boundary_strength(false, false, false, 0, 0, [0, 0], 0, [3, 0], 0), 0);
}

#[test]
fn median_mv_predict_single_ref_match_shortcut() {
    let a = ([1, 1], 0i8);
    let b = ([2, 2], 0i8);
    let c = ([3, 3], 1i8);
    assert_eq!(median_mv_predict(a, b, c, 1), [3, 3]);
}

#[test]
fn median_mv_predict_falls_back_to_componentwise_median() {
    let a = ([1, 5], 0i8);
    let b = ([2, 6], 0i8);
    let c = ([9, 1], 0i8);
    assert_eq!(median_mv_predict(a, b, c, 2), [2, 5]);
}

#[test]
fn filter_luma_strong_computes_expected_edge_samples() {
    let (pf, qf) = filter_luma_strong([10, 20, 30, 40], [45, 50, 60, 70], 100, 50);
    assert_eq!(pf, [24, 34, 38]);
    assert_eq!(qf, [45, 49, 57]);
}

#[test]
fn filter_luma_strong_passes_through_unfiltered_above_threshold() {
    let (pf, qf) = filter_luma_strong([10, 20, 30, 40], [200, 190, 180, 170], 5, 5);
    assert_eq!(pf, [20, 30, 40]);
    assert_eq!(qf, [200, 190, 180]);
}

#[test]
fn filter_luma_normal_computes_expected_deltas_when_below_threshold() {
    assert_eq!(filter_luma_normal([50, 60, 70], [80, 90, 100], 100, 50, 3), Some((62, 71, 79, 87)));
}

#[test]
fn filter_luma_normal_returns_none_above_threshold() {
    assert_eq!(filter_luma_normal([50, 60, 70], [200, 90, 100], 30, 50, 3), None);
}

#[test]
fn filter_chroma_normal_computes_expected_values_when_below_threshold() {
    assert_eq!(filter_chroma_normal(60, 70, 80, 90, 100, 50, 5), Some((71, 79)));
}

#[test]
fn filter_chroma_normal_returns_none_above_threshold() {
    assert_eq!(filter_chroma_normal(60, 70, 250, 90, 30, 50, 5), None);
}

#[test]
fn filter_chroma_strong_averages_when_below_threshold() {
    assert_eq!(filter_chroma_strong(60, 70, 80, 90, 100, 50), Some((70, 80)));
}

#[test]
fn filter_chroma_strong_returns_none_above_threshold() {
    assert_eq!(filter_chroma_strong(60, 70, 250, 90, 30, 50), None);
}
// #endregion 🔖️H264PixelMathTests
// #endregion 🔖️H264Tests

mod long {
    use super::*;

    #[test]
    fn video_in_contract_i_pcm_then_p_skip_chain_via_full_mp4_pipeline() {
        let (mb_w, mb_h) = (3, 2);
        let (width, height) = (mb_w * 16, mb_h * 16);
        let (luma, cb, cr) = pcm_frame(mb_w, mb_h, 2026);
        let (sps_nal, pps_nal) = h264_enc_sps_pps_nals(mb_w, mb_h);
        let mut samples = vec![h264_enc_i_pcm_sample(mb_w, mb_h, 0, &luma, &cb, &cr)];
        for frame_num in 1..8u32 {
            samples.push(h264_enc_p_skip_sample(mb_w, mb_h, frame_num));
        }
        let mp4 = write_mp4_avc(&samples, &sps_nal, &pps_nal, 12.0);

        let probed = probe_mp4(&mp4).expect("probes the muxed avc stream");
        assert_eq!(probed.codec, VideoCodec::Avc);
        assert_eq!(probed.frame_count, 8);
        assert_eq!(probed.width, width);
        assert_eq!(probed.height, height);

        let opts = VideoIngestOptions { stride: 1, max_frames: 0, max_long_edge_px: 0 };
        let frames: Vec<ExtractedFrame> = extract_frames(&mp4, &opts).expect("extracts").map(|f| f.expect("every frame decodes")).collect();
        assert_eq!(frames.len(), 8);
        let expected = ycbcr420_to_rgba(&luma, (mb_w * 16) as usize, &cb, &cr, (mb_w * 8) as usize, width, height);
        for (i, frame) in frames.iter().enumerate() {
            assert_eq!(frame.image, expected, "frame {i} should reconstruct pixel-exact via the I_PCM + P_Skip chain");
        }
        for i in 1..frames.len() {
            assert!(frames[i].timestamp_ms > frames[i - 1].timestamp_ms, "timestamps must be monotone");
        }
    }
}
