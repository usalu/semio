//! ⚡️ Electrical systems: loads, PV, wind, generators, inverters, batteries, transformers, grid.

use crate::units::deg_to_rad;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

// #region 🔖️EndUse
/// 💡️ Generic electrical end-use load.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct EndUseLoad {
    pub name: String,
    pub rated_power_w: f64,
    pub schedule_factor: f64,
    pub power_factor: f64,
}

impl EndUseLoad {
    /// 💡️ Instantaneous real power [W].
    pub fn power_w(&self) -> f64 {
        self.rated_power_w * self.schedule_factor.clamp(0.0, 1.0)
    }

    /// ⚡️ Apparent power [VA].
    pub fn apparent_va(&self) -> f64 {
        let pf = self.power_factor.clamp(0.1, 1.0);
        self.power_w() / pf
    }
}
// #endregion 🔖️EndUse

// #region 🔖️Pv
/// ☀️ Photovoltaic array with inverter.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct PvSystem {
    pub dc_capacity_w: f64,
    pub module_efficiency: f64,
    pub area_m2: f64,
    pub inverter_efficiency: f64,
    pub temperature_coefficient: f64,
    pub tilt_deg: f64,
    pub azimuth_deg: f64,
}

impl PvSystem {
    /// ☀️ AC power output from plane-of-array irradiance.
    pub fn simulate(&self, poa_irradiance_w_m2: f64, cell_temperature_c: f64) -> f64 {
        if poa_irradiance_w_m2 <= 0.0 {
            return 0.0;
        }
        let temp_factor = 1.0 + self.temperature_coefficient * (cell_temperature_c - 25.0);
        let dc_w = self.area_m2 * poa_irradiance_w_m2 * self.module_efficiency * temp_factor;
        let clipped = dc_w.min(self.dc_capacity_w);
        clipped * self.inverter_efficiency.clamp(0.85, 0.99)
    }

    /// 📐️ Tilt/azimuth factor relative to horizontal south-facing surface.
    pub fn orientation_factor(&self, solar_altitude_deg: f64, solar_azimuth_deg: f64) -> f64 {
        let tilt = deg_to_rad(self.tilt_deg);
        let surf_az = deg_to_rad(self.azimuth_deg);
        let sun_alt = deg_to_rad(solar_altitude_deg);
        let sun_az = deg_to_rad(solar_azimuth_deg);
        let cos_inc = sun_alt.sin() * tilt.cos() + sun_alt.cos() * tilt.sin() * (sun_az - surf_az).cos();
        cos_inc.clamp(0.0, 1.0)
    }
}
// #endregion 🔖️Pv

// #region 🔖️Wind
/// 💨️ Wind turbine with cut-in/rated/cut-out speeds.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct WindTurbine {
    pub rated_power_w: f64,
    pub cut_in_m_s: f64,
    pub rated_speed_m_s: f64,
    pub cut_out_m_s: f64,
    pub hub_height_m: f64,
    pub rotor_diameter_m: f64,
}

impl WindTurbine {
    /// 💨️ Electrical output from semio_hub-height wind speed.
    pub fn simulate(&self, wind_speed_m_s: f64, air_density: f64) -> f64 {
        let v = wind_speed_m_s;
        if v < self.cut_in_m_s || v > self.cut_out_m_s {
            return 0.0;
        }
        if v >= self.rated_speed_m_s {
            return self.rated_power_w;
        }
        let frac = (v - self.cut_in_m_s) / (self.rated_speed_m_s - self.cut_in_m_s);
        self.rated_power_w * frac.powi(3) * (air_density / 1.2)
    }
}
// #endregion 🔖️Wind

// #region 🔖️Generator
/// 🔌️ Backup generator (diesel or gas).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct Generator {
    pub rated_power_w: f64,
    pub fuel_lhv_j_per_kg: f64,
    pub electrical_efficiency: f64,
    pub min_load_fraction: f64,
}

impl Generator {
    /// 🔌️ Generator electrical output and fuel consumption.
    pub fn simulate(&self, requested_w: f64, operating: bool) -> (f64, f64) {
        if !operating || requested_w <= 0.0 {
            return (0.0, 0.0);
        }
        let min_w = self.rated_power_w * self.min_load_fraction;
        let output = requested_w.clamp(min_w, self.rated_power_w);
        let fuel_kg_s = output / (self.fuel_lhv_j_per_kg * self.electrical_efficiency);
        (output, fuel_kg_s)
    }
}
// #endregion 🔖️Generator

// #region 🔖️Inverter
/// 🔄️ DC/AC inverter with efficiency curve.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct Inverter {
    pub rated_ac_w: f64,
    pub peak_efficiency: f64,
    pub standby_w: f64,
}

