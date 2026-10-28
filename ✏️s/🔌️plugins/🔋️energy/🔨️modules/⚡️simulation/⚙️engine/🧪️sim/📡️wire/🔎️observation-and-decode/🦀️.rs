//! 🔎️ The observed working-set sizes, the stage and tier code tables, the preview decoder and the packet close.

use super::*;

pub fn observed_weather_records(config: &SimulationConfig) -> usize {
    config.weather.as_ref().map_or_else(
        || SimulationKernel::run_period(config).total_hours().max(24) as usize,
        |weather| weather.records.capacity().max(weather.records.len().max(1)),
    )
}

pub fn weather_copy_target(config: &SimulationConfig) -> usize {
    config.weather.as_ref().map_or_else(
        || SimulationKernel::run_period(config).total_hours().max(24) as usize,
        |weather| weather.records.len(),
    )
}

pub fn checked_sum(values: impl IntoIterator<Item = usize>) -> Option<usize> {
    values.into_iter().try_fold(0usize, usize::checked_add)
}

pub fn observed_identifier_bytes(model: &Model, config: &SimulationConfig) -> Option<usize> {
    let mut bytes = model.name.capacity().checked_add(model.version.capacity())?;
    macro_rules! names {
        ($owners:expr) => {
            for owner in $owners {
                bytes = bytes.checked_add(owner.name.capacity())?;
            }
        };
    }
    names!(&model.zones);
    names!(&model.spaces);
    names!(&model.surfaces);
    names!(&model.fenestrations);
    names!(&model.materials);
    names!(&model.glazing_materials);
    names!(&model.gas_materials);
    names!(&model.constructions);
    names!(&model.setpoint_managers);
    names!(&model.air_loops);
    names!(&model.plant_loops);
    names!(&model.shading_surfaces);
    names!(&model.space_lists);
    names!(&model.thermal_enclosures);
    names!(&model.electrical_load_centers);
    for output in &model.output_variables {
        bytes = bytes.checked_add(output.name.capacity())?.checked_add(output.key.capacity())?;
    }
    if let Some(weather) = &config.weather {
        bytes = bytes.checked_add(weather.location.capacity())?;
    }
    for zone in &model.zones {
        let temperature = "Zone Air Temperature [".len().checked_add(zone.name.len())?.checked_add(1)?;
        let heating_power = "Zone Heating Power [".len().checked_add(zone.name.len())?.checked_add(1)?;
        let cooling_power = "Zone Cooling Power [".len().checked_add(zone.name.len())?.checked_add(1)?;
        let heating = zone.name.len().checked_add(" Heating".len())?;
        let cooling = zone.name.len().checked_add(" Cooling".len())?;
        let fans = zone.name.len().checked_add(" Fans".len())?;
        bytes = bytes.checked_add(temperature.checked_add(heating_power)?.checked_add(cooling_power)?.checked_mul(3)?)?;
        bytes = bytes.checked_add(heating.checked_add(cooling)?.checked_add(fans)?.checked_mul(3)?)?;
    }
    bytes = bytes.checked_add(("Facility Heating".len() + "Facility PV".len()).checked_mul(3)?)?;
    bytes = bytes.checked_add(model.name.len())?.checked_add(model.version.len())?;
    if let Some(weather) = &config.weather {
        bytes = bytes.checked_add(weather.location.len())?;
    }
    for (key, unit) in [("Electricity", "kWh"), ("Natural Gas", "kWh"), ("Energy Use Intensity", "kWh/m²"), ("Annual Energy Cost", "USD"), ("LCCA Present Value", "USD")] {
        bytes = bytes.checked_add(key.len())?.checked_add(unit.len())?;
    }
    Some(bytes)
}

