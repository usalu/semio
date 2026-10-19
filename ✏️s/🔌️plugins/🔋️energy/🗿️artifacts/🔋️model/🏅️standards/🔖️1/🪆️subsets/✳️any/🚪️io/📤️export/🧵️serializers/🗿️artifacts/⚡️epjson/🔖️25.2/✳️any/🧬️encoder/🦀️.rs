//! 🧬️ The model to epJSON encoder: schedules, materials, constructions, zones, surfaces, gains, hvac and outputs.

use super::*;

//#region 🔖️Encoder
/// 📤️ Encodes one [`Model`] as an EnergyPlus 25.2 epJSON document, together with everything the
/// covered object subset could not carry.
pub fn encode_model_with_diagnostics(model: &Model) -> (Value, Vec<EpJsonDiagnostic>) {
    let mut diagnostics = Vec::new();
    let mut document = Object::new();
    let zone_name_of = |id: EntityId| model.zones.iter().find(|zone| zone.id == id).map(|zone| entity_name("Zone", zone.id, &zone.name));
    let material_name_of = |id: EntityId| {
        model
            .materials
            .iter()
            .find(|material| material.id == id)
            .map(|material| entity_name("Material", material.id, &material.name))
            .or_else(|| model.glazing_materials.iter().find(|pane| pane.id == id).map(|pane| entity_name("Glazing", pane.id, &pane.name)))
            .or_else(|| model.gas_materials.iter().find(|gap| gap.id == id).map(|gap| entity_name("Gas", gap.id, &gap.name)))
    };
    let construction_name_of = |id: EntityId| model.constructions.iter().find(|construction| construction.id == id).map(|construction| entity_name("Construction", construction.id, &construction.name));

    document.insert("Version", Value::Object(Object::from_iter([entry("Version 1", [field("version_identifier", EPJSON_VERSION)])])));

    let zone_sizing = !model.sizing_objects.is_empty();
    document.insert(
        "SimulationControl",
        Value::Object(Object::from_iter([entry(
            "SimulationControl 1",
            [
                field("do_zone_sizing_calculation", yes_no(zone_sizing)),
                field("do_system_sizing_calculation", yes_no(false)),
                field("do_plant_sizing_calculation", yes_no(false)),
                field("run_simulation_for_sizing_periods", yes_no(false)),
                field("run_simulation_for_weather_file_run_periods", yes_no(true)),
            ],
        )])),
    );

    let building = entity_name("Building", EntityId(0), &model.name);
    document.insert("Building", Value::Object(Object::from_iter([entry(building.clone(), [field("north_axis", model.site.north_axis_deg), field("terrain", EPJSON_TERRAIN), field("solar_distribution", EPJSON_SOLAR_DISTRIBUTION)])])));
    document.insert(
        "ShadowCalculation",
        Value::Object(Object::from_iter([entry(
            "ShadowCalculation 1",
            [field("shading_calculation_method", "PolygonClipping"), field("shading_calculation_update_frequency_method", "Periodic"), field("shading_calculation_update_frequency", SHADOW_UPDATE_FREQUENCY_DAYS)],
        )])),
    );
    document.insert(
        "Site:Location",
        Value::Object(Object::from_iter([entry(
            format!("{building} Site"),
            [field("latitude", model.site.latitude_deg), field("longitude", model.site.longitude_deg), field("time_zone", model.site.time_zone_hours), field("elevation", model.site.elevation_m)],
        )])),
    );

    if model.surfaces.iter().any(|surface| matches!(surface.outside_boundary_condition, OutsideBoundary::Ground)) {
        const MONTHS: [&str; 12] = ["january", "february", "march", "april", "may", "june", "july", "august", "september", "october", "november", "december"];
        let temperatures = MONTHS.iter().enumerate().map(|(index, month)| field(&format!("{month}_ground_temperature"), model.ground_temperature.building_surface_c[index])).collect::<Vec<_>>();
        document.insert("Site:GroundTemperature:BuildingSurface", Value::Object(Object::from_iter([entry("Site:GroundTemperature:BuildingSurface 1", temperatures)])));
    }

    document.insert(
        "GlobalGeometryRules",
        Value::Object(Object::from_iter([entry("GlobalGeometryRules 1", [field("starting_vertex_position", "UpperLeftCorner"), field("vertex_entry_direction", "Counterclockwise"), field("coordinate_system", "World")])])),
    );
    document.insert("Timestep", Value::Object(Object::from_iter([entry("Timestep 1", [field("number_of_timesteps_per_hour", TIMESTEPS_PER_HOUR)])])));

    let period = &model.run_period;
    document.insert(
        "RunPeriod",
        Value::Object(Object::from_iter([entry(
            "Run Period 1",
            [
                field("begin_month", period.start_month as i64),
                field("begin_day_of_month", period.start_day as i64),
                field("begin_year", period.year as i64),
                field("end_month", period.end_month as i64),
                field("end_day_of_month", period.end_day as i64),
                field("end_year", period.year as i64),
                field("use_weather_file_holidays_and_special_days", yes_no(false)),
                field("use_weather_file_daylight_saving_period", yes_no(false)),
                field("apply_weekend_holiday_rule", yes_no(false)),
                field("use_weather_file_rain_indicators", yes_no(true)),
                field("use_weather_file_snow_indicators", yes_no(true)),
            ],
        )])),
    );

    encode_schedules(model, &mut document, &mut diagnostics);
    encode_materials(model, &mut document);
    encode_constructions(model, &mut document, &material_name_of, &mut diagnostics);
    encode_zones(model, &mut document);
    encode_surfaces(model, &mut document, &zone_name_of, &construction_name_of, &mut diagnostics);
    encode_gains(model, &mut document, &zone_name_of, &mut diagnostics);
    encode_hvac(model, &mut document, &zone_name_of, &mut diagnostics);
    encode_outputs(model, &mut document);
    (Value::Object(document), diagnostics)
}

