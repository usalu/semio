//! ⚙️ Ifc2x3Engine — real IFC2X3 SPF (ISO 10303-21 Part-21) decode/encode. IFC2X3 is buildingSMART
//! Coordination View 2.0-era IFC, ISO/PAS 16739:2005 schema, physically-encoded identically to
//! `📐️step`'s AP214 (`FILE_SCHEMA(('IFC2X3'))` in place of `FILE_SCHEMA(('AUTOMOTIVE_DESIGN'))`).
//! Reuses `step::engine::part21`'s tokenizer/writer functions directly (already `pub` — no
//! visibility change needed, unlike the ticket's contingency plan) — that is PARSING-CODE reuse,
//! explicitly allowed; what's NOT reused is `Part21Document`'s type IDENTITY as this standard's
//! snapshot type (see `🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`'s own doc comment).
//! This file owns the standard-specific layer parsing code alone can't provide: FILE_SCHEMA
//! validation (decode refuses non-IFC2X3 input) and the genuine round-trip test this ticket's law
//! requires (`POLICY_ROUND_TRIP_TEST_ALLOWLIST` is shrink-only — new standards never get added).

use crate::artifacts::step::engine::part21::{parse_part21, write_part21};
use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::mutations::Ifc2x3Mutation;
use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::{Ifc2x3Snapshot, STDIO_IFC2X3_DOCUMENT_SCHEMA};
use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::Ifc2x3Artifact;

//#region 🔖️Codec
/// 📐️ The IFC2X3 FILE_SCHEMA name a conforming Part-21 file must declare.
pub const IFC2X3_SCHEMA_NAME: &str = "IFC2X3";

/// 📥️ Decodes IFC2X3 SPF bytes into an [`Ifc2x3Snapshot`]. Real standard-specific validation
/// beyond generic Part-21 parsing: rejects any file whose `FILE_SCHEMA` doesn't declare
/// `IFC2X3` (so this decoder never silently accepts an IFC4 or plain STEP AP214 file).
pub fn decode_ifc2x3(bytes: &[u8]) -> Result<Ifc2x3Snapshot, String> {
    let text = std::str::from_utf8(bytes).map_err(|e| format!("ifc2x3: not valid utf-8: {e}"))?;
    let document = parse_part21(text).map_err(|e| format!("ifc2x3 parse: {e}"))?;
    let declares_ifc2x3 = document.header.file_schema.iter().any(|v| {
        v.as_list()
            .map(|items| items.iter().any(|item| item.as_str() == Some(IFC2X3_SCHEMA_NAME)))
            .unwrap_or(false)
    });
    if !declares_ifc2x3 {
        return Err(format!("ifc2x3: FILE_SCHEMA does not declare {IFC2X3_SCHEMA_NAME}"));
    }
    Ok(Ifc2x3Snapshot { schema: STDIO_IFC2X3_DOCUMENT_SCHEMA.into(), document })
}

/// 📤️ Regenerates valid IFC2X3 SPF bytes from a snapshot. Losslessness is `write_part21`'s job
/// (shared with `step`/`4`); this function's only own contribution is the byte encoding.
pub fn encode_ifc2x3(snapshot: &Ifc2x3Snapshot) -> Result<Vec<u8>, String> {
    Ok(write_part21(&snapshot.document).into_bytes())
}

pub fn empty_ifc2x3_snapshot() -> Ifc2x3Snapshot {
    Ifc2x3Snapshot::default()
}

