/// 🧵️ Real action route dispatch, scrub gesture, simulation tool run and keybinding laws.

//#region 🧵️DispatchLaw
/// 🧬️ `artifact_app_laws::new_app_with_registry` needs the `App { definition, examples }` shape, which is
/// also the exact shape the plugin root registers — so a drift between the two would be caught
/// here rather than at boot.
pub(crate) fn energy_model_manifest_for_tests() -> semio_framework_plugin::App {
    semio_framework_plugin::App { definition: create_energy_model_editor(), examples: examples().into_iter().map(Into::into).collect() }
}

semio_framework_plugin::history_edit_acceptance_law!("energy", EnergyModelEditor, energy_model_manifest_for_tests, "../..");

/// 🧵️ A registry-backed app bound to the live runtime instance `meta("local")` addresses. The
/// registry-LESS `artifact_app_laws::new_app` cannot be used here: it builds an app with no
/// `AppActionRegistry`, so `migrated_tool_ids()` is empty and `validate_tool_job_rows` fails
/// closed with `interactive-job.catalog-authority` the moment any proof is declared.
async fn dispatchable_app() -> EnergyEditorApp {
    use semio_framework_plugin::PluginApp as _;
    let mut app = semio_framework_plugin::artifact_app_laws::new_app_with_registry_and_members::<EditorApp<EnergyModelEditor>, semio_s_artifact_stdio_semio::SemioMembers>(energy_model_manifest_for_tests, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
    app.bind_instance_id(semio_framework_plugin::artifact_app_laws::meta("local").instance_id).await;
    app
}

/// 🧵️ THE dispatch law: every declared verb must reach this editor's own reducer. A domain fault
/// (`mutation.target-missing` for an id absent from the empty default document,
/// `app.command.kind-unavailable` for a vocabulary group still landing, or an `energy.session.*`
/// rejection for a session verb with no live run) is a PASS — it proves the command was
/// constructed, admitted and reduced. Any `interactive-job.*` code is a FAIL: it means the verb
/// never reached the app at all (missing factory, missing proof, unsupported publication lane,
/// unclassified action).
#[semio_framework_async_macros::async_test]
async fn every_declared_verb_dispatches_without_an_interactive_job_fault() {
    use semio_framework_plugin::PluginApp as _;
    use semio_framework_plugin::{effective_action_args, DslValue};
    let definition = create_energy_model_editor();
    let mut app = dispatchable_app().await;
    let mut reached = 0;
    for tool_id in ENERGY_MODEL_RETAINED_TOOL_IDS {
        let action = definition.window_kinds.iter().flat_map(|window| semio_framework::window_kind_actions(&definition, window)).find(|action| action.id == *tool_id).unwrap_or_else(|| panic!("action {tool_id} is declared on no window kind"));
        let staged = effective_action_args(&action.args, &DslValue::Object(Vec::new()), None);
        match app.handle_action(tool_id, Some(&staged), &semio_framework_plugin::artifact_app_laws::meta("local"), &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await {
            Ok(_) => reached += 1,
            Err(fault) => {
                assert!(!fault.code.0.as_str().starts_with("interactive-job."), "action {tool_id} never reached the app: {} — {}", fault.code.0.as_str(), fault.message);
                reached += 1;
            }
        }
    }
    assert_eq!(reached, ENERGY_MODEL_RETAINED_TOOL_IDS.len());
    let _ = semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app, semio_framework_plugin::artifact_app_laws::meta("local").instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT);
}

/// 🧵️ The positive half of the law: a verb addressing real document state dispatches with NO
/// fault at all, and the operation is genuinely admitted — either the reduction published its
/// mutations inline or the retained tool operation is now live on the app.
#[semio_framework_async_macros::async_test]
async fn renaming_the_model_dispatches_cleanly_through_the_real_action_route() {
    use semio_framework_plugin::DslValue;
    use semio_framework_plugin::PluginApp as _;
    let mut app = dispatchable_app().await;
    let args = DslValue::Object(vec![("id".to_string(), DslValue::String("name".to_string())), ("value".to_string(), DslValue::String("BESTEST 600".to_string()))]);
    let result = app.handle_action(SET_NODE_ACTION_ID, Some(&args), &semio_framework_plugin::artifact_app_laws::meta("local"), &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("set-node dispatches without a fault");
    assert!(!result.mutations.is_empty() || app.has_pending_typed_operations(), "the rename neither published nor retained an operation");
    let _ = semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app, semio_framework_plugin::artifact_app_laws::meta("local").instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT);
}
//#region 🎚️ScrubLaws
/// 🎚️ One dispatch of the site latitude slider as the hosts send it: `{field, value, gesture, commit}`, settled.
async fn slide_site(app: &mut EnergyEditorApp, extra: Vec<(&str, semio_framework_plugin::DslValue)>) {
    slide_site_field(app, "latitudeDeg", extra).await;
}

/// 🎚️ One dispatch of the site control for `field` as the hosts send it, settled unless it is a host cancel.
async fn slide_site_field(app: &mut EnergyEditorApp, field: &str, extra: Vec<(&str, semio_framework_plugin::DslValue)>) {
    use semio_framework_plugin::{DslValue, PluginApp as _};
    let aborting = extra.iter().any(|(key, _)| *key == "abort");
    let mut args = vec![("field".to_string(), DslValue::String(field.into()))];
    args.extend(extra.into_iter().map(|(key, value)| (key.to_string(), value)));
    app.handle_action(SET_SITE_ACTION_ID, Some(&DslValue::Object(args)), &semio_framework_plugin::artifact_app_laws::meta("local"), &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("the slider dispatch is admitted");
    if !aborting {
        semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(app, semio_framework_plugin::artifact_app_laws::meta("local").instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("the slider dispatch settles");
    }
}

fn press(value: f64, gesture: &str, commit: bool) -> Vec<(&'static str, semio_framework_plugin::DslValue)> {
    use semio_framework_plugin::DslValue;
    vec![("value", DslValue::float(value)), ("gesture", DslValue::String(gesture.into())), ("commit", DslValue::Bool(commit))]
}

/// 📜️ Every history row and every mutation row the projection lists, with no lane or edit filter — what L4 observes
/// (design §20.13): a config-lane edit may appear in neither.
async fn history_rows_and_mutations(app: &mut EnergyEditorApp) -> (usize, usize) {
    use semio_framework_plugin::PluginApp as _;
    let upserts = app.history_snapshot().await.expect("history").upserts;
    (upserts.len(), upserts.iter().map(|entry| entry.mutations.len()).sum())
}

async fn transaction_rows(app: &mut EnergyEditorApp) -> Vec<semio_framework::kernel::HistoryEntry> {
    use semio_framework_plugin::PluginApp as _;
    app.history_snapshot().await.expect("history").upserts.into_iter().filter(|entry| entry.edit_id.is_some()).collect()
}

fn latitude(app: &EnergyEditorApp) -> f64 {
    app.snapshot().expect("projection").model.site.latitude_deg
}

/// ⚖️ LAW (W3-T2-CONTROLS): an inspector slider press is ONE tool transaction — the ticks leave the committed model
/// alone, the release lands one edit whose row carries the press's `TransactionRef` and the absolute `change-site-latitude`
/// leaf; a cancelled press leaves zero trace and two presses are two transactions.
#[semio_framework_async_macros::async_test]
async fn a_site_slider_press_is_one_transaction_and_a_cancel_leaves_zero_trace() {
    let mut app = dispatchable_app().await;
    let (before, rows) = (latitude(&app), transaction_rows(&mut app).await.len());
    slide_site(&mut app, press(10.0, "site.latitude:1", false)).await;
    slide_site(&mut app, press(20.0, "site.latitude:1", false)).await;
    assert_eq!(latitude(&app), before, "ticks never touch the committed model");
    slide_site(&mut app, press(30.0, "site.latitude:1", true)).await;
    assert_eq!(latitude(&app), 30.0);
    let after = transaction_rows(&mut app).await;
    assert_eq!(after.len(), rows + 1, "one press, one row");
    let first = after.iter().max_by_key(|entry| entry.seq).and_then(|entry| entry.transaction.clone()).expect("the press is a tool transaction");
    assert!(first.tool.ends_with(&format!("#{SET_SITE_ACTION_ID}")), "{first:?}");
    slide_site(&mut app, press(40.0, "site.latitude:2", false)).await;
    slide_site(&mut app, vec![("gesture", semio_framework_plugin::DslValue::String("site.latitude:2".into())), ("abort", semio_framework_plugin::DslValue::String("blur".into()))]).await;
    assert_eq!((latitude(&app), transaction_rows(&mut app).await.len()), (30.0, rows + 1), "a cancelled press leaves zero trace");
    slide_site(&mut app, press(45.0, "site.latitude:3", true)).await;
    let last = transaction_rows(&mut app).await.into_iter().max_by_key(|entry| entry.seq).and_then(|entry| entry.transaction).expect("the second press is a transaction");
    assert_ne!(last.id, first.id, "two presses, two transactions");
    assert_eq!(latitude(&app), 45.0);
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT);
}

async fn history_edit(app: &mut EnergyEditorApp, verb: &str, args: Vec<(&str, semio_framework_plugin::DslValue)>) {
    use semio_framework_plugin::PluginApp as _;
    let args = semio_framework_plugin::DslValue::Object(args.into_iter().map(|(key, value)| (key.to_string(), value)).collect());
    let result = app.handle_action(verb, Some(&args), &semio_framework_plugin::artifact_app_laws::meta("local"), &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.unwrap_or_else(|fault| panic!("{verb}: {fault:?}"));
    assert!(result.output.get("rejected").is_none(), "{verb} was refused: {:?}", result.output);
}

async fn pump_time_travel(app: &mut EnergyEditorApp, done: impl Fn(Option<semio_framework::kernel::HistoryTimeTravelStage>) -> bool) {
    use semio_framework_plugin::PluginApp as _;
    for _ in 0..10_000 {
        let stage = app.history_snapshot().await.expect("history").time_travel.map(|status| status.stage);
        if done(stage) {
            return;
        }
        app.advance_typed_operation_publication(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.preparation).await.expect("a driver turn");
        while app.take_typed_operation_ui_progress().is_some() {}
    }
    panic!("the history edit never settled");
}

async fn rename(app: &mut EnergyEditorApp, name: &str) {
    use semio_framework_plugin::{DslValue, PluginApp as _};
    let args = DslValue::Object(vec![("id".to_string(), DslValue::String("name".to_string())), ("value".to_string(), DslValue::String(name.to_string()))]);
    app.handle_action(SET_NODE_ACTION_ID, Some(&args), &semio_framework_plugin::artifact_app_laws::meta("local"), &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("the rename is admitted");
    semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(app, semio_framework_plugin::artifact_app_laws::meta("local").instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("the rename settles");
}

/// ⚖️ LAW (W3-T2-CONTROLS, design §13.1): time travel edits the slider press's committed VALUE — the absolute
/// `change-site-latitude` leaf superseded with another latitude replays the downstream rename, and the overwrite leaves exactly
/// the head a fresh run of the edited press and the rename reaches.
#[semio_framework_async_macros::async_test]
async fn a_committed_site_press_edited_in_history_replays_downstream() {
    use semio_framework_plugin::{DslValue, PluginApp as _};
    let mut app = dispatchable_app().await;
    slide_site(&mut app, press(30.0, "site.latitude:4", true)).await;
    let edited = transaction_rows(&mut app).await.into_iter().max_by_key(|entry| entry.seq).and_then(|row| row.mutations.first().map(|mutation| mutation.mutation_id.clone())).expect("the press's mutation");
    rename(&mut app, "Edited downstream").await;
    history_edit(&mut app, "historyEditBegin", vec![("mutationId", DslValue::String(edited))]).await;
    history_edit(&mut app, "historyEditInput", vec![("path", DslValue::String("/newLatitudeDeg".into())), ("value", DslValue::float(15.0))]).await;
    history_edit(&mut app, "historyEditAccept", Vec::new()).await;
    pump_time_travel(&mut app, |stage| stage != Some(semio_framework::kernel::HistoryTimeTravelStage::Replaying)).await;
    let status = app.history_snapshot().await.expect("history").time_travel.expect("a live session");
    assert_eq!((status.stage, status.blocking), (semio_framework::kernel::HistoryTimeTravelStage::Reviewing, false), "{status:?}");
    assert_eq!(latitude(&app), 30.0, "reviewing never touches the committed model");
    history_edit(&mut app, "historyEditFinalize", Vec::new()).await;
    history_edit(&mut app, "historyEditCommit", vec![("choice", DslValue::String("overwrite".into()))]).await;
    pump_time_travel(&mut app, |stage| stage.is_none()).await;
    let edited = app.snapshot().expect("edited head").model;
    assert_eq!((edited.site.latitude_deg, edited.name.as_str()), (15.0, "Edited downstream"), "the overwrite folds the edited latitude under the replayed rename");
    let mut fresh = dispatchable_app().await;
    slide_site(&mut fresh, press(15.0, "site.latitude:5", true)).await;
    rename(&mut fresh, "Edited downstream").await;
    assert_eq!(edited, fresh.snapshot().expect("fresh head").model, "the edited log equals a fresh run of the edited press");
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT);
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut fresh, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT);
}
/// ⚖️ LAW (design §13.1, field-granular site leaves): time travel edits an EARLIER press while a later press of ANOTHER
/// site field exists — the edited latitude and the later longitude both survive the replay and the overwrite, because
/// each press committed only its own field (`change-site-latitude`, `change-site-longitude`), never the whole site.
#[semio_framework_async_macros::async_test]
async fn a_history_edit_of_an_earlier_site_press_keeps_a_later_press_of_another_field() {
    use semio_framework_plugin::{DslValue, PluginApp as _};
    let mut app = dispatchable_app().await;
    slide_site_field(&mut app, "latitudeDeg", press(30.0, "site.latitude:6", true)).await;
    let edited = transaction_rows(&mut app).await.into_iter().max_by_key(|entry| entry.seq).and_then(|row| row.mutations.first().map(|mutation| mutation.mutation_id.clone())).expect("the latitude press's mutation");
    slide_site_field(&mut app, "longitudeDeg", press(50.0, "site.longitude:1", true)).await;
    let site = app.snapshot().expect("projection").model.site;
    assert_eq!((site.latitude_deg, site.longitude_deg), (30.0, 50.0));
    history_edit(&mut app, "historyEditBegin", vec![("mutationId", DslValue::String(edited))]).await;
    history_edit(&mut app, "historyEditInput", vec![("path", DslValue::String("/newLatitudeDeg".into())), ("value", DslValue::float(15.0))]).await;
    history_edit(&mut app, "historyEditAccept", Vec::new()).await;
    pump_time_travel(&mut app, |stage| stage != Some(semio_framework::kernel::HistoryTimeTravelStage::Replaying)).await;
    let status = app.history_snapshot().await.expect("history").time_travel.expect("a live session");
    assert_eq!((status.stage, status.blocking), (semio_framework::kernel::HistoryTimeTravelStage::Reviewing, false), "{status:?}");
    history_edit(&mut app, "historyEditFinalize", Vec::new()).await;
    history_edit(&mut app, "historyEditCommit", vec![("choice", DslValue::String("overwrite".into()))]).await;
    pump_time_travel(&mut app, |stage| stage.is_none()).await;
    let site = app.snapshot().expect("edited head").model.site;
    assert_eq!((site.latitude_deg, site.longitude_deg), (15.0, 50.0), "the edited latitude and the later longitude both survive the replay");
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT);
}
/// 🎛️ One settled dispatch of `action` addressed at the 3d model window `window-1`, `extra` merged into `args`; a host
/// cancel is settled by the runtime itself.
async fn press_in_window(app: &mut EnergyEditorApp, action: &str, args: Vec<(&str, semio_framework_plugin::DslValue)>) {
    use semio_framework_plugin::PluginApp as _;
    let aborting = args.iter().any(|(key, _)| *key == "abort");
    let mut meta = semio_framework_plugin::artifact_app_laws::meta("local");
    meta.view_state = Some(semio_framework_plugin::ViewModel {
        window_id: Some("window-1".into()),
        window_instances: vec![semio_framework::ViewWindowInstance { id: "window-1".into(), window_kind_id: model_window::WINDOW_KIND_ID.into() }],
        ..semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
    });
    let args = semio_framework_plugin::DslValue::Object(args.into_iter().map(|(key, value)| (key.to_string(), value)).collect());
    app.handle_action(action, Some(&args), &meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.unwrap_or_else(|fault| panic!("{action}: {fault:?}"));
    if !aborting {
        semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(app, meta.instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.unwrap_or_else(|fault| panic!("{action} settles: {fault:?}"));
    }
}

fn gesture_args(gesture: &str, commit: Option<bool>) -> Vec<(&'static str, semio_framework_plugin::DslValue)> {
    use semio_framework_plugin::DslValue;
    match commit {
        Some(commit) => vec![("gesture", DslValue::String(gesture.into())), ("commit", DslValue::Bool(commit))],
        None => vec![("gesture", DslValue::String(gesture.into())), ("abort", DslValue::String("blur".into()))],
    }
}

/// ⚖️ LAW (design §20.1): a config control is a press on the config lanes too — its ticks are held as the provisional
/// config (rendered, never published), the release publishes ONE config edit, a cancelled press publishes nothing, and
/// no config edit is ever a history row or a mutation row — observed over ALL rows, never an edit-linked subset (design
/// §20.13); the same holds for a window-config press (the 3d window's camera).
#[semio_framework_async_macros::async_test]
async fn a_config_press_is_one_config_edit_a_cancel_is_none_and_neither_is_a_history_row() {
    use semio_framework_plugin::DslValue;
    let mut app = dispatchable_app().await;
    let rows = history_rows_and_mutations(&mut app).await;
    let settings = |warmup: u32| vec![("zoneTimestepMinutes", DslValue::float(60.0)), ("systemTimestepMinutes", DslValue::float(60.0)), ("warmupDays", DslValue::float(f64::from(warmup)))];
    let (generation, _) = app.config_generations(model_window::WINDOW_KIND_ID, "window-1");
    for (warmup, commit) in [(10, false), (12, false)] {
        press_in_window(&mut app, simulation::SET_SETTINGS_ACTION_ID, [settings(warmup), gesture_args("settings.warmup:1", Some(commit))].concat()).await;
    }
    assert_eq!(app.config_generations(model_window::WINDOW_KIND_ID, "window-1").0, generation, "ticks publish no config edit");
    assert_eq!(app.rendered_config().warmup_days, 12, "the provisional config renders the press");
    press_in_window(&mut app, simulation::SET_SETTINGS_ACTION_ID, [settings(14), gesture_args("settings.warmup:1", Some(true))].concat()).await;
    assert_eq!(app.config_generations(model_window::WINDOW_KIND_ID, "window-1").0, generation + 1, "the release publishes ONE config edit");
    assert_eq!(app.rendered_config().warmup_days, 14);
    press_in_window(&mut app, simulation::SET_SETTINGS_ACTION_ID, [settings(30), gesture_args("settings.warmup:2", Some(false))].concat()).await;
    assert_eq!(app.rendered_config().warmup_days, 30);
    press_in_window(&mut app, simulation::SET_SETTINGS_ACTION_ID, gesture_args("settings.warmup:2", None)).await;
    assert_eq!((app.config_generations(model_window::WINDOW_KIND_ID, "window-1").0, app.rendered_config().warmup_days), (generation + 1, 14), "a cancelled press publishes nothing");
    let camera = |x: f64| vec![("camera", DslValue::Object(vec![("position".into(), DslValue::Array(vec![DslValue::float(x), DslValue::float(-9.0), DslValue::float(7.5)])), ("target".into(), DslValue::Array(vec![DslValue::float(4.0), DslValue::float(3.0), DslValue::float(1.35)])), ("zoom".into(), DslValue::float(1.0))]))];
    press_in_window(&mut app, model_window::SET_CAMERA_ACTION_ID, camera(1.0)).await;
    let window = app.config_generations(model_window::WINDOW_KIND_ID, "window-1").1.expect("the camera opened the window's config partition");
    for x in [2.0, 3.0] {
        press_in_window(&mut app, model_window::SET_CAMERA_ACTION_ID, [camera(x), gesture_args("camera:1", Some(false))].concat()).await;
    }
    assert_eq!(app.config_generations(model_window::WINDOW_KIND_ID, "window-1").1, Some(window), "window-config ticks publish nothing");
    press_in_window(&mut app, model_window::SET_CAMERA_ACTION_ID, [camera(4.0), gesture_args("camera:1", Some(true))].concat()).await;
    assert_eq!(app.config_generations(model_window::WINDOW_KIND_ID, "window-1").1, Some(window + 1), "the release publishes ONE window-config edit");
    press_in_window(&mut app, model_window::SET_CAMERA_ACTION_ID, [camera(5.0), gesture_args("camera:2", Some(false))].concat()).await;
    press_in_window(&mut app, model_window::SET_CAMERA_ACTION_ID, gesture_args("camera:2", None)).await;
    assert_eq!(app.config_generations(model_window::WINDOW_KIND_ID, "window-1").1, Some(window + 1), "a cancelled window-config press publishes nothing");
    assert_eq!(history_rows_and_mutations(&mut app).await, rows, "config edits are never history rows nor mutation rows");
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT);
}
//#endregion 🎚️ScrubLaws

//#region ⏯️SimulationToolRun
const RUN_FIXTURE: &str = include_str!("../../../🧵️simulation-session/🧪️tests/🧫️fixtures/🔣️.json");

fn run_fixture() -> serde_json::Value {
    serde_json::from_str(RUN_FIXTURE).expect("run fixture parses")
}

/// 🧩️ The editor over its real `SemioMembers` roster — `genesis_child_pack` mints the two composed
/// children at construction, and a `NoMembers` app would refuse their `s.stdio.semio` dialects.
type EnergyEditorApp = semio_framework_plugin::VcsArtifactApp<EditorApp<EnergyModelEditor>, semio_s_artifact_stdio_semio::SemioMembers>;

/// 🧫️ A registry-backed editor over the fixture scenario: the ANSI/ASHRAE 140 case with the fixture run
/// period, and the fixture settings published through `set-simulation-settings`.
async fn simulation_app() -> EnergyEditorApp {
    use semio_framework_plugin::PluginApp as _;
    let fixture = run_fixture();
    let scenario = &fixture["scenario"];
    let mut model = crate::examples::bestest_600::model();
    let period = &scenario["runPeriod"];
    model.run_period.start_month = period["startMonth"].as_u64().unwrap() as u8;
    model.run_period.start_day = period["startDay"].as_u64().unwrap() as u8;
    model.run_period.end_month = period["endMonth"].as_u64().unwrap() as u8;
    model.run_period.end_day = period["endDay"].as_u64().unwrap() as u8;
    let snapshot = snapshot_of(&model);
    let pack = <EnergyModelSnapshot as pack::snapshot::ArtifactPack>::encode_pack(&snapshot);
    let envelope = store::os_store::document_materialize::create_document_envelope::<EnergyModelSnapshot, EnergyModelMutation>(ENERGY_MODEL_DOCUMENT_SCHEMA, "model", snapshot, None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).into_owners();
    let spr = store::os_store::text_format::print_document_spr(&envelope).await.expect("fixture spr");
    let mut app = dispatchable_app().await;
    semio_framework_plugin::artifact_app_laws::load_document(&mut app, &store::os_store::serialization::ArtifactPackFiles { pack, spr, ops: String::new() }, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("fixture document loads");
    let settings = &scenario["settings"];
    let args = semio_framework_plugin::DslValue::Object(["zoneTimestepMinutes", "systemTimestepMinutes", "warmupDays"].iter().map(|key| (key.to_string(), semio_framework_plugin::DslValue::String(settings[key].to_string()))).collect());
    app.handle_action(simulation::SET_SETTINGS_ACTION_ID, Some(&args), &semio_framework_plugin::artifact_app_laws::meta("local"), &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("settings dispatch");
    settle(&mut app).await;
    let expected = format!("Warmup days: {}", settings["warmupDays"]);
    pump(&mut app, "settings publish", |text| text.contains(&expected), simulation::BODY_KEY).await;
    app
}

async fn render_text(app: &mut EnergyEditorApp, body_key: &str) -> String {
    use semio_framework_plugin::PluginApp as _;
    let tree = app.render(body_key, None, &semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.unwrap_or_else(|fault| panic!("render {body_key}: {fault:?}"));
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).unwrap_or_else(|error| panic!("project {body_key}: {error}"))
}

fn presence(app: &EnergyEditorApp) -> Option<protocol::PresenceToolRun> {
    use semio_framework_plugin::PluginApp as _;
    app.tool_run_presence()
}

fn state_of(app: &EnergyEditorApp) -> Option<&'static str> {
    presence(app).map(|presence| presence.state.wire_name())
}

async fn pump(app: &mut EnergyEditorApp, what: &str, done: impl Fn(&str) -> bool, body_key: &str) {
    use semio_framework_plugin::PluginApp as _;
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(600);
    while std::time::Instant::now() < deadline {
        if done(&render_text(app, body_key).await) {
            return;
        }
        for _ in 0..32 {
            app.advance_typed_operation_publication(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.preparation).await.unwrap_or_else(|fault| panic!("{what}: driver turn faulted: {fault:?}"));
        }
    }
    panic!("{what} never settled; run {:?}", presence(app));
}

async fn pump_state(app: &mut EnergyEditorApp, what: &str, state: &str) {
    use semio_framework_plugin::PluginApp as _;
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(600);
    while std::time::Instant::now() < deadline {
        if state_of(app) == Some(state) {
            settle(app).await;
            return;
        }
        app.advance_typed_operation_publication(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.preparation).await.unwrap_or_else(|fault| panic!("{what}: driver turn faulted: {fault:?}"));
    }
    panic!("{what} never reached {state}; run {:?}", presence(app));
}

/// 🪪️ The `{runId, generation}` the framework panel's buttons carry for the current run.
async fn run_arguments(app: &mut EnergyEditorApp) -> Vec<(String, semio_framework_plugin::DslValue)> {
    fn find(value: &serde_json::Value) -> Option<(String, u64)> {
        if let Some(object) = value.as_object() {
            if let (Some(run), Some(generation)) = (object.get("runId"), object.get("generation")) {
                let run = run.as_str().map(str::to_string).or_else(|| run.get("text").and_then(serde_json::Value::as_str).map(str::to_string)).or_else(|| run.as_u64().map(|value| value.to_string()))?;
                let generation = generation.as_u64().or_else(|| generation.as_f64().map(|value| value as u64)).or_else(|| generation.get("number").and_then(serde_json::Value::as_f64).map(|value| value as u64))?;
                return Some((run, generation));
            }
            return object.values().find_map(find);
        }
        value.as_array()?.iter().find_map(find)
    }
    let panel: serde_json::Value = serde_json::from_str(&render_text(app, semio_framework_plugin::FRAMEWORK_TOOL_RUN_BODY_KEY).await).expect("panel projection parses");
    let (run, generation) = find(&panel).unwrap_or_else(|| panic!("the run panel carries run arguments: {panel}"));
    vec![("runId".into(), semio_framework_plugin::DslValue::String(run)), ("generation".into(), semio_framework_plugin::DslValue::String(generation.to_string()))]
}

async fn run_action(app: &mut EnergyEditorApp, action: &str, arguments: Vec<(String, semio_framework_plugin::DslValue)>) -> semio_framework_plugin::DslValue {
    use semio_framework_plugin::PluginApp as _;
    app.handle_action(action, Some(&semio_framework_plugin::DslValue::Object(arguments)), &semio_framework_plugin::artifact_app_laws::meta("local"), &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.unwrap_or_else(|fault| panic!("{action}: {fault:?}")).output
}

async fn start(app: &mut EnergyEditorApp) {
    let output = run_action(app, semio_framework_tool_run::TOOL_RUN_START_ACTION_ID, vec![("toolId".into(), semio_framework_plugin::DslValue::String(tools::simulation::TOOL_ID.into()))]).await;
    assert_eq!(output.get("toolRun").and_then(semio_framework_plugin::DslValue::as_str), Some("spawnJob"), "the framework starts the simulation run");
}

/// 🧾️ Everything durable a run may never touch unless it finalizes a mutating result.
async fn durable(app: &mut EnergyEditorApp) -> (Vec<u8>, Vec<u8>, String) {
    use semio_framework_plugin::PluginApp as _;
    let pack = app.document_pack().await.expect("document pack");
    (pack.pack, pack.spr, format!("{:?}", app.history_snapshot().await.expect("history")))
}

/// 🔁️ Settles the retained settings operation through the host's exact continuation and ACK protocol.
async fn settle(app: &mut EnergyEditorApp) {
    semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(app, semio_framework_plugin::artifact_app_laws::meta("local").instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("the retained settings operation settles");
}

fn close(mut app: EnergyEditorApp) {
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT);
}

#[semio_framework_async_macros::async_test]
async fn the_simulation_is_driven_by_the_framework_tool_run_actions_and_chords_only() {
    let fixture = run_fixture();
    let def = definition();
    let tool = def.tools.iter().find(|tool| tool.id == tools::simulation::TOOL_ID).expect("the energy simulation tool is declared");
    assert!(tool.run.as_ref().is_some_and(|run| !run.mutating), "the simulation run is declared read-only");
    assert!(def.modes.iter().all(|mode| mode.tools.iter().any(|tool| tool.as_str() == tools::simulation::TOOL_ID)));
    for (action, chord) in fixture["lifecycle"]["reservedChords"].as_object().unwrap() {
        let binding = def.keybindings.iter().find(|binding| binding.keys == chord.as_str().unwrap()).unwrap_or_else(|| panic!("chord {chord} is bound"));
        assert_eq!(&binding.action.action, action, "chord {chord} drives the framework action");
    }
    for removed in fixture["lifecycle"]["removedActions"].as_array().unwrap().iter().map(|id| id.as_str().unwrap()) {
        assert!(!ENERGY_MODEL_RETAINED_TOOL_IDS.contains(&removed), "{removed} is still a retained tool");
        assert!(def.window_kinds.iter().flat_map(|window| semio_framework::window_kind_actions(&def, window)).all(|action| action.id != removed), "{removed} is still declared");
        assert!(<EnergyModelEditor as ArtifactEditor>::command_from_action(removed, None, None).is_err(), "{removed} still resolves to a command");
    }
}

/// 🏁️ A read-only run has nothing to review or publish, so the framework finalizes it as soon as its job
/// completes; the final tier readout stays in the run's steps and the document never changes.
#[semio_framework_async_macros::async_test]
async fn a_completed_simulation_run_finalizes_without_changing_the_document_or_history() {
    let fixture = run_fixture();
    let expected = &fixture["lifecycle"]["finalize"];
    let mut app = simulation_app().await;
    let before = durable(&mut app).await;
    start(&mut app).await;
    pump_state(&mut app, "run finalizes", expected["state"].as_str().unwrap()).await;
    let run = presence(&app).expect("finalized run");
    assert_eq!((run.completed, run.total), (fixture["fuel"]["ticksBeforeSettle"].as_u64().unwrap(), Some(fixture["fuel"]["ticksBeforeSettle"].as_u64().unwrap())));
    let window = render_text(&mut app, simulation::BODY_KEY).await;
    assert!(window.contains("The final result is accepted"), "the window reads the finalized run from the framework: {window}");
    let panel = render_text(&mut app, semio_framework_plugin::FRAMEWORK_TOOL_RUN_BODY_KEY).await;
    assert!(panel.contains("Final: "), "the final quality tier readout is published into the run steps: {panel}");
    let arguments = run_arguments(&mut app).await;
    let output = run_action(&mut app, semio_framework_tool_run::TOOL_RUN_FINALIZE_ACTION_ID, arguments).await;
    assert!(output.get("rejected").is_some(), "a finalized run accepts no second finalize");
    let after = durable(&mut app).await;
    assert_eq!(expected["documentPackUnchanged"].as_bool(), Some(before.0 == after.0 && before.1 == after.1), "finalizing a read-only run publishes no document edit");
    assert_eq!(expected["historyUnchanged"].as_bool(), Some(before.2 == after.2), "finalizing a read-only run adds no undo entry");
    close(app);
}

async fn pump_presence(app: &mut EnergyEditorApp, what: &str, done: impl Fn(&protocol::PresenceToolRun) -> bool) {
    use semio_framework_plugin::PluginApp as _;
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(600);
    while std::time::Instant::now() < deadline {
        if presence(app).as_ref().is_some_and(&done) {
            return;
        }
        app.advance_typed_operation_publication(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.preparation).await.unwrap_or_else(|fault| panic!("{what}: driver turn faulted: {fault:?}"));
    }
    panic!("{what} never settled; run {:?}", presence(app));
}

#[semio_framework_async_macros::async_test]
async fn aborting_a_simulation_run_leaves_the_document_byte_identical() {
    let fixture = run_fixture();
    let expected = &fixture["lifecycle"]["abort"];
    let mut app = simulation_app().await;
    let before = durable(&mut app).await;
    start(&mut app).await;
    pump_presence(&mut app, "first timesteps", |run| run.completed > 0).await;
    let arguments = run_arguments(&mut app).await;
    let output = run_action(&mut app, semio_framework_tool_run::TOOL_RUN_ABORT_ACTION_ID, arguments).await;
    assert_eq!(output.get("toolRun").and_then(semio_framework_plugin::DslValue::as_str), Some("closeJob"));
    pump_state(&mut app, "abort settles", expected["state"].as_str().unwrap()).await;
    let after = durable(&mut app).await;
    assert_eq!(expected["documentPackUnchanged"].as_bool(), Some(before.0 == after.0 && before.1 == after.1), "abort leaves the document byte-identical");
    assert_eq!(expected["historyUnchanged"].as_bool(), Some(before.2 == after.2), "abort leaves no history trace");
    assert!(render_text(&mut app, simulation::BODY_KEY).await.contains("No result was accepted"));
    close(app);
}

#[semio_framework_async_macros::async_test]
async fn a_paused_simulation_run_steps_exactly_one_timestep_per_step() {
    let fixture = run_fixture();
    let mut app = simulation_app().await;
    start(&mut app).await;
    pump_presence(&mut app, "first timesteps", |run| run.completed > 0).await;
    let arguments = run_arguments(&mut app).await;
    assert_eq!(run_action(&mut app, semio_framework_tool_run::TOOL_RUN_PAUSE_ACTION_ID, arguments).await.get("toolRun").and_then(semio_framework_plugin::DslValue::as_str), Some("stopScheduling"));
    pump_state(&mut app, "pause settles", "paused").await;
    let mut completed = presence(&app).expect("paused run").completed;
    for _ in 0..3 {
        let arguments = run_arguments(&mut app).await;
        assert_eq!(run_action(&mut app, semio_framework_tool_run::TOOL_RUN_STEP_ACTION_ID, arguments).await.get("toolRun").and_then(semio_framework_plugin::DslValue::as_str), Some("driveOneUnit"));
        pump_state(&mut app, "single step settles", "paused").await;
        let after = presence(&app).expect("paused run").completed;
        assert_eq!(after - completed, fixture["fuel"]["completedAdvancePerTick"].as_u64().unwrap(), "one step computes exactly one timestep");
        completed = after;
    }
    let arguments = run_arguments(&mut app).await;
    run_action(&mut app, semio_framework_tool_run::TOOL_RUN_ABORT_ACTION_ID, arguments).await;
    pump_state(&mut app, "abort settles", "aborted").await;
    close(app);
}

#[semio_framework_async_macros::async_test]
async fn changing_the_simulation_settings_restarts_a_live_run_in_its_next_generation() {
    use semio_framework_plugin::DslValue;
    let mut app = simulation_app().await;
    start(&mut app).await;
    pump_presence(&mut app, "first timesteps", |run| run.completed > 1).await;
    let arguments = run_arguments(&mut app).await;
    run_action(&mut app, semio_framework_tool_run::TOOL_RUN_PAUSE_ACTION_ID, arguments).await;
    pump_state(&mut app, "pause settles", "paused").await;
    let generation = run_arguments(&mut app).await[1].1.clone();
    run_action(&mut app, simulation::SET_SETTINGS_ACTION_ID, vec![("zoneTimestepMinutes".into(), DslValue::String("30".into())), ("systemTimestepMinutes".into(), DslValue::String("30".into())), ("warmupDays".into(), DslValue::String("1".into()))]).await;
    settle(&mut app).await;
    assert!(render_text(&mut app, simulation::BODY_KEY).await.contains("Zone timestep: 30 min"), "the settings reach the config store");
    let mut arguments = run_arguments(&mut app).await;
    for _ in 0..256 {
        if arguments[1].1 != generation {
            break;
        }
        semio_framework_plugin::PluginApp::advance_typed_operation_publication(&mut app).await.expect("reconfigure turn");
        arguments = run_arguments(&mut app).await;
    }
    assert_ne!(arguments[1].1, generation, "a settings change moves the run to its next generation");
    assert_eq!(state_of(&app), Some("paused"), "reconfigure keeps the paused state");
    run_action(&mut app, semio_framework_tool_run::TOOL_RUN_STEP_ACTION_ID, arguments).await;
    pump_state(&mut app, "restarted step settles", "paused").await;
    assert_eq!(presence(&app).expect("restarted run").completed, 1, "reconfigure: restart recomputes from the first timestep");
    let arguments = run_arguments(&mut app).await;
    run_action(&mut app, semio_framework_tool_run::TOOL_RUN_ABORT_ACTION_ID, arguments).await;
    pump_state(&mut app, "abort settles", "aborted").await;
    close(app);
}
//#endregion 🧵️DispatchLaw

/// ⚖️ LAW: every declared chord uses canonical key tokens — `ShellHost` compares the last `+`
/// segment to `event.key` verbatim, so `mod+period` is a dead chord (`event.key` is `"."`).
#[semio_framework_async_macros::async_test]
async fn every_keybinding_uses_canonical_punctuation_key_tokens() {
    const FORBIDDEN: &[&str] = &["period", "comma", "slash", "minus", "equal", "semicolon", "quote", "backquote", "bracketleft", "bracketright", "backslash"];
    let def = definition();
    for binding in &def.keybindings {
        for chord in binding.keys.split(',') {
            let key = chord.trim().split('+').last().unwrap_or("").to_lowercase();
            assert!(!FORBIDDEN.iter().any(|name| *name == key), "dead chord token `{key}` in `{binding}`", binding = binding.keys);
        }
    }
    let abort = def.keybindings.iter().find(|binding| binding.keys == "mod+.").expect("the abort chord is bound");
    assert_eq!(abort.action.action, semio_framework_tool_run::TOOL_RUN_ABORT_ACTION_ID, "mod+. aborts the framework tool run, never a plugin verb");
}

#[semio_framework_async_macros::async_test]
async fn a_command_round_trips_through_its_own_text_and_binary_codec() {
    use protocol::{OpBinary as _, OpText as _};
    let command = EnergyModelEditorCommand::RenameZone { zone: 7, new_name: "Kitchen".into() };
    assert_eq!(EnergyModelEditorCommand::parse_op(&command.print_op()).expect("text round trip"), command);
    assert_eq!(EnergyModelEditorCommand::decode_op(&command.encode_op().expect("encode")).expect("binary round trip"), command);
}
