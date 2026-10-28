/// 🌉️ Host merge round trip, app level ownership and the finalized-run recolour laws.

//#region 🌉️HostMergeRoundTrip
/// 🌉️ Exactly what the react host dispatches for a `Trigger::Change` control: the descriptor's own
/// authored args with the control's value merged in under the literal key `value`
/// (`🗣️Interpreter/🟦️.tsx` `dispatchDeclarativeControlAction`).
fn host_merged(authored: &[(&str, &str)], value: &str) -> semio_framework_value::DslValue {
    let mut entries: Vec<(String, semio_framework_value::DslValue)> = authored.iter().map(|(key, value)| ((*key).to_string(), semio_framework_value::DslValue::String((*value).to_string()))).collect();
    entries.retain(|(key, _)| key != "value");
    entries.push(("value".to_string(), semio_framework_value::DslValue::String(value.to_string())));
    semio_framework_value::DslValue::Object(entries)
}

/// 📍️ THE blocker the review found: the host merges the typed number under `value` and the bridge has to route it into
/// the slot `field` NAMES — and only that slot: a press sets one site field (`change-site-<field>`), so scalars a render
/// authored (possibly stale) never ride along. Before the `{field, value}` indirection the bridge read `northAxisDeg` by
/// name, found the key missing and silently wrote 0.0 — so this law round-trips the real wire payload.
#[semio_framework_async_macros::async_test]
async fn a_site_control_writes_the_number_the_host_merged_not_the_bridges_default() {
    let mut model = inspector_model();
    model.site.north_axis_deg = 15.0;
    model.site.latitude_deg = 47.5;
    model.site.longitude_deg = 8.5;
    model.site.elevation_m = 400.0;
    model.site.time_zone_hours = 1.0;
    let snapshot = snapshot_of(&model);

    let authored: Vec<(&str, String)> = vec![
        ("elevationM", model.site.elevation_m.to_string()),
        ("field", "northAxisDeg".to_string()),
        ("latitudeDeg", model.site.latitude_deg.to_string()),
        ("longitudeDeg", model.site.longitude_deg.to_string()),
        ("northAxisDeg", model.site.north_axis_deg.to_string()),
        ("timeZoneHours", model.site.time_zone_hours.to_string()),
    ];
    let borrowed: Vec<(&str, &str)> = authored.iter().map(|(key, value)| (*key, value.as_str())).collect();
    let args = host_merged(&borrowed, "30");
    let command = <EnergyModelEditor as ArtifactEditor>::command_from_action(SET_SITE_ACTION_ID, Some((args).clone()), None).expect("the site payload resolves");
    assert_eq!(command, EnergyModelEditorCommand::SetSite { latitude_deg: None, longitude_deg: None, elevation_m: None, time_zone_hours: None, north_axis_deg: Some(30.0) });

    let edited = applied(&snapshot, &command);
    assert!((edited.site.north_axis_deg - 30.0).abs() < 1e-12, "the site kept the typed north axis, not 0.0: {:?}", edited.site);
    assert!((edited.site.latitude_deg - 47.5).abs() < 1e-12, "the untouched scalars survive the edit: {:?}", edited.site);
    assert_eq!(emitted_kinds(&snapshot, &command), vec!["change-site-north-axis".to_string()]);
}

