//! ☀️ Solar thermal collectors: flat-plate, ICS, unglazed transpired, PVT.

use crate::units::{CP_DRY_AIR, RHO_AIR_REF, STEFAN_BOLTZMANN};
use serde::{Deserialize, Serialize};

// #region 🔖️CollectorKind
/// ☀️ Solar thermal collector technology.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CollectorKind {
    FlatPlate,
    IntegralCollectorStorage,
    UnglazedTranspired,
    PhotovoltaicThermal,
}
// #endregion 🔖️CollectorKind

// #region 🔖️FlatPlate
/// ☀️ Glazed flat-plate collector (Hottel-Whillier-Bliss).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FlatPlateCollector {
    pub area_m2: f64,
    pub tau_alpha: f64,
    pub ul_w_m2k: f64,
    pub iam_factor: f64,
}

impl FlatPlateCollector {
    /// ☀️ Useful thermal gain [W] from incident irradiance.
    pub fn useful_gain_w(&self, irradiance_w_m2: f64, ambient_c: f64, wind_m_s: f64, fluid_inlet_c: f64, mass_flow_kg_s: f64, fluid_cp: f64) -> f64 {
        collector_thermal_output_w(CollectorKind::FlatPlate, self.area_m2, irradiance_w_m2, ambient_c, wind_m_s, fluid_inlet_c, mass_flow_kg_s, fluid_cp, self.tau_alpha, self.ul_w_m2k, self.iam_factor, 0.0)
    }
}
// #endregion 🔖️FlatPlate

// #region 🔖️Ics
/// 🫙️ Integral collector-storage (ICS) batch heater.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegralCollectorStorage {
    pub area_m2: f64,
    pub storage_volume_l: f64,
    pub tau_alpha: f64,
    pub loss_coefficient_w_m2k: f64,
}

impl IntegralCollectorStorage {
    /// 🫙️ ICS timestep storage temperature update.
    pub fn simulate(&self, storage_temperature_c: f64, irradiance_w_m2: f64, ambient_c: f64, dt_s: f64) -> (f64, f64) {
        let gain = collector_thermal_output_w(CollectorKind::IntegralCollectorStorage, self.area_m2, irradiance_w_m2, ambient_c, 1.0, storage_temperature_c, 0.0, 4180.0, self.tau_alpha, self.loss_coefficient_w_m2k, 1.0, storage_temperature_c);
        let volume_m3 = self.storage_volume_l / 1000.0;
        let stored_j = 1000.0 * volume_m3 * 4180.0;
        let new_t = storage_temperature_c + gain * dt_s / stored_j.max(1.0);
        (new_t, gain)
    }
}
// #endregion 🔖️Ics

// #region 🔖️Unglazed
/// 🌀️ Unglazed transpired solar collector (solar wall).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnglazedTranspiredCollector {
    pub area_m2: f64,
    pub porosity: f64,
    pub h_conv_w_m2k: f64,
    pub suction_velocity_m_s: f64,
}

impl UnglazedTranspiredCollector {
    /// 🌀️ Preheat ventilation air via transpired absorber.
    pub fn preheat_air_w(&self, irradiance_w_m2: f64, ambient_c: f64, wind_m_s: f64) -> (f64, f64) {
        let gain = collector_thermal_output_w(
            CollectorKind::UnglazedTranspired,
            self.area_m2,
            irradiance_w_m2,
            ambient_c,
            wind_m_s,
            ambient_c,
            self.suction_velocity_m_s * RHO_AIR_REF * self.area_m2,
            CP_DRY_AIR,
            self.porosity,
            self.h_conv_w_m2k,
            1.0,
            ambient_c,
        );
        let m_dot = self.suction_velocity_m_s * RHO_AIR_REF * self.area_m2;
        let outlet_t = if m_dot > 1e-6 { ambient_c + gain / (m_dot * CP_DRY_AIR) } else { ambient_c };
        (gain, outlet_t)
    }
}
// #endregion 🔖️Unglazed

// #region 🔖️Pvt
/// ⚡️☀️ Photovoltaic-thermal hybrid collector.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PvtCollector {
    pub area_m2: f64,
    pub pv_efficiency: f64,
    pub tau_alpha: f64,
    pub ul_w_m2k: f64,
    pub fluid_cp: f64,
}

