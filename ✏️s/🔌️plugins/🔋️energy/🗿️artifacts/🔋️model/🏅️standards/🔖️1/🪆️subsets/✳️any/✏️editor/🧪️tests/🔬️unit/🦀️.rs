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
    let migrated = def.window_kinds.iter().flat_map(|window| semio_framework::window_kind_actions(&def, window)).filter(|action| action.semantics.execution.interactive_job == InteractiveJobClassification::Migrated).map(|action| action.id.as_str()).collect::<BTreeSet<_>>();
    assert!(roster.is_subset(&migrated), "unclassified retained tools: {:?}", roster.difference(&migrated).collect::<Vec<_>>());
}

#[semio_framework_async_macros::async_test]
async fn every_declared_action_is_classified_and_resolves_to_a_command() {
    let def = definition();
    for action in def.window_kinds.iter().flat_map(|window| semio_framework::window_kind_actions(&def, window)) {
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
            simulation::SET_SETTINGS_ACTION_ID | simulation::SET_RESULT_FIELD_ACTION_ID => ArtifactToolPublicationLane::Config,
            // 🎥️ The 3d window's camera writes ONE window instance's own retained state.
            model_window::SET_CAMERA_ACTION_ID => ArtifactToolPublicationLane::WindowConfig,
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
    let picker = definition.window_kinds.iter().flat_map(|window| semio_framework::window_kind_actions(&definition, window)).find(|action| action.id == SET_ACTIVE_EXAMPLE_ACTION_ID).expect("the shell dispatches setActiveExample, so the app must declare it on a window");
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

    let insulated = applied(&snapshot, &EnergyModelEditorCommand::SetMaterialProperty { material: 1, property: "conductivityWMK".into(), value: "0.04".into() });
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
        EnergyModelEditorCommand::SetMaterialProperty { material: 1, property: "conductivityWMK".into(), value: "-1.0".into() },
        EnergyModelEditorCommand::SetSite { latitude_deg: 120.0, longitude_deg: 0.0, elevation_m: 0.0, time_zone_hours: 0.0, north_axis_deg: 0.0 },
        EnergyModelEditorCommand::SetRunPeriod { start_month: 13, start_day: 1, end_month: 12, end_day: 31 },
        EnergyModelEditorCommand::CreateZone { name: "Void".into(), volume_m3: 0.0, multiplier: 1, conditioned: true },
    ] {
        let fault = reduce(&command, &doc).err().expect("an out-of-range payload is refused");
        assert_eq!(fault.code.0.as_str(), "mutation.invalid-payload", "{} failed for the wrong reason", command.action_id());
    }
    let fault = reduce(&EnergyModelEditorCommand::SetMaterialProperty { material: 9, property: "conductivityWMK".into(), value: "0.04".into() }, &doc).err().expect("an unknown material is refused");
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
        let action = definition.window_kinds.iter().flat_map(|window| semio_framework::window_kind_actions(&definition, window)).find(|action| action.id == *tool_id).unwrap_or_else(|| panic!("action {tool_id} is declared on no window kind"));
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
        assert!(def.window_kinds.iter().flat_map(|window| semio_framework::window_kind_actions(&def, window)).all(|action| action.id != removed), "{removed} is still declared");
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

//#region 🪟️ActionOwnership
/// 🪟️ THE law the first browser probe broke: a panel action is dispatched in the ACTIVE window's
/// context, and `VcsArtifactApp` refuses one the active window kind does not own
/// (`window kind energy.model.3d does not own action set-fenestration-property`). So EVERY verb the
/// inspection panel can dispatch must be declared app-level — an action listed on one window kind is
/// "explicitly owned" and `build_definition` then stops copying it onto the others.
#[semio_framework_async_macros::async_test]
async fn every_inspector_verb_is_owned_by_every_window_kind() {
    let def = definition();
    assert!(def.window_kinds.len() >= 3, "the editor declares its windows");
    for action in inspector_action_definitions() {
        for window in &def.window_kinds {
            assert!(semio_framework::window_kind_actions(&def, window).iter().any(|declared| declared.id == action.id), "window kind {} does not own inspector action {}", window.id, action.id);
        }
    }
}

/// 🪟️ The other half of the same law: no window kind may declare an inspector verb itself, or it
/// becomes that window's alone again the moment someone re-adds it to a window's `actions()`.
#[semio_framework_async_macros::async_test]
async fn no_window_kind_declares_an_inspector_verb_itself() {
    let inspector: BTreeSet<String> = inspector_action_definitions().into_iter().map(|action| action.id).collect();
    for (window, own) in [
        (structure::WINDOW_KIND_ID, structure::actions()),
        (zones::WINDOW_KIND_ID, zones::actions()),
    ] {
        for action in own {
            assert!(!inspector.contains(&action.id), "window kind {window} re-declares inspector verb {}, which would un-own it everywhere else", action.id);
        }
    }
}

/// 🧵️ Every inspector verb is still a classified retained tool with a publication contract — moving
/// its declaration app-level must not drop it out of the dispatch catalogue.
#[semio_framework_async_macros::async_test]
async fn every_inspector_verb_stays_a_classified_retained_tool() {
    let def = definition();
    for action in inspector_action_definitions() {
        assert!(ENERGY_MODEL_RETAINED_TOOL_IDS.contains(&action.id.as_str()), "{} left the retained roster", action.id);
        let declared = def.window_kinds.iter().flat_map(|window| semio_framework::window_kind_actions(&def, window)).find(|declared| declared.id == action.id).expect("declared on a window");
        assert_eq!(declared.semantics.execution.interactive_job, InteractiveJobClassification::Migrated, "{} is not Migrated", action.id);
        assert!(!declared.args.is_empty(), "{} lost its argument declarations", action.id);
    }
}
//#endregion 🪟️ActionOwnership


//#region 🧱️MaterialAndConstructionVerbs
fn material_property(material: u32, property: &str, value: &str) -> EnergyModelEditorCommand {
    EnergyModelEditorCommand::SetMaterialProperty { material, property: property.into(), value: value.into() }
}

fn construction_property(construction: u32, property: &str, value: &str) -> EnergyModelEditorCommand {
    EnergyModelEditorCommand::SetConstructionProperty { construction, property: property.into(), value: value.into() }
}

/// 🧱️ `set-material-property` carries TEXT now, so the material's whole record — including the name
/// and the roughness class `change-material-roughness` just gave a mutation kind — round-trips
/// through the granular vocabulary instead of being read-only in the inspector.
#[semio_framework_async_macros::async_test]
async fn every_material_property_round_trips_through_the_granular_vocabulary() {
    let snapshot = snapshot_of(&inspector_model());
    let cases: &[(&str, &str, &str)] = &[
        ("name", "Heavyweight Concrete", "rename-material"),
        ("roughness", "verySmooth", "change-material-roughness"),
        ("thicknessM", "0.15", "change-material-thickness"),
        ("conductivityWMK", "1.2", "change-material-conductivity"),
        ("densityKgM3", "2100", "change-material-density"),
        ("specificHeatJKgK", "950", "change-material-specific-heat"),
        ("thermalAbsorptance", "0.85", "change-material-thermal-absorptance"),
        ("solarAbsorptance", "0.55", "change-material-solar-absorptance"),
        ("visibleAbsorptance", "0.5", "change-material-visible-absorptance"),
    ];
    for (property, value, kind) in cases {
        assert_eq!(emitted_kinds(&snapshot, &material_property(10, property, value)), vec![(*kind).to_string()], "material property {property} must emit exactly {kind}");
    }
    assert_eq!(applied(&snapshot, &material_property(10, "roughness", "verySmooth")).materials[0].roughness, crate::model::SurfaceRoughness::VerySmooth);
    assert_eq!(applied(&snapshot, &material_property(10, "name", "Heavyweight Concrete")).materials[0].name, "Heavyweight Concrete");
}

/// 🧱️ The four refusal shapes of the widened material verb.
#[semio_framework_async_macros::async_test]
async fn the_material_property_verb_refuses_the_four_bad_payloads() {
    let snapshot = snapshot_of(&inspector_model());
    assert_eq!(refusal(&snapshot, &material_property(10, "roughness", "gritty")), "mutation.invalid-payload", "an unknown roughness spelling is refused, never written");
    assert_eq!(refusal(&snapshot, &material_property(10, "conductivityWMK", "-1")), "mutation.invalid-payload");
    assert_eq!(refusal(&snapshot, &material_property(10, "reflectance", "0.5")), "mutation.invalid-payload", "a property this record does not have is refused");
    assert_eq!(refusal(&snapshot, &material_property(9_999, "name", "Ghost")), "mutation.target-missing");
    let held = inspector_model().materials[0].roughness;
    assert!(emitted_kinds(&snapshot, &material_property(10, "roughness", crate::editor::model::surface_roughness_id(held))).is_empty(), "an unchanged roughness opens no revision");
}

/// 🧱️ `set-construction-property`'s five shapes, each landing on the list kind the vocabulary really
/// declares. `replaceLayer` has no kind of its own, so it travels as the remove/insert pair.
#[semio_framework_async_macros::async_test]
async fn every_construction_property_round_trips_through_the_granular_vocabulary() {
    let snapshot = snapshot_of(&inspector_model());
    let model = inspector_model();
    let construction = model.constructions.iter().find(|entry| entry.layer_material_ids.len() >= 2).expect("the demo has a multi-layer construction");
    let id = construction.id.0;
    let spare = model.materials.iter().find(|material| !construction.layer_material_ids.contains(&material.id)).expect("the demo has a material this construction does not use");

    assert_eq!(emitted_kinds(&snapshot, &construction_property(id, "name", "Insulated Wall")), vec!["rename-construction".to_string()]);
    assert_eq!(emitted_kinds(&snapshot, &construction_property(id, "addLayer", &spare.id.0.to_string())), vec!["add-construction-layer".to_string()]);
    assert_eq!(emitted_kinds(&snapshot, &construction_property(id, "removeLayer", "0")), vec!["remove-construction-layer".to_string()]);
    assert_eq!(emitted_kinds(&snapshot, &construction_property(id, "moveLayerDown", "0")), vec!["reorder-construction-layers".to_string()]);
    assert_eq!(
        emitted_kinds(&snapshot, &construction_property(id, "replaceLayer:0", &spare.id.0.to_string())),
        vec!["remove-construction-layer".to_string(), "add-construction-layer".to_string()],
        "a one-slot exchange is the remove/insert pair at that index"
    );

    let renamed = applied(&snapshot, &construction_property(id, "name", "Insulated Wall"));
    assert_eq!(renamed.constructions.iter().find(|entry| entry.id.0 == id).expect("the construction survives").name, "Insulated Wall");
    let appended = applied(&snapshot, &construction_property(id, "addLayer", &spare.id.0.to_string()));
    assert_eq!(appended.constructions.iter().find(|entry| entry.id.0 == id).expect("the construction survives").layer_material_ids.last(), Some(&spare.id));
    let swapped = applied(&snapshot, &construction_property(id, "moveLayerDown", "0"));
    assert_eq!(swapped.constructions.iter().find(|entry| entry.id.0 == id).expect("the construction survives").layer_material_ids[1], construction.layer_material_ids[0]);
}

/// 🧱️ The construction verb's refusals — including the vocabulary limit that `add-construction-layer`
/// admits an OPAQUE material only, refused here rather than at the store.
#[semio_framework_async_macros::async_test]
async fn the_construction_property_verb_refuses_the_four_bad_payloads() {
    let snapshot = snapshot_of(&inspector_model());
    let model = inspector_model();
    let construction = model.constructions.iter().find(|entry| entry.layer_material_ids.len() >= 2).expect("a multi-layer construction");
    let id = construction.id.0;
    let glazing = model.glazing_materials.first().expect("the demo has a glazing material");
    assert_eq!(refusal(&snapshot, &construction_property(id, "addLayer", &glazing.id.0.to_string())), "mutation.invalid-payload", "a glazing pane has no add-construction-layer");
    assert_eq!(refusal(&snapshot, &construction_property(id, "removeLayer", "99")), "mutation.invalid-payload", "an index past the end is refused");
    assert_eq!(refusal(&snapshot, &construction_property(id, "moveLayerUp", "0")), "mutation.invalid-payload", "the first layer has nothing above it");
    assert_eq!(refusal(&snapshot, &construction_property(id, "thickness", "0.2")), "mutation.invalid-payload", "a property a construction does not have is refused");
    assert_eq!(refusal(&snapshot, &construction_property(9_999, "name", "Ghost")), "mutation.target-missing");
}

/// 🔬️ The masking gap this lane closed: `probe.materials` used to CLONE, so a field with no mutation
/// kind vanished silently. Roughness was the live case — now it emits its own kind, and every one of
/// the four newly projected collections reports back exactly what it diffed.
#[semio_framework_async_macros::async_test]
async fn the_four_projected_collections_emit_a_step_for_every_field_they_carry() {
    let base = inspector_model();
    let snapshot = snapshot_of(&base);

    let mut roughened = base.clone();
    roughened.materials[0].roughness = crate::model::SurfaceRoughness::VerySmooth;
    assert_eq!(model_edit_kinds(&base, &roughened), vec!["change-material-roughness".to_string()], "a roughness edit is no longer masked by a cloning probe");

    let mut renamed = base.clone();
    renamed.zones[0].name = "Attic".into();
    assert_eq!(model_edit_kinds(&base, &renamed), vec!["rename-zone".to_string()]);

    let mut resurfaced = base.clone();
    resurfaced.surfaces[0].multiplier = 3;
    assert_eq!(model_edit_kinds(&base, &resurfaced), vec!["change-surface-multiplier".to_string()]);

    let mut rehomed = base.clone();
    rehomed.thermostats[0].heating_throttle_range_k = 3.5;
    assert_eq!(model_edit_kinds(&base, &rehomed), vec!["change-thermostat-heating-throttle-range".to_string()]);

    let mut relayered = base.clone();
    let construction = relayered.constructions.iter_mut().find(|entry| entry.layer_material_ids.len() >= 2).expect("a multi-layer construction");
    construction.layer_material_ids.swap(0, 1);
    assert_eq!(model_edit_kinds(&base, &relayered), vec!["reorder-construction-layers".to_string()], "constructions are projected too — the probe never compared them before");
}

/// 🔬️ Runs the mutation seam directly over a (base, edited) pair and names the steps it emitted.
fn model_edit_kinds(base: &crate::model::Model, edited: &crate::model::Model) -> Vec<String> {
    use protocol::SemanticMutation as _;
    super::model_edit("probe", base, edited, "probe".into()).expect("the seam names every edited field").artifact_mutations.iter().map(|mutation| mutation.semantics().kind.to_string()).collect()
}
//#endregion 🧱️MaterialAndConstructionVerbs

//#region 🌉️HostMergeRoundTrip
/// 🌉️ Exactly what the react host dispatches for a `Trigger::Change` control: the descriptor's own
/// authored args with the control's value merged in under the literal key `value`
/// (`🗣️Interpreter/🟦️.tsx` `dispatchDeclarativeControlAction`).
fn host_merged(authored: &[(&str, &str)], value: &str) -> dsl::DslValue {
    let mut entries: Vec<(String, dsl::DslValue)> = authored.iter().map(|(key, value)| ((*key).to_string(), dsl::DslValue::String((*value).to_string()))).collect();
    entries.retain(|(key, _)| key != "value");
    entries.push(("value".to_string(), dsl::DslValue::String(value.to_string())));
    dsl::DslValue::Object(entries)
}

/// 📍️ THE blocker the review found: the site form authors all five scalars plus `field`, the host
/// merges the typed number under `value`, and the bridge has to route it into the NAMED slot. Before
/// the `{field, value}` indirection the bridge read `northAxisDeg` by name, found the key missing and
/// silently wrote 0.0 — so this law round-trips the real wire payload instead of asserting a shape.
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
    let command = <EnergyModelEditor as ArtifactEditor>::command_from_action(SET_SITE_ACTION_ID, Some(&args)).expect("the site payload resolves");
    assert_eq!(command, EnergyModelEditorCommand::SetSite { latitude_deg: 47.5, longitude_deg: 8.5, elevation_m: 400.0, time_zone_hours: 1.0, north_axis_deg: 30.0 });

    let edited = applied(&snapshot, &command);
    assert!((edited.site.north_axis_deg - 30.0).abs() < 1e-12, "the site kept the typed north axis, not 0.0: {:?}", edited.site);
    assert!((edited.site.latitude_deg - 47.5).abs() < 1e-12, "the untouched scalars survive the edit: {:?}", edited.site);
    assert_eq!(emitted_kinds(&snapshot, &command), vec!["update-site".to_string()]);
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
    ];
    let borrowed: Vec<(&str, &str)> = authored.iter().map(|(key, value)| (*key, value.as_str())).collect();
    let command = <EnergyModelEditor as ArtifactEditor>::command_from_action(SET_THERMOSTAT_SETPOINTS_ACTION_ID, Some(&host_merged(&borrowed, "4.5"))).expect("the thermostat payload resolves");
    assert_eq!(
        command,
        EnergyModelEditorCommand::SetThermostatSetpoints {
            thermostat: thermostat.id.0,
            heating_schedule: thermostat.heating_setpoint_schedule_id.0,
            cooling_schedule: thermostat.cooling_setpoint_schedule_id.0,
            heating_throttle_range_k: 4.5,
            cooling_throttle_range_k: thermostat.cooling_throttle_range_k,
        }
    );
    let edited = applied(&snapshot, &command);
    assert!((edited.thermostats[0].heating_throttle_range_k - 4.5).abs() < 1e-12, "the thermostat kept the typed range, not 2.0");
    assert_eq!(edited.thermostats[0].cooling_setpoint_schedule_id, thermostat.cooling_setpoint_schedule_id, "the untouched schedule survives");

    // 🎛️ A schedule select goes the same way — that one used to write id 0 and fail loudly.
    let other = model.schedules.constants.iter().map(|schedule| schedule.id).find(|id| *id != thermostat.heating_setpoint_schedule_id).expect("a second constant schedule");
    let mut swapped = borrowed.clone();
    swapped[2] = ("field", "heatingSchedule");
    let command = <EnergyModelEditor as ArtifactEditor>::command_from_action(SET_THERMOSTAT_SETPOINTS_ACTION_ID, Some(&host_merged(&swapped, &other.0.to_string()))).expect("the schedule payload resolves");
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
    let command = <EnergyModelEditor as ArtifactEditor>::command_from_action(SET_SURFACE_PROPERTY_ACTION_ID, Some(&args)).expect("the partner payload resolves");
    assert_eq!(command, EnergyModelEditorCommand::SetSurfaceProperty { surface: first.0, property: "interzonePartner".into(), value: second.0.to_string(), partner_surface: 0 });
    assert_eq!(emitted_kinds(&snapshot, &command), vec!["change-surface-boundary-condition".to_string()]);
    assert_eq!(applied(&snapshot, &command).surfaces[0].outside_boundary_condition, OutsideBoundary::Interzone(second));

    assert_eq!(refusal(&snapshot, &surface_property(first.0, "interzonePartner", &first.0.to_string())), "mutation.invalid-payload", "a surface is never its own neighbour");
    assert_eq!(refusal(&snapshot, &surface_property(first.0, "interzonePartner", "9999")), "mutation.target-missing");
    assert_eq!(refusal(&snapshot, &surface_property(first.0, "interzonePartner", "0")), "mutation.invalid-payload");

    // 🚧️ And a plain boundary pick on an ALREADY interzone surface keeps its neighbour instead of
    // refusing for want of a partner it was never given a control for.
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
    // 📚️ `setActiveExample` is app-level through `.mutation(…)` rather than the roster above, and the
    // shell dispatches it from whatever window has focus, so it is held to the same law.
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
    // ⌨️ Only this EDITOR's own verbs: the builder also binds framework-reserved chords (history,
    // tool run) whose actions live outside every window's action list by construction.
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
    crate::editor::model::results::legend_bands().iter().enumerate().filter(|(_, hex)| crate::editor::model::results::hex_to_rgb01(hex).iter().all(|channel| channel_present(body, *channel))).map(|(index, _)| index).collect()
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
    use crate::editor::model::results::ResultField;

    // 🪟️ The refresh half. `ToolRunLedger::dirty_scope()` unions `entry.window_bodies`, which the driver
    // builds as `definition.windows.iter().filter_map(|id| registry.window_body_key(id))` — an id no
    // window kind declares is silently FILTERED AWAY and the body then never redraws on a tick.
    let def = definition();
    let declared = crate::energy_simulation_session::energy_simulation_run_definition().windows;
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

    // 🦶️ Pump to `finalized` the way the shell does: drain the UI outbox every turn, so the ledger can
    // push the next scope, and watch the 3d body's colours while the run is still live.
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(600);
    let mut painted_while_running = std::collections::BTreeSet::new();
    let mut live_caption = false;
    while std::time::Instant::now() < deadline && state_of(&app) != Some("finalized") {
        // 🏃️ Sample ONLY while the run is still computing. A sample taken after the job settled would be
        // satisfied by the completing tick's payload and would say nothing about whether the window
        // recolours DURING the run — which is the half the tick-interval republish exists for.
        let running = matches!(state_of(&app), Some("starting" | "running"));
        app.advance_typed_operation_publication().await.expect("driver turn");
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

    // (c) At least one tick's dirty scope named the 3d body — otherwise the window only ever redraws
    // by accident, on somebody else's full refresh.
    assert!(scopes.iter().any(|scope| scope_covers(scope, model_window::BODY_KEY)), "no dirty scope during the run covered {}: {scopes:?}", model_window::BODY_KEY);

    let after = render_text(&mut app, model_window::BODY_KEY).await;
    // (a) The legend caption survives finalization.
    assert!(after.contains("kWh"), "the FINALIZED run left no legend caption in the 3d body — `ToolRunView::payload` was cleared on finalize, or the body never re-rendered. Body: {}", &after[..after.len().min(1500)]);
    assert!(ResultField::ALL.iter().any(|field| after.contains(field.label())), "the caption names no published result field");
    assert_ne!(after, before, "the finalized body is byte-identical to the pre-run body: nothing was recoloured");
    assert!(live_caption, "the caption never appeared WHILE the run was live, only after it finalized");

    // (b) Distinct surfaces take distinct bands, during the run and after it. One bucket for every
    // vertex is what the browser saw and what the rank ramp exists to prevent.
    let bands = painted_bands(&after);
    assert!(bands.len() >= 2, "every surface landed on the same ramp band after finalize ({bands:?}) — the colour map is unreadable. Body: {}", &after[..after.len().min(1500)]);
    assert!(painted_while_running.len() >= 2, "every surface landed on the same ramp band WHILE the run was live ({painted_while_running:?}) — the mid-run payload is the all-zero pre-run map");

    close(app);
}
//#endregion 🎨️RecolourLaw
