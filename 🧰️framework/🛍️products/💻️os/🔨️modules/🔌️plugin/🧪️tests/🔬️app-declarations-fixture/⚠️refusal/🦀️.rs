use semio_framework::kernel::ActorId;
use semio_framework::AppRole;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_decl_registry::declarations::ArtifactDeclaration;
#[cfg(test)]
use semio_framework_plugin_decl_emission::ArtifactMutationOutcome;
use semio_framework_plugin_decl_surface::EditorApp;
use semio_framework_plugin_app_builder::ExampleSource;
use semio_framework_plugin_decl_registry::declarations::IoDeclaration;
use semio_framework_plugin_decl_registry::declarations::LanguagePair;
use semio_framework_plugin_decl_registry::declarations::MediaDeclaration;
use semio_framework_plugin_decl_registry::declarations::NativeCodecs;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_decl_registry::declarations::SchemaDeclaration;
#[cfg(test)]
use semio_framework_plugin_decl_surface::SnapshotBuilder;
use semio_framework_plugin_decl_registry::declarations::StandardDeclaration;
use semio_framework_plugin_decl_registry::declarations::SubsetDeclaration;
use semio_framework_plugin_instance_app::VcsArtifactApp;
use semio_framework_plugin_decl_surface::ViewerApp;
use semio_framework_artifact_reference::io::text::artifact_reference::DialectCoordinateText as _;
#[cfg(test)]
use semio_framework_plugin_window_transient::built_text_to_component_tree;
use semio_framework_plugin_instance_surface::editor_surface;
use crate::declarations_fixture_tests::fixture;
use semio_framework_plugin_decl_registry::declarations::format_descriptor_of;
use semio_framework_plugin_decl_registry::declarations::project_artifact_declarations;
use semio_framework_plugin_instance_surface::viewer_surface;
use semio_framework_plugin_abi::ArtifactBuilder;
fn sqlite_guest_refusal_origin(dialect: &semio_framework_artifact_reference::ArtifactDialect) -> semio_framework::io_schema::IoError {
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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/⚠️refusal/🔣️.json")).unwrap();
    let diagnostic = &fixture["diagnostic"];
    semio_framework::io_schema::IoError {
        cause: ValueError::new(kind, fixture["message"].as_str().unwrap()),
        diagnostics: vec![semio_framework::Diagnostic { code: semio_framework::FaultCode::new(diagnostic["code"].as_str().unwrap()), severity: semio_framework::Severity::Error, span: semio_framework::TextSpan::at(u32::try_from(diagnostic["line"].as_u64().unwrap()).unwrap(), u32::try_from(diagnostic["column"].as_u64().unwrap()).unwrap()), message: diagnostic["message"].as_str().unwrap().into(), expected: None, scope: semio_framework::FaultScope::default() }],
    }
}

fn sqlite_guest_refusal_export(_: &str, dialect: &semio_framework_artifact_reference::ArtifactDialect, _: &semio_framework::io_schema::IoPayload, _: &mut store::sqlite_snapshot::SqliteSnapshotControl<'_>, _: &mut semio_framework_os_kernel::io::control::NativeSnapshotDecodeOwner<'_,'_>) -> semio_framework::io_schema::IoResult<store::sqlite_snapshot::SqliteDatabase> {
    Err(sqlite_guest_refusal_origin(dialect))
}

fn sqlite_guest_refusal_import(_: &str, dialect: &semio_framework_artifact_reference::ArtifactDialect, _: &mut Option<store::sqlite_snapshot::SqliteDatabase>, _: store::sqlite_snapshot::SnapshotEncoding, _: &mut store::sqlite_snapshot::SqliteSnapshotControl<'_>, _: &mut semio_framework_os_kernel::io::control::NativeSnapshotEncodeOwner<'_,'_>) -> semio_framework::io_schema::IoResult<semio_framework::io_schema::IoPayload> {
    Err(sqlite_guest_refusal_origin(dialect))
}

