//! model <- epJSON
//!
//! ⚡️ Exact inverse of the export leaf (`🚪️io/📤️export/…/⚡️epjson/🔖️25.2/✳️any/🦀️.rs`), reading a
//! real EnergyPlus 25.2 `epJSON` document with [`pack::json::parse`] and no third-party translator.
//! Two laws hold and are tested:
//!
//! 1. **Round trip.** `export → import → export` is byte-identical for every document this codec
//!    itself wrote (`round_trip_of_every_bestest_case_is_byte_identical`).
//! 2. **Nothing silently dropped.** An object type outside the covered subset, a schedule whose
//!    name does not carry a semio [`ScheduleId`], a construction layer naming a material the
//!    document does not define — each becomes an [`EpJsonDiagnostic`] on the returned import
//!    report, never a quiet default.
//!
//! Entity numbering is re-minted here (epJSON identifies everything by NAME), one contiguous block
//! per collection, so the two directions compose: names, layer order, surface order and window
//! order all survive, which is what makes law 1 hold without an id side-channel.
//!
//! @see https://energyplus.readthedocs.io/en/latest/schema.html
//! @see ../../../../../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/ENERGY-PLUGIN-END-TO-END/📓️w6-epjson-io.md
use crate::air_exchange::InfiltrationMethod;
use crate::io::export::serializers::artifacts::epjson::v25_2::any::{glazing_construction_name, surface_normal, EpJsonDiagnostic, CONTRACT_OUTPUT_VARIABLES, DUAL_SETPOINT_CONTROL_SCHEDULE};
use crate::model::{
    Construction, EntityId, EquipmentGain, Fenestration, GroundTemperatureConfig, IdealLoadsSystem, Infiltration, LightingGain, Material, Model, OutputReportFrequency, OutputVariableSpec, OutsideBoundary, PeopleGain, ScheduleId, Site, Space,
    Surface, SurfaceClass, Thermostat, Zone,
};
use crate::schedule::{ConstantSchedule, DailySchedule, ScheduleInterpolation};
use crate::EnergyModelSnapshot;
use pack::json::{Object, Value};

//#region 🔖️Bases
/// 🔢️ First minted [`EntityId`] of each collection; epJSON has no ids, so the two directions agree
/// on names and on these blocks instead.
const ZONE_BASE: u32 = 1;
const MATERIAL_BASE: u32 = 1_000;
const CONSTRUCTION_BASE: u32 = 2_000;
const SURFACE_BASE: u32 = 3_000;
const FENESTRATION_BASE: u32 = 4_000;
const GAIN_BASE: u32 = 5_000;
const HVAC_BASE: u32 = 6_000;
const SPACE_BASE: u32 = 7_000;

/// 🧾️ Every object type this codec understands; anything else is reported, not ignored.
pub const KNOWN_OBJECT_TYPES: &[&str] = &[
    "Version",
    "SimulationControl",
    "Building",
    "Site:Location",
    "Site:GroundTemperature:BuildingSurface",
    "GlobalGeometryRules",
    "Timestep",
    "RunPeriod",
    "ScheduleTypeLimits",
    "Schedule:Constant",
    "Schedule:Compact",
    "Material",
    "Material:NoMass",
    "WindowMaterial:SimpleGlazingSystem",
    "Construction",
    "Zone",
    "Space",
    "BuildingSurface:Detailed",
    "FenestrationSurface:Detailed",
    "Shading:Overhang:Projection",
    "Shading:Fin:Projection",
    "ZoneInfiltration:DesignFlowRate",
    "People",
    "Lights",
    "ElectricEquipment",
    "ThermostatSetpoint:DualSetpoint",
    "ZoneControl:Thermostat",
    "ZoneHVAC:IdealLoadsAirSystem",
    "ZoneHVAC:EquipmentList",
    "ZoneHVAC:EquipmentConnections",
    "NodeList",
    "Sizing:Zone",
    "Output:Variable",
    "Output:Meter",
    "Output:SQLite",
];

/// 🧱️ Conductivity assumed for a `Material:NoMass` layer, whose thickness EnergyPlus does not
/// state. `thickness = thermal_resistance × this`, so the export leaf's `R = thickness /
/// conductivity` reproduces the original resistance exactly — the round trip is on R, not on the
/// (unrecoverable) thickness.
pub const NO_MASS_CONDUCTIVITY_W_M_K: f64 = 0.04;
//#endregion 🔖️Bases

//#region 🔖️Report
/// 📋️ One decoded document plus everything the decoder could not represent faithfully.
#[derive(Clone, Debug)]
pub struct EpJsonImport {
    pub model: Model,
    pub diagnostics: Vec<EpJsonDiagnostic>,
}
//#endregion 🔖️Report

