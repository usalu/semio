//! 🧬️ The infiltration entity, the `Model` aggregate with its validation and the public test models.

use super::*;

// #region 🔖️Infiltration
/// 💨️ Zone infiltration specification — a full [`crate::air_exchange::InfiltrationMethod`]
/// selection plus every parameter each method needs, so the kernel maps this entity onto
/// [`crate::air_exchange::InfiltrationSpec`] one-to-one instead of pinning one method and
/// smuggling the flow through a coefficient field.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct Infiltration {
    pub id: EntityId,
    pub zone_id: EntityId,
    pub schedule_id: ScheduleId,
    pub method: air_exchange::InfiltrationMethod,
    pub design_flow_ach: f64,
    pub flow_per_exterior_area_m3_s_m2: f64,
    pub effective_leakage_area_m2: f64,
    pub discharge_coefficient: f64,
    pub stack_height_m: f64,
    pub constant_term_coefficient: f64,
    pub temperature_term_coefficient: f64,
    pub velocity_term_coefficient: f64,
    pub velocity_squared_term_coefficient: f64,
}
// #endregion 🔖️Infiltration

// #region 🔖️Model
/// 🏢️ Complete building energy model (single native representation). `run_period` and `schedules`
/// are persisted model data, not per-run session state: every `ScheduleId` reference in this
/// document resolves inside `schedules`, and [`crate::kernel::SimulationConfig`] reads both out of
/// the model at run time (ticket 26/09/06/ENERGY-PLUGIN-END-TO-END).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct Model {
    pub name: String,
    pub version: String,
    pub site: Site,
    pub zones: Vec<Zone>,
    pub spaces: Vec<Space>,
    pub surfaces: Vec<Surface>,
    pub fenestrations: Vec<Fenestration>,
    pub materials: Vec<Material>,
    pub glazing_materials: Vec<GlazingMaterial>,
    pub gas_materials: Vec<GasMaterial>,
    pub constructions: Vec<Construction>,
    pub people: Vec<PeopleGain>,
    pub lighting: Vec<LightingGain>,
    pub equipment: Vec<EquipmentGain>,
    pub thermostats: Vec<Thermostat>,
    pub humidistats: Vec<Humidistat>,
    pub setpoint_managers: Vec<SetpointManager>,
    pub ideal_loads: Vec<IdealLoadsSystem>,
    pub zone_equipment: Vec<ZoneEquipmentAssignment>,
    pub air_loops: Vec<ModelAirLoop>,
    pub plant_loops: Vec<PlantLoopConfig>,
    pub outdoor_air_systems: Vec<OutdoorAirSystem>,
    pub infiltrations: Vec<Infiltration>,
    pub mechanical_ventilations: Vec<MechanicalVentilation>,
    pub shading_surfaces: Vec<ShadingSurface>,
    pub space_lists: Vec<SpaceList>,
    pub thermal_enclosures: Vec<ThermalEnclosure>,
    pub adjacency_pairs: Vec<AdjacencyPair>,
    pub airflow_network: Option<AirflowNetworkDefinition>,
    pub electrical_load_centers: Vec<ElectricalLoadCenter>,
    pub pv_systems: Vec<PvSystemAssignment>,
    pub battery_storage: Vec<BatteryAssignment>,
    pub shw_systems: Vec<ShwSystemConfig>,
    pub solar_thermal_systems: Vec<SolarThermalConfig>,
    pub refrigeration_systems: Vec<RefrigerationConfig>,
    pub water_systems: Vec<WaterSystemConfig>,
    pub faults: Vec<FaultDefinition>,
    pub output_variables: Vec<OutputVariableSpec>,
    pub sizing_objects: Vec<SizingObject>,
    pub daylight_zones: Vec<DaylightZoneConfig>,
    pub room_air_models: Vec<RoomAirModelAssignment>,
    pub ground_temperature: GroundTemperatureConfig,
    pub run_period: calendar::RunPeriod,
    pub schedules: schedule::ScheduleSet,
}

