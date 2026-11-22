use semio_plugin::artifact_app_laws::{close_registered_fixture_app, fixture_identity, render_registered_fixture_app, FixtureUiArena, OriginalFixtureScope};
use semio_plugin::{AppDefinition, Locale, PanelTabDefinition, PluginApp, Terminology, ViewModel};
use std::collections::BTreeSet;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Fixture {
    contract_id: String,
    rows: Vec<Surface>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Surface {
    variant: String,
    role: String,
    app_id: String,
    body_keys: Vec<String>,
}

fn panel_bodies(panels: &[PanelTabDefinition], keys: &mut BTreeSet<String>) {
    for panel in panels {
        if let Some(body) = &panel.body_key {
            keys.insert(body.clone());
        }
        panel_bodies(&panel.children, keys);
    }
}

async fn check_surface<A: PluginApp>(app: &mut A, scope: &mut OriginalFixtureScope, definition: &AppDefinition, fixture: &Surface) -> Result<usize, String> {
    if app.app_id().await != fixture.app_id {
        return Err(format!("{} factory returned the wrong app identity", fixture.app_id));
    }
    let artifact_schema = app.artifact_schema().await;
    if artifact_schema != format!("semio.norm.{}/v1", fixture.variant) || definition.io.artifact_schema != artifact_schema {
        return Err(format!("{} factory returned the wrong document schema", fixture.app_id));
    }
    let mut keys: BTreeSet<String> = definition.window_kinds.iter().map(|window| window.body_key.clone()).collect();
    panel_bodies(&definition.panel_tabs, &mut keys);
    if keys != fixture.body_keys.iter().cloned().collect() {
        return Err(format!("{} manifest returned the wrong surface inventory", fixture.app_id));
    }
    let view = ViewModel { locale: Locale::En, terminology: Terminology::Native, ..ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native) };
    for key in &fixture.body_keys {
        let projection = render_registered_fixture_app(app, key, &view, scope.cx()).await.map_err(|error| format!("{} {key}: {error:?}", fixture.app_id))?;
        if projection.contains("Unknown body") {
            return Err(format!("{} {key} resolved to the unknown-body fallback", fixture.app_id));
        }
    }
    let unknown_projection = render_registered_fixture_app(app, "unregistered.norm.surface", &view, scope.cx()).await.map_err(|error| format!("{} unknown fallback: {error:?}", fixture.app_id))?;
    if !unknown_projection.contains("Unknown body") {
        return Err(format!("{} did not render its unknown-body fallback", fixture.app_id));
    }
    match render_registered_fixture_app(app, &"x".repeat(70_000), &view, scope.cx()).await {
        Ok(_) => return Err(format!("{} accepted an oversized surface identity", fixture.app_id)),
        Err(refusal) => scope.retire_failure(refusal),
    }
    Ok(fixture.body_keys.len())
}

#[semio_framework_async_macros::async_test]
async fn norm_public_surfaces_render_all_declared_bodies() {
    let mut fixture: Fixture = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    assert_eq!(fixture.contract_id, "semio.norm.surface-render/v1");
    assert_eq!(fixture.rows.len(), 30);
    assert_eq!(fixture.rows.iter().map(|row| row.body_keys.len()).sum::<usize>(), 195);
    fixture.rows.retain(|row| VARIANTS.contains(&row.variant.as_str()));
    assert_eq!(fixture.rows.len(), VARIANTS.len() * 2);
    let plugin = plugin().expect("the real plugin must assemble its complete surface registry");
    assert_eq!(plugin.manifest.apps.iter().map(|app| app.id.clone()).collect::<BTreeSet<_>>(), fixture.rows.iter().map(|row| row.app_id.clone()).collect());
    assert_eq!(plugin.artifact_definitions().definitions().map(|definition| definition.identity().as_str().to_owned()).collect::<BTreeSet<_>>(), fixture.rows.iter().map(|row| format!("s.{}.{}", plugin.manifest.plugin_id, row.variant)).collect());
    let _arena = FixtureUiArena::admit();
    let mut visited = BTreeSet::new();
    let mut rendered = 0;
    for row in &fixture.rows {
        assert!(visited.insert(row.app_id.clone()));
        let definition = plugin.manifest.apps.iter().find(|app| app.id == row.app_id).unwrap();
        assert_eq!(definition.id, row.app_id);
        assert_eq!(definition.role.as_str(), row.role);
        assert_eq!(definition.dialect.artifact_kind, format!("s.{}.{}", plugin.manifest.plugin_id, row.variant));
        let mut scope = OriginalFixtureScope::admit();
        let mut app = plugin.create_app(&row.app_id, semio_os_kernel::ActorId(semio_os_kernel::os_spr::LOCAL_ACTOR_ID.into()), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut fixture_identity()).expect("each manifest app must construct").expect("each manifest app must have a registered factory");
        let result = check_surface(&mut app, &mut scope, definition, row).await;
        close_registered_fixture_app(&mut app, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, scope.cx()).expect("each surface app must close");
        scope.retire();
        rendered += result.unwrap_or_else(|error| panic!("{error}"));
    }
    assert_eq!(visited.len(), fixture.rows.len());
    assert_eq!(rendered, fixture.rows.iter().map(|row| row.body_keys.len()).sum::<usize>());
    assert!(plugin.create_app("s.norm.unknown@1/*#editor", semio_os_kernel::ActorId(semio_os_kernel::os_spr::LOCAL_ACTOR_ID.into()), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut fixture_identity()).expect("an unknown app id is a typed absence, not a fault").is_none());
}

