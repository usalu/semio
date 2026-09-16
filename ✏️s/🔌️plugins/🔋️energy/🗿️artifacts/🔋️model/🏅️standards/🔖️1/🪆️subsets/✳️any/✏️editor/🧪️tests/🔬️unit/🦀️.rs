use super::*;
use std::collections::BTreeSet;

fn definition() -> semio_framework_plugin::AppDefinition {
    create_energy_model_editor()
}

#[semio_framework_async_macros::async_test]
async fn create_energy_model_editor_builds_a_definition_for_the_editor_role() {
    let def = definition();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, MODEL_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<EnergyModelEditor as ArtifactEditor>::DIALECT, MODEL_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_declares_every_window() {
    let def = definition();
    for id in [structure::WINDOW_KIND_ID, zones::WINDOW_KIND_ID, simulation::WINDOW_KIND_ID, model_window::WINDOW_KIND_ID] {
        assert!(def.window_kinds.iter().any(|window| window.id == id), "missing window kind {id}");
    }
    // 🧊️ The 3d window is the one bound to the shared selection/hover domain — that binding is what
    // makes a viewport pick and a tree pick the same framework event.
    let world = def.window_kinds.iter().find(|window| window.id == model_window::WINDOW_KIND_ID).expect("the 3d model window");
    assert!(world.interactions.iter().any(|reference| reference.as_str() == ENERGY_MODEL_INTERACTION_DOMAIN), "the 3d window is not bound to the energy interaction domain: {:?}", world.interactions);
    assert!(def.interactions.iter().any(|domain| domain.id == ENERGY_MODEL_INTERACTION_DOMAIN), "the domain itself is declared exactly once on the app");
}

/// 🧵️ The framework demands set equality between `TOOL_JOB_IDS`, the `Migrated` action ids, the
/// factory's `TOOL_IDS`, its per-tool publication contracts and the proof rows. Any drift is
/// `interactive-job.catalog-incomplete`/`catalog-authority` at app construction, so it is checked
/// here rather than discovered at dispatch.
#[semio_framework_async_macros::async_test]
async fn retained_roster_is_exact_and_exhaustive() {
    use semio_framework_plugin::ArtifactOwnedToolJobFactory;
    let roster = ENERGY_MODEL_RETAINED_TOOL_IDS.iter().copied().collect::<BTreeSet<_>>();
    assert_eq!(roster.len(), ENERGY_MODEL_RETAINED_TOOL_IDS.len(), "the retained roster repeats a tool id");
    assert_eq!(roster, <EnergyModelEditorCommand as protocol::OpBinary>::TOOL_JOB_IDS.iter().copied().collect::<BTreeSet<_>>());
    assert_eq!(roster, <EnergyModelCommandJobFactory as ArtifactOwnedToolJobFactory>::TOOL_IDS.iter().copied().collect::<BTreeSet<_>>());
    assert_eq!(roster, <EnergyModelCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.iter().map(|contract| contract.tool_id).collect::<BTreeSet<_>>());
    assert_eq!(roster, <EnergyModelEditor as ArtifactEditor>::bounded_first_step_tool_proofs().iter().map(|proof| proof.tool_id()).collect::<BTreeSet<_>>());
    let def = definition();
    let migrated = def.window_kinds.iter().flat_map(|window| window.actions.iter()).filter(|action| action.semantics.execution.interactive_job == InteractiveJobClassification::Migrated).map(|action| action.id.as_str()).collect::<BTreeSet<_>>();
    assert!(roster.is_subset(&migrated), "unclassified retained tools: {:?}", roster.difference(&migrated).collect::<Vec<_>>());
}

#[semio_framework_async_macros::async_test]
async fn every_declared_action_is_classified_and_resolves_to_a_command() {
    for action in definition().window_kinds.iter().flat_map(|window| window.actions.iter()) {
        assert_ne!(action.semantics.execution.interactive_job, InteractiveJobClassification::Unclassified, "action {} is unclassified", action.id);
    }
    for tool_id in ENERGY_MODEL_RETAINED_TOOL_IDS {
        let command = <EnergyModelEditor as ArtifactEditor>::command_from_action(tool_id, None).unwrap_or_else(|error| panic!("action {tool_id} has no command: {error:?}"));
        assert_eq!(command.action_id(), *tool_id);
    }
}

#[semio_framework_async_macros::async_test]
async fn document_verbs_publish_to_the_artifact_lane_settings_to_the_config_lane_and_examples_to_the_host() {
    use semio_framework_plugin::ArtifactOwnedToolJobFactory;
    for contract in <EnergyModelCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS {
        let expected = match contract.tool_id {
            id if ENERGY_MODEL_DOCUMENT_TOOL_IDS.contains(&id) => ArtifactToolPublicationLane::Artifact,
            simulation::SET_SETTINGS_ACTION_ID => ArtifactToolPublicationLane::Config,
            _ => ArtifactToolPublicationLane::HostOnly,
        };
        assert_eq!(contract.lanes, &[expected], "wrong publication lane for {}", contract.tool_id);
    }
    assert!(<EnergyModelEditor as ArtifactEditor>::build_artifact_store_one_item_preparation_factory().is_some(), "the artifact lane needs its one-item preparation factory");
    assert!(<EnergyModelEditor as ArtifactEditor>::build_config_store_one_item_preparation_factory().is_some(), "the config lane needs its one-item preparation factory");
}

#[semio_framework_async_macros::async_test]
async fn examples_are_registered_for_the_shell_picker() {
    let sources = examples();
    assert_eq!(sources.len(), example_rows().len());
    assert!(sources.len() >= 15, "the demo plus the fifteen ANSI/ASHRAE 140 cases must all reach the picker");
    let mut seen = BTreeSet::new();
    for source in &sources {
        assert!(seen.insert(source.id().to_string()), "duplicate example id {}", source.id());
        assert!(!source.document_json().trim().is_empty(), "example {} carries no document", source.id());
        assert!(example_model(source.id()).is_some(), "example {} has no model behind its picker row", source.id());
    }
    let options = example_options().into_iter().map(|option| option.value).collect::<Vec<_>>();
    assert_eq!(options, sources.iter().map(|source| source.id().to_string()).collect::<Vec<_>>(), "the picker's option list must be the example list");
    let definition = create_energy_model_editor();
    let picker = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == SET_ACTIVE_EXAMPLE_ACTION_ID).expect("the shell dispatches setActiveExample, so the app must declare it on a window");
    assert_eq!(picker.args.len(), 1);
    assert_eq!(picker.args[0].id, "exampleId");
}

