//! 🚪️ IO — 🚧 scaffolded by W1b: structure only. Registration flows through
//! 🎹️composer::register (matching the repo-wide convention — see gif's own io leaf doc comment).
//! W4 adds the real import/export leaves under 📥️import/🧩️deserializers and
//! 📤️export/🧵️serializers.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueSnapshot, ValueId};
    use crate::artifacts::semio::standards::v1::subsets::value::schema::SemioValueAnalyzer;
    use semio_framework_plugin::{
        deserializer_entry_of, register_composer_entries, register_subset_validator, serializer_entry_of, subset_validator_entry_of, AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, ComposerEntry, Composition,
        Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry,
    };
    use std::collections::HashSet;

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("value") };

    //#region 🔖️Composer
    pub struct SemioValueComposerComposition;

    impl ArtifactComposition for SemioValueComposerComposition {
        type Snapshot = SemioValueSnapshot;
        const WRITES: Dialect = DIALECT;

        async fn reads() -> &'static [Dialect] {
            &[DIALECT]
        }

        async fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "SemioValueComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = SemioValueAnalyzer::analyze(&native).await;
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "SemioValueComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🕸️ Recursively collects every `Ref{id}` reachable from `value` — used against BOTH `root` and
    /// every `nodes` node's own `value` (a `Ref` can legally point from inside the graph back into
    /// itself, or into a sibling node, not only from `root`).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn collect_refs(value: &SemioValue, out: &mut Vec<ValueId>) {
        match value {
            SemioValue::Ref { id } => out.push(id.clone()),
            SemioValue::List { items } => items.iter().for_each(|v| collect_refs(v, out)),
            SemioValue::Map { entries } => entries.iter().for_each(|e| collect_refs(&e.value, out)),
            _ => {}
        }
    }

    /// 🛡️ Decodes the payload as this subset's OWN `SemioValueSnapshot`, then checks two real
    /// referential invariants over its own collections: (1) every `Ref{id}` reachable from `root` or
    /// from any `nodes` node's value resolves to a real entry in `nodes` (no dangling ids); (2)
    /// `nodes` carries no duplicate `id` (the graph's backing store is id-ADDRESSABLE, a duplicate
    /// id makes resolution ambiguous).
    pub struct SemioValueValidator;

    impl SubsetValidator for SemioValueValidator {
        const DIALECT: Dialect = DIALECT;
        async fn validate(payload: &IoPayload) -> Vec<dsl::Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <SemioValueSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <SemioValueSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            let snapshot = match decoded {
                Some(snapshot) => snapshot,
                None => {
                    return vec![dsl::Diagnostic::error("stdio.semio_value.validate-decode-failed", dsl::TextSpan::at(1, 1), "SemioValueValidator: payload did not decode as a SemioValueSnapshot".to_string())];
                }
            };

            let mut diagnostics = Vec::new();

            let known_ids: HashSet<&ValueId> = snapshot.nodes.iter().map(|n| &n.id).collect();
            let mut seen_ids: HashSet<&ValueId> = HashSet::new();
            for node in &snapshot.nodes {
                if !seen_ids.insert(&node.id) {
                    diagnostics.push(dsl::Diagnostic::error("stdio.semio_value.validate-duplicate-id", dsl::TextSpan::at(1, 1), format!("SemioValueValidator: duplicate value id '{}' in `nodes`", node.id.value)));
                }
            }

            let mut refs = Vec::new();
            collect_refs(&snapshot.root, &mut refs);
            for node in &snapshot.nodes {
                collect_refs(&node.value, &mut refs);
            }
            let mut reported_dangling: HashSet<String> = HashSet::new();
            for id in refs {
                if !known_ids.contains(&id) && reported_dangling.insert(id.value.clone()) {
                    diagnostics.push(dsl::Diagnostic::error("stdio.semio_value.validate-dangling-ref", dsl::TextSpan::at(1, 1), format!("SemioValueValidator: Ref{{id: '{}'}} does not resolve to any entry in `nodes`", id.value)));
                }
            }

            diagnostics
        }
    }

    static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioValueValidator>)
    }
    //#endregion 🔖️SubsetValidator

    //#region 🔖️Register
    /// 📌️ Registers this subset's schema descriptor, document codec, and SubsetValidator. Called from
    /// this artifact's standard-level `engine::register()`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::value::schema::semio_value_artifact_schema_descriptor());
        let _ = store::register_document_codec(store::ArtifactCodec::of::<SemioValueSnapshot, crate::artifacts::semio::standards::v1::subsets::value::schema::mutations::SemioValueMutation>(
            crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::STDIO_SEMIOVALUE_DOCUMENT_SCHEMA,
        ));
        let _ = register_subset_validator(validator_entry());
        let _ = register_composer_entries(io_bridge_entries());
        register_artifact_inferences();
    }

    /// 💡️ Registers `s.stdio.semio.value.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::schema::register_artifact_inference_descriptor(crate::artifacts::semio::standards::v1::subsets::value::schema::inferences::semio_value_artifact_inference_descriptor());
    }
    //#endregion 🔖️Register

    //#region 🧪️Tests
    #[cfg(test)]
    mod tests {
        

        //#region 🔖️ConformanceLaws
        /// 🧪️ Per-facet conformance laws (grammar-recipe.md §4 deliverable 7): grammar/protocol
        /// parseability, `Recognizer` against real fixtures AND real `print_op`/`print_diff` output,
        /// `walk_protocol` against real `encode_pack`/`encode_op`/`encode_diff` bytes, and the
        /// fixture-honesty round-trip. Lives here (this subset's own `🎹️composer`, its closest
        /// "engine-equivalent" home — `value` has no per-subset `⚙️engine/` dir, only the SHARED
        /// 14-subset `🏅️standards/🔖️v1/⚙️engine/🦀️component.rs` aggregator, out of this subset's edit
        /// scope), same convention `json`'s `⚙️engine/🦀️component.rs` and `flow`'s own
        /// `🎹️composer/🦀️component.rs` use.
        mod conformance_laws {
            
            use crate::artifacts::semio::standards::v1::subsets::value::schema::{diff, mutations, snapshot};
            use protocol::{DiffCodec, OpBinary, OpText};

            /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
            /// parse under the real dialect — independent of, and cheaper than, the two `recognize`/
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
            /// for the demo snapshot — same preamble-stripped body reconstruction the eventual
            /// `m5_handcrafted_grammar_conformance` harness uses (envelope id prepended as the bare
            /// first token), so this is a direct proof this facet will pass that harness once graduated.
            #[semio_framework_async_macros::async_test]
            async fn grammar_conformance_law() {
                let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
                let recognizer = dsl::Recognizer::compile(&grammar);
                let text = store::ArtifactDsl::print_dsl(&snapshot::demo_semio_value_snapshot());
                let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
                let reconstructed = format!("{}\n{body}", envelope.envelope_id());
                assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
            }

            /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
            /// output for every `SemioValueMutation` variant (`mutations::demo_mutation_cases()`),
            /// incl. nested list/map payload values and a multi-segment mixed `SemioValuePath`.
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
            /// for every representative `SemioValueTreeDiff` (`diff::demo_diff_cases()`), incl. the empty
            /// (no-op) diff and the `Replace` kind-change fallback.
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
            /// snapshot pack (`encode_pack`, envelope-unwrapped first), every demo mutation's
            /// `encode_op`, and every demo diff's `encode_diff` — asserting `consumed == bytes.len()`.
            #[semio_framework_async_macros::async_test]
            async fn protocol_walk_law() {
                let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
                let packed = store::ArtifactPack::encode_pack(&snapshot::demo_semio_value_snapshot());
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
            /// `print_dsl`/`encode_pack` output of `snapshot::demo_semio_value_snapshot()` —
            /// `parse_dsl(fixture) == demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the
            /// pack twin — so the fixtures can never silently drift back to a fake.
            #[semio_framework_async_macros::async_test]
            async fn fixture_honesty_law() {
                const FIXTURE_DSL: &str = include_str!("../../✳️any/📚️examples/🕸️graph/🖼️assets/🗣️example.dsl.semio");
                const FIXTURE_PACK: &[u8] = include_bytes!("../../✳️any/📚️examples/🕸️graph/🖼️assets/🎒️example.pack.semio");

                let demo = snapshot::demo_semio_value_snapshot();

                let parsed = <snapshot::SemioValueSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
                assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_semio_value_snapshot()");
                assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_semio_value_snapshot()) drifted from the shipped .dsl.semio fixture");

                let decoded = <snapshot::SemioValueSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
                assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_semio_value_snapshot()");
                assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_semio_value_snapshot()) drifted from the shipped .pack.semio fixture");
            }
        }
        //#endregion 🔖️ConformanceLaws
    }
    //#endregion 🧪️Tests

    //#region 🔖️IoBridges
    /// 🌉️ W4 real semio↔format bridge entries. Each `deserializer_entry_of`/`serializer_entry_of`
    /// pair registers BOTH `IoKey` directions per `register_composer_entries`'s own doc comment.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn io_bridge_entries() -> &'static [ComposerEntry] {
        static ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
        ENTRIES
            .get_or_init(|| {
                vec![
                    deserializer_entry_of::<crate::artifacts::semio::standards::v1::subsets::value::io::import::deserializers::artifacts::json::v_rfc8259::any::SemioValueFromJson>(),
                    serializer_entry_of::<crate::artifacts::semio::standards::v1::subsets::value::io::export::serializers::artifacts::json::v_rfc8259::any::SemioValueToJson>(),
                    deserializer_entry_of::<crate::artifacts::semio::standards::v1::subsets::value::io::import::deserializers::artifacts::xml::v1_0::any::SemioValueFromXml>(),
                    serializer_entry_of::<crate::artifacts::semio::standards::v1::subsets::value::io::export::serializers::artifacts::xml::v1_0::any::SemioValueToXml>(),
                    deserializer_entry_of::<crate::artifacts::semio::standards::v1::subsets::value::io::import::deserializers::artifacts::csv::v_rfc4180::any::SemioValueFromCsv>(),
                    serializer_entry_of::<crate::artifacts::semio::standards::v1::subsets::value::io::export::serializers::artifacts::csv::v_rfc4180::any::SemioValueToCsv>(),
                ]
            })
            .as_slice()
    }
    //#endregion 🔖️IoBridges
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
