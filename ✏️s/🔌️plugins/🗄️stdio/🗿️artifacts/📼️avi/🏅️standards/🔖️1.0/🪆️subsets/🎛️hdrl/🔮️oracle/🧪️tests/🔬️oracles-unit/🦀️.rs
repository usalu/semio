mod tests {
    use super::*;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free) — see R9
    fn tiny_snapshot() -> ODoc {
        ODoc {
            main_header: OMainHeader {
                micro_sec_per_frame: 100_000,
                max_bytes_per_sec: 1400,
                padding_granularity: 0,
                flags: 0x10,
                total_frames: 2,
                initial_frames: 0,
                streams: 1,
                suggested_buffer_size: 140,
                width: 16,
                height: 16,
                reserved: [0, 0, 0, 0],
            },
            streams: vec![OStream {
                strh: OStreamHeader {
                    fcc_type: "vids".into(),
                    fcc_handler: "MJPG".into(),
                    flags: 0,
                    priority: 0,
                    language: 0,
                    initial_frames: 0,
                    scale: 1,
                    rate: 10,
                    start: 0,
                    length: 2,
                    suggested_buffer_size: 140,
                    quality: -1,
                    sample_size: 0,
                    rc_frame_left: 0,
                    rc_frame_top: 0,
                    rc_frame_right: 16,
                    rc_frame_bottom: 16,
                },
                strf: OStreamFormat::BitmapInfo { size: 40, width: 16, height: 16, planes: 1, bit_count: 24, compression: "MJPG".into(), size_image: 140, x_pels_per_meter: 0, y_pels_per_meter: 0, colors_used: 0, colors_important: 0 },
                chunks: vec![OChunk { fourcc: "00dc".into(), data: vec![1, 2, 3, 4], keyframe: true }, OChunk { fourcc: "00dc".into(), data: vec![5, 6, 7], keyframe: false }],
            }],
            idx1_present: true,
            unknown_chunks: vec![ORiffChunk { fourcc: "JUNK".into(), data: vec![0, 0, 0, 0] }],
        }
    }

    #[test]
    fn encode_decode_round_trips_a_synthetic_document() {
        let doc = tiny_snapshot();
        let bytes = encode(&doc);
        assert!(bytes.starts_with(b"RIFF"));
        assert_eq!(&bytes[8..12], b"AVI ");
        let back = decode(&bytes).expect("decode");
        assert_eq!(back, doc);
    }

    #[test]
    fn decode_recognises_a_non_avi_riff_form() {
        let mut wave = b"RIFF".to_vec();
        wave.extend_from_slice(&4u32.to_le_bytes());
        wave.extend_from_slice(b"WAVE");
        let error = decode(&wave).unwrap_err();
        assert!(error.contains("RIFF form"), "unexpected error: {error}");
    }

    #[test]
    fn apply_mutation_sets_idx1_present_and_inverse_restores_it() {
        let bytes = encode(&tiny_snapshot());
        let spec = obj(vec![("idx1Present", Json::Bool(false))]);
        let mutated = apply_mutation(&bytes, "set-idx1-present", &spec).expect("apply");
        assert!(!decode(&mutated).unwrap().idx1_present);
        let restored = apply_mutation_inverse(&bytes, "set-idx1-present", &spec).expect("apply+inverse");
        assert!(decode(&restored).unwrap().idx1_present);
    }

    #[test]
    fn apply_mutation_removes_and_inverse_restores_a_chunk() {
        let bytes = encode(&tiny_snapshot());
        let spec = obj(vec![("streamIndex", Json::Number(0.0)), ("index", Json::Number(0.0))]);
        let mutated = apply_mutation(&bytes, "remove-chunk", &spec).expect("apply");
        assert_eq!(decode(&mutated).unwrap().streams[0].chunks.len(), 1);
        let restored = apply_mutation_inverse(&bytes, "remove-chunk", &spec).expect("apply+inverse");
        assert_eq!(decode(&restored).unwrap(), tiny_snapshot());
    }

    #[test]
    fn unrecognised_kind_is_an_error_not_a_silent_no_op() {
        let bytes = encode(&tiny_snapshot());
        let error = apply_mutation(&bytes, "not-a-real-kind", &Json::Object(vec![])).unwrap_err();
        assert!(error.contains("no oracle implementation"), "unexpected error: {error}");
    }

    #[test]
    fn project_reports_stream_and_chunk_shape() {
        let bytes = encode(&tiny_snapshot());
        let projection = project(&bytes).expect("project");
        assert_eq!(projection.array("streams").len(), 1);
        assert_eq!(projection.array("streams")[0].array("chunks").len(), 2);
        assert!(projection.get("idx1Present").is_some());
    }
}