/// 📄️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: the demo
/// `stdio.ifc.2x3` document — a real, minimal IFC2X3 exchange structure (raw HEADER value tuples +
/// two real entities incl. an `IFCOWNERHISTORY` reference chain), matching `4`'s own
/// `demo_ifc_snapshot()` shape but declaring `FILE_SCHEMA(('IFC2X3'))` so `decode_ifc2x3`'s own
/// schema gate accepts it. Fodder for `mutations::demo_mutation_cases()`/`diff::demo_diff_cases()`
/// and this module's own `conformance_laws` tests (a non-empty snapshot, unlike the prior
/// `empty_ifc2x3_snapshot()` stub, so every recognizer/walk law actually exercises real content).
pub fn demo_ifc2x3_snapshot() -> Ifc2x3Snapshot {
    use crate::artifacts::step::engine::part21::{Part21Document, Part21Header, Part21Instance, Part21Value};
    Ifc2x3Snapshot {
        schema: STDIO_IFC2X3_DOCUMENT_SCHEMA.into(),
        document: Part21Document {
            header: Part21Header {
                file_description: vec![Part21Value::List(vec![]), Part21Value::Str("2;1".into())],
                file_name: vec![
                    Part21Value::Str("semio.ifc".into()),
                    Part21Value::Str("2026-08-11T00:00:00".into()),
                    Part21Value::List(vec![Part21Value::Str("Ueli".into())]),
                    Part21Value::List(vec![Part21Value::Str("semio".into())]),
                    Part21Value::Str("semio".into()),
                    Part21Value::Str("".into()),
                    Part21Value::Str("".into()),
                ],
                file_schema: vec![Part21Value::List(vec![Part21Value::Str("IFC2X3".into())])],
            },
            instances: vec![
                Part21Instance { id: 1, entities: vec![("IFCPROJECT".into(), vec![Part21Value::Str("gid-project".into()), Part21Value::Ref(2), Part21Value::Str("Demo Project".into())])] },
                Part21Instance { id: 2, entities: vec![("IFCOWNERHISTORY".into(), vec![Part21Value::Unset, Part21Value::Int(0)])] },
            ],
        },
    }
}
//#endregion 🔖️Codec

//#region 🔖️Register
/// 🗂️ Registers this standard's schema descriptor, document codec, 5-role `LanguageSpec`s, and (via
/// each real subset's own composer) its `SubsetValidator`s. Does NOT call the artifact-level
/// `ifc::composer::register()` (that union is already invoked once from `4`'s own
/// `engine::register()`, extended by this ticket to also union `v2x3::composer::entries()` —
/// calling it a second time here would be a redundant registration, same reasoning gif's
/// `89a::engine::register` doc comment gives).
pub fn register() {
    ::schema::register_artifact_schema_descriptor(
        crate::artifacts::ifc::standards::v2x3::subsets::any::schema::ifc2x3_artifact_schema_descriptor(),
    );
    register_pilot_languages();
    store::register_document_codec(store::ArtifactCodec::of::<Ifc2x3Snapshot, Ifc2x3Mutation>(STDIO_IFC2X3_DOCUMENT_SCHEMA));
    // 🛡️ D5's generic validate-on-build hook: registers each real subset's `SubsetValidator` so
    // `io_dispatch`/`wire_artifact_compose` re-check them for free. Each subset's `ComposerEntry`
    // is registered separately via this standard's own `composer::entries()` aggregation.
    crate::artifacts::ifc::standards::v2x3::subsets::cv20::composer::register();
    crate::artifacts::ifc::standards::v2x3::subsets::sav::composer::register();
    crate::artifacts::ifc::standards::v2x3::subsets::cobie::composer::register();
}

/// 📌️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: 5-role
/// `LanguageSpec` registration (Document/Ops/Diff/Pack/Spr), per the recipe's json exemplar —
/// `stdio.ifc.2x3`/`.op`/`.diff`/`.pack`/`.spr`, all `dsl::passthrough_hooks`. `diff`'s `protocol`
/// slot stays `None` matching the exemplar's own shape exactly (the 5-role scheme has no dedicated
/// "diff binary" role even though `🔺️diff/💾️binary/📡️component.protocol.semio` is a real,
/// conformance-tested file — its binary form is exercised directly by `protocol_walk_law` below,
/// just not wired through a 6th `LanguageRole`), same precedent `4`'s own
/// `register_pilot_languages` established.
pub fn register_pilot_languages() {
    use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::{diff, mutations, snapshot};
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.ifc.2x3",
        extension: Some("ifc"),
        role: dsl::LanguageRole::Document,
        grammar: Some(snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.ifc.2x3"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.ifc.2x3.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(mutations::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(mutations::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.ifc.2x3.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.ifc.2x3.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(diff::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(diff::text::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("stdio.ifc.2x3.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.ifc.2x3.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.ifc.2x3.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.ifc.2x3.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.ifc.2x3.spr"),
    });
}

