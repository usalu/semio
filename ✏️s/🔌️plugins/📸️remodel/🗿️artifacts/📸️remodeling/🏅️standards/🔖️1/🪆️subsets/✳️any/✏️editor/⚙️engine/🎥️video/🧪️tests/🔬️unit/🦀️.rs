use super::*;
use semio_s_artifact_remodel_remodeling_video_h264::editor::remodeling::engine::video::h264::{h264_enc_i_pcm_sample, h264_enc_p_skip_sample, h264_enc_sps_pps, h264_enc_sps_pps_nals};

/// 🔌️Exercises extension callbacks and refusals without adding a concrete container branch.
#[test]
fn video_container_provider_callbacks_v1() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static CALLS: AtomicUsize = AtomicUsize::new(0);
    fn recognizes(bytes: &[u8]) -> bool { bytes == b"future" }
    fn unmatched(_: &[u8]) -> bool { false }
    fn forbidden(_: &[u8]) -> bool { panic!("invalid inventory must not invoke recognition") }
    fn project(bytes: &[u8]) -> Result<VideoProbe, VideoError> {
        assert_eq!(bytes, b"future");
        CALLS.fetch_add(1, Ordering::SeqCst);
        let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔌️container-providers/🔣️.json")).expect("portable corpus");
        Ok(serde_json::from_value(corpus["probes"][0]["probe"].clone()).expect("owned record"))
    }
    fn refuse(_: &[u8]) -> Result<VideoProbe, VideoError> { Err(VideoError::Container("owned callback failure".into())) }
    let selected = VideoContainerProviderV1 { id: "future", matches: recognizes, probe: project };
    let unrelated = VideoContainerProviderV1 { id: "unrelated", matches: unmatched, probe: project };
    let info = probe_with_providers_v1(&[unrelated, selected], b"future").expect("future provider");
    assert_eq!(info.container, "future");
    assert_eq!(CALLS.load(Ordering::SeqCst), 1);
    assert_eq!(probe_with_providers_v1(&[], b"future"), Err(VideoError::ContainerAdmission(VideoContainerAdmissionErrorV1::MissingProvider)));
    assert_eq!(probe_with_providers_v1(&[selected, VideoContainerProviderV1 { id: "other", ..selected }], b"future"), Err(VideoError::ContainerAdmission(VideoContainerAdmissionErrorV1::AmbiguousProvider)));
    assert_eq!(probe_with_providers_v1(&[VideoContainerProviderV1 { id: "../invalid", matches: forbidden, ..selected }], b"future"), Err(VideoError::ContainerAdmission(VideoContainerAdmissionErrorV1::InvalidInventory)));
    assert_eq!(CALLS.load(Ordering::SeqCst), 1);
    assert_eq!(probe_with_providers_v1(&[VideoContainerProviderV1 { probe: refuse, ..selected }], b"future"), Err(VideoError::Container("owned callback failure".into())));
    assert_eq!(probe_with_providers_v1(&[VideoContainerProviderV1 { id: "other", ..selected }], b"future"), Err(VideoError::InvalidProbe));
}



use semio_s_artifact_stdio_mp4::{standards::isobmff::subsets::any::{io as mp4_engine, schema::snapshot::{Mp4Codec, Mp4CodecFormat, Mp4Ftyp, Mp4Sample, Mp4Snapshot, Mp4Track}}, STDIO_MP4_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_avi::{standards::v1_0::subsets::any::{io as avi_engine, schema::snapshot::{AviMainHeader, AviSnapshot, AviStream, AviStreamFormat, AviStreamHeader}}, STDIO_AVI_DOCUMENT_SCHEMA};

/// 🔌️Checks every provider record vector before accepting extension-owned samples.
#[test]
fn video_container_provider_probe_corpus_v1() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔌️container-providers/🔣️.json")).expect("portable corpus");
    for row in corpus["probes"].as_array().expect("probes") {
        let actual = serde_json::from_value::<VideoProbe>(row["probe"].clone()).map(|probe| admit_video_probe_v1(row["provider"].as_str().expect("provider"), &probe)).unwrap_or(false);
        assert_eq!(actual, row["expected"].as_bool().expect("expected"), "{}", row["id"]);
    }
}

