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
    let command = EnergyModelEditorCommand::SetThermostatSetpoints { thermostat: thermostat.id.0, zone: thermostat.zone_id.0, heating_schedule: thermostat.heating_setpoint_schedule_id.0, cooling_schedule: thermostat.cooling_setpoint_schedule_id.0, heating_throttle_range_k: 3.5, cooling_throttle_range_k: thermostat.cooling_throttle_range_k };
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

/// 📐️ Coordinate editing emits the existing geometry mutation and preserves all other coordinates.
#[semio_framework_async_macros::async_test]
async fn inspector_geometry_edits_preserve_other_coordinates_and_refuse_bad_operands() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📐️geometry-edit/🔣️.json")).unwrap();
    let mut model = inspector_model();
    model.fenestrations.iter_mut().find(|row| row.id.0 == 50).unwrap().vertices_m = vec![[1.0, 0.0, 1.0], [3.0, 0.0, 1.0], [3.0, 0.0, 3.0], [1.0, 0.0, 3.0]];
    let snapshot = snapshot_of(&model);
    for (kind, command) in [("surface", surface_property as fn(u32, &str, &str) -> EnergyModelEditorCommand), ("fenestration", fenestration_property)] {
        let case = &law[kind];
        let id = case["id"].as_u64().unwrap() as u32;
        let property = case["property"].as_str().unwrap();
        let value = case["value"].as_str().unwrap();
        let edit = command(id, property, value);
        assert_eq!(emitted_kinds(&snapshot, &edit), vec![case["kind"].as_str().unwrap()]);
        let result = applied(&snapshot, &edit);
        let (before, after, axis) = if kind == "surface" {
            (&model.surfaces.iter().find(|row| row.id.0 == id).unwrap().vertices_m, &result.surfaces.iter().find(|row| row.id.0 == id).unwrap().vertices_m, 0)
        } else {
            (&model.fenestrations.iter().find(|row| row.id.0 == id).unwrap().vertices_m, &result.fenestrations.iter().find(|row| row.id.0 == id).unwrap().vertices_m, 2)
        };
        let mut expected = before.clone();
        expected[0][axis] = value.parse().unwrap();
        assert_eq!(*after, expected);
        assert!(emitted_kinds(&snapshot_of(&result), &edit).is_empty());
        for invalid in law["invalid"].as_array().unwrap() {
            assert_eq!(refusal(&snapshot, &command(id, invalid.as_str().unwrap(), value)), "app.command.invalid-payload");
        }
        for invalid in law["nonfinite"].as_array().unwrap() {
            assert_eq!(refusal(&snapshot, &command(id, property, invalid.as_str().unwrap())), "app.command.invalid-payload");
        }
    }
}

/// 🗺️ A complete polygon input also initializes windows whose geometry was only described by area.
#[semio_framework_async_macros::async_test]
async fn an_inspector_polygon_can_initialize_geometry_and_refuses_incomplete_polygons() {
    let snapshot = snapshot_of(&inspector_model());
    let edit = fenestration_property(50, "vertices", "1,0,1; 3,0,1; 3,0,3; 1,0,3");
    assert_eq!(emitted_kinds(&snapshot, &edit), vec!["replace-fenestration-vertices"]);
    let result = applied(&snapshot, &edit);
    assert_eq!(result.fenestrations.iter().find(|row| row.id.0 == 50).unwrap().vertices_m, vec![[1.0,0.0,1.0], [3.0,0.0,1.0], [3.0,0.0,3.0], [1.0,0.0,3.0]]);
    let cleared = applied(&snapshot_of(&result), &fenestration_property(50, "vertices", ""));
    assert!(cleared.fenestrations.iter().find(|row| row.id.0 == 50).unwrap().vertices_m.is_empty());
    for value in ["1,0,1; 3,0,1", "1,0; 3,0,1; 3,0,3", "NaN,0,1; 3,0,1; 3,0,3"] {
        assert_eq!(refusal(&snapshot, &fenestration_property(50, "vertices", value)), "app.command.invalid-payload");
    }
}

