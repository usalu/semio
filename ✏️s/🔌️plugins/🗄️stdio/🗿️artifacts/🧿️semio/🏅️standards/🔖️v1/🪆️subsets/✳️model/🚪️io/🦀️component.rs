//! 🚪️ IO — 🚧 scaffolded by W1b: structure only. Registration flows through
//! 🎹️composer::register (matching the repo-wide convention — see gif's own io leaf doc comment).
//! W4 adds the real import/export leaves under 📥️import/🧩️deserializers and
//! 📤️export/🧵️serializers.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;
    use crate::artifacts::semio::standards::v1::subsets::model::schema::SemioModelAnalyzer;
    use semio_framework_plugin::{
        deserializer_entry_of, register_composer_entries, register_subset_validator, serializer_entry_of, subset_validator_entry_of, AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, ComposerEntry, Composition,
        Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry,
    };
    use std::collections::HashSet;

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("model") };

    //#region 🔖️Composer
    pub struct SemioModelComposerComposition;

    impl ArtifactComposition for SemioModelComposerComposition {
        type Snapshot = SemioModelSnapshot;
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
                return Err(ComposeError { message: "SemioModelComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = SemioModelAnalyzer::analyze(&native).await;
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "SemioModelComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ Real referential-invariant checks over `model`'s OWN collections (decode + dangling-id
    /// checks): a spatial node's `parent_id`, an element's `spatial_id`, and a relation's `from`/`to`
    /// must all resolve within THIS snapshot's own `spatial`/`elements` id spaces. Cross-subset
    /// references (`GeometryRef::Brep{brep_id}`/`Mesh{mesh_id}` into the sibling `brep`/`mesh`
    /// subsets) are NOT checked here — they are not decodable from a `model` snapshot alone, per the
    /// snapshot module's own doc comment.
    pub struct SemioModelValidator;

    /// 🔎️ Dangling-reference diagnostics for a decoded snapshot — split out from `validate()` so it's
    /// directly unit-testable against a typed `SemioModelSnapshot` (not just through the `IoPayload`
    /// wire boundary).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn semio_model_referential_diagnostics(snapshot: &SemioModelSnapshot) -> Vec<dsl::Diagnostic> {
        let spatial_ids: HashSet<&str> = snapshot.spatial.iter().map(|n| n.id.as_str()).collect();
        let element_ids: HashSet<&str> = snapshot.elements.iter().map(|e| e.id.as_str()).collect();
        let mut diagnostics = Vec::new();

        for node in &snapshot.spatial {
            if let Some(parent) = &node.parent_id {
                if parent == &node.id {
                    diagnostics.push(dsl::Diagnostic::error("stdio.semio_model.validate-self-parent", dsl::TextSpan::at(1, 1), format!("spatial node {:?} is its own parent", node.id)));
                } else if !spatial_ids.contains(parent.as_str()) {
                    diagnostics.push(dsl::Diagnostic::error("stdio.semio_model.validate-dangling-parent", dsl::TextSpan::at(1, 1), format!("spatial node {:?} references missing parent {:?}", node.id, parent)));
                }
            }
        }

        for element in &snapshot.elements {
            if let Some(spatial_id) = &element.spatial_id {
                if !spatial_ids.contains(spatial_id.as_str()) {
                    diagnostics.push(dsl::Diagnostic::error("stdio.semio_model.validate-dangling-spatial-ref", dsl::TextSpan::at(1, 1), format!("element {:?} references missing spatial node {:?}", element.id, spatial_id)));
                }
            }
        }

        for relation in &snapshot.relations {
            let endpoint_known = |id: &str| element_ids.contains(id) || spatial_ids.contains(id);
            if !endpoint_known(&relation.from) {
                diagnostics.push(dsl::Diagnostic::error("stdio.semio_model.validate-dangling-relation-from", dsl::TextSpan::at(1, 1), format!("relation {:?} references missing from-id {:?}", relation.id, relation.from)));
            }
            if !endpoint_known(&relation.to) {
                diagnostics.push(dsl::Diagnostic::error("stdio.semio_model.validate-dangling-relation-to", dsl::TextSpan::at(1, 1), format!("relation {:?} references missing to-id {:?}", relation.id, relation.to)));
            }
        }

        diagnostics
    }

    impl SubsetValidator for SemioModelValidator {
        const DIALECT: Dialect = DIALECT;
        async fn validate(payload: &IoPayload) -> Vec<dsl::Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <SemioModelSnapshot as store::ArtifactPack>::decode_pack(bytes).await.ok(),
                IoPayload::Text(text) => <SemioModelSnapshot as store::ArtifactDsl>::parse_dsl(text).await.ok(),
            };
            match decoded {
                Some(snapshot) => semio_model_referential_diagnostics(&snapshot),
                None => vec![dsl::Diagnostic::error("stdio.semio_model.validate-decode-failed", dsl::TextSpan::at(1, 1), "SemioModelValidator: payload did not decode as a SemioModelSnapshot".to_string())],
            }
        }
    }

    static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioModelValidator>)
    }
    //#endregion 🔖️SubsetValidator

    //#region 🔖️Register
    /// 📌️ Registers this subset's schema descriptor, document codec, and SubsetValidator. Called from
    /// this artifact's standard-level `engine::register()`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::model::schema::semio_model_artifact_schema_descriptor());
        let _ = store::register_document_codec(store::ArtifactCodec::of::<SemioModelSnapshot, crate::artifacts::semio::standards::v1::subsets::model::schema::mutations::SemioModelMutation>(
            crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::STDIO_SEMIOMODEL_DOCUMENT_SCHEMA,
        ));
        let _ = register_subset_validator(validator_entry());
        let _ = register_composer_entries(io_bridge_entries());
        register_artifact_inferences();
    }

    /// 💡️ Registers `s.stdio.semio.model.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::schema::register_artifact_inference_descriptor(crate::artifacts::semio::standards::v1::subsets::model::schema::inferences::semio_model_artifact_inference_descriptor());
    }
    //#endregion 🔖️Register

    //#region 🔖️IoBridges
    /// 🌉️ W4 real semio↔format bridge entries. Each `deserializer_entry_of`/`serializer_entry_of`
    /// pair registers BOTH `IoKey` directions per `register_composer_entries`'s own doc comment (a
    /// deserializer writing `model`/reading `<format>` also gives `<format>`-exports-to-`model`; its
    /// mirror serializer gives the other two) — four `IoKey`s per (subset, format) pair from these two
    /// rows, no hand-written reverse registration needed.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn io_bridge_entries() -> &'static [ComposerEntry] {
        static ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
        ENTRIES
            .get_or_init(|| {
                vec![
                    deserializer_entry_of::<crate::artifacts::semio::standards::v1::subsets::model::io::import::deserializers::artifacts::ifc::v4::any::SemioModelFromIfc>(),
                    serializer_entry_of::<crate::artifacts::semio::standards::v1::subsets::model::io::export::serializers::artifacts::ifc::v4::any::SemioModelToIfc>(),
                    deserializer_entry_of::<crate::artifacts::semio::standards::v1::subsets::model::io::import::deserializers::artifacts::bcf::v2_1::any::SemioModelFromBcf>(),
                    serializer_entry_of::<crate::artifacts::semio::standards::v1::subsets::model::io::export::serializers::artifacts::bcf::v2_1::any::SemioModelToBcf>(),
                ]
            })
            .as_slice()
    }
    //#endregion 🔖️IoBridges

    //#region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioTransform;
        use crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::{ElementClass, GeometryRef, ModelRelation, RelationKind, SemioModelElement, SpatialKind, SpatialNode};

        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        fn clean_snapshot() -> SemioModelSnapshot {
            SemioModelSnapshot {
                schema: SemioModelSnapshot::default().schema,
                spatial: vec![SpatialNode { id: "s1".into(), kind: SpatialKind::Site, name: "Site".into(), parent_id: None, placement: SemioTransform::identity() }],
                elements: vec![SemioModelElement { id: "e1".into(), class: ElementClass::Wall, placement: SemioTransform::identity(), geometry: GeometryRef::None, spatial_id: Some("s1".into()), psets: vec![] }],
                relations: vec![ModelRelation { id: "r1".into(), kind: RelationKind::Aggregates, from: "e1".into(), to: "s1".into() }],
            }
        }

        #[semio_framework_async_macros::async_test]
        async fn clean_snapshot_has_no_referential_diagnostics() {
            let diagnostics = semio_model_referential_diagnostics(&clean_snapshot());
            assert!(diagnostics.is_empty(), "expected no diagnostics, got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn dangling_spatial_parent_is_flagged() {
            let mut snap = clean_snapshot();
            snap.spatial[0].parent_id = Some("missing".into());
            let diagnostics = semio_model_referential_diagnostics(&snap);
            assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_model.validate-dangling-parent"), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn dangling_element_spatial_ref_is_flagged() {
            let mut snap = clean_snapshot();
            snap.elements[0].spatial_id = Some("missing".into());
            let diagnostics = semio_model_referential_diagnostics(&snap);
            assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_model.validate-dangling-spatial-ref"), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn dangling_relation_endpoints_are_flagged() {
            let mut snap = clean_snapshot();
            snap.relations[0].from = "missing-from".into();
            snap.relations[0].to = "missing-to".into();
            let diagnostics = semio_model_referential_diagnostics(&snap);
            assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_model.validate-dangling-relation-from"), "got {diagnostics:?}");
            assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_model.validate-dangling-relation-to"), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn validator_validate_runs_the_same_checks_through_the_io_payload_boundary() {
            let bytes = <SemioModelSnapshot as store::ArtifactPack>::encode_pack(&clean_snapshot());
            let diagnostics = SemioModelValidator::validate(&IoPayload::Binary(bytes.await));
            assert!(diagnostics.await.is_empty(), "clean snapshot must validate through the wire boundary too: {diagnostics:?}");
        }

        //#region 🔖️ConformanceLaws
        /// 🧪️ Per-artifact conformance laws (grammar recipe §4 item 8) for `s.stdio.semio.model`'s
        /// three facets — following `stdio.semio.flow`'s proven P2 pilot pattern. Lives in this
        /// composer's own test region: `model` has no per-standard `⚙️engine` dir the way json/csv/zip/
        /// png do, and v1's SHARED `🏅️standards/🔖️v1/⚙️engine/🦀️component.rs` only aggregates all 14
        /// subsets' `register()` calls (no test module of its own, and out of this ticket's
        /// `✳️model/`-only edit scope anyway).
        mod conformance_laws {
            
            use crate::artifacts::semio::standards::v1::subsets::model::schema::{diff, mutations, snapshot};
            use protocol::{DiffCodec, OpBinary, OpText};

            /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
            /// parse under the real dialect — independent of, and cheaper than, the two `recognize`/
            /// `walk_protocol` laws below.
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
            /// the demo snapshot — same preamble-stripped body reconstruction the eventual
            /// `m5_handcrafted_grammar_conformance` harness uses (envelope id prepended as the bare
            /// `artifact-mark` token), so this is a direct proof this facet will pass that harness once
            /// graduated.
            #[semio_framework_async_macros::async_test]
            async fn grammar_conformance_law() {
                let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
                let recognizer = dsl::Recognizer::compile(&grammar);
                let text = store::ArtifactDsl::print_dsl(&snapshot::demo_semio_model_snapshot());
                let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
                let reconstructed = format!("{}\n{body}", envelope.envelope_id());
                assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
            }

            /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op` output
            /// for every `SemioModelMutation` variant (`mutations::demo_mutation_cases()`).
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
            /// for every representative `SemioModelDiff` (`diff::demo_diff_cases()`), incl. the empty
            /// (no-op) diff.
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
                let packed = store::ArtifactPack::encode_pack(&snapshot::demo_semio_model_snapshot());
                let (_, inner) = store::semio_format::unwrap_binary(&packed).expect("unwrap semio envelope");
                let trace = dsl::walk_protocol(&pack_spec, &inner).unwrap_or_else(|e| panic!("walk_protocol(pack) failed @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, inner.len(), "pack walk did not consume every byte");

                let op_spec = dsl::parse_protocol(mutations::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse mutations protocol");
                for mutation in mutations::demo_mutation_cases() {
                    let bytes = mutation.encode_op().await.unwrap_or_else(|e| panic!("encode_op failed for {mutation:?}: {e:?}"));
                    let trace = dsl::walk_protocol(&op_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(op) failed for {mutation:?} @{}: {}", e.offset, e.message));
                    assert_eq!(trace.consumed, bytes.len(), "op walk did not consume every byte for {mutation:?}");
                }

                let diff_spec = dsl::parse_protocol(diff::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse diff protocol");
                for d in diff::demo_diff_cases() {
                    let bytes = d.encode_diff().await.unwrap_or_else(|e| panic!("encode_diff failed for {d:?}: {e:?}"));
                    let trace = dsl::walk_protocol(&diff_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(diff) failed for {d:?} @{}: {}", e.offset, e.message));
                    assert_eq!(trace.consumed, bytes.len(), "diff walk did not consume every byte for {d:?}");
                }
            }

            /// ✅️ `fixture_honesty_law`: the shipped `.dsl.semio`/`.pack.semio` fixtures are GENUINE
            /// `print_dsl`/`encode_pack` output of `snapshot::demo_semio_model_snapshot()` —
            /// `parse_dsl(fixture) == demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the
            /// pack twin — so the fixtures can never silently drift back to a fake.
            #[semio_framework_async_macros::async_test]
            async fn fixture_honesty_law() {
                const FIXTURE_DSL: &str = include_str!("../../✳️any/📚️examples/🏢️building/🖼️assets/🗣️example.dsl.semio");
                const FIXTURE_PACK: &[u8] = include_bytes!("../../✳️any/📚️examples/🏢️building/🖼️assets/🎒️example.pack.semio");

                let demo = snapshot::demo_semio_model_snapshot();

                let parsed = <snapshot::SemioModelSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).await.expect("parse shipped .dsl.semio fixture");
                assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_semio_model_snapshot()");
                assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_semio_model_snapshot()) drifted from the shipped .dsl.semio fixture");

                let decoded = <snapshot::SemioModelSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).await.expect("decode shipped .pack.semio fixture");
                assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_semio_model_snapshot()");
                assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_semio_model_snapshot()) drifted from the shipped .pack.semio fixture");
            }
        }
        //#endregion 🔖️ConformanceLaws
    }
    //#endregion 🔖️Tests
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
