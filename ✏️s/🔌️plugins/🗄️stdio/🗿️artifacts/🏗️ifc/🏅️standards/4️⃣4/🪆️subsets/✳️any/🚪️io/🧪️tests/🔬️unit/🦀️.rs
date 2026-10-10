use crate::standards::v4::engine::{demo_ifc_snapshot, empty_ifc_snapshot};
use crate::IfcSnapshot;
use crate::STDIO_IFC_DOCUMENT_SCHEMA;

#[semio_framework_async_macros::async_test]
async fn empty_snapshot_matches_schema() {
    let snapshot = empty_ifc_snapshot();
    assert_eq!(snapshot.schema, STDIO_IFC_DOCUMENT_SCHEMA);
}

#[semio_framework_async_macros::async_test]
async fn codec_round_trip() {
    let snap = empty_ifc_snapshot();
    let text = store::ArtifactDsl::print_dsl(&snap);
    let parsed = <IfcSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(parsed.schema, snap.schema);
    let bytes = store::ArtifactPack::encode_pack(&snap);
    let decoded = <IfcSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(decoded, snap);
}

//#region 🔖️ConformanceLaws
/// 🧪️ P2-FG1: per-artifact conformance laws — grammar/protocol parseability, `Recognizer`
/// against real fixtures AND real `print_op`/`print_diff` output, `walk_protocol` against real
/// `encode_pack`/`encode_op`/`encode_diff` bytes, and the fixture-honesty round-trip. Dissolved
/// out of `⚙️engine`'s own test region (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES)
/// — same shape as every P1-P3 pilot's own `conformance_laws` module and the sibling STEP
/// artifact's own.
mod conformance_laws {
    use super::*;
    use crate::schema::{diff, mutations, snapshot};
    use protocol::{DiffBinary,DiffCodec,DiffText, OpBinary, OpText};

    /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
    /// parse under the real dialect.
    #[semio_framework_async_macros::async_test]
    async fn committed_facet_files_parse() {
        for (label, text) in [("snapshot grammar", crate::standards::v4::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO), ("mutations grammar", crate::standards::v4::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO), ("diff grammar", crate::standards::v4::subsets::any::io::text::diff::COMPONENT_GRAMMAR_SEMIO)] {
            let grammar = semio_framework_dsl::parse_grammar(text).unwrap_or_else(|e| panic!("{label}: parse_grammar failed: {e:?}"));
            assert_eq!(grammar.dialect, semio_framework_dsl::SemioDialect::Grammar, "{label}: expected grammar dialect");
        }
        for (label, text) in [("snapshot protocol", crate::standards::v4::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO), ("mutations protocol", crate::standards::v4::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO), ("diff protocol", crate::standards::v4::subsets::any::io::binary::diff::COMPONENT_PROTOCOL_SEMIO)] {
            semio_framework_dsl::parse_protocol(text).unwrap_or_else(|e| panic!("{label}: parse_protocol failed: {e:?}"));
        }
    }