pub fn observed_model_bytes(model: &Model, config: &SimulationConfig) -> Option<usize> {
    let mut bytes = 0usize;
    macro_rules! backing {
        ($owners:expr) => {
            bytes = bytes.checked_add(observed_vector_bytes(&$owners)?)?;
        };
    }
    backing!(model.zones);
    backing!(model.spaces);
    backing!(model.surfaces);
    backing!(model.fenestrations);
    backing!(model.materials);
    backing!(model.glazing_materials);
    backing!(model.gas_materials);
    backing!(model.constructions);
    backing!(model.people);
    backing!(model.lighting);
    backing!(model.equipment);
    backing!(model.thermostats);
    backing!(model.humidistats);
    backing!(model.setpoint_managers);
    backing!(model.ideal_loads);
    backing!(model.zone_equipment);
    backing!(model.air_loops);
    backing!(model.plant_loops);
    backing!(model.outdoor_air_systems);
    backing!(model.infiltrations);
    backing!(model.mechanical_ventilations);
    backing!(model.shading_surfaces);
    backing!(model.space_lists);
    backing!(model.thermal_enclosures);
    backing!(model.adjacency_pairs);
    backing!(model.electrical_load_centers);
    backing!(model.pv_systems);
    backing!(model.battery_storage);
    backing!(model.shw_systems);
    backing!(model.solar_thermal_systems);
    backing!(model.refrigeration_systems);
    backing!(model.water_systems);
    backing!(model.faults);
    backing!(model.output_variables);
    backing!(model.sizing_objects);
    backing!(model.daylight_zones);
    backing!(model.room_air_models);
    for surface in &model.surfaces {
        bytes = bytes.checked_add(surface.vertices_m.capacity().checked_mul(size_of::<[f64; 3]>())?)?;
    }
    for construction in &model.constructions {
        bytes = bytes.checked_add(construction.layer_material_ids.capacity().checked_mul(size_of::<model::EntityId>())?)?;
    }
    for plant in &model.plant_loops {
        bytes = bytes.checked_add(plant.equipment_ids.capacity().checked_mul(size_of::<model::EntityId>())?)?;
    }
    for air_loop in &model.air_loops {
        bytes = bytes.checked_add(observed_vector_bytes(&air_loop.terminal_zone_ids)?)?;
    }
    for shading in &model.shading_surfaces {
        bytes = bytes.checked_add(observed_vector_bytes(&shading.vertices_m)?)?;
    }
    for list in &model.space_lists {
        bytes = bytes.checked_add(observed_vector_bytes(&list.space_ids)?)?;
    }
    for enclosure in &model.thermal_enclosures {
        bytes = bytes.checked_add(observed_vector_bytes(&enclosure.zone_ids)?)?;
    }
    for center in &model.electrical_load_centers {
        bytes = bytes.checked_add(observed_vector_bytes(&center.generator_ids)?)?;
        bytes = bytes.checked_add(observed_vector_bytes(&center.pv_ids)?)?;
        bytes = bytes.checked_add(observed_vector_bytes(&center.battery_ids)?)?;
    }
    if let Some(network) = &model.airflow_network {
        bytes = bytes.checked_add(network.zone_node_ids.capacity().checked_mul(size_of::<(model::EntityId, u32)>())?)?;
        bytes = bytes.checked_add(network.link_ids.capacity().checked_mul(size_of::<u32>())?)?;
    }
    if let Some(weather) = &config.weather {
        bytes = bytes.checked_add(weather.records.capacity().checked_mul(size_of::<WeatherRecord>())?)?;
    }
    backing!(config.schedules.constants);
    backing!(config.schedules.daily);
    backing!(config.schedules.weekly);
    backing!(config.schedules.annual);
    backing!(config.schedules.time_series);
    for schedule in &config.schedules.annual {
        bytes = bytes.checked_add(observed_vector_bytes(&schedule.rules)?)?;
        bytes = bytes.checked_add(observed_vector_bytes(&schedule.holiday_dates)?)?;
    }
    for schedule in &config.schedules.time_series {
        bytes = bytes.checked_add(observed_vector_bytes(&schedule.values)?)?;
    }
    Some(bytes)
}

pub fn observed_vector_bytes<T>(owners: &Vec<T>) -> Option<usize> {
    owners.capacity().checked_mul(size_of::<T>())
}

impl EnergyJobStage {
    pub fn label(self) -> &'static str {
        match self {
            EnergyJobStage::Validate => "energy.validate",
            EnergyJobStage::ResolveWeather => "energy.resolve-weather",
            EnergyJobStage::Precompute => "energy.precompute",
            EnergyJobStage::InitializeZones => "energy.initialize-zones",
            EnergyJobStage::InitializeSurfaces => "energy.initialize-surfaces",
            EnergyJobStage::InitializeWarmupHistory => "energy.initialize-warmup-history",
            EnergyJobStage::WarmupTimestep => "energy.warmup-timestep",
            EnergyJobStage::WarmupConvergence => "energy.warmup-convergence",
            EnergyJobStage::StartRun => "energy.start-run",
            EnergyJobStage::RunZoneTimestep => "energy.run-zone-timestep",
            EnergyJobStage::AggregateZone => "energy.aggregate-zone",
            EnergyJobStage::AggregateFacility => "energy.aggregate-facility",
            EnergyJobStage::PublishTimestep => "energy.publish-timestep",
            EnergyJobStage::Finalize => "energy.finalize",
            EnergyJobStage::Size => "energy.size",
            EnergyJobStage::FinalizeSummaries => "energy.finalize-summaries",
            EnergyJobStage::FinalizeMetrics => "energy.finalize-metrics",
            EnergyJobStage::FinalizeEconomics => "energy.finalize-economics",
            EnergyJobStage::BuildResults => "energy.build-results",
            EnergyJobStage::PublishFinal => "energy.publish-final",
            EnergyJobStage::EncodeOutput => "energy.encode-output",
            EnergyJobStage::Complete => "energy.complete",
        }
    }

    pub fn code(self) -> u8 {
        self as u8
    }
}

impl EnergyQualityTier {
    pub fn code(self) -> u8 {
        self as u8
    }
}