impl Model {
    /// ✅️ Validate model topology, references, and SI ranges — a pure, allocation-only pre-flight
    /// check every caller (not only this crate's own tests) can run before handing a model to
    /// the caller-driven EnergyJob, which performs its own incremental, cursor-owned validation
    /// pass but only reports it as a job fault rather than as addressable diagnostics.
    pub fn validate(&self) -> Result<(), Diagnostics> {
        let mut diag = Diagnostics::default();
        let zone_ids: HashSet<_> = self.zones.iter().map(|z| z.id).collect();
        let surface_ids: HashSet<_> = self.surfaces.iter().map(|s| s.id).collect();
        let material_ids: HashSet<_> = self.materials.iter().map(|m| m.id).collect();
        let construction_ids: HashSet<_> = self.constructions.iter().map(|c| c.id).collect();

        if self.zones.is_empty() {
            diag.push(Error::fatal("model must contain at least one zone"));
        }

        let mut names = HashSet::new();
        for zone in &self.zones {
            if zone.volume_m3 <= 0.0 {
                diag.push(Error::severe(format!("zone {} has non-positive volume", zone.name)));
            }
            if !names.insert(zone.name.clone()) {
                diag.push(Error::severe(format!("duplicate zone name: {}", zone.name)));
            }
        }

        for space in &self.spaces {
            if !zone_ids.contains(&space.zone_id) {
                diag.push(Error::severe(format!("space {} references unknown zone", space.name)));
            }
        }

        for surface in &self.surfaces {
            if !zone_ids.contains(&surface.zone_id) {
                diag.push(Error::severe(format!("surface {} references unknown zone", surface.name)));
            }
            if !construction_ids.contains(&surface.construction_id) {
                diag.push(Error::severe(format!("surface {} references unknown construction", surface.name)));
            }
            if surface.vertices_m.len() < 3 {
                diag.push(Error::severe(format!("surface {} has fewer than 3 vertices", surface.name)));
            }
            if let OutsideBoundary::Interzone(other) = surface.outside_boundary_condition {
                if !surface_ids.contains(&other) {
                    diag.push(Error::severe(format!("surface {} interzone pair missing", surface.name)));
                }
            }
        }

        for fen in &self.fenestrations {
            if let Some(glazing) = fen.glazing_construction_id {
                if !construction_ids.contains(&glazing) {
                    diag.push(Error::severe(format!("fenestration {} references unknown glazing construction", fen.name)));
                }
            }
            if !surface_ids.contains(&fen.surface_id) {
                diag.push(Error::severe(format!("fenestration {} references unknown surface", fen.name)));
            }
        }

        let glazing_ids: HashSet<_> = self.glazing_materials.iter().map(|m| m.id).collect();
        let gas_ids: HashSet<_> = self.gas_materials.iter().map(|m| m.id).collect();
        let glazing_construction_ids: HashSet<_> = self.fenestrations.iter().filter_map(|fen| fen.glazing_construction_id).collect();
        for construction in &self.constructions {
            if construction.layer_material_ids.is_empty() {
                diag.push(Error::severe(format!("construction {} has no layers", construction.name)));
            }
            let all_glazing_layers = !construction.layer_material_ids.is_empty() && construction.layer_material_ids.iter().all(|id| glazing_ids.contains(id) || gas_ids.contains(id));
            if glazing_construction_ids.contains(&construction.id) || all_glazing_layers {
                let alternates = construction.layer_material_ids.iter().enumerate().all(|(index, id)| if index % 2 == 0 { glazing_ids.contains(id) } else { gas_ids.contains(id) });
                if !alternates || construction.layer_material_ids.len() % 2 == 0 || construction.layer_material_ids.len() > 2 * MAX_PANES - 1 {
                    diag.push(Error::severe(format!("glazing construction {} must alternate one to {} glazing panes with gas gaps", construction.name, MAX_PANES)));
                }
            } else {
                for mid in &construction.layer_material_ids {
                    if !material_ids.contains(mid) {
                        diag.push(Error::severe(format!("construction {} references unknown material", construction.name)));
                    }
                }
            }
        }

        for material in &self.materials {
            if material.thickness_m <= 0.0 || material.conductivity_w_m_k <= 0.0 {
                diag.push(Error::severe(format!("material {} has invalid thermal properties", material.name)));
            }
        }

        for glazing in &self.glazing_materials {
            let optics = [glazing.solar_transmittance, glazing.solar_reflectance_front, glazing.solar_reflectance_back, glazing.infrared_transmittance, glazing.infrared_emissivity_front, glazing.infrared_emissivity_back];
            if glazing.thickness_m <= 0.0 || glazing.conductivity_w_m_k <= 0.0 || optics.iter().any(|value| !(0.0..=1.0).contains(value)) || glazing.solar_transmittance + glazing.solar_reflectance_front > 1.0 || glazing.solar_transmittance + glazing.solar_reflectance_back > 1.0 {
                diag.push(Error::severe(format!("glazing material {} has invalid thermal or optical properties", glazing.name)));
            }
        }

        for gas in &self.gas_materials {
            if gas.thickness_m <= 0.0 {
                diag.push(Error::severe(format!("gas material {} has non-positive thickness", gas.name)));
            }
        }

        for thermostat in &self.thermostats {
            if !zone_ids.contains(&thermostat.zone_id) {
                diag.push(Error::severe("thermostat references unknown zone"));
            }
        }

        for ils in &self.ideal_loads {
            if !zone_ids.contains(&ils.zone_id) {
                diag.push(Error::severe("ideal loads system references unknown zone"));
            }
        }

        for hv in &self.humidistats {
            if !zone_ids.contains(&hv.zone_id) {
                diag.push(Error::severe("humidistat references unknown zone"));
            }
        }

        for ze in &self.zone_equipment {
            if !zone_ids.contains(&ze.zone_id) {
                diag.push(Error::severe("zone equipment references unknown zone"));
            }
        }

        for mv in &self.mechanical_ventilations {
            if !zone_ids.contains(&mv.zone_id) {
                diag.push(Error::severe("mechanical ventilation references unknown zone"));
            }
        }

        for al in &self.air_loops {
            for zid in &al.terminal_zone_ids {
                if !zone_ids.contains(zid) {
                    diag.push(Error::severe(format!("air loop {} references unknown zone", al.name)));
                }
            }
        }

        for dz in &self.daylight_zones {
            if !zone_ids.contains(&dz.zone_id) {
                diag.push(Error::severe("daylight zone references unknown zone"));
            }
        }

        for pair in &self.adjacency_pairs {
            if !surface_ids.contains(&pair.surface_a_id) || !surface_ids.contains(&pair.surface_b_id) {
                diag.push(Error::severe("adjacency pair references unknown surface"));
            }
        }

        let schedule_ids: HashSet<_> = self
            .schedules
            .constants
            .iter()
            .map(|s| s.id)
            .chain(self.schedules.daily.iter().map(|s| s.id))
            .chain(self.schedules.weekly.iter().map(|s| s.id))
            .chain(self.schedules.annual.iter().map(|s| s.id))
            .chain(self.schedules.time_series.iter().map(|s| s.id))
            .collect();
        let mut require_schedule = |id: ScheduleId, owner: &str| {
            if !schedule_ids.contains(&id) {
                diag.push(Error::severe(format!("{owner} references schedule {} which the model does not define", id.0)));
            }
        };
        for person in &self.people {
            require_schedule(person.schedule_id, "people gain");
            require_schedule(person.activity_schedule_id, "people activity");
        }
        for light in &self.lighting {
            require_schedule(light.schedule_id, "lighting gain");
        }
        for equipment in &self.equipment {
            require_schedule(equipment.schedule_id, "equipment gain");
        }
        for infiltration in &self.infiltrations {
            require_schedule(infiltration.schedule_id, "infiltration");
        }
        for ventilation in &self.mechanical_ventilations {
            require_schedule(ventilation.schedule_id, "mechanical ventilation");
        }
        for thermostat in &self.thermostats {
            require_schedule(thermostat.heating_setpoint_schedule_id, "thermostat heating setpoint");
            require_schedule(thermostat.cooling_setpoint_schedule_id, "thermostat cooling setpoint");
        }

        for fen in &self.fenestrations {
            if fen.area_m2 <= 0.0 {
                diag.push(Error::severe(format!("fenestration {} has non-positive area", fen.name)));
            }
            if fen.height_m <= 0.0 {
                diag.push(Error::severe(format!("fenestration {} has non-positive height", fen.name)));
            }
            if fen.u_value_w_m2k <= 0.0 {
                diag.push(Error::severe(format!("fenestration {} has non-positive U-value", fen.name)));
            }
            if !(0.0..=1.0).contains(&fen.shgc) {
                diag.push(Error::severe(format!("fenestration {} has an SHGC outside 0..1", fen.name)));
            }
            if !fen.vertices_m.is_empty() {
                if fen.vertices_m.len() < 3 {
                    diag.push(Error::severe(format!("fenestration {} has fewer than 3 vertices", fen.name)));
                } else if let Some(host) = self.surfaces.iter().find(|surface| surface.id == fen.surface_id) {
                    if !geometry::polygon_lies_on_plane(&fen.vertices_m, &host.vertices_m, FENESTRATION_PLANE_TOLERANCE_M) {
                        diag.push(Error::severe(format!("fenestration {} has a polygon outside its host surface's plane", fen.name)));
                    }
                }
            }
        }

        for pv in &self.pv_systems {
            if !pv.thermal_parameters_valid() {
                diag.push(Error::severe(format!("PV system {} has invalid authored thermal parameters", pv.id.0)));
            }
        }

        if diag.has_fatal() || diag.messages.iter().any(|m| m.severity == Severity::Severe) {
            Err(diag)
        } else {
            Ok(())
        }
    }

