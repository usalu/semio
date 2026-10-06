use super::*;
use crate::standards::v2x3::engine::{demo_ifc2x3_snapshot, empty_ifc2x3_snapshot};
use semio_framework_plugin::{AnalyzeSource, ArtifactAnalyzer, ArtifactComposition, ComposeSource, Dialect, StandardId, SubsetId};
use std::sync::OnceLock;

const IFC2X3_FIXTURE: &str = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('semio.ifc','2026-08-11T00:00:00',('Ueli'),('semio'),'semio','','');\nFILE_SCHEMA(('IFC2X3'));\nENDSEC;\nDATA;\n#1=IFCPROJECT('0YvctVUKr0kugbFTf53O9L',$,'Project',$,$,$,$,(#20),#30);\n#20=IFCUNITASSIGNMENT((#21));\n#21=IFCSIUNIT(*,.LENGTHUNIT.,$,.METRE.);\n#30=IFCGEOMETRICREPRESENTATIONCONTEXT($,'Model',3,1.E-05,#31,$);\n#31=IFCAXIS2PLACEMENT3D(#32,$,$);\n#32=IFCCARTESIANPOINT((0.,0.,0.));\n#40=IFCBUILDING('0YvctVUKr0kugbFTf53O9M',$,'Building',$,$,#41,$,$,.ELEMENT.,$,$,$);\n#41=IFCLOCALPLACEMENT($,#31);\nENDSEC;\nEND-ISO-10303-21;\n";

async fn exact_fixture_bytes() -> &'static [u8] {
    static BYTES: OnceLock<Vec<u8>> = OnceLock::new();
    BYTES.get_or_init(|| std::fs::read(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧫️fixtures/🏥️wellness-center-sama-street-level/🏥️wellness-center-sama-street-level.ifc")).expect("read temp/wellness-center-sama.ifc"))
}

async fn assert_exact(label: &str, actual: &[u8]) {
    let expected = exact_fixture_bytes().await;
    let first_difference = actual.iter().zip(expected).position(|(left, right)| left != right);
    assert!(actual == expected, "{label}: expected {} bytes, got {}; first differing byte: {first_difference:?}", expected.len(), actual.len(),);
}

#[semio_framework_async_macros::async_test]
async fn decode_rejects_non_ifc2x3_schema() {
    let step_ap214 = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('AUTOMOTIVE_DESIGN'));\nENDSEC;\nDATA;\nENDSEC;\nEND-ISO-10303-21;\n";
    assert!(decode_ifc2x3(step_ap214.as_bytes()).is_err(), "must reject a non-IFC2X3 FILE_SCHEMA");
    let ifc4 = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('IFC4'));\nENDSEC;\nDATA;\nENDSEC;\nEND-ISO-10303-21;\n";
    assert!(decode_ifc2x3(ifc4.as_bytes()).is_err(), "must reject IFC4 too -- 2x3 is a distinct standard, not a superset reader");
}

/// 🧪️ THE genuine decode→encode→decode round-trip law this ticket's own policy requires
/// (`POLICY_ROUND_TRIP_TEST_ALLOWLIST` is shrink-only for new standards).
#[semio_framework_async_macros::async_test]
async fn decode_encode_decode_round_trip_is_lossless() {
    let once = decode_ifc2x3(IFC2X3_FIXTURE.as_bytes()).expect("decode fixture");
    // 🩹 8 distinct instance ids in IFC2X3_FIXTURE: #1, #20, #21, #30, #31, #32, #40, #41.
    assert_eq!(once.document.instances.len(), 8);
    assert!(once.document.by_type("IFCPROJECT").next().is_some());
    let bytes = encode_ifc2x3(&once).expect("encode");
    let twice = decode_ifc2x3(&bytes).expect("decode re-encoded bytes");
    assert_eq!(once, twice, "decode -> encode -> decode must be lossless at the snapshot level");
}

#[semio_framework_async_macros::async_test]
async fn empty_snapshot_matches_schema() {
    let snapshot = empty_ifc2x3_snapshot();
    assert_eq!(snapshot.schema, STDIO_IFC2X3_DOCUMENT_SCHEMA);
}

#[semio_framework_async_macros::async_test]
async fn codec_round_trip_through_store_traits() {
    let snap = decode_ifc2x3(IFC2X3_FIXTURE.as_bytes()).expect("decode");
    let text = store::ArtifactDsl::print_dsl(&snap);
    let parsed = <Ifc2x3Snapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse_dsl");
    assert_eq!(parsed, snap);
    let packed = store::ArtifactPack::encode_pack(&snap);
    let decoded = <Ifc2x3Snapshot as store::ArtifactPack>::decode_pack(&packed).expect("decode_pack");
    assert_eq!(decoded, snap);
}