/// 📤️ [`encode_model_with_diagnostics`] without the diagnostics channel.
pub fn encode_model(model: &Model) -> Value {
    encode_model_with_diagnostics(model).0
}

fn encode_schedules(model: &Model, document: &mut Object, diagnostics: &mut Vec<EpJsonDiagnostic>) {
    let schedules = &model.schedules;
    let mut limits = vec![entry(SCHEDULE_LIMITS_ANY, [field("numeric_type", "Continuous"), field("unit_type", "Dimensionless")])];
    let thermostat = !model.thermostats.is_empty();
    if thermostat {
        limits.push(entry(SCHEDULE_LIMITS_CONTROL, [field("lower_limit_value", 0.0), field("upper_limit_value", 4.0), field("numeric_type", "Discrete"), field("unit_type", "Control")]));
    }
    document.insert("ScheduleTypeLimits", Value::Object(Object::from_iter(limits)));

    let mut constants: Vec<(String, Value)> = schedules.constants.iter().map(|constant| entry(schedule_name(constant.id), [field("schedule_type_limits_name", SCHEDULE_LIMITS_ANY), field("hourly_value", constant.value)])).collect();
    if thermostat {
        constants.push(entry(DUAL_SETPOINT_CONTROL_SCHEDULE, [field("schedule_type_limits_name", SCHEDULE_LIMITS_CONTROL), field("hourly_value", 4.0)]));
    }
    if !constants.is_empty() {
        document.insert("Schedule:Constant", Value::Object(Object::from_iter(constants)));
    }

    let compact: Vec<(String, Value)> = schedules
        .daily
        .iter()
        .map(|daily| {
            let mut data = vec![Value::Object(Object::from_iter([field("field", "Through: 12/31")])), Value::Object(Object::from_iter([field("field", "For: AllDays")]))];
            for (hour, value) in daily.hourly_values.iter().enumerate() {
                let clamped = daily.limits.map_or(*value, |limit| value.clamp(limit.min, limit.max));
                data.push(Value::Object(Object::from_iter([field("field", format!("Until: {:02}:00", hour + 1))])));
                data.push(Value::Object(Object::from_iter([field("field", semio_framework_pack_json::format_f64(clamped))])));
            }
            entry(schedule_name(daily.id), [field("schedule_type_limits_name", SCHEDULE_LIMITS_ANY), ("data".to_string(), Value::Array(data))])
        })
        .collect();
    if !compact.is_empty() {
        document.insert("Schedule:Compact", Value::Object(Object::from_iter(compact)));
    }

    for weekly in &schedules.weekly {
        diagnostics.push(EpJsonDiagnostic::new("epjson.schedule.weekly-unsupported", schedule_name(weekly.id), "a weekly schedule is not written to epJSON: this codec does not emit Schedule:Week:Daily. daily_schedule_ids are Sunday through Saturday, slot 0 Sunday"));
    }
    for annual in &schedules.annual {
        diagnostics.push(EpJsonDiagnostic::new(
            "epjson.schedule.annual-unsupported",
            schedule_name(annual.id),
            "an annual rule schedule is not written to epJSON: its holiday-date overrides have no Schedule:Compact equivalent without a RunPeriodControl:SpecialDays projection",
        ));
    }
    for series in &schedules.time_series {
        diagnostics.push(EpJsonDiagnostic::new("epjson.schedule.time-series-unsupported", schedule_name(series.id), "a time-series schedule is not written to epJSON: Schedule:File needs an out-of-band CSV this synchronous codec cannot publish"));
    }
}