pub fn energy_stage_from_code(code: u8) -> Option<EnergyJobStage> {
    const STAGES: [EnergyJobStage; 22] = [
        EnergyJobStage::Validate,
        EnergyJobStage::ResolveWeather,
        EnergyJobStage::Precompute,
        EnergyJobStage::InitializeZones,
        EnergyJobStage::InitializeSurfaces,
        EnergyJobStage::InitializeWarmupHistory,
        EnergyJobStage::WarmupTimestep,
        EnergyJobStage::WarmupConvergence,
        EnergyJobStage::StartRun,
        EnergyJobStage::RunZoneTimestep,
        EnergyJobStage::AggregateZone,
        EnergyJobStage::AggregateFacility,
        EnergyJobStage::PublishTimestep,
        EnergyJobStage::Finalize,
        EnergyJobStage::Size,
        EnergyJobStage::FinalizeSummaries,
        EnergyJobStage::FinalizeMetrics,
        EnergyJobStage::FinalizeEconomics,
        EnergyJobStage::BuildResults,
        EnergyJobStage::PublishFinal,
        EnergyJobStage::EncodeOutput,
        EnergyJobStage::Complete,
    ];
    STAGES.get(code as usize).copied()
}

pub fn energy_tier_from_code(code: u8) -> Option<EnergyQualityTier> {
    const TIERS: [EnergyQualityTier; 4] = [EnergyQualityTier::SteadyStateEstimate, EnergyQualityTier::DesignDay, EnergyQualityTier::CoarseTimestep, EnergyQualityTier::Final];
    TIERS.get(code as usize).copied()
}

pub fn decode_preview_packet(packet: &EnergyWirePacket) -> Option<EnergyJobPreview> {
    let page = packet.payload.single_page()?;
    if packet.kind != EnergyWireKind::Preview
        || page.len() != ENERGY_WIRE_HEADER_BYTES + 20
        || page[..8] != ENERGY_WIRE_MAGIC
        || u16::from_le_bytes(page[8..10].try_into().ok()?) != ENERGY_WIRE_VERSION
        || page[10] != EnergyWireKind::Preview as u8
        || u64::from_le_bytes(page[16..24].try_into().ok()?) != packet.identity.operation
        || u64::from_le_bytes(page[24..32].try_into().ok()?) != packet.identity.base_revision
        || u64::from_le_bytes(page[32..40].try_into().ok()?) != packet.identity.generation
        || u64::from_le_bytes(page[40..48].try_into().ok()?) != packet.identity.seed
        || u64::from_le_bytes(page[48..56].try_into().ok()?) != packet.identity.sequence
    {
        return None;
    }
    Some(EnergyJobPreview {
        sequence: packet.identity.sequence,
        tier: energy_tier_from_code(page[12])?,
        stage: energy_stage_from_code(page[11])?,
        warmup_hour: u32::from_le_bytes(page[80..84].try_into().ok()?),
        timestep: u32::from_le_bytes(page[84..88].try_into().ok()?),
        total_timesteps: u32::from_le_bytes(page[88..92].try_into().ok()?),
        facility_electricity_kwh: f64::from_le_bytes(page[92..100].try_into().ok()?),
    })
}

pub fn fuel_code(fuel: FuelType) -> u8 {
    match fuel {
        FuelType::Electricity => 0,
        FuelType::NaturalGas => 1,
        FuelType::Propane => 2,
        FuelType::FuelOil => 3,
        FuelType::DistrictHeating => 4,
        FuelType::DistrictCooling => 5,
        FuelType::Steam => 6,
        FuelType::Water => 7,
        FuelType::OnSiteGeneration => 8,
    }
}

pub fn end_use_code(end_use: EndUse) -> u32 {
    match end_use {
        EndUse::Heating => 0,
        EndUse::Cooling => 1,
        EndUse::InteriorLighting => 2,
        EndUse::ExteriorLighting => 3,
        EndUse::InteriorEquipment => 4,
        EndUse::ExteriorEquipment => 5,
        EndUse::Fans => 6,
        EndUse::Pumps => 7,
        EndUse::HeatRejection => 8,
        EndUse::Humidification => 9,
        EndUse::Dehumidification => 10,
        EndUse::WaterSystems => 11,
        EndUse::Refrigeration => 12,
        EndUse::Generators => 13,
        EndUse::Custom(value) => 0x8000_0000 | value,
    }
}

pub fn unit_code(unit: Unit) -> u8 {
    match unit {
        Unit::Dimensionless => 0,
        Unit::Meters => 1,
        Unit::SquareMeters => 2,
        Unit::CubicMeters => 3,
        Unit::Kelvin => 4,
        Unit::Celsius => 5,
        Unit::Pascals => 6,
        Unit::Watts => 7,
        Unit::Joules => 8,
        Unit::KilogramsPerSecond => 9,
        Unit::CubicMetersPerSecond => 10,
        Unit::KilowattHours => 11,
        Unit::HumidityRatio => 12,
        Unit::Percent => 13,
    }
}

/// ♻️ Releases one locally owned packet turn by turn with grants it quotes itself.
pub fn close_retained_packet(packet: &mut EnergyWirePacket) {
    while !packet.terminal_is_empty() {
        let grant = energy_self_funded(packet.retirement_demands().expect("a locally owned packet quotes its close demand"));
        energy_turn(packet.close_step(grant)).expect("a locally owned packet close is never refused");
    }
}