//#region 🔖️LosslessNativeRouting
#[semio_framework_async_macros::async_test]
async fn exact_native_engine_raw_serializers_analyzer_and_composer_roundtrip() {
    use crate::standards::v2x3::subsets::base::io::export::serializers::artifacts::{binary::v_raw::base as binary_export, txt::v_utf_8::base as text_export};
    use crate::standards::v2x3::subsets::base::io::import::deserializers::artifacts::{binary::v_raw::base as binary_import, txt::v_utf_8::base as text_import};
    use crate::standards::v2x3::subsets::base::io::Ifc2x3Analyzer;
    use semio_s_artifact_stdio_binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
    use semio_s_artifact_stdio_txt::TxtSnapshot;

    let original = exact_fixture_bytes().await;
    let imported = decode_ifc2x3(original).expect("direct IFC2X3 import");
    assert_eq!(imported.document.instances.len(), 3_464, "fixture entity count changed");
    let reexported = encode_ifc2x3(&imported).expect("direct engine export");
    let reimported = decode_ifc2x3(&reexported).expect("direct engine re-import");
    assert_eq!(reimported, imported, "direct engine export must be semantically lossless");

    let binary = BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes: original.to_vec() };
    let binary_snapshot = binary_import::deserialize(&binary).expect("raw binary deserialize");
    let binary_output = binary_export::serialize(&binary_snapshot).expect("raw binary serialize");
    let binary_round = decode_ifc2x3(&binary_output.bytes).expect("raw binary re-import");
    assert_eq!(binary_round, imported, "raw binary route must be semantically lossless");

    let text = std::str::from_utf8(original).expect("fixture UTF-8");
    let txt = TxtSnapshot::from_body(text);
    let text_snapshot = text_import::deserialize(&txt).expect("raw text deserialize");
    let text_output = text_export::serialize(&text_snapshot).expect("raw text serialize");
    let text_round = decode_ifc2x3(text_output.to_body().as_bytes()).expect("raw text re-import");
    assert_eq!(text_round, imported, "raw text route must be semantically lossless");

    let text_analysis = <Ifc2x3Analyzer as ArtifactAnalyzer>::analyze(&[AnalyzeSource::Text(text)]);
    assert!(text_analysis.diagnostics.is_empty(), "text analyzer diagnostics: {:?}", text_analysis.diagnostics);
    let text_analyzed = text_analysis.parts.snapshot.expect("text analyzer snapshot");
    assert_eq!(text_analyzed, imported, "text analyzer must recover the fixture snapshot");

    let pack = store::ArtifactPack::encode_pack(&imported);
    let pack_analysis = <Ifc2x3Analyzer as ArtifactAnalyzer>::analyze(&[AnalyzeSource::Binary(&pack)]);
    assert!(pack_analysis.diagnostics.is_empty(), "pack analyzer diagnostics: {:?}", pack_analysis.diagnostics);
    let pack_analyzed = pack_analysis.parts.snapshot.expect("pack analyzer snapshot");
    assert_eq!(pack_analyzed, imported, "pack analyzer must recover the fixture snapshot");
}

#[semio_framework_async_macros::async_test]
async fn snapshot_and_facets_forbid_native_shadow_state() {
    let value = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&(demo_ifc2x3_snapshot()))).expect("serialize logical snapshot");
    let object = value.as_object().expect("snapshot object");
    assert_eq!(object.keys().map(String::as_str).collect::<Vec<_>>(), vec!["document", "schema"]);
    for (relative, text) in [
        ("snapshot.proto", include_str!("../../../🧬️schema/📸️snapshot/🛰️.proto")),
        ("snapshot.graphql", include_str!("../../../🧬️schema/📸️snapshot/🔗️.graphql")),
        ("snapshot.ts", include_str!("../../../🧬️schema/📸️snapshot/🟦️.ts")),
        ("artifact.proto", include_str!("../../../🧬️schema/🛰️.proto")),
        ("artifact.graphql", include_str!("../../../🧬️schema/🔗️.graphql")),
        ("artifact.ts", include_str!("../../../🧬️schema/🟦️.ts")),
    ] {
        for forbidden in ["ArtifactSource", "physical", "lexical", "document_wire", "document: Bytes", "sourceBytes"] {
            assert!(!text.contains(forbidden), "{relative} contains forbidden shadow marker {forbidden}");
        }
        assert!(text.contains("Part21Document") || (relative == "artifact.ts" && text.contains("extends Ifc2x3Snapshot")), "{relative} must expose the typed Part21 document directly or through the owned snapshot");
    }
}
//#endregion 🔖️LosslessNativeRouting