fn encode_materials(model: &Model, document: &mut Object) {
    let mut massive: Vec<(String, Value)> = Vec::new();
    let mut massless: Vec<(String, Value)> = Vec::new();
    for material in &model.materials {
        let name = entity_name("Material", material.id, &material.name);
        let optical = [field("thermal_absorptance", material.thermal_absorptance), field("solar_absorptance", material.solar_absorptance), field("visible_absorptance", material.visible_absorptance)];
        if is_massless(material) {
            let resistance = if material.conductivity_w_m_k > 0.0 { material.thickness_m / material.conductivity_w_m_k } else { 0.001 };
            massless.push(entry(name, [field("roughness", "Rough"), field("thermal_resistance", resistance.max(0.001))].into_iter().chain(optical)));
        } else {
            massive.push(entry(
                name,
                [field("roughness", "Rough"), field("thickness", material.thickness_m), field("conductivity", material.conductivity_w_m_k), field("density", material.density_kg_m3), field("specific_heat", material.specific_heat_j_kg_k)]
                    .into_iter()
                    .chain(optical),
            ));
        }
    }
    if !massive.is_empty() {
        document.insert("Material", Value::Object(Object::from_iter(massive)));
    }
    if !massless.is_empty() {
        document.insert("Material:NoMass", Value::Object(Object::from_iter(massless)));
    }
    let panes: Vec<(String, Value)> = model
        .glazing_materials
        .iter()
        .map(|pane| {
            entry(
                entity_name("Glazing", pane.id, &pane.name),
                [
                    field("optical_data_type", "SpectralAverage"),
                    field("thickness", pane.thickness_m),
                    field("solar_transmittance_at_normal_incidence", pane.solar_transmittance),
                    field("front_side_solar_reflectance_at_normal_incidence", pane.solar_reflectance_front),
                    field("back_side_solar_reflectance_at_normal_incidence", pane.solar_reflectance_back),
                    field("visible_transmittance_at_normal_incidence", pane.visible_transmittance),
                    field("front_side_visible_reflectance_at_normal_incidence", pane.visible_reflectance_front),
                    field("back_side_visible_reflectance_at_normal_incidence", pane.visible_reflectance_back),
                    field("infrared_transmittance_at_normal_incidence", pane.infrared_transmittance),
                    field("front_side_infrared_hemispherical_emissivity", pane.infrared_emissivity_front),
                    field("back_side_infrared_hemispherical_emissivity", pane.infrared_emissivity_back),
                    field("conductivity", pane.conductivity_w_m_k),
                ],
            )
        })
        .collect();
    if !panes.is_empty() {
        document.insert("WindowMaterial:Glazing", Value::Object(Object::from_iter(panes)));
    }
    let gaps: Vec<(String, Value)> = model.gas_materials.iter().map(|gap| entry(entity_name("Gas", gap.id, &gap.name), [field("gas_type", gas_kind_name(gap.gas)), field("thickness", gap.thickness_m)])).collect();
    if !gaps.is_empty() {
        document.insert("WindowMaterial:Gas", Value::Object(Object::from_iter(gaps)));
    }
}

/// 🌫️ EnergyPlus's `WindowMaterial:Gas` `gas_type` spelling of one [`GasKind`].
pub fn gas_kind_name(gas: GasKind) -> &'static str {
    match gas {
        GasKind::Air => "Air",
        GasKind::Argon => "Argon",
        GasKind::Krypton => "Krypton",
        GasKind::Xenon => "Xenon",
    }
}

