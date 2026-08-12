//! ⚙️ JsonEngine — owns a real `JsonArtifact`.

use crate::artifacts::json::{JsonArtifact, JsonDiff, JsonMutation, JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot.
pub fn empty_json_snapshot() -> JsonSnapshot {
    JsonSnapshot::default()
}

/// 📄️ The demo `stdio.json` document — a genuinely 3-level-nested `JsonValue` (object → array,
/// object → object → object → array) exercising every `JsonValue` variant (`Null`/`Bool`/`Number`/
/// `String`/`Array`/`Object`) at least once. The single source of truth for
/// `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`/`🎒️example.pack.semio` (both are literally this
/// snapshot's `print_dsl`/`encode_pack` output, asserted equal by `fixture_honesty_law` below) and
/// for `nontrivial_nested_value_round_trip`'s own fixture, which now calls this instead of
/// duplicating the literal.
pub fn demo_json_snapshot() -> JsonSnapshot {
    use crate::artifacts::json::schema::snapshot::{JsonMember, JsonValue};
    let value = JsonValue::Object {
        members: vec![
            JsonMember { key: "name".into(), value: JsonValue::String { value: "semio".into() } },
            JsonMember { key: "count".into(), value: JsonValue::Number { lexeme: "42".into() } },
            JsonMember { key: "ratio".into(), value: JsonValue::Number { lexeme: "3.5".into() } },
            JsonMember { key: "active".into(), value: JsonValue::Bool { value: true } },
            JsonMember { key: "missing".into(), value: JsonValue::Null },
            JsonMember {
                key: "tags".into(),
                value: JsonValue::Array { items: vec![JsonValue::String { value: "a".into() }, JsonValue::String { value: "b".into() }, JsonValue::String { value: "c".into() }] },
            },
            JsonMember {
                key: "nested".into(),
                value: JsonValue::Object { members: vec![JsonMember {
                    key: "deep".into(),
                    value: JsonValue::Object { members: vec![JsonMember {
                        key: "deeper".into(),
                        value: JsonValue::Array { items: vec![JsonValue::Number { lexeme: "1".into() }, JsonValue::Number { lexeme: "2".into() }, JsonValue::Number { lexeme: "3".into() }] },
                    }] },
                }] },
            },
        ],
    };
    JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value }
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Register
/// 🗂️ Registers codecs and the artifact schema descriptor.
pub fn register() {
    crate::artifacts::json::io_registry::register();
    register_artifact_schema();
    register_artifact_inferences();
    register_pilot_languages();
    store::register_document_codec(store::ArtifactCodec::of::<JsonSnapshot, JsonMutation>(STDIO_JSON_DOCUMENT_SCHEMA));
    // 🛡️ D5's generic validate-on-build hook: registers the ✳️i-json subset's SubsetValidator so
    // the wire-level `io_dispatch`/`wire_artifact_compose` hook can re-check it. The ComposerEntry
    // itself is registered separately via this standard's own `composer::entries()` aggregation.
    crate::artifacts::json::standards::v_rfc8259::subsets::i_json::io::register();
}

/// 📌️ P2-P1: 5-role `LanguageSpec` registration (Document/Ops/Diff/Pack/Spr), per note's exemplar
/// pattern (`✏️s/🔌️plugins/🗒️note/…/⚙️engine/🦀️component.rs`'s `register_pilot_languages`) —
/// `stdio.json`/`.op`/`.diff`/`.pack`/`.spr`, all `dsl::passthrough_hooks`. `diff`'s `protocol` slot
/// stays `None` matching the exemplar's own shape exactly (the role scheme has no dedicated "diff
/// binary" role even though `🔺️diff/💾️binary/📡️component.protocol.semio` is a real, conformance-
/// tested file — its binary form is exercised directly by `protocol_walk_law` below, just not
/// wired through a 6th `LanguageRole`).
///
/// `register_schema_spec` (P2-M3's `FullResolver` insertion API) is deliberately NOT called here —
/// see this wave's report `mechanism_gaps`: it requires `fn() -> RecordSpec`, and `stdio.json` has
/// no derivable `RecordSpec` by design (`JsonSnapshot`'s `ArtifactDsl`/`ArtifactPack` are hand-rolled
/// because `JsonValue` is a data-carrying recursive enum with no `DslField` impl — same root cause
/// that blocks `#[derive(dsl::DslArtifact)]` here, see `📸️snapshot/🦀️component.rs`'s own doc
/// comment). Fabricating an unrelated `RecordSpec` just to satisfy the call would be dishonest.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.json",
        extension: Some("json"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::json::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::json::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::json::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::json::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.json"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.json.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::json::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::json::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::json::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::json::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.json.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.json.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::json::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::json::schema::diff::text::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("stdio.json.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.json.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::json::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::json::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.json.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.json.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::json::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::json::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.json.spr"),
    });
}

/// 📌️ Registers schema leaves for `s.stdio.json`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::json::schema::json_artifact_schema_descriptor());
}