/// 📂️ Loading an example is a whole-document swap, and this artifact's vocabulary has NO
/// whole-document-replace kind on purpose. So the verb publishes a `kernel::Effect::LoadDocument`
/// (the host's `ArtifactStore::reset` route, outside undo history) and NOT a single mutation —
/// asserted for every bundled example, so a new row can never silently fall back on the seam.
#[semio_framework_async_macros::async_test]
async fn loading_an_example_swaps_the_document_through_an_effect_and_never_a_mutation() {
    let snapshot = EnergyModelSnapshot::default();
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    for (id, _, model) in example_rows() {
        let emit = reduce(&EnergyModelEditorCommand::SetActiveExample { example_id: id.to_string() }, &doc).unwrap_or_else(|error| panic!("example {id} must load: {error:?}"));
        assert!(emit.artifact_mutations.is_empty(), "example {id} must not reach the mutation seam");
        let [semio_framework_plugin::kernel::Effect::LoadDocument { pack, spr }] = emit.effects.as_slice() else { panic!("example {id} must emit exactly one LoadDocument effect") };
        assert!(!pack.is_empty() && !spr.is_empty(), "example {id} emitted an empty document");
        let loaded = <EnergyModelSnapshot as store::ArtifactPack>::decode_pack(pack).expect("the emitted pack decodes");
        assert_eq!(loaded.model, model, "example {id} loaded a different model than its own leaf declares");
    }
    let fault = reduce(&EnergyModelEditorCommand::SetActiveExample { example_id: "nonsense".into() }, &doc).err().expect("an unknown example id is refused");
    assert_eq!(fault.code.0.as_str(), "mutation.target-missing");
}

fn model_with_one_zone() -> crate::model::Model {
    let mut model = crate::model::Model::default();
    model.zones.push(Zone { id: EntityId(1), name: "Zone 1".into(), volume_m3: 100.0, multiplier: 1, conditioned: true, part_of_total_floor_area: true });
    model
}

/// 🧫️ One zone, two constructions, one material, two constant schedules, one surface and one
/// thermostat — the smallest document in which every authored document verb has a real target.
fn populated_model() -> crate::model::Model {
    let mut model = model_with_one_zone();
    model.constructions.push(crate::model::Construction { id: EntityId(1), name: "Wall".into(), layer_material_ids: vec![EntityId(1)] });
    model.constructions.push(crate::model::Construction { id: EntityId(2), name: "Roof".into(), layer_material_ids: vec![EntityId(1)] });
    model.materials.push(Material {
        id: EntityId(1),
        name: "Concrete".into(),
        roughness: crate::model::SurfaceRoughness::MediumRough,
        thickness_m: 0.1,
        conductivity_w_m_k: 1.0,
        density_kg_m3: 2000.0,
        specific_heat_j_kg_k: 900.0,
        thermal_absorptance: 0.9,
        solar_absorptance: 0.6,
        visible_absorptance: 0.6,
    });
    model.surfaces.push(Surface {
        id: EntityId(1),
        name: "South".into(),
        zone_id: EntityId(1),
        class: SurfaceClass::ExteriorWall,
        vertices_m: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 1.0]],
        construction_id: EntityId(1),
        outside_boundary_condition: OutsideBoundary::OutdoorAir,
        sun_exposed: true,
        wind_exposed: true,
        multiplier: 1,
    });
    model.schedules.constants.push(crate::schedule::ConstantSchedule { id: ScheduleId(1), value: 20.0 });
    model.schedules.constants.push(crate::schedule::ConstantSchedule { id: ScheduleId(2), value: 27.0 });
    model.thermostats.push(Thermostat { id: EntityId(1), zone_id: EntityId(1), heating_setpoint_schedule_id: ScheduleId(1), cooling_setpoint_schedule_id: ScheduleId(2), heating_throttle_range_k: 2.0, cooling_throttle_range_k: 2.0 });
    model
}

fn snapshot_of(model: &crate::model::Model) -> EnergyModelSnapshot {
    crate::energy_snapshot_with_state(ENERGY_MODEL_DOCUMENT_SCHEMA, model, None)
}

