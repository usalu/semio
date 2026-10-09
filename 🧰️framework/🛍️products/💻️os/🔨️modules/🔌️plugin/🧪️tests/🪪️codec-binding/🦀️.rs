fn contract() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../🚪️io/🪪️bindings/🧫️fixtures/🔣️.json")).expect("original codec binding contract")
}

fn flat(schema: &str) -> ArtifactDeclaration {
    use crate::test_app_mutation_fixture::config::{TestConfig, TestConfigMutation};
    let kind = "s.testkit.w1c-fixture";
    ArtifactDeclaration {
        kind: kind.to_string(), schemas: Vec::new(), inferences: Vec::new(), inference_services: Vec::new(), composers: Vec::new(), formats: Vec::new(), subset_validators: Vec::new(), languages: Vec::new(),
        document_codecs: vec![DocumentCodecSpec::bare::<TestConfig, TestConfigMutation>(schema, declarations::fixture::STD1_ANY_DIALECT)],
        migrations: Vec::new(), schema_documents: Vec::new(), child_slots: &[], link_slots: &[], capabilities: Vec::new(),
        definition: ArtifactDefinition::new(ArtifactIdentity::parse(kind).expect("actual original artifact identity")), definition_error: None,
    }
}

#[test]
fn codec_binding_original_flat_capture_preserves_app_less_owned_and_hosted_rows() {
    use semio_framework_os_kernel::io::ArtifactCodecBindingChannel as Channel;
    let owned = flat("binding.original.owned/v1");
    let hosted = flat("binding.original.hosted/v1");
    let context = owned.document_codec_contexts(Channel::OwnedDeclaration, "testkit");
    let plan = ArtifactRegistrationPlan::from_declarations(&[owned], &[hosted], Vec::new(), &[], "testkit", Vec::new(), Vec::new(), Vec::new());
    assert_eq!(plan.document_bindings.len(), 2);
    assert_eq!(plan.document_bindings[0], context[0]);
    assert_eq!(plan.document_bindings[0].channel, Channel::OwnedDeclaration);
    assert_eq!(plan.document_bindings[1].channel, Channel::HostedDeclaration);
    let expected = contract();
    for (row, codec) in plan.document_bindings.iter().zip(&plan.document_codecs) {
        assert_eq!(serde_json::json!(row.apps), expected["appLess"]);
        assert_eq!(row.contributor, "testkit");
        assert_eq!(row.artifact_kind.as_deref(), Some("s.testkit.w1c-fixture"));
        assert_eq!(row.native_identity, codec.native_identity);
        assert_eq!(row.sqlite_schema, codec.snapshot_sqlite.as_ref().map(|provider| provider.schema.clone()));
        assert!(row.sqlite_schema.is_none());
        assert_eq!(serde_json::json!(row.factory), expected["unjoinedFactory"]);
    }
    assert!(plan.native_snapshots.is_empty());
    let (runtime, registry) = plan.into_runtime(ArtifactDefinitionRegistry::new()).expect("original binding preflight");
    assert_eq!(runtime.document_bindings(), registry.document_bindings);
    assert_eq!(registry.document_bindings.len(), registry.document_codecs.len());
    eprintln!("[DEBUG] original Plugin flat bindings retained=2 nativeSqlRows=0 appLess=2");
}

#[test]
fn codec_binding_original_tree_capture_keeps_one_row_and_both_surface_apps_per_subset() {
    use semio_framework_os_kernel::io::ArtifactCodecBindingChannel as Channel;
    let declaration = declarations::fixture::build_declaration();
    let rows = declarations::artifact_declaration_bindings("testkit", std::slice::from_ref(&declaration));
    let subsets: Vec<_> = declaration.standards.iter().flat_map(|standard| &standard.subsets).collect();
    assert_eq!(rows.len(), subsets.len());
    assert_eq!(rows.len(), 3);
    let expected = contract();
    assert_eq!(expected["missingSql"], "retain");
    assert_eq!(expected["treeApps"], serde_json::json!(["viewer", "editor"]));
    for (row, subset) in rows.iter().zip(&subsets) {
        assert_eq!(row.channel, Channel::TreeSubset);
        assert_eq!(row.contributor, "testkit");
        assert_eq!(row.artifact_kind.as_deref(), Some(declaration.kind.as_str()));
        let mut apps = row.apps.clone();
        apps.sort();
        let mut original_apps = vec![subset.viewer.definition.id.clone(), subset.editor.definition.id.clone()];
        original_apps.sort();
        assert_eq!(apps, original_apps);
        assert_eq!(row.schema, subset.io.native.codec.schema);
        assert_eq!(row.dialect, Some(subset.dialect.into()));
        assert_eq!(row.native_identity, subset.io.native.codec.native_identity);
        assert_eq!(row.sqlite_schema, subset.io.native.codec.snapshot_sqlite.as_ref().map(|provider| provider.schema.clone()));
        assert_eq!(serde_json::json!(row.factory), expected["unjoinedFactory"]);
    }
    let codecs: Vec<_> = subsets.iter().map(|subset| subset.io.native.codec.clone()).collect();
    semio_framework_os_kernel::io::preflight_artifact_codec_bindings(&rows, &codecs).expect("all actual subset rows remain admissible");
    let mut runtime = PluginRuntimeRegistry::empty();
    runtime.retain_document_bindings(rows.clone());
    assert_eq!(runtime.document_bindings(), rows);
    eprintln!("[DEBUG] original Plugin tree bindings retained=3 surfaceApps=6 nativeSqlRows={}", rows.iter().filter(|row| row.sqlite_schema.is_some()).count());
}

#[semio_framework_async_macros::async_test]
async fn codec_binding_original_surface_specs_resolve_real_apps_before_flat_and_foreign_capture() {
    use semio_framework_os_kernel::io::ArtifactCodecBindingChannel as Channel;
    type App = EditorApp<declarations::fixture::Std1AnyEditor>;
    let tree = declarations::fixture::build_declaration();
    let expected = &tree.standards[0].subsets[0].editor.definition.id;
    let spec = DocumentCodecSpec::of::<App>().await;
    let foreign = DocumentCodecSpec::foreign::<App>("binding.original.foreign-surface/v1");
    assert_eq!(&spec.app_id, expected);
    assert_eq!(&foreign.app_id, expected);
    assert_ne!(spec.app_id, App::APP_ID);
    let mut declared_apps = BTreeSet::new();
    declared_apps.insert(expected.clone());
    foreign.preflight_foreign(&declared_apps).expect("exact original derived owner app is declared");
    let owned_row = spec.binding(Channel::OwnedDeclaration, "testkit", Some(tree.kind.as_str().to_string()), &spec.codec());
    assert_eq!(owned_row.apps, vec![expected.clone()]);
    let plan = ArtifactRegistrationPlan::from_declarations(&[], &[], Vec::new(), &[foreign], "testkit", Vec::new(), Vec::new(), Vec::new());
    assert_eq!(plan.document_bindings.len(), 1);
    assert_eq!(plan.document_bindings[0].channel, Channel::ForeignApp);
    assert_eq!(plan.document_bindings[0].artifact_kind, None);
    assert_eq!(plan.document_bindings[0].apps, vec![expected.clone()]);
    assert!(plan.document_bindings[0].factory.is_none());
    eprintln!("[DEBUG] original flat and foreign spec resolve actual surface app={expected}");
}
