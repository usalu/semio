/// 🧰️ Definition, roster, dispatch classification, document verb and granular vocabulary laws of the editor.

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
        let command = <EnergyModelEditor as ArtifactEditor>::command_from_action(tool_id, None, None).unwrap_or_else(|error| panic!("action {tool_id} has no command: {error:?}"));
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
        let loaded = <EnergyModelSnapshot as pack::snapshot::ArtifactPack>::decode_pack(pack).expect("the emitted pack decodes");
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
    crate::snapshot::energy_snapshot_with_state(ENERGY_MODEL_DOCUMENT_SCHEMA, model, None)
}

fn applied(snapshot: &EnergyModelSnapshot, command: &EnergyModelEditorCommand) -> crate::model::Model {
    use protocol::Mutation as _;
    let history = HistoryView::empty();
    let doc = ArtifactView::new(snapshot, &history);
    let emit = reduce(command, &doc).expect("document verb reduces");
    let mut next = snapshot.clone();
    for mutation in &emit.artifact_mutations {
        next = protocol::apply_diff_cold(mutation.diff(&next).diff(), &next).expect("diff applies");
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
    let sited = applied(&snapshot, &EnergyModelEditorCommand::SetSite { latitude_deg: Some(39.74), longitude_deg: Some(-105.18), elevation_m: Some(1609.0), time_zone_hours: Some(-7.0), north_axis_deg: Some(0.0) });
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
/// asked for. A `app.command.kind-unavailable` here is a real regression, not a pending group.
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
    assert_eq!(fault.code.0.as_str(), "app.command.target-in-use");
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
        EnergyModelEditorCommand::SetSite { latitude_deg: Some(120.0), longitude_deg: None, elevation_m: None, time_zone_hours: None, north_axis_deg: None },
        EnergyModelEditorCommand::SetRunPeriod { start_month: 13, start_day: 1, end_month: 12, end_day: 31 },
        EnergyModelEditorCommand::CreateZone { name: "Void".into(), volume_m3: 0.0, multiplier: 1, conditioned: true },
    ] {
        let fault = reduce(&command, &doc).err().expect("an out-of-range payload is refused");
        assert_eq!(fault.code.0.as_str(), "app.command.invalid-payload", "{} failed for the wrong reason", command.action_id());
    }
    let fault = reduce(&EnergyModelEditorCommand::SetMaterialProperty { material: 9, property: "conductivityWMK".into(), value: "0.04".into() }, &doc).err().expect("an unknown material is refused");
    assert_eq!(fault.code.0.as_str(), "mutation.target-missing");
}
