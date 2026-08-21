//! 🎛️ Zone controls: thermostats, humidistats, load prediction, equipment priority.

use serde::{Deserialize, Serialize};

// #region 🔖️ZoneLoad
/// 📊️ Predicted zone heating/cooling/humidification loads [W].
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ZoneLoad {
    pub heating_w: f64,
    pub cooling_w: f64,
    pub humidifying_w: f64,
    pub dehumidifying_w: f64,
    pub sensible_w: f64,
    pub latent_w: f64,
}

impl ZoneLoad {
    pub fn total_w(&self) -> f64 {
        self.heating_w + self.cooling_w + self.humidifying_w + self.dehumidifying_w
    }

    pub fn net_sensible_w(&self) -> f64 {
        self.heating_w - self.cooling_w + self.sensible_w
    }

    pub fn net_latent_w(&self) -> f64 {
        self.humidifying_w - self.dehumidifying_w + self.latent_w
    }
}
// #endregion 🔖️ZoneLoad

// #region 🔖️ControlAction
/// 🎛️ HVAC control action requested by zone controller.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ControlAction {
    NoAction,
    Heat { power_w: f64 },
    Cool { power_w: f64 },
    Humidify { power_w: f64 },
    Dehumidify { power_w: f64 },
    Ventilate { flow_m3_s: f64 },
}
// #endregion 🔖️ControlAction

// #region 🔖️ThermostatOutput
/// 🌡️ Thermostat and humidistat combined output.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThermostatOutput {
    pub heating_fraction: f64,
    pub cooling_fraction: f64,
    pub humidifying_fraction: f64,
    pub dehumidifying_fraction: f64,
    pub heating_setpoint_c: f64,
    pub cooling_setpoint_c: f64,
    pub humidifying_setpoint_rh: f64,
    pub dehumidifying_setpoint_rh: f64,
}
// #endregion 🔖️ThermostatOutput

// #region 🔖️ThermostatSpec
/// 🌡️ Proportional thermostat with throttle ranges [K].
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThermostatSpec {
    pub heating_setpoint_c: f64,
    pub cooling_setpoint_c: f64,
    pub heating_throttle_range_k: f64,
    pub cooling_throttle_range_k: f64,
    pub min_heating_setpoint_c: f64,
    pub max_cooling_setpoint_c: f64,
}
// #endregion 🔖️ThermostatSpec

// #region 🔖️HumidistatSpec
/// 💧️ Humidistat with RH setpoints and throttle ranges.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct HumidistatSpec {
    pub humidifying_setpoint_rh: f64,
    pub dehumidifying_setpoint_rh: f64,
    pub humidifying_throttle_range: f64,
    pub dehumidifying_throttle_range: f64,
}
// #endregion 🔖️HumidistatSpec

// #region 🔖️ZoneEquipmentPriority
/// 🏆️ Equipment serving priority for load allocation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ZoneEquipmentPriority(pub u8);
// #endregion 🔖️ZoneEquipmentPriority

// #region 🔖️Thermostat
fn proportional_fraction(error: f64, throttle: f64) -> f64 {
    if error <= 0.0 || throttle <= 0.0 {
        return 0.0;
    }
    (error / throttle).clamp(0.0, 1.0)
}

/// 🌡️ Evaluate thermostat and humidistat for current zone conditions.
pub fn evaluate_controls(thermostat: &ThermostatSpec, humidistat: Option<&HumidistatSpec>, zone_temp_c: f64, zone_rh: f64) -> ThermostatOutput {
    let heat_err = thermostat.heating_setpoint_c - zone_temp_c;
    let cool_err = zone_temp_c - thermostat.cooling_setpoint_c;
    let heating_fraction = proportional_fraction(heat_err, thermostat.heating_throttle_range_k);
    let cooling_fraction = proportional_fraction(cool_err, thermostat.cooling_throttle_range_k);

    let (humid_frac, dehumid_frac, hum_sp, dehum_sp) = if let Some(h) = humidistat {
        let hum_err = h.humidifying_setpoint_rh - zone_rh;
        let dehum_err = zone_rh - h.dehumidifying_setpoint_rh;
        (proportional_fraction(hum_err, h.humidifying_throttle_range), proportional_fraction(dehum_err, h.dehumidifying_throttle_range), h.humidifying_setpoint_rh, h.dehumidifying_setpoint_rh)
    } else {
        (0.0, 0.0, 0.0, 1.0)
    };

    ThermostatOutput {
        heating_fraction,
        cooling_fraction,
        humidifying_fraction: humid_frac,
        dehumidifying_fraction: dehumid_frac,
        heating_setpoint_c: thermostat.heating_setpoint_c.max(thermostat.min_heating_setpoint_c),
        cooling_setpoint_c: thermostat.cooling_setpoint_c.min(thermostat.max_cooling_setpoint_c),
        humidifying_setpoint_rh: hum_sp,
        dehumidifying_setpoint_rh: dehum_sp,
    }
}
// #endregion 🔖️Thermostat

