//! 🌳️ The shared energy `structure` tree both the editor and the viewer render, split out of the snapshot crate so each stays inside the source boundary.

pub use semio_plugin_window_kits_rich::tree_kit::{TreeNodeView, TreeView};

//#region 🗂️StructureOverview
/// 🗂️ The framework tree kit's sibling ceiling: `TreeWindowKit::render` collects each node's
/// children into a `UiFixedList<BuiltNode>` (`UI_FIXED_LIST_ITEMS` = 32) and the 33rd sibling
/// faults the whole assembly with `tree-window.siblings`, so the window renders EMPTY at runtime.
/// `Model` carries 35 collections — [`energy_structure_overview`] folds them into groups that stay
/// under this ceiling, and both the editor's and the viewer's `structure` windows read that one
/// grouping (the viewer may never import from the editor surface).
pub const STRUCTURE_TREE_SIBLING_CEILING: usize = 32;

/// 🗂️ One overview group of the structure tree: node id, English and German label, and the collections it
/// folds as `(node id, live element count)`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StructureOverviewGroup {
    pub id: &'static str,
    pub label: (&'static str, &'static str),
    pub counts: Vec<(&'static str, usize)>,
}

/// 🗂️ Every collection on [`Model`] exactly once, folded into seven domain groups, each with its
/// live element counts — the read overview both `structure` windows draw beneath `name`/`version`.
pub fn energy_structure_overview(model: &semio_s_artifact_energy_model_snapshot::state::EnergyModelView<'_>) -> Vec<StructureOverviewGroup> {
    vec![
        StructureOverviewGroup {
            id: "geometry",
            label: ("Geometry", "Geometrie"),
            counts: vec![
                ("zones", model.zones.len()),
                ("spaces", model.spaces.len()),
                ("surfaces", model.surfaces.len()),
                ("fenestrations", model.fenestrations.len()),
                ("shadingSurfaces", model.shading_surfaces.len()),
                ("spaceLists", model.space_lists.len()),
                ("thermalEnclosures", model.thermal_enclosures.len()),
                ("adjacencyPairs", model.adjacency_pairs.len()),
            ],
        },
        StructureOverviewGroup { id: "envelope", label: ("Envelope", "Gebäudehülle"), counts: vec![("materials", model.materials.len()), ("constructions", model.constructions.len())] },
        StructureOverviewGroup {
            id: "loads",
            label: ("Internal loads & air exchange", "Interne Lasten & Luftwechsel"),
            counts: vec![
                ("people", model.people.len()),
                ("lighting", model.lighting.len()),
                ("equipment", model.equipment.len()),
                ("infiltrations", model.infiltrations.len()),
                ("mechanicalVentilations", model.mechanical_ventilations.len()),
            ],
        },
        StructureOverviewGroup { id: "controls", label: ("Controls", "Regelung"), counts: vec![("thermostats", model.thermostats.len()), ("humidistats", model.humidistats.len()), ("setpointManagers", model.setpoint_managers.len())] },
        StructureOverviewGroup {
            id: "hvac",
            label: ("HVAC", "Haustechnik"),
            counts: vec![
                ("idealLoads", model.ideal_loads.len()),
                ("zoneEquipment", model.zone_equipment.len()),
                ("airLoops", model.air_loops.len()),
                ("plantLoops", model.plant_loops.len()),
                ("outdoorAirSystems", model.outdoor_air_systems.len()),
            ],
        },
        StructureOverviewGroup {
            id: "systems",
            label: ("Electrical, water & refrigeration", "Elektro, Wasser & Kälte"),
            counts: vec![
                ("electricalLoadCenters", model.electrical_load_centers.len()),
                ("pvSystems", model.pv_systems.len()),
                ("batteryStorage", model.battery_storage.len()),
                ("shwSystems", model.shw_systems.len()),
                ("solarThermalSystems", model.solar_thermal_systems.len()),
                ("refrigerationSystems", model.refrigeration_systems.len()),
                ("waterSystems", model.water_systems.len()),
            ],
        },
        StructureOverviewGroup {
            id: "analysis",
            label: ("Analysis & output", "Analyse & Ausgabe"),
            counts: vec![("faults", model.faults.len()), ("outputVariables", model.output_variables.len()), ("sizingObjects", model.sizing_objects.len()), ("daylightZones", model.daylight_zones.len()), ("roomAirModels", model.room_air_models.len())],
        },
    ]
}