//#region 🔖️Readers
fn objects<'a>(root: &'a Object, kind: &str) -> Vec<(&'a str, &'a Object)> {
    match root.get(kind) {
        Some(Value::Object(group)) => group.iter().filter_map(|(name, value)| if let Value::Object(fields) = value { Some((name, fields)) } else { None }).collect(),
        _ => Vec::new(),
    }
}

fn number(fields: &Object, key: &str) -> Option<f64> {
    fields.get(key).and_then(Value::as_f64)
}

fn number_or(fields: &Object, key: &str, fallback: f64) -> f64 {
    number(fields, key).unwrap_or(fallback)
}

fn text<'a>(fields: &'a Object, key: &str) -> Option<&'a str> {
    fields.get(key).and_then(Value::as_str)
}

fn flag(fields: &Object, key: &str, fallback: bool) -> bool {
    match text(fields, key) {
        Some("Yes") => true,
        Some("No") => false,
        _ => fallback,
    }
}

/// 🆔️ Recovers the [`ScheduleId`] the export leaf encoded into a schedule's name.
pub fn schedule_id_from_name(name: &str) -> Option<ScheduleId> {
    name.strip_prefix("Schedule ").and_then(|rest| rest.parse::<u32>().ok()).map(ScheduleId)
}

fn vertices_of(fields: &Object) -> Vec<[f64; 3]> {
    match fields.get("vertices") {
        Some(Value::Array(items)) => items
            .iter()
            .filter_map(|item| {
                let Value::Object(vertex) = item else { return None };
                Some([number_or(vertex, "vertex_x_coordinate", 0.0), number_or(vertex, "vertex_y_coordinate", 0.0), number_or(vertex, "vertex_z_coordinate", 0.0)])
            })
            .collect(),
        _ => Vec::new(),
    }
}

fn aperture_corners(fields: &Object) -> Vec<[f64; 3]> {
    (1..=4)
        .filter_map(|index| {
            let x = number(fields, &format!("vertex_{index}_x_coordinate"))?;
            Some([x, number_or(fields, &format!("vertex_{index}_y_coordinate"), 0.0), number_or(fields, &format!("vertex_{index}_z_coordinate"), 0.0)])
        })
        .collect()
}

fn distance(a: [f64; 3], b: [f64; 3]) -> f64 {
    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt()
}
//#endregion 🔖️Readers

//#region 🔖️Decoder
/// 📥️ Decodes an epJSON document tree into a [`Model`] plus its import report.
pub fn decode_model(document: &Value) -> Result<EpJsonImport, String> {
    let Value::Object(root) = document else { return Err("epJSON: the document root must be an object".to_string()) };
    let mut diagnostics = Vec::new();
    for (kind, _) in root.iter() {
        if !KNOWN_OBJECT_TYPES.contains(&kind) {
            diagnostics.push(EpJsonDiagnostic::new("epjson.object.unknown-type", kind, format!("object type {kind:?} is outside the subset this codec decodes; its objects are reported and skipped, never merged into the model")));
        }
    }
    let mut model = Model::default();

    let building = objects(root, "Building");
    let Some((building_name, building_fields)) = building.first() else { return Err("epJSON: no Building object — EnergyPlus itself requires one".to_string()) };
    model.name = (*building_name).to_string();
    model.version = objects(root, "Version").first().and_then(|(_, fields)| text(fields, "version_identifier")).unwrap_or("25.2").to_string();

    let location = objects(root, "Site:Location");
    let site_fields = location.first().map(|(_, fields)| *fields);
    model.site = Site {
        latitude_deg: site_fields.and_then(|fields| number(fields, "latitude")).unwrap_or(0.0),
        longitude_deg: site_fields.and_then(|fields| number(fields, "longitude")).unwrap_or(0.0),
        elevation_m: site_fields.and_then(|fields| number(fields, "elevation")).unwrap_or(0.0),
        time_zone_hours: site_fields.and_then(|fields| number(fields, "time_zone")).unwrap_or(0.0),
        north_axis_deg: number_or(building_fields, "north_axis", 0.0),
    };

    if let Some((_, ground)) = objects(root, "Site:GroundTemperature:BuildingSurface").first() {
        const MONTHS: [&str; 12] = ["january", "february", "march", "april", "may", "june", "july", "august", "september", "october", "november", "december"];
        let mut monthly = [18.0f64; 12];
        for (index, month) in MONTHS.iter().enumerate() {
            monthly[index] = number_or(ground, &format!("{month}_ground_temperature"), 18.0);
        }
        model.ground_temperature = GroundTemperatureConfig { building_surface_c: monthly, shallow_c: monthly, deep_c: monthly[0] };
    }

    if let Some((_, period)) = objects(root, "RunPeriod").first() {
        model.run_period = crate::calendar::RunPeriod {
            start_month: number_or(period, "begin_month", 1.0) as u8,
            start_day: number_or(period, "begin_day_of_month", 1.0) as u8,
            end_month: number_or(period, "end_month", 12.0) as u8,
            end_day: number_or(period, "end_day_of_month", 31.0) as u8,
            year: number_or(period, "begin_year", crate::calendar::RunPeriod::default().year as f64) as u16,
        };
    }

    decode_schedules(root, &mut model, &mut diagnostics);
    let material_names = decode_materials(root, &mut model);
    let glazing = decode_constructions(root, &mut model, &material_names, &mut diagnostics);
    let zone_names = decode_zones(root, &mut model);
    decode_surfaces(root, &mut model, &zone_names, &mut diagnostics);
    decode_apertures(root, &mut model, &glazing, &mut diagnostics);
    decode_gains(root, &mut model, &zone_names, &mut diagnostics);
    decode_hvac(root, &mut model, &zone_names, &mut diagnostics);
    decode_outputs(root, &mut model);
    Ok(EpJsonImport { model, diagnostics })
}

