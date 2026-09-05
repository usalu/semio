//! 🚪️ IO stdio.ifc.2x3 (2x3/🧱️base) — registration flows through 🎹️composer::register /
//! `engine::register` (now `schema::register`, reached through the `engine` barrel shim —
//! ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES leaves ifc's own imperative
//! registration alone per that ticket's explicit instruction; only physically dissolved out of
//! `⚙️engine`), not per-leaf register().
//!
//! 📐️ IFC2X3 is buildingSMART Coordination View 2.0-era IFC, ISO/PAS 16739:2005 schema,
//! physically-encoded identically to `📐️step`'s AP214 (`FILE_SCHEMA(('IFC2X3'))` in place of
//! `FILE_SCHEMA(('AUTOMOTIVE_DESIGN'))`). Reuses `step::engine::part21`'s tokenizer/writer
//! functions directly — PARSING-CODE reuse; what's NOT reused is `Part21Document`'s type IDENTITY
//! as this standard's snapshot type.
use crate::artifacts::ifc::standards::v2x3::subsets::base::schema::snapshot::{Ifc2x3EdmPreamble, Ifc2x3Snapshot, STDIO_IFC2X3_DOCUMENT_SCHEMA};
use crate::artifacts::step::engine::part21::{parse_part21, write_part21_with, Part21Preamble, Part21WriteOptions};
use std::fmt::Write as _;

//#region 🔖️Codec
/// 📐️ The IFC2X3 FILE_SCHEMA name a conforming Part-21 file must declare.
pub const IFC2X3_SCHEMA_NAME: &str = "IFC2X3";

/// 📥️ Decodes IFC2X3 SPF bytes into an [`Ifc2x3Snapshot`]. Real standard-specific validation
/// beyond generic Part-21 parsing: rejects any file whose `FILE_SCHEMA` doesn't declare
/// `IFC2X3` (so this decoder never silently accepts an IFC4 or plain STEP AP214 file).
pub fn decode_ifc2x3(bytes: &[u8]) -> Result<Ifc2x3Snapshot, String> {
    let text = std::str::from_utf8(bytes).map_err(|e| format!("ifc2x3: not valid utf-8: {e}"))?;
    let document = parse_part21(text).map_err(|e| format!("ifc2x3 parse: {e}"))?;
    let declares_ifc2x3 = document.header.file_schema.iter().any(|v| v.as_list().map(|items| items.iter().any(|item| item.as_str() == Some(IFC2X3_SCHEMA_NAME))).unwrap_or(false));
    if !declares_ifc2x3 {
        return Err(format!("ifc2x3: FILE_SCHEMA does not declare {IFC2X3_SCHEMA_NAME}"));
    }
    Ok(Ifc2x3Snapshot { schema: STDIO_IFC2X3_DOCUMENT_SCHEMA.into(), document, edm_preamble: parse_edm_preamble(text) })
}

/// 📤️ Regenerates valid IFC2X3 SPF bytes from a snapshot. Losslessness is `write_part21`'s job
/// (shared with `step`/`4`); this function's only own contribution is the byte encoding.
pub fn encode_ifc2x3(snapshot: &Ifc2x3Snapshot) -> Result<Vec<u8>, String> {
    crate::artifacts::ifc::standards::v2x3::subsets::base::schema::snapshot::validate_ifc2x3_snapshot(snapshot)?;
    let options = Part21WriteOptions { line_ending: "\r\n", blank_after_header: snapshot.edm_preamble.is_some(), blank_before_data: true, blank_before_terminator: true, space_after_instance_equals: true };
    Ok(write_part21_with(&snapshot.document, options, snapshot.edm_preamble.as_ref()).into_bytes())
}
//#endregion 🔖️Codec