fn applied(snapshot: &EnergyModelSnapshot, command: &EnergyModelEditorCommand) -> crate::model::Model {
    use protocol::Mutation as _;
    let history = HistoryView::empty();
    let doc = ArtifactView::new(snapshot, &history);
    let emit = reduce(command, &doc).expect("document verb reduces");
    let mut next = snapshot.clone();
    for mutation in &emit.artifact_mutations {
        next = protocol::MutationDiff::apply(mutation.diff(&next).diff(), &next).expect("diff applies");
    }
    next.model
}

/// 🧬️ The verbs whose semantic mutation kinds W-D0's model-root group has already landed —
/// these must produce real granular steps, never a refusal.
#[semio_framework_async_macros::async_test]
async fn landed_semantic_kinds_produce_granular_mutations() {
    let snapshot = snapshot_of(&model_with_one_zone());
    let renamed = applied(&snapshot, &EnergyModelEditorCommand::RenameZone { zone: 1, new_name: "Loft".into() });
    assert_eq!(renamed.zones[0].name, "Loft");
    let sited = applied(&snapshot, &EnergyModelEditorCommand::SetSite { latitude_deg: 39.74, longitude_deg: -105.18, elevation_m: 1609.0, time_zone_hours: -7.0, north_axis_deg: 0.0 });
    assert!((sited.site.latitude_deg - 39.74).abs() < 1e-9);
    let period = applied(&snapshot, &EnergyModelEditorCommand::SetRunPeriod { start_month: 2, start_day: 1, end_month: 2, end_day: 28 });
    assert_eq!((period.run_period.start_month, period.run_period.end_day), (2, 28));
    let field = applied(&snapshot, &EnergyModelEditorCommand::SetStructureField { field: "name".into(), value: "BESTEST 600".into() });
    assert_eq!(field.name, "BESTEST 600");
    let cell = applied(&snapshot, &EnergyModelEditorCommand::SetZoneCell { row: 0, column: "volumeM3".into(), value: "129.6".into() });
    assert!((cell.zones[0].volume_m3 - 129.6).abs() < 1e-9);
}

/// 🧬️ The verbs that were waiting on the 100s–900s mutation groups. Those leaves have landed
/// (`create-zone`, `delete-zone`, `create-surface`, `delete-surface`, `change-surface-*`,
/// `change-material-*`, `change-thermostat-*`), so this is now a strict ROUND TRIP: each verb
/// must produce granular semantic steps whose application reproduces exactly the edit the user
/// asked for. A `mutation.kind-unavailable` here is a real regression, not a pending group.
#[semio_framework_async_macros::async_test]
async fn every_document_verb_round_trips_through_the_granular_vocabulary() {
    let snapshot = snapshot_of(&populated_model());

    let created = applied(&snapshot, &EnergyModelEditorCommand::CreateZone { name: "Attic".into(), volume_m3: 40.0, multiplier: 1, conditioned: false });
    assert_eq!(created.zones.len(), 2);
    assert_eq!(created.zones[1].name, "Attic");
    assert!(!created.zones[1].conditioned);

    let with_surface = applied(&snapshot, &EnergyModelEditorCommand::CreateSurface { name: "North".into(), zone: 1, construction: 1, class: "roof".into() });
    assert_eq!(with_surface.surfaces.len(), 2);
    assert_eq!(with_surface.surfaces[1].name, "North");
    assert_eq!(with_surface.surfaces[1].class, SurfaceClass::Roof);

    let without_surface = applied(&snapshot, &EnergyModelEditorCommand::DeleteSurface { surface: 1 });
    assert!(without_surface.surfaces.is_empty());

    let reconstructed = applied(&snapshot, &EnergyModelEditorCommand::AssignSurfaceConstruction { surface: 1, construction: 2 });
    assert_eq!(reconstructed.surfaces[0].construction_id, EntityId(2));

    let insulated = applied(&snapshot, &EnergyModelEditorCommand::SetMaterialProperty { material: 1, property: "conductivityWMK".into(), value: 0.04 });
    assert!((insulated.materials[0].conductivity_w_m_k - 0.04).abs() < 1e-12);

    let retuned = applied(&snapshot, &EnergyModelEditorCommand::SetThermostatSetpoints { thermostat: 1, heating_schedule: 2, cooling_schedule: 1, heating_throttle_range_k: 1.0, cooling_throttle_range_k: 1.5 });
    assert_eq!(retuned.thermostats[0].heating_setpoint_schedule_id, ScheduleId(2));
    assert!((retuned.thermostats[0].cooling_throttle_range_k - 1.5).abs() < 1e-12);

    let mut freed = populated_model();
    freed.surfaces.clear();
    freed.thermostats.clear();
    let dropped = applied(&snapshot_of(&freed), &EnergyModelEditorCommand::DeleteZone { zone: 1 });
    assert!(dropped.zones.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn a_zone_still_referenced_by_a_surface_cannot_be_deleted() {
    let mut model = model_with_one_zone();
    model.constructions.push(crate::model::Construction { id: EntityId(1), name: "Wall".into(), layer_material_ids: Vec::new() });
    model.surfaces.push(Surface {
        id: EntityId(1),
        name: "South".into(),
        zone_id: EntityId(1),
        class: SurfaceClass::ExteriorWall,
        vertices_m: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 1.0]],
        construction_id: EntityId(1),
        outside_boundary_condition: OutsideBoundary::OutdoorAir,
        sun_exposed: true,
        wind_exposed: true,
        multiplier: 1,
    });
    let snapshot = snapshot_of(&model);
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let fault = reduce(&EnergyModelEditorCommand::DeleteZone { zone: 1 }, &doc).err().expect("a referenced zone is refused");
    assert_eq!(fault.code.0.as_str(), "mutation.target-in-use");
}

