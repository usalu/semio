
use super::*;

async fn fixture_artifact_kind(id: &str) -> semio_framework::ArtifactKindSpec {
    semio_framework::ArtifactKindSpec {
        id: id.into(),
        name: id.into(),
        source_format: id.into(),
        component_kind: "document".into(),
        dimension: "data".into(),
        media_capability: semio_framework::OsMediaCapability::MeshOnly,
        media_type: semio_framework::MediaType { class: semio_framework::MediaClass::Data, form: semio_framework::MediaForm::Value },
        schema: id.into(),
        export_formats: Vec::new(),
        import_formats: Vec::new(),
        export_stdio_kinds: Vec::new(),
        import_stdio_kinds: Vec::new(),
    }
}

pub(super) async fn fixture_app(id: &str, dialect: semio_framework::ArtifactDialect, role: semio_framework::AppRole) -> semio_framework::AppDefinition {
    semio_framework::AppDefinition {
        id: id.into(),
        role,
        dialect,
        label: ui_wgpu::wgpu::LocalizedLabel::data(id),
        breadcrumb: vec![id.into()],
        icon_id: None,
        controller_id: format!("{id}-play"),
        modes: semio_framework::Modes::one(semio_framework::ModeDefinition { id: "edit".into(), label: ui_wgpu::wgpu::LocalizedLabel::data("Edit"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }),
        default_mode_id: "edit".into(),
        window_kinds: semio_framework::WindowKinds::one(semio_framework::WindowKindDefinition {
            id: id.into(),
            label: ui_wgpu::wgpu::LocalizedLabel::data(id),
            body_key: id.into(),
            surface_kind: ui_wgpu::wgpu::SurfaceKind::Canvas2d,
            icon_id: "app-window".into(),
            options: ui_wgpu::wgpu::WindowOptions::default(),
            actions: Vec::new(),
            utilities: Vec::new(),
            interactions: Vec::new(),
            params_schema: None,
            artifact_snapshot_schema: None,
            input_event_schema: None,
            output_schema: None,
            capabilities: Vec::new(),
        }),
        panel_tabs: Vec::new(),
        keybindings: Vec::new(),
        utilities: Vec::new(),
        tools: Vec::new(),
        commands: Vec::new(),
        interactions: Vec::new(),
        named_layouts: Vec::new(),
        default_layout: None,
        terminologies: Vec::new(),
        terminology_breadcrumbs: HashMap::new(),
        introduction: None,
        tutorials: Vec::new(),
        dialogs: Vec::new(),
        media_inputs: Vec::new(),
        media_outputs: Vec::new(),
        artifact_kinds: Vec::new(),
        config: semio_framework::ConfigSpec::empty().await,
        command_grammar: semio_framework::CommandGrammar::empty().await,
        io: semio_framework::AppIo::from_document(
            id,
            semio_framework::MediaType { class: semio_framework::MediaClass::Data, form: semio_framework::MediaForm::Value },
            semio_framework::ArtifactPresentation { id: id.into(), name: id.into(), dimension: String::new(), component_kind: id.into() },
        )
        .await,
    }
}

async fn fixture_manifest(plugin_id: &str, dependency_ids: Vec<&str>, artifact_kinds: Vec<semio_framework::ArtifactKindSpec>, apps: Vec<semio_framework::AppDefinition>) -> PluginManifest {
    let mut dependencies = Vec::with_capacity(dependency_ids.len());
    for id in dependency_ids {
        dependencies.push(semio_framework::PluginDependency::new(id, semio_framework::VersionReq::Any));
    }
    PluginManifest {
        plugin_id: plugin_id.into(),
        label: plugin_id.into(),
        version: "0.1.0".into(),
        apps,
        examples: Vec::new(),
        capabilities: Vec::new(),
        topic_contributions: Vec::new(),
        commands: Vec::new(),
        artifact_kinds,
        dependencies,
        contributions: Vec::new(),
    }
}

pub(super) async fn dialect(subset: &str) -> semio_framework::ArtifactDialect {
    semio_framework::ArtifactDialect { artifact_kind: "s.cad.cad".into(), standard: "1".into(), subset: subset.into() }
}

async fn register(router: &AppRouter, plugin_id: &str, dependencies: Vec<&str>, artifact_kinds: Vec<semio_framework::ArtifactKindSpec>, apps: Vec<semio_framework::AppDefinition>) -> Result<(), semio_framework::Fault> {
    router.register_manifest(plugin_id, &fixture_manifest(plugin_id, dependencies, artifact_kinds, apps).await).await
}

#[semio_framework_async_macros::async_test]
async fn owner_surface_sorts_first_then_plugin_id_then_app_id() {
    let router = AppRouter::new();
    let editor_dialect = dialect("*").await;
    register(&router, "cad", vec![], vec![fixture_artifact_kind("s.cad.cad").await], vec![fixture_app("s.cad.cad@1/*#editor", editor_dialect.clone(), semio_framework::AppRole::Editor).await]).await.expect("owner registers");
    register(&router, "aec-building", vec!["cad"], vec![], vec![fixture_app("s.cad.cad@1/1#editor", dialect("1").await, semio_framework::AppRole::Editor).await])
        .await
        .expect("a distinct subset's editor, contributed by a dependent, does not conflict");
    let refs = router.surfaces_for(&editor_dialect, semio_framework::AppRole::Editor).await;
    assert_eq!(refs, vec![semio_framework::AppRef { plugin_id: "cad".into(), app_id: "s.cad.cad@1/*#editor".into() }]);
    assert_eq!(router.owner_of("s.cad.cad").await, Some("cad".to_string()));
}

#[semio_framework_async_macros::async_test]
async fn duplicate_app_ref_is_a_conflict() {
    let router = AppRouter::new();
    let editor_dialect = dialect("*").await;
    let app = fixture_app("s.cad.cad@1/*#editor", editor_dialect.clone(), semio_framework::AppRole::Editor).await;
    register(&router, "cad", vec![], vec![fixture_artifact_kind("s.cad.cad").await], vec![app.clone()]).await.expect("first registration succeeds");
    let error = register(&router, "cad", vec![], vec![], vec![app]).await.expect_err("re-registering the same AppRef must conflict");
    assert_eq!(error.code.0, "surface.conflict");
}

#[semio_framework_async_macros::async_test]
async fn contribution_without_a_declared_dependency_is_rejected() {
    let router = AppRouter::new();
    let editor_dialect = dialect("*").await;
    register(&router, "cad", vec![], vec![fixture_artifact_kind("s.cad.cad").await], vec![]).await.expect("owner claims the kind with zero apps");
    let error = register(&router, "norm", vec![], vec![], vec![fixture_app("s.cad.cad@1/*#viewer", editor_dialect, semio_framework::AppRole::Viewer).await]).await.expect_err("a non-owner plugin without a dependency on the owner must be rejected");
    assert_eq!(error.code.0, "surface.contribution-not-permitted");
}

#[semio_framework_async_macros::async_test]
async fn contribution_with_a_declared_dependency_is_admitted_and_sorted_after_the_owner() {
    let router = AppRouter::new();
    let editor_dialect = dialect("*").await;
    register(&router, "cad", vec![], vec![fixture_artifact_kind("s.cad.cad").await], vec![fixture_app("s.cad.cad@1/*#editor", editor_dialect.clone(), semio_framework::AppRole::Editor).await]).await.expect("owner registers its editor");
    register(&router, "norm", vec!["cad"], vec![], vec![fixture_app("s.cad.cad@1/*#viewer", editor_dialect.clone(), semio_framework::AppRole::Viewer).await])
        .await
        .expect("norm depends on cad, so contributing a viewer for cad's dialect is permitted");
    let viewers = router.surfaces_for(&editor_dialect, semio_framework::AppRole::Viewer).await;
    assert_eq!(viewers, vec![semio_framework::AppRef { plugin_id: "norm".into(), app_id: "s.cad.cad@1/*#viewer".into() }]);
}

#[semio_framework_async_macros::async_test]
async fn owned_surface_gaps_reports_the_missing_role_only() {
    let router = AppRouter::new();
    let editor_dialect = dialect("*").await;
    register(&router, "cad", vec![], vec![fixture_artifact_kind("s.cad.cad").await], vec![fixture_app("s.cad.cad@1/*#editor", editor_dialect, semio_framework::AppRole::Editor).await]).await.expect("owner registers only an editor");
    let gaps = router.owned_surface_gaps().await;
    assert_eq!(gaps.len(), 1);
    assert_eq!(gaps[0].code.0, "surface.missing-owner-surface");
    assert!(gaps[0].message.contains("viewer"));
}

#[semio_framework_async_macros::async_test]
async fn unregister_plugin_drops_its_surfaces_but_keeps_its_ownership_claim() {
    let router = AppRouter::new();
    let editor_dialect = dialect("*").await;
    register(&router, "cad", vec![], vec![fixture_artifact_kind("s.cad.cad").await], vec![fixture_app("s.cad.cad@1/*#editor", editor_dialect.clone(), semio_framework::AppRole::Editor).await]).await.expect("owner registers");
    router.unregister_plugin("cad").await;
    assert!(router.surfaces_for(&editor_dialect, semio_framework::AppRole::Editor).await.is_empty(), "the surface itself is gone");
    assert_eq!(router.owner_of("s.cad.cad").await, Some("cad".to_string()), "ownership claim survives so a re-registering hot-reload reclaims it, not a stray contributor");
    register(&router, "cad", vec![], vec![fixture_artifact_kind("s.cad.cad").await], vec![fixture_app("s.cad.cad@1/*#editor", editor_dialect.clone(), semio_framework::AppRole::Editor).await])
        .await
        .expect("re-registering after unregister succeeds (no stale conflict)");
    assert_eq!(router.surfaces_for(&editor_dialect, semio_framework::AppRole::Editor).await, vec![semio_framework::AppRef { plugin_id: "cad".into(), app_id: "s.cad.cad@1/*#editor".into() }]);
}

/// 🧯️ Ticket 26/09/05/S-END-TO-END lane H: the SAME language-neutral vectors the TS twin
/// (`🎠️kernel/🟦️.ts`, `AppRouter.build`) consumes — a breaching plugin is excluded whole, every
/// other plugin still routes, and the fault is retrievable per plugin id. Both sides read
/// `🎠️kernel/🧫️fixtures/🧫️app-router-plugin-faults/🔣️.json`; only codes, owners and route
/// orders are compared (the two messages are written in each language's own voice).
#[semio_framework_async_macros::async_test]
async fn plugin_fault_isolation_matches_the_shared_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🎠️kernel/🧫️fixtures/🧫️app-router-plugin-faults/🔣️.json")).expect("app router plugin fault fixture");
    let text = |value: &serde_json::Value, key: &str| value[key].as_str().expect("fixture string").to_string();
    let fixture_dialect = |value: &serde_json::Value| semio_framework::ArtifactDialect { artifact_kind: text(value, "artifactKind"), standard: text(value, "standard"), subset: text(value, "subset") };
    let router = AppRouter::new();
    let mut manifests = Vec::new();
    for row in fixture["manifests"].as_array().expect("fixture manifests") {
        let plugin_id = text(row, "pluginId");
        let dependencies: Vec<&str> = row["dependencies"].as_array().map(|rows| rows.iter().map(|entry| entry["pluginId"].as_str().expect("fixture dependency id")).collect()).unwrap_or_default();
        let mut artifact_kinds = Vec::new();
        for kind in row["artifactKinds"].as_array().map(Vec::as_slice).unwrap_or_default() {
            artifact_kinds.push(fixture_artifact_kind(kind["id"].as_str().expect("fixture kind id")).await);
        }
        let mut apps = Vec::new();
        for app in row["apps"].as_array().expect("fixture apps") {
            let role = if text(app, "role") == "viewer" { semio_framework::AppRole::Viewer } else { semio_framework::AppRole::Editor };
            apps.push(fixture_app(&text(app, "id"), fixture_dialect(&app["dialect"]), role).await);
        }
        manifests.push((plugin_id.clone(), fixture_manifest(&plugin_id, dependencies, artifact_kinds, apps).await));
    }
    let faults = router.register_manifests(&manifests).await;
    let expected_faults: Vec<(String, String)> = fixture["expectedFaults"].as_array().expect("fixture faults").iter().map(|row| (text(row, "pluginId"), text(row, "code"))).collect();
    assert_eq!(faults.iter().map(|fault| (fault.scope.plugin_id.clone().unwrap_or_default(), fault.code.0.clone())).collect::<Vec<_>>(), expected_faults);
    for (plugin_id, code) in &expected_faults {
        assert_eq!(router.fault_for(plugin_id).await.map(|fault| fault.code.0), Some(code.clone()), "{plugin_id}");
    }
    for row in fixture["expectedOwners"].as_array().expect("fixture owners") {
        assert_eq!(router.owner_of(&text(row, "artifactKind")).await, Some(text(row, "pluginId")), "{}", text(row, "artifactKind"));
    }
    for row in fixture["expectedRoutes"].as_array().expect("fixture routes") {
        let role = if text(row, "role") == "viewer" { semio_framework::AppRole::Viewer } else { semio_framework::AppRole::Editor };
        let expected: Vec<semio_framework::AppRef> = row["entries"].as_array().expect("fixture entries").iter().map(|entry| semio_framework::AppRef { plugin_id: text(entry, "pluginId"), app_id: text(entry, "appId") }).collect();
        assert_eq!(router.surfaces_for(&fixture_dialect(&row["dialect"]), role).await, expected, "{}#{}", fixture_dialect(&row["dialect"]).to_coordinate(), text(row, "role"));
    }
    assert!(router.fault_for("cad").await.is_none(), "a clean plugin carries no fault");
}

