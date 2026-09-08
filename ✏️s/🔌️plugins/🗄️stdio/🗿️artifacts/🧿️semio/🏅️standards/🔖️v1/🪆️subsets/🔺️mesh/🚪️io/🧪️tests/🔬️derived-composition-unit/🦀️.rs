mod tests {
    use super::*;
    use crate::standards::v1::subsets::mesh::schema::snapshot::{SemioMaterial, SemioMesh, SemioPrimitive};

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
        let diagnostics = SemioMeshValidator::validate(&IoPayload::Binary(bytes)).await;
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_mesh.dangling-material-ref"), "got {diagnostics:?}");
    }

    //#region 🔖️ConformanceLaws
    /// 🧪️ The 6 real-codec conformance-law tests (ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-
    /// REUSE-EVOLUTION's mesh wave), mirroring `🌊️flow`'s own proven, fully-verified template
    /// (`ws-codec-workflow-report.md`) — same 6 test names, same shape, only the facet modules and
    /// demo-case helpers differ.
    mod conformance_laws {

        use crate::standards::v1::subsets::mesh::schema::{diff, mutations, snapshot};
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
        /// `print_dsl`/`encode_pack` output of `engine::demo_mesh_snapshot()` —
        /// `parse_dsl(fixture) == demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the
        /// pack twin — so the fixtures can never silently drift back to a fake.
        #[semio_framework_async_macros::async_test]
        async fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../../../📚️examples/🧊️cube/🖼️assets/🗣️.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../../../📚️examples/🧊️cube/🖼️assets/🎒️.pack.semio");

            let demo = snapshot::demo_mesh_snapshot();

            let parsed = <snapshot::SemioMeshSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
            assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_mesh_snapshot()");
            assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_mesh_snapshot()) drifted from the shipped .dsl.semio fixture");

            let decoded = <snapshot::SemioMeshSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
            assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_mesh_snapshot()");
            assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_mesh_snapshot()) drifted from the shipped .pack.semio fixture");
        }
    }
    //#endregion 🔖️ConformanceLaws

    //#region 🧪️SubsetRoundtrip
    struct SemioMeshRoundtrip;

    impl store::os_store::test_support::SubsetRoundtripSpec for SemioMeshRoundtrip {
        type Snapshot = SemioMeshSnapshot;
        type Mutation = crate::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
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
            crate::standards::v1::subsets::mesh::schema::snapshot::parse_mesh_dsl(text).map_err(|e| e.to_string())
        }

        async fn export_native(snapshot: &Self::Snapshot) -> Result<Vec<u8>, String> {
            Ok(crate::standards::v1::subsets::mesh::schema::snapshot::print_mesh_dsl(snapshot).into_bytes())
        }

        async fn reimport_native(bytes: &[u8]) -> Result<Self::Snapshot, String> {
            let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
            crate::standards::v1::subsets::mesh::schema::snapshot::parse_mesh_dsl(text).map_err(|e| e.to_string())
        }

        async fn infer(_snapshot: &Self::Snapshot) -> Self::Inference {}

        async fn sample_mutations(_snapshot: &Self::Snapshot) -> Vec<Self::Mutation> {
            // 🔧️ Mechanical fallout from DKM's mesh mutation-vocabulary rewrite: the banned
            // no-op sentinel no longer exists (a mutation with nothing to undo returns
            // `Vec::new()` from `inverse`, no sentinel variant needed — taxonomy.md), so every
            // `demo_mutation_cases()` entry is now a genuine mutation; the filter that used to
            // skip the no-op is no longer expressible (and no longer necessary).
            use crate::standards::v1::subsets::mesh::schema::mutations::demo_mutation_cases;
            demo_mutation_cases().into_iter().take(1).collect()
        }

        async fn validate_payload(bytes: &[u8]) -> Result<(), Vec<String>> {
            let text = std::str::from_utf8(bytes).map_err(|e| vec![e.to_string()])?;
            let snapshot = <SemioMeshSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|e| vec![e.to_string()])?;
            let hard: Vec<String> = check_mesh_referential_invariants(&snapshot).into_iter().filter(|d| matches!(d.severity, dsl::Severity::Error | dsl::Severity::Fatal)).map(|d| d.code.0).collect();
            if hard.is_empty() { Ok(()) } else { Err(hard) }
        }

        async fn validate_negative(_bytes: &[u8]) -> Result<Vec<String>, String> {
            Err("SKIP:owning subset has no negative fixture".into())
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn cube_subset_integrated_roundtrip() {
        let text = include_str!("../../../📚️examples/🧊️cube/🖼️assets/🗣️.dsl.semio");
        let asset = store::os_store::test_support::ExampleAsset { bytes: text.as_bytes(), text: Some(text), provenance: "🔺️mesh/📚️examples/🧊️cube/🖼️assets/🗣️.dsl.semio" };
        store::os_store::test_support::assert_subset_roundtrip::<SemioMeshRoundtrip>(&asset, None).await;
    }
    //#endregion 🧪️SubsetRoundtrip
}