/// ⛱️ Shading details publish one semantic edit and admit only schedules that exist.
#[semio_framework_async_macros::async_test]
async fn shading_inspector_properties_publish_native_mutations() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📐️geometry-edit/🔣️.json")).unwrap();
    assert_eq!(law["shading"]["nameKind"], "rename-shading-surface");
    let mut model = inspector_model();
    model.shading_surfaces.push(crate::model::ShadingSurface { id: EntityId(500), name: "Canopy".into(), vertices_m: vec![[0.0,0.0,3.0], [1.0,0.0,3.0], [1.0,1.0,3.0]], transmittance_schedule_id: None });
    let snapshot = snapshot_of(&model);
    let command = |property: &str, value: &str| EnergyModelEditorCommand::SetShadingProperty { shading: 500, property: property.into(), value: value.into() };
    for (property, value, kind) in [("name", "Roof canopy", "rename-shading-surface"), ("vertex:0:z", "3.5", "replace-shading-surface-vertices")] {
        assert_eq!(emitted_kinds(&snapshot, &command(property, value)), vec![kind]);
    }
    let changed = applied(&snapshot, &command("name", "Roof canopy"));
    assert_eq!(changed.shading_surfaces.last().unwrap().name, "Roof canopy");
    assert_eq!(refusal(&snapshot, &command("transmittanceSchedule", "999999")), "mutation.target-missing");
    assert!(emitted_kinds(&snapshot, &command("transmittanceSchedule", "")).is_empty());
}

/// 🛋️ Space controls reach native mutations and refuse dangling zone assignments.
#[semio_framework_async_macros::async_test]
async fn space_inspector_edits_publish_native_mutations() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📐️geometry-edit/🔣️.json")).unwrap();
    assert_eq!(law["space"]["areaKind"], "change-space-floor-area");
    let mut model = inspector_model();
    model.spaces.push(crate::model::Space { id: EntityId(501), name: "Office".into(), zone_id: EntityId(1), floor_area_m2: 20.0 });
    let snapshot = snapshot_of(&model);
    let command = |property: &str, value: &str| EnergyModelEditorCommand::SetSpaceProperty { space: 501, property: property.into(), value: value.into() };
    assert_eq!(emitted_kinds(&snapshot, &command("name", "Main office")), vec!["rename-space"]);
    assert_eq!(emitted_kinds(&snapshot, &command("floorAreaM2", "25")), vec!["change-space-floor-area"]);
    assert_eq!(applied(&snapshot, &command("floorAreaM2", "25")).spaces.last().unwrap().floor_area_m2, 25.0);
    assert!(emitted_kinds(&snapshot, &command("zone", "1")).is_empty());
    assert_eq!(refusal(&snapshot, &command("zone", "999999")), "mutation.target-missing");
    assert_eq!(refusal(&snapshot, &command("floorAreaM2", "-1")), "app.command.invalid-payload");
}

/// 🔗️ Reference controls publish a native reassignment and refuse references to missing targets.
#[semio_framework_async_macros::async_test]
async fn geometry_host_references_are_editable_and_admitted() {
    let mut model = inspector_model();
    let mut zone = model.zones[0].clone();
    zone.id = EntityId(502);
    zone.name = "Second zone".into();
    model.zones.push(zone);
    let snapshot = snapshot_of(&model);
    assert_eq!(emitted_kinds(&snapshot, &surface_property(40, "zone", "502")), vec!["change-surface-zone"]);
    assert_eq!(emitted_kinds(&snapshot, &fenestration_property(50, "surface", "41")), vec!["change-fenestration-surface"]);
    assert_eq!(refusal(&snapshot, &surface_property(40, "zone", "999999")), "mutation.target-missing");
    assert_eq!(refusal(&snapshot, &fenestration_property(50, "surface", "999999")), "mutation.target-missing");
}