/// 🧱️ The SI guards run BEFORE the seam, so they are observable whatever the vocabulary state.
#[semio_framework_async_macros::async_test]
async fn out_of_range_payloads_are_refused_before_they_reach_the_vocabulary() {
    let mut model = model_with_one_zone();
    model.materials.push(Material {
        id: EntityId(1),
        name: "Concrete".into(),
        roughness: crate::model::SurfaceRoughness::MediumRough,
        thickness_m: 0.1,
        conductivity_w_m_k: 1.0,
        density_kg_m3: 2000.0,
        specific_heat_j_kg_k: 900.0,
        thermal_absorptance: 0.9,
        solar_absorptance: 0.6,
        visible_absorptance: 0.6,
    });
    let snapshot = snapshot_of(&model);
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    for command in [
        EnergyModelEditorCommand::SetMaterialProperty { material: 1, property: "conductivityWMK".into(), value: -1.0 },
        EnergyModelEditorCommand::SetSite { latitude_deg: 120.0, longitude_deg: 0.0, elevation_m: 0.0, time_zone_hours: 0.0, north_axis_deg: 0.0 },
        EnergyModelEditorCommand::SetRunPeriod { start_month: 13, start_day: 1, end_month: 12, end_day: 31 },
        EnergyModelEditorCommand::CreateZone { name: "Void".into(), volume_m3: 0.0, multiplier: 1, conditioned: true },
    ] {
        let fault = reduce(&command, &doc).err().expect("an out-of-range payload is refused");
        assert_eq!(fault.code.0.as_str(), "mutation.invalid-payload", "{} failed for the wrong reason", command.action_id());
    }
    let fault = reduce(&EnergyModelEditorCommand::SetMaterialProperty { material: 9, property: "conductivityWMK".into(), value: 0.04 }, &doc).err().expect("an unknown material is refused");
    assert_eq!(fault.code.0.as_str(), "mutation.target-missing");
}

//#region 🧵️DispatchLaw
/// 🧬️ `artifact_app_laws::new_app_with_registry` needs the `App { definition, examples }` shape, which is
/// also the exact shape the plugin root registers — so a drift between the two would be caught
/// here rather than at boot.
pub(crate) fn energy_model_manifest_for_tests() -> semio_framework_plugin::App {
    semio_framework_plugin::App { definition: create_energy_model_editor(), examples: examples().into_iter().map(Into::into).collect() }
}

/// 🧵️ A registry-backed app bound to the live runtime instance `meta("local")` addresses. The
/// registry-LESS `artifact_app_laws::new_app` cannot be used here: it builds an app with no
/// `AppActionRegistry`, so `migrated_tool_ids()` is empty and `validate_tool_job_rows` fails
/// closed with `interactive-job.catalog-authority` the moment any proof is declared.
async fn dispatchable_app() -> EnergyEditorApp {
    use semio_framework_plugin::PluginApp as _;
    let mut app = semio_framework_plugin::artifact_app_laws::new_app_with_registry_and_members::<EditorApp<EnergyModelEditor>, semio_s_artifact_stdio_semio::SemioMembers>(energy_model_manifest_for_tests).await;
    app.bind_instance_id(semio_framework_plugin::artifact_app_laws::meta("local").instance_id).await;
    app
}

/// 🧵️ THE dispatch law: every declared verb must reach this editor's own reducer. A domain fault
/// (`mutation.target-missing` for an id absent from the empty default document,
/// `mutation.kind-unavailable` for a vocabulary group still landing, or an `energy.session.*`
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
        let action = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == *tool_id).unwrap_or_else(|| panic!("action {tool_id} is declared on no window kind"));
        let staged = effective_action_args(&action.args, &DslValue::Object(Vec::new()), None);
        match app.handle_action(tool_id, Some(&staged), &semio_framework_plugin::artifact_app_laws::meta("local")).await {
            Ok(_) => reached += 1,
            Err(fault) => {
                assert!(!fault.code.0.as_str().starts_with("interactive-job."), "action {tool_id} never reached the app: {} — {}", fault.code.0.as_str(), fault.message);
                reached += 1;
            }
        }
    }
    assert_eq!(reached, ENERGY_MODEL_RETAINED_TOOL_IDS.len());
    let _ = semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app, semio_framework_plugin::artifact_app_laws::meta("local").instance_id).await;
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
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
    let result = app.handle_action(SET_NODE_ACTION_ID, Some(&args), &semio_framework_plugin::artifact_app_laws::meta("local")).await.expect("set-node dispatches without a fault");
    assert!(!result.mutations.is_empty() || app.has_pending_typed_operations(), "the rename neither published nor retained an operation");
    let _ = semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app, semio_framework_plugin::artifact_app_laws::meta("local").instance_id).await;
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
}
//#region ⏯️SimulationToolRun
const RUN_FIXTURE: &str = include_str!("../../🧵️simulation-session/🧫️fixtures/🔣️.json");

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
    let pack = <EnergyModelSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
    let envelope = store::create_document_envelope::<EnergyModelSnapshot, EnergyModelMutation>(ENERGY_MODEL_DOCUMENT_SCHEMA, "model", snapshot, None).into_owners();
    let spr = store::print_document_spr(&envelope).await.expect("fixture spr");
    let mut app = dispatchable_app().await;
    app.load_document_pack(&store::ArtifactPackFiles { pack, spr, ops: String::new() }).await.expect("fixture document loads");
    let settings = &scenario["settings"];
    let args = semio_framework_plugin::DslValue::Object(["zoneTimestepMinutes", "systemTimestepMinutes", "warmupDays"].iter().map(|key| (key.to_string(), semio_framework_plugin::DslValue::String(settings[key].to_string()))).collect());
    app.handle_action(simulation::SET_SETTINGS_ACTION_ID, Some(&args), &semio_framework_plugin::artifact_app_laws::meta("local")).await.expect("settings dispatch");
    settle(&mut app).await;
    let expected = format!("Warmup days: {}", settings["warmupDays"]);
    pump(&mut app, "settings publish", |text| text.contains(&expected), simulation::BODY_KEY).await;
    app
}

