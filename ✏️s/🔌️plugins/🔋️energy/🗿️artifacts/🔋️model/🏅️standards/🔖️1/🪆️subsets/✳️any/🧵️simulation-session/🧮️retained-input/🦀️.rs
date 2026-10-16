//! 🧮️ Retained input: the bounded census of the base model and its capture in grants.

use crate::kernel::SimulationConfig;
use crate::model::Model;
use super::{CAPTURE_LAST_LANE, MAXIMUM_CAPTURE_BYTES, MAXIMUM_CAPTURE_ITEMS};

//#region 🧮️RetainedInput
#[derive(Clone, Copy)]
pub(super) struct CaptureCensus {
    pub(super) lane: u8,
    pub(super) index: usize,
    pub(super) items: usize,
    pub(super) bytes: usize,
}

impl CaptureCensus {
    pub(super) fn new() -> Self {
        Self { lane: 0, index: 0, items: 2, bytes: size_of::<Model>() + size_of::<SimulationConfig>() }
    }

    pub(super) fn charge_backing(&mut self, capacity: usize, item_bytes: usize) -> Result<(), &'static str> {
        self.charge_observed(capacity, capacity.checked_mul(item_bytes).ok_or("energy.session.capture-byte-overflow")?)
    }

    fn charge_observed(&mut self, items: usize, bytes: usize) -> Result<(), &'static str> {
        self.items = self.items.checked_add(items).ok_or("energy.session.capture-item-overflow")?;
        self.bytes = self.bytes.checked_add(bytes).ok_or("energy.session.capture-byte-overflow")?;
        if self.items > MAXIMUM_CAPTURE_ITEMS || self.bytes > MAXIMUM_CAPTURE_BYTES {
            return Err("energy.session.capture-admission-exceeded");
        }
        Ok(())
    }

    fn charge(&mut self, capacity: usize, item_bytes: usize) -> Result<(), &'static str> {
        self.charge_backing(capacity, item_bytes)?;
        self.lane = self.lane.checked_add(1).ok_or("energy.session.capture-lane-overflow")?;
        self.index = 0;
        Ok(())
    }

    pub(super) fn step_one(&mut self, source: &Model) -> Result<bool, &'static str> {
        macro_rules! vector {
            ($($field:ident).+) => {{
                let (capacity, bytes) = vector_credit(&source.$($field).+);
                self.charge(capacity, bytes)?;
            }};
        }
        macro_rules! nested {
            ($($field:ident).+, $backing:expr) => {{
                if let Some(item) = source.$($field).+.get(self.index) {
                    let (items, bytes) = $backing(item);
                    self.charge_observed(items, bytes)?;
                    self.index += 1;
                } else {
                    let (capacity, bytes) = vector_credit(&source.$($field).+);
                    self.charge(capacity, bytes)?;
                }
            }};
        }
        match self.lane {
            0 => self.charge(source.name.capacity(), 1)?,
            1 => self.charge(source.version.capacity(), 1)?,
            2 => self.charge(1, size_of_val(&source.site))?,
            3 => nested!(zones, |item: &crate::model::Zone| (item.name.capacity(), item.name.capacity())),
            4 => nested!(spaces, |item: &crate::model::Space| (item.name.capacity(), item.name.capacity())),
            5 => {
                nested!(surfaces, |item: &crate::model::Surface| { (item.name.capacity().saturating_add(item.vertices_m.capacity()), item.name.capacity().saturating_add(item.vertices_m.capacity().saturating_mul(size_of::<[f64; 3]>())),) });
            }
            6 => nested!(fenestrations, |item: &crate::model::Fenestration| (item.name.capacity(), item.name.capacity())),
            7 => nested!(materials, |item: &crate::model::Material| (item.name.capacity(), item.name.capacity())),
            8 => nested!(constructions, |item: &crate::model::Construction| {
                (item.name.capacity().saturating_add(item.layer_material_ids.capacity()), item.name.capacity().saturating_add(item.layer_material_ids.capacity().saturating_mul(size_of::<crate::model::EntityId>())))
            }),
            9 => vector!(people),
            10 => vector!(lighting),
            11 => vector!(equipment),
            12 => vector!(thermostats),
            13 => vector!(humidistats),
            14 => nested!(setpoint_managers, |item: &crate::model::SetpointManager| (item.name.capacity(), item.name.capacity())),
            15 => vector!(ideal_loads),
            16 => vector!(zone_equipment),
            17 => nested!(air_loops, |item: &crate::model::ModelAirLoop| {
                (item.name.capacity().saturating_add(item.terminal_zone_ids.capacity()), item.name.capacity().saturating_add(item.terminal_zone_ids.capacity().saturating_mul(size_of::<crate::model::EntityId>())))
            }),
            18 => nested!(plant_loops, |item: &crate::model::PlantLoopConfig| {
                (item.name.capacity().saturating_add(item.equipment_ids.capacity()), item.name.capacity().saturating_add(item.equipment_ids.capacity().saturating_mul(size_of::<crate::model::EntityId>())))
            }),
            19 => vector!(outdoor_air_systems),
            20 => vector!(infiltrations),
            21 => vector!(mechanical_ventilations),
            22 => nested!(shading_surfaces, |item: &crate::model::ShadingSurface| {
                (item.name.capacity().saturating_add(item.vertices_m.capacity()), item.name.capacity().saturating_add(item.vertices_m.capacity().saturating_mul(size_of::<[f64; 3]>())))
            }),
            23 => nested!(space_lists, |item: &crate::model::SpaceList| {
                (item.name.capacity().saturating_add(item.space_ids.capacity()), item.name.capacity().saturating_add(item.space_ids.capacity().saturating_mul(size_of::<crate::model::EntityId>())))
            }),
            24 => nested!(thermal_enclosures, |item: &crate::model::ThermalEnclosure| {
                (item.name.capacity().saturating_add(item.zone_ids.capacity()), item.name.capacity().saturating_add(item.zone_ids.capacity().saturating_mul(size_of::<crate::model::EntityId>())))
            }),
            25 => vector!(adjacency_pairs),
            26 => {
                if self.index == 0 {
                    if let Some(item) = source.airflow_network.as_ref() {
                        self.charge_backing(item.zone_node_ids.capacity(), size_of::<(crate::model::EntityId, u32)>())?;
                        self.charge_backing(item.link_ids.capacity(), size_of::<u32>())?;
                    }
                    self.index = 1;
                } else {
                    self.charge(usize::from(source.airflow_network.is_some()), source.airflow_network.as_ref().map_or(0, size_of_val))?;
                }
            }
            27 => nested!(electrical_load_centers, |item: &crate::model::ElectricalLoadCenter| {
                (
                    item.name.capacity().saturating_add(item.generator_ids.capacity()).saturating_add(item.pv_ids.capacity()).saturating_add(item.battery_ids.capacity()),
                    item.name.capacity().saturating_add(item.generator_ids.capacity().saturating_add(item.pv_ids.capacity()).saturating_add(item.battery_ids.capacity()).saturating_mul(size_of::<crate::model::EntityId>())),
                )
            }),
            28 => vector!(pv_systems),
            29 => vector!(battery_storage),
            30 => vector!(shw_systems),
            31 => vector!(solar_thermal_systems),
            32 => vector!(refrigeration_systems),
            33 => vector!(water_systems),
            34 => vector!(faults),
            35 => nested!(output_variables, |item: &crate::model::OutputVariableSpec| {
                let bytes = item.name.capacity().saturating_add(item.key.capacity());
                (bytes, bytes)
            }),
            36 => vector!(sizing_objects),
            37 => vector!(daylight_zones),
            38 => vector!(room_air_models),
            39 => self.charge(1, size_of_val(&source.ground_temperature))?,
            40 => self.charge(1, size_of_val(&source.run_period))?,
            41 => vector!(schedules.constants),
            42 => vector!(schedules.daily),
            43 => vector!(schedules.weekly),
            44 => nested!(schedules.annual, |item: &crate::schedule::AnnualSchedule| {
                (item.rules.capacity().saturating_add(item.holiday_dates.capacity()), item.rules.capacity().saturating_mul(size_of::<crate::schedule::CompactScheduleRule>()).saturating_add(item.holiday_dates.capacity().saturating_mul(size_of::<(u16, u8, u8)>())))
            }),
            45 => nested!(schedules.time_series, |item: &crate::schedule::TimeSeriesSchedule| (item.values.capacity(), item.values.capacity().saturating_mul(size_of::<f64>()))),
            46 => nested!(glazing_materials, |item: &crate::model::GlazingMaterial| (item.name.capacity(), item.name.capacity())),
            47 => nested!(gas_materials, |item: &crate::model::GasMaterial| (item.name.capacity(), item.name.capacity())),
            _ => return Ok(true),
        }
        Ok(self.lane > CAPTURE_LAST_LANE)
    }
}