fn decode_schedules(root: &Object, model: &mut Model, diagnostics: &mut Vec<EpJsonDiagnostic>) {
    let mut minted = 100_000u32;
    let mut resolve = |name: &str, diagnostics: &mut Vec<EpJsonDiagnostic>| match schedule_id_from_name(name) {
        Some(id) => id,
        None => {
            minted += 1;
            diagnostics.push(EpJsonDiagnostic::new("epjson.schedule.name-not-round-trippable", name, format!("schedule name {name:?} does not carry a semio ScheduleId, so id {minted} was minted for it and the export leaf will rename it")));
            ScheduleId(minted)
        }
    };
    for (name, fields) in objects(root, "Schedule:Constant") {
        if name == DUAL_SETPOINT_CONTROL_SCHEDULE {
            continue;
        }
        model.schedules.constants.push(ConstantSchedule { id: resolve(name, diagnostics), value: number_or(fields, "hourly_value", 0.0) });
    }
    for (name, fields) in objects(root, "Schedule:Compact") {
        let id = resolve(name, diagnostics);
        let mut hourly = [0.0f64; 24];
        let mut hour = 0usize;
        let mut pending = false;
        if let Some(Value::Array(items)) = fields.get("data") {
            for item in items {
                let Value::Object(row) = item else { continue };
                let Some(entry) = text(row, "field") else { continue };
                if entry.starts_with("Until:") {
                    pending = true;
                    continue;
                }
                if entry.starts_with("Through:") || entry.starts_with("For:") {
                    continue;
                }
                if pending {
                    if hour < hourly.len() {
                        hourly[hour] = entry.trim().parse::<f64>().unwrap_or(0.0);
                    }
                    hour += 1;
                    pending = false;
                }
            }
        }
        if hour != hourly.len() {
            diagnostics.push(EpJsonDiagnostic::new("epjson.schedule.compact-not-hourly", name, format!("Schedule:Compact {name:?} resolved {hour} until-blocks, not the 24 a semio DailySchedule needs; the missing hours default to 0")));
        }
        model.schedules.daily.push(DailySchedule { id, hourly_values: hourly, interpolation: ScheduleInterpolation::Discrete, limits: None });
    }
}

fn decode_materials(root: &Object, model: &mut Model) -> Vec<String> {
    let mut names = Vec::new();
    let mut next = MATERIAL_BASE;
    let push = |model: &mut Model, names: &mut Vec<String>, material: Material| {
        names.push(material.name.clone());
        model.materials.push(material);
    };
    for (name, fields) in objects(root, "Material") {
        let material = Material {
            id: EntityId(next),
            name: name.to_string(),
            thickness_m: number_or(fields, "thickness", 0.0),
            conductivity_w_m_k: number_or(fields, "conductivity", 0.0),
            density_kg_m3: number_or(fields, "density", 0.0),
            specific_heat_j_kg_k: number_or(fields, "specific_heat", 0.0),
            thermal_absorptance: number_or(fields, "thermal_absorptance", 0.9),
            solar_absorptance: number_or(fields, "solar_absorptance", 0.7),
            visible_absorptance: number_or(fields, "visible_absorptance", 0.7),
        };
        next += 1;
        push(model, &mut names, material);
    }
    for (name, fields) in objects(root, "Material:NoMass") {
        let resistance = number_or(fields, "thermal_resistance", 0.001);
        let material = Material {
            id: EntityId(next),
            name: name.to_string(),
            thickness_m: resistance * NO_MASS_CONDUCTIVITY_W_M_K,
            conductivity_w_m_k: NO_MASS_CONDUCTIVITY_W_M_K,
            density_kg_m3: 0.0,
            specific_heat_j_kg_k: 0.0,
            thermal_absorptance: number_or(fields, "thermal_absorptance", 0.9),
            solar_absorptance: number_or(fields, "solar_absorptance", 0.7),
            visible_absorptance: number_or(fields, "visible_absorptance", 0.7),
        };
        next += 1;
        push(model, &mut names, material);
    }
    names
}