/// 🗣️ Picks one of the two authored languages.
fn say(locale: semio_ui_locale::Locale, en: &'static str, de: &'static str) -> &'static str {
    if locale == semio_ui_locale::Locale::De { de } else { en }
}

/// 🗣️ The German name of a collection node id; the English name is the id itself.
fn collection_name(id: &'static str, locale: semio_ui_locale::Locale) -> &'static str {
    let de = match id {
        "zones" => "Zonen",
        "spaces" => "Räume",
        "surfaces" => "Flächen",
        "fenestrations" => "Öffnungen",
        "shadingSurfaces" => "Verschattungsflächen",
        "spaceLists" => "Raumlisten",
        "thermalEnclosures" => "Thermische Hüllen",
        "adjacencyPairs" => "Angrenzungspaare",
        "materials" => "Materialien",
        "constructions" => "Konstruktionen",
        "people" => "Personen",
        "lighting" => "Beleuchtung",
        "equipment" => "Geräte",
        "infiltrations" => "Infiltrationen",
        "mechanicalVentilations" => "Mechanische Lüftungen",
        "thermostats" => "Thermostate",
        "humidistats" => "Hygrostate",
        "setpointManagers" => "Sollwertregler",
        "idealLoads" => "Ideale Lasten",
        "zoneEquipment" => "Zonengeräte",
        "airLoops" => "Luftkreisläufe",
        "plantLoops" => "Anlagenkreisläufe",
        "outdoorAirSystems" => "Außenluftsysteme",
        "electricalLoadCenters" => "Elektrische Lastzentren",
        "pvSystems" => "PV-Anlagen",
        "batteryStorage" => "Batteriespeicher",
        "shwSystems" => "Warmwassersysteme",
        "solarThermalSystems" => "Solarthermieanlagen",
        "refrigerationSystems" => "Kälteanlagen",
        "waterSystems" => "Wassersysteme",
        "faults" => "Fehler",
        "outputVariables" => "Ausgabevariablen",
        "sizingObjects" => "Dimensionierungsobjekte",
        "daylightZones" => "Tageslichtzonen",
        "roomAirModels" => "Raumluftmodelle",
        _ => id,
    };
    say(locale, id, de)
}

/// 🌳️ The shared `structure` tree both surfaces render in the view's language: `name`/`version`
/// (the editor's two `set-node` targets), the site line, then one node per overview group labeled
/// with its total.
pub fn energy_structure_tree(model: &semio_s_artifact_energy_model_snapshot::state::EnergyModelView<'_>, locale: semio_ui_locale::Locale) -> TreeView {
    fn leaf(id: &str, label: String) -> TreeNodeView {
        TreeNodeView { id: id.into(), label, children: Vec::new() }
    }
    let mut children = vec![
        leaf("name", format!("{}: {}", say(locale, "Name", "Name"), model.name)),
        leaf("version", format!("{}: {}", say(locale, "Version", "Version"), model.version)),
        leaf("site", format!("{}: {} {:.2}°, {} {:.2}°, {} {:.1} m", say(locale, "Site", "Standort"), say(locale, "lat", "Breite"), model.site.latitude_deg, say(locale, "lon", "Länge"), model.site.longitude_deg, say(locale, "elev", "Höhe"), model.site.elevation_m)),
    ];
    children.extend(energy_structure_overview(model).into_iter().map(|group| {
        let total: usize = group.counts.iter().map(|(_, count)| count).sum();
        TreeNodeView { id: group.id.into(), label: format!("{}: {total}", say(locale, group.label.0, group.label.1)), children: group.counts.iter().map(|(name, count)| leaf(name, format!("{}: {count}", collection_name(name, locale)))).collect() }
    }));
    TreeView { roots: vec![TreeNodeView { id: "model".into(), label: format!("{} (v{})", model.name, model.version), children }] }
}
//#endregion 🗂️StructureOverview