//#region 🔖️ConformanceLaws
/// 🧪️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: per-artifact
/// conformance laws — grammar/protocol parseability, `Recognizer` against real fixtures AND real
/// `print_op`/`print_diff` output, `walk_protocol` against real `encode_pack`/`encode_op`/
/// `encode_diff` bytes, and the fixture-honesty round-trip. Dissolved out of `⚙️engine`'s own
/// test region (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — same shape as
/// `4`'s own `conformance_laws` module and every P1-P3 pilot's own.
mod conformance_laws {
    use super::*;
    use crate::standards::v2x3::subsets::base::schema::{diff, mutations, snapshot};
    use protocol::{DiffBinary,DiffCodec,DiffText, OpBinary, OpText};

    /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
    /// parse under the real dialect.
    #[semio_framework_async_macros::async_test]
    async fn committed_facet_files_parse() {
        for (label, text) in [("snapshot grammar", crate::standards::v2x3::subsets::base::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO), ("mutations grammar", crate::standards::v2x3::subsets::base::io::text::mutations::COMPONENT_GRAMMAR_SEMIO), ("diff grammar", crate::standards::v2x3::subsets::base::io::text::diff::COMPONENT_GRAMMAR_SEMIO)] {
            let grammar = semio_framework_dsl::parse_grammar(text).unwrap_or_else(|e| panic!("{label}: parse_grammar failed: {e:?}"));
            assert_eq!(grammar.dialect, semio_framework_dsl::SemioDialect::Grammar, "{label}: expected grammar dialect");
        }
        for (label, text) in [("snapshot protocol", crate::standards::v2x3::subsets::base::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO), ("mutations protocol", crate::standards::v2x3::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO), ("diff protocol", crate::standards::v2x3::subsets::base::io::binary::diff::COMPONENT_PROTOCOL_SEMIO)] {
            semio_framework_dsl::parse_protocol(text).unwrap_or_else(|e| panic!("{label}: parse_protocol failed: {e:?}"));
        }
    }