    #[cfg(test)]
    pub(crate) fn zone_by_id(&self, id: EntityId) -> Option<&Zone> {
        self.zones.iter().find(|z| z.id == id)
    }

    #[cfg(test)]
    pub(crate) fn construction_by_id(&self, id: EntityId) -> Option<&Construction> {
        self.constructions.iter().find(|c| c.id == id)
    }

    #[cfg(test)]
    pub(crate) fn material_by_id(&self, id: EntityId) -> Option<&Material> {
        self.materials.iter().find(|m| m.id == id)
    }

    #[cfg(test)]
    pub(crate) fn surfaces_for_zone(&self, zone_id: EntityId) -> Vec<&Surface> {
        self.surfaces.iter().filter(|s| s.zone_id == zone_id).collect()
    }
}
// #endregion 🔖️Model
// #region 🔖️Fixtures
/// 🧪️ Build a minimal test model for integration tests.
pub fn test_model_single_zone() -> Model {
    Model {
        name: "BESTEST Single Zone".into(),
        version: "1.0".into(),
        site: Site { latitude_deg: 45.0, longitude_deg: 0.0, elevation_m: 100.0, time_zone_hours: 0.0, north_axis_deg: 0.0 },
        zones: vec![Zone { id: EntityId(1), name: "Zone1".into(), volume_m3: 106.0, multiplier: 1, conditioned: true, part_of_total_floor_area: true }],
        materials: vec![Material {
            id: EntityId(10),
            name: "Insulation".into(),
            roughness: SurfaceRoughness::MediumRough,
            thickness_m: 0.1,
            conductivity_w_m_k: 0.04,
            density_kg_m3: 50.0,
            specific_heat_j_kg_k: 1000.0,
            thermal_absorptance: 0.9,
            solar_absorptance: 0.7,
            visible_absorptance: 0.7,
        }],
        constructions: vec![Construction { id: EntityId(20), name: "Wall".into(), layer_material_ids: vec![EntityId(10)] }],
        surfaces: vec![Surface {
            id: EntityId(30),
            name: "ExtWall".into(),
            zone_id: EntityId(1),
            class: SurfaceClass::ExteriorWall,
            vertices_m: vec![[0.0, 0.0, 0.0], [10.0, 0.0, 0.0], [10.0, 0.0, 3.0], [0.0, 0.0, 3.0]],
            construction_id: EntityId(20),
            outside_boundary_condition: OutsideBoundary::OutdoorAir,
            sun_exposed: true,
            wind_exposed: true,
            multiplier: 1,
        }],
        ideal_loads: vec![IdealLoadsSystem {
            id: EntityId(40),
            zone_id: EntityId(1),
            max_heating_supply_air_temp_c: 50.0,
            min_cooling_supply_air_temp_c: 13.0,
            max_heating_capacity_w: None,
            max_cooling_capacity_w: None,
            outdoor_air_per_person_m3_s: 0.00944,
            outdoor_air_per_area_m3_s_m2: 0.0,
        }],
        ..Default::default()
    }
}

