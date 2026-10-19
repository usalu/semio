/// 🔍️ Inspector verb, action ownership and material and construction verb laws.

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
    assert_eq!(refusal(&snapshot, &fenestration_property(50, "nonsense", "1.0")), "app.command.invalid-payload");
    assert_eq!(refusal(&snapshot, &fenestration_property(50, "uValueWM2K", "warm")), "app.command.invalid-payload");
    assert_eq!(refusal(&snapshot, &fenestration_property(50, "shgc", "1.5")), "app.command.invalid-payload", "a fraction outside 0..=1 is refused");
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
    assert_eq!(refusal(&snapshot, &surface_property(40, "boundary", "interzone")), "app.command.invalid-payload", "an interzone boundary without a partner is refused");
    let dangling = EnergyModelEditorCommand::SetSurfaceProperty { surface: 40, property: "boundary".into(), value: "interzone".into(), partner_surface: 9_999 };
    assert_eq!(refusal(&snapshot, &dangling), "mutation.target-missing");
}

#[semio_framework_async_macros::async_test]
async fn a_surface_property_verb_refuses_the_three_bad_payloads() {
    let snapshot = snapshot_of(&inspector_model());
    assert_eq!(refusal(&snapshot, &surface_property(40, "nonsense", "x")), "app.command.invalid-payload");
    assert_eq!(refusal(&snapshot, &surface_property(40, "class", "wall")), "app.command.invalid-payload");
    assert_eq!(refusal(&snapshot, &surface_property(40, "multiplier", "0")), "app.command.invalid-payload");
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
    assert_eq!(refusal(&snapshot, &zone_property(1, "nonsense", "x")), "app.command.invalid-payload");
    assert_eq!(refusal(&snapshot, &zone_property(1, "volumeM3", "-4")), "app.command.invalid-payload");
    assert_eq!(refusal(&snapshot, &zone_property(9_999, "name", "Ghost")), "mutation.target-missing");
}

/// 🌉️ The inspector's controls author only `{field, id}` and the host merges `value` — the bridge
/// has to read that spelling back, not just the palette's `{property, <entity>}` one.
#[semio_framework_async_macros::async_test]
async fn the_action_bridge_accepts_the_inspector_field_id_value_payload() {
    let text = |value: &str| semio_framework_value::DslValue::String(value.to_string());
    let args = semio_framework_value::DslValue::Object(vec![("field".to_string(), text("uValueWM2K")), ("id".to_string(), text("50")), ("value".to_string(), text("1.4"))]);
    let command = <EnergyModelEditor as ArtifactEditor>::command_from_action(SET_FENESTRATION_PROPERTY_ACTION_ID, Some((args).clone()), None).expect("the inspector payload resolves");
    assert_eq!(command, fenestration_property(50, "uValueWM2K", "1.4"));
}