/// 💡️ Registers `s.stdio.json.inference`'s facet leaves into the OS-wide inference catalog —
/// sibling to `register_artifact_schema()` (separate registry, ticket
/// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
pub fn register_artifact_inferences() {
    ::schema::register_artifact_inference_descriptor(crate::artifacts::json::standards::v_rfc8259::subsets::any::schema::inferences::json_artifact_inference_descriptor());
}
//#endregion 🔖️Register

//#region 🔖️ArtifactEngine
/// ⚙️ `stdio.json` artifact engine.
pub struct JsonEngine {
    artifact_state: JsonArtifact,
    snapshot_state: JsonSnapshot,
}

impl JsonEngine {
    /// 🏗️ Builds an engine from a persisted snapshot.
    pub fn new(snapshot: JsonSnapshot) -> Self {
        let artifact_state = JsonArtifact::from_snapshot(snapshot.clone());
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
        let snapshot = empty_json_snapshot();
        assert_eq!(snapshot.schema, STDIO_JSON_DOCUMENT_SCHEMA);
    }

    #[test]
    fn codec_round_trip() {
        let snap = empty_json_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <JsonSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.schema, snap.schema);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <JsonSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }

    #[test]
    fn nontrivial_nested_value_round_trip() {
        let snap = demo_json_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <JsonSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.value, snap.value);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <JsonSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded.value, snap.value);
    }

    //#region 🔖️ConformanceLaws
    /// 🧪️ P2-P1: per-artifact conformance laws (item 6 of the deliverable list) — grammar/protocol
    /// parseability, `Recognizer` against real fixtures AND real `print_op`/`print_diff` output,
    /// `walk_protocol` against real `encode_pack`/`encode_op`/`encode_diff` bytes, and the
    /// fixture-honesty round-trip. Lives here (the engine's own test region), not any framework
    /// file — `m5` auto-discovers the snapshot grammar+`.dsl.semio`/protocol+`.pack.semio` pairs
    /// independently (`🧪️fixture-sweep/🦀️component.rs`'s `m5_auto_discovery`); these tests are this
    /// artifact's OWN early-warning, plus direct coverage of the mutations/diff facets that harness
    /// does not auto-discover at all.
    mod conformance_laws {
        use super::*;
        use crate::artifacts::json::schema::{diff, mutations, snapshot};
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

        /// ✅️ `grammar_conformance_law`: the snapshot grammar recognizes real `print_dsl` output for
        /// the demo (genuinely 3-level-nested) snapshot — same preamble-stripped body reconstruction
        /// `m5_handcrafted_grammar_conformance`'s own `dsl_body_from_fixture` uses, so this is a
        /// direct proof this artifact will pass that harness once graduated, not merely an analogue.
        #[test]
        fn grammar_conformance_law() {
            let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            let text = store::ArtifactDsl::print_dsl(&demo_json_snapshot());
            let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
            let reconstructed = format!("{}\n{body}", envelope.envelope_id());
            assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
        }

        /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op` output
        /// for every `JsonMutation` variant (`mutations::demo_mutation_cases()`), not just one
        /// trivial case — incl. nested array/object payload values and a mixed `Key`/`Index` path.
        #[test]
        fn ops_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for mutation in mutations::demo_mutation_cases() {
                let printed = mutation.print_op();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
            }
        }

        /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff` output
        /// for every representative `JsonDiff` (`diff::demo_diff_cases()`), incl. the empty-line
        /// (`None`) diff and the `Replace` kind-change fallback.
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
        /// `m5_handcrafted_protocol_conformance` itself feeds `walk_protocol`), every demo mutation's
        /// `encode_op`, and every demo diff's `encode_diff` — asserting `consumed == bytes.len()`.
        #[test]
        fn protocol_walk_law() {
            let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
            let packed = store::ArtifactPack::encode_pack(&demo_json_snapshot());
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
        /// `print_dsl`/`encode_pack` output of `demo_json_snapshot()` — `parse_dsl(fixture) ==
        /// demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the pack twin — so the
        /// fixtures can never silently drift back to a fake again.
        #[test]
        fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio");

            let demo = demo_json_snapshot();

            let parsed = <JsonSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
            assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_json_snapshot()");
            assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_json_snapshot()) drifted from the shipped .dsl.semio fixture");

            let decoded = <JsonSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
            assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_json_snapshot()");
            assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_json_snapshot()) drifted from the shipped .pack.semio fixture");
        }
    }
    //#endregion 🔖️ConformanceLaws
}
//#endregion 🧪️Tests
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, composer_entry_of};
    use crate::artifacts::json::standards::v_rfc8259::subsets::any::schema::JsonComposer as JsonRawAnyComposer;
    use crate::artifacts::json::standards::v_rfc8259::subsets::i_json::schema::JsonIJsonComposer;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<JsonRawAnyComposer>(), composer_entry_of::<JsonIJsonComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
