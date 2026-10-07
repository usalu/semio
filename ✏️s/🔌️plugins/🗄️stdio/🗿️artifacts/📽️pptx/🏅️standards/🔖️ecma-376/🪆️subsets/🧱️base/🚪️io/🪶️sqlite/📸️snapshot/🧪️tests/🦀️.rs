use crate::standards::v_ecma_376::subsets::base::io::sqlite::snapshot::*;
#[path = "💰️backing/🦀️.rs"]
mod owned_requests;
use semio_framework_os_kernel::{sqlite_snapshot::*, ArtifactSqliteSnapshot};
fn profile_fixture(plan: &serde_json::Value) -> PptxSnapshot {
    use semio_s_artifact_stdio_zip::opc::{OpcPackage, OpcRelationship, OpcTargetMode};
    let mut opc = OpcPackage::default();
    opc.relationships
        .replace_owner(String::new(), vec![OpcRelationship { id: "main".into(), rel_type: format!("{}/officeDocument", plan["relationshipBase"].as_str().unwrap()), target: "ppt/presentation.xml".into(), target_mode: OpcTargetMode::Internal }]);
    PptxSnapshot {
        schema: "literal owned schema".into(),
        opc,
        xml_parts: vec![PptxXmlPart {
            path: "ppt/presentation.xml".into(),
            content_type: "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml".into(),
            document: semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::xml_document_from_text(plan["xml"].as_str().unwrap()).unwrap(),
        }],
    }
}
#[test]
fn sqlite_snapshot_pptx_exact_profile_policies_use_typed_root_namespace_entities() {
    let plan: serde_json::Value = serde_json::from_str(include_str!("../../../../🧬️schema/📸️snapshot/🧫️fixtures/🛡️profile/🔣️.json")).unwrap();
    let limits = SqliteDatabaseLimits::default();
    for case in plan["cases"].as_array().unwrap() {
        let snapshot = profile_fixture(case);
        let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.pptx".into(), standard: "ecma-376".into(), subset: case["subset"].as_str().unwrap().into() };
        let db = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
        let result = snapshot.validate_sqlite_snapshot_subset(&dialect, &db, &mut SqliteSnapshotControl::new(&mut |_| true, limits));
        assert_eq!(result.is_ok(), case["codes"].as_array().unwrap().len() == case["warnings"].as_u64().unwrap() as usize, "{}", case["id"]);
        let diagnostics = match result {
            Ok(outcome) => outcome.diagnostics,
            Err(error) => error.diagnostics,
        };
        assert_eq!(diagnostics.iter().map(|item| item.code.0.as_str()).collect::<Vec<_>>(), case["codes"].as_array().unwrap().iter().map(|item| item.as_str().unwrap()).collect::<Vec<_>>(), "{}", case["id"]);
        assert_eq!(diagnostics.iter().filter(|item| item.severity == semio_framework_diagnostic::Severity::Warning).count(), case["warnings"].as_u64().unwrap() as usize);
        snapshot.retire_sqlite_snapshot();
    }
}
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_pptx_actual_exact_profile_declarations_preserve_owned_warnings() {
    use semio_framework_os_kernel::io::io_mechanism::{io_export_sqlite_snapshot, io_import_sqlite_snapshot};
    semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio")
        .label("PPTX exact profiles")
        .version("0.0.1")
        .package_id("semio:stdio")
        .artifact(crate::declaration(crate::definition().unwrap()).unwrap())
        .try_build()
        .unwrap();
    let plan: serde_json::Value = serde_json::from_str(include_str!("../../../../🧬️schema/📸️snapshot/🧫️fixtures/🛡️profile/🔣️.json")).unwrap();
    let limits = SqliteDatabaseLimits::default();
    for case in plan["cases"].as_array().unwrap().iter().filter(|case| case["codes"].as_array().unwrap().len() == case["warnings"].as_u64().unwrap() as usize) {
        let snapshot = profile_fixture(case);
        let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.pptx".into(), standard: "ecma-376".into(), subset: case["subset"].as_str().unwrap().into() };
        for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
            let exported = io_export_sqlite_snapshot(&dialect, &snapshot, encoding, limits, &mut |_| true).await.unwrap();
            assert_eq!(exported.diagnostics.len(), case["warnings"].as_u64().unwrap() as usize);
            let restored = io_import_sqlite_snapshot::<PptxSnapshot>(&dialect, &exported.value, limits, &mut |_| true).await.unwrap();
            assert_eq!(restored.value, snapshot);
            assert_eq!(restored.diagnostics.len(), exported.diagnostics.len());
            let foreign = semio_framework_artifact_reference::ArtifactDialect { subset: if dialect.subset == "strict" { "transitional".into() } else { "strict".into() }, ..dialect.clone() };
            assert!(io_import_sqlite_snapshot::<PptxSnapshot>(&foreign, &exported.value, limits, &mut |_| true).await.is_err());
            restored.value.retire_sqlite_snapshot();
        }
        snapshot.retire_sqlite_snapshot();
    }
}
#[test]
fn sqlite_snapshot_pptx_native_grammar_describes_complete_owned_fields() {
    let snapshot = fixture();
    let text = <PptxSnapshot as store::ArtifactDsl>::print_dsl(&snapshot);
    let (envelope, body) = store::semio_format::split_text_preamble(&text).unwrap();
    let grammar = semio_framework_dsl::parse_grammar(crate::standards::v_ecma_376::subsets::base::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO).unwrap();
    assert!(semio_framework_dsl::Recognizer::compile(&grammar, &semio_framework_os_kernel::os_dsl::grammar::family_fragments().expect("OS family grammar"), semio_framework_os_kernel::os_dsl::grammar::product_macros())
        .expect("selected grammar fragments")
        .recognize(&format!("{}\n{body}", envelope.envelope_id()))
        .unwrap());
    snapshot.retire_sqlite_snapshot();
}
#[test]
fn sqlite_snapshot_pptx_native_protocol_checks_complete_literal_fields() {
    let snapshot = fixture();
    let bytes = <PptxSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
    let (_, body) = store::semio_format::unwrap_binary(&bytes).unwrap();
    let protocol = semio_framework_dsl::parse_protocol(crate::standards::v_ecma_376::subsets::base::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO).unwrap();
    assert_eq!(semio_framework_dsl::walk_protocol(&protocol, &body).unwrap().consumed, body.len());
    for length in [0, 1, body.len() - 1] {
        assert!(semio_framework_dsl::walk_protocol(&protocol, &body[..length]).is_err());
    }
    let mut trailing = body.clone();
    trailing.push(0);
    assert!(semio_framework_dsl::walk_protocol(&protocol, &trailing).is_err());
    snapshot.retire_sqlite_snapshot();
}
fn fixture() -> PptxSnapshot {
    semio_framework_pack_json::from_json_str(include_str!("../🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()
}
#[test]
fn sqlite_snapshot_pptx_all_owned_fields_and_literal_package_domains() {
    let snapshot = fixture();
    let limits = SqliteDatabaseLimits::default();
    let db = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
    let restored = PptxSnapshot::from_sqlite_database(&db, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
    assert_eq!(snapshot, restored);
    assert_eq!(db.tables.len(), 21);
}
#[test]
fn sqlite_snapshot_pptx_exact_aggregate_row_budget() {
    let snapshot = fixture();
    let defaults = SqliteDatabaseLimits::default();
    let db = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, defaults)).unwrap();
    let count = db.tables.iter().map(|table| table.rows.len()).sum::<usize>();
    let exact = SqliteDatabaseLimits { max_rows: count, ..defaults };
    assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, exact)).is_ok());
    assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits { max_rows: count - 1, ..defaults })).is_err());
}

