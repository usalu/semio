use super::*;
#[path = "💰️backing/🦀️.rs"]
mod owned_requests;
use semio_framework_os_kernel::{sqlite_snapshot::*, ArtifactSqliteSnapshot};
fn fixture() -> XlsxSnapshot {
    semio_framework_pack_json::from_json_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()
}
#[test]
fn sqlite_snapshot_xlsx_all_owned_fields_and_literal_package_domains() {
    let snapshot = fixture();
    let limits = SqliteDatabaseLimits::default();
    let db = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
    let restored = XlsxSnapshot::from_sqlite_database(&db, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
    assert_eq!(snapshot, restored);
    assert_eq!(db.tables.len(), 21);
}
#[test]
fn sqlite_snapshot_xlsx_exact_aggregate_row_budget() {
    let snapshot = fixture();
    let defaults = SqliteDatabaseLimits::default();
    let db = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, defaults)).unwrap();
    let count = db.tables.iter().map(|table| table.rows.len()).sum::<usize>();
    let exact = SqliteDatabaseLimits { max_rows: count, ..defaults };
    assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, exact)).is_ok());
    assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits { max_rows: count - 1, ..defaults })).is_err());
}

#[test]
fn sqlite_snapshot_xlsx_components_advance_one_owned_projection_ledger() {
    let snapshot = fixture();
    let defaults = SqliteDatabaseLimits::default();
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, defaults)).unwrap();
    let rows = database.tables.iter().map(|table| table.rows.len()).sum::<usize>();
    let mut owned_progress = Vec::new();
    snapshot
        .to_sqlite_database(&mut SqliteSnapshotControl::new(
            &mut |event| {
                if event.phase == SqliteSnapshotPhase::ProjectSnapshot && event.total == 0 && event.completed > 0 {
                    owned_progress.push(event.completed);
                }
                true
            },
            SqliteDatabaseLimits { max_rows: rows, ..defaults },
        ))
        .unwrap();
    let mut next = 1;
    for completed in owned_progress {
        if completed == next {
            next += 1;
        } else if completed == 1 {
            next = 2;
        }
        if next == rows + 1 {
            break;
        }
    }
    assert_eq!(next, rows + 1, "OPC, XML, and XLSX owner rows must advance one cumulative Projection ledger");
}

