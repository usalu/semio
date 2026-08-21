//! 🌬️ Zone sensible and latent air balance: transient BDF3, analytical steady state, unmet load.

use crate::props::{latent_heat_vaporization, moist_air_density, moist_air_enthalpy_j_per_kg};
use crate::units::{CP_DRY_AIR, P_STD};
use serde::{Deserialize, Serialize};

// #region 🔖️HumiditySolutionMethod
/// 💧️ Humidity ratio integration method for zone air mass balance.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HumiditySolutionMethod {
    AnalyticalSteadyState,
    ThirdOrderBackward,
}
// #endregion 🔖️HumiditySolutionMethod

// #region 🔖️ZoneAirState
/// 🌡️ Zone air state with BDF3 temperature history [T_n, T_{n-1}, T_{n-2}, T_{n-3}].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ZoneAirState {
    pub temp_c: f64,
    pub humidity_ratio: f64,
    pub temp_history_c: [f64; 4],
    pub humidity_history: [f64; 4],
}

impl ZoneAirState {
    pub fn new(temp_c: f64, humidity_ratio: f64) -> Self {
        Self { temp_c, humidity_ratio, temp_history_c: [temp_c; 4], humidity_history: [humidity_ratio; 4] }
    }

    pub fn push_temp(&mut self, temp_c: f64) {
        self.temp_history_c = [temp_c, self.temp_history_c[0], self.temp_history_c[1], self.temp_history_c[2]];
        self.temp_c = temp_c;
    }

    pub fn push_humidity(&mut self, w: f64) {
        self.humidity_history = [w, self.humidity_history[0], self.humidity_history[1], self.humidity_history[2]];
        self.humidity_ratio = w;
    }
}
// #endregion 🔖️ZoneAirState

// #region 🔖️ZoneAirBalance
/// ⚖️ Zone air energy and moisture balance inputs [W] and [kg/s].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ZoneAirBalance {
    pub volume_m3: f64,
    pub conditioned: bool,
    pub sensible_gain_w: f64,
    pub latent_gain_w: f64,
    pub infiltration_sensible_w: f64,
    pub infiltration_latent_w: f64,
    pub ventilation_sensible_w: f64,
    pub ventilation_latent_w: f64,
    pub system_sensible_w: f64,
    pub system_latent_w: f64,
    pub surface_convection_w: f64,
    pub mass_flow_in_kg_s: f64,
    pub supply_humidity_ratio: f64,
    pub outdoor_humidity_ratio: f64,
    pub heating_setpoint_c: Option<f64>,
    pub cooling_setpoint_c: Option<f64>,
    pub max_heating_w: Option<f64>,
    pub max_cooling_w: Option<f64>,
}

impl ZoneAirBalance {
    pub fn net_sensible_w(&self) -> f64 {
        self.sensible_gain_w + self.surface_convection_w + self.infiltration_sensible_w + self.ventilation_sensible_w + self.system_sensible_w
    }

    pub fn net_latent_w(&self) -> f64 {
        self.latent_gain_w + self.infiltration_latent_w + self.ventilation_latent_w + self.system_latent_w
    }
}
// #endregion 🔖️ZoneAirBalance

// #region 🔖️ZoneAirResult
/// 📊️ Zone air step result including unmet loads.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ZoneAirResult {
    pub temp_c: f64,
    pub humidity_ratio: f64,
    pub unmet_heating_w: f64,
    pub unmet_cooling_w: f64,
    pub unmet_humidifying_w: f64,
    pub unmet_dehumidifying_w: f64,
}
// #endregion 🔖️ZoneAirResult

// #region 🔖️Capacitance
fn zone_sensible_capacitance_j_per_k(volume_m3: f64, temp_c: f64, w: f64, p_atm: f64) -> f64 {
    let rho = moist_air_density(temp_c, w, p_atm);
    rho * volume_m3 * CP_DRY_AIR
}

fn zone_moisture_capacitance_kg_per_k(volume_m3: f64, temp_c: f64, w: f64, p_atm: f64) -> f64 {
    moist_air_density(temp_c, w, p_atm) * volume_m3
}
// #endregion 🔖️Capacitance

// #region 🔖️Bdf3
fn bdf3_next_value(history: [f64; 4], dt_s: f64, rate: f64) -> f64 {
    let coeff = 6.0 * dt_s * rate;
    (coeff + 18.0 * history[1] - 9.0 * history[2] + 2.0 * history[3]) / 11.0
}