/// 📌️ `dsl::registry::register_schema_spec` is intentionally NOT called here — `Part21Value` (a
/// genuine data-carrying enum) has no `DslField` impl, so no `fn() -> RecordSpec` exists for
/// `Ifc2x3Snapshot`/`Ifc2x3Diff` at all (same `register-schema-spec-needs-recordspec` mechanism gap
/// `4`'s own `IfcSnapshot`/`IfcDiff` doc comment documents for the isomorphic shape) — filed as a
/// `mechanism_gaps` entry rather than fabricating an unrelated spec.
//#endregion 🔖️Register

//#region 🔖️ArtifactEngine
pub struct Ifc2x3Engine {
    artifact_state: Ifc2x3Artifact,
    snapshot_state: Ifc2x3Snapshot,
}

impl Ifc2x3Engine {
    pub fn new(snapshot: Ifc2x3Snapshot) -> Self {
        let artifact_state = Ifc2x3Artifact::from_snapshot(snapshot.clone());
        Self { artifact_state, snapshot_state: snapshot }
    }
}
//#endregion 🔖️ArtifactEngine

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    const IFC2X3_FIXTURE: &str = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('semio.ifc','2026-08-11T00:00:00',('Ueli'),('semio'),'semio','','');\nFILE_SCHEMA(('IFC2X3'));\nENDSEC;\nDATA;\n#1=IFCPROJECT('0YvctVUKr0kugbFTf53O9L',$,'Project',$,$,$,$,(#20),#30);\n#20=IFCUNITASSIGNMENT((#21));\n#21=IFCSIUNIT(*,.LENGTHUNIT.,$,.METRE.);\n#30=IFCGEOMETRICREPRESENTATIONCONTEXT($,'Model',3,1.E-05,#31,$);\n#31=IFCAXIS2PLACEMENT3D(#32,$,$);\n#32=IFCCARTESIANPOINT((0.,0.,0.));\n#40=IFCBUILDING('0YvctVUKr0kugbFTf53O9M',$,'Building',$,$,#41,$,$,.ELEMENT.,$,$,$);\n#41=IFCLOCALPLACEMENT($,#31);\nENDSEC;\nEND-ISO-10303-21;\n";

    #[test]
    fn decode_rejects_non_ifc2x3_schema() {
        let step_ap214 = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('AUTOMOTIVE_DESIGN'));\nENDSEC;\nDATA;\nENDSEC;\nEND-ISO-10303-21;\n";
        assert!(decode_ifc2x3(step_ap214.as_bytes()).is_err(), "must reject a non-IFC2X3 FILE_SCHEMA");
        let ifc4 = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('IFC4'));\nENDSEC;\nDATA;\nENDSEC;\nEND-ISO-10303-21;\n";
        assert!(decode_ifc2x3(ifc4.as_bytes()).is_err(), "must reject IFC4 too -- 2x3 is a distinct standard, not a superset reader");
    }

    /// 🧪️ THE genuine decode→encode→decode round-trip law this ticket's own policy requires
    /// (`POLICY_ROUND_TRIP_TEST_ALLOWLIST` is shrink-only for new standards).
    #[test]
    fn decode_encode_decode_round_trip_is_lossless() {
        let once = decode_ifc2x3(IFC2X3_FIXTURE.as_bytes()).expect("decode fixture");
        // 🩹 8 distinct instance ids in IFC2X3_FIXTURE: #1, #20, #21, #30, #31, #32, #40, #41.
        assert_eq!(once.document.instances.len(), 8);
        assert!(once.document.by_type("IFCPROJECT").next().is_some());
        let bytes = encode_ifc2x3(&once).expect("encode");
        let twice = decode_ifc2x3(&bytes).expect("decode re-encoded bytes");
        assert_eq!(once, twice, "decode -> encode -> decode must be lossless at the snapshot level");
    }

    #[test]
    fn empty_snapshot_matches_schema() {
        let snapshot = empty_ifc2x3_snapshot();
        assert_eq!(snapshot.schema, STDIO_IFC2X3_DOCUMENT_SCHEMA);
    }

    #[test]
    fn codec_round_trip_through_store_traits() {
        let snap = decode_ifc2x3(IFC2X3_FIXTURE.as_bytes()).expect("decode");
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <Ifc2x3Snapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse_dsl");
        assert_eq!(parsed, snap);
        let packed = store::ArtifactPack::encode_pack(&snap);
        let decoded = <Ifc2x3Snapshot as store::ArtifactPack>::decode_pack(&packed).expect("decode_pack");
        assert_eq!(decoded, snap);
    }

    //#region 🔖️ConformanceLaws
    /// 🧪️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: per-artifact
    /// conformance laws — grammar/protocol parseability, `Recognizer` against real fixtures AND real
    /// `print_op`/`print_diff` output, `walk_protocol` against real `encode_pack`/`encode_op`/
    /// `encode_diff` bytes, and the fixture-honesty round-trip. Lives here (the engine's own test
    /// region), never a framework file — same shape as `4`'s own `conformance_laws` module and every
    /// P1-P3 pilot's own.
    mod conformance_laws {
        use super::*;
        use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::{diff, mutations, snapshot};
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

        /// ✅️ `grammar_conformance_law`: the snapshot grammar recognizes real `print_dsl` output for
        /// the demo snapshot AND the empty-instances case, preamble-stripped-and-reconstructed the
        /// same way `m5_handcrafted_grammar_conformance` itself does.
        #[test]
        fn grammar_conformance_law() {
            let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            let text = store::ArtifactDsl::print_dsl(&demo_ifc2x3_snapshot());
            let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
            let reconstructed = format!("{}\n{body}", envelope.envelope_id());
            assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");

            // 🔬️ Also the empty-instances case (`empty_ifc2x3_snapshot()`), exercising `instance*`'s
            // zero-width match and the empty-value-list optional group.
            let empty_text = store::ArtifactDsl::print_dsl(&empty_ifc2x3_snapshot());
            let (empty_envelope, empty_body) = store::semio_format::split_text_preamble(&empty_text).expect("split preamble");
            let empty_reconstructed = format!("{}\n{empty_body}", empty_envelope.envelope_id());
            assert!(recognizer.recognize(&empty_reconstructed).expect("recognize"), "grammar did not recognize empty dsl body:\n{empty_reconstructed}");
        }

        /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op` output
        /// for every `Ifc2x3Mutation` demo case.
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
        /// for every representative `Ifc2x3Diff` demo case.
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
            let packed = store::ArtifactPack::encode_pack(&demo_ifc2x3_snapshot());
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
        /// `print_dsl`/`encode_pack` output of `demo_ifc2x3_snapshot()`.
        #[test]
        fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio");

            let demo = demo_ifc2x3_snapshot();

            let parsed = <Ifc2x3Snapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
            assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_ifc2x3_snapshot()");
            assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_ifc2x3_snapshot()) drifted from the shipped .dsl.semio fixture");

            let decoded = <Ifc2x3Snapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
            assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_ifc2x3_snapshot()");
            assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_ifc2x3_snapshot()) drifted from the shipped .pack.semio fixture");
        }
    }
    //#endregion 🔖️ConformanceLaws
}
//#endregion 🧪️Tests