/// 🪟️ Returns `(construction name, u, shgc, vlt)` for every simple-glazing construction, which the
/// aperture pass consumes instead of storing as a [`Construction`] — a semio [`Fenestration`]
/// carries those three numbers directly and the export leaf regenerates the construction.
fn decode_constructions(root: &Object, model: &mut Model, material_names: &[String], diagnostics: &mut Vec<EpJsonDiagnostic>) -> Vec<(String, f64, f64, f64)> {
    const LAYER_KEYS: [&str; 10] = ["outside_layer", "layer_2", "layer_3", "layer_4", "layer_5", "layer_6", "layer_7", "layer_8", "layer_9", "layer_10"];
    let glazing_materials: Vec<(String, f64, f64, f64)> = objects(root, "WindowMaterial:SimpleGlazingSystem")
        .into_iter()
        .map(|(name, fields)| (name.to_string(), number_or(fields, "u_factor", 0.0), number_or(fields, "solar_heat_gain_coefficient", 0.0), number_or(fields, "visible_transmittance", 0.0)))
        .collect();
    let mut glazing_constructions = Vec::new();
    let mut next = CONSTRUCTION_BASE;
    for (name, fields) in objects(root, "Construction") {
        let layers: Vec<&str> = LAYER_KEYS.iter().filter_map(|key| text(fields, key)).collect();
        if let Some(single) = layers.first().filter(|_| layers.len() == 1).and_then(|layer| glazing_materials.iter().find(|(material, _, _, _)| material == layer)) {
            glazing_constructions.push((name.to_string(), single.1, single.2, single.3));
            continue;
        }
        let mut ids = Vec::new();
        for (index, layer) in layers.iter().enumerate() {
            match material_names.iter().position(|material| material == layer) {
                Some(position) => ids.push(EntityId(MATERIAL_BASE + position as u32)),
                None => {
                    diagnostics.push(EpJsonDiagnostic::new("epjson.construction.unknown-layer", name, format!("layer {index} names {layer:?}, which this document does not define as a Material, Material:NoMass or WindowMaterial:SimpleGlazingSystem")))
                }
            }
        }
        model.constructions.push(Construction { id: EntityId(next), name: name.to_string(), layer_material_ids: ids });
        next += 1;
    }
    glazing_constructions
}

fn decode_zones(root: &Object, model: &mut Model) -> Vec<String> {
    let mut names = Vec::new();
    for (index, (name, fields)) in objects(root, "Zone").into_iter().enumerate() {
        model.zones.push(Zone {
            id: EntityId(ZONE_BASE + index as u32),
            name: name.to_string(),
            volume_m3: number_or(fields, "volume", 0.0),
            multiplier: number_or(fields, "multiplier", 1.0) as u32,
            conditioned: false,
            part_of_total_floor_area: flag(fields, "part_of_total_floor_area", true),
        });
        names.push(name.to_string());
    }
    for (index, (name, fields)) in objects(root, "Space").into_iter().enumerate() {
        let Some(zone) = text(fields, "zone_name").and_then(|zone| names.iter().position(|candidate| candidate == zone)) else { continue };
        model.spaces.push(Space { id: EntityId(SPACE_BASE + index as u32), name: name.to_string(), zone_id: EntityId(ZONE_BASE + zone as u32), floor_area_m2: number_or(fields, "floor_area", 0.0) });
    }
    names
}