fn encode_constructions(model: &Model, document: &mut Object, material_name_of: &dyn Fn(EntityId) -> Option<String>, diagnostics: &mut Vec<EpJsonDiagnostic>) {
    const LAYER_KEYS: [&str; 10] = ["outside_layer", "layer_2", "layer_3", "layer_4", "layer_5", "layer_6", "layer_7", "layer_8", "layer_9", "layer_10"];
    let mut glazing: Vec<(String, Value)> = Vec::new();
    let mut constructions: Vec<(String, Value)> = Vec::new();
    for construction in &model.constructions {
        let name = entity_name("Construction", construction.id, &construction.name);
        if construction.layer_material_ids.len() > LAYER_KEYS.len() {
            diagnostics.push(EpJsonDiagnostic::new(
                "epjson.construction.too-many-layers",
                name.clone(),
                format!("EnergyPlus Construction carries at most {} layers, this one has {}; the surplus is dropped", LAYER_KEYS.len(), construction.layer_material_ids.len()),
            ));
        }
        let layers: Vec<(String, Value)> = construction
            .layer_material_ids
            .iter()
            .take(LAYER_KEYS.len())
            .enumerate()
            .filter_map(|(index, id)| match material_name_of(*id) {
                Some(material) => Some(field(LAYER_KEYS[index], material)),
                None => {
                    diagnostics.push(EpJsonDiagnostic::new("epjson.construction.unknown-layer", name.clone(), format!("layer {index} references material {} which the model does not define", id.0)));
                    None
                }
            })
            .collect();
        constructions.push(entry(name, layers));
    }
    for window in &model.fenestrations {
        let name = entity_name("Fenestration", window.id, &window.name);
        let material = glazing_material_name(&name);
        glazing.push(entry(material.clone(), [field("u_factor", window.u_value_w_m2k), field("solar_heat_gain_coefficient", window.shgc), field("visible_transmittance", window.vlt)]));
        if glazing_stack(model, window).is_none() {
            if let Some(missing) = window.glazing_construction_id {
                diagnostics.push(EpJsonDiagnostic::new(
                    "epjson.fenestration.dangling-glazing-construction",
                    name.clone(),
                    format!("glazing construction {} is not defined by the model; the window is written with its fallback simple glazing", missing.0),
                ));
            }
            constructions.push(entry(glazing_construction_name(&name), [field("outside_layer", material)]));
        }
        if window.frame_conductance_w_k > 0.0 || window.divider_conductance_w_k > 0.0 {
            diagnostics.push(EpJsonDiagnostic::new(
                "epjson.fenestration.frame-dropped",
                name,
                "frame/divider conductance has no WindowMaterial:SimpleGlazingSystem equivalent and is not written; use WindowProperty:FrameAndDivider once the schema carries frame geometry",
            ));
        }
    }
    if !glazing.is_empty() {
        document.insert("WindowMaterial:SimpleGlazingSystem", Value::Object(Object::from_iter(glazing)));
    }
    if !constructions.is_empty() {
        document.insert("Construction", Value::Object(Object::from_iter(constructions)));
    }
}

fn encode_zones(model: &Model, document: &mut Object) {
    let zones: Vec<(String, Value)> = model
        .zones
        .iter()
        .map(|zone| {
            entry(
                entity_name("Zone", zone.id, &zone.name),
                [
                    field("direction_of_relative_north", 0.0),
                    field("x_origin", 0.0),
                    field("y_origin", 0.0),
                    field("z_origin", 0.0),
                    field("multiplier", zone.multiplier.max(1) as i64),
                    field("volume", zone.volume_m3),
                    field("part_of_total_floor_area", yes_no(zone.part_of_total_floor_area)),
                ],
            )
        })
        .collect();
    if !zones.is_empty() {
        document.insert("Zone", Value::Object(Object::from_iter(zones)));
    }
}

