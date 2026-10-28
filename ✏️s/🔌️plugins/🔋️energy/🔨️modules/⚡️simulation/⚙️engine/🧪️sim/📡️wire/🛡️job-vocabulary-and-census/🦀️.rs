//! 🛡️ The quality tier, job stage, preview and cursor vocabulary and the numerical admission census with its bounds.

use super::*;

/// ⚡️ Fidelity label carried by every preview so provisional fields cannot be mistaken for a
/// validated final simulation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub enum EnergyQualityTier {
    SteadyStateEstimate,
    DesignDay,
    CoarseTimestep,
    Final,
}

/// 🧭️ Persistent stage of [`EnergyJob`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub enum EnergyJobStage {
    Validate,
    ResolveWeather,
    Precompute,
    InitializeZones,
    InitializeSurfaces,
    InitializeWarmupHistory,
    WarmupTimestep,
    WarmupConvergence,
    StartRun,
    RunZoneTimestep,
    AggregateZone,
    AggregateFacility,
    PublishTimestep,
    Finalize,
    Size,
    FinalizeSummaries,
    FinalizeMetrics,
    FinalizeEconomics,
    BuildResults,
    PublishFinal,
    EncodeOutput,
    Complete,
}

/// 📸️ Typed view of the latest replaceable energy preview.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub struct EnergyJobPreview {
    pub sequence: u64,
    pub tier: EnergyQualityTier,
    pub stage: EnergyJobStage,
    pub warmup_hour: u32,
    pub timestep: u32,
    pub total_timesteps: u32,
    pub facility_electricity_kwh: f64,
}

/// 🧭️ Live numerical cursor of [`EnergyJob`]: what a host projects into run progress without decoding a
/// retained preview packet. `timestep` counts completed run timesteps, `warmup_hour` the warmup cursor.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EnergyJobCursor {
    pub input_identity_bytes: usize,
    pub input_identity_ready: bool,
    pub stage: EnergyJobStage,
    pub tier: EnergyQualityTier,
    pub warmup_hour: u32,
    pub warmup_hours: u32,
    pub timestep: u32,
    pub total_timesteps: u32,
    pub facility_electricity_kwh: f64,
}

/// 📏️ Schema-first simultaneous working-set census for an Energy numerical job.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub struct EnergyNumericalCensus {
    pub zones: usize,
    pub surfaces: usize,
    pub fenestrations: usize,
    pub people: usize,
    pub lighting: usize,
    pub equipment: usize,
    pub infiltrations: usize,
    pub airflow_nodes: usize,
    pub airflow_links: usize,
    pub mechanical_ventilations: usize,
    pub thermostats: usize,
    pub humidistats: usize,
    pub ideal_loads: usize,
    pub faults: usize,
    pub zone_equipment: usize,
    pub plant_loops: usize,
    pub plant_equipment: usize,
    pub pv_systems: usize,
    pub batteries: usize,
    pub service_hot_water: usize,
    pub refrigeration: usize,
    pub water: usize,
    pub weather_records: usize,
    pub timesteps: usize,
    pub meters: usize,
    pub series: usize,
    pub samples: usize,
    pub history_values: usize,
    pub summary_rows: usize,
    pub identifier_bytes: usize,
    pub observed_items: usize,
    pub observed_bytes: usize,
    pub pages: usize,
    pub operations: usize,
    pub process_jobs: usize,
}

/// 🚧️ The independent admitted dimension that rejected an exact owner graph.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnergyNumericalDimension {
    Zones,
    Surfaces,
    Fenestrations,
    People,
    Lighting,
    Equipment,
    Infiltrations,
    AirflowNodes,
    AirflowLinks,
    MechanicalVentilations,
    Thermostats,
    Humidistats,
    IdealLoads,
    Faults,
    ZoneEquipment,
    PlantLoops,
    PlantEquipment,
    PvSystems,
    Batteries,
    ServiceHotWater,
    Refrigeration,
    Water,
    WeatherRecords,
    Timesteps,
    Meters,
    Series,
    Samples,
    HistoryValues,
    SummaryRows,
    IdentifierBytes,
    ObservedItems,
    ObservedBytes,
    Pages,
    Operations,
    ProcessJobs,
    ArithmeticOverflow,
}

/// 🛡️ Checked maxima for every independent numerical dimension.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnergyNumericalBounds(pub EnergyNumericalCensus);

impl Default for EnergyNumericalBounds {
    fn default() -> Self {
        Self(EnergyNumericalCensus {
            zones: 65_536,
            surfaces: 1_048_576,
            fenestrations: 1_048_576,
            people: 1_048_576,
            lighting: 1_048_576,
            equipment: 1_048_576,
            infiltrations: 1_048_576,
            airflow_nodes: 1_048_576,
            airflow_links: 1_048_576,
            mechanical_ventilations: 1_048_576,
            thermostats: 65_536,
            humidistats: 65_536,
            ideal_loads: 1_048_576,
            faults: 1_048_576,
            zone_equipment: 1_048_576,
            plant_loops: 65_536,
            plant_equipment: 1_048_576,
            pv_systems: 1_048_576,
            batteries: 1_048_576,
            service_hot_water: 1_048_576,
            refrigeration: 1_048_576,
            water: 1_048_576,
            weather_records: 1_048_576,
            timesteps: 1_048_576,
            meters: 1_048_576,
            series: 65_536,
            samples: 67_108_864,
            history_values: 67_108_864,
            summary_rows: 1_048_576,
            identifier_bytes: 268_435_456,
            observed_items: 134_217_728,
            observed_bytes: 2_147_483_648,
            pages: 131_072,
            operations: 1,
            process_jobs: 1,
        })
    }
}

