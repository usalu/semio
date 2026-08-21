//! 🚪️ IO stdio.svg (1.1/✳️any) — registration now flows through 🎹️composer::register
//! (called once from 🔌️plugin/🔧️setup via ⚙️engine::register), not per-leaf register().
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::svg::standards::v1_1::subsets::any::schema::SvgAnalyzer;
    use crate::artifacts::svg::SvgSnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("*") };
    const DEP_XML: Dialect = Dialect { artifact_kind: "s.stdio.xml", standard: StandardId("1.0"), subset: SubsetId("*") };

    pub struct SvgComposerComposition;

    impl ArtifactComposition for SvgComposerComposition {
        type Snapshot = SvgSnapshot;
        const WRITES: Dialect = DIALECT;

        async fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_XML]
        }

        async fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            // 🌱 Every listed read dialect's payload is raw text/bytes that this artifact's own
            // analyzer already round-trips through `store::Document{Dsl,Pack}` -- including bytes
            // claiming a dependency's dialect, since (for a single-standard DAG-adjacent dependency
            // like binary) that payload IS the same byte/text shape `analyze` already accepts.
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT || s.dialect == DEP_XML)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "SvgComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = SvgAnalyzer::analyze(&native).await;
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "SvgComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️DerivedIoRegistry
// 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): moved
// here verbatim (destination rule 3 — `io_registry`/`ComposerEntry` → `🚪️io/`). This is the REAL
// `&'static [ComposerEntry]` entries (NOT the artifact root's own shadowing `io_registry` module,
// which returns `&[&ComposerEntry]` — a different type; the root's `.composers(...)` reaches this
// module by its fully-qualified path, never the bare `io_registry::entries()` shortcut, per this
// ticket's own "silent rebind" hazard).
pub mod io_registry {
    use crate::artifacts::svg::standards::v1_1::subsets::any::schema::SvgComposer as SvgRawAnyComposer;
    use crate::artifacts::svg::standards::v1_1::subsets::basic::schema::SvgBasicComposer;
    use crate::artifacts::svg::standards::v1_1::subsets::tiny::schema::SvgTinyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<SvgRawAnyComposer>(), composer_entry_of::<SvgTinyComposer>(), composer_entry_of::<SvgBasicComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::svg::schema::{demo_svg_snapshot, empty_svg_snapshot};
    use crate::artifacts::svg::{SvgSnapshot, STDIO_SVG_DOCUMENT_SCHEMA};
    use semio_framework_plugin::{AnalyzeSource, ArtifactAnalysis, ArtifactComposition, ComposeSource, Dialect, StandardId, SubsetId};