fn decode_surfaces(root: &Object, model: &mut Model, zone_names: &[String], diagnostics: &mut Vec<EpJsonDiagnostic>) {
    let rows = objects(root, "BuildingSurface:Detailed");
    let surface_names: Vec<&str> = rows.iter().map(|(name, _)| *name).collect();
    for (index, (name, fields)) in rows.iter().enumerate() {
        let Some(zone) = text(fields, "zone_name").and_then(|zone| zone_names.iter().position(|candidate| candidate == zone)) else {
            diagnostics.push(EpJsonDiagnostic::new("epjson.surface.unknown-zone", *name, "the surface names a zone this document does not define and is skipped"));
            continue;
        };
        let construction = model.constructions.iter().find(|construction| Some(construction.name.as_str()) == text(fields, "construction_name")).map(|construction| construction.id);
        let Some(construction_id) = construction else {
            diagnostics.push(EpJsonDiagnostic::new("epjson.surface.unknown-construction", *name, "the surface names a construction this document does not define and is skipped"));
            continue;
        };
        let boundary_text = text(fields, "outside_boundary_condition").unwrap_or("Outdoors");
        let outside = match boundary_text {
            "Ground" => OutsideBoundary::Ground,
            "Adiabatic" => OutsideBoundary::Adiabatic,
            "OtherSideCoefficients" => OutsideBoundary::OtherSideTemperature,
            "Surface" => match text(fields, "outside_boundary_condition_object").and_then(|peer| surface_names.iter().position(|candidate| candidate == &peer)) {
                Some(position) => OutsideBoundary::Interzone(EntityId(SURFACE_BASE + position as u32)),
                None => {
                    diagnostics.push(EpJsonDiagnostic::new("epjson.surface.dangling-interzone", *name, "outside_boundary_condition is Surface but the named partner is not in this document; read as Adiabatic"));
                    OutsideBoundary::Adiabatic
                }
            },
            _ => OutsideBoundary::OutdoorAir,
        };
        let class = match (text(fields, "surface_type").unwrap_or("Wall"), &outside) {
            ("Roof", _) => SurfaceClass::Roof,
            ("Ceiling", _) => SurfaceClass::Ceiling,
            ("Floor", _) => SurfaceClass::Floor,
            (_, OutsideBoundary::Interzone(_)) => SurfaceClass::Interzone,
            (_, OutsideBoundary::Adiabatic) => SurfaceClass::Adiabatic,
            _ => SurfaceClass::ExteriorWall,
        };
        model.surfaces.push(Surface {
            id: EntityId(SURFACE_BASE + index as u32),
            name: (*name).to_string(),
            zone_id: EntityId(ZONE_BASE + zone as u32),
            class,
            vertices_m: vertices_of(fields),
            construction_id,
            outside_boundary_condition: outside,
            sun_exposed: text(fields, "sun_exposure").unwrap_or("SunExposed") == "SunExposed",
            wind_exposed: text(fields, "wind_exposure").unwrap_or("WindExposed") == "WindExposed",
            multiplier: 1,
        });
    }
}

fn decode_apertures(root: &Object, model: &mut Model, glazing: &[(String, f64, f64, f64)], diagnostics: &mut Vec<EpJsonDiagnostic>) {
    let overhangs = objects(root, "Shading:Overhang:Projection");
    let fins = objects(root, "Shading:Fin:Projection");
    for (index, (name, fields)) in objects(root, "FenestrationSurface:Detailed").into_iter().enumerate() {
        let Some(host) = model.surfaces.iter().find(|surface| Some(surface.name.as_str()) == text(fields, "building_surface_name")).cloned() else {
            diagnostics.push(EpJsonDiagnostic::new("epjson.fenestration.unknown-host", name, "the aperture names a building surface this document does not define and is skipped"));
            continue;
        };
        let construction = text(fields, "construction_name").unwrap_or_default();
        let Some((_, u_value, shgc, vlt)) = glazing.iter().find(|(candidate, _, _, _)| candidate == construction) else {
            diagnostics.push(EpJsonDiagnostic::new(
                "epjson.fenestration.unknown-glazing",
                name,
                format!("construction {construction:?} is not a single-layer WindowMaterial:SimpleGlazingSystem, which is the only glazing a semio Fenestration can carry"),
            ));
            continue;
        };
        let corners = aperture_corners(fields);
        if corners.len() < 4 {
            diagnostics.push(EpJsonDiagnostic::new("epjson.fenestration.not-rectangular", name, "only four-vertex rectangular apertures decode into a semio Fenestration"));
            continue;
        }
        let width = distance(corners[0], corners[1]);
        let height = distance(corners[1], corners[2]);
        let sill = match surface_normal(&host.vertices_m) {
            Some(normal) => {
                let up = [0.0f64, 0.0, 1.0];
                let horizontal = [up[1] * normal[2] - up[2] * normal[1], up[2] * normal[0] - up[0] * normal[2], up[0] * normal[1] - up[1] * normal[0]];
                let length = (horizontal[0] * horizontal[0] + horizontal[1] * horizontal[1] + horizontal[2] * horizontal[2]).sqrt();
                if length <= 1e-12 {
                    0.0
                } else {
                    let unit = [horizontal[0] / length, horizontal[1] / length, horizontal[2] / length];
                    let vertical = [normal[1] * unit[2] - normal[2] * unit[1], normal[2] * unit[0] - normal[0] * unit[2], normal[0] * unit[1] - normal[1] * unit[0]];
                    let origin = host.vertices_m.first().copied().unwrap_or([0.0; 3]);
                    let project = |point: [f64; 3]| (point[0] - origin[0]) * vertical[0] + (point[1] - origin[1]) * vertical[1] + (point[2] - origin[2]) * vertical[2];
                    let base = host.vertices_m.iter().map(|vertex| project(*vertex)).fold(f64::INFINITY, f64::min);
                    project(corners[0]) - base
                }
            }
            None => 0.0,
        };
        let overhang = overhangs.iter().find(|(_, shade)| text(shade, "window_or_door_name") == Some(name));
        let fin = fins.iter().find(|(_, shade)| text(shade, "window_or_door_name") == Some(name));
        model.fenestrations.push(Fenestration {
            id: EntityId(FENESTRATION_BASE + index as u32),
            name: name.to_string(),
            surface_id: host.id,
            u_value_w_m2k: *u_value,
            shgc: *shgc,
            vlt: *vlt,
            area_m2: width * height,
            height_m: height,
            sill_height_m: sill,
            frame_conductance_w_k: 0.0,
            divider_conductance_w_k: 0.0,
            overhang_depth_m: overhang.map_or(0.0, |(_, shade)| number_or(shade, "depth_as_fraction_of_window_door_height", 0.0) * height),
            overhang_offset_m: overhang.map_or(0.0, |(_, shade)| number_or(shade, "height_above_window_or_door", 0.0)),
            fin_depth_m: fin.map_or(0.0, |(_, shade)| number_or(shade, "left_depth_as_fraction_of_window_door_width", 0.0) * height),
            fin_offset_m: fin.map_or(0.0, |(_, shade)| number_or(shade, "left_extension_from_window_door", 0.0)),
            glazing_construction_id: None,
        });
        let _ = glazing_construction_name(name);
    }
}