/// 🌡️ The same law for the thermostat's five-slot payload — a throttle-range edit used to write the
/// bridge's hard-coded 2.0 K whatever the user typed.
#[semio_framework_async_macros::async_test]
async fn a_thermostat_control_writes_the_number_the_host_merged_not_the_bridges_default() {
    let model = inspector_model();
    let thermostat = model.thermostats.first().expect("the demo has a thermostat").clone();
    let snapshot = snapshot_of(&model);
    let authored: Vec<(&str, String)> = vec![
        ("coolingSchedule", thermostat.cooling_setpoint_schedule_id.0.to_string()),
        ("coolingThrottleRangeK", thermostat.cooling_throttle_range_k.to_string()),
        ("field", "heatingThrottleRangeK".to_string()),
        ("heatingSchedule", thermostat.heating_setpoint_schedule_id.0.to_string()),
        ("heatingThrottleRangeK", thermostat.heating_throttle_range_k.to_string()),
        ("thermostat", thermostat.id.0.to_string()),
        ("zone", thermostat.zone_id.0.to_string()),
    ];
    let borrowed: Vec<(&str, &str)> = authored.iter().map(|(key, value)| (*key, value.as_str())).collect();
    let command = <EnergyModelEditor as ArtifactEditor>::command_from_action(SET_THERMOSTAT_SETPOINTS_ACTION_ID, Some((host_merged(&borrowed, "4.5")).clone()), None).expect("the thermostat payload resolves");
    assert_eq!(
        command,
        EnergyModelEditorCommand::SetThermostatSetpoints {
            thermostat: thermostat.id.0,
            zone: thermostat.zone_id.0,
            heating_schedule: thermostat.heating_setpoint_schedule_id.0,
            cooling_schedule: thermostat.cooling_setpoint_schedule_id.0,
            heating_throttle_range_k: 4.5,
            cooling_throttle_range_k: thermostat.cooling_throttle_range_k,
        }
    );
    let edited = applied(&snapshot, &command);
    assert!((edited.thermostats[0].heating_throttle_range_k - 4.5).abs() < 1e-12, "the thermostat kept the typed range, not 2.0");
    assert_eq!(edited.thermostats[0].cooling_setpoint_schedule_id, thermostat.cooling_setpoint_schedule_id, "the untouched schedule survives");

    let other = model.schedules.constants.iter().map(|schedule| schedule.id).find(|id| *id != thermostat.heating_setpoint_schedule_id).expect("a second constant schedule");
    let mut swapped = borrowed.clone();
    swapped[2] = ("field", "heatingSchedule");
    let command = <EnergyModelEditor as ArtifactEditor>::command_from_action(SET_THERMOSTAT_SETPOINTS_ACTION_ID, Some((host_merged(&swapped, &other.0.to_string())).clone()), None).expect("the schedule payload resolves");
    assert_eq!(applied(&snapshot, &command).thermostats[0].heating_setpoint_schedule_id, other);
}

/// 🚧️ The other review finding: `interzone` needs a partner, so the inspector picks the PARTNER and
/// the boundary follows. The round trip is the same host merge as above.
#[semio_framework_async_macros::async_test]
async fn picking_an_interzone_partner_makes_the_surface_interzone() {
    let model = inspector_model();
    let snapshot = snapshot_of(&model);
    let (first, second) = (model.surfaces[0].id, model.surfaces[1].id);
    let args = host_merged(&[("field", "interzonePartner"), ("id", &first.0.to_string())], &second.0.to_string());
    let command = <EnergyModelEditor as ArtifactEditor>::command_from_action(SET_SURFACE_PROPERTY_ACTION_ID, Some((args).clone()), None).expect("the partner payload resolves");
    assert_eq!(command, EnergyModelEditorCommand::SetSurfaceProperty { surface: first.0, property: "interzonePartner".into(), value: second.0.to_string(), partner_surface: 0 });
    assert_eq!(emitted_kinds(&snapshot, &command), vec!["change-surface-boundary-condition".to_string()]);
    assert_eq!(applied(&snapshot, &command).surfaces[0].outside_boundary_condition, OutsideBoundary::Interzone(second));

    assert_eq!(refusal(&snapshot, &surface_property(first.0, "interzonePartner", &first.0.to_string())), "app.command.invalid-payload", "a surface is never its own neighbour");
    assert_eq!(refusal(&snapshot, &surface_property(first.0, "interzonePartner", "9999")), "mutation.target-missing");
    assert_eq!(refusal(&snapshot, &surface_property(first.0, "interzonePartner", "0")), "app.command.invalid-payload");

    let interzone = applied(&snapshot, &command);
    let held = snapshot_of(&interzone);
    assert_eq!(applied(&held, &surface_property(first.0, "boundary", "interzone")).surfaces[0].outside_boundary_condition, OutsideBoundary::Interzone(second));
}
//#endregion 🌉️HostMergeRoundTrip

//#region 🪟️AppLevelOwnership
/// 🪟️ Generalises `every_inspector_verb_is_owned_by_every_window_kind` to the WHOLE app-level roster:
/// the simulation window's three config/document verbs and the two creation verbs moved there too, so
/// the inspector's result-field select and the `mod+shift+*` chords fire whatever window is active.
#[semio_framework_async_macros::async_test]
async fn every_app_level_verb_is_owned_by_every_window_kind() {
    let def = definition();
    assert!(def.window_kinds.len() >= 4, "the editor declares its four windows");
    for action in app_level_action_definitions() {
        for window in &def.window_kinds {
            assert!(semio_framework::window_kind_actions(&def, window).iter().any(|declared| declared.id == action.id), "window kind {} does not own app-level action {}", window.id, action.id);
        }
    }
    for window in &def.window_kinds {
        assert!(semio_framework::window_kind_actions(&def, window).iter().any(|declared| declared.id == SET_ACTIVE_EXAMPLE_ACTION_ID), "window kind {} does not own {SET_ACTIVE_EXAMPLE_ACTION_ID}", window.id);
    }
}