//#region 🧵️RetainedCohort
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct RetainedFactoryRow {
    payload_schema: String,
    maximum_raw_bytes: usize,
    shared: bool,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct RetainedContractRow {
    tool_id: String,
    lanes: Vec<String>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct RetainedRouteRow {
    id: String,
    publication_lanes: Vec<String>,
    admission: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct RetainedAppRow {
    variant: String,
    controller: String,
    artifact_schema: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct RetainedExpected {
    apps: usize,
    routes_per_app: usize,
    identities: usize,
    retained: usize,
    batch_only_pending_rewrite: usize,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct RetainedFixture {
    factory: RetainedFactoryRow,
    publication_contracts: Vec<RetainedContractRow>,
    routes: Vec<RetainedRouteRow>,
    apps: Vec<RetainedAppRow>,
    expected: RetainedExpected,
}

macro_rules! assert_norm_pair {
    ($($artifact:path => ($editor:path, $viewer:path)),+ $(,)?) => {
        $(
            semio_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<$editor, $viewer>().await;
        )+
    };
}

/// 🧵️ The language-neutral cohort law: every editor this plugin owns classifies their three actions
/// `Migrated` and build against the one shared owned bounded factory. `create_app` is the real gate —
/// it runs the framework's proof↔factory join (`interactive-job.catalog-authority`) and the
/// publication-lane availability check, both of which fail closed on a mechanical classification flip.
#[semio_framework_async_macros::async_test]
async fn every_norm_editor_action_is_migrated_onto_the_shared_owned_factory() {
    let fixture: RetainedFixture = serde_json::from_str(include_str!("../../../../../../✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🧪️tests/🧫️fixtures/🧫️retained-command-dispositions/🔣️.json")).unwrap();
    assert!(fixture.factory.shared);
    assert_eq!(fixture.factory.payload_schema, semio_s_plugin_norm_app_surface::NORM_RETAINED_PAYLOAD_SCHEMA);
    assert_eq!(fixture.factory.maximum_raw_bytes, semio_s_plugin_norm_app_surface::NORM_RETAINED_RAW_BYTES);
    assert_eq!(fixture.apps.len(), fixture.expected.apps);
    assert_eq!(fixture.routes.len(), fixture.expected.routes_per_app);
    assert_eq!(fixture.expected.identities, fixture.expected.apps * fixture.expected.routes_per_app);
    assert_eq!(fixture.expected.retained, fixture.expected.identities);
    assert_eq!(fixture.expected.batch_only_pending_rewrite, 0);
    assert_eq!(fixture.routes.iter().map(|route| route.id.as_str()).collect::<Vec<_>>(), semio_s_plugin_norm_app_surface::NORM_RETAINED_TOOL_IDS.to_vec());
    assert_eq!(fixture.publication_contracts.len(), semio_s_plugin_norm_app_surface::NORM_PUBLICATION_CONTRACTS.len());
    for (row, declared) in fixture.publication_contracts.iter().zip(semio_s_plugin_norm_app_surface::NORM_PUBLICATION_CONTRACTS) {
        assert_eq!(row.tool_id, declared.tool_id);
        assert_eq!(row.lanes, declared.lanes.iter().copied().map(semio_s_plugin_norm_app_surface::publication_lane_id).collect::<Vec<_>>());
    }
    for (route, contract) in fixture.routes.iter().zip(&fixture.publication_contracts) {
        assert_eq!(route.admission, "migrated");
        assert_eq!(route.publication_lanes, contract.lanes);
    }

    let plugin = plugin().expect("the real plugin must assemble its complete surface registry");
    let mut identities = 0usize;
    for app in fixture.apps.iter().filter(|app| VARIANTS.contains(&app.variant.as_str())) {
        let definition = plugin.manifest.apps.iter().find(|entry| entry.id == app.controller).unwrap_or_else(|| panic!("{} is not a registered norm editor", app.controller));
        assert_eq!(definition.io.artifact_schema, app.artifact_schema);
        assert_eq!(definition.dialect.artifact_kind, format!("s.{}.{}", plugin.manifest.plugin_id, app.variant));
        for window in definition.window_kinds.iter() {
            let presented = semio_plugin::window_kind_actions(definition, window);
            for route in &fixture.routes {
                let action = presented.iter().find(|action| action.id == route.id).unwrap_or_else(|| panic!("{} window {} does not present {}", app.controller, window.id, route.id));
                assert_eq!(action.semantics.execution.interactive_job, semio_plugin::InteractiveJobClassification::Migrated, "{} {} must dispatch from the UI", app.controller, route.id);
            }
        }
        let mut scope = OriginalFixtureScope::admit();
        let mut runtime = plugin.create_app(&app.controller, semio_os_kernel::ActorId(semio_os_kernel::os_spr::LOCAL_ACTOR_ID.into()), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut fixture_identity()).unwrap_or_else(|error| panic!("{} must construct: {error:?}", app.controller)).unwrap_or_else(|| panic!("{} must build with its owned bounded factory registered", app.controller));
        close_registered_fixture_app(&mut runtime, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, scope.cx()).expect("each editor app must close");
        scope.retire();
        identities += fixture.routes.len();
    }
    assert_eq!(identities, VARIANTS.len() * fixture.expected.routes_per_app);
    assert_family_pairs!();
}
//#endregion 🧵️RetainedCohort