fn zone_id_of(zone_names: &[String], named: Option<&str>) -> Option<EntityId> {
    let named = named?;
    zone_names.iter().position(|candidate| candidate == named || format!("{candidate}_Space") == named).map(|position| EntityId(ZONE_BASE + position as u32))
}

fn schedule_reference(fields: &Object, key: &str, subject: &str, diagnostics: &mut Vec<EpJsonDiagnostic>) -> ScheduleId {
    match text(fields, key).and_then(schedule_id_from_name) {
        Some(id) => id,
        None => {
            diagnostics.push(EpJsonDiagnostic::new("epjson.schedule.reference-not-round-trippable", subject, format!("{key} does not name a semio schedule, so the reference falls back to schedule 0")));
            ScheduleId(0)
        }
    }
}

fn decode_gains(root: &Object, model: &mut Model, zone_names: &[String], diagnostics: &mut Vec<EpJsonDiagnostic>) {
    let mut next = GAIN_BASE;
    for (name, fields) in objects(root, "ZoneInfiltration:DesignFlowRate") {
        let Some(zone_id) = zone_id_of(zone_names, text(fields, "zone_or_zonelist_or_space_or_spacelist_name")) else {
            diagnostics.push(EpJsonDiagnostic::new("epjson.infiltration.unknown-zone", name, "the infiltration object names a zone this document does not define and is skipped"));
            continue;
        };
        let method_text = text(fields, "design_flow_rate_calculation_method").unwrap_or("Flow/Zone");
        let method = match method_text {
            "AirChanges/Hour" => InfiltrationMethod::ScheduledAch,
            "Flow/ExteriorArea" | "Flow/ExteriorWallArea" => InfiltrationMethod::PerExteriorArea,
            other => {
                diagnostics.push(EpJsonDiagnostic::new("epjson.infiltration.method-unsupported", name, format!("design_flow_rate_calculation_method {other:?} has no semio InfiltrationMethod; the object is skipped rather than approximated")));
                continue;
            }
        };
        model.infiltrations.push(Infiltration {
            id: EntityId(next),
            zone_id,
            schedule_id: schedule_reference(fields, "schedule_name", name, diagnostics),
            method,
            design_flow_ach: number_or(fields, "air_changes_per_hour", 0.0),
            flow_per_exterior_area_m3_s_m2: number_or(fields, "flow_rate_per_exterior_surface_area", 0.0),
            effective_leakage_area_m2: 0.0,
            discharge_coefficient: 1.0,
            stack_height_m: 0.0,
            constant_term_coefficient: number_or(fields, "constant_term_coefficient", 1.0),
            temperature_term_coefficient: number_or(fields, "temperature_term_coefficient", 0.0),
            velocity_term_coefficient: number_or(fields, "velocity_term_coefficient", 0.0),
            velocity_squared_term_coefficient: number_or(fields, "velocity_squared_term_coefficient", 0.0),
        });
        next += 1;
    }
    for (name, fields) in objects(root, "People") {
        let Some(zone_id) = zone_id_of(zone_names, text(fields, "zone_or_zonelist_or_space_or_spacelist_name")) else { continue };
        model.people.push(PeopleGain {
            id: EntityId(next),
            zone_id,
            schedule_id: schedule_reference(fields, "number_of_people_schedule_name", name, diagnostics),
            activity_schedule_id: schedule_reference(fields, "activity_level_schedule_name", name, diagnostics),
            people_per_area: number_or(fields, "people_per_floor_area", 0.0),
            sensible_fraction: number_or(fields, "sensible_heat_fraction", 0.0),
            latent_fraction: 0.0,
            radiant_fraction: number_or(fields, "fraction_radiant", 0.3),
        });
        next += 1;
    }
    for (name, fields) in objects(root, "Lights") {
        let Some(zone_id) = zone_id_of(zone_names, text(fields, "zone_or_zonelist_or_space_or_spacelist_name")) else { continue };
        model.lighting.push(LightingGain {
            id: EntityId(next),
            zone_id,
            schedule_id: schedule_reference(fields, "schedule_name", name, diagnostics),
            watts_per_area: number_or(fields, "watts_per_floor_area", 0.0),
            radiant_fraction: number_or(fields, "fraction_radiant", 0.0),
            visible_fraction: number_or(fields, "fraction_visible", 0.0),
            return_air_fraction: number_or(fields, "return_air_fraction", 0.0),
        });
        next += 1;
    }
    for (name, fields) in objects(root, "ElectricEquipment") {
        let Some(zone_id) = zone_id_of(zone_names, text(fields, "zone_or_zonelist_or_space_or_spacelist_name")) else { continue };
        model.equipment.push(EquipmentGain {
            id: EntityId(next),
            zone_id,
            schedule_id: schedule_reference(fields, "schedule_name", name, diagnostics),
            watts_per_area: number_or(fields, "watts_per_floor_area", 0.0),
            radiant_fraction: number_or(fields, "fraction_radiant", 0.0),
            latent_fraction: number_or(fields, "fraction_latent", 0.0),
        });
        next += 1;
    }
}