/// 🧱️ A material rename and a thermostat's setpoint edit emit their own kinds, and restating a stored value emits nothing.
#[semio_framework_async_macros::async_test]
async fn the_material_and_thermostat_verbs_emit_only_their_own_kinds() {
    let snapshot = snapshot_of(&inspector_model());
    assert_eq!(emitted_kinds(&snapshot, &material_property(10, "name", "Cedar Siding")), vec!["rename-material".to_string()]);
    let stored = snapshot.model.materials[0].name.clone();
    assert!(emitted_kinds(&snapshot, &material_property(10, "name", &stored)).is_empty(), "restating the stored name is not an edit");
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
        ("solarReflectanceFront", "0.18", "change-glazing-material-solar-reflectance-front"),
        ("solarReflectanceBack", "0.18", "change-glazing-material-solar-reflectance-back"),
        ("visibleReflectanceFront", "0.18", "change-glazing-material-visible-reflectance-front"),
        ("visibleReflectanceBack", "0.18", "change-glazing-material-visible-reflectance-back"),
        ("infraredTransmittance", "0.18", "change-glazing-material-infrared-transmittance"),
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
    assert_eq!(refusal(&snapshot, &glazing_property(22, "solarReflectanceFront", "1.1")), "app.command.invalid-payload");
    assert_eq!(refusal(&snapshot, &glazing_property(22, "unknownOpticalField", "0.1")), "app.command.invalid-payload");
    assert_eq!(refusal(&snapshot, &glazing_property(22, "thicknessM", "clear")), "app.command.invalid-payload");
    assert_eq!(refusal(&snapshot, &glazing_property(9_999, "thicknessM", "0.006")), "mutation.target-missing");
    assert_eq!(refusal(&snapshot, &gas_property(23, "gas", "helium")), "app.command.invalid-payload");
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
    assert_eq!(refusal(&snapshot, &material_property(10, "roughness", "gritty")), "app.command.invalid-payload", "an unknown roughness spelling is refused, never written");
    assert_eq!(refusal(&snapshot, &material_property(10, "conductivityWMK", "-1")), "app.command.invalid-payload");
    assert_eq!(refusal(&snapshot, &material_property(10, "reflectance", "0.5")), "app.command.invalid-payload", "a property this record does not have is refused");
    assert_eq!(refusal(&snapshot, &material_property(9_999, "name", "Ghost")), "mutation.target-missing");
    let held = inspector_model().materials[0].roughness;
    assert!(emitted_kinds(&snapshot, &material_property(10, "roughness", crate::editor_commands::surface_roughness_id(held))).is_empty(), "an unchanged roughness opens no revision");
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
    assert_eq!(refusal(&snapshot, &construction_property(id, "addLayer", &glazing.id.0.to_string())), "app.command.invalid-payload", "a glazing pane has no add-construction-layer");
    assert_eq!(refusal(&snapshot, &construction_property(id, "removeLayer", "99")), "app.command.invalid-payload", "an index past the end is refused");
    assert_eq!(refusal(&snapshot, &construction_property(id, "moveLayerUp", "0")), "app.command.invalid-payload", "the first layer has nothing above it");
    assert_eq!(refusal(&snapshot, &construction_property(id, "thickness", "0.2")), "app.command.invalid-payload", "a property a construction does not have is refused");
    assert_eq!(refusal(&snapshot, &construction_property(9_999, "name", "Ghost")), "mutation.target-missing");
}

/// 🔬️ Each projected field emits its granular mutation through the real editor command and central diff applier.
#[semio_framework_async_macros::async_test]
async fn the_four_projected_collections_emit_a_step_for_every_field_they_carry() {
    let base = inspector_model();
    let snapshot = snapshot_of(&base);
    let mut roughened = base.clone();
    roughened.materials[0].roughness = crate::model::SurfaceRoughness::VerySmooth;
    let command = material_property(base.materials[0].id.0, "roughness", "verySmooth");
    assert_eq!(emitted_kinds(&snapshot, &command), vec!["change-material-roughness".to_string()]);
    assert_eq!(applied(&snapshot, &command), roughened);
    let mut renamed = base.clone();
    renamed.zones[0].name = "Attic".into();
    let command = zone_property(base.zones[0].id.0, "name", "Attic");
    assert_eq!(emitted_kinds(&snapshot, &command), vec!["rename-zone".to_string()]);
    assert_eq!(applied(&snapshot, &command), renamed);
    let mut resurfaced = base.clone();
    resurfaced.surfaces[0].multiplier = 3;
    let command = surface_property(base.surfaces[0].id.0, "multiplier", "3");
    assert_eq!(emitted_kinds(&snapshot, &command), vec!["change-surface-multiplier".to_string()]);
    assert_eq!(applied(&snapshot, &command), resurfaced);
    let mut rehomed = base.clone();
    rehomed.thermostats[0].heating_throttle_range_k = 3.5;
    let thermostat = &base.thermostats[0];
    let command = EnergyModelEditorCommand::SetThermostatSetpoints { thermostat: thermostat.id.0, heating_schedule: thermostat.heating_setpoint_schedule_id.0, cooling_schedule: thermostat.cooling_setpoint_schedule_id.0, heating_throttle_range_k: 3.5, cooling_throttle_range_k: thermostat.cooling_throttle_range_k };
    assert_eq!(emitted_kinds(&snapshot, &command), vec!["change-thermostat-heating-throttle-range".to_string()]);
    assert_eq!(applied(&snapshot, &command), rehomed);
    let mut relayered = base.clone();
    let construction = relayered.constructions.iter_mut().find(|entry| entry.layer_material_ids.len() >= 2).expect("a multi-layer construction");
    construction.layer_material_ids.swap(0, 1);
    let command = construction_property(construction.id.0, "moveLayerDown", "0");
    assert_eq!(emitted_kinds(&snapshot, &command), vec!["reorder-construction-layers".to_string()]);
    assert_eq!(applied(&snapshot, &command), relayered);
}
//#endregion 🧱️MaterialAndConstructionVerbs
