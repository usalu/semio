//! 🚪️ IO — 🚧 scaffolded by W1b: structure only. Registration flows through
//! 🎹️composer::register (matching the repo-wide convention — see gif's own io leaf doc comment).
//! W4 adds the real import/export leaves under 📥️import/🧩️deserializers and
//! 📤️export/🧵️serializers.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
    use crate::artifacts::semio::standards::v1::subsets::mesh::schema::SemioMeshAnalyzer;
    use semio_framework_plugin::{
        deserializer_entry_of, register_composer_entries, register_subset_validator, serializer_entry_of, subset_validator_entry_of, AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, ComposerEntry, Composition,
        Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry,
    };
    //#region 🔖️IoBridgeImports
    // 🌉️ W4 (mesh↔{gltf,stl,obj,ply,las}) io leaves — real trait impls registered below.
    use crate::artifacts::semio::standards::v1::subsets::mesh::io::export::serializers::artifacts::gltf::v2_0::any::SemioMeshToGltf;
    use crate::artifacts::semio::standards::v1::subsets::mesh::io::export::serializers::artifacts::las::v1_0::any::SemioMeshToLas;
    use crate::artifacts::semio::standards::v1::subsets::mesh::io::export::serializers::artifacts::obj::v3_0::any::SemioMeshToObj;
    use crate::artifacts::semio::standards::v1::subsets::mesh::io::export::serializers::artifacts::ply::v1_0::any::SemioMeshToPly;
    use crate::artifacts::semio::standards::v1::subsets::mesh::io::export::serializers::artifacts::stl::v_ascii::any::SemioMeshToStl;
    use crate::artifacts::semio::standards::v1::subsets::mesh::io::import::deserializers::artifacts::gltf::v2_0::any::SemioMeshFromGltf;
    use crate::artifacts::semio::standards::v1::subsets::mesh::io::import::deserializers::artifacts::las::v1_0::any::SemioMeshFromLas;
    use crate::artifacts::semio::standards::v1::subsets::mesh::io::import::deserializers::artifacts::obj::v3_0::any::SemioMeshFromObj;
    use crate::artifacts::semio::standards::v1::subsets::mesh::io::import::deserializers::artifacts::ply::v1_0::any::SemioMeshFromPly;
    use crate::artifacts::semio::standards::v1::subsets::mesh::io::import::deserializers::artifacts::stl::v_ascii::any::SemioMeshFromStl;
    //#endregion 🔖️IoBridgeImports

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("mesh") };

    //#region 🔖️Composer
    pub struct SemioMeshComposerComposition;

    impl ArtifactComposition for SemioMeshComposerComposition {
        type Snapshot = SemioMeshSnapshot;
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
                return Err(ComposeError { message: "SemioMeshComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = SemioMeshAnalyzer::analyze(&native).await;
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "SemioMeshComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ Decodes the payload as this subset's own `SemioMeshSnapshot`, then checks real
    /// referential invariants across its own collections: every `primitive.material_id` (when
    /// `Some`) must resolve to a real entry in `materials`, and mesh/primitive/material/texture ids
    /// must be unique within their own collection (dangling refs + duplicate keys are the two
    /// invariant classes the master plan calls out for subset validators).
    pub struct SemioMeshValidator;

    impl SubsetValidator for SemioMeshValidator {
        const DIALECT: Dialect = DIALECT;
        async fn validate(payload: &IoPayload) -> Vec<dsl::Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <SemioMeshSnapshot as store::ArtifactPack>::decode_pack(bytes).await.ok(),
                IoPayload::Text(text) => <SemioMeshSnapshot as store::ArtifactDsl>::parse_dsl(text).await.ok(),
            };
            match decoded {
                Some(snapshot) => check_mesh_referential_invariants(&snapshot),
                None => vec![dsl::Diagnostic::error("stdio.semio_mesh.validate-decode-failed", dsl::TextSpan::at(1, 1), "SemioMeshValidator: payload did not decode as a SemioMeshSnapshot".to_string())],
            }
        }
    }

    /// 🔗 Real cross-collection referential check, shared by the registered validator above and its
    /// own direct unit tests below.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn check_mesh_referential_invariants(snapshot: &SemioMeshSnapshot) -> Vec<dsl::Diagnostic> {
        let mut diagnostics = Vec::new();

        let mut seen_mesh_ids = std::collections::HashSet::new();
        for mesh in &snapshot.meshes {
            if !seen_mesh_ids.insert(mesh.id.as_str()) {
                diagnostics.push(dsl::Diagnostic::error("stdio.semio_mesh.duplicate-mesh-id", dsl::TextSpan::at(1, 1), format!("SemioMeshValidator: duplicate mesh id {:?}", mesh.id)));
            }
            let mut seen_primitive_ids = std::collections::HashSet::new();
            for primitive in &mesh.primitives {
                if !seen_primitive_ids.insert(primitive.id.as_str()) {
                    diagnostics.push(dsl::Diagnostic::error("stdio.semio_mesh.duplicate-primitive-id", dsl::TextSpan::at(1, 1), format!("SemioMeshValidator: mesh {:?} has duplicate primitive id {:?}", mesh.id, primitive.id)));
                }
                if let Some(material_id) = &primitive.material_id {
                    if !snapshot.materials.iter().any(|m| &m.id == material_id) {
                        diagnostics.push(dsl::Diagnostic::error(
                            "stdio.semio_mesh.dangling-material-ref",
                            dsl::TextSpan::at(1, 1),
                            format!("SemioMeshValidator: mesh {:?} primitive {:?} references missing material {:?}", mesh.id, primitive.id, material_id),
                        ));
                    }
                }
            }
        }

        let mut seen_material_ids = std::collections::HashSet::new();
        for material in &snapshot.materials {
            if !seen_material_ids.insert(material.id.as_str()) {
                diagnostics.push(dsl::Diagnostic::error("stdio.semio_mesh.duplicate-material-id", dsl::TextSpan::at(1, 1), format!("SemioMeshValidator: duplicate material id {:?}", material.id)));
            }
        }

        let mut seen_texture_ids = std::collections::HashSet::new();
        for texture in &snapshot.textures {
            if !seen_texture_ids.insert(texture.id.as_str()) {
                diagnostics.push(dsl::Diagnostic::error("stdio.semio_mesh.duplicate-texture-id", dsl::TextSpan::at(1, 1), format!("SemioMeshValidator: duplicate texture id {:?}", texture.id)));
            }
        }

        diagnostics
    }

    static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioMeshValidator>)
    }
    //#endregion 🔖️SubsetValidator

    //#region 🔖️Register
    /// 📌️ Registers this subset's schema descriptor, document codec, and SubsetValidator. Called from
    /// this artifact's standard-level `engine::register()`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::mesh::schema::semio_mesh_artifact_schema_descriptor());
        let _ = store::register_document_codec(store::ArtifactCodec::of::<SemioMeshSnapshot, crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation>(
            crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::STDIO_SEMIOMESH_DOCUMENT_SCHEMA,
        ));
        let _ = register_subset_validator(validator_entry());
        let _ = register_composer_entries(io_bridge_entries());
        register_artifact_inferences();
    }

    /// 💡️ Registers `s.stdio.semio.mesh.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::schema::register_artifact_inference_descriptor(crate::artifacts::semio::standards::v1::subsets::mesh::schema::inferences::semio_mesh_artifact_inference_descriptor());
    }

    //#region 🔖️IoBridgeEntries
    /// 🌉️ The 5 (format) x 2 (direction) real semio↔format bridges (gltf/stl/obj/ply/las). Each
    /// `deserializer_entry_of`/`serializer_entry_of` row is single-read (`reads: &[FROM]`) and,
    /// via `register_composer_entries`'s own bidirectional insert (one entry -> BOTH
    /// "mesh imports from format" and "format exports to mesh" IoKeys, see that fn's doc comment),
    /// the 10 rows below give all 20 IoKeys (5 formats x 2 directions x 2 perspectives) without
    /// hand-writing each perspective separately.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn io_bridge_entries() -> &'static [ComposerEntry] {
        static ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
        ENTRIES
            .get_or_init(|| {
                vec![
                    deserializer_entry_of::<SemioMeshFromGltf>(),
                    serializer_entry_of::<SemioMeshToGltf>(),
                    deserializer_entry_of::<SemioMeshFromStl>(),
                    serializer_entry_of::<SemioMeshToStl>(),
                    deserializer_entry_of::<SemioMeshFromObj>(),
                    serializer_entry_of::<SemioMeshToObj>(),
                    deserializer_entry_of::<SemioMeshFromPly>(),
                    serializer_entry_of::<SemioMeshToPly>(),
                    deserializer_entry_of::<SemioMeshFromLas>(),
                    serializer_entry_of::<SemioMeshToLas>(),
                ]
            })
            .as_slice()
    }
    //#endregion 🔖️IoBridgeEntries
    //#endregion 🔖️Register

    //#region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::{SemioMaterial, SemioMesh, SemioPrimitive};

        #[semio_framework_async_macros::async_test]
        async fn clean_snapshot_has_no_diagnostics() {
            let snapshot = SemioMeshSnapshot {
                meshes: vec![SemioMesh { id: "m1".into(), primitives: vec![SemioPrimitive { id: "p1".into(), material_id: Some("mat1".into()), ..Default::default() }] }],
                materials: vec![SemioMaterial { id: "mat1".into(), ..Default::default() }],
                ..Default::default()
            };
            assert!(check_mesh_referential_invariants(&snapshot).is_empty());
        }

        #[semio_framework_async_macros::async_test]
        async fn dangling_material_ref_is_flagged() {
            let snapshot = SemioMeshSnapshot { meshes: vec![SemioMesh { id: "m1".into(), primitives: vec![SemioPrimitive { id: "p1".into(), material_id: Some("missing".into()), ..Default::default() }] }], ..Default::default() };
            let diagnostics = check_mesh_referential_invariants(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_mesh.dangling-material-ref"), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn duplicate_ids_are_flagged_per_collection() {
            let snapshot = SemioMeshSnapshot {
                meshes: vec![SemioMesh { id: "dup".into(), primitives: vec![] }, SemioMesh { id: "dup".into(), primitives: vec![] }],
                materials: vec![SemioMaterial { id: "dup".into(), ..Default::default() }, SemioMaterial { id: "dup".into(), ..Default::default() }],
                textures: vec![],
                ..Default::default()
            };
            let diagnostics = check_mesh_referential_invariants(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_mesh.duplicate-mesh-id"), "got {diagnostics:?}");
            assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_mesh.duplicate-material-id"), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn validator_decodes_and_runs_the_referential_checks_end_to_end() {
            let snapshot = SemioMeshSnapshot { meshes: vec![SemioMesh { id: "m1".into(), primitives: vec![SemioPrimitive { id: "p1".into(), material_id: Some("missing".into()), ..Default::default() }] }], ..Default::default() };
            let bytes = store::ArtifactPack::encode_pack(&snapshot);
            let diagnostics = SemioMeshValidator::validate(&IoPayload::Binary(bytes.await));
            assert!(diagnostics.await.iter().any(|d| d.code.0 == "stdio.semio_mesh.dangling-material-ref"), "got {diagnostics:?}");
        }

        //#region 🔖️ConformanceLaws
        /// 🧪️ The 6 real-codec conformance-law tests (ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-
        /// REUSE-EVOLUTION's mesh wave), mirroring `✳️flow`'s own proven, fully-verified template
        /// (`ws-codec-workflow-report.md`) — same 6 test names, same shape, only the facet modules and
        /// demo-case helpers differ.
        mod conformance_laws {
            
            use crate::artifacts::semio::standards::v1::subsets::mesh::schema::{diff, mutations, snapshot};
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
                let text = snapshot::print_mesh_dsl(&snapshot::demo_mesh_snapshot());
                let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
                let reconstructed = format!("{}\n{body}", envelope.envelope_id());
                assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
            }

            /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op` output
            /// for every `SemioMeshMutation` variant (`mutations::demo_mutation_cases()`).
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
            /// for every representative `SemioMeshDiff` (`diff::demo_diff_cases()`), incl. the empty
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
                let packed = snapshot::encode_mesh_pack(&snapshot::demo_mesh_snapshot());
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
            /// `print_dsl`/`encode_pack` output of `engine::demo_mesh_snapshot()` —
            /// `parse_dsl(fixture) == demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the
            /// pack twin — so the fixtures can never silently drift back to a fake.
            #[semio_framework_async_macros::async_test]
            async fn fixture_honesty_law() {
                const FIXTURE_DSL: &str = include_str!("../📚️examples/🧊️cube/🖼️assets/🗣️example.dsl.semio");
                const FIXTURE_PACK: &[u8] = include_bytes!("../📚️examples/🧊️cube/🖼️assets/🎒️example.pack.semio");

                let demo = snapshot::demo_mesh_snapshot();

                let parsed = <snapshot::SemioMeshSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).await.expect("parse shipped .dsl.semio fixture");
                assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_mesh_snapshot()");
                assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_mesh_snapshot()) drifted from the shipped .dsl.semio fixture");

                let decoded = <snapshot::SemioMeshSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).await.expect("decode shipped .pack.semio fixture");
                assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_mesh_snapshot()");
                assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_mesh_snapshot()) drifted from the shipped .pack.semio fixture");
            }
        }
        //#endregion 🔖️ConformanceLaws

        //#region 🧪️SubsetRoundtrip
        struct SemioMeshRoundtrip;

        impl store::os_store::test_support::SubsetRoundtripSpec for SemioMeshRoundtrip {
            type Snapshot = SemioMeshSnapshot;
            type Mutation = crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
            type Inference = ();

            async fn dialect() -> store::os_io::ArtifactDialect {
                store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "mesh".into() }
            }

            async fn fidelity() -> store::os_store::test_support::IoFidelityClass {
                store::os_store::test_support::IoFidelityClass::Canonical
            }

            async fn drops() -> &'static [&'static str] {
                &[]
            }

            async fn parse_native(asset: &store::os_store::test_support::ExampleAsset<'_>) -> Result<Self::Snapshot, String> {
                let text = asset.text.ok_or_else(|| "mesh cube requires dsl text".to_string())?;
                crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::parse_mesh_dsl(text).map_err(|e| e.to_string())
            }

            async fn export_native(snapshot: &Self::Snapshot) -> Result<Vec<u8>, String> {
                Ok(crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::print_mesh_dsl(snapshot).into_bytes())
            }

            async fn reimport_native(bytes: &[u8]) -> Result<Self::Snapshot, String> {
                let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
                crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::parse_mesh_dsl(text).map_err(|e| e.to_string())
            }

            async fn infer(_snapshot: &Self::Snapshot) -> Self::Inference {}

            async fn sample_mutations(_snapshot: &Self::Snapshot) -> Vec<Self::Mutation> {
                // 🔧️ Mechanical fallout from DKM's mesh mutation-vocabulary rewrite: the banned
                // no-op sentinel no longer exists (a mutation with nothing to undo returns
                // `Vec::new()` from `inverse`, no sentinel variant needed — taxonomy.md), so every
                // `demo_mutation_cases()` entry is now a genuine mutation; the filter that used to
                // skip the no-op is no longer expressible (and no longer necessary).
                use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::demo_mutation_cases;
                demo_mutation_cases().into_iter().take(1).collect()
            }

            async fn validate_payload(bytes: &[u8]) -> Result<(), Vec<String>> {
                let text = std::str::from_utf8(bytes).map_err(|e| vec![e.to_string()])?;
                let snapshot = <SemioMeshSnapshot as store::ArtifactDsl>::parse_dsl(text).await.map_err(|e| vec![e.to_string()])?;
                let hard: Vec<String> = check_mesh_referential_invariants(&snapshot).into_iter().filter(|d| matches!(d.severity, dsl::Severity::Error | dsl::Severity::Fatal)).map(|d| d.code.0).collect();
                if hard.is_empty() {
                    Ok(())
                } else {
                    Err(hard)
                }
            }

            async fn validate_negative(_bytes: &[u8]) -> Result<Vec<String>, String> {
                Err("SKIP:owning subset has no negative fixture".into())
            }
        }

        #[semio_framework_async_macros::async_test]
        async fn cube_subset_integrated_roundtrip() {
            let text = include_str!("../📚️examples/🧊️cube/🖼️assets/🗣️example.dsl.semio");
            let asset = store::os_store::test_support::ExampleAsset { bytes: text.as_bytes(), text: Some(text), provenance: "✳️mesh/📚️examples/🧊️cube/🖼️assets/🗣️example.dsl.semio" };
            store::os_store::test_support::assert_subset_roundtrip::<SemioMeshRoundtrip>(&asset, None);
        }
        //#endregion 🧪️SubsetRoundtrip
    }
    //#endregion 🔖️Tests
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