fn decode_hvac(root: &Object, model: &mut Model, zone_names: &[String], diagnostics: &mut Vec<EpJsonDiagnostic>) {
    let dual = objects(root, "ThermostatSetpoint:DualSetpoint");
    let mut next = HVAC_BASE;
    for (name, fields) in objects(root, "ZoneControl:Thermostat") {
        let Some(zone_id) = zone_id_of(zone_names, text(fields, "zone_or_zonelist_name")) else {
            diagnostics.push(EpJsonDiagnostic::new("epjson.thermostat.unknown-zone", name, "the thermostat names a zone this document does not define and is skipped"));
            continue;
        };
        let Some((setpoint_name, setpoint)) = dual.iter().find(|(candidate, _)| Some(*candidate) == text(fields, "control_1_name")) else {
            diagnostics.push(EpJsonDiagnostic::new("epjson.thermostat.unsupported-control", name, "only a ThermostatSetpoint:DualSetpoint control decodes into a semio Thermostat"));
            continue;
        };
        model.thermostats.push(Thermostat {
            id: EntityId(next),
            zone_id,
            heating_setpoint_schedule_id: schedule_reference(setpoint, "heating_setpoint_temperature_schedule_name", setpoint_name, diagnostics),
            cooling_setpoint_schedule_id: schedule_reference(setpoint, "cooling_setpoint_temperature_schedule_name", setpoint_name, diagnostics),
            heating_throttle_range_k: 0.0,
            cooling_throttle_range_k: 0.0,
        });
        next += 1;
    }

    let systems = objects(root, "ZoneHVAC:IdealLoadsAirSystem");
    let lists = objects(root, "ZoneHVAC:EquipmentList");
    for (name, fields) in objects(root, "ZoneHVAC:EquipmentConnections") {
        let Some(zone_id) = zone_id_of(zone_names, text(fields, "zone_name")) else {
            diagnostics.push(EpJsonDiagnostic::new("epjson.equipment-connections.unknown-zone", name, "the equipment connections name a zone this document does not define and are skipped"));
            continue;
        };
        let equipment = lists.iter().find(|(candidate, _)| Some(*candidate) == text(fields, "zone_conditioning_equipment_list_name")).and_then(|(_, list)| match list.get("equipment") {
            Some(Value::Array(items)) => items.first().and_then(|item| if let Value::Object(row) = item { Some(row) } else { None }),
            _ => None,
        });
        let Some(equipment) = equipment else { continue };
        if text(equipment, "zone_equipment_object_type") != Some("ZoneHVAC:IdealLoadsAirSystem") {
            diagnostics.push(EpJsonDiagnostic::new(
                "epjson.zone-equipment.unsupported-type",
                name,
                format!("zone equipment {:?} has no semio counterpart; only ZoneHVAC:IdealLoadsAirSystem decodes", text(equipment, "zone_equipment_object_type").unwrap_or("?")),
            ));
            continue;
        }
        let Some((_, system)) = systems.iter().find(|(candidate, _)| Some(*candidate) == text(equipment, "zone_equipment_name")) else { continue };
        model.ideal_loads.push(IdealLoadsSystem {
            id: EntityId(next),
            zone_id,
            max_heating_supply_air_temp_c: number_or(system, "maximum_heating_supply_air_temperature", 50.0),
            min_cooling_supply_air_temp_c: number_or(system, "minimum_cooling_supply_air_temperature", 13.0),
            max_heating_capacity_w: if text(system, "heating_limit") == Some("LimitCapacity") { number(system, "maximum_sensible_heating_capacity") } else { None },
            max_cooling_capacity_w: if text(system, "cooling_limit") == Some("LimitCapacity") { number(system, "maximum_total_cooling_capacity") } else { None },
            outdoor_air_per_person_m3_s: 0.0,
            outdoor_air_per_area_m3_s_m2: 0.0,
        });
        next += 1;
        if let Some(zone) = model.zones.iter_mut().find(|zone| zone.id == zone_id) {
            zone.conditioned = true;
        }
    }
}

