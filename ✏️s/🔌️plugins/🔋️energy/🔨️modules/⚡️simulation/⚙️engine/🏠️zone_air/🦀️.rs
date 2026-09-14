//! 🌬️ Well-mixed zone air: third-order backward-difference storage of heat and moisture.
//!
//! The zone air node stores `C·dT/dt` with `C = ρ·V·c_p` and the time derivative taken by the
//! third-order backward difference EnergyPlus uses by default,
//! `dT/dt ≈ (11/6·Tₙ − 3·Tₙ₋₁ + 3/2·Tₙ₋₂ − 1/3·Tₙ₋₃) / Δt`, so the sensible balance of one step
//! is linear in `Tₙ` and can be solved simultaneously with the surface heat balances.

use crate::props::{latent_heat_vaporization, moist_air_cp_j_per_kg_k, moist_air_density};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};
use serde::{Deserialize, Serialize};

// #region 🔖️ZoneAirState
/// 🌡️ Zone air state with its last three committed temperatures and humidity ratios.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct ZoneAirState {
    pub temp_c: f64,
    pub humidity_ratio: f64,
    pub temp_history_c: [f64; 3],
    pub humidity_history: [f64; 3],
}

impl ZoneAirState {
    pub fn new(temp_c: f64, humidity_ratio: f64) -> Self {
        Self { temp_c, humidity_ratio, temp_history_c: [temp_c; 3], humidity_history: [humidity_ratio; 3] }
    }

    /// 🔄️ Commits the temperature and humidity ratio of a completed step.
    pub fn commit(&mut self, temp_c: f64, humidity_ratio: f64) {
        self.temp_history_c = [temp_c, self.temp_history_c[0], self.temp_history_c[1]];
        self.humidity_history = [humidity_ratio, self.humidity_history[0], self.humidity_history[1]];
        self.temp_c = temp_c;
        self.humidity_ratio = humidity_ratio;
    }

    /// 🧮️ Sensible heat capacity rate `ρ·V·c_p / Δt` [W/K] at the current state.
    pub fn capacity_rate_w_k(&self, volume_m3: f64, pressure_pa: f64, step_s: f64) -> f64 {
        moist_air_density(self.temp_c, self.humidity_ratio, pressure_pa) * volume_m3 * moist_air_cp_j_per_kg_k(self.humidity_ratio) / step_s.max(1e-9)
    }

    /// 🧮️ Third-order backward difference split into the coefficient of the new temperature and the
    /// known history part: storage `= rate·(11/6·T − history)`, returned as `(11/6·rate, rate·history)`.
    pub fn storage_terms(&self, capacity_rate_w_k: f64) -> (f64, f64) {
        let history = 3.0 * self.temp_history_c[0] - 1.5 * self.temp_history_c[1] + self.temp_history_c[2] / 3.0;
        (capacity_rate_w_k * 11.0 / 6.0, capacity_rate_w_k * history)
    }

    /// 💧️ Humidity ratio after one step of the moisture balance with an outdoor-air mass flow
    /// [kg/s] and a latent gain [W], by the same third-order backward difference.
    pub fn next_humidity_ratio(&self, volume_m3: f64, pressure_pa: f64, step_s: f64, air_mass_flow_kg_s: f64, outdoor_humidity_ratio: f64, latent_gain_w: f64) -> f64 {
        let mass_rate = moist_air_density(self.temp_c, self.humidity_ratio, pressure_pa) * volume_m3 / step_s.max(1e-9);
        let history = 3.0 * self.humidity_history[0] - 1.5 * self.humidity_history[1] + self.humidity_history[2] / 3.0;
        let source = latent_gain_w / latent_heat_vaporization(self.temp_c);
        ((mass_rate * history + air_mass_flow_kg_s * outdoor_humidity_ratio + source) / (mass_rate * 11.0 / 6.0 + air_mass_flow_kg_s)).max(0.0)
    }
}
// #endregion 🔖️ZoneAirState

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