//#region 🏭️EdmPreamble
fn parse_edm_preamble(text: &str) -> Option<Ifc2x3EdmPreamble> {
    let lines = text.lines().map(|line| line.trim_end_matches('\r')).collect::<Vec<_>>();
    let start = lines.iter().position(|line| *line == "/******************************************************************************************")?;
    let end = lines[start + 1..].iter().position(|line| *line == "******************************************************************************************/")? + start + 1;
    let value = |label: &str| {
        let prefix = format!("* {label}");
        lines[start + 1..end].iter().find_map(|line| line.strip_prefix(&prefix).map(str::trim_start)).map(str::to_string)
    };
    Some(Ifc2x3EdmPreamble {
        producer: value("STEP Physical File produced by:")?,
        module: value("Module:")?,
        creation_date: value("Creation date:")?,
        host: value("Host:")?,
        database: value("Database:")?,
        database_version: value("Database version:")?,
        database_creation_date: value("Database creation date:")?,
        schema: value("Schema:")?,
        model: value("Model:")?,
        model_creation_date: value("Model creation date:")?,
        header_model: value("Header model:")?,
        header_model_creation_date: value("Header model creation date:")?,
        user: value("EDMuser:")?,
        group: value("EDMgroup:")?,
        license: value("License ID and type:")?,
        options: value("EDMstepFileFactory options:")?,
    })
}