/// 🧪️ Full topology test model with plant, PV, AFN, daylight.
pub fn test_model_full_topology() -> Model {
    let mut model = test_model_single_zone();
    model.name = "Full Topology".into();
    model.thermostats.push(Thermostat { id: EntityId(50), zone_id: EntityId(1), heating_setpoint_schedule_id: ScheduleId(1), cooling_setpoint_schedule_id: ScheduleId(1), heating_throttle_range_k: 2.0, cooling_throttle_range_k: 2.0 });
    model.plant_loops.push(PlantLoopConfig { id: EntityId(60), name: "Hot Water".into(), loop_type: PlantLoopType::Heating, supply_temperature_c: 55.0, return_temperature_c: 45.0, design_flow_kg_s: 2.0, equipment_ids: vec![EntityId(61)] });
    model.pv_systems.push(PvSystemAssignment { id: EntityId(70), dc_capacity_w: 5000.0, area_m2: 25.0, tilt_deg: 30.0, azimuth_deg: 180.0, module_efficiency: 0.2, inverter_efficiency: 0.96, temperature_coefficient: -0.004, faiman_heat_loss_constant_w_m2k: 25.0, faiman_heat_loss_wind_w_s_m3k: 6.84, mounting: crate::PhotovoltaicMounting::OpenRack});
    model.daylight_zones.push(DaylightZoneConfig { id: EntityId(80), zone_id: EntityId(1), illuminance_target_lux: 500.0, glare_limit: 0.4, window_transmittance: 0.6 });
    model
}
// #endregion 🔖️Fixtures