#[test]
fn sqlite_snapshot_pptx_rejects_dangling_and_shared_package_owners() {
    let snapshot = fixture();
    let limits = SqliteDatabaseLimits::default();
    let db = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
    for (table, row, column, value) in [
        ("pptx_document", 0, 2, SqliteValue::Integer(2)),
        ("pptx_xml_part", 1, 4, SqliteValue::Integer(1)),
        ("pptx_relationship", 0, 1, SqliteValue::Integer(99)),
        ("pptx_default_content_type", 0, 2, SqliteValue::Integer(99)),
        ("pptx_relationship_owner", 1, 2, SqliteValue::Text("".into())),
    ] {
        let mut invalid = db.clone();
        invalid.table_mut(table).unwrap().rows[row].values[column] = value;
        assert!(PptxSnapshot::from_sqlite_database(&invalid, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err(), "{table}");
    }
}
#[test]
fn sqlite_snapshot_pptx_literal_large_part_bytes_have_interior_controls() {
    let mut snapshot = fixture();
    snapshot.opc.parts[0].bytes = vec![0x97; 100000];
    let limits = SqliteDatabaseLimits::default();
    let db = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
    for phase in [SqliteSnapshotPhase::ProjectSnapshot, SqliteSnapshotPhase::ReconstructSnapshot] {
        let mut observed = false;
        let mut callback = |event: SqliteSnapshotProgress| {
            if event.phase == phase && event.total == 100000 && event.completed >= 65536 && event.completed < event.total {
                observed = true;
                false
            } else {
                true
            }
        };
        let mut control = SqliteSnapshotControl::new(&mut callback, limits);
        let failed = if phase == SqliteSnapshotPhase::ProjectSnapshot { snapshot.to_sqlite_database(&mut control).is_err() } else { PptxSnapshot::from_sqlite_database(&db, &mut control).is_err() };
        assert!(failed);
        assert!(observed);
    }
}
#[test]
fn sqlite_snapshot_pptx_partial_part_reconstruction_retires_deep_completed_documents() {
    std::thread::Builder::new()
        .stack_size(256 * 1024)
        .spawn(|| {
            struct Owned(Option<PptxSnapshot>);
            impl Drop for Owned {
                fn drop(&mut self) {
                    if let Some(snapshot) = self.0.take() {
                        snapshot.retire_sqlite_snapshot();
                    }
                }
            }
            let mut snapshot = fixture();
            let mut node = semio_s_artifact_stdio_xml::schema::snapshot::XmlNode::Text { text: "leaf".into() };
            for _ in 0..8192 {
                node = semio_s_artifact_stdio_xml::schema::snapshot::XmlNode::Element { name: "node".into(), attrs: Vec::new(), children: vec![node] };
            }
            snapshot.xml_parts[0].document.root = Some(node);
            snapshot.xml_parts[1].content_type = "z".repeat(100000);
            let snapshot = Owned(Some(snapshot));
            let limits = SqliteDatabaseLimits::default();
            let db = snapshot.0.as_ref().unwrap().to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
            let mut observed = false;
            assert!(PptxSnapshot::from_sqlite_database(
                &db,
                &mut SqliteSnapshotControl::new(
                    &mut |event| {
                        if event.phase == SqliteSnapshotPhase::ReconstructSnapshot && event.total == 100000 && event.completed >= 65536 && event.completed < event.total {
                            observed = true;
                            false
                        } else {
                            true
                        }
                    },
                    limits
                )
            )
            .is_err());
            assert!(observed);
            let restored = PptxSnapshot::from_sqlite_database(&db, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
            restored.retire_sqlite_snapshot();
        })
        .unwrap()
        .join()
        .unwrap();
}
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_pptx_actual_declaration_exposes_complete_typed_snapshot() {
    use {semio_framework_os_kernel::io::io_mechanism::io_export_sqlite_snapshot,semio_framework_os_kernel::io::io_mechanism::io_import_sqlite_snapshot,semio_framework_artifact_reference::ArtifactDialect};
    semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("PPTX SQLite").version("0.0.1").package_id("semio:stdio").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();
    let dialect = ArtifactDialect { artifact_kind: "s.stdio.pptx".into(), standard: "ecma-376".into(), subset: "*".into() };
    let snapshot = fixture();
    let file = io_export_sqlite_snapshot(&dialect, &snapshot, SnapshotEncoding::Binary, SqliteDatabaseLimits::default(), &mut |_| true).await.unwrap().value;
    let restored = io_import_sqlite_snapshot::<PptxSnapshot>(&dialect, &file, SqliteDatabaseLimits::default(), &mut |_| true).await.unwrap().value;
    assert_eq!(restored, snapshot);
    restored.retire_sqlite_snapshot();
    snapshot.retire_sqlite_snapshot();
}

#[test]
fn sqlite_snapshot_pptx_complete_signed64_transform_domain() {
    let mut snapshot = fixture();
    use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlNode};
    let values = [("x", i64::MIN), ("y", i64::MAX), ("cx", 9007199254740993), ("cy", -9007199254740993)];
    snapshot.xml_parts[0].document.root = Some(XmlNode::Element { name: "a:xfrm".into(), attrs: values.iter().map(|(name, value)| XmlAttr { name: (*name).into(), value: value.to_string() }).collect(), children: vec![] });
    let limits = SqliteDatabaseLimits::default();
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
    let node = database.table("pptx_xml_element").unwrap().rows.iter().find(|row| row.text(1).unwrap() == "a:xfrm").unwrap().rowid;
    let attrs = database.table("pptx_xml_attribute").unwrap().rows.iter().filter(|row| row.integer(1).unwrap() == node).collect::<Vec<_>>();
    assert_eq!(attrs.len(), values.len());
    for (row, (name, value)) in attrs.iter().zip(values) {
        assert_eq!(row.text(3).unwrap(), name);
        assert_eq!(row.text(4).unwrap(), value.to_string());
    }
    let restored = PptxSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
    assert_eq!(restored, snapshot);
    restored.retire_sqlite_snapshot();
    snapshot.retire_sqlite_snapshot();
}

#[test]
fn sqlite_snapshot_pptx_actual_erased_native_boundaries_retain_all_owned_fields() {
    let snapshot = fixture();
    let limits = SqliteDatabaseLimits::default();
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.pptx".into(), standard: "ecma-376".into(), subset: "*".into() };
    let codec = <PptxSnapshot as ArtifactSqliteSnapshot>::sqlite_codec();
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let db = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
        let payload = (codec.import)(crate::STDIO_PPTX_DOCUMENT_SCHEMA, &dialect, db, encoding, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap().value;
        let projected = (codec.export)(crate::STDIO_PPTX_DOCUMENT_SCHEMA, &dialect, &payload, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap().value;
        let restored = PptxSnapshot::from_sqlite_database(&projected, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
        assert_eq!(restored, snapshot);
        restored.retire_sqlite_snapshot();
    }
    snapshot.retire_sqlite_snapshot();
}
#[test]
fn sqlite_snapshot_pptx_deep_erased_native_input_output_are_interior_cancellable() {
    let plan: serde_json::Value = serde_json::from_str(include_str!("../../../../🧬️schema/📸️snapshot/🧫️fixtures/🛫️native/🔣️.json")).unwrap();
    std::thread::Builder::new()
        .stack_size(plan["smallStackBytes"].as_u64().unwrap() as usize)
        .spawn(move || {
            struct Owner(Option<PptxSnapshot>);
            impl Drop for Owner {
                fn drop(&mut self) {
                    if let Some(snapshot) = self.0.take() {
                        snapshot.retire_sqlite_snapshot();
                    }
                }
            }
            let mut snapshot = fixture();
            let mut node = semio_s_artifact_stdio_xml::schema::snapshot::XmlNode::Text { text: "leaf".into() };
            for _ in 0..plan["depth"].as_u64().unwrap() {
                node = semio_s_artifact_stdio_xml::schema::snapshot::XmlNode::Element { name: "node".into(), attrs: vec![], children: vec![node] };
            }
            snapshot.xml_parts[0].document.root = Some(node);
            let mut other = semio_s_artifact_stdio_xml::schema::snapshot::XmlNode::Text { text: "other leaf".into() };
            for _ in 0..plan["depth"].as_u64().unwrap() {
                other = semio_s_artifact_stdio_xml::schema::snapshot::XmlNode::Element { name: "other node".into(), attrs: vec![], children: vec![other] };
            }
            snapshot.xml_parts[1].document.root = Some(other);
            snapshot.xml_parts[1].content_type = "z".repeat(plan["lateTextBytes"].as_u64().unwrap() as usize);
            let snapshot = Owner(Some(snapshot));
            let limits = SqliteDatabaseLimits::default();
            let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.pptx".into(), standard: "ecma-376".into(), subset: "*".into() };
            let codec = <PptxSnapshot as ArtifactSqliteSnapshot>::sqlite_codec();
            for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
                let db = snapshot.0.as_ref().unwrap().to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
                let expected_database = db.clone();
                let payload = (codec.import)(crate::STDIO_PPTX_DOCUMENT_SCHEMA, &dialect, db, encoding, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap().value;
                let projected = (codec.export)(crate::STDIO_PPTX_DOCUMENT_SCHEMA, &dialect, &payload, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap().value;
                assert!(projected == expected_database, "complete deep native graph differs");
                assert_eq!(projected.table("pptx_xml_element").unwrap().rows.iter().filter(|row| row.text(1).unwrap() == "node").count(), plan["depth"].as_u64().unwrap() as usize);
                assert_eq!(projected.table("pptx_xml_text").unwrap().rows.iter().filter(|row| row.text(1).unwrap() == "leaf").count(), 1);
                assert_eq!(projected.table("pptx_xml_element").unwrap().rows.iter().filter(|row| row.text(1).unwrap() == "other node").count(), plan["depth"].as_u64().unwrap() as usize);
                assert_eq!(projected.table("pptx_xml_text").unwrap().rows.iter().filter(|row| row.text(1).unwrap() == "other leaf").count(), 1);
                for phase in [SqliteSnapshotPhase::EncodeNative, SqliteSnapshotPhase::DecodeNative] {
                    let mut reached = false;
                    let mut callback = |event: SqliteSnapshotProgress| {
                        if event.phase == phase && event.completed >= plan["cancelAfter"].as_u64().unwrap() as usize && event.completed < event.total {
                            reached = true;
                            false
                        } else {
                            true
                        }
                    };
                    let failed = if phase == SqliteSnapshotPhase::EncodeNative {
                        let db = snapshot.0.as_ref().unwrap().to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
                        (codec.import)(crate::STDIO_PPTX_DOCUMENT_SCHEMA, &dialect, db, encoding, &mut SqliteSnapshotControl::new(&mut callback, limits)).is_err()
                    } else {
                        (codec.export)(crate::STDIO_PPTX_DOCUMENT_SCHEMA, &dialect, &payload, &mut SqliteSnapshotControl::new(&mut callback, limits)).is_err()
                    };
                    assert!(failed);
                    assert!(reached, "actual interior native phase {phase:?}");
                }
            }
        })
        .unwrap()
        .join()
        .unwrap();
}

#[test]
fn sqlite_snapshot_pptx_native_exact_file_and_entity_admission() {
    let snapshot = fixture();
    let defaults = SqliteDatabaseLimits::default();
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, defaults)).unwrap();
    let rows = database.tables.iter().map(|table| table.rows.len()).sum::<usize>();
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let payload = snapshot.encode_sqlite_snapshot_native(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, defaults)).unwrap();
        let bytes = match &payload {
            store::io_schema::IoPayload::Binary(bytes) => bytes.len(),
            store::io_schema::IoPayload::Text(text) => text.len(),
        };
        let exact = SqliteDatabaseLimits { max_file_bytes: bytes, max_rows: rows, ..defaults };
        assert_eq!(snapshot.encode_sqlite_snapshot_native(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, exact)).unwrap(), payload);
        let restored = PptxSnapshot::decode_sqlite_snapshot_native(&payload, &mut SqliteSnapshotControl::new(&mut |_| true, exact)).unwrap();
        assert_eq!(restored, snapshot);
        restored.retire_sqlite_snapshot();
        for limited in [SqliteDatabaseLimits { max_file_bytes: bytes - 1, ..exact }, SqliteDatabaseLimits { max_rows: rows - 1, ..exact }] {
            assert!(snapshot.encode_sqlite_snapshot_native(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, limited)).is_err());
            assert!(PptxSnapshot::decode_sqlite_snapshot_native(&payload, &mut SqliteSnapshotControl::new(&mut |_| true, limited)).is_err());
        }
    }
    snapshot.retire_sqlite_snapshot();
}