impl Part21Preamble for Ifc2x3EdmPreamble {
    fn write_preamble(&self, out: &mut String, line_ending: &str) {
        out.push_str("/******************************************************************************************");
        out.push_str(line_ending);
        for (label, value) in [
            ("STEP Physical File produced by:", self.producer.as_str()),
            ("Module:", self.module.as_str()),
            ("Creation date:", self.creation_date.as_str()),
            ("Host:", self.host.as_str()),
            ("Database:", self.database.as_str()),
            ("Database version:", self.database_version.as_str()),
            ("Database creation date:", self.database_creation_date.as_str()),
            ("Schema:", self.schema.as_str()),
            ("Model:", self.model.as_str()),
            ("Model creation date:", self.model_creation_date.as_str()),
            ("Header model:", self.header_model.as_str()),
            ("Header model creation date:", self.header_model_creation_date.as_str()),
            ("EDMuser:", self.user.as_str()),
            ("EDMgroup:", self.group.as_str()),
            ("License ID and type:", self.license.as_str()),
            ("EDMstepFileFactory options:", self.options.as_str()),
        ] {
            write!(out, "* {label:<31} {value}{line_ending}").expect("String write");
        }
        out.push_str("******************************************************************************************/");
        out.push_str(line_ending);
    }
}
//#endregion 🏭️EdmPreamble

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::ifc::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot;
    use crate::artifacts::ifc::standards::v2x3::subsets::base::schema::Ifc2x3Analyzer;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("2x3"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    pub struct Ifc2x3ComposerComposition;

    impl ArtifactComposition for Ifc2x3ComposerComposition {
        type Snapshot = Ifc2x3Snapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_TXT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT || s.dialect == DEP_TXT)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "Ifc2x3ComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = Ifc2x3Analyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "Ifc2x3ComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::ifc::standards::v2x3::engine::{demo_ifc2x3_snapshot, empty_ifc2x3_snapshot};
    use semio_framework_plugin::{AnalyzeSource, ArtifactAnalyzer, ArtifactComposition, ComposeSource, Dialect, StandardId, SubsetId};
    use std::sync::OnceLock;

    const IFC2X3_FIXTURE: &str = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('semio.ifc','2026-08-11T00:00:00',('Ueli'),('semio'),'semio','','');\nFILE_SCHEMA(('IFC2X3'));\nENDSEC;\nDATA;\n#1=IFCPROJECT('0YvctVUKr0kugbFTf53O9L',$,'Project',$,$,$,$,(#20),#30);\n#20=IFCUNITASSIGNMENT((#21));\n#21=IFCSIUNIT(*,.LENGTHUNIT.,$,.METRE.);\n#30=IFCGEOMETRICREPRESENTATIONCONTEXT($,'Model',3,1.E-05,#31,$);\n#31=IFCAXIS2PLACEMENT3D(#32,$,$);\n#32=IFCCARTESIANPOINT((0.,0.,0.));\n#40=IFCBUILDING('0YvctVUKr0kugbFTf53O9M',$,'Building',$,$,#41,$,$,.ELEMENT.,$,$,$);\n#41=IFCLOCALPLACEMENT($,#31);\nENDSEC;\nEND-ISO-10303-21;\n";

    async fn exact_fixture_bytes() -> &'static [u8] {
        static BYTES: OnceLock<Vec<u8>> = OnceLock::new();
        BYTES.get_or_init(|| std::fs::read(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../../temp/wellness-center-sama.ifc")).expect("read temp/wellness-center-sama.ifc"))
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
        use crate::artifacts::binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
        use crate::artifacts::ifc::standards::v2x3::subsets::base::io::export::serializers::artifacts::{binary::v_raw::base as binary_export, txt::v_utf_8::base as text_export};
        use crate::artifacts::ifc::standards::v2x3::subsets::base::io::import::deserializers::artifacts::{binary::v_raw::base as binary_import, txt::v_utf_8::base as text_import};
        use crate::artifacts::ifc::standards::v2x3::subsets::base::schema::Ifc2x3Analyzer;
        use crate::artifacts::txt::TxtSnapshot;

        let original = exact_fixture_bytes().await;
        let imported = decode_ifc2x3(original).expect("direct IFC2X3 import");
        assert_eq!(imported.document.instances.len(), 409_102, "fixture entity count changed");
        assert_exact("direct engine export", &encode_ifc2x3(&imported).expect("direct engine export")).await;

        let binary = BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes: original.to_vec() };
        let binary_snapshot = binary_import::deserialize(&binary).expect("raw binary deserialize");
        let binary_output = binary_export::serialize(&binary_snapshot).expect("raw binary serialize");
        assert_exact("raw binary route", &binary_output.bytes).await;

        let text = std::str::from_utf8(original).expect("fixture UTF-8");
        let txt = TxtSnapshot::from_body(text);
        let text_snapshot = text_import::deserialize(&txt).expect("raw text deserialize");
        let text_output = text_export::serialize(&text_snapshot).expect("raw text serialize");
        assert_exact("raw text route", text_output.to_body().as_bytes()).await;

        let text_analysis = <Ifc2x3Analyzer as ArtifactAnalyzer>::analyze(&[AnalyzeSource::Text(text)]);
        assert!(text_analysis.diagnostics.is_empty(), "text analyzer diagnostics: {:?}", text_analysis.diagnostics);
        assert_exact("text analyzer export", &encode_ifc2x3(&text_analysis.parts.snapshot.expect("text analyzer snapshot")).expect("text analyzer export")).await;

        let pack = store::ArtifactPack::encode_pack(&imported);
        let pack_analysis = <Ifc2x3Analyzer as ArtifactAnalyzer>::analyze(&[AnalyzeSource::Binary(&pack)]);
        assert!(pack_analysis.diagnostics.is_empty(), "pack analyzer diagnostics: {:?}", pack_analysis.diagnostics);
        assert_exact("pack analyzer export", &encode_ifc2x3(&pack_analysis.parts.snapshot.expect("pack analyzer snapshot")).expect("pack analyzer export")).await;

        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("2x3"), subset: SubsetId("*") };
        let text_sources = [ComposeSource { dialect: DIALECT, payload: AnalyzeSource::Text(text) }];
        let text_composition = Ifc2x3ComposerComposition::compose(&text_sources).expect("compose native IFC2X3 text");
        assert_exact("text composer export", &encode_ifc2x3(&text_composition.snapshot).expect("text composer export")).await;

        let pack_sources = [ComposeSource { dialect: DIALECT, payload: AnalyzeSource::Binary(&pack) }];
        let pack_composition = Ifc2x3ComposerComposition::compose(&pack_sources).expect("compose IFC2X3 pack");
        assert_exact("pack composer export", &encode_ifc2x3(&pack_composition.snapshot).expect("pack composer export")).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn snapshot_and_facets_forbid_native_shadow_state() {
        let value = serde_json::to_value(demo_ifc2x3_snapshot()).expect("serialize logical snapshot");
        let object = value.as_object().expect("snapshot object");
        assert_eq!(object.keys().map(String::as_str).collect::<Vec<_>>(), vec!["document", "edmPreamble", "schema"]);
        for (relative, text) in [
            ("snapshot.proto", include_str!("../🧬️schema/📸️snapshot/🛰️.proto")),
            ("snapshot.graphql", include_str!("../🧬️schema/📸️snapshot/🔗️.graphql")),
            ("snapshot.ts", include_str!("../🧬️schema/📸️snapshot/🟦️.ts")),
            ("artifact.proto", include_str!("../🧬️schema/🛰️.proto")),
            ("artifact.graphql", include_str!("../🧬️schema/🔗️.graphql")),
            ("artifact.ts", include_str!("../🧬️schema/🟦️.ts")),
        ] {
            for forbidden in ["ArtifactSource", "physical", "lexical", "document_wire", "document: Bytes", "sourceBytes"] {
                assert!(!text.contains(forbidden), "{relative} contains forbidden shadow marker {forbidden}");
            }
            assert!(text.contains("Part21Document"), "{relative} must expose the typed Part21 document");
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
        use crate::artifacts::ifc::standards::v2x3::subsets::base::schema::{diff, mutations, snapshot};
        use protocol::{DiffCodec, OpBinary, OpText};

        /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
        /// parse under the real dialect.
        #[semio_framework_async_macros::async_test]
        async fn committed_facet_files_parse() {
            for (label, text) in [("snapshot grammar", snapshot::text::COMPONENT_GRAMMAR_SEMIO), ("mutations grammar", mutations::text::COMPONENT_GRAMMAR_SEMIO), ("diff grammar", diff::text::COMPONENT_GRAMMAR_SEMIO)] {
                let grammar = dsl::parse_grammar(text).unwrap_or_else(|e| panic!("{label}: parse_grammar failed: {e:?}"));
                assert_eq!(grammar.dialect, dsl::SemioDialect::Grammar, "{label}: expected grammar dialect");
            }
            for (label, text) in [("snapshot protocol", snapshot::binary::COMPONENT_PROTOCOL_SEMIO), ("mutations protocol", mutations::binary::COMPONENT_PROTOCOL_SEMIO), ("diff protocol", diff::binary::COMPONENT_PROTOCOL_SEMIO)] {
                dsl::parse_protocol(text).unwrap_or_else(|e| panic!("{label}: parse_protocol failed: {e:?}"));
            }
        }

        /// ✅️ `grammar_conformance_law`: the snapshot grammar recognizes real `print_dsl` output for
        /// the demo snapshot AND the empty-instances case, preamble-stripped-and-reconstructed the
        /// same way `m5_handcrafted_grammar_conformance` itself does.
        #[semio_framework_async_macros::async_test]
        async fn grammar_conformance_law() {
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
        #[semio_framework_async_macros::async_test]
        async fn ops_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for mutation in mutations::demo_mutation_cases() {
                let printed = mutation.print_op();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
            }
        }

        /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff` output
        /// for every representative `Ifc2x3Diff` demo case.
        #[semio_framework_async_macros::async_test]
        async fn diff_grammar_conformance_law() {
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
        #[semio_framework_async_macros::async_test]
        async fn protocol_walk_law() {
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
        #[semio_framework_async_macros::async_test]
        async fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../📚️examples/🎬️demo/🖼️assets/🎒️.pack.semio");

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

//#region 🚪️DerivedIoRegistry
/// 🚪️ Dissolved out of `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
pub mod io_registry {
    use crate::artifacts::ifc::standards::v2x3::subsets::base::schema::Ifc2x3Composer as Ifc2x3RawAnyComposer;
    use crate::artifacts::ifc::standards::v2x3::subsets::cobie::schema::Ifc2x3CobieComposer;
    use crate::artifacts::ifc::standards::v2x3::subsets::cv20::schema::Ifc2x3Cv20Composer;
    use crate::artifacts::ifc::standards::v2x3::subsets::sav::schema::Ifc2x3SavComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<Ifc2x3RawAnyComposer>(), composer_entry_of::<Ifc2x3Cv20Composer>(), composer_entry_of::<Ifc2x3SavComposer>(), composer_entry_of::<Ifc2x3CobieComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
