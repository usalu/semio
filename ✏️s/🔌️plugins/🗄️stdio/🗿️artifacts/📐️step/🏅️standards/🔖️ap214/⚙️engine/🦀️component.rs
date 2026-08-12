//! ⚙️ StepEngine — owns a real `StepArtifact`.

use crate::artifacts::step::{StepArtifact, StepMutation, StepSnapshot, STDIO_STEP_DOCUMENT_SCHEMA};

//#region 🔖️Submodules
/// 📐 Shared ISO 10303-21 tokenizer + generic graph — public, importable cross-artifact (ifc reuses it).
#[path = "📐️part21/🦀️component.rs"]
pub mod part21;
/// 🧱 BrepMesh analyzer view, derived from the generic graph — never persisted itself.
#[path = "🧱️brep/🦀️component.rs"]
pub mod brep;
/// 🪜 Shared CC ladder classification + FILE_SCHEMA/PRODUCT-chain scans, reused by all six
/// `✳️ccN` subset analyzers (ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES).
#[path = "🪜️ladder/🦀️component.rs"]
pub mod ladder;
//#endregion 🔖️Submodules

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot.
pub fn empty_step_snapshot() -> StepSnapshot {
    StepSnapshot::default()
}

/// 📄️ P2-FG1: the demo `stdio.step` document — a real, minimal AP214 exchange structure (typed
/// HEADER triple + two `CARTESIAN_POINT` entities). The single source of truth for
/// `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`/`🎒️example.pack.semio` (both are literally
/// this snapshot's `print_dsl`/`encode_pack` output, asserted equal by `fixture_honesty_law`) and
/// for `mutations::demo_mutation_cases()`/`diff::demo_diff_cases()`.
pub fn demo_step_snapshot() -> StepSnapshot {
    use crate::artifacts::step::schema::snapshot::{StepEntity, StepFileDescription, StepFileName, StepFileSchema, StepHeader, StepValue};
    StepSnapshot {
        schema: STDIO_STEP_DOCUMENT_SCHEMA.into(),
        header: StepHeader {
            file_description: StepFileDescription { description: vec!["".into()], implementation_level: "2;1".into() },
            file_name: StepFileName {
                name: "semio.step".into(),
                timestamp: "2026-08-11T00:00:00".into(),
                author: vec!["Ueli".into()],
                organization: vec!["semio".into()],
                preprocessor_version: "semio".into(),
                originating_system: "".into(),
                authorization: "".into(),
            },
            file_schema: StepFileSchema { schemas: vec!["AUTOMOTIVE_DESIGN".into()] },
        },
        entities: vec![
            StepEntity {
                id: 1,
                name: "CARTESIAN_POINT".into(),
                args: vec![StepValue::String("".into()), StepValue::Aggregate(vec![StepValue::Real(0.0), StepValue::Real(0.0), StepValue::Real(0.0)])],
                complex: Vec::new(),
            },
            StepEntity {
                id: 2,
                name: "CARTESIAN_POINT".into(),
                args: vec![StepValue::String("".into()), StepValue::Aggregate(vec![StepValue::Real(10.0), StepValue::Real(0.0), StepValue::Real(0.0)])],
                complex: Vec::new(),
            },
        ],
    }
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Register
/// 🗂️ Registers codecs and the artifact schema descriptor.
pub fn register() {
    crate::artifacts::step::io_registry::register();
    register_artifact_schema();
    register_pilot_languages();
    register_subset_validators();
    store::register_document_codec(store::ArtifactCodec::of::<StepSnapshot, StepMutation>(STDIO_STEP_DOCUMENT_SCHEMA));
}

/// 📌️ Registers the `SubsetValidator` of every real (non-`✳️any`) ap214 subset — the six ISO
/// 10303-214 conformance classes (ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES).
pub fn register_subset_validators() {
    crate::artifacts::step::standards::v_ap214::subsets::cc1::io::register();
    crate::artifacts::step::standards::v_ap214::subsets::cc2::io::register();
    crate::artifacts::step::standards::v_ap214::subsets::cc3::io::register();
    crate::artifacts::step::standards::v_ap214::subsets::cc4::io::register();
    crate::artifacts::step::standards::v_ap214::subsets::cc5::io::register();
    crate::artifacts::step::standards::v_ap214::subsets::cc6::io::register();
}

/// 📌️ P2-FG1: 5-role `LanguageSpec` registration (Document/Ops/Diff/Pack/Spr), per the recipe's
/// json exemplar — `stdio.step`/`.op`/`.diff`/`.pack`/`.spr`, all `dsl::passthrough_hooks`. `diff`'s
/// `protocol` slot stays `None` matching the exemplar's own shape exactly (the 5-role scheme has no
/// dedicated "diff binary" role even though `🔺️diff/💾️binary/📡️component.protocol.semio` is a
/// real, conformance-tested file — its binary form is exercised directly by `protocol_walk_law`
/// below, just not wired through a 6th `LanguageRole`).
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.step",
        extension: Some("step"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::step::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::step::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::step::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::step::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.step"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.step.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::step::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::step::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::step::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::step::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.step.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.step.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::step::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::step::schema::diff::text::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("stdio.step.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.step.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::step::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::step::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.step.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.step.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::step::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::step::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.step.spr"),
    });
}