fn sqlite_guest_assert_refusal(rejection: &semio_framework_plugin_abi::sqlite_wire::SnapshotRejection, expected: &semio_framework::io_schema::IoError) {
    use semio_framework_value::ToValue;
    let value = rejection.to_value();
    assert_eq!(value.get("kind").and_then(semio_framework_value::DslValue::as_str), Some(expected.cause.kind.as_str()), "actual guest rejection lost the intrinsic provider cause");
    assert_eq!(rejection.message, expected.cause.message);
    assert_eq!(semio_framework_plugin_abi::sqlite_wire::decode_diagnostics(&rejection.diagnostics).unwrap(), expected.diagnostics);
    let bytes = semio_framework_os_kernel::pack_rt::encode_wire_value(&value);
    let restored = <semio_framework_plugin_abi::sqlite_wire::SnapshotRejection as semio_framework_value::FromValue>::from_value(semio_framework_os_kernel::pack_rt::decode_wire_value(&bytes).unwrap()).unwrap();
    assert_eq!(restored.to_value(), value);
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_guest_refusal_runtime_export_import_preserves_all_eight_causes() {
use pack::snapshot::ArtifactSqliteSnapshot as _;
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

    use semio_framework_os_kernel::io::io_mechanism::{attach_sqlite_snapshot_metadata, NativeSnapshotRegistration};
    use semio_framework_os_kernel::io::{ArtifactAssemblyRegistryPlan, commit_artifact_assembly_registry_plan};
    use store::sqlite_snapshot::{SnapshotEncoding, SqliteDatabase, SqliteDatabaseLimits, SqliteSnapshotControl, export_sqlite_database};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/⚠️refusal/🔣️.json")).unwrap();
    let mut registrations = Vec::new();
    let mut codecs = Vec::new();
    let mut document_bindings = Vec::new();
    let mut dialects = Vec::new();
    for kind in fixture["kinds"].as_array().unwrap() {
        let kind = kind.as_str().unwrap();
        let dialect = semio_framework_artifact_reference::ArtifactDialect::parse_coordinate(&format!("s.testkit.sqlite-refusal@1/{kind}")).unwrap();
        let mut codec = store::artifact_codec::<Std1AnySnapshot, Std1AnyMutation>(&format!("semio.testkit.sqlite-refusal.{kind}/v1"));
        let original = codec.snapshot_sqlite.as_ref().expect("actual typed snapshot provider");
        codec.snapshot_sqlite = Some(store::os_store::serialization::ArtifactSqliteSnapshotCodec {
            schema: original.schema.clone(),
            snapshot_type: original.snapshot_type,
            export: sqlite_guest_refusal_export,
            import: sqlite_guest_refusal_import,
            export_file:None,
            import_file:None,
        });
        document_bindings.push(semio_framework_os_kernel::io::ArtifactCodecBinding::from_codec(semio_framework_os_kernel::io::ArtifactCodecBindingChannel::DirectNative, "testkit", Some(dialect.artifact_kind.clone()), Vec::new(), Some(dialect.clone()), None, &codec));
        codecs.push(codec.clone());
        registrations.push(NativeSnapshotRegistration { dialect: dialect.clone(), codec });
        dialects.push(dialect);
    }
    let assembly = semio_framework_schema_registry::assembly::begin().unwrap();
    commit_artifact_assembly_registry_plan(&assembly, ArtifactAssemblyRegistryPlan { document_codecs: codecs, document_bindings, native_snapshots: registrations, ..Default::default() }).unwrap();
    drop(assembly);
    for dialect in dialects {
        let expected = sqlite_guest_refusal_origin(&dialect);
        for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
            let limits = SqliteDatabaseLimits::default();
            let neutral:semio_framework_plugin_abi::sqlite_wire::SnapshotInput=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();
            let grant=neutral.native.native().unwrap();
            let mut receive=|_|true;let mut publish=|_|true;let mut progress=|_|true;
            let mut decoder=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(grant.maximum_capacity_bytes,&mut receive);
            let mut encoder=semio_framework_value::NativeEncodeControl::new(grant.maximum_capacity_bytes,&mut publish);
            let mut native=semio_framework_os_kernel::io::io_mechanism::IoRunControl::new(&mut decoder,&mut encoder,grant);
            let mut snapshot=SqliteSnapshotControl::new(&mut progress,limits);
            let input = semio_framework_plugin_abi::sqlite_wire::SnapshotInput { dialect: dialect.to_coordinate(), encoding: encoding.as_str().into(), payload: Std1AnySnapshot { value: 7 }.encode_pack(), limits: limits.into(),native:grant.into() };
            let mut input = input;
            if encoding == SnapshotEncoding::Text { input.payload = b"{\"value\":7}".to_vec(); }
            let exported = semio_framework_plugin_runtime_ops::ops_document::plugin_snapshot_sqlite_export(&mut semio_framework_plugin_abi::sqlite_wire::SnapshotInputOwner{wire:Some(input),dialect:None,route:None,payload:None,hop:None},&mut native,&mut snapshot).await.unwrap();
            let semio_framework_plugin_abi::sqlite_wire::SnapshotFileResult::Rejected(rejection) = exported else { panic!("refused provider exported a file") };
            sqlite_guest_assert_refusal(&rejection, &expected);
            let mut database = SqliteDatabase::from_schema(<Std1AnySnapshot as pack::snapshot::ArtifactSqliteSnapshot>::SQLITE_SCHEMA).unwrap();
            let mut progress = |_| true;
            let mut control = SqliteSnapshotControl::new(&mut progress, limits);
            attach_sqlite_snapshot_metadata(&mut database, &dialect, encoding, &mut control).unwrap();
            let bytes = export_sqlite_database(&database, limits, &mut |_| true).unwrap();
            let imported = semio_framework_plugin_runtime_ops::ops_document::plugin_snapshot_sqlite_import(&mut semio_framework_plugin_abi::sqlite_wire::SnapshotInputOwner{wire:Some(semio_framework_plugin_abi::sqlite_wire::SnapshotInput { dialect: dialect.to_coordinate(), encoding: encoding.as_str().into(), payload: bytes, limits: limits.into(),native:grant.into() }),dialect:None,route:None,payload:None,hop:None},&mut native,&mut snapshot).await.unwrap();
            let semio_framework_plugin_abi::sqlite_wire::SnapshotPayloadResult::Rejected(rejection) = imported else { panic!("refused provider imported a snapshot") };
            sqlite_guest_assert_refusal(&rejection, &expected);
        }
    }
}