#[test]
fn sqlite_snapshot_xlsx_rejects_dangling_and_shared_package_owners() {
    let snapshot = fixture();
    let limits = SqliteDatabaseLimits::default();
    let db = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
    for (table, row, column, value) in [
        ("xlsx_document", 0, 2, SqliteValue::Integer(2)),
        ("xlsx_xml_part", 1, 4, SqliteValue::Integer(1)),
        ("xlsx_relationship", 0, 1, SqliteValue::Integer(99)),
        ("xlsx_default_content_type", 0, 2, SqliteValue::Integer(99)),
        ("xlsx_relationship_owner", 1, 2, SqliteValue::Text("".into())),
    ] {
        let mut invalid = db.clone();
        invalid.table_mut(table).unwrap().rows[row].values[column] = value;
        assert!(XlsxSnapshot::from_sqlite_database(&invalid, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err(), "{table}");
    }
}
#[test]
fn sqlite_snapshot_xlsx_literal_large_part_bytes_have_interior_controls() {
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
        let failed = if phase == SqliteSnapshotPhase::ProjectSnapshot { snapshot.to_sqlite_database(&mut control).is_err() } else { XlsxSnapshot::from_sqlite_database(&db, &mut control).is_err() };
        assert!(failed);
        assert!(observed);
    }
}
#[test]
fn sqlite_snapshot_xlsx_partial_part_reconstruction_retires_deep_completed_documents() {
    std::thread::Builder::new()
        .stack_size(256 * 1024)
        .spawn(|| {
            struct Owned(Option<XlsxSnapshot>);
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
            assert!(XlsxSnapshot::from_sqlite_database(
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
            let restored = XlsxSnapshot::from_sqlite_database(&db, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
            restored.retire_sqlite_snapshot();
        })
        .unwrap()
        .join()
        .unwrap();
}
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_xlsx_actual_declaration_exposes_complete_typed_snapshot() {
    use semio_framework_os_kernel::io::{
        io_mechanism::{io_export_sqlite_snapshot, io_import_sqlite_snapshot},
        ArtifactDialect,
    };
    semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("XLSX SQLite").version("0.0.1").package_id("semio:stdio").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();
    let dialect = ArtifactDialect { artifact_kind: "s.stdio.xlsx".into(), standard: "ecma-376".into(), subset: "*".into() };
    let snapshot = fixture();
    let file = io_export_sqlite_snapshot(&dialect, &snapshot, SnapshotEncoding::Binary, SqliteDatabaseLimits::default(), &mut |_| true).await.unwrap().value;
    let restored = io_import_sqlite_snapshot::<XlsxSnapshot>(&dialect, &file, SqliteDatabaseLimits::default(), &mut |_| true).await.unwrap().value;
    assert_eq!(restored, snapshot);
    restored.retire_sqlite_snapshot();
    snapshot.retire_sqlite_snapshot();
}

#[test]
fn sqlite_snapshot_xlsx_actual_erased_native_boundaries_retain_all_owned_fields() {
    let snapshot = fixture();
    let limits = SqliteDatabaseLimits::default();
    let dialect = store::io_schema::ArtifactDialect { artifact_kind: "s.stdio.xlsx".into(), standard: "ecma-376".into(), subset: "*".into() };
    let codec = <XlsxSnapshot as ArtifactSqliteSnapshot>::sqlite_codec();
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let db = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
        let payload = (codec.import)(crate::STDIO_XLSX_DOCUMENT_SCHEMA, &dialect, db, encoding, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap().value;
        let projected = (codec.export)(crate::STDIO_XLSX_DOCUMENT_SCHEMA, &dialect, &payload, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap().value;
        let restored = XlsxSnapshot::from_sqlite_database(&projected, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
        assert_eq!(restored, snapshot);
        restored.retire_sqlite_snapshot();
    }
    snapshot.retire_sqlite_snapshot();
}
#[test]
fn sqlite_snapshot_xlsx_deep_erased_native_input_output_are_interior_cancellable() {
    let plan: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🛫️native/🔣️.json")).unwrap();
    std::thread::Builder::new()
        .stack_size(plan["smallStackBytes"].as_u64().unwrap() as usize)
        .spawn(move || {
            struct Owner(Option<XlsxSnapshot>);
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
            snapshot.xml_parts[1].content_type = "z".repeat(plan["lateTextBytes"].as_u64().unwrap() as usize);
            let snapshot = Owner(Some(snapshot));
            let limits = SqliteDatabaseLimits::default();
            let dialect = store::io_schema::ArtifactDialect { artifact_kind: "s.stdio.xlsx".into(), standard: "ecma-376".into(), subset: "*".into() };
            let codec = <XlsxSnapshot as ArtifactSqliteSnapshot>::sqlite_codec();
            for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
                let db = snapshot.0.as_ref().unwrap().to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
                let expected_database = db.clone();
                let payload = (codec.import)(crate::STDIO_XLSX_DOCUMENT_SCHEMA, &dialect, db, encoding, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap().value;
                let projected = (codec.export)(crate::STDIO_XLSX_DOCUMENT_SCHEMA, &dialect, &payload, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap().value;
                assert!(projected == expected_database, "complete deep native graph differs");
                assert_eq!(projected.table("xlsx_xml_element").unwrap().rows.iter().filter(|row| row.text(1).unwrap() == "node").count(), plan["depth"].as_u64().unwrap() as usize);
                assert_eq!(projected.table("xlsx_xml_text").unwrap().rows.iter().filter(|row| row.text(1).unwrap() == "leaf").count(), 1);
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
                        (codec.import)(crate::STDIO_XLSX_DOCUMENT_SCHEMA, &dialect, db, encoding, &mut SqliteSnapshotControl::new(&mut callback, limits)).is_err()
                    } else {
                        (codec.export)(crate::STDIO_XLSX_DOCUMENT_SCHEMA, &dialect, &payload, &mut SqliteSnapshotControl::new(&mut callback, limits)).is_err()
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
fn sqlite_snapshot_xlsx_opc_native_component_has_exact_literal_fields() {
    let plan: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../🎒️zip/📦️opc/🧩️native/🧫️fixtures/🔣️.json")).unwrap();
    let package: semio_s_artifact_stdio_zip::opc::OpcPackage = semio_framework_pack_json::from_json_str(&plan["package"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let limits = SqliteDatabaseLimits::default();
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let expected: Vec<u8> = if encoding == SnapshotEncoding::Binary { plan["binary"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u8).collect() } else { plan["text"].as_str().unwrap().as_bytes().to_vec() };
        let mut callback = |_| true;
        let mut control = semio_framework_value::NativeEncodeControl::new(limits.max_value_bytes, &mut callback);
        control.begin_stage(0).unwrap();
        let mut measure = semio_s_artifact_stdio_zip::opc::native::OpcNativeWriter { control: &mut control, output: None, count: 0, rows: 0, limits, encoding };
        measure.package(&package).unwrap();
        assert_eq!(measure.count, expected.len());
        assert_eq!(measure.rows, plan["rows"].as_u64().unwrap() as usize);
        let total = measure.count;
        let mut output = control.allocate_vec::<u8>(total + 7).unwrap();
        output.extend_from_slice(b"pre");
        control.begin_stage(total).unwrap();
        let mut writer = semio_s_artifact_stdio_zip::opc::native::OpcNativeWriter { control: &mut control, output: Some(&mut output), count: 3, rows: 7, limits, encoding };
        writer.package(&package).unwrap();
        assert_eq!(writer.count, total + 3);
        assert_eq!(writer.rows, 7 + plan["rows"].as_u64().unwrap() as usize);
        assert_eq!(&output[3..], expected);
        output.extend_from_slice(b"tail");
        let mut callback = |_| true;
        let mut control = semio_framework_value::NativeDecodeControl::new(limits.max_value_bytes, &mut callback);
        control.begin_stage(output.len()).unwrap();
        control.advance(3).unwrap();
        let mut reader = semio_s_artifact_stdio_zip::opc::native::OpcNativeReader { bytes: &output, position: 3, rows: 7, limits, binary: encoding == SnapshotEncoding::Binary, control: &mut control };
        assert_eq!(reader.package().unwrap(), package);
        assert_eq!(reader.position, total + 3);
        assert_eq!(&output[reader.position..], b"tail");
        assert_eq!(reader.rows, 7 + plan["rows"].as_u64().unwrap() as usize);
    }
}

#[test]
fn sqlite_snapshot_xlsx_owned_native_actual_file_and_entity_ceilings() {
    let snapshot = fixture();
    let defaults = SqliteDatabaseLimits::default();
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, defaults)).unwrap();
    let rows = database.tables.iter().map(|table| table.rows.len()).sum::<usize>();
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let payload = snapshot.encode_sqlite_snapshot_native(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, defaults)).unwrap();
        let length = match &payload {
            store::io_schema::IoPayload::Binary(bytes) => bytes.len(),
            store::io_schema::IoPayload::Text(text) => text.len(),
        };
        let exact = SqliteDatabaseLimits { max_file_bytes: length, max_rows: rows, ..defaults };
        assert_eq!(snapshot.encode_sqlite_snapshot_native(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, exact)).unwrap(), payload);
        let restored = XlsxSnapshot::decode_sqlite_snapshot_native(&payload, &mut SqliteSnapshotControl::new(&mut |_| true, exact)).unwrap();
        assert_eq!(restored, snapshot);
        restored.retire_sqlite_snapshot();
        for refused in [SqliteDatabaseLimits { max_file_bytes: length - 1, ..exact }, SqliteDatabaseLimits { max_rows: rows - 1, ..exact }] {
            assert!(snapshot.encode_sqlite_snapshot_native(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, refused)).is_err());
            assert!(XlsxSnapshot::decode_sqlite_snapshot_native(&payload, &mut SqliteSnapshotControl::new(&mut |_| true, refused)).is_err());
        }
    }
    snapshot.retire_sqlite_snapshot();
}

#[test]
fn sqlite_snapshot_xlsx_public_snapshot_pack_and_text_match_controlled_literal_owners() {
    let snapshot = fixture();
    let expected = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    let bytes = <XlsxSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
    let packed = <XlsxSnapshot as store::ArtifactPack>::decode_pack(&bytes).unwrap();
    assert!(packed.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap() == expected);
    packed.retire_sqlite_snapshot();
    let text = <XlsxSnapshot as store::ArtifactDsl>::print_dsl(&snapshot);
    let parsed = <XlsxSnapshot as store::ArtifactDsl>::parse_dsl(&text).unwrap();
    assert!(parsed.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap() == expected);
    parsed.retire_sqlite_snapshot();
    snapshot.retire_sqlite_snapshot();
}

#[test]
fn sqlite_snapshot_xlsx_native_grammar_describes_the_actual_owned_fields() {
    let snapshot = fixture();
    let text = <XlsxSnapshot as store::ArtifactDsl>::print_dsl(&snapshot);
    let (envelope, body) = store::semio_format::split_text_preamble(&text).unwrap();
    let grammar = semio_framework_dsl::parse_grammar(crate::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO).unwrap();
    assert!(semio_framework_dsl::Recognizer::compile(&grammar, &semio_framework_os_kernel::os_dsl::grammar::family_fragments().expect("OS family grammar"), semio_framework_os_kernel::os_dsl::grammar::product_macros())
        .expect("selected grammar fragments")
        .recognize(&format!("{}\n{body}", envelope.envelope_id()))
        .unwrap());
    snapshot.retire_sqlite_snapshot();
}

#[test]
fn sqlite_snapshot_xlsx_native_protocol_describes_the_actual_owned_fields() {
    let snapshot = fixture();
    let bytes = <XlsxSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
    let (_, body) = store::semio_format::unwrap_binary(&bytes).unwrap();
    let protocol = semio_framework_dsl::parse_protocol(crate::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO).unwrap();
    let trace = semio_framework_dsl::walk_protocol(&protocol, &body).unwrap();
    assert_eq!(trace.consumed, body.len());
    snapshot.retire_sqlite_snapshot();
}

#[test]
fn sqlite_snapshot_xlsx_complete_literal_native_file_matches_independent_fixture() {
    let plan: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧩️native-literals/🔣️.json")).unwrap();
    let snapshot: XlsxSnapshot = semio_framework_pack_json::from_json_str(&plan["snapshot"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let defaults = SqliteDatabaseLimits::default();
    let rows = plan["rows"].as_u64().unwrap() as usize;
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let expected = match encoding {
            SnapshotEncoding::Binary => store::io_schema::IoPayload::Binary(plan["binary"].as_array().unwrap().iter().map(|byte| byte.as_u64().unwrap() as u8).collect()),
            SnapshotEncoding::Text => store::io_schema::IoPayload::Text(plan["text"].as_str().unwrap().into()),
        };
        let length = match &expected {
            store::io_schema::IoPayload::Binary(bytes) => bytes.len(),
            store::io_schema::IoPayload::Text(text) => text.len(),
        };
        let limits = SqliteDatabaseLimits { max_file_bytes: length, max_rows: rows, ..defaults };
        assert_eq!(snapshot.encode_sqlite_snapshot_native(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap(), expected);
        let restored = XlsxSnapshot::decode_sqlite_snapshot_native(&expected, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
        assert_eq!(restored, snapshot);
        restored.retire_sqlite_snapshot();
    }
    snapshot.retire_sqlite_snapshot();
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_xlsx_exact_named_owner_declarations_preserve_warnings_and_reject_foreign_state() {
    use semio_framework_os_kernel::io::io_mechanism::{io_export_sqlite_snapshot, io_import_sqlite_snapshot};
    semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio")
        .label("XLSX exact owned subsets")
        .version("0.0.1")
        .package_id("semio:stdio")
        .artifact(crate::declaration(crate::definition().unwrap()).unwrap())
        .try_build()
        .unwrap();
    let limits = SqliteDatabaseLimits::default();
    for subset in ["strict", "transitional"] {
        let base = crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_xlsx(XlsxWorkbook::default());
        let mut snapshot = if subset == "strict" { crate::standards::v_ecma_376::subsets::strict::schema::stamp_strict_namespace(base) } else { crate::standards::v_ecma_376::subsets::transitional::schema::stamp_transitional_namespace(base) };
        snapshot.schema = "literal retained schema".into();
        let main = snapshot.workbook_part_path().unwrap();
        if snapshot.opc.relationships.relationships(&main).is_none() { snapshot.opc.relationships.replace_owner(main.clone(), Vec::new()); }
        snapshot.opc.relationships.relationships_mut(&main).unwrap().push(semio_s_artifact_stdio_zip::opc::OpcRelationship {
            id: "literal unresolved worksheet".into(),
            rel_type: crate::standards::v_ecma_376::subsets::base::io::REL_TYPE_WORKSHEET.into(),
            target: "missing.xml".into(),
            target_mode: semio_s_artifact_stdio_zip::opc::OpcTargetMode::Internal,
        });
        let expected =
            if subset == "strict" { crate::standards::v_ecma_376::subsets::strict::schema::check_strict_conformance(&snapshot) } else { crate::standards::v_ecma_376::subsets::transitional::schema::check_transitional_conformance(&snapshot) };
        assert_eq!(expected.len(), 1);
        assert_eq!(expected[0].severity, semio_framework_diagnostic::Severity::Warning);
        let dialect = store::io_schema::ArtifactDialect { artifact_kind: "s.stdio.xlsx".into(), standard: "ecma-376".into(), subset: subset.into() };
        for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
            let outcome = io_export_sqlite_snapshot(&dialect, &snapshot, encoding, limits, &mut |_| true).await.unwrap();
            assert_eq!(format!("{:?}", outcome.diagnostics), format!("{:?}", expected));
            let restored = io_import_sqlite_snapshot::<XlsxSnapshot>(&dialect, &outcome.value, limits, &mut |_| true).await.unwrap();
            assert_eq!(format!("{:?}", restored.diagnostics), format!("{:?}", expected));
            assert_eq!(restored.value, snapshot);
            restored.value.retire_sqlite_snapshot();
            let foreign = store::io_schema::ArtifactDialect { subset: if subset == "strict" { "transitional".into() } else { "strict".into() }, ..dialect.clone() };
            assert!(io_import_sqlite_snapshot::<XlsxSnapshot>(&foreign, &outcome.value, limits, &mut |_| true).await.is_err());
        }
        let part = snapshot.xml_part_mut(&main).unwrap();
        let semio_s_artifact_stdio_xml::schema::snapshot::XmlNode::Element { attrs, .. } = part.document.root.as_mut().unwrap() else { panic!("typed workbook root") };
        attrs.iter_mut().find(|attr| attr.name == "xmlns").unwrap().value = "invalid namespace".into();
        assert!(io_export_sqlite_snapshot(&dialect, &snapshot, SnapshotEncoding::Binary, limits, &mut |_| true).await.is_err());
        snapshot.retire_sqlite_snapshot();
    }
}

#[test]
fn sqlite_snapshot_xlsx_profile_diagnostics_are_interior_cancellable_and_admitted_before_copy() {
    let mut snapshot = crate::standards::v_ecma_376::subsets::strict::schema::stamp_strict_namespace(crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_xlsx(XlsxWorkbook::default()));
    let main = snapshot.workbook_part_path().unwrap();
    let semio_s_artifact_stdio_xml::schema::snapshot::XmlNode::Element { attrs, .. } = snapshot.xml_part_mut(&main).unwrap().document.root.as_mut().unwrap() else { panic!("workbook root") };
    attrs.iter_mut().find(|attr| attr.name == "xmlns").unwrap().value = "invalid世界".repeat(10000);
    let defaults = SqliteDatabaseLimits::default();
    let mut reached = false;
    let mut callback = |event: SqliteSnapshotProgress| {
        if event.phase == SqliteSnapshotPhase::ProjectSnapshot && event.total > 100000 && event.completed >= 256 && event.completed < event.total {
            reached = true;
            false
        } else {
            true
        }
    };
    assert!(super::subset::validate(&snapshot, "strict", &mut SqliteSnapshotControl::new(&mut callback, defaults)).is_err());
    assert!(reached);
    let error = super::subset::validate(&snapshot, "strict", &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits { max_value_bytes: 1024, ..defaults })).unwrap_err();
    assert_eq!(error.cause.kind, semio_framework_value::ValueRefusalKind::OwnershipLimit);
    assert!(error.diagnostics.is_empty());
    snapshot.retire_sqlite_snapshot();
}

#[test]
fn sqlite_snapshot_xlsx_exact_profile_neutral_cases_preserve_native_policy_diagnostics() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🛡️profile/🔣️.json")).unwrap();
    let limits = SqliteDatabaseLimits::default();
    for plan in fixture["cases"].as_array().unwrap() {
        let subset = plan["subset"].as_str().unwrap();
        let mut snapshot = crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_xlsx(XlsxWorkbook::default());
        let main = snapshot.workbook_part_path().unwrap();
        let semio_s_artifact_stdio_xml::schema::snapshot::XmlNode::Element { attrs, .. } = snapshot.xml_part_mut(&main).unwrap().document.root.as_mut().unwrap() else { panic!("workbook root") };
        attrs.retain(|attr| !matches!(attr.name.as_str(), "xmlns" | "xmlns:r" | "conformance"));
        for (name, key) in [("xmlns", "xmlns"), ("xmlns:r", "relationshipsNamespace"), ("conformance", "conformance")] {
            if let Some(value) = plan[key].as_str() {
                attrs.push(semio_s_artifact_stdio_xml::schema::snapshot::XmlAttr { name: name.into(), value: value.into() });
            }
        }
        if plan["vml"] == true {
            snapshot.opc.parts.push(semio_s_artifact_stdio_zip::opc::OpcPart { path: "drawing.vml".into(), content_type: "application/vnd.openxmlformats-officedocument.vmlDrawing".into(), bytes: vec![] });
        }
        if plan["missingWorksheet"] == true {
            if snapshot.opc.relationships.relationships(&main).is_none() { snapshot.opc.relationships.replace_owner(main.clone(), Vec::new()); }
            snapshot.opc.relationships.relationships_mut(&main).unwrap().push(semio_s_artifact_stdio_zip::opc::OpcRelationship {
                id: "sheet".into(),
                rel_type: crate::standards::v_ecma_376::subsets::base::io::REL_TYPE_WORKSHEET.into(),
                target: "../missing.xml".into(),
                target_mode: semio_s_artifact_stdio_zip::opc::OpcTargetMode::Internal,
            });
        }
        let expected =
            if subset == "strict" { crate::standards::v_ecma_376::subsets::strict::schema::check_strict_conformance(&snapshot) } else { crate::standards::v_ecma_376::subsets::transitional::schema::check_transitional_conformance(&snapshot) };
        let actual = match super::subset::validate(&snapshot, subset, &mut SqliteSnapshotControl::new(&mut |_| true, limits)) {
            Ok(outcome) => outcome.diagnostics,
            Err(error) => error.diagnostics,
        };
        assert_eq!(format!("{actual:?}"), format!("{expected:?}"), "{}", plan["id"]);
        assert_eq!(actual.len(), plan["expected"].as_array().unwrap().len());
        snapshot.retire_sqlite_snapshot();
    }
}

#[test]
fn sqlite_snapshot_xlsx_borrowed_preflight_pays_frontiers_without_materializing_output() {
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
        let error = <XlsxSnapshot as ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits { max_allocation_bytes: 1, ..limits })).unwrap_err();
        assert_eq!(error.kind, semio_framework_value::ValueRefusalKind::OwnershipLimit);
    }
    snapshot.retire_sqlite_snapshot();
}