fn encode_surfaces(model: &Model, document: &mut Object, zone_name_of: &dyn Fn(EntityId) -> Option<String>, construction_name_of: &dyn Fn(EntityId) -> Option<String>, diagnostics: &mut Vec<EpJsonDiagnostic>) {
    let mut surfaces: Vec<(String, Value)> = Vec::new();
    for surface in &model.surfaces {
        let name = entity_name("Surface", surface.id, &surface.name);
        let (Some(zone), Some(construction)) = (zone_name_of(surface.zone_id), construction_name_of(surface.construction_id)) else {
            diagnostics.push(EpJsonDiagnostic::new("epjson.surface.dangling-reference", name, "the surface references a zone or construction the model does not define and is not written"));
            continue;
        };
        if surface.multiplier > 1 {
            diagnostics.push(EpJsonDiagnostic::new("epjson.surface.multiplier-dropped", name.clone(), "BuildingSurface:Detailed has no multiplier field; repeat the surface geometry instead"));
        }
        let mut fields = vec![
            field("surface_type", surface_type(surface.class)),
            field("construction_name", construction),
            field("zone_name", zone),
            field("outside_boundary_condition", boundary(&surface.outside_boundary_condition)),
            field("sun_exposure", if surface.sun_exposed { "SunExposed" } else { "NoSun" }),
            field("wind_exposure", if surface.wind_exposed { "WindExposed" } else { "NoWind" }),
        ];
        if let OutsideBoundary::Interzone(other) = surface.outside_boundary_condition {
            match model.surfaces.iter().find(|candidate| candidate.id == other) {
                Some(peer) => {
                    fields[3] = field("outside_boundary_condition", "Surface");
                    fields.push(field("outside_boundary_condition_object", entity_name("Surface", peer.id, &peer.name)));
                }
                None => diagnostics.push(EpJsonDiagnostic::new("epjson.surface.dangling-interzone", name.clone(), format!("the interzone partner surface {} is not defined; the surface is written as Adiabatic", other.0))),
            }
        }
        fields.push(("vertices".to_string(), Value::Array(upper_left_first(&surface.vertices_m).into_iter().map(vertex_value).collect())));
        surfaces.push(entry(name, fields));
    }
    if !surfaces.is_empty() {
        document.insert("BuildingSurface:Detailed", Value::Object(Object::from_iter(surfaces)));
    }

    let mut apertures: Vec<(String, Value)> = Vec::new();
    let mut overhangs: Vec<(String, Value)> = Vec::new();
    let mut fins: Vec<(String, Value)> = Vec::new();
    for surface in &model.surfaces {
        let hosted: Vec<&Fenestration> = model.fenestrations.iter().filter(|window| window.surface_id == surface.id).collect();
        for (index, window) in hosted.iter().enumerate() {
            let name = entity_name("Fenestration", window.id, &window.name);
            let Some(rectangle) = aperture_rectangle(surface, window, index, hosted.len()) else {
                diagnostics.push(EpJsonDiagnostic::new("epjson.fenestration.undetermined-geometry", name, "the host surface is degenerate or the aperture area is non-positive, so no rectangle could be placed"));
                continue;
            };
            let construction = glazing_stack(model, window).map_or_else(|| glazing_construction_name(&name), |stack| entity_name("Construction", stack.id, &stack.name));
            let mut fields = vec![field("surface_type", "Window"), field("construction_name", construction), field("building_surface_name", entity_name("Surface", surface.id, &surface.name))];
            if rectangle.len() != 4 {
                fields.push(field("number_of_vertices", rectangle.len() as f64));
                diagnostics.push(EpJsonDiagnostic::new(
                    "epjson.fenestration.non-rectangular",
                    name.clone(),
                    format!("the aperture carries its own {}-vertex polygon; FenestrationSurface:Detailed accepts four corners, so EnergyPlus will refuse this object", rectangle.len()),
                ));
            }
            for (corner, vertex) in upper_left_first(&rectangle).iter().enumerate() {
                fields.push(field(&format!("vertex_{}_x_coordinate", corner + 1), vertex[0]));
                fields.push(field(&format!("vertex_{}_y_coordinate", corner + 1), vertex[1]));
                fields.push(field(&format!("vertex_{}_z_coordinate", corner + 1), vertex[2]));
            }
            apertures.push(entry(name.clone(), fields));
            if window.overhang_depth_m > 0.0 {
                overhangs.push(entry(
                    format!("{name} Overhang"),
                    [
                        field("window_or_door_name", name.clone()),
                        field("height_above_window_or_door", window.overhang_offset_m),
                        field("tilt_angle_from_window_door", 90.0),
                        field("left_extension_from_window_door_width", 0.0),
                        field("right_extension_from_window_door_width", 0.0),
                        field("depth_as_fraction_of_window_door_height", window.overhang_depth_m / if window.height_m > 0.0 { window.height_m } else { (window.area_m2 / APERTURE_ASPECT).sqrt() }),
                    ],
                ));
            }
            if window.fin_depth_m > 0.0 {
                let height = if window.height_m > 0.0 { window.height_m } else { (window.area_m2 / APERTURE_ASPECT).sqrt() };
                fins.push(entry(
                    format!("{name} Fins"),
                    [
                        field("window_or_door_name", name.clone()),
                        field("left_extension_from_window_door", window.fin_offset_m),
                        field("left_distance_above_top_of_window", 0.0),
                        field("left_distance_below_bottom_of_window", 0.0),
                        field("left_tilt_angle_from_window_door", 90.0),
                        field("left_depth_as_fraction_of_window_door_width", window.fin_depth_m / height),
                        field("right_extension_from_window_door", window.fin_offset_m),
                        field("right_distance_above_top_of_window", 0.0),
                        field("right_distance_below_bottom_of_window", 0.0),
                        field("right_tilt_angle_from_window_door", 90.0),
                        field("right_depth_as_fraction_of_window_door_width", window.fin_depth_m / height),
                    ],
                ));
            }
        }
    }
    for window in &model.fenestrations {
        if !model.surfaces.iter().any(|surface| surface.id == window.surface_id) {
            diagnostics.push(EpJsonDiagnostic::new("epjson.fenestration.dangling-host", entity_name("Fenestration", window.id, &window.name), format!("host surface {} is not defined by the model", window.surface_id.0)));
        }
    }
    if !apertures.is_empty() {
        document.insert("FenestrationSurface:Detailed", Value::Object(Object::from_iter(apertures)));
    }
    if !overhangs.is_empty() {
        document.insert("Shading:Overhang:Projection", Value::Object(Object::from_iter(overhangs)));
    }
    if !fins.is_empty() {
        document.insert("Shading:Fin:Projection", Value::Object(Object::from_iter(fins)));
    }
}

