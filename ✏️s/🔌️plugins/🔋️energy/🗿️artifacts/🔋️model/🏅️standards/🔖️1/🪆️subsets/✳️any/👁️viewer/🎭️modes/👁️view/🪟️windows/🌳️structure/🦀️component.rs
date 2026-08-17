//! 🌳️ Energy model viewer — `structure` window: a real, READ-ONLY overview tree of the working
//! `crate::model::Model` behind the artifact's composed `structure` child, built from the framework
//! `TreeWindowKit` (contract §2.6). Independent render from the sibling mutation-capable surface — the
//! same `crate::artifacts::model::energy_model` read, no edit affordances (`window_kind()`, the
//! read-only variant, not the editable one).

use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_plugin::app::{TreeNodeView, TreeView, TreeWindowKit, WindowKit};
use semio_framework_plugin::{LocalizedLabel, UiNode, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TreeWindowKit::KIND_ID;
pub const BODY_KEY: &str = TreeWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::model::create_energy_model_viewer`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Structure", "Struktur"), icon_id: "list-tree".into(), ..TreeWindowKit::window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Pure `EnergyModelSnapshot -> UiNode` read: `name`/`version` plus one leaf per collection on
/// `crate::model::Model`, each labeled with its live element count — a real overview, no mutation.
pub fn render(document: &EnergyModelSnapshot) -> UiNode {
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

    #[test]
    fn definition_declares_a_tree_window() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
        assert_eq!(def.body_key, BODY_KEY);
    }

    #[test]
    fn render_lists_name_version_and_every_collection_count() {
        let document = EnergyModelSnapshot::default();
        let UiNode::Tree(node) = render(&document) else { panic!("expected Tree") };
        let root = &node.sections[0].items[0];
        let root_children = root.items.as_ref().expect("root has children");
        assert!(root_children.iter().any(|item| item.id == "name"));
        assert!(root_children.iter().any(|item| item.id == "zones"));
    }
}
//#endregion 🧪️Tests
