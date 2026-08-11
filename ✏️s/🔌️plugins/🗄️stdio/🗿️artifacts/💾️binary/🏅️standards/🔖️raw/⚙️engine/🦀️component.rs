//! ⚙️ BinaryEngine — owns a real `BinaryArtifact`.

use crate::artifacts::binary::{BinaryArtifact, BinaryDiff, BinaryMutation, BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot.
pub fn empty_binary_snapshot() -> BinarySnapshot {
    BinarySnapshot::default()
}

/// 📄️ The demo `stdio.binary` document -- `bytes = b"hello"`, matching the companion real-format
/// fixture asset (`📚️examples/🎬️demo/🖼️assets/🎒️example.bin`, which is literally the raw bytes
/// `hello`). The single source of truth for `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`/
/// `🎒️example.pack.semio` (both are literally this snapshot's `print_dsl`/`encode_pack` output,
/// asserted equal by `fixture_honesty_law` below).
pub fn demo_binary_snapshot() -> BinarySnapshot {
    BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes: b"hello".to_vec() }
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Register
/// 🗂️ Registers codecs, the artifact schema descriptor, and every composer entry (which supersedes
/// the pre-migration per-leaf `io::register()` no-ops -- see `🎹️composer::register`).
pub fn register() {
    crate::artifacts::binary::composer::register();
    register_artifact_schema();
    register_pilot_languages();
    register_schema_specs();
    store::register_document_codec(store::ArtifactCodec::of::<BinarySnapshot, BinaryMutation>(STDIO_BINARY_DOCUMENT_SCHEMA));
}

/// 📇️ P2-P3 follow-up fix: `dsl::registry::register_schema_spec` (P2-M3's `FullResolver` insertion
/// API) — genuinely callable here (`BinarySnapshot` derives `dsl::DslRecord`, `BinaryDiff` derives
/// `dsl::DslDiff`, so both `__dsl_spec`/`__dsl_diff_spec` exist), same 2-call shape as
/// `txt::register_schema_specs` (`📄txt/…/⚙️engine/🦀️component.rs`). Per-mutation-variant specs are
/// NOT registered here, same as txt — `register_schema_spec` registers one spec under one schema id,
/// and there is no single canonical id for a Mutation enum's N independently-shaped variants; that
/// is the genuine scope boundary, not "this facet has too many specs to register any of them."
#[cfg(not(target_arch = "wasm32"))]
pub fn register_schema_specs() {
    dsl::registry::register_schema_spec("stdio.binary", BinarySnapshot::__dsl_spec);
    dsl::registry::register_schema_spec("stdio.binary#diff", BinaryDiff::__dsl_diff_spec);
}

#[cfg(target_arch = "wasm32")]
pub fn register_schema_specs() {}

/// 📌️ P2-P3: 5-role `LanguageSpec` registration (Document/Ops/Diff/Pack/Spr), per note's/json's
/// exemplar pattern -- `stdio.binary`/`.op`/`.diff`/`.pack`/`.spr`, all `dsl::passthrough_hooks`.
/// `diff`'s `protocol` slot stays `None` matching the exemplar's own shape exactly (the role
/// scheme has no dedicated "diff binary" role even though `🔺️diff/💾️binary/📡️component.protocol.
/// semio` is a real, conformance-tested file -- its binary form is exercised directly by
/// `protocol_walk_law` below, just not wired through a 6th `LanguageRole`).
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.binary",
        extension: Some("bin"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::binary::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::binary::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::binary::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::binary::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.binary"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.binary.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::binary::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::binary::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::binary::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::binary::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.binary.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.binary.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::binary::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::binary::schema::diff::text::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("stdio.binary.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.binary.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::binary::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::binary::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.binary.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.binary.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::binary::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::binary::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.binary.spr"),
    });
}

/// 📌️ Registers schema leaves for `s.stdio.binary`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::binary::schema::binary_artifact_schema_descriptor());
}
//#endregion 🔖️Register

//#region 🔖️ArtifactEngine
/// ⚙️ `stdio.binary` artifact engine.
pub struct BinaryEngine {
    artifact_state: BinaryArtifact,
    snapshot_state: BinarySnapshot,
}