fn encode_gains(model: &Model, document: &mut Object, zone_name_of: &dyn Fn(EntityId) -> Option<String>, diagnostics: &mut Vec<EpJsonDiagnostic>) {
    let mut infiltrations: Vec<(String, Value)> = Vec::new();
    for infiltration in &model.infiltrations {
        let name = entity_name("Infiltration", infiltration.id, "");
        let Some(zone) = zone_name_of(infiltration.zone_id) else {
            diagnostics.push(EpJsonDiagnostic::new("epjson.infiltration.dangling-zone", name, "the infiltration object references an undefined zone"));
            continue;
        };
        let (method, magnitude) = match infiltration.method {
            InfiltrationMethod::ScheduledAch => ("AirChanges/Hour", field("air_changes_per_hour", infiltration.design_flow_ach)),
            InfiltrationMethod::PerExteriorArea => ("Flow/ExteriorArea", field("flow_rate_per_exterior_surface_area", infiltration.flow_per_exterior_area_m3_s_m2)),
            InfiltrationMethod::EffectiveLeakageArea | InfiltrationMethod::WindAndStack => {
                diagnostics.push(EpJsonDiagnostic::new(
                    "epjson.infiltration.method-unsupported",
                    name,
                    "EffectiveLeakageArea and WindAndStack map onto ZoneInfiltration:EffectiveLeakageArea / :FlowCoefficient, which this codec does not write; the object is refused rather than approximated as a design flow rate",
                ));
                continue;
            }
        };
        let [constant, temperature, velocity, velocity_squared] = CONSTANT_DESIGN_FLOW_COEFFICIENTS;
        infiltrations.push(entry(
            format!("{zone} Infiltration {}", infiltration.id.0),
            [
                field("zone_or_zonelist_or_space_or_spacelist_name", zone),
                field("schedule_name", schedule_name(infiltration.schedule_id)),
                field("design_flow_rate_calculation_method", method),
                magnitude,
                field("constant_term_coefficient", constant),
                field("temperature_term_coefficient", temperature),
                field("velocity_term_coefficient", velocity),
                field("velocity_squared_term_coefficient", velocity_squared),
            ],
        ));
    }
    if !infiltrations.is_empty() {
        document.insert("ZoneInfiltration:DesignFlowRate", Value::Object(Object::from_iter(infiltrations)));
    }

    let mut people: Vec<(String, Value)> = Vec::new();
    for occupancy in &model.people {
        let Some(zone) = zone_name_of(occupancy.zone_id) else { continue };
        people.push(entry(
            format!("{zone} People {}", occupancy.id.0),
            [
                field("zone_or_zonelist_or_space_or_spacelist_name", zone),
                field("number_of_people_schedule_name", schedule_name(occupancy.schedule_id)),
                field("number_of_people_calculation_method", "People/Area"),
                field("people_per_floor_area", occupancy.people_per_area),
                field("fraction_radiant", occupancy.radiant_fraction),
                field("sensible_heat_fraction", occupancy.sensible_fraction),
                field("activity_level_schedule_name", schedule_name(occupancy.activity_schedule_id)),
            ],
        ));
    }
    if !people.is_empty() {
        document.insert("People", Value::Object(Object::from_iter(people)));
    }

    let mut lights: Vec<(String, Value)> = Vec::new();
    for lighting in &model.lighting {
        let Some(zone) = zone_name_of(lighting.zone_id) else { continue };
        lights.push(entry(
            format!("{zone} Lights {}", lighting.id.0),
            [
                field("zone_or_zonelist_or_space_or_spacelist_name", zone),
                field("schedule_name", schedule_name(lighting.schedule_id)),
                field("design_level_calculation_method", "Watts/Area"),
                field("watts_per_floor_area", lighting.watts_per_area),
                field("return_air_fraction", lighting.return_air_fraction),
                field("fraction_radiant", lighting.radiant_fraction),
                field("fraction_visible", lighting.visible_fraction),
            ],
        ));
    }
    if !lights.is_empty() {
        document.insert("Lights", Value::Object(Object::from_iter(lights)));
    }

    let mut equipment: Vec<(String, Value)> = Vec::new();
    for gain in &model.equipment {
        let Some(zone) = zone_name_of(gain.zone_id) else { continue };
        equipment.push(entry(
            format!("{zone} Equipment {}", gain.id.0),
            [
                field("zone_or_zonelist_or_space_or_spacelist_name", zone),
                field("schedule_name", schedule_name(gain.schedule_id)),
                field("design_level_calculation_method", "Watts/Area"),
                field("watts_per_floor_area", gain.watts_per_area),
                field("fraction_latent", gain.latent_fraction),
                field("fraction_radiant", gain.radiant_fraction),
                field("fraction_lost", 0.0),
            ],
        ));
    }
    if !equipment.is_empty() {
        document.insert("ElectricEquipment", Value::Object(Object::from_iter(equipment)));
    }
}