fn vector_credit<T>(owner: &Vec<T>) -> (usize, usize) {
    (owner.capacity(), size_of::<T>())
}

pub(super) struct ModelCapture {
    pub(super) model: Model,
    pub(super) lane: u8,
    pub(super) index: usize,
    substage: u8,
    text_byte: usize,
}

impl ModelCapture {
    pub(super) fn new() -> Self {
        Self { model: Model::default(), lane: 0, index: 0, substage: 0, text_byte: 0 }
    }

    fn next_lane(&mut self) {
        self.lane += 1;
        self.index = 0;
        self.substage = 0;
        self.text_byte = 0;
    }

    fn copy_text(target: &mut String, source: &String, byte: &mut usize) -> Result<bool, &'static str> {
        if target.capacity() == 0 && source.capacity() != 0 {
            target.try_reserve_exact(source.capacity()).map_err(|_| "energy.session.capture-string-reserve")?;
            return Ok(false);
        }
        if *byte < source.len() {
            let tail = &source[*byte..];
            let ch = tail.chars().next().ok_or("energy.session.capture-string-boundary")?;
            target.push(ch);
            *byte += ch.len_utf8();
            return Ok(false);
        }
        Ok(true)
    }

    fn copy_items<T: Copy>(target: &mut Vec<T>, source: &Vec<T>) -> Result<bool, &'static str> {
        if target.capacity() == 0 && source.capacity() != 0 {
            target.try_reserve_exact(source.capacity()).map_err(|_| "energy.session.capture-nested-vector-reserve")?;
            return Ok(false);
        }
        if let Some(item) = source.get(target.len()) {
            target.push(*item);
            return Ok(false);
        }
        Ok(true)
    }

    fn copy_rules(target: &mut Vec<crate::schedule::CompactScheduleRule>, source: &Vec<crate::schedule::CompactScheduleRule>) -> Result<bool, &'static str> {
        if target.capacity() == 0 && source.capacity() != 0 {
            target.try_reserve_exact(source.capacity()).map_err(|_| "energy.session.capture-nested-vector-reserve")?;
            return Ok(false);
        }
        if let Some(rule) = source.get(target.len()) {
            target.push(crate::schedule::CompactScheduleRule { start_month: rule.start_month, start_day: rule.start_day, end_month: rule.end_month, end_day: rule.end_day, daily_schedule_id: rule.daily_schedule_id });
            return Ok(false);
        }
        Ok(true)
    }

    pub(super) fn step_one(&mut self, source: &Model) -> Result<bool, &'static str> {
        macro_rules! plain {
            ($($field:ident).+, $value:expr) => {{
                if self.model.$($field).+.capacity() == 0 && source.$($field).+.capacity() != 0 {
                    self.model.$($field).+.try_reserve_exact(source.$($field).+.capacity()).map_err(|_| "energy.session.capture-vector-reserve")?;
                } else if let Some(item) = source.$($field).+.get(self.index) {
                    let value = $value(item);
                    self.model.$($field).+.push(value);
                    self.index += 1;
                } else {
                    self.next_lane();
                }
                return Ok(false);
            }};
        }
        macro_rules! dynamic {
            ($($field:ident).+, $source_item:ident, $target_item:ident, $empty:expr, $body:block) => {{
                if self.model.$($field).+.capacity() == 0 && source.$($field).+.capacity() != 0 {
                    self.model.$($field).+.try_reserve_exact(source.$($field).+.capacity()).map_err(|_| "energy.session.capture-vector-reserve")?;
                    return Ok(false);
                }
                let Some($source_item) = source.$($field).+.get(self.index) else {
                    self.next_lane();
                    return Ok(false);
                };
                if self.model.$($field).+.len() == self.index {
                    self.model.$($field).+.push($empty($source_item));
                    return Ok(false);
                }
                let $target_item = self.model.$($field).+.get_mut(self.index).ok_or("energy.session.capture-record-missing")?;
                $body
                return Ok(false);
            }};
        }
        macro_rules! text {
            ($target:expr, $source:expr) => {{
                if Self::copy_text($target, $source, &mut self.text_byte)? {
                    self.text_byte = 0;
                    self.substage += 1;
                }
            }};
        }
        macro_rules! items {
            ($target:expr, $source:expr) => {{
                if Self::copy_items($target, $source)? {
                    self.substage += 1;
                }
            }};
        }
        macro_rules! finish_record {
            () => {{
                self.index += 1;
                self.substage = 0;
            }};
        }
        match self.lane {
            0 => {
                if Self::copy_text(&mut self.model.name, &source.name, &mut self.text_byte)? {
                    self.next_lane();
                }
            }
            1 => {
                if Self::copy_text(&mut self.model.version, &source.version, &mut self.text_byte)? {
                    self.next_lane();
                }
            }
            2 => {
                self.model.site = source.site;
                self.next_lane();
            }
            3 => dynamic!(
                zones,
                source_item,
                target_item,
                |item: &crate::model::Zone| crate::model::Zone { id: item.id, name: String::new(), volume_m3: item.volume_m3, multiplier: item.multiplier, conditioned: item.conditioned, part_of_total_floor_area: item.part_of_total_floor_area },
                {
                    match self.substage {
                        0 => text!(&mut target_item.name, &source_item.name),
                        _ => finish_record!(),
                    }
                }
            ),
            4 => dynamic!(spaces, source_item, target_item, |item: &crate::model::Space| crate::model::Space { id: item.id, name: String::new(), zone_id: item.zone_id, floor_area_m2: item.floor_area_m2 }, {
                match self.substage {
                    0 => text!(&mut target_item.name, &source_item.name),
                    _ => finish_record!(),
                }
            }),
            5 => dynamic!(
                surfaces,
                source_item,
                target_item,
                |item: &crate::model::Surface| crate::model::Surface {
                    id: item.id,
                    name: String::new(),
                    zone_id: item.zone_id,
                    class: item.class,
                    vertices_m: Vec::new(),
                    construction_id: item.construction_id,
                    outside_boundary_condition: item.outside_boundary_condition,
                    sun_exposed: item.sun_exposed,
                    wind_exposed: item.wind_exposed,
                    multiplier: item.multiplier,
                },
                {
                    match self.substage {
                        0 => text!(&mut target_item.name, &source_item.name),
                        1 => items!(&mut target_item.vertices_m, &source_item.vertices_m),
                        _ => finish_record!(),
                    }
                }
            ),
            6 => dynamic!(
                fenestrations,
                source_item,
                target_item,
                |item: &crate::model::Fenestration| crate::model::Fenestration {
                    id: item.id,
                    name: String::new(),
                    surface_id: item.surface_id,
                    u_value_w_m2k: item.u_value_w_m2k,
                    shgc: item.shgc,
                    vlt: item.vlt,
                    area_m2: item.area_m2,
                    height_m: item.height_m,
                    sill_height_m: item.sill_height_m,
                    frame_conductance_w_k: item.frame_conductance_w_k,
                    divider_conductance_w_k: item.divider_conductance_w_k,
                    overhang_depth_m: item.overhang_depth_m,
                    overhang_offset_m: item.overhang_offset_m,
                    fin_depth_m: item.fin_depth_m,
                    fin_offset_m: item.fin_offset_m,
                    glazing_construction_id: item.glazing_construction_id,
                    vertices_m: Vec::new(),
                },
                {
                    match self.substage {
                        0 => text!(&mut target_item.name, &source_item.name),
                        // 🔶️ The aperture's own polygon is variable-length, so it copies in its own
                        // step exactly as a surface's does (lane 5, substage 1).
                        1 => items!(&mut target_item.vertices_m, &source_item.vertices_m),
                        _ => finish_record!(),
                    }
                }
            ),
            7 => dynamic!(
                materials,
                source_item,
                target_item,
                |item: &crate::model::Material| crate::model::Material {
                    id: item.id,
                    name: String::new(),
                    roughness: item.roughness,
                    thickness_m: item.thickness_m,
                    conductivity_w_m_k: item.conductivity_w_m_k,
                    density_kg_m3: item.density_kg_m3,
                    specific_heat_j_kg_k: item.specific_heat_j_kg_k,
                    thermal_absorptance: item.thermal_absorptance,
                    solar_absorptance: item.solar_absorptance,
                    visible_absorptance: item.visible_absorptance,
                },
                {
                    match self.substage {
                        0 => text!(&mut target_item.name, &source_item.name),
                        _ => finish_record!(),
                    }
                }
            ),
            8 => dynamic!(constructions, source_item, target_item, |item: &crate::model::Construction| crate::model::Construction { id: item.id, name: String::new(), layer_material_ids: Vec::new() }, {
                match self.substage {
                    0 => text!(&mut target_item.name, &source_item.name),
                    1 => items!(&mut target_item.layer_material_ids, &source_item.layer_material_ids),
                    _ => finish_record!(),
                }
            }),
            9 => plain!(people, |item: &crate::model::PeopleGain| crate::model::PeopleGain {
                id: item.id,
                zone_id: item.zone_id,
                schedule_id: item.schedule_id,
                activity_schedule_id: item.activity_schedule_id,
                people_per_area: item.people_per_area,
                sensible_fraction: item.sensible_fraction,
                latent_fraction: item.latent_fraction,
                radiant_fraction: item.radiant_fraction,
            }),
            10 => plain!(lighting, |item: &crate::model::LightingGain| crate::model::LightingGain {
                id: item.id,
                zone_id: item.zone_id,
                schedule_id: item.schedule_id,
                watts_per_area: item.watts_per_area,
                radiant_fraction: item.radiant_fraction,
                visible_fraction: item.visible_fraction,
                return_air_fraction: item.return_air_fraction,
            }),
            11 => plain!(equipment, |item: &crate::model::EquipmentGain| crate::model::EquipmentGain {
                id: item.id,
                zone_id: item.zone_id,
                schedule_id: item.schedule_id,
                watts_per_area: item.watts_per_area,
                radiant_fraction: item.radiant_fraction,
                latent_fraction: item.latent_fraction,
            }),
            12 => plain!(thermostats, |item: &crate::model::Thermostat| crate::model::Thermostat {
                id: item.id,
                zone_id: item.zone_id,
                heating_setpoint_schedule_id: item.heating_setpoint_schedule_id,
                cooling_setpoint_schedule_id: item.cooling_setpoint_schedule_id,
                heating_throttle_range_k: item.heating_throttle_range_k,
                cooling_throttle_range_k: item.cooling_throttle_range_k,
            }),
            13 => plain!(humidistats, |item: &crate::model::Humidistat| crate::model::Humidistat {
                id: item.id,
                zone_id: item.zone_id,
                humidifying_setpoint_schedule_id: item.humidifying_setpoint_schedule_id,
                dehumidifying_setpoint_schedule_id: item.dehumidifying_setpoint_schedule_id,
                humidifying_throttle_range: item.humidifying_throttle_range,
                dehumidifying_throttle_range: item.dehumidifying_throttle_range,
            }),
            14 => dynamic!(
                setpoint_managers,
                source_item,
                target_item,
                |item: &crate::model::SetpointManager| crate::model::SetpointManager {
                    id: item.id,
                    name: String::new(),
                    kind: match &item.kind {
                        crate::model::SetpointManagerKind::Scheduled => crate::model::SetpointManagerKind::Scheduled,
                        crate::model::SetpointManagerKind::OutdoorAirReset { low_outdoor_c, high_outdoor_c, low_setpoint_c, high_setpoint_c } =>
                            crate::model::SetpointManagerKind::OutdoorAirReset { low_outdoor_c: *low_outdoor_c, high_outdoor_c: *high_outdoor_c, low_setpoint_c: *low_setpoint_c, high_setpoint_c: *high_setpoint_c },
                        crate::model::SetpointManagerKind::WarmestZone => crate::model::SetpointManagerKind::WarmestZone,
                        crate::model::SetpointManagerKind::ColdestZone => crate::model::SetpointManagerKind::ColdestZone,
                    },
                    schedule_id: item.schedule_id,
                },
                {
                    match self.substage {
                        0 => text!(&mut target_item.name, &source_item.name),
                        _ => finish_record!(),
                    }
                }
            ),
            15 => plain!(ideal_loads, |item: &crate::model::IdealLoadsSystem| crate::model::IdealLoadsSystem {
                id: item.id,
                zone_id: item.zone_id,
                max_heating_supply_air_temp_c: item.max_heating_supply_air_temp_c,
                min_cooling_supply_air_temp_c: item.min_cooling_supply_air_temp_c,
                max_heating_capacity_w: item.max_heating_capacity_w,
                max_cooling_capacity_w: item.max_cooling_capacity_w,
                outdoor_air_per_person_m3_s: item.outdoor_air_per_person_m3_s,
                outdoor_air_per_area_m3_s_m2: item.outdoor_air_per_area_m3_s_m2,
            }),
            16 => plain!(zone_equipment, |item: &crate::model::ZoneEquipmentAssignment| crate::model::ZoneEquipmentAssignment {
                id: item.id,
                zone_id: item.zone_id,
                equipment_type: match &item.equipment_type {
                    crate::model::ZoneEquipmentType::Baseboard => crate::model::ZoneEquipmentType::Baseboard,
                    crate::model::ZoneEquipmentType::Radiant => crate::model::ZoneEquipmentType::Radiant,
                    crate::model::ZoneEquipmentType::FanCoil => crate::model::ZoneEquipmentType::FanCoil,
                    crate::model::ZoneEquipmentType::Ptac => crate::model::ZoneEquipmentType::Ptac,
                    crate::model::ZoneEquipmentType::VrfTerminal => crate::model::ZoneEquipmentType::VrfTerminal,
                    crate::model::ZoneEquipmentType::Erv => crate::model::ZoneEquipmentType::Erv,
                    crate::model::ZoneEquipmentType::UnitHeater => crate::model::ZoneEquipmentType::UnitHeater,
                    crate::model::ZoneEquipmentType::WaterToAirHp => crate::model::ZoneEquipmentType::WaterToAirHp,
                },
                priority: item.priority,
                heating_capacity_w: item.heating_capacity_w,
                cooling_capacity_w: item.cooling_capacity_w,
            }),
            17 => dynamic!(
                air_loops,
                source_item,
                target_item,
                |item: &crate::model::ModelAirLoop| crate::model::ModelAirLoop {
                    id: item.id,
                    name: String::new(),
                    supply_node_id: item.supply_node_id,
                    return_node_id: item.return_node_id,
                    design_supply_air_flow_m3_s: item.design_supply_air_flow_m3_s,
                    terminal_zone_ids: Vec::new(),
                },
                {
                    match self.substage {
                        0 => text!(&mut target_item.name, &source_item.name),
                        1 => items!(&mut target_item.terminal_zone_ids, &source_item.terminal_zone_ids),
                        _ => finish_record!(),
                    }
                }
            ),
            18 => dynamic!(
                plant_loops,
                source_item,
                target_item,
                |item: &crate::model::PlantLoopConfig| crate::model::PlantLoopConfig {
                    id: item.id,
                    name: String::new(),
                    loop_type: item.loop_type,
                    supply_temperature_c: item.supply_temperature_c,
                    return_temperature_c: item.return_temperature_c,
                    design_flow_kg_s: item.design_flow_kg_s,
                    equipment_ids: Vec::new(),
                },
                {
                    match self.substage {
                        0 => text!(&mut target_item.name, &source_item.name),
                        1 => items!(&mut target_item.equipment_ids, &source_item.equipment_ids),
                        _ => finish_record!(),
                    }
                }
            ),
            19 => {
                plain!(outdoor_air_systems, |item: &crate::model::OutdoorAirSystem| crate::model::OutdoorAirSystem { id: item.id, air_loop_id: item.air_loop_id, min_oa_flow_m3_s: item.min_oa_flow_m3_s, economizer_enabled: item.economizer_enabled })
            }
            20 => plain!(infiltrations, |item: &crate::model::Infiltration| crate::model::Infiltration {
                id: item.id,
                zone_id: item.zone_id,
                schedule_id: item.schedule_id,
                method: item.method,
                design_flow_ach: item.design_flow_ach,
                flow_per_exterior_area_m3_s_m2: item.flow_per_exterior_area_m3_s_m2,
                effective_leakage_area_m2: item.effective_leakage_area_m2,
                discharge_coefficient: item.discharge_coefficient,
                stack_height_m: item.stack_height_m,
                constant_term_coefficient: item.constant_term_coefficient,
                temperature_term_coefficient: item.temperature_term_coefficient,
                velocity_term_coefficient: item.velocity_term_coefficient,
                velocity_squared_term_coefficient: item.velocity_squared_term_coefficient
            }),
            21 => plain!(mechanical_ventilations, |item: &crate::model::MechanicalVentilation| crate::model::MechanicalVentilation {
                id: item.id,
                zone_id: item.zone_id,
                schedule_id: item.schedule_id,
                design_flow_m3_s: item.design_flow_m3_s,
                fan_total_efficiency: item.fan_total_efficiency,
                fan_delta_pressure_pa: item.fan_delta_pressure_pa
            }),
            22 => dynamic!(
                shading_surfaces,
                source_item,
                target_item,
                |item: &crate::model::ShadingSurface| crate::model::ShadingSurface { id: item.id, name: String::new(), vertices_m: Vec::new(), transmittance_schedule_id: item.transmittance_schedule_id },
                {
                    match self.substage {
                        0 => text!(&mut target_item.name, &source_item.name),
                        1 => items!(&mut target_item.vertices_m, &source_item.vertices_m),
                        _ => finish_record!(),
                    }
                }
            ),
            23 => dynamic!(space_lists, source_item, target_item, |item: &crate::model::SpaceList| crate::model::SpaceList { id: item.id, name: String::new(), space_ids: Vec::new() }, {
                match self.substage {
                    0 => text!(&mut target_item.name, &source_item.name),
                    1 => items!(&mut target_item.space_ids, &source_item.space_ids),
                    _ => finish_record!(),
                }
            }),
            24 => dynamic!(thermal_enclosures, source_item, target_item, |item: &crate::model::ThermalEnclosure| crate::model::ThermalEnclosure { id: item.id, name: String::new(), zone_ids: Vec::new() }, {
                match self.substage {
                    0 => text!(&mut target_item.name, &source_item.name),
                    1 => items!(&mut target_item.zone_ids, &source_item.zone_ids),
                    _ => finish_record!(),
                }
            }),
            25 => plain!(adjacency_pairs, |item: &crate::model::AdjacencyPair| crate::model::AdjacencyPair { surface_a_id: item.surface_a_id, surface_b_id: item.surface_b_id }),
            26 => {
                let Some(source_item) = source.airflow_network.as_ref() else {
                    self.next_lane();
                    return Ok(false);
                };
                if self.model.airflow_network.is_none() {
                    self.model.airflow_network = Some(crate::model::AirflowNetworkDefinition { zone_node_ids: Vec::new(), outdoor_node_id: source_item.outdoor_node_id, link_ids: Vec::new() });
                    return Ok(false);
                }
                let target_item = self.model.airflow_network.as_mut().expect("mounted airflow capture");
                match self.substage {
                    0 => items!(&mut target_item.zone_node_ids, &source_item.zone_node_ids),
                    1 => items!(&mut target_item.link_ids, &source_item.link_ids),
                    _ => self.next_lane(),
                }
            }
            27 => dynamic!(
                electrical_load_centers,
                source_item,
                target_item,
                |item: &crate::model::ElectricalLoadCenter| crate::model::ElectricalLoadCenter { id: item.id, name: String::new(), generator_ids: Vec::new(), pv_ids: Vec::new(), battery_ids: Vec::new() },
                {
                    match self.substage {
                        0 => text!(&mut target_item.name, &source_item.name),
                        1 => items!(&mut target_item.generator_ids, &source_item.generator_ids),
                        2 => items!(&mut target_item.pv_ids, &source_item.pv_ids),
                        3 => items!(&mut target_item.battery_ids, &source_item.battery_ids),
                        _ => finish_record!(),
                    }
                }
            ),
            28 => plain!(pv_systems, |item: &crate::model::PvSystemAssignment| crate::model::PvSystemAssignment {
                id: item.id,
                dc_capacity_w: item.dc_capacity_w,
                area_m2: item.area_m2,
                tilt_deg: item.tilt_deg,
                azimuth_deg: item.azimuth_deg,
                module_efficiency: item.module_efficiency,
                inverter_efficiency: item.inverter_efficiency
            }),
            29 => plain!(battery_storage, |item: &crate::model::BatteryAssignment| crate::model::BatteryAssignment {
                id: item.id,
                capacity_kwh: item.capacity_kwh,
                max_charge_w: item.max_charge_w,
                max_discharge_w: item.max_discharge_w,
                round_trip_efficiency: item.round_trip_efficiency
            }),
            30 => plain!(shw_systems, |item: &crate::model::ShwSystemConfig| crate::model::ShwSystemConfig {
                id: item.id,
                heater_capacity_w: item.heater_capacity_w,
                storage_volume_m3: item.storage_volume_m3,
                setpoint_c: item.setpoint_c,
                schedule_id: item.schedule_id
            }),
            31 => plain!(solar_thermal_systems, |item: &crate::model::SolarThermalConfig| crate::model::SolarThermalConfig {
                id: item.id,
                collector_area_m2: item.collector_area_m2,
                efficiency: item.efficiency,
                storage_volume_m3: item.storage_volume_m3,
                tilt_deg: item.tilt_deg,
                azimuth_deg: item.azimuth_deg
            }),
            32 => {
                plain!(refrigeration_systems, |item: &crate::model::RefrigerationConfig| crate::model::RefrigerationConfig { id: item.id, case_count: item.case_count, design_load_w: item.design_load_w, defrost_schedule_id: item.defrost_schedule_id })
            }
            33 => plain!(water_systems, |item: &crate::model::WaterSystemConfig| crate::model::WaterSystemConfig { id: item.id, fixture_count: item.fixture_count, peak_flow_l_s: item.peak_flow_l_s, schedule_id: item.schedule_id }),
            34 => plain!(faults, |item: &crate::model::FaultDefinition| crate::model::FaultDefinition {
                id: item.id,
                target_equipment_id: item.target_equipment_id,
                fault_type: item.fault_type,
                severity: item.severity,
                start_schedule_id: item.start_schedule_id
            }),
            35 => dynamic!(output_variables, source_item, target_item, |item: &crate::model::OutputVariableSpec| crate::model::OutputVariableSpec { name: String::new(), key: String::new(), reporting_frequency: item.reporting_frequency }, {
                match self.substage {
                    0 => text!(&mut target_item.name, &source_item.name),
                    1 => text!(&mut target_item.key, &source_item.key),
                    _ => finish_record!(),
                }
            }),
            36 => plain!(sizing_objects, |item: &crate::model::SizingObject| crate::model::SizingObject { id: item.id, zone_id: item.zone_id, sizing_type: item.sizing_type, design_day_type: item.design_day_type }),
            37 => plain!(daylight_zones, |item: &crate::model::DaylightZoneConfig| crate::model::DaylightZoneConfig {
                id: item.id,
                zone_id: item.zone_id,
                illuminance_target_lux: item.illuminance_target_lux,
                glare_limit: item.glare_limit,
                window_transmittance: item.window_transmittance
            }),
            38 => plain!(room_air_models, |item: &crate::model::RoomAirModelAssignment| crate::model::RoomAirModelAssignment { zone_id: item.zone_id, model: item.model }),
            39 => {
                self.model.ground_temperature = crate::model::GroundTemperatureConfig { building_surface_c: source.ground_temperature.building_surface_c, shallow_c: source.ground_temperature.shallow_c, deep_c: source.ground_temperature.deep_c };
                self.next_lane();
            }
            40 => {
                self.model.run_period = source.run_period;
                self.next_lane();
            }
            41 => plain!(schedules.constants, |item: &crate::schedule::ConstantSchedule| crate::schedule::ConstantSchedule { id: item.id, value: item.value }),
            42 => plain!(schedules.daily, |item: &crate::schedule::DailySchedule| crate::schedule::DailySchedule { id: item.id, hourly_values: item.hourly_values, interpolation: item.interpolation, limits: item.limits }),
            43 => plain!(schedules.weekly, |item: &crate::schedule::WeeklySchedule| crate::schedule::WeeklySchedule { id: item.id, daily_schedule_ids: item.daily_schedule_ids }),
            44 => dynamic!(
                schedules.annual,
                source_item,
                target_item,
                |item: &crate::schedule::AnnualSchedule| crate::schedule::AnnualSchedule { id: item.id, rules: Vec::new(), default_daily_schedule_id: item.default_daily_schedule_id, holiday_daily_schedule_id: item.holiday_daily_schedule_id, holiday_dates: Vec::new() },
                {
                    match self.substage {
                        0 => {
                            if Self::copy_rules(&mut target_item.rules, &source_item.rules)? {
                                self.substage += 1;
                            }
                        }
                        1 => items!(&mut target_item.holiday_dates, &source_item.holiday_dates),
                        _ => finish_record!(),
                    }
                }
            ),
            45 => dynamic!(schedules.time_series, source_item, target_item, |item: &crate::schedule::TimeSeriesSchedule| crate::schedule::TimeSeriesSchedule { id: item.id, values: Vec::new(), timestep_seconds: item.timestep_seconds }, {
                match self.substage {
                    0 => items!(&mut target_item.values, &source_item.values),
                    _ => finish_record!(),
                }
            }),
            46 => dynamic!(glazing_materials, source_item, target_item, |item: &crate::model::GlazingMaterial| crate::model::GlazingMaterial { id: item.id, name: String::new(), thickness_m: item.thickness_m, conductivity_w_m_k: item.conductivity_w_m_k, solar_transmittance: item.solar_transmittance, solar_reflectance_front: item.solar_reflectance_front, solar_reflectance_back: item.solar_reflectance_back, visible_transmittance: item.visible_transmittance, visible_reflectance_front: item.visible_reflectance_front, visible_reflectance_back: item.visible_reflectance_back, infrared_transmittance: item.infrared_transmittance, infrared_emissivity_front: item.infrared_emissivity_front, infrared_emissivity_back: item.infrared_emissivity_back }, {
                match self.substage {
                    0 => text!(&mut target_item.name, &source_item.name),
                    _ => finish_record!(),
                }
            }),
            47 => dynamic!(gas_materials, source_item, target_item, |item: &crate::model::GasMaterial| crate::model::GasMaterial { id: item.id, name: String::new(), thickness_m: item.thickness_m, gas: item.gas }, {
                match self.substage {
                    0 => text!(&mut target_item.name, &source_item.name),
                    _ => finish_record!(),
                }
            }),
            _ => return Ok(true),
        }
        Ok(self.lane > CAPTURE_LAST_LANE)
    }

    pub(super) fn finish(self) -> Model {
        self.model
    }
}
//#endregion 🧮️RetainedInput