#[allow(dead_code, reason = "BDF3 rate-recovery counterpart to bdf3_next_value, validated by its own unit test but not yet wired into a production call site — in-flight energy BEM zone-air numerics")]
fn bdf3_rate(history: [f64; 4], dt_s: f64) -> f64 {
    (11.0 * history[0] - 18.0 * history[1] + 9.0 * history[2] - 2.0 * history[3]) / (6.0 * dt_s)
}
// #endregion 🔖️Bdf3

// #region 🔖️Analytical
/// 🌡️ Steady-state zone air temperature [°C] from sensible balance Q [W] and reference temp.
pub fn analytical_steady_temp_c(q_sensible_w: f64, temp_ref_c: f64, ua_w_per_k: f64) -> f64 {
    if ua_w_per_k.abs() < 1e-9 {
        return temp_ref_c;
    }
    temp_ref_c + q_sensible_w / ua_w_per_k
}

/// 💧️ Steady-state humidity ratio [kg/kg] from latent balance.
pub fn analytical_steady_humidity_ratio(latent_gain_w: f64, mass_flow_kg_s: f64, w_supply: f64, temp_c: f64) -> f64 {
    if mass_flow_kg_s.abs() < 1e-12 {
        return w_supply;
    }
    let h_fg = latent_heat_vaporization(temp_c);
    w_supply + latent_gain_w / (mass_flow_kg_s * h_fg)
}
// #endregion 🔖️Analytical

// #region 🔖️UnmetLoad
fn compute_unmet_loads(balance: &ZoneAirBalance, temp_c: f64, humidity_ratio: f64) -> (f64, f64, f64, f64) {
    let mut unmet_heating = 0.0;
    let mut unmet_cooling = 0.0;
    let unmet_humid = 0.0;
    let unmet_dehumid = 0.0;

    if balance.conditioned {
        if let Some(t_heat) = balance.heating_setpoint_c {
            if temp_c < t_heat {
                let deficit = (t_heat - temp_c) * zone_sensible_capacitance_j_per_k(balance.volume_m3, temp_c, humidity_ratio, P_STD);
                let delivered = balance.system_sensible_w.max(0.0);
                let cap = balance.max_heating_w.unwrap_or(f64::INFINITY);
                unmet_heating = (deficit - delivered).max(0.0).min(cap);
            }
        }
        if let Some(t_cool) = balance.cooling_setpoint_c {
            if temp_c > t_cool {
                let excess = (temp_c - t_cool) * zone_sensible_capacitance_j_per_k(balance.volume_m3, temp_c, humidity_ratio, P_STD);
                let delivered = (-balance.system_sensible_w).max(0.0);
                let cap = balance.max_cooling_w.unwrap_or(f64::INFINITY);
                unmet_cooling = (excess - delivered).max(0.0).min(cap);
            }
        }
    }
    (unmet_heating, unmet_cooling, unmet_humid, unmet_dehumid)
}
// #endregion 🔖️UnmetLoad

// #region 🔖️Advance
/// ⏩️ Advance zone air state one timestep.
pub fn advance_zone_air(state: &ZoneAirState, balance: &ZoneAirBalance, dt_s: f64, method: HumiditySolutionMethod, p_atm: f64) -> ZoneAirResult {
    let c_sens = zone_sensible_capacitance_j_per_k(balance.volume_m3, state.temp_c, state.humidity_ratio, p_atm);
    let q_sens = balance.net_sensible_w();

    let temp_c = if balance.conditioned {
        if dt_s > 0.0 && c_sens > 0.0 {
            let rate = q_sens / c_sens;
            bdf3_next_value(state.temp_history_c, dt_s, rate)
        } else {
            state.temp_c
        }
    } else {
        let ua = c_sens / 3600.0;
        analytical_steady_temp_c(q_sens, state.temp_c, ua.max(1.0))
    };

    let humidity_ratio = match method {
        HumiditySolutionMethod::AnalyticalSteadyState => {
            let w_in = if balance.mass_flow_in_kg_s > 0.0 { balance.supply_humidity_ratio } else { balance.outdoor_humidity_ratio };
            analytical_steady_humidity_ratio(balance.net_latent_w(), balance.mass_flow_in_kg_s, w_in, temp_c)
        }
        HumiditySolutionMethod::ThirdOrderBackward => {
            let c_moist = zone_moisture_capacitance_kg_per_k(balance.volume_m3, temp_c, state.humidity_ratio, p_atm);
            let h_fg = latent_heat_vaporization(temp_c);
            let w_in = balance.supply_humidity_ratio;
            let latent_kg_s = balance.net_latent_w() / h_fg;
            let rate = if c_moist > 0.0 { (balance.mass_flow_in_kg_s * (w_in - state.humidity_ratio) + latent_kg_s) / c_moist } else { 0.0 };
            if dt_s > 0.0 {
                bdf3_next_value(state.humidity_history, dt_s, rate).max(0.0)
            } else {
                state.humidity_ratio
            }
        }
    };

    let (unmet_heating_w, unmet_cooling_w, unmet_humidifying_w, unmet_dehumidifying_w) = compute_unmet_loads(balance, temp_c, humidity_ratio);

    ZoneAirResult { temp_c, humidity_ratio, unmet_heating_w, unmet_cooling_w, unmet_humidifying_w, unmet_dehumidifying_w }
}

