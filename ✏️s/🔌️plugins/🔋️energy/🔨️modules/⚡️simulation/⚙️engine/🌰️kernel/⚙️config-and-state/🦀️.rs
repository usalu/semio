//! ⚙️ The simulation environment and configuration, the delivered energy record and the zone, surface, window and solver state.

use super::*;

// #region 🔖️Config
/// ⚙️ Simulation environment type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum SimulationEnvironment {
    WeatherRunPeriod,
    HeatingDesignDay,
    CoolingDesignDay,
    CustomDesignPeriod,
}

/// ⚙️ Convergence tolerances.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct ConvergenceTolerances {
    pub temperature_k: f64,
    pub humidity_ratio: f64,
    pub mass_flow: f64,
    pub energy_w: f64,
    pub max_iterations: u32,
}

impl Default for ConvergenceTolerances {
    fn default() -> Self {
        Self { temperature_k: 0.01, humidity_ratio: 1e-5, mass_flow: 1e-4, energy_w: 1.0, max_iterations: 20 }
    }
}

/// ⚙️ Simulation configuration. `zone_timestep_minutes` is the interval at which weather, sun
/// position, shading and exterior film coefficients are refreshed; `system_timestep_minutes` the
/// implicit step of the coupled surface/air balance and its HVAC control (never longer than the
/// zone timestep). Both divide the hour.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct SimulationConfig {
    pub environment: SimulationEnvironment,
    pub zone_timestep_minutes: u32,
    pub system_timestep_minutes: u32,
    pub warmup_days: u32,
    pub run_period_year: u16,
    pub run_period_start_month: u8,
    pub run_period_start_day: u8,
    pub run_period_end_month: u8,
    pub run_period_end_day: u8,
    pub tolerances: ConvergenceTolerances,
    pub schedules: ScheduleSet,
    pub weather: Option<site::WeatherData>,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            environment: SimulationEnvironment::WeatherRunPeriod,
            zone_timestep_minutes: 60,
            system_timestep_minutes: 60,
            warmup_days: 7,
            run_period_year: 2026,
            run_period_start_month: 1,
            run_period_start_day: 1,
            run_period_end_month: 12,
            run_period_end_day: 31,
            tolerances: ConvergenceTolerances::default(),
            schedules: ScheduleSet::default(),
            weather: None,
        }
    }
}
// #endregion 🔖️Config

// #region 🔖️DeliveredEnergy
/// ⚡️ Delivered energy per timestep for metering.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub struct DeliveredEnergy {
    pub heating_w: f64,
    pub cooling_w: f64,
    pub fan_w: f64,
    pub pump_w: f64,
    pub compressor_w: f64,
    pub gas_w: f64,
    pub pv_generation_w: f64,
    pub battery_charge_w: f64,
    pub shw_electric_w: f64,
    pub shw_gas_w: f64,
    pub refrigeration_w: f64,
    pub water_pump_w: f64,
}

impl DeliveredEnergy {
    pub fn total_electric_w(&self) -> f64 {
        self.heating_w + self.cooling_w + self.fan_w + self.pump_w + self.compressor_w + self.shw_electric_w + self.refrigeration_w + self.water_pump_w - self.pv_generation_w + self.battery_charge_w
    }

    pub(super) fn scaled(&self, factor: f64) -> Self {
        Self {
            heating_w: self.heating_w * factor,
            cooling_w: self.cooling_w * factor,
            fan_w: self.fan_w * factor,
            pump_w: self.pump_w * factor,
            compressor_w: self.compressor_w * factor,
            gas_w: self.gas_w * factor,
            pv_generation_w: self.pv_generation_w * factor,
            battery_charge_w: self.battery_charge_w * factor,
            shw_electric_w: self.shw_electric_w * factor,
            shw_gas_w: self.shw_gas_w * factor,
            refrigeration_w: self.refrigeration_w * factor,
            water_pump_w: self.water_pump_w * factor,
        }
    }
}
// #endregion 🔖️DeliveredEnergy

// #region 🔖️State
/// 🔄️ Per-zone simulation state. `mean_air_temp_c` and `delivered` are the averages over the last
/// completed hour; `air` is the state after the last heat-balance step.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub struct ZoneState {
    pub air: ZoneAirState,
    pub mean_air_temp_c: f64,
    pub mean_radiant_temp_c: f64,
    pub heating_demand_w: f64,
    pub cooling_demand_w: f64,
    pub unmet_heating_w: f64,
    pub unmet_cooling_w: f64,
    pub delivered: DeliveredEnergy,
    pub hour_temperature_sum_c: f64,
    pub hour_delivered: DeliveredEnergy,
    pub hour_steps: u32,
}

