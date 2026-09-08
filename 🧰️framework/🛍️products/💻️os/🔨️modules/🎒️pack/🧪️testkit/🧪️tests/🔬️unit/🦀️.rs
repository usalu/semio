
    use super::*;
    use crate::os_dsl::schema::{FieldSpec, JoinMode, ParseOptions, RecordLayout};

    //#region 🔖️Fixtures
    /// @emoji 🧬️ One field of most scalar `Shape` variants plus a nested `Record`, a `List`, a
    /// `Map`, and a `Tuple` — enough shape variety to exercise `RecordValueGen` and the round-trip
    /// laws without duplicating `pack_value`'s own exhaustive per-tag coverage.
    // 🚫️async: E4 fn-pointer slot — stored bare as `fn() -> RecordSpec` via `Shape::Record` below
    fn nested_point_spec() -> RecordSpec {
        RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(0, "x", Shape::Float), FieldSpec::new(1, "y", Shape::Float)])
    }

    fn mixed_spec() -> RecordSpec {
        RecordSpec::new(
            None,
            RecordLayout::Lines,
            vec![
                FieldSpec::new(1, "flag", Shape::Bool),
                FieldSpec::new(2, "count", Shape::UInt),
                FieldSpec::new(3, "delta", Shape::Int),
                FieldSpec::new(4, "ratio", Shape::Float),
                FieldSpec::new(5, "name", Shape::Text),
                FieldSpec::new(6, "nickname", Shape::Text).optional(),
                FieldSpec::new(7, "payload", Shape::Bytes64),
                FieldSpec::new(8, "color", Shape::Enum(vec![("red".to_string(), 0), ("green".to_string(), 1), ("blue".to_string(), 2)])),
                FieldSpec::new(9, "point", Shape::Record(nested_point_spec)),
                FieldSpec::new(10, "tags", Shape::List(Box::new(Shape::Text))),
                FieldSpec::new(11, "coords", Shape::Tuple(Box::new(Shape::Int), Some(3))),
                FieldSpec::new(12, "labels", Shape::Map(Box::new(Shape::Int))),
            ],
        )
    }

    /// @emoji 📷️ Simple scalar spec, small enough to print/parse deterministically for
    /// `assert_dsl_pack_bidirectional`.
    fn camera_spec() -> RecordSpec {
        RecordSpec::new(Some("camera"), RecordLayout::Inline, vec![FieldSpec::new(0, "x", Shape::Float), FieldSpec::new(1, "y", Shape::Float), FieldSpec::new(2, "zoom", Shape::Float), FieldSpec::new(3, "label", Shape::Text).optional()])
    }

    fn camera_sample() -> RecordValue {
        let mut fields = HashMap::new();
        fields.insert(0, FieldValue::Float(1.0));
        fields.insert(1, FieldValue::Float(2.5));
        fields.insert(2, FieldValue::Float(3.0));
        fields.insert(3, FieldValue::Absent);
        RecordValue { fields }
    }
    //#endregion 🔖️Fixtures

    //#region 🔖️Arbitrary
    #[semio_framework_async_macros::async_test]
    async fn record_value_gen_is_deterministic_for_the_same_seed() {
        let spec = mixed_spec();
        let mut a = RecordValueGen::new(42);
        let mut b = RecordValueGen::new(42);
        assert_eq!(normalize_record(&a.generate(&spec, 4)), normalize_record(&b.generate(&spec, 4)), "same seed must produce the same RecordValue");
    }

    #[semio_framework_async_macros::async_test]
    async fn record_value_gen_differs_across_seeds() {
        let spec = mixed_spec();
        let mut a = RecordValueGen::new(1);
        let mut b = RecordValueGen::new(2);
        assert_ne!(normalize_record(&a.generate(&spec, 4)), normalize_record(&b.generate(&spec, 4)), "different seeds should (almost always) diverge");
    }

    #[semio_framework_async_macros::async_test]
    async fn record_value_gen_produces_values_the_pack_codec_accepts() {
        let spec = mixed_spec();
        for seed in 0..20u64 {
            let record = RecordValueGen::new(seed).generate(&spec, 4);
            let bytes = crate::os_pack::encode_document(&spec, &record, &crate::os_pack::EncodeOptions::default()).unwrap_or_else(|e| panic!("seed {seed}: encode_document failed: {e}"));
            crate::os_pack::decode_document(&bytes, &spec, &crate::os_pack::DecodeOptions::default()).unwrap_or_else(|e| panic!("seed {seed}: decode_document failed: {e}"));
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn record_value_gen_bounds_recursion_by_max_depth() {
        // 🌳️ `group_spec`-style self-referential Statements table: its own single variant's spec
        // points right back at itself. A generator that ignored `max_depth` would stack-overflow.
        // 🚫️async: E4 fn-pointer slot — stored bare as `fn() -> RecordSpec` via `Shape::Statements` below
        fn recursive_spec() -> RecordSpec {
            RecordSpec::new(Some("group"), RecordLayout::Inline, vec![FieldSpec::new(0, "id", Shape::Text), FieldSpec::new(1, "children", Shape::Statements(vec![("group".to_string(), recursive_spec)]))])
        }
        let spec = recursive_spec();
        let mut gen = RecordValueGen::new(7);
        let record = gen.generate(&spec, 3);
        // 🌱️ Merely surviving without a stack overflow (plus a successful pack round trip) is the
        // assertion here — the value's shape isn't otherwise constrained.
        assert_encode_decode_identity(&spec, &record).await;
    }
    //#endregion 🔖️Arbitrary

    //#region 🔖️Laws
    #[semio_framework_async_macros::async_test]
    async fn law_encode_decode_identity_holds_for_generated_records() {
        let spec = mixed_spec();
        for seed in 0..12u64 {
            let record = RecordValueGen::new(seed).generate(&spec, 4);
            assert_encode_decode_identity(&spec, &record).await;
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn law_canonical_stable_holds_for_generated_records() {
        let spec = mixed_spec();
        for seed in 0..12u64 {
            let record = RecordValueGen::new(seed).generate(&spec, 4);
            assert_canonical_stable(&spec, &record).await;
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn law_unknown_field_preserved_holds() {
        let spec = mixed_spec();
        let mut record = RecordValueGen::new(3).generate(&spec, 3);
        record.fields.insert(999, FieldValue::Text("from-the-future".to_string()));
        record.fields.insert(1000, FieldValue::UInt(7));
        assert_unknown_field_preserved(&spec, &record, &[999, 1000]).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn law_streamed_equals_buffered_holds() {
        let spec = mixed_spec();
        for seed in 0..6u64 {
            let record = RecordValueGen::new(seed).generate(&spec, 4);
            assert_streamed_equals_buffered(&spec, &record).await;
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn law_dsl_pack_bidirectional_holds_for_a_hand_built_sample() {
        let spec = camera_spec();
        let parse_dsl = async |text: &str| crate::os_dsl::schema::parse(text, &spec, &ParseOptions::default()).unwrap_or_else(|e| panic!("parse failed: {e}"));
        let print_dsl = async |value: &RecordValue| crate::os_dsl::schema::print(value, &spec, JoinMode::Document);
        let encode_pack = async |value: &RecordValue| crate::os_pack::encode_document(&spec, value, &crate::os_pack::EncodeOptions::default()).expect("encode_pack");
        let decode_pack = async |bytes: &[u8]| crate::os_pack::decode_document(bytes, &spec, &crate::os_pack::DecodeOptions::default()).expect("decode_pack").0;
        assert_dsl_pack_bidirectional(parse_dsl, print_dsl, encode_pack, decode_pack, &camera_sample()).await;
    }
    //#endregion 🔖️Laws

    //#region 🔖️Corrupt
    // 🚫️async: E1 pure adapter consumed by `fuzz_truncation`/`fuzz_bit_flips`'s sync `impl Fn`
    // decoder slot — bridges via `os_io::resolve_ready` (the same sanctioned pattern already used
    // by `📡️spr/🧪️testkit`'s `fuzz_truncation_never_panics_history_reader_open`), see R9/E5.
    fn decode_closure(spec: RecordSpec) -> impl Fn(&[u8]) -> Result<(), String> {
        move |bytes: &[u8]| crate::os_pack::decode_document(bytes, &spec, &crate::os_pack::DecodeOptions::default()).map(|_| ()).map_err(|e| e.to_string())
    }

    #[semio_framework_async_macros::async_test]
    async fn fuzz_truncation_never_panics_on_a_real_encoded_document() {
        let spec = mixed_spec();
        let record = RecordValueGen::new(9).generate(&spec, 4);
        let bytes = crate::os_pack::encode_document(&spec, &record, &crate::os_pack::EncodeOptions::default()).expect("encode_document");
        let report = fuzz_truncation(&bytes, CorruptionLevel::Quick, decode_closure(spec));
        assert!(report.cases_panicked.is_empty(), "fuzz_truncation observed panics: {:?}", report.cases_panicked);
        assert!(report.cases_run > 0, "fuzz_truncation must have run at least one case");
    }

    #[semio_framework_async_macros::async_test]
    async fn fuzz_bit_flips_never_panics_on_a_real_encoded_document() {
        let spec = mixed_spec();
        let record = RecordValueGen::new(11).generate(&spec, 4);
        let bytes = crate::os_pack::encode_document(&spec, &record, &crate::os_pack::EncodeOptions::default()).expect("encode_document");
        let report = fuzz_bit_flips(&bytes, CorruptionLevel::Quick, decode_closure(spec));
        assert!(report.cases_panicked.is_empty(), "fuzz_bit_flips observed panics: {:?}", report.cases_panicked);
        assert!(report.cases_run > 0, "fuzz_bit_flips must have run at least one case");
    }

    #[semio_framework_async_macros::async_test]
    async fn fuzz_truncation_and_bit_flips_report_zero_cases_for_empty_input() {
        let decode: fn(&[u8]) -> Result<(), String> = |_| Ok(());
        let truncation_report = fuzz_truncation(&[], CorruptionLevel::Quick, decode);
        let bit_flip_report = fuzz_bit_flips(&[], CorruptionLevel::Quick, decode);
        assert_eq!(truncation_report.cases_run, 0);
        assert_eq!(bit_flip_report.cases_run, 0);
        assert!(truncation_report.cases_panicked.is_empty());
        assert!(bit_flip_report.cases_panicked.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn fuzz_harness_catches_a_panicking_decoder_instead_of_crashing_the_process() {
        let always_panics: fn(&[u8]) -> Result<(), String> = |_| panic!("intentional panic for harness self-test");
        let report = fuzz_truncation(&[1, 2, 3, 4], CorruptionLevel::Quick, always_panics);
        assert_eq!(report.cases_panicked.len() as u64, report.cases_run, "every case should have been caught as a panic");
        assert!(report.cases_panicked.iter().all(|msg| msg.contains("intentional panic")), "panic message should be captured: {:?}", report.cases_panicked);
    }
    //#endregion 🔖️Corrupt

    //#region 🔖️Golden
    #[semio_framework_async_macros::async_test]
    async fn golden_hash_hex_is_deterministic_and_sensitive_to_content() {
        let a = golden_hash_hex(b"hello pack").await;
        let b = golden_hash_hex(b"hello pack").await;
        let c = golden_hash_hex(b"hello pack!").await;
        assert_eq!(a, b, "golden_hash_hex must be deterministic for the same bytes");
        assert_ne!(a, c, "golden_hash_hex must differ for different bytes");
        assert_eq!(a.len(), 64, "blake3 hex digest is 64 hex chars (32 bytes)");
        assert!(a.chars().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()), "golden_hash_hex must be lowercase hex: {a}");
    }

    #[semio_framework_async_macros::async_test]
    async fn golden_hash_hex_matches_a_real_encoded_document_across_two_encodes() {
        let spec = mixed_spec();
        let record = RecordValueGen::new(5).generate(&spec, 4);
        let a = crate::os_pack::encode_document(&spec, &record, &crate::os_pack::EncodeOptions::default()).unwrap();
        let b = crate::os_pack::encode_document(&spec, &record, &crate::os_pack::EncodeOptions::default()).unwrap();
        assert_eq!(golden_hash_hex(&a).await, golden_hash_hex(&b).await, "golden hash of a canonical encoding must be stable across repeated encodes");
    }
    //#endregion 🔖️Golden

    //#region 🔖️Long
    mod long {
        use super::*;

        #[semio_framework_async_macros::async_test]
        async fn fuzz_truncation_and_bit_flips_never_panic_at_long_density() {
            let spec = mixed_spec();
            let record = RecordValueGen::new(21).generate(&spec, 5);
            let bytes = crate::os_pack::encode_document(&spec, &record, &crate::os_pack::EncodeOptions::default()).expect("encode_document");
            let truncation_report = fuzz_truncation(&bytes, CorruptionLevel::Long, decode_closure(spec.clone()));
            let bit_flip_report = fuzz_bit_flips(&bytes, CorruptionLevel::Long, decode_closure(spec));
            assert!(truncation_report.cases_panicked.is_empty(), "long-level truncation fuzz observed panics: {:?}", truncation_report.cases_panicked);
            assert!(bit_flip_report.cases_panicked.is_empty(), "long-level bit-flip fuzz observed panics: {:?}", bit_flip_report.cases_panicked);
            assert!(truncation_report.cases_run >= bytes.len().min(128) as u64);
            assert!(bit_flip_report.cases_run >= 128);
        }
    }
    //#endregion 🔖️Long

    //#region 🔖️Exhaustive
    mod exhaustive {
        use super::*;

        #[semio_framework_async_macros::async_test]
        async fn fuzz_truncation_never_panics_at_every_single_byte_offset() {
            let spec = mixed_spec();
            let record = RecordValueGen::new(33).generate(&spec, 5);
            let bytes = crate::os_pack::encode_document(&spec, &record, &crate::os_pack::EncodeOptions::default()).expect("encode_document");
            let report = fuzz_truncation(&bytes, CorruptionLevel::Exhaustive, decode_closure(spec));
            assert!(report.cases_panicked.is_empty(), "exhaustive truncation fuzz observed panics: {:?}", report.cases_panicked);
            assert_eq!(report.cases_run, bytes.len() as u64, "exhaustive truncation must try every offset");
        }

        #[semio_framework_async_macros::async_test]
        async fn fuzz_bit_flips_never_panics_at_every_single_bit() {
            let spec = mixed_spec();
            let record = RecordValueGen::new(34).generate(&spec, 3);
            let bytes = crate::os_pack::encode_document(&spec, &record, &crate::os_pack::EncodeOptions::default()).expect("encode_document");
            let report = fuzz_bit_flips(&bytes, CorruptionLevel::Exhaustive, decode_closure(spec));
            assert!(report.cases_panicked.is_empty(), "exhaustive bit-flip fuzz observed panics: {:?}", report.cases_panicked);
            assert_eq!(report.cases_run, bytes.len() as u64 * 8, "exhaustive bit-flip must try every bit");
        }
    }
    //#endregion 🔖️Exhaustive