/// 🔄️ Commit zone air result into mutable state history.
pub fn commit_zone_air(state: &mut ZoneAirState, result: ZoneAirResult) {
    state.push_temp(result.temp_c);
    state.push_humidity(result.humidity_ratio);
}

/// 🔥️ Sensible load to change zone air from T1 to T2 [W] over dt [s].
pub fn sensible_load_for_delta_t_w(volume_m3: f64, t1_c: f64, t2_c: f64, w: f64, dt_s: f64, p_atm: f64) -> f64 {
    if dt_s <= 0.0 {
        return 0.0;
    }
    let c = zone_sensible_capacitance_j_per_k(volume_m3, (t1_c + t2_c) * 0.5, w, p_atm);
    c * (t2_c - t1_c) / dt_s
}

/// 💧️ Enthalpy difference for ventilation [J/kg dry air].
pub fn ventilation_enthalpy_delta_j_per_kg(t_zone_c: f64, w_zone: f64, t_out_c: f64, w_out: f64) -> f64 {
    moist_air_enthalpy_j_per_kg(t_zone_c, w_zone) - moist_air_enthalpy_j_per_kg(t_out_c, w_out)
}
// #endregion 🔖️Advance

#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    fn analytical_temp_rises_with_gain() {
        let t = analytical_steady_temp_c(1000.0, 20.0, 100.0);
        assert!((t - 30.0).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    fn bdf3_rate_constant_history() {
        let h = [22.0, 22.0, 22.0, 22.0];
        assert!(bdf3_rate(h, 3600.0).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    fn floating_zone_uses_steady_analytical() {
        let state = ZoneAirState::new(20.0, 0.008);
        let balance = ZoneAirBalance {
            volume_m3: 100.0,
            conditioned: false,
            sensible_gain_w: 500.0,
            latent_gain_w: 0.0,
            infiltration_sensible_w: 0.0,
            infiltration_latent_w: 0.0,
            ventilation_sensible_w: 0.0,
            ventilation_latent_w: 0.0,
            system_sensible_w: 0.0,
            system_latent_w: 0.0,
            surface_convection_w: 0.0,
            mass_flow_in_kg_s: 0.0,
            supply_humidity_ratio: 0.008,
            outdoor_humidity_ratio: 0.008,
            heating_setpoint_c: None,
            cooling_setpoint_c: None,
            max_heating_w: None,
            max_cooling_w: None,
        };
        let result = advance_zone_air(&state, &balance, 3600.0, HumiditySolutionMethod::AnalyticalSteadyState, P_STD);
        assert!(result.temp_c > 20.0);
    }

    #[semio_framework_async_macros::async_test]
    fn conditioned_bdf3_warms_zone() {
        let state = ZoneAirState::new(20.0, 0.008);
        let balance = ZoneAirBalance {
            volume_m3: 200.0,
            conditioned: true,
            sensible_gain_w: 2000.0,
            latent_gain_w: 0.0,
            infiltration_sensible_w: -100.0,
            infiltration_latent_w: 0.0,
            ventilation_sensible_w: 0.0,
            ventilation_latent_w: 0.0,
            system_sensible_w: 0.0,
            system_latent_w: 0.0,
            surface_convection_w: 0.0,
            mass_flow_in_kg_s: 0.05,
            supply_humidity_ratio: 0.008,
            outdoor_humidity_ratio: 0.006,
            heating_setpoint_c: Some(21.0),
            cooling_setpoint_c: Some(26.0),
            max_heating_w: Some(5000.0),
            max_cooling_w: Some(5000.0),
        };
        let result = advance_zone_air(&state, &balance, 3600.0, HumiditySolutionMethod::ThirdOrderBackward, P_STD);
        assert!(result.temp_c > 20.0);
    }

    #[semio_framework_async_macros::async_test]
    fn humidity_analytical_increases_with_latent_gain() {
        let w = analytical_steady_humidity_ratio(200.0, 0.1, 0.008, 22.0);
        assert!(w > 0.008);
    }
}