async fn render_text(app: &mut EnergyEditorApp, body_key: &str) -> String {
    use semio_framework_plugin::PluginApp as _;
    let tree = app.render(body_key, None, &semio_framework_plugin::ViewModel::default()).await.unwrap_or_else(|fault| panic!("render {body_key}: {fault:?}"));
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
            app.advance_typed_operation_publication().await.unwrap_or_else(|fault| panic!("{what}: driver turn faulted: {fault:?}"));
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
        app.advance_typed_operation_publication().await.unwrap_or_else(|fault| panic!("{what}: driver turn faulted: {fault:?}"));
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
    app.handle_action(action, Some(&semio_framework_plugin::DslValue::Object(arguments)), &semio_framework_plugin::artifact_app_laws::meta("local")).await.unwrap_or_else(|fault| panic!("{action}: {fault:?}")).output
}

async fn start(app: &mut EnergyEditorApp) {
    let output = run_action(app, semio_framework_plugin::TOOL_RUN_START_ACTION_ID, vec![("toolId".into(), semio_framework_plugin::DslValue::String(tools::simulation::TOOL_ID.into()))]).await;
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
    semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(app, semio_framework_plugin::artifact_app_laws::meta("local").instance_id).await.expect("the retained settings operation settles");
}

fn close(mut app: EnergyEditorApp) {
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
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
        assert!(def.window_kinds.iter().flat_map(|window| window.actions.iter()).all(|action| action.id != removed), "{removed} is still declared");
        assert!(<EnergyModelEditor as ArtifactEditor>::command_from_action(removed, None).is_err(), "{removed} still resolves to a command");
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
    let output = run_action(&mut app, semio_framework_plugin::TOOL_RUN_FINALIZE_ACTION_ID, arguments).await;
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
        app.advance_typed_operation_publication().await.unwrap_or_else(|fault| panic!("{what}: driver turn faulted: {fault:?}"));
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
    let output = run_action(&mut app, semio_framework_plugin::TOOL_RUN_ABORT_ACTION_ID, arguments).await;
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
    assert_eq!(run_action(&mut app, semio_framework_plugin::TOOL_RUN_PAUSE_ACTION_ID, arguments).await.get("toolRun").and_then(semio_framework_plugin::DslValue::as_str), Some("stopScheduling"));
    pump_state(&mut app, "pause settles", "paused").await;
    let mut completed = presence(&app).expect("paused run").completed;
    for _ in 0..3 {
        let arguments = run_arguments(&mut app).await;
        assert_eq!(run_action(&mut app, semio_framework_plugin::TOOL_RUN_STEP_ACTION_ID, arguments).await.get("toolRun").and_then(semio_framework_plugin::DslValue::as_str), Some("driveOneUnit"));
        pump_state(&mut app, "single step settles", "paused").await;
        let after = presence(&app).expect("paused run").completed;
        assert_eq!(after - completed, fixture["fuel"]["completedAdvancePerTick"].as_u64().unwrap(), "one step computes exactly one timestep");
        completed = after;
    }
    let arguments = run_arguments(&mut app).await;
    run_action(&mut app, semio_framework_plugin::TOOL_RUN_ABORT_ACTION_ID, arguments).await;
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
    run_action(&mut app, semio_framework_plugin::TOOL_RUN_PAUSE_ACTION_ID, arguments).await;
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
    run_action(&mut app, semio_framework_plugin::TOOL_RUN_STEP_ACTION_ID, arguments).await;
    pump_state(&mut app, "restarted step settles", "paused").await;
    assert_eq!(presence(&app).expect("restarted run").completed, 1, "reconfigure: restart recomputes from the first timestep");
    let arguments = run_arguments(&mut app).await;
    run_action(&mut app, semio_framework_plugin::TOOL_RUN_ABORT_ACTION_ID, arguments).await;
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
    assert_eq!(abort.action.action, semio_framework_plugin::TOOL_RUN_ABORT_ACTION_ID, "mod+. aborts the framework tool run, never a plugin verb");
}

#[semio_framework_async_macros::async_test]
async fn a_command_round_trips_through_its_own_text_and_binary_codec() {
    use protocol::{OpBinary as _, OpText as _};
    let command = EnergyModelEditorCommand::RenameZone { zone: 7, new_name: "Kitchen".into() };
    assert_eq!(EnergyModelEditorCommand::parse_op(&command.print_op()).expect("text round trip"), command);
    assert_eq!(EnergyModelEditorCommand::decode_op(&command.encode_op().expect("encode")).expect("binary round trip"), command);
}