/// ⌨️ Every chord the editor binds must reach its verb from ANY window — a keybinding is dispatched in
/// the active window's context exactly like a panel action, so a chord bound to a window-owned verb is
/// refused the moment another window has focus.
#[semio_framework_async_macros::async_test]
async fn every_bound_chord_reaches_a_verb_every_window_kind_owns() {
    let def = definition();
    let bound: Vec<(&str, &str)> = def
        .keybindings
        .iter()
        .map(|binding| (binding.keys.as_str(), binding.action.action.as_str()))
        .filter(|(_, action)| ENERGY_MODEL_RETAINED_TOOL_IDS.contains(action))
        .collect();
    assert!(bound.len() >= 3, "the editor's own chords are bound: {:?}", def.keybindings.iter().map(|binding| (&binding.keys, &binding.action.action)).collect::<Vec<_>>());
    for (keys, action) in bound {
        for window in &def.window_kinds {
            assert!(semio_framework::window_kind_actions(&def, window).iter().any(|declared| declared.id == action), "chord {keys} is bound to {action}, which window kind {} does not own", window.id);
        }
    }
}

/// 🪟️ No window kind may re-declare an app-level verb — doing so un-owns it everywhere else.
#[semio_framework_async_macros::async_test]
async fn no_window_kind_declares_an_app_level_verb_itself() {
    let app: BTreeSet<String> = app_level_action_definitions().into_iter().map(|action| action.id).collect();
    for (window, own) in [(structure::WINDOW_KIND_ID, structure::actions()), (zones::WINDOW_KIND_ID, zones::actions())] {
        for action in own {
            assert!(!app.contains(&action.id), "window kind {window} re-declares app-level verb {}, which would un-own it everywhere else", action.id);
        }
    }
    assert!(simulation::definition().actions.is_empty(), "the simulation window's three verbs are app-level now, not its own");
}
//#endregion 🪟️AppLevelOwnership

//#region 🎨️RecolourLaw
/// 🎨️ Every `UiDirtyScope` the app has owed since the last drain, the way the shell reads them
/// (`PluginApp::take_typed_operation_ui_scope`). Draining matters: the ledger only pushes a new scope
/// when the outbox is EMPTY (`flush_tool_run_ui_dirty`), so a test that never drains sees exactly one.
fn drain_ui_scopes(app: &mut EnergyEditorApp, into: &mut Vec<semio_framework::kernel::UiDirtyScope>) {
    use semio_framework_plugin::PluginApp as _;
    while let Some(scope) = app.take_typed_operation_ui_scope() {
        into.push(scope);
    }
}

fn scope_covers(scope: &semio_framework::kernel::UiDirtyScope, body_key: &str) -> bool {
    match scope {
        semio_framework::kernel::UiDirtyScope::Full => true,
        semio_framework::kernel::UiDirtyScope::None => false,
        semio_framework::kernel::UiDirtyScope::Partial { window_bodies, .. } => window_bodies.iter().any(|body| body == body_key),
    }
}

/// 🎨️ The ramp stop indices the projected 3d body actually paints, read out of the scene's per-vertex
/// colour arrays. The mesh lane rounds its channels (the projection prints `0.839,0.812,0.769`), and
/// the lane is chunked across `dataAttributes` keys, so a stop counts as present when each of its
/// three channels appears at ANY of the precisions the projection may have written it at — never at
/// one fixed precision, which would silently miss a painted band and make this law vacuous.
fn painted_bands(body: &str) -> std::collections::BTreeSet<usize> {
    fn channel_present(body: &str, channel: f64) -> bool {
        [format!("{channel:.3}"), format!("{channel:.4}"), format!("{channel:.5}"), format!("{channel}")].iter().any(|text| body.contains(text.as_str()))
    }
    crate::editor_results::legend_bands().iter().enumerate().filter(|(_, hex)| crate::editor_results::hex_to_rgb01(hex).iter().all(|channel| channel_present(body, *channel))).map(|(index, _)| index).collect()
}