impl EnergyNumericalCensus {
    pub fn observe(model: &Model, config: &SimulationConfig) -> Option<Self> {
        Self::from_observations(model,config,checked_sum(model.plant_loops.iter().map(|plant|plant.equipment_ids.capacity()))?,observed_identifier_bytes(model,config)?,observed_model_bytes(model,config)?)
    }

    /// 🧮️ Combines original scalar headers with the completed incremental metadata census.
    pub fn from_observations(model:&Model,config:&SimulationConfig,plant_equipment:usize,identifier_bytes:usize,model_bytes:usize)->Option<Self>{
        let timesteps = SimulationKernel::run_period(config).total_hours() as usize;
        let weather_records = observed_weather_records(config);
        let airflow_nodes = model.airflow_network.as_ref().map_or(0, |network| network.zone_node_ids.capacity().saturating_add(1));
        let airflow_links = model.airflow_network.as_ref().map_or(0, |network| network.link_ids.capacity());
        let meters = model.zones.capacity().checked_mul(3)?.checked_add(2)?;
        let series = model.zones.capacity().checked_mul(3)?;
        let samples = timesteps.checked_mul(series)?;
        let history_values = timesteps.checked_mul(model.zones.capacity())?;
        let summary_rows = 7usize;
        let dimensions = [
            model.zones.capacity(),
            model.surfaces.capacity(),
            model.fenestrations.capacity(),
            model.people.capacity(),
            model.lighting.capacity(),
            model.equipment.capacity(),
            model.infiltrations.capacity(),
            airflow_nodes,
            airflow_links,
            model.mechanical_ventilations.capacity(),
            model.thermostats.capacity(),
            model.humidistats.capacity(),
            model.ideal_loads.capacity(),
            model.faults.capacity(),
            model.zone_equipment.capacity(),
            model.plant_loops.capacity(),
            plant_equipment,
            model.pv_systems.capacity(),
            model.battery_storage.capacity(),
            model.shw_systems.capacity(),
            model.refrigeration_systems.capacity(),
            model.water_systems.capacity(),
            weather_records,
            timesteps,
            meters,
            series,
            samples,
            history_values,
            summary_rows,
            identifier_bytes,
        ];
        let observed_items = checked_sum(dimensions)?;
        let observed_bytes = model_bytes.checked_add(samples.checked_mul(size_of::<f64>() * 2)?.checked_add(history_values.checked_mul(size_of::<f64>())?)?)?.checked_add(identifier_bytes)?;
        let pages = observed_bytes.checked_add(16_383)?.checked_div(16_384)?;
        Some(Self {
            zones: model.zones.capacity(),
            surfaces: model.surfaces.capacity(),
            fenestrations: model.fenestrations.capacity(),
            people: model.people.capacity(),
            lighting: model.lighting.capacity(),
            equipment: model.equipment.capacity(),
            infiltrations: model.infiltrations.capacity(),
            airflow_nodes,
            airflow_links,
            mechanical_ventilations: model.mechanical_ventilations.capacity(),
            thermostats: model.thermostats.capacity(),
            humidistats: model.humidistats.capacity(),
            ideal_loads: model.ideal_loads.capacity(),
            faults: model.faults.capacity(),
            zone_equipment: model.zone_equipment.capacity(),
            plant_loops: model.plant_loops.capacity(),
            plant_equipment,
            pv_systems: model.pv_systems.capacity(),
            batteries: model.battery_storage.capacity(),
            service_hot_water: model.shw_systems.capacity(),
            refrigeration: model.refrigeration_systems.capacity(),
            water: model.water_systems.capacity(),
            weather_records,
            timesteps,
            meters,
            series,
            samples,
            history_values,
            summary_rows,
            identifier_bytes,
            observed_items,
            observed_bytes,
            pages,
            operations: 1,
            process_jobs: 1,
        })
    }

    pub fn first_exceeded(self, maximum: Self) -> Option<EnergyNumericalDimension> {
        macro_rules! dimension {
            ($field:ident, $variant:ident) => {
                if self.$field > maximum.$field {
                    return Some(EnergyNumericalDimension::$variant);
                }
            };
        }
        dimension!(zones, Zones);
        dimension!(surfaces, Surfaces);
        dimension!(fenestrations, Fenestrations);
        dimension!(people, People);
        dimension!(lighting, Lighting);
        dimension!(equipment, Equipment);
        dimension!(infiltrations, Infiltrations);
        dimension!(airflow_nodes, AirflowNodes);
        dimension!(airflow_links, AirflowLinks);
        dimension!(mechanical_ventilations, MechanicalVentilations);
        dimension!(thermostats, Thermostats);
        dimension!(humidistats, Humidistats);
        dimension!(ideal_loads, IdealLoads);
        dimension!(faults, Faults);
        dimension!(zone_equipment, ZoneEquipment);
        dimension!(plant_loops, PlantLoops);
        dimension!(plant_equipment, PlantEquipment);
        dimension!(pv_systems, PvSystems);
        dimension!(batteries, Batteries);
        dimension!(service_hot_water, ServiceHotWater);
        dimension!(refrigeration, Refrigeration);
        dimension!(water, Water);
        dimension!(weather_records, WeatherRecords);
        dimension!(timesteps, Timesteps);
        dimension!(meters, Meters);
        dimension!(series, Series);
        dimension!(samples, Samples);
        dimension!(history_values, HistoryValues);
        dimension!(summary_rows, SummaryRows);
        dimension!(identifier_bytes, IdentifierBytes);
        dimension!(observed_items, ObservedItems);
        dimension!(observed_bytes, ObservedBytes);
        dimension!(pages, Pages);
        dimension!(operations, Operations);
        dimension!(process_jobs, ProcessJobs);
        None
    }
}
