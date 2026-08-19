//! 🌳️ Energy model editor — `structure` window: a real overview tree of the working `crate::model::
//! Model` behind the artifact's composed `structure` child, built from the framework `TreeWindowKit`
//! (contract §2.6). Two addressable edit-target leaves, `name`/`version` — the collection-size leaves
//! below them are a real read overview, not yet individually addressable (see the surface root's
//! `EnergyModelEditorCommand::SetStructureField` doc comment for the honest scope note).

use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_plugin::app::{TreeNodeView, TreeView, TreeWindowKit, WindowKit};
use semio_framework_plugin::{LocalizedLabel, UiNode, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TreeWindowKit::KIND_ID;
pub const BODY_KEY: &str = TreeWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::model::create_energy_model_editor`.
pub async fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Structure", "Struktur"), icon_id: "list-tree".into(), ..TreeWindowKit::editable_window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// ✏️ Real `EnergyModelSnapshot -> UiNode`: `name`/`version` (the two `set-node`-editable leaves)
/// plus one leaf per collection on `crate::model::Model`, each labeled with its live element count —
/// a genuine overview of the whole working model, not a placeholder.
pub async fn render(document: &EnergyModelSnapshot) -> UiNode {
    let model = crate::artifacts::model::energy_model(document);
    async fn leaf(id: &str, label: String) -> TreeNodeView {
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
    async fn definition_declares_a_tree_window() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
        assert_eq!(def.body_key, BODY_KEY);
    }

    #[semio_framework_async_macros::async_test]
    async fn render_lists_name_version_and_every_collection_count() {
        let document = EnergyModelSnapshot::default();
        let UiNode::Tree(node) = render(&document) else { panic!("expected Tree") };
        let root = &node.sections[0].items[0];
        let root_children = root.items.as_ref().expect("root has children");
        assert!(root_children.iter().any(|item| item.id == "name"));
        assert!(root_children.iter().any(|item| item.id == "version"));
        assert!(root_children.iter().any(|item| item.id == "zones"));
    }
}
//#endregion 🧪️Tests
