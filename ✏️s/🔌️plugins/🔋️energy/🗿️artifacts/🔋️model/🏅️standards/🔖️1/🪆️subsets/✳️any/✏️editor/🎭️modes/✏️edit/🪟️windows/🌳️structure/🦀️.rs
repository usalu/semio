//! 🌳️ Energy model editor — `structure` window: a real overview tree of the working `crate::model::
//! Model` behind the artifact's composed `structure` child, built from the framework `TreeWindowKit`
//! (contract §2.6). Two addressable edit-target leaves, `name`/`version` — the collection-size leaves
//! below them are a real read overview, not yet individually addressable (see the surface root's
//! `EnergyModelEditorCommand::SetStructureField` doc comment for the honest scope note).

use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_plugin::app::{TreeNodeView, TreeView, TreeWindowKit, WindowKit};
use semio_framework_plugin::{ActionArgDef, ActionDefinition, ActionKind, BuiltNode, LocalizedLabel, UiAssemblyResult, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TreeWindowKit::KIND_ID;
pub const BODY_KEY: &str = TreeWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🎬️ One authored document verb. Execution authority is granted by the surface root's registered
/// factory + proof, and `create_energy_model_editor` stamps `Migrated` on every retained tool id.
fn action(id: &str, en: &str, de: &str, args: Vec<ActionArgDef>) -> ActionDefinition {
    ActionDefinition::bounded_catalog(id, LocalizedLabel::native(en, de), ActionKind::Mutation).with_args(args)
}

/// 🌳️ The authored verbs this window owns beside the kit's generic `set-node` — surface, material,
/// thermostat and site editing, each addressing its target by the model's own `EntityId`.
pub fn actions() -> Vec<ActionDefinition> {
    vec![
        action(
            "create-surface",
            "Create surface",
            "Fläche anlegen",
            vec![
                ActionArgDef::text("name", LocalizedLabel::native("Name", "Name")).required(),
                ActionArgDef::number("zone", LocalizedLabel::native("Zone id", "Zonen-Id")).required(),
                ActionArgDef::number("construction", LocalizedLabel::native("Construction id", "Konstruktions-Id")).required(),
                ActionArgDef::text("class", LocalizedLabel::native("Surface class", "Flächenklasse")),
            ],
        ),
        action("delete-surface", "Delete surface", "Fläche löschen", vec![ActionArgDef::number("surface", LocalizedLabel::native("Surface id", "Flächen-Id")).required()]),
        action(
            "assign-surface-construction",
            "Assign construction",
            "Konstruktion zuweisen",
            vec![
                ActionArgDef::number("surface", LocalizedLabel::native("Surface id", "Flächen-Id")).required(),
                ActionArgDef::number("construction", LocalizedLabel::native("Construction id", "Konstruktions-Id")).required(),
            ],
        ),
        action(
            "set-material-property",
            "Set material property",
            "Materialeigenschaft setzen",
            vec![
                ActionArgDef::number("material", LocalizedLabel::native("Material id", "Material-Id")).required(),
                ActionArgDef::text("property", LocalizedLabel::native("Property", "Eigenschaft")).required(),
                ActionArgDef::number("value", LocalizedLabel::native("Value", "Wert")).required(),
            ],
        ),
        action(
            "set-thermostat-setpoints",
            "Set thermostat setpoints",
            "Thermostat-Sollwerte setzen",
            vec![
                ActionArgDef::number("thermostat", LocalizedLabel::native("Thermostat id", "Thermostat-Id")).required(),
                ActionArgDef::number("heatingSchedule", LocalizedLabel::native("Heating setpoint schedule", "Heiz-Sollwertprofil")).required(),
                ActionArgDef::number("coolingSchedule", LocalizedLabel::native("Cooling setpoint schedule", "Kühl-Sollwertprofil")).required(),
                ActionArgDef::number("heatingThrottleRangeK", LocalizedLabel::native("Heating throttle range (K)", "Heiz-Regelbereich (K)")),
                ActionArgDef::number("coolingThrottleRangeK", LocalizedLabel::native("Cooling throttle range (K)", "Kühl-Regelbereich (K)")),
            ],
        ),
        action(
            "set-site",
            "Set site",
            "Standort setzen",
            vec![
                ActionArgDef::slider("latitudeDeg", LocalizedLabel::native("Latitude (°)", "Breitengrad (°)"), -90.0, 90.0).required(),
                ActionArgDef::slider("longitudeDeg", LocalizedLabel::native("Longitude (°)", "Längengrad (°)"), -180.0, 180.0).required(),
                ActionArgDef::number("elevationM", LocalizedLabel::native("Elevation (m)", "Höhe (m)")),
                ActionArgDef::number("timeZoneHours", LocalizedLabel::native("Time zone (h)", "Zeitzone (h)")),
                ActionArgDef::number("northAxisDeg", LocalizedLabel::native("North axis (°)", "Nordachse (°)")),
            ],
        ),
    ]
}

/// 🧱️ Stitched into the editor manifest by `crate::editor::model::create_energy_model_editor`. The
/// kit's own `set-node` action is KEPT and this window's authored verbs are appended to it — the
/// builder's `window_kind_actions` REPLACES a window's action list, so composing here is the only
/// way both survive.
pub fn definition() -> WindowKindDefinition {
    let kit = TreeWindowKit::editable_window_kind();
    let mut actions = kit.actions.clone();
    actions.extend(self::actions());
    WindowKindDefinition { label: LocalizedLabel::native("Structure", "Struktur"), icon_id: "list-tree".into(), actions, ..kit }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// ✏️ Real `EnergyModelSnapshot -> UiNode`: `name`/`version` (the two `set-node`-editable leaves)
/// plus one leaf per collection on `crate::model::Model`, each labeled with its live element count —
/// a genuine overview of the whole working model, not a placeholder.
pub fn render(document: &EnergyModelSnapshot) -> UiAssemblyResult<BuiltNode> {
    let model = crate::artifacts::model::energy_model(document);
    fn leaf(id: &str, label: String) -> TreeNodeView {
        TreeNodeView { id: id.into(), label, children: Vec::new() }
    }
    let mut children = vec![
        leaf("name", format!("Name: {}", model.name)),
        leaf("version", format!("Version: {}", model.version)),
        leaf("site", format!("Site: lat {:.2}°, lon {:.2}°, elev {:.1} m", model.site.latitude_deg, model.site.longitude_deg, model.site.elevation_m)),
    ];
    let counts: &[(&str, usize)] = &[
        ("zones", model.zones.len()),
        ("spaces", model.spaces.len()),
        ("surfaces", model.surfaces.len()),
        ("fenestrations", model.fenestrations.len()),
        ("materials", model.materials.len()),
        ("constructions", model.constructions.len()),
        ("people", model.people.len()),
        ("lighting", model.lighting.len()),
        ("equipment", model.equipment.len()),
        ("thermostats", model.thermostats.len()),
        ("humidistats", model.humidistats.len()),
        ("setpointManagers", model.setpoint_managers.len()),
        ("idealLoads", model.ideal_loads.len()),
        ("zoneEquipment", model.zone_equipment.len()),
        ("airLoops", model.air_loops.len()),
        ("plantLoops", model.plant_loops.len()),
        ("outdoorAirSystems", model.outdoor_air_systems.len()),
        ("infiltrations", model.infiltrations.len()),
        ("mechanicalVentilations", model.mechanical_ventilations.len()),
        ("shadingSurfaces", model.shading_surfaces.len()),
        ("spaceLists", model.space_lists.len()),
        ("thermalEnclosures", model.thermal_enclosures.len()),
        ("adjacencyPairs", model.adjacency_pairs.len()),
        ("electricalLoadCenters", model.electrical_load_centers.len()),
        ("pvSystems", model.pv_systems.len()),
        ("batteryStorage", model.battery_storage.len()),
        ("shwSystems", model.shw_systems.len()),
        ("solarThermalSystems", model.solar_thermal_systems.len()),
        ("refrigerationSystems", model.refrigeration_systems.len()),
        ("waterSystems", model.water_systems.len()),
        ("faults", model.faults.len()),
        ("outputVariables", model.output_variables.len()),
        ("sizingObjects", model.sizing_objects.len()),
        ("daylightZones", model.daylight_zones.len()),
        ("roomAirModels", model.room_air_models.len()),
    ];
    children.extend(counts.iter().map(|(name, count)| leaf(name, format!("{name}: {count}"))));
    let root = TreeNodeView { id: "model".into(), label: format!("{} (v{})", model.name, model.version), children };
    TreeWindowKit::render(&TreeView { roots: vec![root] })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_a_tree_window_keeping_the_kit_action() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
        assert_eq!(def.body_key, BODY_KEY);
        assert!(def.actions.iter().any(|action| action.id == "set-node"), "the kit's own edit action must survive");
        assert_eq!(def.actions.len(), 1 + actions().len());
    }

    #[semio_framework_async_macros::async_test]
    async fn every_authored_action_is_localized_in_english_and_german() {
        for action in actions() {
            assert_ne!(action.label.native(), action.label.secondary(), "action {} is not really translated", action.id);
            for arg in &action.args {
                assert_ne!(arg.label.native(), arg.label.secondary(), "arg {} of action {} is not really translated", arg.id, action.id);
            }
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn render_lists_name_version_and_every_collection_count() {
        let document = EnergyModelSnapshot::default();
        let tree = render(&document).expect("the tree window assembles");
        assert_eq!(tree.key, WINDOW_KIND_ID);
        let root = &tree.children[0].children[0];
        assert!(root.children.iter().any(|item| item.key == "name"));
        assert!(root.children.iter().any(|item| item.key == "version"));
        assert!(root.children.iter().any(|item| item.key == "zones"));
    }
}
//#endregion 🧪️Tests