#[test]
fn sqlite_snapshot_pptx_complete_literal_native_file_matches_independent_fixture() {
    let plan: serde_json::Value = serde_json::from_str(include_str!("../../../../🧬️schema/📸️snapshot/🧫️fixtures/🧩️native-literals/🔣️.json")).unwrap();
    let snapshot: PptxSnapshot = semio_framework_pack_json::from_json_str(&plan["snapshot"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let defaults = SqliteDatabaseLimits::default();
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let expected = if encoding == SnapshotEncoding::Binary {
            store::io_schema::IoPayload::Binary(plan["binary"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u8).collect())
        } else {
            store::io_schema::IoPayload::Text(plan["text"].as_str().unwrap().into())
        };
        let file_size = match &expected {
            store::io_schema::IoPayload::Binary(bytes) => bytes.len(),
            store::io_schema::IoPayload::Text(text) => text.len(),
        };
        let limits = SqliteDatabaseLimits { max_file_bytes: file_size, max_rows: plan["rows"].as_u64().unwrap() as usize, ..defaults };
        assert_eq!(snapshot.encode_sqlite_snapshot_native(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap(), expected);
        let restored = PptxSnapshot::decode_sqlite_snapshot_native(&expected, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
        assert_eq!(restored, snapshot);
        restored.retire_sqlite_snapshot();
    }
    snapshot.retire_sqlite_snapshot();
}

#[test]
fn sqlite_snapshot_pptx_borrowed_preflight_pays_frontiers_without_materializing_output() {
    let snapshot = fixture();
    let limits = SqliteDatabaseLimits::default();
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let mut callback = |_| true;
        let mut control = SqliteSnapshotControl::new(&mut callback, limits);
        snapshot.preflight_sqlite_snapshot_encoding(encoding, &mut control).expect("actual complete owner must expose borrowed preflight");
        let error = snapshot.preflight_sqlite_snapshot_encoding(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits { max_allocation_bytes: 1, ..limits })).unwrap_err();
        assert_eq!(error.kind, semio_framework_value::ValueRefusalKind::OwnershipLimit);
        let error = snapshot.preflight_sqlite_snapshot_encoding(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits { max_rows: 0, ..limits })).unwrap_err();
        assert_eq!(error.kind, semio_framework_value::ValueRefusalKind::WorkLimit);
        let mut reached = false;
        let mut callback = |event: SqliteSnapshotProgress| {
            let cancel = event.phase == SqliteSnapshotPhase::EncodeNative;
            reached |= cancel;
            !cancel
        };
        let error = snapshot.preflight_sqlite_snapshot_encoding(encoding, &mut SqliteSnapshotControl::new(&mut callback, limits)).unwrap_err();
        assert_eq!(error.kind, semio_framework_value::ValueRefusalKind::Canceled);
        assert!(reached);
        let payload = snapshot.encode_sqlite_snapshot_native(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
        let error = snapshot.encode_sqlite_snapshot_native(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits { max_allocation_bytes: 1, ..limits })).unwrap_err();
        assert_eq!(error.kind, semio_framework_value::ValueRefusalKind::OwnershipLimit);
        let error = <PptxSnapshot as ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits { max_allocation_bytes: 1, ..limits })).unwrap_err();
        assert_eq!(error.kind, semio_framework_value::ValueRefusalKind::OwnershipLimit);
    }
    snapshot.retire_sqlite_snapshot();
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_pptx_strict_initial_has_independent_complete_package_and_public_files() {
    use semio_framework_os_kernel::io::io_mechanism::{io_export_sqlite_snapshot,io_import_sqlite_snapshot};
    fn independent_package(bytes: &[u8]) -> serde_json::Value {
        use std::collections::{BTreeMap, BTreeSet};
        use std::io::Read;
        use quick_xml::{events::Event, Reader};
        fn attributes(element: &quick_xml::events::BytesStart<'_>) -> BTreeMap<String, String> {
            let mut values = BTreeMap::new();
            for attribute in element.attributes() {
                let attribute = attribute.unwrap();
                let key = attribute.key.as_ref().to_owned();
                let value = attribute.normalized_value(quick_xml::XmlVersion::Implicit1_0).unwrap().into_owned();
                assert!(values.insert(key, value).is_none());
            }
            values
        }
        let mut zip = zip::ZipArchive::new(std::io::Cursor::new(bytes)).unwrap();
        let names = zip.file_names().map(str::to_owned).collect::<BTreeSet<_>>();
        let mut files = BTreeMap::new();
        for name in &names {
            let mut body = Vec::new();
            zip.by_name(name).unwrap().read_to_end(&mut body).unwrap();
            assert!(files.insert(name.clone(), body).is_none());
        }
        let mut defaults = BTreeMap::new();
        let mut overrides = BTreeMap::new();
        let mut reader = Reader::from_reader(files["[Content_Types].xml"].as_slice());
        loop {
            match reader.read_event().unwrap() {
                Event::Start(element) | Event::Empty(element) => {
                    let values = attributes(&element);
                    match element.local_name().as_ref() {
                        "Default" => { assert!(defaults.insert(values["Extension"].clone(), values["ContentType"].clone()).is_none()); }
                        "Override" => { assert!(overrides.insert(values["PartName"].trim_start_matches('/').to_owned(), values["ContentType"].clone()).is_none()); }
                        _ => {}
                    }
                }
                Event::Eof => break,
                _ => {}
            }
        }
        let graph = independent_relationships(bytes);
        let office = graph.as_array().unwrap().iter().filter(|edge| edge["source"] == "package" && edge["type"].as_str().unwrap().ends_with("/officeDocument")).collect::<Vec<_>>();
        assert_eq!(office.len(), 1);
        let main = office[0]["target"].as_str().unwrap();
        let mut root_name = None;
        let mut root_attributes = Vec::new();
        let mut namespaces = BTreeSet::new();
        let mut alternate = BTreeSet::new();
        for (name, body) in files.iter().filter(|(name, _)| name.ends_with(".xml") && name.as_str() != "[Content_Types].xml") {
            let mut reader = Reader::from_reader(body.as_slice());
            let mut first = true;
            loop {
                match reader.read_event().unwrap() {
                    Event::Start(element) | Event::Empty(element) => {
                        let values = attributes(&element);
                        if first && name == main {
                            root_name = Some(element.name().as_ref().to_owned());
                            root_attributes = values.iter().map(|(name, value)| serde_json::json!({"name":name,"value":value})).collect();
                        }
                        first = false;
                        for (key, value) in values {
                            if key == "xmlns" || key.starts_with("xmlns:") { namespaces.insert(value); }
                        }
                        if element.local_name().as_ref() == "AlternateContent" { alternate.insert(name.clone()); }
                    }
                    Event::Eof => break,
                    _ => {}
                }
            }
        }
        let parts = names.iter().map(|path| {
            let extension = path.rsplit_once('.').unwrap().1;
            let content_type = overrides.get(path).or_else(|| defaults.get(extension)).expect("independently declared part type");
            serde_json::json!({"path":path,"contentType":content_type})
        }).collect::<Vec<_>>();
        let kinds = graph.as_array().unwrap().iter().map(|edge| edge["type"].as_str().unwrap()).collect::<BTreeSet<_>>();
        serde_json::json!({"format":"pptx","mainPart":main,"mainRootName":root_name.unwrap(),"mainRootAttributes":root_attributes,"parts":parts,"namespaces":namespaces,"relationshipTypes":kinds,"alternateContentParts":alternate})
    }
    fn independent_relationships(bytes: &[u8]) -> serde_json::Value {
        use std::collections::{BTreeMap, BTreeSet};
        use std::io::Read;
        use quick_xml::{events::Event, Reader};
        let mut zip = zip::ZipArchive::new(std::io::Cursor::new(bytes)).unwrap();
        let names = zip.file_names().map(str::to_owned).collect::<BTreeSet<_>>();
        let mut relationships = Vec::new();
        let mut identities = BTreeSet::new();
        for path in names.iter().filter(|name| name.ends_with(".rels")) {
            let source = if path == "_rels/.rels" { "package".to_owned() } else {
                let (directory, file) = path.rsplit_once("/_rels/").expect("owned relationship part");
                format!("{directory}/{}", file.strip_suffix(".rels").unwrap())
            };
            assert!(source == "package" || names.contains(&source));
            let mut xml = Vec::new();
            zip.by_name(path).unwrap().read_to_end(&mut xml).unwrap();
            let mut reader = Reader::from_reader(xml.as_slice());
            loop {
                match reader.read_event().unwrap() {
                    Event::Start(element) | Event::Empty(element) if element.local_name().as_ref() == "Relationship" => {
                        let mut attributes = BTreeMap::new();
                        for attribute in element.attributes() {
                            let attribute = attribute.unwrap();
                            let key = attribute.key.as_ref().to_owned();
                            let value = attribute.normalized_value(quick_xml::XmlVersion::Implicit1_0).unwrap().into_owned();
                            assert!(attributes.insert(key, value).is_none());
                        }
                        assert!(attributes.get("TargetMode").is_none_or(|mode| mode == "Internal"));
                        let id = &attributes["Id"];
                        assert!(!id.is_empty() && identities.insert((source.clone(), id.clone())));
                        let kind = &attributes["Type"];
                        assert!(kind.starts_with("http://purl.oclc.org/ooxml/officeDocument/relationships/"));
                        let target = &attributes["Target"];
                        let mut resolved = if source == "package" || target.starts_with('/') {
                            Vec::new()
                        } else {
                            source.rsplit_once('/').map_or_else(Vec::new, |(parent, _)| parent.split('/').collect::<Vec<_>>())
                        };
                        for component in target.split('/') {
                            match component {
                                "" | "." => {}
                                ".." => { assert!(resolved.pop().is_some()); }
                                component => resolved.push(component),
                            }
                        }
                        let target = resolved.join("/");
                        assert!(names.contains(&target), "missing independently resolved target {target}");
                        relationships.push(serde_json::json!({"source":source,"id":id,"type":kind,"target":target}));
                    }
                    Event::Eof => break,
                    _ => {}
                }
            }
        }
        relationships.sort_by(|left, right| (left["source"].as_str(), left["id"].as_str()).cmp(&(right["source"].as_str(), right["id"].as_str())));
        serde_json::Value::Array(relationships)
    }
    let contract: serde_json::Value = serde_json::from_str(include_str!("../../../../🧬️schema/📸️snapshot/🧫️fixtures/🔒️strict-initial/🔣️.json")).unwrap();
    let mut expected_edges = contract["relationships"].as_array().unwrap().clone();
    expected_edges.sort_by(|left,right| (left["source"].as_str(),left["id"].as_str()).cmp(&(right["source"].as_str(),right["id"].as_str())));
    let expected_edges = serde_json::Value::Array(expected_edges);
    let owner = crate::standards::v_ecma_376::subsets::strict::schema::blank_strict_pptx_snapshot();
    let native = crate::standards::v_ecma_376::subsets::base::io::export::serializers::encode_pptx(&owner).unwrap();
    assert_eq!(independent_package(&native),contract["projection"]);
    assert_eq!(independent_relationships(&native),expected_edges);
    eprintln!("[DEBUG] strict PPTX initial independent ZIP/QuickXML parts=9 resolved_relationships=5");
    semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("PPTX SQLite").version("0.0.1").package_id("semio:stdio").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind:"s.stdio.pptx".into(),standard:"ecma-376".into(),subset:"strict".into() };
    for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text] {
        let limits=SqliteDatabaseLimits::default();
        let bytes=io_export_sqlite_snapshot(&dialect,&owner,encoding,limits,&mut |_|true).await.unwrap().value;
        let restored=io_import_sqlite_snapshot::<PptxSnapshot>(&dialect,&bytes,limits,&mut |_|true).await.unwrap().value;
        assert_eq!(restored,owner);
        let native_back=crate::standards::v_ecma_376::subsets::base::io::export::serializers::encode_pptx(&restored).unwrap();
        assert_eq!(independent_package(&native_back),contract["projection"]);
        assert_eq!(independent_relationships(&native_back),expected_edges);
        eprintln!("[DEBUG] strict PPTX initial public SQLite native={encoding:?} full_owner_equal=true independent_relationships=5");
        restored.retire_sqlite_snapshot();
    }
    owner.retire_sqlite_snapshot();
}