/// 🔌️Proves explicit provider selection against every closed portable inventory vector.
#[test]
fn video_container_provider_corpus_v1() {
    fn selected(_: &[u8]) -> bool { true }
    fn unselected(_: &[u8]) -> bool { false }
    fn unused(_: &[u8]) -> Result<VideoProbe, VideoError> { panic!("selection must not invoke probes") }
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔌️container-providers/🔣️.json")).expect("portable corpus");
    for row in corpus["cases"].as_array().expect("cases") {
        let providers: Vec<_> = row["providers"].as_array().expect("providers").iter().map(|provider| VideoContainerProviderV1 {
            id: provider["id"].as_str().expect("id"),
            matches: if provider["matches"].as_bool().expect("matches") { selected } else { unselected },
            probe: unused,
        }).collect();
        let actual = match select_video_container_provider_v1(&providers, &[]) {
            Ok(index) => serde_json::json!({"status":"selected","id":providers[index].id}),
            Err(VideoContainerAdmissionErrorV1::MissingProvider) => serde_json::json!({"status":"missing"}),
            Err(VideoContainerAdmissionErrorV1::AmbiguousProvider) => serde_json::json!({"status":"ambiguous"}),
            Err(VideoContainerAdmissionErrorV1::InvalidInventory) => serde_json::json!({"status":"invalid"}),
        };
        assert_eq!(actual, row["expected"], "{}", row["id"]);
    }
}

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

fn pcm_frame(mb_w: u32, mb_h: u32, seed: u64) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let mut state = seed;
    let luma = fill_deterministic(&mut state, (mb_w * 16 * mb_h * 16) as usize);
    let cb = fill_deterministic(&mut state, (mb_w * 8 * mb_h * 8) as usize);
    let cr = fill_deterministic(&mut state, (mb_w * 8 * mb_h * 8) as usize);
    (luma, cb, cr)
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
/// 🔬Checks owned MP4 timing against the linked provider's real demux/mux implementation.
#[test]
fn write_mp4_mjpeg_probe_round_trip_reports_exact_frames() {
    let frames: Vec<Vec<u8>> = (0..5).map(|i| remodeling_image::encode_jpeg(&synth_rgba(16, 16, 100 + i), 90)).collect();
    let mp4 = container_providers::mp4::write_mjpeg(&frames, 10.0);
    let info = container_providers::mp4::probe(&mp4).expect("probes");
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
    let mp4 = container_providers::mp4::write_avc(&[h264_enc_i_pcm_sample(1, 1, 0, &[0; 256], &[0; 64], &[0; 64])], &sps_nal, &pps_nal, 5.0);
    let info = container_providers::mp4::probe(&mp4).expect("probes");
    assert_eq!(info.codec, VideoCodec::Avc);
    assert_eq!(info.width, 16);
    assert_eq!(info.height, 16);
    assert!(info.avc_config.is_some());
}

/// 🧯Preserves the linked container implementation's actual decode refusal.
#[test]
fn mp4_probe_wraps_stdio_decode_errors_as_container() {
    assert!(matches!(container_providers::mp4::probe(&[]), Err(VideoError::Container(_))));
    assert!(matches!(container_providers::mp4::probe(b"not an mp4 at all"), Err(VideoError::Container(_))));
}

#[test]
fn mp4_probe_reports_no_video_track_when_none_present() {
    let snapshot = Mp4Snapshot { schema: STDIO_MP4_DOCUMENT_SCHEMA.into(), ftyp: Mp4Ftyp { major_brand: "isom".into(), minor_version: 0, compatible_brands: vec!["isom".into()] }, movie: Default::default(), tracks: vec![] };
    let bytes = mp4_engine::encode_mp4(&snapshot);
    assert!(matches!(container_providers::mp4::probe(&bytes), Err(VideoError::NoVideoTrack)));
}

#[test]
fn write_avi_mjpg_probe_round_trip_reports_exact_frames() {
    let frames: Vec<Vec<u8>> = (0..4).map(|i| remodeling_image::encode_jpeg(&synth_rgba(8, 8, 300 + i), 85)).collect();
    let avi = container_providers::avi::write_mjpeg(&frames, 8.0);
    let info = container_providers::avi::probe(&avi).expect("probes");
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
    assert!(matches!(container_providers::avi::probe(b"not an avi at all!!"), Err(VideoError::Container(_))));
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
    assert!(matches!(container_providers::avi::probe(&bytes), Err(VideoError::NoVideoTrack)));
}