impl PvtCollector {
    /// ⚡️☀️ Split incident solar into electrical and thermal output.
    pub fn simulate(&self, irradiance_w_m2: f64, ambient_c: f64, wind_m_s: f64, fluid_inlet_c: f64, mass_flow_kg_s: f64) -> (f64, f64) {
        let t_cell = fluid_inlet_c + irradiance_w_m2 * 0.02;
        let pv_eta = (self.pv_efficiency * (1.0 - 0.004 * (t_cell - 25.0))).max(0.05);
        let pv_w = self.area_m2 * irradiance_w_m2 * pv_eta;
        let thermal_irrad = irradiance_w_m2 * (1.0 - pv_eta);
        let thermal_w = collector_thermal_output_w(CollectorKind::PhotovoltaicThermal, self.area_m2, thermal_irrad, ambient_c, wind_m_s, fluid_inlet_c, mass_flow_kg_s, self.fluid_cp, self.tau_alpha, self.ul_w_m2k, 1.0, fluid_inlet_c);
        (pv_w, thermal_w)
    }
}
// #endregion 🔖️Pvt

// #region 🔖️Core
/// ☀️ Universal collector useful thermal output [W].
///
/// Implements Hottel-Whillier-Bliss with wind-adjusted loss coefficient:
/// `Q_u = A * [τα * G * IAM - U_L * (T_m - T_amb)]`.
pub fn collector_thermal_output_w(
    kind: CollectorKind,
    area_m2: f64,
    irradiance_w_m2: f64,
    ambient_c: f64,
    wind_m_s: f64,
    fluid_inlet_c: f64,
    mass_flow_kg_s: f64,
    fluid_cp: f64,
    tau_alpha: f64,
    ul_w_m2k: f64,
    iam: f64,
    reference_temperature_c: f64,
) -> f64 {
    let g = irradiance_w_m2.max(0.0);
    let wind = wind_m_s.max(0.1);
    let ul = match kind {
        CollectorKind::UnglazedTranspired => ul_w_m2k + 2.0 * wind,
        _ => ul_w_m2k + 0.5 * wind,
    };
    let t_m = if mass_flow_kg_s > 1e-6 {
        let f = (ul * area_m2 / (mass_flow_kg_s * fluid_cp)).min(50.0);
        fluid_inlet_c + g * tau_alpha * iam / (ul * (1.0 + f).max(1.0))
    } else {
        reference_temperature_c
    };
    let mut q_u = area_m2 * (tau_alpha * iam * g - ul * (t_m - ambient_c));
    if matches!(kind, CollectorKind::IntegralCollectorStorage) {
        let rad_loss = STEFAN_BOLTZMANN * area_m2 * ((t_m + 273.15).powi(4) - (ambient_c + 273.15).powi(4));
        q_u -= rad_loss * 0.1;
    }
    if matches!(kind, CollectorKind::UnglazedTranspired) {
        q_u *= tau_alpha;
    }
    q_u.max(0.0)
}
// #endregion 🔖️Core

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flat_plate_gain_positive_at_noon() {
        let collector = FlatPlateCollector { area_m2: 10.0, tau_alpha: 0.75, ul_w_m2k: 3.5, iam_factor: 0.95 };
        let gain = collector.useful_gain_w(800.0, 20.0, 2.0, 25.0, 0.2, 4180.0);
        assert!(gain > 0.0);
    }

    #[test]
    fn zero_irradiance_zero_gain() {
        let q = collector_thermal_output_w(CollectorKind::FlatPlate, 5.0, 0.0, 15.0, 1.0, 20.0, 0.1, 4180.0, 0.7, 4.0, 1.0, 20.0);
        assert!(q.abs() < 1.0);
    }

    #[test]
    fn ics_raises_storage_temperature() {
        let ics = IntegralCollectorStorage { area_m2: 3.0, storage_volume_l: 200.0, tau_alpha: 0.8, loss_coefficient_w_m2k: 5.0 };
        let (new_t, gain) = ics.simulate(25.0, 700.0, 18.0, 3600.0);
        assert!(gain > 0.0);
        assert!(new_t > 25.0);
    }

    #[test]
    fn unglazed_preheats_air() {
        let utc = UnglazedTranspiredCollector { area_m2: 50.0, porosity: 0.6, h_conv_w_m2k: 15.0, suction_velocity_m_s: 0.04 };
        let (gain, outlet_t) = utc.preheat_air_w(600.0, 5.0, 3.0);
        assert!(gain > 0.0);
        assert!(outlet_t > 5.0);
    }

    #[test]
    fn pvt_splits_electric_and_thermal() {
        let pvt = PvtCollector { area_m2: 8.0, pv_efficiency: 0.18, tau_alpha: 0.9, ul_w_m2k: 4.0, fluid_cp: 4180.0 };
        let (pv, thermal) = pvt.simulate(900.0, 22.0, 1.5, 30.0, 0.15);
        assert!(pv > 500.0);
        assert!(thermal >= 0.0);
    }
}