impl ZoneState {
    pub fn new(temp_c: f64, humidity_ratio: f64) -> Self {
        Self {
            air: ZoneAirState::new(temp_c, humidity_ratio),
            mean_air_temp_c: temp_c,
            mean_radiant_temp_c: temp_c,
            heating_demand_w: 0.0,
            cooling_demand_w: 0.0,
            unmet_heating_w: 0.0,
            unmet_cooling_w: 0.0,
            delivered: DeliveredEnergy::default(),
            hour_temperature_sum_c: 0.0,
            hour_delivered: DeliveredEnergy::default(),
            hour_steps: 0,
        }
    }
}

/// 🧱️ Node temperatures of an opaque surface, outside face first, and its last room-side film.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub struct SurfaceState {
    pub temperatures_c: Vec<f64>,
    pub inside_convection_w_m2k: f64,
}

impl SurfaceState {
    /// 🧱️ Inside-face temperature.
    pub fn inside_temp_c(&self) -> f64 {
        self.temperatures_c.last().copied().unwrap_or(0.0)
    }
}

/// 🪟️ Glazing face temperatures of a window, outside face first.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub struct WindowState {
    pub face_temperatures_c: [f64; 2 * MAX_PANES],
    pub inside_convection_w_m2k: f64,
}

/// 🧮️ Reserved scratch of the zone heat-balance solver, sized once for the largest enclosure.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub struct SolverWorkspace {
    pub faces: usize,
    pub nodes: usize,
    pub matrix: Vec<f64>,
    pub scratch: Vec<f64>,
    pub rhs: Vec<f64>,
    pub solution: Vec<f64>,
    pub inside_absorbed_w_m2: Vec<f64>,
    pub pane_absorbed_w_m2: Vec<f64>,
    pub inside_convection_w_m2k: Vec<f64>,
    pub reduction: Vec<f64>,
    pub chain_a: Vec<f64>,
    pub chain_b: Vec<f64>,
}

impl SolverWorkspace {
    /// 🧮️ Reserves every buffer for enclosures of up to `faces` faces and chains of `nodes` nodes.
    pub fn reserve(&mut self, faces: usize, nodes: usize) -> Result<(), Error> {
        let order = faces + 1;
        let reject = || Error::severe("energy heat-balance workspace backing rejected");
        let exact = faces.min(precompute::EXACT_ENCLOSURE_FACES) + 1;
        for (buffer, length) in [
            (&mut self.matrix, exact * exact),
            (&mut self.scratch, exact * exact),
            (&mut self.rhs, order),
            (&mut self.solution, order),
            (&mut self.inside_absorbed_w_m2, faces),
            (&mut self.pane_absorbed_w_m2, faces * MAX_PANES),
            (&mut self.inside_convection_w_m2k, faces),
            (&mut self.reduction, faces * 3),
            (&mut self.chain_a, faces * nodes.max(2 * MAX_PANES)),
            (&mut self.chain_b, faces * nodes.max(2 * MAX_PANES)),
        ] {
            buffer.try_reserve_exact(length).map_err(|_| reject())?;
            buffer.resize(length, 0.0);
        }
        self.faces = faces;
        self.nodes = nodes.max(2 * MAX_PANES);
        Ok(())
    }
}

/// 🔄️ Full simulation state.
#[derive(Clone, Debug, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub struct SimulationModel {
    pub zones: FixedTable<EntityId, ZoneState>,
    pub surfaces: FixedTable<EntityId, SurfaceState>,
    pub windows: FixedTable<EntityId, WindowState>,
    pub solver: SolverWorkspace,
    /// 🧱️ Per-surface energy integrated over the RUN period only — every accumulate site is gated on
    /// `warmup_complete`, so the warmup days never contribute.
    pub per_surface: results::SurfaceEnergyTable,
    pub warmup_complete: bool,
    pub hour: u32,
    pub delivered_total: DeliveredEnergy,
    pub(crate) battery_soc: f64,
    pub(crate) plant_supply_c: f64,
}

impl Default for SimulationModel {
    fn default() -> Self {
        Self {
            zones: FixedTable::default(),
            surfaces: FixedTable::default(),
            windows: FixedTable::default(),
            solver: SolverWorkspace::default(),
            per_surface: results::SurfaceEnergyTable::default(),
            warmup_complete: false,
            hour: 0,
            delivered_total: DeliveredEnergy::default(),
            battery_soc: 0.5,
            plant_supply_c: 55.0,
        }
    }
}
// #endregion 🔖️State