    /// ✅️ `grammar_conformance_law`: the snapshot grammar recognizes real `print_dsl` output for
    /// the demo snapshot AND the empty-instances case, preamble-stripped-and-reconstructed the
    /// same way `m5_handcrafted_grammar_conformance` itself does.
    #[semio_framework_async_macros::async_test]
    async fn grammar_conformance_law() {
        let grammar = semio_framework_dsl::parse_grammar(crate::standards::v2x3::subsets::base::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
        let recognizer = semio_framework_dsl::Recognizer::compile(&grammar, &semio_framework_os_kernel::os_dsl::grammar::family_fragments().expect("OS family grammar"), semio_framework_os_kernel::os_dsl::grammar::product_macros()).expect("selected grammar fragments");
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
    #[semio_framework_async_macros::async_test]
    async fn ops_grammar_conformance_law() {
        let grammar = semio_framework_dsl::parse_grammar(crate::standards::v2x3::subsets::base::io::text::mutations::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
        let recognizer = semio_framework_dsl::Recognizer::compile(&grammar, &semio_framework_os_kernel::os_dsl::grammar::family_fragments().expect("OS family grammar"), semio_framework_os_kernel::os_dsl::grammar::product_macros()).expect("selected grammar fragments");
        for mutation in mutations::demo_mutation_cases() {
            let printed = mutation.print_op();
            assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
        }
    }

    /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff` output
    /// for every representative `Ifc2x3Diff` demo case.
    #[semio_framework_async_macros::async_test]
    async fn diff_grammar_conformance_law() {
        let grammar = semio_framework_dsl::parse_grammar(crate::standards::v2x3::subsets::base::io::text::diff::COMPONENT_GRAMMAR_SEMIO).expect("parse diff grammar");
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
        let pack_spec = semio_framework_dsl::parse_protocol(crate::standards::v2x3::subsets::base::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
        let packed = store::ArtifactPack::encode_pack(&demo_ifc2x3_snapshot());
        let (_, inner) = store::semio_format::unwrap_binary(&packed).expect("unwrap semio envelope");
        let trace = semio_framework_dsl::walk_protocol(&pack_spec, &inner).unwrap_or_else(|e| panic!("walk_protocol(pack) failed @{}: {}", e.offset, e.message));
        assert_eq!(trace.consumed, inner.len(), "pack walk did not consume every byte");

        let op_spec = semio_framework_dsl::parse_protocol(crate::standards::v2x3::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO).expect("parse mutations protocol");
        for mutation in mutations::demo_mutation_cases() {
            let bytes = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed for {mutation:?}: {e:?}"));
            let trace = semio_framework_dsl::walk_protocol(&op_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(op) failed for {mutation:?} @{}: {}", e.offset, e.message));
            assert_eq!(trace.consumed, bytes.len(), "op walk did not consume every byte for {mutation:?}");
        }

        let diff_spec = semio_framework_dsl::parse_protocol(crate::standards::v2x3::subsets::base::io::binary::diff::COMPONENT_PROTOCOL_SEMIO).expect("parse diff protocol");
        for d in diff::demo_diff_cases() {
            let bytes = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed for {d:?}: {e:?}"));
            let trace = semio_framework_dsl::walk_protocol(&diff_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(diff) failed for {d:?} @{}: {}", e.offset, e.message));
            assert_eq!(trace.consumed, bytes.len(), "diff walk did not consume every byte for {d:?}");
        }
    }

    /// ✅️ `fixture_honesty_law`: the shipped `.dsl.semio`/`.pack.semio` fixtures are GENUINE
    /// `print_dsl`/`encode_pack` output of `demo_ifc2x3_snapshot()`.
    #[semio_framework_async_macros::async_test]
    async fn fixture_honesty_law() {
        const FIXTURE_DSL: &str = include_str!("../../../🧫️fixtures/🎬️demo/🗣️.dsl.semio");
        const FIXTURE_PACK: &[u8] = include_bytes!("../../../🧫️fixtures/🎬️demo/🎒️.pack.semio");

        let demo = demo_ifc2x3_snapshot();
        let authored_pack = store::ArtifactPack::encode_pack(&demo);
        assert_eq!(<Ifc2x3Snapshot as store::ArtifactPack>::decode_pack(&authored_pack).expect("current authored pack readback"), demo);


        let parsed = <Ifc2x3Snapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
        assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_ifc2x3_snapshot()");
        assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_ifc2x3_snapshot()) drifted from the shipped .dsl.semio fixture");

        let decoded = <Ifc2x3Snapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
        assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_ifc2x3_snapshot()");
        assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_ifc2x3_snapshot()) drifted from the shipped .pack.semio fixture");
    }


}
//#endregion 🔖️ConformanceLaws


/// 📐️ Every committed IFC 2x3 fixture reads through the contract's canonical Part-21 codec: the canonical text re-reads as the
/// same graph and is a fixed point. With `SEMIO_PART21_ORACLE_OUT` naming a directory, each file's canonical JSON projection is
/// written there beside its source path for the IfcOpenShell oracle (ticket `26/09/23/END-TO-END-OS-HUB-COLLABORATION-MCP`,
/// `wp-lb2/lb2-p16-part21-oracle.py`).
#[test]
fn committed_ifc2x3_fixtures_read_through_the_canonical_part21_codec() {
    use semio_s_artifact_stdio_contract::part21::{parse_part21, write_part21};
    let mut pending = vec![std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️2x3")];
    let mut files = Vec::new();
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(&directory).expect("fixture directory") {
            let path = entry.expect("fixture entry").path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().is_some_and(|extension| extension == "ifc") {
                files.push(path);
            }
        }
    }
    files.sort();
    assert!(files.len() >= 40, "the committed IFC 2x3 fixtures are found: {}", files.len());
    let out = std::env::var_os("SEMIO_PART21_ORACLE_OUT").map(std::path::PathBuf::from);
    for (index, path) in files.iter().enumerate() {
        let text = String::from_utf8(std::fs::read(path).expect("fixture bytes")).expect("utf-8 fixture");
        let document = parse_part21(&text).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
        let canonical = write_part21(&document);
        let reread = parse_part21(&canonical).expect("canonical text re-reads");
        assert_eq!(reread, document, "{}: the canonical text re-reads as the same graph", path.display());
        assert_eq!(write_part21(&reread), canonical, "{}: the canonical text is a fixed point", path.display());
        if let Some(out) = &out {
            std::fs::write(out.join(format!("{index:03}.json")), format!("{{\"source\":{},\"document\":{}}}", semio_framework_pack_json::to_json_string(&path.to_string_lossy().to_string()), semio_framework_pack_json::to_json_string(&document))).expect("oracle projection");
        }
    }
}