/// 📌️ P2-FG1: `dsl::registry::register_schema_spec` is intentionally NOT called here — `StepValue`
/// (a genuine data-carrying enum) has no `DslField` impl, so no `fn() -> RecordSpec` exists for
/// `StepSnapshot`/`StepDiff` at all (real `cargo check` confirmed, see `🔺️diff/🦀️component.rs`'s own
/// doc comment) — filed as the `register-schema-spec-needs-recordspec` mechanism gap rather than
/// fabricating an unrelated spec, per the recipe's own instruction.

/// 📌️ Registers schema leaves for `s.stdio.step`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::step::schema::step_artifact_schema_descriptor());
}
//#endregion 🔖️Register

//#region 🔖️ArtifactEngine
/// ⚙️ `stdio.step` artifact engine.
pub struct StepEngine {
    artifact_state: StepArtifact,
    snapshot_state: StepSnapshot,
}

impl StepEngine {
    /// 🏗️ Builds an engine from a persisted snapshot.
    pub fn new(snapshot: StepSnapshot) -> Self {
        let artifact_state = StepArtifact::from_snapshot(snapshot.clone());
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
        let snapshot = empty_step_snapshot();
        assert_eq!(snapshot.schema, STDIO_STEP_DOCUMENT_SCHEMA);
    }

    #[test]
    fn codec_round_trip() {
        let snap = empty_step_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <StepSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.schema, snap.schema);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <StepSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }

    //#region 🔖️ConformanceLaws
    /// 🧪️ P2-FG1: per-artifact conformance laws — grammar/protocol parseability, `Recognizer`
    /// against real fixtures AND real `print_op`/`print_diff` output, `walk_protocol` against real
    /// `encode_pack`/`encode_op`/`encode_diff` bytes, and the fixture-honesty round-trip. Lives here
    /// (the engine's own test region), never a framework file — same shape as every P1-P3 pilot's
    /// own `conformance_laws` module (`binary`'s own is the direct template this mirrors).
    mod conformance_laws {
        use super::*;
        use crate::artifacts::step::schema::{diff, mutations, snapshot};
        use protocol::{DiffCodec, OpBinary, OpText};

        /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
        /// parse under the real dialect.
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
        /// for the demo snapshot, preamble-stripped-and-reconstructed the same way
        /// `m5_handcrafted_grammar_conformance` itself does.
        #[test]
        fn grammar_conformance_law() {
            let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            let text = store::ArtifactDsl::print_dsl(&demo_step_snapshot());
            let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
            let reconstructed = format!("{}\n{body}", envelope.envelope_id());
            assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");

            // 🔬️ Also the empty-entities case (`StepSnapshot::default()`), exercising `instance*`'s
            // zero-width match and the empty-value-list optional group.
            let empty_text = store::ArtifactDsl::print_dsl(&empty_step_snapshot());
            let (empty_envelope, empty_body) = store::semio_format::split_text_preamble(&empty_text).expect("split preamble");
            let empty_reconstructed = format!("{}\n{empty_body}", empty_envelope.envelope_id());
            assert!(recognizer.recognize(&empty_reconstructed).expect("recognize"), "grammar did not recognize empty dsl body:\n{empty_reconstructed}");
        }

        /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
        /// output for every `StepMutation` demo case.
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
        /// for every representative `StepDiff` demo case.
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
        /// snapshot pack (envelope-unwrapped first), every demo mutation's `encode_op`, every demo
        /// diff's `encode_diff` — asserting `consumed == bytes.len()`.
        #[test]
        fn protocol_walk_law() {
            let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
            let packed = store::ArtifactPack::encode_pack(&demo_step_snapshot());
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
        /// `print_dsl`/`encode_pack` output of `demo_step_snapshot()`.
        #[test]
        fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../../../📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio");

            let demo = demo_step_snapshot();

            let parsed = <StepSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
            assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_step_snapshot()");
            assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_step_snapshot()) drifted from the shipped .dsl.semio fixture");

            let decoded = <StepSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
            assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_step_snapshot()");
            assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_step_snapshot()) drifted from the shipped .pack.semio fixture");
        }
    }
    //#endregion 🔖️ConformanceLaws
}
//#endregion 🧪️Tests
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, composer_entry_of};
    use crate::artifacts::step::standards::v_ap214::subsets::any::schema::StepComposer as StepRawAnyComposer;
    use crate::artifacts::step::standards::v_ap214::subsets::cc1::schema::StepCc1Composer;
    use crate::artifacts::step::standards::v_ap214::subsets::cc2::schema::StepCc2Composer;
    use crate::artifacts::step::standards::v_ap214::subsets::cc3::schema::StepCc3Composer;
    use crate::artifacts::step::standards::v_ap214::subsets::cc4::schema::StepCc4Composer;
    use crate::artifacts::step::standards::v_ap214::subsets::cc5::schema::StepCc5Composer;
    use crate::artifacts::step::standards::v_ap214::subsets::cc6::schema::StepCc6Composer;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| {
            vec![
                composer_entry_of::<StepRawAnyComposer>(),
                composer_entry_of::<StepCc1Composer>(),
                composer_entry_of::<StepCc2Composer>(),
                composer_entry_of::<StepCc3Composer>(),
                composer_entry_of::<StepCc4Composer>(),
                composer_entry_of::<StepCc5Composer>(),
                composer_entry_of::<StepCc6Composer>(),
            ]
        })
        .as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