//#region 🔍️InspectorVerbs
/// 🧫️ ANSI/ASHRAE 140 case 600 — the smallest REAL document that carries a window, a glazing stack,
/// a gas gap and a thermostat at once, so the three inspector verbs all have live targets.
fn inspector_model() -> crate::model::Model {
    crate::examples::demo::model()
}

fn emitted_kinds(snapshot: &EnergyModelSnapshot, command: &EnergyModelEditorCommand) -> Vec<String> {
    use protocol::SemanticMutation as _;
    let history = HistoryView::empty();
    let doc = ArtifactView::new(snapshot, &history);
    reduce(command, &doc).expect("the verb reduces").artifact_mutations.iter().map(|mutation| mutation.semantics().kind.to_string()).collect()
}

fn refusal(snapshot: &EnergyModelSnapshot, command: &EnergyModelEditorCommand) -> String {
    let history = HistoryView::empty();
    let doc = ArtifactView::new(snapshot, &history);
    reduce(command, &doc).err().expect("the verb refuses").code.0.as_str().to_string()
}

fn fenestration_property(fenestration: u32, property: &str, value: &str) -> EnergyModelEditorCommand {
    EnergyModelEditorCommand::SetFenestrationProperty { fenestration, property: property.into(), value: value.into() }
}

fn surface_property(surface: u32, property: &str, value: &str) -> EnergyModelEditorCommand {
    EnergyModelEditorCommand::SetSurfaceProperty { surface, property: property.into(), value: value.into(), partner_surface: 0 }
}

fn zone_property(zone: u32, property: &str, value: &str) -> EnergyModelEditorCommand {
    EnergyModelEditorCommand::SetZoneProperty { zone, property: property.into(), value: value.into() }
}

/// 🪟️ THE regression this ticket opened on: before `diff_fenestrations` existed, a window field edit
/// reduced cleanly and emitted NOTHING — `model_edit`'s probe copied the whole collection verbatim,
/// so neither a mutation nor a `kind-unavailable` fault ever reached the caller. A u-value edit must
/// now emit EXACTLY ONE `change-fenestration-u-value` and nothing else.
#[semio_framework_async_macros::async_test]
async fn a_window_u_value_edit_emits_exactly_one_granular_mutation() {
    let snapshot = snapshot_of(&inspector_model());
    let kinds = emitted_kinds(&snapshot, &fenestration_property(50, "uValueWM2K", "1.4"));
    assert_eq!(kinds, vec!["change-fenestration-u-value".to_string()], "a single field edit is a single granular step");
    let applied = applied(&snapshot, &fenestration_property(50, "uValueWM2K", "1.4"));
    let window = applied.fenestrations.iter().find(|window| window.id == EntityId(50)).expect("the window survives its own edit");
    assert!((window.u_value_w_m2k - 1.4).abs() < 1e-12, "the edit reaches the document");
}

/// 🪟️ Every fenestration field the inspector binds round-trips through its own granular kind.
#[semio_framework_async_macros::async_test]
async fn every_fenestration_property_round_trips_through_the_granular_vocabulary() {
    let snapshot = snapshot_of(&inspector_model());
    let cases: &[(&str, &str, &str)] = &[
        ("name", "South Left", "rename-fenestration"),
        ("uValueWM2K", "1.4", "change-fenestration-u-value"),
        ("shgc", "0.4", "change-fenestration-shgc"),
        ("vlt", "0.5", "change-fenestration-vlt"),
        ("areaM2", "5.5", "change-fenestration-area"),
        ("heightM", "1.7", "change-fenestration-height"),
        ("sillHeightM", "0.9", "change-fenestration-sill-height"),
        ("frameConductanceWK", "0.3", "change-fenestration-frame-conductance"),
        ("dividerConductanceWK", "0.2", "change-fenestration-divider-conductance"),
        ("overhangDepthM", "1.0", "change-fenestration-overhang-depth"),
        ("overhangOffsetM", "0.5", "change-fenestration-overhang-offset"),
        ("finDepthM", "0.8", "change-fenestration-fin-depth"),
        ("finOffsetM", "0.1", "change-fenestration-fin-offset"),
        ("glazingConstruction", "", "clear-fenestration-glazing-construction"),
        ("glazingConstruction", "30", "bind-fenestration-glazing-construction"),
    ];
    for (property, value, kind) in cases {
        let kinds = emitted_kinds(&snapshot, &fenestration_property(50, property, value));
        assert_eq!(kinds, vec![(*kind).to_string()], "property {property} must emit exactly {kind}");
    }
}

/// ⛔️ The refusal triple: an unknown property, an unparsable/out-of-range value, a missing entity.
#[semio_framework_async_macros::async_test]
async fn a_fenestration_property_verb_refuses_the_three_bad_payloads() {
    let snapshot = snapshot_of(&inspector_model());
    assert_eq!(refusal(&snapshot, &fenestration_property(50, "nonsense", "1.0")), "mutation.invalid-payload");
    assert_eq!(refusal(&snapshot, &fenestration_property(50, "uValueWM2K", "warm")), "mutation.invalid-payload");
    assert_eq!(refusal(&snapshot, &fenestration_property(50, "shgc", "1.5")), "mutation.invalid-payload", "a fraction outside 0..=1 is refused");
    assert_eq!(refusal(&snapshot, &fenestration_property(9_999, "uValueWM2K", "1.0")), "mutation.target-missing");
    assert_eq!(refusal(&snapshot, &fenestration_property(50, "glazingConstruction", "9999")), "mutation.target-missing", "a dangling glazing construction is refused");
}