/// 👥️ Load fixtures span all four families and publish only the addressed native parameter.
#[semio_framework_async_macros::async_test]
async fn load_inspector_law_spans_people_lighting_equipment_and_infiltration() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/💡️load-edit/🔣️.json")).unwrap();
    let mut model = inspector_model();
    let schedule = model.schedules.constants[0].id;
    model.people.push(crate::model::PeopleGain { id: EntityId(503), zone_id: EntityId(1), schedule_id: schedule, activity_schedule_id: schedule, people_per_area: 0.1, sensible_fraction: 0.6, latent_fraction: 0.4, radiant_fraction: 0.3 });
    model.lighting.push(crate::model::LightingGain { id: EntityId(504), zone_id: EntityId(1), schedule_id: schedule, watts_per_area: 5.0, radiant_fraction: 0.3, visible_fraction: 0.2, return_air_fraction: 0.1 });
    let snapshot = snapshot_of(&model);
    for case in law["cases"].as_array().unwrap() {
        let command = EnergyModelEditorCommand::SetLoadProperty { load: case["id"].as_u64().unwrap() as u32, property: case["field"].as_str().unwrap().into(), value: case["value"].as_str().unwrap().into() };
        assert_eq!(emitted_kinds(&snapshot, &command), vec![case["kind"].as_str().unwrap()]);
        let result = applied(&snapshot, &command);
        assert!(emitted_kinds(&snapshot_of(&result), &command).is_empty());
    }
    for (property, value, expected) in [("zone", "999999", "mutation.target-missing"), ("schedule", "999999", "mutation.target-missing"), ("wattsPerArea", "NaN", "app.command.invalid-payload"), ("radiantFraction", "1.1", "app.command.invalid-payload")] {
        assert_eq!(refusal(&snapshot, &EnergyModelEditorCommand::SetLoadProperty { load: 60, property: property.into(), value: value.into() }), expected);
    }
}