fn encode_hvac(model: &Model, document: &mut Object, zone_name_of: &dyn Fn(EntityId) -> Option<String>, diagnostics: &mut Vec<EpJsonDiagnostic>) {
    let mut setpoints: Vec<(String, Value)> = Vec::new();
    let mut controls: Vec<(String, Value)> = Vec::new();
    for thermostat in &model.thermostats {
        let Some(zone) = zone_name_of(thermostat.zone_id) else {
            diagnostics.push(EpJsonDiagnostic::new("epjson.thermostat.dangling-zone", entity_name("Thermostat", thermostat.id, ""), "the thermostat references an undefined zone"));
            continue;
        };
        if thermostat.heating_throttle_range_k != 0.0 || thermostat.cooling_throttle_range_k != 0.0 {
            diagnostics.push(EpJsonDiagnostic::new(
                "epjson.thermostat.throttle-range-dropped",
                zone.clone(),
                "ThermostatSetpoint:DualSetpoint has no throttling range; EnergyPlus applies its own deadband, so the model's throttle ranges are not written",
            ));
        }
        setpoints.push(entry(
            dual_setpoint_name(&zone),
            [field("heating_setpoint_temperature_schedule_name", schedule_name(thermostat.heating_setpoint_schedule_id)), field("cooling_setpoint_temperature_schedule_name", schedule_name(thermostat.cooling_setpoint_schedule_id))],
        ));
        controls.push(entry(
            thermostat_name(&zone),
            [field("zone_or_zonelist_name", zone.clone()), field("control_type_schedule_name", DUAL_SETPOINT_CONTROL_SCHEDULE), field("control_1_object_type", "ThermostatSetpoint:DualSetpoint"), field("control_1_name", dual_setpoint_name(&zone))],
        ));
    }
    if !setpoints.is_empty() {
        document.insert("ThermostatSetpoint:DualSetpoint", Value::Object(Object::from_iter(setpoints)));
        document.insert("ZoneControl:Thermostat", Value::Object(Object::from_iter(controls)));
    }

    let mut systems: Vec<(String, Value)> = Vec::new();
    let mut lists: Vec<(String, Value)> = Vec::new();
    let mut connections: Vec<(String, Value)> = Vec::new();
    let mut node_lists: Vec<(String, Value)> = Vec::new();
    for ideal in &model.ideal_loads {
        let Some(zone) = zone_name_of(ideal.zone_id) else {
            diagnostics.push(EpJsonDiagnostic::new("epjson.ideal-loads.dangling-zone", entity_name("IdealLoads", ideal.id, ""), "the ideal loads system references an undefined zone"));
            continue;
        };
        if ideal.outdoor_air_per_person_m3_s > 0.0 || ideal.outdoor_air_per_area_m3_s_m2 > 0.0 {
            diagnostics.push(EpJsonDiagnostic::new(
                "epjson.ideal-loads.outdoor-air-dropped",
                zone.clone(),
                "outdoor-air rates need a DesignSpecification:OutdoorAir object, which this codec does not write; the system is emitted without mechanical outdoor air",
            ));
        }
        let mut fields =
            vec![field("zone_supply_air_node_name", supply_node_name(&zone)), field("maximum_heating_supply_air_temperature", ideal.max_heating_supply_air_temp_c), field("minimum_cooling_supply_air_temperature", ideal.min_cooling_supply_air_temp_c)];
        match ideal.max_heating_capacity_w {
            Some(capacity) => {
                fields.push(field("heating_limit", "LimitCapacity"));
                fields.push(field("maximum_sensible_heating_capacity", capacity));
            }
            None => fields.push(field("heating_limit", "NoLimit")),
        }
        match ideal.max_cooling_capacity_w {
            Some(capacity) => {
                fields.push(field("cooling_limit", "LimitCapacity"));
                fields.push(field("maximum_total_cooling_capacity", capacity));
            }
            None => fields.push(field("cooling_limit", "NoLimit")),
        }
        fields.push(field("dehumidification_control_type", "None"));
        systems.push(entry(ideal_loads_name(&zone), fields));
        lists.push(entry(
            equipment_list_name(&zone),
            [
                field("load_distribution_scheme", "SequentialLoad"),
                (
                    "equipment".to_string(),
                    Value::Array(vec![Value::Object(Object::from_iter([
                        field("zone_equipment_object_type", "ZoneHVAC:IdealLoadsAirSystem"),
                        field("zone_equipment_name", ideal_loads_name(&zone)),
                        field("zone_equipment_cooling_sequence", 1i64),
                        field("zone_equipment_heating_or_no_load_sequence", 1i64),
                    ]))]),
                ),
            ],
        ));
        node_lists.push(entry(inlet_node_list_name(&zone), [("nodes".to_string(), Value::Array(vec![Value::Object(Object::from_iter([field("node_name", supply_node_name(&zone))]))]))]));
        connections.push(entry(
            zone.clone(),
            [
                field("zone_name", zone.clone()),
                field("zone_conditioning_equipment_list_name", equipment_list_name(&zone)),
                field("zone_air_inlet_node_or_nodelist_name", inlet_node_list_name(&zone)),
                field("zone_air_node_name", zone_air_node_name(&zone)),
                field("zone_return_air_node_or_nodelist_name", return_node_name(&zone)),
            ],
        ));
    }
    if !systems.is_empty() {
        document.insert("ZoneHVAC:IdealLoadsAirSystem", Value::Object(Object::from_iter(systems)));
        document.insert("ZoneHVAC:EquipmentList", Value::Object(Object::from_iter(lists)));
        document.insert("ZoneHVAC:EquipmentConnections", Value::Object(Object::from_iter(connections)));
        document.insert("NodeList", Value::Object(Object::from_iter(node_lists)));
    }

    let sizing: Vec<(String, Value)> = model
        .sizing_objects
        .iter()
        .filter_map(|object| {
            let zone = zone_name_of(object.zone_id)?;
            Some(entry(
                format!("{zone} Sizing {}", object.id.0),
                [
                    field("zone_or_zonelist_name", zone),
                    field("zone_cooling_design_supply_air_temperature", 13.0),
                    field("zone_heating_design_supply_air_temperature", 50.0),
                    field("zone_cooling_design_supply_air_humidity_ratio", 0.0085),
                    field("zone_heating_design_supply_air_humidity_ratio", 0.008),
                ],
            ))
        })
        .collect();
    if !sizing.is_empty() {
        document.insert("Sizing:Zone", Value::Object(Object::from_iter(sizing)));
    }
}