    /// ✅️ `grammar_conformance_law`: the snapshot grammar recognizes real `print_dsl` output
    /// for the demo snapshot, preamble-stripped-and-reconstructed the same way
    /// `m5_handcrafted_grammar_conformance` itself does.
    #[semio_framework_async_macros::async_test]
    async fn grammar_conformance_law() {
        let grammar = semio_framework_dsl::parse_grammar(crate::standards::v4::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
        let recognizer = semio_framework_dsl::Recognizer::compile(&grammar, &semio_framework_os_kernel::os_dsl::grammar::family_fragments().expect("OS family grammar"), semio_framework_os_kernel::os_dsl::grammar::product_macros()).expect("selected grammar fragments");
        let text = store::ArtifactDsl::print_dsl(&demo_ifc_snapshot());
        let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
        let reconstructed = format!("{}\n{body}", envelope.envelope_id());
        assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");

        // 🔬️ Also the empty-entities case (`IfcSnapshot::default()`), exercising `instance*`'s
        // zero-width match and the empty-value-list optional group.
        let empty_text = store::ArtifactDsl::print_dsl(&empty_ifc_snapshot());
        let (empty_envelope, empty_body) = store::semio_format::split_text_preamble(&empty_text).expect("split preamble");
        let empty_reconstructed = format!("{}\n{empty_body}", empty_envelope.envelope_id());
        assert!(recognizer.recognize(&empty_reconstructed).expect("recognize"), "grammar did not recognize empty dsl body:\n{empty_reconstructed}");
    }

    #[test]
    fn grammar_and_native_codecs_retain_all_neutral_binary64_words() {
        let input: serde_json::Value = serde_json::from_str(include_str!("../../🪶️sqlite/📸️snapshot/🧫️fixtures/🔣️.json")).expect("neutral IFC4 scalar words");
        let words: Vec<u64> = input["binary64Words"].as_array().expect("binary64 words").iter().map(|word| u64::from_str_radix(word.as_str().expect("word"), 16).expect("hex word")).collect();
        let mut value = demo_ifc_snapshot();
        value.entities[0].args = words.iter().map(|word| snapshot::IfcValue::Real(f64::from_bits(*word))).collect();
        let grammar = semio_framework_dsl::parse_grammar(crate::standards::v4::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO).expect("IFC4 native grammar");
        let recognizer = semio_framework_dsl::Recognizer::compile(&grammar, &semio_framework_os_kernel::os_dsl::grammar::family_fragments().expect("OS grammar"), semio_framework_os_kernel::os_dsl::grammar::product_macros()).expect("IFC4 grammar fragments");
        let text = store::ArtifactDsl::print_dsl(&value);
        let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("native envelope");
        assert!(recognizer.recognize(&format!("{}\n{body}", envelope.envelope_id())).expect("native word grammar"));
        let decoded = [<IfcSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("native scalar text"), <IfcSnapshot as store::ArtifactPack>::decode_pack(&store::ArtifactPack::encode_pack(&value)).expect("native scalar Pack")];
        for restored in decoded {
            assert_eq!(restored.entities[0].args.iter().map(|value| match value { snapshot::IfcValue::Real(value) => value.to_bits(), _ => panic!("IFC4 real became another variant") }).collect::<Vec<_>>(), words);
        }
    }

    /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
    /// output for every `IfcMutation` demo case.
    #[semio_framework_async_macros::async_test]
    async fn ops_grammar_conformance_law() {
        let grammar = semio_framework_dsl::parse_grammar(crate::standards::v4::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
        let recognizer = semio_framework_dsl::Recognizer::compile(&grammar, &semio_framework_os_kernel::os_dsl::grammar::family_fragments().expect("OS family grammar"), semio_framework_os_kernel::os_dsl::grammar::product_macros()).expect("selected grammar fragments");
        for mutation in mutations::demo_mutation_cases() {
            let printed = mutation.print_op();
            assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
        }
    }

    /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff` output
    /// for every representative `IfcDiff` demo case.
    #[semio_framework_async_macros::async_test]
    async fn diff_grammar_conformance_law() {
        let grammar = semio_framework_dsl::parse_grammar(crate::standards::v4::subsets::any::io::text::diff::COMPONENT_GRAMMAR_SEMIO).expect("parse diff grammar");
        let recognizer = semio_framework_dsl::Recognizer::compile(&grammar, &semio_framework_os_kernel::os_dsl::grammar::family_fragments().expect("OS family grammar"), semio_framework_os_kernel::os_dsl::grammar::product_macros()).expect("selected grammar fragments");
        for d in diff::demo_diff_cases() {
            let printed = d.print_diff();
            assert!(recognizer.recognize(&printed).unwrap_or(false), "diff grammar did not recognize {printed:?} (from {d:?})");
        }
    }

    /// ✅️ `protocol_walk_law`: `walk_protocol` against REAL bytes for all three facets —
    /// snapshot pack (envelope-unwrapped first), every demo mutation's `encode_op`, every demo
    /// diff's `encode_diff` — asserting `consumed == bytes.len()`.
    #[semio_framework_async_macros::async_test]
    async fn protocol_walk_law() {
        let pack_spec = semio_framework_dsl::parse_protocol(crate::standards::v4::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
        let packed = store::ArtifactPack::encode_pack(&demo_ifc_snapshot());
        let (_, inner) = store::semio_format::unwrap_binary(&packed).expect("unwrap semio envelope");
        let trace = semio_framework_dsl::walk_protocol(&pack_spec, &inner).unwrap_or_else(|e| panic!("walk_protocol(pack) failed @{}: {}", e.offset, e.message));
        assert_eq!(trace.consumed, inner.len(), "pack walk did not consume every byte");

        let op_spec = semio_framework_dsl::parse_protocol(crate::standards::v4::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO).expect("parse mutations protocol");
        for mutation in mutations::demo_mutation_cases() {
            let bytes = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed for {mutation:?}: {e:?}"));
            let trace = semio_framework_dsl::walk_protocol(&op_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(op) failed for {mutation:?} @{}: {}", e.offset, e.message));
            assert_eq!(trace.consumed, bytes.len(), "op walk did not consume every byte for {mutation:?}");
        }

        let diff_spec = semio_framework_dsl::parse_protocol(crate::standards::v4::subsets::any::io::binary::diff::COMPONENT_PROTOCOL_SEMIO).expect("parse diff protocol");
        for d in diff::demo_diff_cases() {
            let bytes = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed for {d:?}: {e:?}"));
            let trace = semio_framework_dsl::walk_protocol(&diff_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(diff) failed for {d:?} @{}: {}", e.offset, e.message));
            assert_eq!(trace.consumed, bytes.len(), "diff walk did not consume every byte for {d:?}");
        }
    }

    /// ✅️ `fixture_honesty_law`: the shipped `.dsl.semio`/`.pack.semio` fixtures are GENUINE
    /// `print_dsl`/`encode_pack` output of `demo_ifc_snapshot()`.
    #[semio_framework_async_macros::async_test]
    async fn fixture_honesty_law() {
        const FIXTURE_DSL: &str = include_str!("../../../🧫️fixtures/🎬️demo/🗣️.dsl.semio");
        const FIXTURE_PACK: &[u8] = include_bytes!("../../../🧫️fixtures/🎬️demo/🎒️.pack.semio");

        let demo = demo_ifc_snapshot();

        let parsed = <IfcSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
        assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_ifc_snapshot()");
        assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_ifc_snapshot()) drifted from the shipped .dsl.semio fixture");

        let decoded = <IfcSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
        assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_ifc_snapshot()");
        assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_ifc_snapshot()) drifted from the shipped .pack.semio fixture");
    }
}
//#endregion 🔖️ConformanceLaws

#[test]
fn the_ifc4_document_codec_round_trips_an_ifc4_file_and_refuses_other_schemas() {
    use crate::standards::v4::engine::{decode_ifc4_document, encode_ifc4_document};
    let ifc4 = b"ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((\x27ViewDefinition []\x27),\x272;1\x27);\nFILE_NAME(\x27a.ifc\x27,\x27\x27,(\x27\x27),(\x27\x27),\x27\x27,\x27\x27,\x27\x27);\nFILE_SCHEMA((\x27IFC4\x27));\nENDSEC;\nDATA;\n#1=IFCCARTESIANPOINT((0.,0.,0.));\nENDSEC;\nEND-ISO-10303-21;\n";
    let document = decode_ifc4_document(ifc4).expect("an IFC4 file decodes");
    assert_eq!(document.instances.len(), 1);
    let again = encode_ifc4_document(&document).expect("it encodes");
    assert_eq!(decode_ifc4_document(&again).expect("and decodes again").instances, document.instances);
    let other = String::from_utf8_lossy(ifc4).replace("\x27IFC4\x27", "\x27IFC2X3\x27");
    assert!(decode_ifc4_document(other.as_bytes()).is_err(), "an IFC2X3 file is refused");
    let mut foreign = document.clone();
    foreign.header.file_schema = Vec::new();
    assert!(encode_ifc4_document(&foreign).is_err(), "a header without IFC4 is refused");
    let mut twice = document;
    twice.instances.push(twice.instances[0].clone());
    assert!(encode_ifc4_document(&twice).is_err(), "a repeated instance id is refused");
}