/// 🔗️ Lane 1-D parity reconciliation (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET,
/// `📓️w1-d-report.md`): the SAME ordered fixture — owner surface, two contributed surfaces
/// from different plugins, a duplicate, an unknown dialect — is asserted here AND in the TS
/// twin (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET/🧪️w1-d-parity.ts`),
/// which builds the identical manifests through `AppRouter.build`/`resolveOpeningApp`. Both
/// sides must produce identical `surfaces_for` ordering and identical fault codes — run both,
/// paste both outputs into the report, per the ticket's verification rule.
#[semio_framework_async_macros::async_test]
async fn w1_d_parity_fixture_owner_two_contributors_duplicate_and_unknown_dialect() {
    let router = AppRouter::new();
    let editor_dialect = dialect("*").await;
    register(&router, "cad", vec![], vec![fixture_artifact_kind("s.cad.cad").await], vec![fixture_app("s.cad.cad@1/*#editor", editor_dialect.clone(), semio_framework::AppRole::Editor).await]).await.expect("owner registers");
    register(&router, "norm", vec!["cad"], vec![], vec![fixture_app("s.cad.cad@1/*#editor-norm", editor_dialect.clone(), semio_framework::AppRole::Editor).await]).await.expect("norm depends on cad, contributes a second editor");
    register(&router, "aec-building", vec!["cad"], vec![], vec![fixture_app("s.cad.cad@1/*#editor-aec", editor_dialect.clone(), semio_framework::AppRole::Editor).await]).await.expect("aec-building depends on cad, contributes a third editor");

    let refs = router.surfaces_for(&editor_dialect, semio_framework::AppRole::Editor).await;
    assert_eq!(
        refs,
        vec![
            semio_framework::AppRef { plugin_id: "cad".into(), app_id: "s.cad.cad@1/*#editor".into() },
            semio_framework::AppRef { plugin_id: "aec-building".into(), app_id: "s.cad.cad@1/*#editor-aec".into() },
            semio_framework::AppRef { plugin_id: "norm".into(), app_id: "s.cad.cad@1/*#editor-norm".into() },
        ],
        "owner first, then contributors pluginId-ascending (aec-building < norm)"
    );

    let duplicate =
        register(&router, "aec-building", vec!["cad"], vec![], vec![fixture_app("s.cad.cad@1/*#editor-aec", editor_dialect.clone(), semio_framework::AppRole::Editor).await]).await.expect_err("re-registering the same AppRef must conflict");
    assert_eq!(duplicate.code.0, "surface.conflict");

    let unknown_dialect = dialect("does-not-exist").await;
    let unknown = OpeningResolver::resolve(&router, &unknown_dialect, semio_framework::AppRole::Editor, None).await.expect_err("no surface registered for this subset");
    assert_eq!(unknown.code.0, "surface.unknown-dialect");
}