impl Inverter {
    /// 🔄️ Convert DC to AC with part-load efficiency penalty.
    pub fn simulate(&self, dc_w: f64) -> f64 {
        if dc_w <= 0.0 {
            return -self.standby_w;
        }
        let plr = (dc_w / self.rated_ac_w).clamp(0.05, 1.0);
        let eta = self.peak_efficiency * (0.9 + 0.1 * plr);
        dc_w * eta
    }
}
// #endregion 🔖️Inverter

// #region 🔖️Battery
/// 🔋️ Electrochemical storage with SOC limits.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct Battery {
    pub capacity_kwh: f64,
    pub max_charge_w: f64,
    pub max_discharge_w: f64,
    pub round_trip_efficiency: f64,
    pub min_soc: f64,
    pub max_soc: f64,
    pub state_of_charge: f64,
}

impl Battery {
    /// 🔋️ Charge or discharge for one timestep; returns (grid_power_w, new_soc).
    pub fn simulate(&self, requested_w: f64, dt_s: f64) -> (f64, f64) {
        let capacity_j = self.capacity_kwh * 3_600_000.0;
        let mut soc = self.state_of_charge.clamp(self.min_soc, self.max_soc);
        let mut actual_w = 0.0;
        if requested_w > 0.0 {
            let charge_w = requested_w.min(self.max_charge_w);
            let energy_j = charge_w * dt_s * self.round_trip_efficiency.sqrt();
            let delta_soc = energy_j / capacity_j;
            if soc + delta_soc <= self.max_soc {
                soc += delta_soc;
                actual_w = charge_w;
            } else {
                let allowed_j = (self.max_soc - soc) * capacity_j;
                actual_w = allowed_j / (dt_s * self.round_trip_efficiency.sqrt());
                soc = self.max_soc;
            }
        } else if requested_w < 0.0 {
            let discharge_w = (-requested_w).min(self.max_discharge_w);
            let energy_j = discharge_w * dt_s / self.round_trip_efficiency.sqrt();
            let delta_soc = energy_j / capacity_j;
            if soc - delta_soc >= self.min_soc {
                soc -= delta_soc;
                actual_w = -discharge_w;
            } else {
                let allowed_j = (soc - self.min_soc) * capacity_j;
                actual_w = -(allowed_j * self.round_trip_efficiency.sqrt() / dt_s);
                soc = self.min_soc;
            }
        }
        (actual_w, soc)
    }
}
// #endregion 🔖️Battery

// #region 🔖️Transformer
/// 🔌️ Building transformer with no-load and load losses.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct Transformer {
    pub rated_kva: f64,
    pub no_load_loss_w: f64,
    pub load_loss_w: f64,
    pub impedance_fraction: f64,
}

impl Transformer {
    /// 🔌️ Transformer total losses [W] at given apparent load.
    pub fn losses_w(&self, apparent_va: f64) -> f64 {
        let plr = (apparent_va / (self.rated_kva * 1000.0)).clamp(0.0, 1.5);
        self.no_load_loss_w + self.load_loss_w * plr * plr
    }

    /// 📉️ Secondary voltage drop fraction.
    pub fn voltage_drop_fraction(&self, apparent_va: f64) -> f64 {
        let plr = (apparent_va / (self.rated_kva * 1000.0)).clamp(0.0, 1.5);
        self.impedance_fraction * plr
    }
}
// #endregion 🔖️Transformer

// #region 🔖️Grid
/// 🏭️ Grid interconnection balance for one timestep.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct GridBalance {
    pub building_load_w: f64,
    pub pv_generation_w: f64,
    pub wind_generation_w: f64,
    pub generator_output_w: f64,
    pub battery_power_w: f64,
    pub transformer_loss_w: f64,
    pub net_import_w: f64,
    pub net_export_w: f64,
}

/// 🏭️ Compute grid import/export from supply and demand.
pub fn grid_balance(building_load_w: f64, pv_w: f64, wind_w: f64, generator_w: f64, battery_w: f64, transformer: &Transformer) -> GridBalance {
    let supply_w = pv_w + wind_w + generator_w - battery_w;
    let apparent = (building_load_w - supply_w).abs();
    let transformer_loss = transformer.losses_w(apparent);
    let net = building_load_w + transformer_loss - supply_w;
    GridBalance { building_load_w, pv_generation_w: pv_w, wind_generation_w: wind_w, generator_output_w: generator_w, battery_power_w: battery_w, transformer_loss_w: transformer_loss, net_import_w: net.max(0.0), net_export_w: (-net).max(0.0) }
}
// #endregion 🔖️Grid

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