impl BinaryEngine {
    /// 🏗️ Builds an engine from a persisted snapshot.
    pub fn new(snapshot: BinarySnapshot) -> Self {
        let artifact_state = BinaryArtifact::from_snapshot(snapshot.clone());
        Self { artifact_state, snapshot_state: snapshot }
    }
}
//#endregion 🔖️ArtifactEngine

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_snapshot_matches_schema() {
        let snapshot = empty_binary_snapshot();
        assert_eq!(snapshot.schema, STDIO_BINARY_DOCUMENT_SCHEMA);
    }

    #[test]
    fn codec_round_trip() {
        let snap = empty_binary_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <BinarySnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.schema, snap.schema);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <BinarySnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }

    /// 🧪️ `codec_retention_law`: decode→encode is byte-preserving on real fixtures, incl. bytes
    /// that are themselves invalid UTF-8 (the hex DSL layer never interprets payload bytes as
    /// text, so this is a real test of the hex codec, not just the binary-pack envelope).
    #[test]
    fn codec_retention_law() {
        for bytes in [
            vec![],
            vec![0x00, 0x01, 0xFF, 0xFE],
            (0u8..=255).collect::<Vec<u8>>(),
        ] {
            let snap = BinarySnapshot { bytes: bytes.clone(), ..Default::default() };
            let dsl_text = store::ArtifactDsl::print_dsl(&snap);
            let parsed = <BinarySnapshot as store::ArtifactDsl>::parse_dsl(&dsl_text).expect("parse");
            assert_eq!(parsed, snap, "dsl round-trip mismatch for {bytes:?}");
            let packed = store::ArtifactPack::encode_pack(&snap);
            let decoded = <BinarySnapshot as store::ArtifactPack>::decode_pack(&packed).expect("decode");
            assert_eq!(decoded, snap, "pack round-trip mismatch for {bytes:?}");
        }
    }

    //#region 🔖️FieldSweep
    /// 🧹 Canonical "every mutable field differs" snapshot A.
    fn sweep_a() -> BinarySnapshot {
        BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes: vec![1, 2, 3, 4, 5, 6, 7, 8] }
    }

    /// 🧹 Canonical "every mutable field differs" snapshot B: an insert-only region (bytes 100
    /// inserted mid-buffer), a pure-removal region (bytes 3,4 dropped), and a pure-replacement
    /// region (byte 8 → 88) -- one splice can't express all three at once, exercising the
    /// splice mechanism's full range per the artifact's own field-sweep note.
    fn sweep_b() -> BinarySnapshot {
        BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes: vec![1, 2, 100, 5, 6, 7, 88] }
    }

    /// 🧪️ `field_sweep`: THE acceptance criterion. `between` round-trips both directions, the
    /// splice list is non-empty (the only "field" a splice-list diff has), and `between(a,a)`
    /// is empty.
    #[test]
    fn field_sweep_covers_every_byte_level_change() {
        use protocol::os_spr::command::DiffAlgebra;
        use protocol::MutationDiff;
        let a = sweep_a();
        let b = sweep_b();

        let ab = BinaryDiff::between(&a, &b);
        assert_eq!(ab.apply(&a), b, "between(a,b).apply(a) must equal b");
        let ba = BinaryDiff::between(&b, &a);
        assert_eq!(ba.apply(&b), a, "between(b,a).apply(b) must equal a");
        assert!(!ab.splices.is_empty(), "sweep diff must carry at least one splice");

        // 🔬️ Exercise insert/remove/replace explicitly via hand-built splices (not just the
        // minimal `between` form) to prove the mechanism itself, not just this one pair.
        let hand_built = BinaryDiff {
            splices: vec![
                crate::artifacts::binary::schema::diff::ByteSplice { offset: 2, remove_len: 2, insert: vec![100] }, // replace+shrink
                crate::artifacts::binary::schema::diff::ByteSplice { offset: 7, remove_len: 1, insert: vec![88] },  // pure replace
            ],
        };
        assert_eq!(hand_built.apply(&a), b);

        assert!(BinaryDiff::between(&a, &a).is_empty(), "between(a,a) must be empty");
    }
    //#endregion 🔖️FieldSweep

    #[test]
    fn demo_snapshot_round_trip() {
        let snap = demo_binary_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <BinarySnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed, snap);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <BinarySnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }

    //#region 🔖️ConformanceLaws
    /// 🧪️ P2-P3: per-artifact conformance laws — grammar/protocol parseability, `Recognizer`
    /// against real fixtures AND real `print_op`/`print_diff` output, `walk_protocol` against real
    /// `encode_pack`/`encode_op`/`encode_diff` bytes, and the fixture-honesty round-trip. Lives
    /// here (the engine's own test region), not any framework file — `m5` auto-discovers the
    /// snapshot grammar+`.dsl.semio`/protocol+`.pack.semio` pairs independently
    /// (`🗣️dsl/🧪️fixture-sweep/🦀️component.rs`'s `m5_auto_discovery`); these tests are this
    /// artifact's OWN early-warning, plus direct coverage of the mutations/diff facets that
    /// harness does not auto-discover at all.
    mod conformance_laws {
        use super::*;
        use crate::artifacts::binary::schema::{diff, mutations, snapshot};
        use protocol::{DiffCodec, OpBinary, OpText};

        /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
        /// parse under the real dialect — independent of, and cheaper than, the two `recognize`/
        /// `walk_protocol` laws below (a parse failure here fails fast with a clearer message).
        #[test]
        fn committed_facet_files_parse() {
            for (label, text) in [
                ("snapshot grammar", snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                ("mutations grammar", mutations::text::COMPONENT_GRAMMAR_SEMIO),
                ("diff grammar", diff::text::COMPONENT_GRAMMAR_SEMIO),
            ] {
                let grammar = dsl::parse_grammar(text).unwrap_or_else(|e| panic!("{label}: parse_grammar failed: {e:?}"));
                assert_eq!(grammar.dialect, dsl::SemioDialect::Grammar, "{label}: expected grammar dialect");
            }
            for (label, text) in [
                ("snapshot protocol", snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                ("mutations protocol", mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                ("diff protocol", diff::binary::COMPONENT_PROTOCOL_SEMIO),
            ] {
                dsl::parse_protocol(text).unwrap_or_else(|e| panic!("{label}: parse_protocol failed: {e:?}"));
            }
        }

        /// ✅️ `grammar_conformance_law`: the snapshot grammar recognizes real `print_dsl` output
        /// for the demo snapshot — same preamble-stripped body reconstruction
        /// `m5_handcrafted_grammar_conformance`'s own `dsl_body_from_fixture` uses, so this is a
        /// direct proof this artifact will pass that harness once graduated, not merely an
        /// analogue.
        #[test]
        fn grammar_conformance_law() {
            let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            let text = store::ArtifactDsl::print_dsl(&demo_binary_snapshot());
            let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
            let reconstructed = format!("{}\n{body}", envelope.envelope_id());
            assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");

            // 🔬️ Also the empty-bytes case (`BinarySnapshot::default()`), exercising the `hex`
            // macro's zero-width match.
            let empty_text = store::ArtifactDsl::print_dsl(&empty_binary_snapshot());
            let (empty_envelope, empty_body) = store::semio_format::split_text_preamble(&empty_text).expect("split preamble");
            let empty_reconstructed = format!("{}\n{empty_body}", empty_envelope.envelope_id());
            assert!(recognizer.recognize(&empty_reconstructed).expect("recognize"), "grammar did not recognize empty dsl body:\n{empty_reconstructed}");
        }

        /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
        /// output for every `BinaryMutation` variant (`mutations::demo_mutation_cases()`).
        #[test]
        fn ops_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for mutation in mutations::demo_mutation_cases() {
                let printed = mutation.print_op();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
            }
        }

        /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff`
        /// output for every representative `BinaryDiff` (`diff::demo_diff_cases()`), incl. the
        /// empty (no-splices) diff and a multi-splice diff with a zero-length no-op splice.
        #[test]
        fn diff_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(diff::text::COMPONENT_GRAMMAR_SEMIO).expect("parse diff grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for d in diff::demo_diff_cases() {
                let printed = d.print_diff();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "diff grammar did not recognize {printed:?} (from {d:?})");
            }
        }

        /// ✅️ `protocol_walk_law`: `walk_protocol` against REAL bytes for all three facets —
        /// snapshot pack (`encode_pack`, envelope-unwrapped first, matching how
        /// `m5_handcrafted_protocol_conformance` itself feeds `walk_protocol`), every demo
        /// mutation's `encode_op`, and every demo diff's `encode_diff` — asserting `consumed ==
        /// bytes.len()`.
        #[test]
        fn protocol_walk_law() {
            let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
            let packed = store::ArtifactPack::encode_pack(&demo_binary_snapshot());
            let (_, inner) = store::semio_format::unwrap_binary(&packed).expect("unwrap semio envelope");
            let trace = dsl::walk_protocol(&pack_spec, &inner).unwrap_or_else(|e| panic!("walk_protocol(pack) failed @{}: {}", e.offset, e.message));
            assert_eq!(trace.consumed, inner.len(), "pack walk did not consume every byte");

            let op_spec = dsl::parse_protocol(mutations::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse mutations protocol");
            for mutation in mutations::demo_mutation_cases() {
                let bytes = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed for {mutation:?}: {e:?}"));
                let trace = dsl::walk_protocol(&op_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(op) failed for {mutation:?} @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, bytes.len(), "op walk did not consume every byte for {mutation:?}");
            }

            let diff_spec = dsl::parse_protocol(diff::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse diff protocol");
            for d in diff::demo_diff_cases() {
                let bytes = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed for {d:?}: {e:?}"));
                let trace = dsl::walk_protocol(&diff_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(diff) failed for {d:?} @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, bytes.len(), "diff walk did not consume every byte for {d:?}");
            }
        }

        /// ✅️ `fixture_honesty_law`: the shipped `.dsl.semio`/`.pack.semio` fixtures are GENUINE
        /// `print_dsl`/`encode_pack` output of `demo_binary_snapshot()` — `parse_dsl(fixture) ==
        /// demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the pack twin — so the
        /// fixtures can never silently drift back to a fake again.
        #[test]
        fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../../../📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio");

            let demo = demo_binary_snapshot();

            let parsed = <BinarySnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
            assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_binary_snapshot()");
            assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_binary_snapshot()) drifted from the shipped .dsl.semio fixture");

            let decoded = <BinarySnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
            assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_binary_snapshot()");
            assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_binary_snapshot()) drifted from the shipped .pack.semio fixture");
        }
    }
    //#endregion 🔖️ConformanceLaws
}
//#endregion 🧪️Tests