    const SVG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("*") };

    async fn exact_fixture_bytes() -> Vec<u8> {
        std::fs::read(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../../temp/artifacts.svg")).expect("read temp/artifacts.svg")
    }

    #[semio_framework_async_macros::async_test]
    async fn empty_snapshot_matches_schema() {
        let snapshot = empty_svg_snapshot();
        assert_eq!(snapshot.schema, STDIO_SVG_DOCUMENT_SCHEMA);
    }

    #[semio_framework_async_macros::async_test]
    async fn codec_round_trip() {
        let snap = empty_svg_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <SvgSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.schema, snap.schema);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <SvgSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }

    //#region 🔖️LosslessNativeRouting
    #[semio_framework_async_macros::async_test]
    async fn exact_native_analyzer_text_and_pack_roundtrip() {
        let original = exact_fixture_bytes();
        let text = std::str::from_utf8(&original).expect("fixture UTF-8");
        let text_analysis = <crate::artifacts::svg::standards::v1_1::subsets::any::schema::SvgAnalyzerAnalysis as ArtifactAnalysis>::analyze(&[AnalyzeSource::Text(text)]);
        assert!(text_analysis.diagnostics.is_empty(), "text diagnostics: {:?}", text_analysis.diagnostics);
        let text_snapshot = text_analysis.parts.snapshot.expect("text snapshot");
        assert_eq!(text_snapshot.export_utf8().expect("text analyzer export"), original);

        let pack = store::ArtifactPack::encode_pack(&text_snapshot);
        let pack_analysis = <crate::artifacts::svg::standards::v1_1::subsets::any::schema::SvgAnalyzerAnalysis as ArtifactAnalysis>::analyze(&[AnalyzeSource::Binary(&pack)]);
        assert!(pack_analysis.diagnostics.is_empty(), "pack diagnostics: {:?}", pack_analysis.diagnostics);
        assert_eq!(pack_analysis.parts.snapshot.expect("pack snapshot").export_utf8().expect("pack analyzer export"), original);
    }

    #[semio_framework_async_macros::async_test]
    async fn exact_native_composer_text_and_pack_roundtrip() {
        let original = exact_fixture_bytes();
        let text = std::str::from_utf8(&original).expect("fixture UTF-8");
        let text_sources = [ComposeSource { dialect: SVG_DIALECT, payload: AnalyzeSource::Text(text) }];
        let text_composition = SvgComposerComposition::compose(&text_sources).expect("compose raw SVG text");
        assert_eq!(text_composition.snapshot.export_utf8().expect("text composition export"), original);

        let pack = store::ArtifactPack::encode_pack(&text_composition.snapshot);
        let pack_sources = [ComposeSource { dialect: SVG_DIALECT, payload: AnalyzeSource::Binary(&pack) }];
        let pack_composition = SvgComposerComposition::compose(&pack_sources).expect("compose SVG pack");
        assert_eq!(pack_composition.snapshot.export_utf8().expect("pack composition export"), original);
    }
    //#endregion 🔖️LosslessNativeRouting

    //#region 🔖️ConformanceLaws
    /// 🧪️ P2-FG3: per-artifact conformance laws (`📖️grammar-recipe.md` §4's checklist item) --
    /// grammar/protocol parseability, `Recognizer` against real fixtures AND real `print_op`/
    /// `print_diff` output, `walk_protocol` against real `encode_pack`/`encode_op`/`encode_diff`
    /// bytes, and the fixture-honesty round-trip. Relocated verbatim from `⚙️engine`'s own test
    /// region (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) -- `m5` auto-discovers
    /// the snapshot grammar+`.dsl.semio`/protocol+`.pack.semio` pairs independently; these tests
    /// are this artifact's OWN early-warning, plus direct coverage of the mutations/diff facets
    /// that harness does not auto-discover at all.
    mod conformance_laws {
        use super::*;
        use crate::artifacts::svg::schema::{diff, mutations, snapshot};
        use protocol::{DiffCodec, OpBinary, OpText};

        /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
        /// parse under the real dialect -- independent of, and cheaper than, the two `recognize`/
        /// `walk_protocol` laws below (a parse failure here fails fast with a clearer message).
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

        /// ✅️ `grammar_conformance_law`: the snapshot grammar recognizes real `print_dsl` output
        /// for the demo document -- same preamble-stripped body reconstruction
        /// `m5_handcrafted_grammar_conformance`'s own `dsl_body_from_fixture` uses, so this is a
        /// direct proof this artifact will pass that harness once graduated, not merely an analogue.
        #[semio_framework_async_macros::async_test]
        async fn grammar_conformance_law() {
            let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            let text = store::ArtifactDsl::print_dsl(&demo_svg_snapshot());
            let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
            let reconstructed = format!("{}\n{body}", envelope.envelope_id());
            assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
        }

        /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
        /// output for every `SvgMutation` variant (`mutations::demo_mutation_cases()`).
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
        /// for every representative `SvgDiff` (`diff::demo_diff_cases()`), incl. the empty-line
        /// (all-`None`) diff and the `Replace` kind-change fallback.
        #[semio_framework_async_macros::async_test]
        async fn diff_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(diff::text::COMPONENT_GRAMMAR_SEMIO).expect("parse diff grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for d in diff::demo_diff_cases() {
                let printed = d.print_diff();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "diff grammar did not recognize {printed:?} (from {d:?})");
            }
        }

        /// ✅️ `protocol_walk_law`: `walk_protocol` against REAL bytes for all three facets --
        /// snapshot pack (`encode_pack`, envelope-unwrapped first, matching how
        /// `m5_handcrafted_protocol_conformance` itself feeds `walk_protocol`), every demo
        /// mutation's `encode_op`, and every demo diff's `encode_diff`, asserting `consumed ==
        /// bytes.len()`.
        #[semio_framework_async_macros::async_test]
        async fn protocol_walk_law() {
            let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
            let packed = store::ArtifactPack::encode_pack(&demo_svg_snapshot());
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
        /// `print_dsl`/`encode_pack` output of `demo_svg_snapshot()` -- `parse_dsl(fixture) ==
        /// demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the pack twin -- so the
        /// fixtures can never silently drift back to a fake again.
        #[semio_framework_async_macros::async_test]
        async fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio");

            let demo = demo_svg_snapshot();

            let parsed = <SvgSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
            assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_svg_snapshot()");
            assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_svg_snapshot()) drifted from the shipped .dsl.semio fixture");

            let decoded = <SvgSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
            assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_svg_snapshot()");
            assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_svg_snapshot()) drifted from the shipped .pack.semio fixture");
        }
    }
    //#endregion 🔖️ConformanceLaws
}
//#endregion 🧪️Tests