#[test]
fn probe_selects_present_container_providers() {
    let frames: Vec<Vec<u8>> = (0..2).map(|i| remodeling_image::encode_jpeg(&synth_rgba(4, 4, 900 + i), 80)).collect();
    let avi = container_providers::avi::write_mjpeg(&frames, 5.0);
    assert_eq!(probe(&avi).expect("selected AVI provider").container, "avi");
    let mp4 = container_providers::mp4::write_mjpeg(&frames, 5.0);
    assert_eq!(probe(&mp4).expect("selected MP4 provider").container, "mp4");
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
    let mp4 = container_providers::mp4::write_mjpeg(&frames, 10.0);
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
    let mp4 = container_providers::mp4::write_mjpeg(&frames, 10.0);
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
    let mp4 = container_providers::mp4::write_mjpeg(&frames, 5.0);
    let opts = VideoIngestOptions { stride: 1, max_frames: 0, max_long_edge_px: 16 };
    let extracted: Vec<ExtractedFrame> = extract_frames(&mp4, &opts).expect("extracts").map(|f| f.expect("decodes")).collect();
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].image.width, 16);
    assert_eq!(extracted[0].image.height, 8);
}

#[test]
fn extract_frames_rejects_unsupported_codec_with_provenance() {
    let track =
        Mp4Track { track_id: 1, timescale: 1000, codec: Mp4Codec::hevc(Mp4CodecFormat::Hvc1, Default::default(), 4), width: 4, height: 4, metadata: Default::default(), chunk_sample_counts: vec![1], samples: vec![Mp4Sample { data: vec![0; 10], duration: 100, cts_offset: 0, sync: true }] };
    let snapshot = Mp4Snapshot { schema: STDIO_MP4_DOCUMENT_SCHEMA.into(), ftyp: Mp4Ftyp { major_brand: "isom".into(), minor_version: 0, compatible_brands: vec!["isom".into()] }, movie: Default::default(), tracks: vec![track] };
    let bytes = mp4_engine::encode_mp4(&snapshot);
    let info = container_providers::mp4::probe(&bytes).expect("hvc1 still probes for provenance");
    assert_eq!(info.codec, VideoCodec::Hevc);
    let opts = VideoIngestOptions { stride: 1, max_frames: 0, max_long_edge_px: 0 };
    assert!(matches!(extract_frames(&bytes, &opts), Err(VideoError::UnsupportedCodec(_))));
}

#[test]
fn extract_frames_mjpeg_propagates_jpeg_decode_error() {
    let frames = vec![vec![0xDE, 0xAD, 0xBE, 0xEF]];
    let mp4 = container_providers::mp4::write_mjpeg(&frames, 5.0);
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
    let info = container_providers::avi::probe(&bytes).expect("XVID still probes for provenance");
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
        let mp4 = container_providers::mp4::write_avc(&samples, &sps_nal, &pps_nal, 12.0);

        let probed = container_providers::mp4::probe(&mp4).expect("probes the muxed avc stream");
        assert_eq!(probed.codec, VideoCodec::Avc);
        assert_eq!(probed.frame_count, 8);
        assert_eq!(probed.width, width);
        assert_eq!(probed.height, height);

        let opts = VideoIngestOptions { stride: 1, max_frames: 0, max_long_edge_px: 0 };
        let frames: Vec<ExtractedFrame> = extract_frames(&mp4, &opts).expect("extracts").map(|f| f.expect("every frame decodes")).collect();
        assert_eq!(frames.len(), 8);
        let expected = H264Decoder::new(&h264_enc_sps_pps(mb_w, mb_h)).expect("sps/pps parse").decode_sample(&samples[0]).expect("direct decode").expect("immediate output");
        for (i, frame) in frames.iter().enumerate() {
            assert_eq!(frame.image, expected, "frame {i} should reconstruct pixel-exact via the I_PCM + P_Skip chain");
        }
        for i in 1..frames.len() {
            assert!(frames[i].timestamp_ms > frames[i - 1].timestamp_ms, "timestamps must be monotone");
        }
    }
}