/// 🔁️ Setting a field to the value it already holds opens no revision.
#[semio_framework_async_macros::async_test]
async fn an_unchanged_property_value_emits_nothing() {
    let model = inspector_model();
    let snapshot = snapshot_of(&model);
    let current = model.fenestrations.iter().find(|window| window.id == EntityId(50)).expect("window 50").u_value_w_m2k;
    assert!(emitted_kinds(&snapshot, &fenestration_property(50, "uValueWM2K", &current.to_string())).is_empty());
    let zone_volume = model.zones[0].volume_m3;
    assert!(emitted_kinds(&snapshot, &zone_property(1, "volumeM3", &zone_volume.to_string())).is_empty());
    assert!(emitted_kinds(&snapshot, &surface_property(40, "name", &model.surfaces[0].name.clone())).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn every_surface_property_round_trips_through_the_granular_vocabulary() {
    let snapshot = snapshot_of(&inspector_model());
    let cases: &[(&str, &str, &str)] = &[
        ("name", "South Facade", "rename-surface"),
        ("class", "roof", "change-surface-class"),
        ("boundary", "adiabatic", "change-surface-boundary-condition"),
        ("construction", "32", "change-surface-construction"),
        ("sunExposed", "false", "change-surface-sun-exposed"),
        ("windExposed", "false", "change-surface-wind-exposed"),
        ("multiplier", "3", "change-surface-multiplier"),
    ];
    for (property, value, kind) in cases {
        assert_eq!(emitted_kinds(&snapshot, &surface_property(40, property, value)), vec![(*kind).to_string()], "property {property} must emit exactly {kind}");
    }
    let adiabatic = applied(&snapshot, &surface_property(40, "boundary", "adiabatic"));
    assert_eq!(adiabatic.surfaces[0].outside_boundary_condition, OutsideBoundary::Adiabatic);
}

/// 🚧️ An `Interzone` boundary needs its partner surface, and the partner has to exist — the union's
/// two payload halves are validated together before anything touches the document.
#[semio_framework_async_macros::async_test]
async fn an_interzone_boundary_carries_and_validates_its_partner_surface() {
    let snapshot = snapshot_of(&inspector_model());
    let paired = EnergyModelEditorCommand::SetSurfaceProperty { surface: 40, property: "boundary".into(), value: "interzone".into(), partner_surface: 42 };
    assert_eq!(emitted_kinds(&snapshot, &paired), vec!["change-surface-boundary-condition".to_string()]);
    assert_eq!(applied(&snapshot, &paired).surfaces[0].outside_boundary_condition, OutsideBoundary::Interzone(EntityId(42)));
    assert_eq!(refusal(&snapshot, &surface_property(40, "boundary", "interzone")), "mutation.invalid-payload", "an interzone boundary without a partner is refused");
    let dangling = EnergyModelEditorCommand::SetSurfaceProperty { surface: 40, property: "boundary".into(), value: "interzone".into(), partner_surface: 9_999 };
    assert_eq!(refusal(&snapshot, &dangling), "mutation.target-missing");
}

#[semio_framework_async_macros::async_test]
async fn a_surface_property_verb_refuses_the_three_bad_payloads() {
    let snapshot = snapshot_of(&inspector_model());
    assert_eq!(refusal(&snapshot, &surface_property(40, "nonsense", "x")), "mutation.invalid-payload");
    assert_eq!(refusal(&snapshot, &surface_property(40, "class", "wall")), "mutation.invalid-payload");
    assert_eq!(refusal(&snapshot, &surface_property(40, "multiplier", "0")), "mutation.invalid-payload");
    assert_eq!(refusal(&snapshot, &surface_property(9_999, "name", "Ghost")), "mutation.target-missing");
    assert_eq!(refusal(&snapshot, &surface_property(40, "construction", "9999")), "mutation.target-missing");
}

#[semio_framework_async_macros::async_test]
async fn every_zone_property_round_trips_through_the_granular_vocabulary() {
    let snapshot = snapshot_of(&inspector_model());
    let cases: &[(&str, &str, &str)] = &[
        ("name", "Living", "rename-zone"),
        ("volumeM3", "200", "change-zone-volume"),
        ("multiplier", "2", "change-zone-multiplier"),
        ("conditioned", "false", "change-zone-conditioned"),
        ("partOfTotalFloorArea", "false", "change-zone-floor-area-participation"),
    ];
    for (property, value, kind) in cases {
        assert_eq!(emitted_kinds(&snapshot, &zone_property(1, property, value)), vec![(*kind).to_string()], "property {property} must emit exactly {kind}");
    }
}

#[semio_framework_async_macros::async_test]
async fn a_zone_property_verb_refuses_the_three_bad_payloads() {
    let snapshot = snapshot_of(&inspector_model());
    assert_eq!(refusal(&snapshot, &zone_property(1, "nonsense", "x")), "mutation.invalid-payload");
    assert_eq!(refusal(&snapshot, &zone_property(1, "volumeM3", "-4")), "mutation.invalid-payload");
    assert_eq!(refusal(&snapshot, &zone_property(9_999, "name", "Ghost")), "mutation.target-missing");
}

/// 🌉️ The inspector's controls author only `{field, id}` and the host merges `value` — the bridge
/// has to read that spelling back, not just the palette's `{property, <entity>}` one.
#[semio_framework_async_macros::async_test]
async fn the_action_bridge_accepts_the_inspector_field_id_value_payload() {
    let text = |value: &str| dsl::DslValue::String(value.to_string());
    let args = dsl::DslValue::Object(vec![("field".to_string(), text("uValueWM2K")), ("id".to_string(), text("50")), ("value".to_string(), text("1.4"))]);
    let command = <EnergyModelEditor as ArtifactEditor>::command_from_action(SET_FENESTRATION_PROPERTY_ACTION_ID, Some(&args)).expect("the inspector payload resolves");
    assert_eq!(command, fenestration_property(50, "uValueWM2K", "1.4"));
}

/// 🧱️ The material and thermostat diffs gained the two fields they were missing — a material rename
/// and a thermostat re-homing now emit their own kinds instead of being masked by the probe.
#[semio_framework_async_macros::async_test]
async fn the_material_and_thermostat_diffs_cover_their_reference_fields() {
    let base = inspector_model();
    let mut edited = base.clone();
    edited.materials[0].name = "Cedar Siding".into();
    use protocol::SemanticMutation as _;
    let emit = model_edit("rename-material", &base, &edited, "rename".into()).expect("the rename diffs");
    assert_eq!(emit.artifact_mutations.iter().map(|mutation| mutation.semantics().kind).collect::<Vec<_>>(), vec!["rename-material"]);
    let mut rehomed = base.clone();
    rehomed.zones.push(Zone { id: EntityId(2), name: "Attic".into(), volume_m3: 40.0, multiplier: 1, conditioned: false, part_of_total_floor_area: true });
    rehomed.thermostats[0].zone_id = EntityId(2);
    let emit = model_edit("change-thermostat-zone", &base, &rehomed, "rehome".into()).expect("the re-homing diffs");
    assert!(emit.artifact_mutations.iter().any(|mutation| mutation.semantics().kind == "change-thermostat-zone"), "a thermostat's zone is no longer masked");
}

fn glazing_property(material: u32, property: &str, value: &str) -> EnergyModelEditorCommand {
    EnergyModelEditorCommand::SetGlazingMaterialProperty { material, property: property.into(), value: value.into() }
}

fn gas_property(material: u32, property: &str, value: &str) -> EnergyModelEditorCommand {
    EnergyModelEditorCommand::SetGasMaterialProperty { material, property: property.into(), value: value.into() }
}

/// 🧊️ Lane A's glazing/gas mutation kinds landed, so the two catalogues are addressable end to end.
#[semio_framework_async_macros::async_test]
async fn every_glazing_and_gas_material_property_round_trips_through_the_granular_vocabulary() {
    let snapshot = snapshot_of(&inspector_model());
    let glazing: &[(&str, &str, &str)] = &[
        ("name", "Low-E Glass", "rename-glazing-material"),
        ("thicknessM", "0.006", "change-glazing-material-thickness"),
        ("conductivityWMK", "1.1", "change-glazing-material-conductivity"),
        ("solarTransmittance", "0.6", "change-glazing-material-solar-transmittance"),
        ("visibleTransmittance", "0.7", "change-glazing-material-visible-transmittance"),
        ("infraredEmissivityFront", "0.1", "change-glazing-material-infrared-emissivity"),
        ("infraredEmissivityBack", "0.2", "change-glazing-material-infrared-emissivity"),
    ];
    for (property, value, kind) in glazing {
        assert_eq!(emitted_kinds(&snapshot, &glazing_property(22, property, value)), vec![(*kind).to_string()], "glazing property {property} must emit exactly {kind}");
    }
    let gases: &[(&str, &str, &str)] = &[("name", "Argon Gap".into(), "rename-gas-material"), ("thicknessM", "0.016", "change-gas-material-thickness"), ("gas", "argon", "change-gas-material-gas")];
    for (property, value, kind) in gases {
        assert_eq!(emitted_kinds(&snapshot, &gas_property(23, property, value)), vec![(*kind).to_string()], "gas property {property} must emit exactly {kind}");
    }
    assert_eq!(applied(&snapshot, &gas_property(23, "gas", "krypton")).gas_materials[0].gas, crate::model::GasKind::Krypton);
}

#[semio_framework_async_macros::async_test]
async fn the_glazing_and_gas_property_verbs_refuse_the_three_bad_payloads() {
    let snapshot = snapshot_of(&inspector_model());
    assert_eq!(refusal(&snapshot, &glazing_property(22, "solarReflectanceFront", "0.1")), "mutation.invalid-payload", "a field with no mutation kind is refused, never silently written");
    assert_eq!(refusal(&snapshot, &glazing_property(22, "thicknessM", "clear")), "mutation.invalid-payload");
    assert_eq!(refusal(&snapshot, &glazing_property(9_999, "thicknessM", "0.006")), "mutation.target-missing");
    assert_eq!(refusal(&snapshot, &gas_property(23, "gas", "helium")), "mutation.invalid-payload");
    assert_eq!(refusal(&snapshot, &gas_property(9_999, "thicknessM", "0.016")), "mutation.target-missing");
    let model = inspector_model();
    assert!(emitted_kinds(&snapshot, &glazing_property(22, "thicknessM", &model.glazing_materials[0].thickness_m.to_string())).is_empty(), "an unchanged value opens no revision");
}
//#endregion 🔍️InspectorVerbs