// #region 🔖️LoadPrediction
/// 📈️ Predict zone loads from balance residuals and control fractions.
pub fn predict_zone_load(sensible_residual_w: f64, latent_residual_w: f64, output: &ThermostatOutput, max_heating_w: f64, max_cooling_w: f64, max_humidifying_w: f64, max_dehumidifying_w: f64) -> ZoneLoad {
    let mut load = ZoneLoad { sensible_w: sensible_residual_w, latent_w: latent_residual_w, ..Default::default() };

    if sensible_residual_w < 0.0 {
        load.heating_w = (-sensible_residual_w * output.heating_fraction).min(max_heating_w);
    } else if sensible_residual_w > 0.0 {
        load.cooling_w = (sensible_residual_w * output.cooling_fraction).min(max_cooling_w);
    }

    if latent_residual_w < 0.0 {
        load.humidifying_w = (-latent_residual_w * output.humidifying_fraction).min(max_humidifying_w);
    } else if latent_residual_w > 0.0 {
        load.dehumidifying_w = (latent_residual_w * output.dehumidifying_fraction).min(max_dehumidifying_w);
    }

    load
}
// #endregion 🔖️LoadPrediction

// #region 🔖️ActionMapping
/// 🎛️ Map zone load to prioritized control actions.
pub fn load_to_actions(load: &ZoneLoad, ventilation_flow_m3_s: f64) -> Vec<ControlAction> {
    let mut actions = Vec::new();
    if load.heating_w > 0.0 {
        actions.push(ControlAction::Heat { power_w: load.heating_w });
    }
    if load.cooling_w > 0.0 {
        actions.push(ControlAction::Cool { power_w: load.cooling_w });
    }
    if load.humidifying_w > 0.0 {
        actions.push(ControlAction::Humidify { power_w: load.humidifying_w });
    }
    if load.dehumidifying_w > 0.0 {
        actions.push(ControlAction::Dehumidify { power_w: load.dehumidifying_w });
    }
    if ventilation_flow_m3_s > 0.0 {
        actions.push(ControlAction::Ventilate { flow_m3_s: ventilation_flow_m3_s });
    }
    if actions.is_empty() {
        actions.push(ControlAction::NoAction);
    }
    actions
}
// #endregion 🔖️ActionMapping

// #region 🔖️EquipmentAllocation
/// 🏆️ Allocate zone load across equipment by priority until capacity exhausted.
pub fn allocate_load_by_priority(load: ZoneLoad, capacities_w: &[(ZoneEquipmentPriority, f64)]) -> Vec<(ZoneEquipmentPriority, ZoneLoad)> {
    let mut sorted: Vec<_> = capacities_w.to_vec();
    sorted.sort_by_key(|(p, _)| *p);
    let mut remaining = load;
    let mut result = Vec::new();

    for (priority, capacity) in sorted {
        if remaining.total_w() <= 0.0 {
            break;
        }
        let frac = (remaining.total_w() / load.total_w().max(1.0)).min(1.0);
        let alloc = ZoneLoad {
            heating_w: remaining.heating_w.min(capacity * frac),
            cooling_w: remaining.cooling_w.min(capacity * frac),
            humidifying_w: remaining.humidifying_w.min(capacity * frac * 0.5),
            dehumidifying_w: remaining.dehumidifying_w.min(capacity * frac * 0.5),
            sensible_w: remaining.sensible_w * frac,
            latent_w: remaining.latent_w * frac,
        };
        remaining.heating_w -= alloc.heating_w;
        remaining.cooling_w -= alloc.cooling_w;
        remaining.humidifying_w -= alloc.humidifying_w;
        remaining.dehumidifying_w -= alloc.dehumidifying_w;
        result.push((priority, alloc));
    }
    result
}
// #endregion 🔖️EquipmentAllocation

#[cfg(test)]
mod tests {
    use super::*;

    fn default_thermostat() -> ThermostatSpec {
        ThermostatSpec { heating_setpoint_c: 21.0, cooling_setpoint_c: 24.0, heating_throttle_range_k: 2.0, cooling_throttle_range_k: 2.0, min_heating_setpoint_c: 10.0, max_cooling_setpoint_c: 30.0 }
    }

    #[semio_framework_async_macros::async_test]
    fn heating_fraction_full_when_cold() {
        let out = evaluate_controls(&default_thermostat(), None, 18.0, 0.5);
        assert!((out.heating_fraction - 1.0).abs() < 1e-9);
        assert!((out.cooling_fraction).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    fn cooling_fraction_when_warm() {
        let out = evaluate_controls(&default_thermostat(), None, 26.0, 0.5);
        assert!((out.cooling_fraction - 1.0).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    fn predict_heating_load_when_negative_residual() {
        let out = evaluate_controls(&default_thermostat(), None, 19.0, 0.5);
        let load = predict_zone_load(-3000.0, 0.0, &out, 5000.0, 5000.0, 1000.0, 1000.0);
        assert!(load.heating_w > 0.0);
        assert!(load.heating_w <= 5000.0);
    }

    #[semio_framework_async_macros::async_test]
    fn equipment_priority_allocates_in_order() {
        let load = ZoneLoad { heating_w: 8000.0, ..Default::default() };
        let caps = [(ZoneEquipmentPriority(1), 3000.0), (ZoneEquipmentPriority(2), 5000.0)];
        let alloc = allocate_load_by_priority(load, &caps);
        assert_eq!(alloc.len(), 2);
        assert!((alloc[0].1.heating_w - 3000.0).abs() < 1e-6);
    }
}