/// 🎨️ THE recolour law (W1-D §10). Every hop between the kernel accumulator and a painted surface is
/// unit-tested in isolation; this is the only law that drives the WHOLE chain through the route the
/// shell uses — `toolRunStart` → the framework ledger's tick ingest → `ArtifactView::tool_run()` →
/// `surface_energy_from_run` → `surface_colors` → the World3d scene's per-vertex colours — and so the
/// only one that can catch the payload being dropped in the middle of it.
///
/// It exists because the browser timeline (`🗑️generated/energy-results-w5/timeline.txt`) recorded
/// exactly that: the meshes recoloured while the run ticked and snapped back BYTE-IDENTICAL to the
/// pre-run scene at `Finalized`. The cause was framework-side — the post-finalize
/// `ToolRunEffect::ReleaseProvisional` path called `discard_provisional()`, which clears
/// `ToolRunEntry::payload` along with the provisional ops. A read-only run's payload IS its result, so
/// that path now calls `release_provisional(true)`.
#[semio_framework_async_macros::async_test]
async fn a_finalized_simulation_run_recolours_the_three_d_model_window() {
    use semio_framework_plugin::PluginApp as _;
    use crate::editor_config::ResultField;

    let def = definition();
    let declared = crate::session::energy_simulation_run_definition().windows;
    assert!(!declared.is_empty(), "the run declares no windows, so no tick can mark a body dirty");
    for window_kind_id in &declared {
        assert!(def.window_kinds.iter().any(|kind| &kind.id == window_kind_id), "the run declares window kind {window_kind_id}, which this editor never registers — the driver drops it and the body never redraws on a tick");
    }
    assert!(declared.iter().any(|id| id == model_window::WINDOW_KIND_ID));
    assert_eq!(def.window_kinds.iter().find(|kind| kind.id == model_window::WINDOW_KIND_ID).expect("the 3d window kind").body_key, model_window::BODY_KEY);

    let mut app = simulation_app().await;
    let mut scopes = Vec::new();
    drain_ui_scopes(&mut app, &mut scopes);
    scopes.clear();

    let before = render_text(&mut app, model_window::BODY_KEY).await;
    assert!(!before.contains("kWh"), "there is no run yet, so there is no legend");
    assert!(painted_bands(&before).len() < 2, "the unpainted scene must not already look like a results overlay");

    start(&mut app).await;

    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(600);
    let mut painted_while_running = std::collections::BTreeSet::new();
    let mut live_caption = false;
    while std::time::Instant::now() < deadline && state_of(&app) != Some("finalized") {
        let running = matches!(state_of(&app), Some("starting" | "running"));
        app.0.advance_typed_operation_publication(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("driver turn");
        drain_ui_scopes(&mut app, &mut scopes);
        if running && scopes.iter().any(|scope| scope_covers(scope, model_window::BODY_KEY)) && painted_while_running.len() < 2 {
            let live = render_text(&mut app, model_window::BODY_KEY).await;
            live_caption |= live.contains("kWh");
            let bands = painted_bands(&live);
            if bands.len() > painted_while_running.len() {
                painted_while_running = bands;
            }
        }
    }
    assert_eq!(state_of(&app), Some("finalized"), "the run never finalized");

    assert!(scopes.iter().any(|scope| scope_covers(scope, model_window::BODY_KEY)), "no dirty scope during the run covered {}: {scopes:?}", model_window::BODY_KEY);

    let after = render_text(&mut app, model_window::BODY_KEY).await;
    assert!(after.contains("kWh"), "the FINALIZED run left no legend caption in the 3d body — `ToolRunView::payload` was cleared on finalize, or the body never re-rendered. Body: {}", &after[..after.len().min(1500)]);
    assert!(ResultField::ALL.iter().any(|field| after.contains(field.label())), "the caption names no published result field");
    assert_ne!(after, before, "the finalized body is byte-identical to the pre-run body: nothing was recoloured");
    assert!(live_caption, "the caption never appeared WHILE the run was live, only after it finalized");

    let bands = painted_bands(&after);
    assert!(bands.len() >= 2, "every surface landed on the same ramp band after finalize ({bands:?}) — the colour map is unreadable. Body: {}", &after[..after.len().min(1500)]);
    assert!(painted_while_running.len() >= 2, "every surface landed on the same ramp band WHILE the run was live ({painted_while_running:?}) — the mid-run payload is the all-zero pre-run map");

    close(app);
}
//#endregion 🎨️RecolourLaw