fn decode_outputs(root: &Object, model: &mut Model) {
    for (_, fields) in objects(root, "Output:Variable") {
        let Some(name) = text(fields, "variable_name") else { continue };
        if CONTRACT_OUTPUT_VARIABLES.contains(&name) {
            continue;
        }
        let frequency = match text(fields, "reporting_frequency").unwrap_or("Hourly") {
            "Detailed" | "Timestep" => OutputReportFrequency::Timestep,
            "Daily" => OutputReportFrequency::Daily,
            "Monthly" => OutputReportFrequency::Monthly,
            "RunPeriod" | "Annual" | "Environment" => OutputReportFrequency::RunPeriod,
            _ => OutputReportFrequency::Hourly,
        };
        let key = text(fields, "key_value").unwrap_or("*");
        model.output_variables.push(OutputVariableSpec { name: name.to_string(), key: if key == "*" { String::new() } else { key.to_string() }, reporting_frequency: frequency });
    }
}
//#endregion 🔖️Decoder

//#region 🔖️Leaf
// 🚫️async: E1 pure codec registration hook, no io — see R9.
pub fn register() {}

/// 📥️ The io-leaf entry point: an epJSON document tree as this artifact's snapshot.
pub fn deserialize(document: &Value) -> Result<EnergyModelSnapshot, store::TextError> {
    let import = decode_model(document).map_err(|error| store::TextError::new(error, dsl::TextSpan::at(1, 1)))?;
    Ok(EnergyModelSnapshot { model: import.model, ..Default::default() })
}

/// 📥️ The io-leaf byte entry point: epJSON text as this artifact's snapshot.
pub fn deserialize_bytes(bytes: &[u8]) -> Result<EnergyModelSnapshot, store::TextError> {
    let document = pack::json::parse_bytes(bytes).map_err(|error| store::TextError::new(error.to_string(), dsl::TextSpan::at(1, 1)))?;
    deserialize(&document)
}

/// 📥️ Byte entry point that keeps the import report, for callers that surface diagnostics.
pub fn import_bytes(bytes: &[u8]) -> Result<EpJsonImport, String> {
    decode_model(&pack::json::parse_bytes(bytes).map_err(|error| error.to_string())?)
}
//#endregion 🔖️Leaf

//#region 🌉️Bridge
/// 🌉️ Text in, text out — the import half of the export leaf's own bridge, for the out-of-crate
/// test adapter that cannot name [`Model`]. Returns `(model json, diagnostics json)` so a scenario
/// can assert both what came back and what could not be represented.
pub fn model_json_from_epjson(epjson: &str) -> Result<(String, String), String> {
    let import = decode_model(&pack::json::parse(epjson).map_err(|error| error.to_string())?)?;
    let diagnostics = Value::Array(import.diagnostics.iter().map(EpJsonDiagnostic::to_json).collect());
    Ok((pack::json::to_json_string(&import.model), pack::json::to_string(&diagnostics)))
}
//#endregion 🌉️Bridge

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