fn encode_outputs(model: &Model, document: &mut Object) {
    let mut variables: Vec<(String, Value)> = Vec::new();
    for (index, name) in CONTRACT_OUTPUT_VARIABLES.iter().enumerate() {
        variables.push(entry(format!("Output:Variable {}", index + 1), [field("key_value", "*"), field("variable_name", *name), field("reporting_frequency", "Hourly")]));
    }
    for (index, requested) in model.output_variables.iter().enumerate() {
        if CONTRACT_OUTPUT_VARIABLES.contains(&requested.name.as_str()) {
            continue;
        }
        let frequency = match requested.reporting_frequency {
            crate::model::OutputReportFrequency::Timestep => "Detailed",
            crate::model::OutputReportFrequency::Hourly => "Hourly",
            crate::model::OutputReportFrequency::Daily => "Daily",
            crate::model::OutputReportFrequency::Monthly => "Monthly",
            crate::model::OutputReportFrequency::RunPeriod => "RunPeriod",
        };
        let key = if requested.key.trim().is_empty() { "*".to_string() } else { requested.key.clone() };
        variables.push(entry(format!("Output:Variable {}", CONTRACT_OUTPUT_VARIABLES.len() + index + 1), [field("key_value", key), field("variable_name", requested.name.as_str()), field("reporting_frequency", frequency)]));
    }
    document.insert("Output:Variable", Value::Object(Object::from_iter(variables)));
    document.insert("Output:Meter", Value::Object(Object::from_iter(CONTRACT_OUTPUT_METERS.iter().enumerate().map(|(index, meter)| entry(format!("Output:Meter {}", index + 1), [field("key_name", *meter), field("reporting_frequency", "Hourly")])))));
    document.insert("Output:SQLite", Value::Object(Object::from_iter([entry("Output:SQLite 1", [field("option_type", "SimpleAndTabular")])])));
}
//#endregion 🔖️Encoder
