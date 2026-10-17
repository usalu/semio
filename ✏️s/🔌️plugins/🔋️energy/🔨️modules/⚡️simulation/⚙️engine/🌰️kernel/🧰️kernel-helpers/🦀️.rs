//! 🧰️ The delivered energy accumulation, state pane source, inside face temperature and default weather helpers.

use super::*;

pub(super) fn accumulate_delivered(total: &DeliveredEnergy, step: &DeliveredEnergy) -> DeliveredEnergy {
    DeliveredEnergy {
        heating_w: total.heating_w + step.heating_w,
        cooling_w: total.cooling_w + step.cooling_w,
        fan_w: total.fan_w + step.fan_w,
        pump_w: total.pump_w + step.pump_w,
        compressor_w: total.compressor_w + step.compressor_w,
        gas_w: total.gas_w + step.gas_w,
        pv_generation_w: total.pv_generation_w + step.pv_generation_w,
        battery_charge_w: total.battery_charge_w + step.battery_charge_w,
        shw_electric_w: total.shw_electric_w + step.shw_electric_w,
        shw_gas_w: total.shw_gas_w + step.shw_gas_w,
        refrigeration_w: total.refrigeration_w + step.refrigeration_w,
        water_pump_w: total.water_pump_w + step.water_pump_w,
    }
}

pub(super) fn state_pane_source(pane_absorbed_w_m2: &[f64], face: usize, node: usize) -> f64 {
    0.5 * pane_absorbed_w_m2[face * MAX_PANES + node / 2]
}

pub(super) fn face_inside_c(state: &SimulationModel, pre: &PrecomputedModel, face: EnclosureFace, fallback_c: f64) -> f64 {
    match face {
        EnclosureFace::Opaque(id) => state.surfaces.get(&id).map_or(fallback_c, SurfaceState::inside_temp_c),
        EnclosureFace::Window(id) => match (state.windows.get(&id), pre.windows.get(&id)) {
            (Some(window_state), Some(window)) => window_state.face_temperatures_c[window.glazing.faces() - 1],
            _ => fallback_c,
        },
    }
}

#[cfg(test)]
pub(super) fn default_weather(hour: u32) -> WeatherRecord {
    WeatherRecord {
        year: 2026,
        month: 1,
        day: 1,
        hour: (hour % 24) as u8,
        minute: 0,
        dry_bulb_c: 20.0,
        dew_point_c: 10.0,
        relative_humidity: 0.5,
        atmospheric_pressure_pa: 101_325.0,
        wind_speed_m_s: 2.0,
        wind_direction_deg: 0.0,
        direct_normal_irradiance_w_m2: 0.0,
        diffuse_horizontal_irradiance_w_m2: 0.0,
        horizontal_infrared_w_m2: 250.0,
        precipitation_mm: 0.0,
        snow_depth_mm: 0.0,
    }
}
