fn sqlite_guest_refusal_origin(dialect: &semio_framework::io_schema::ArtifactDialect) -> semio_framework::io_schema::IoError {
    use semio_framework_value::{ValueError, ValueRefusalKind};
    let kind = match dialect.subset.as_str() {
        "invalidValue" => ValueRefusalKind::InvalidValue,
        "canceled" => ValueRefusalKind::Canceled,
        "ownershipLimit" => ValueRefusalKind::OwnershipLimit,
        "allocationFailed" => ValueRefusalKind::AllocationFailed,
        "workLimit" => ValueRefusalKind::WorkLimit,
        "depthLimit" => ValueRefusalKind::DepthLimit,
        "unsupportedOwner" => ValueRefusalKind::UnsupportedOwner,
        "invariantViolated" => ValueRefusalKind::InvariantViolated,
        _ => panic!("unregistered intrinsic refusal fixture"),
    };
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧬️schema/🪶️sqlite/⚠️refusal/🧫️fixtures/🔣️.json")).unwrap();
    let diagnostic = &fixture["diagnostic"];
    semio_framework::io_schema::IoError {
        cause: ValueError::new(kind, fixture["message"].as_str().unwrap()),
        diagnostics: vec![semio_framework::Diagnostic { code: semio_framework::FaultCode::new(diagnostic["code"].as_str().unwrap()), severity: semio_framework::Severity::Error, span: semio_framework::TextSpan::at(u32::try_from(diagnostic["line"].as_u64().unwrap()).unwrap(), u32::try_from(diagnostic["column"].as_u64().unwrap()).unwrap()), message: diagnostic["message"].as_str().unwrap().into(), expected: None, scope: semio_framework::FaultScope::default() }],
    }
}

fn sqlite_guest_refusal_export(_: &str, dialect: &semio_framework::io_schema::ArtifactDialect, _: &semio_framework::io_schema::IoPayload, _: &mut store::sqlite_snapshot::SqliteSnapshotControl<'_>) -> semio_framework::io_schema::IoResult<store::sqlite_snapshot::SqliteDatabase> {
    Err(sqlite_guest_refusal_origin(dialect))
}

fn sqlite_guest_refusal_import(_: &str, dialect: &semio_framework::io_schema::ArtifactDialect, _: store::sqlite_snapshot::SqliteDatabase, _: store::sqlite_snapshot::SnapshotEncoding, _: &mut store::sqlite_snapshot::SqliteSnapshotControl<'_>) -> semio_framework::io_schema::IoResult<semio_framework::io_schema::IoPayload> {
    Err(sqlite_guest_refusal_origin(dialect))
}

fn sqlite_guest_assert_refusal(rejection: &crate::sqlite_wire::SnapshotRejection, expected: &semio_framework::io_schema::IoError) {
    use semio_framework_value::ToValue;
    let value = rejection.to_value();
    assert_eq!(value.get("kind").and_then(semio_framework_value::DslValue::as_str), Some(expected.cause.kind.as_str()), "actual guest rejection lost the intrinsic provider cause");
    assert_eq!(rejection.message, expected.cause.message);
    assert_eq!(crate::sqlite_wire::decode_diagnostics(&rejection.diagnostics).unwrap(), expected.diagnostics);
    let bytes = semio_framework_os_kernel::pack_rt::encode_wire_value(&value);
    let restored = <crate::sqlite_wire::SnapshotRejection as semio_framework_value::FromValue>::from_value(semio_framework_os_kernel::pack_rt::decode_wire_value(&bytes).unwrap()).unwrap();
    assert_eq!(restored.to_value(), value);
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_guest_refusal_runtime_export_import_preserves_all_eight_causes() {
    use semio_framework::io::io_mechanism::{attach_sqlite_snapshot_metadata, NativeSnapshotRegistration};
    use semio_framework::io::{ArtifactAssemblyRegistryPlan, commit_artifact_assembly_registry_plan};
    use store::sqlite_snapshot::{SnapshotEncoding, SqliteDatabase, SqliteDatabaseLimits, SqliteSnapshotControl, export_sqlite_database};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧬️schema/🪶️sqlite/⚠️refusal/🧫️fixtures/🔣️.json")).unwrap();
    let mut registrations = Vec::new();
    let mut codecs = Vec::new();
    let mut dialects = Vec::new();
    for kind in fixture["kinds"].as_array().unwrap() {
        let kind = kind.as_str().unwrap();
        let dialect = semio_framework::io_schema::ArtifactDialect::parse_coordinate(&format!("s.testkit.sqlite-refusal@1/{kind}")).unwrap();
        let mut codec = store::ArtifactCodec::of::<Std1AnySnapshot, Std1AnyMutation>(&format!("semio.testkit.sqlite-refusal.{kind}/v1"));
        codec.snapshot_sqlite = Some(store::ArtifactSqliteSnapshotCodec {
            schema: <Std1AnySnapshot as store::ArtifactSqliteSnapshot>::SQLITE_SCHEMA.into(),
            snapshot_type: None,
            export: sqlite_guest_refusal_export,
            import: sqlite_guest_refusal_import,
        });
        codecs.push(codec.clone());
        registrations.push(NativeSnapshotRegistration { dialect: dialect.clone(), codec });
        dialects.push(dialect);
    }
    let assembly = store::begin_artifact_assembly().unwrap();
    commit_artifact_assembly_registry_plan(&assembly, ArtifactAssemblyRegistryPlan { document_codecs: codecs, native_snapshots: registrations, ..Default::default() }).unwrap();
    drop(assembly);
    for dialect in dialects {
        let expected = sqlite_guest_refusal_origin(&dialect);
        for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
            let limits = SqliteDatabaseLimits::default();
            let input = crate::sqlite_wire::SnapshotInput { dialect: dialect.to_coordinate(), encoding: encoding.as_str().into(), payload: Std1AnySnapshot { value: 7 }.encode_pack(), limits: limits.into() };
            let mut input = input;
            if encoding == SnapshotEncoding::Text { input.payload = b"{\"value\":7}".to_vec(); }
            let exported = crate::plugin_runtime::plugin_snapshot_sqlite_export(input).await.unwrap();
            let crate::sqlite_wire::SnapshotFileResult::Rejected(rejection) = exported else { panic!("refused provider exported a file") };
            sqlite_guest_assert_refusal(&rejection, &expected);
            let mut database = SqliteDatabase::from_schema(<Std1AnySnapshot as store::ArtifactSqliteSnapshot>::SQLITE_SCHEMA).unwrap();
            let mut progress = |_| true;
            let mut control = SqliteSnapshotControl::new(&mut progress, limits);
            attach_sqlite_snapshot_metadata(&mut database, &dialect, encoding, &mut control).unwrap();
            let bytes = export_sqlite_database(&database, limits, &mut |_| true).unwrap();
            let imported = crate::plugin_runtime::plugin_snapshot_sqlite_import(crate::sqlite_wire::SnapshotInput { dialect: dialect.to_coordinate(), encoding: encoding.as_str().into(), payload: bytes, limits: limits.into() }).await.unwrap();
            let crate::sqlite_wire::SnapshotPayloadResult::Rejected(rejection) = imported else { panic!("refused provider imported a snapshot") };
            sqlite_guest_assert_refusal(&rejection, &expected);
        }
    }
}