/// 🧬️ The catalog route preserves native mutation identity, sparse delta and inverse behavior.
#[semio_framework_async_macros::async_test]
async fn native_catalog_action_publishes_a_granular_mutation_and_its_original_inverse() {
    use protocol::{ColdMutation, Mutation};
    let snapshot = snapshot_of(&inspector_model());
    let command = EnergyModelEditorCommand::ApplyNativeMutation { kind: "change-zone-volume".into(), payload: r#"{"id":1,"newVolumeM3":200}"#.into() };
    assert_eq!(emitted_kinds(&snapshot, &command), vec!["change-zone-volume"]);
    let history = HistoryView::empty();
    let emit = reduce(&command, &ArtifactView::new(&snapshot, &history)).unwrap();
    let mutation = &emit.artifact_mutations[0];
    let outcome = mutation.diff(&snapshot);
    let changed = protocol::apply_diff_cold(outcome.diff(), &snapshot).unwrap();
    assert_eq!(changed.model.zones[0].volume_m3, 200.0);
    let inverse = <EnergyModelMutation as ColdMutation<EnergyModelSnapshot>>::inverse(mutation, &snapshot).unwrap();
    let mut undone = changed;
    for mutation in inverse.iter().rev() {
        let outcome = mutation.diff(&undone);
        undone = protocol::apply_diff_cold(outcome.diff(), &undone).unwrap();
    }
    assert_eq!(undone.model, snapshot.model);
    for (kind, payload) in [("unknown-native-kind", "{}"), ("change-zone-volume", "{}"), ("change-zone-volume", "not JSON")] {
        assert_eq!(refusal(&snapshot, &EnergyModelEditorCommand::ApplyNativeMutation { kind: kind.into(), payload: payload.into() }), "app.command.invalid-payload");
    }
}

/// 🗂️ Every native mutation has one schema-derived typed payload form in the retained action.
#[semio_framework_async_macros::async_test]
async fn native_catalog_forms_cover_the_complete_mutation_vocabulary() {
    use protocol::Mutation;
    let actions = crate::editor_commands::native_mutation_actions();
    let descriptors = <EnergyModelMutation as Mutation<EnergyModelSnapshot>>::DESCRIPTORS;
    assert_eq!(actions.len(), descriptors.len());
    for descriptor in descriptors {
        let action = actions.iter().find(|action| action.id == descriptor.aggregate_variant).unwrap();
        assert_eq!(action.id, descriptor.aggregate_variant);
        let command = crate::editor_commands::args_bridge::command_from_action(&action.id, None).unwrap();
        assert_eq!(command.action_id(), action.id);
    }
}

/// ❄️ Ideal loads retain native unlimited capacity and editable supply/outdoor-air controls.
#[semio_framework_async_macros::async_test]
async fn ideal_loads_properties_emit_native_intents() {
    let model = inspector_model();
    let id = model.ideal_loads[0].id.0;
    let snapshot = snapshot_of(&model);
    let command = |property: &str, value: &str| EnergyModelEditorCommand::SetHvacProperty { system: id, property: property.into(), value: value.into() };
    assert_eq!(emitted_kinds(&snapshot, &command("maxHeatingCapacityW", "12000")), vec!["change-ideal-loads-system-max-heating-capacity"]);
    let result = applied(&snapshot, &command("maxHeatingCapacityW", "12000"));
    assert_eq!(result.ideal_loads[0].max_heating_capacity_w, Some(12000.0));
    let cleared = applied(&snapshot_of(&result), &command("maxHeatingCapacityW", ""));
    assert_eq!(cleared.ideal_loads[0].max_heating_capacity_w, None);
    assert_eq!(refusal(&snapshot, &command("minCoolingSupplyAirTempC", "-300")), "app.command.invalid-payload");
}

/// 🧮️ Retained admission includes nested schedule values and text outside the original six row tables.
#[semio_framework_async_macros::async_test]
async fn native_action_extent_accounts_for_all_nested_model_owners() {
    let mut model = inspector_model();
    let command = EnergyModelEditorCommand::ApplyNativeMutation { kind: "change-zone-volume".into(), payload: r#"{"id":1,"newVolumeM3":200}"#.into() };
    assert!(energy_model_extent(&command, &snapshot_of(&model), &protocol::InteractionState::default()).is_some());
    model.schedules.time_series.push(crate::schedule::TimeSeriesSchedule { id: crate::schedule::ScheduleId(999), values: vec![1.0; ENERGY_MODEL_RETAINED_WORK_ITEMS + 1], timestep_seconds: 3600 });
    assert!(energy_model_extent(&command, &snapshot_of(&model), &protocol::InteractionState::default()).is_none());
    model.schedules.time_series.clear();
    model.name = "x".repeat(ENERGY_MODEL_RETAINED_WORK_ITEMS + 1);
    assert!(energy_model_extent(&command, &snapshot_of(&model), &protocol::InteractionState::default()).is_none());
}

/// 🌡️ Zone reassignment uses the same native diff path as the setpoint controls.
#[semio_framework_async_macros::async_test]
async fn thermostat_zone_control_changes_only_its_native_reference() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🌡️thermostat-zone/🔣️.json")).unwrap();
    let mut model = inspector_model();
    let mut zone = model.zones[0].clone();
    zone.id = crate::model::EntityId(law["replacement"].as_u64().unwrap() as u32);
    model.zones.push(zone);
    let thermostat = &model.thermostats[0];
    let command = EnergyModelEditorCommand::SetThermostatSetpoints { thermostat: thermostat.id.0, zone: model.zones.last().unwrap().id.0, heating_schedule: thermostat.heating_setpoint_schedule_id.0, cooling_schedule: thermostat.cooling_setpoint_schedule_id.0, heating_throttle_range_k: thermostat.heating_throttle_range_k, cooling_throttle_range_k: thermostat.cooling_throttle_range_k };
    let snapshot = snapshot_of(&model);
    assert_eq!(emitted_kinds(&snapshot, &command), vec![law["mutation"].as_str().unwrap().to_string()]);
    let mut expected = model.clone();
    expected.thermostats[0].zone_id = model.zones.last().unwrap().id;
    assert_eq!(applied(&snapshot, &command), expected);
    let EnergyModelEditorCommand::SetThermostatSetpoints { thermostat, heating_schedule, cooling_schedule, heating_throttle_range_k, cooling_throttle_range_k, .. } = command else { unreachable!() };
    let missing = EnergyModelEditorCommand::SetThermostatSetpoints { thermostat, zone: law["missing"].as_u64().unwrap() as u32, heating_schedule, cooling_schedule, heating_throttle_range_k, cooling_throttle_range_k };
    assert_eq!(refusal(&snapshot, &missing), "mutation.target-missing");
}
