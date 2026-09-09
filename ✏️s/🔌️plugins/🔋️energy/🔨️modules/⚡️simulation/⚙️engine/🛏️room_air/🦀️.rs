//! 🌀️ Room air distribution models: mixed, stratified, displacement, UFAD, surface-specific.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};
use serde::{Deserialize, Serialize};

// #region 🔖️RoomAirInput
/// 📥️ Inputs for room air model evaluation.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct RoomAirInput {
    pub zone_temp_c: f64,
    pub supply_temp_c: f64,
    pub outdoor_temp_c: f64,
    pub floor_area_m2: f64,
    pub ceiling_height_m: f64,
    pub supply_flow_m3_s: f64,
    pub internal_gain_w: f64,
    pub surface_temps_c: [f64; 6],
}
// #endregion 🔖️RoomAirInput

// #region 🔖️RoomAirOutput
/// 📤️ Room air model temperatures [°C].
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct RoomAirOutput {
    pub occupied_temp_c: f64,
    pub return_temp_c: f64,
    pub exhaust_temp_c: f64,
    pub floor_temp_c: f64,
    pub ceiling_temp_c: f64,
    pub surface_air_temps_c: [f64; 6],
}
// #endregion 🔖️RoomAirOutput

// #region 🔖️RoomAirModel
/// 🌀️ Room air distribution model per ASHRAE / ISO 7730 room air classifications.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub enum RoomAirModel {
    FullyMixed,
    VerticalGradient { gradient_k_per_m: f64 },
    Displacement1Node { mixing_factor: f64 },
    Displacement3Node { lower_fraction: f64, upper_fraction: f64 },
    Ufad { diffuser_height_m: f64, throw_m: f64 },
    SurfaceSpecific,
}

impl RoomAirModel {
    /// 🌡️ Apply room air model and return stratified temperatures.
    pub fn apply(&self, input: &RoomAirInput) -> RoomAirOutput {
        match self {
            Self::FullyMixed => fully_mixed(input),
            Self::VerticalGradient { gradient_k_per_m } => vertical_gradient(input, *gradient_k_per_m),
            Self::Displacement1Node { mixing_factor } => displacement_1node(input, *mixing_factor),
            Self::Displacement3Node { lower_fraction, upper_fraction } => displacement_3node(input, *lower_fraction, *upper_fraction),
            Self::Ufad { diffuser_height_m, throw_m } => ufad(input, *diffuser_height_m, *throw_m),
            Self::SurfaceSpecific => surface_specific(input),
        }
    }
}
// #endregion 🔖️RoomAirModel

// #region 🔖️FullyMixed
fn fully_mixed(input: &RoomAirInput) -> RoomAirOutput {
    let t = input.zone_temp_c;
    RoomAirOutput { occupied_temp_c: t, return_temp_c: t, exhaust_temp_c: t, floor_temp_c: t, ceiling_temp_c: t, surface_air_temps_c: [t; 6] }
}
// #endregion 🔖️FullyMixed

// #region 🔖️VerticalGradient
fn vertical_gradient(input: &RoomAirInput, gradient_k_per_m: f64) -> RoomAirOutput {
    let h = input.ceiling_height_m.max(0.1);
    let t_floor = input.zone_temp_c - gradient_k_per_m * 0.1;
    let t_ceil = input.zone_temp_c + gradient_k_per_m * (h - 0.1);
    let t_occ = input.zone_temp_c;
    RoomAirOutput { occupied_temp_c: t_occ, return_temp_c: t_ceil, exhaust_temp_c: t_ceil, floor_temp_c: t_floor, ceiling_temp_c: t_ceil, surface_air_temps_c: [t_floor, input.zone_temp_c, t_ceil, input.zone_temp_c, t_floor, t_ceil] }
}
// #endregion 🔖️VerticalGradient

// #region 🔖️Displacement
fn displacement_1node(input: &RoomAirInput, mixing_factor: f64) -> RoomAirOutput {
    let f = mixing_factor.clamp(0.0, 1.0);
    let t_supply = input.supply_temp_c;
    let t_zone = input.zone_temp_c;
    let t_occ = f * t_zone + (1.0 - f) * t_supply;
    let t_return = t_zone + f * (t_zone - t_supply) * 0.3;
    RoomAirOutput { occupied_temp_c: t_occ, return_temp_c: t_return, exhaust_temp_c: t_return, floor_temp_c: t_supply + 0.5 * (t_occ - t_supply), ceiling_temp_c: t_return, surface_air_temps_c: [t_occ; 6] }
}

fn displacement_3node(input: &RoomAirInput, lower_fraction: f64, upper_fraction: f64) -> RoomAirOutput {
    let lf = lower_fraction.clamp(0.05, 0.95);
    let uf = upper_fraction.clamp(0.05, 0.95);
    let h = input.ceiling_height_m.max(0.1);
    let z_occ = h * 0.4;
    let z_lower = h * lf;
    let z_upper = h * uf;
    let t_supply = input.supply_temp_c;
    let t_zone = input.zone_temp_c;
    let grad = (t_zone - t_supply) / h;
    let t_lower = t_supply + grad * z_lower * 0.5;
    let t_occ = t_supply + grad * z_occ;
    let t_upper = t_supply + grad * z_upper;
    RoomAirOutput { occupied_temp_c: t_occ, return_temp_c: t_upper, exhaust_temp_c: t_upper, floor_temp_c: t_lower, ceiling_temp_c: t_upper, surface_air_temps_c: [t_lower, t_occ, t_upper, t_occ, t_lower, t_upper] }
}
// #endregion 🔖️Displacement

// #region 🔖️Ufad
fn ufad(input: &RoomAirInput, diffuser_height_m: f64, throw_m: f64) -> RoomAirOutput {
    let h = input.ceiling_height_m.max(0.1);
    let _z_diff = diffuser_height_m.clamp(0.05, h * 0.5);
    let throw = throw_m.max(0.1);
    let penetration = (throw / h).clamp(0.1, 1.0);
    let t_supply = input.supply_temp_c;
    let t_zone = input.zone_temp_c;
    let t_occ = t_supply + penetration * (t_zone - t_supply);
    let t_return = t_zone + (1.0 - penetration) * 0.2 * (t_zone - t_supply);
    let t_floor = t_supply + 0.3 * (t_occ - t_supply);
    RoomAirOutput { occupied_temp_c: t_occ, return_temp_c: t_return, exhaust_temp_c: t_return, floor_temp_c: t_floor, ceiling_temp_c: t_return + 0.5 * (t_zone - t_occ), surface_air_temps_c: [t_floor, t_occ, t_return, t_occ, t_floor, t_return] }
}
// #endregion 🔖️Ufad

// #region 🔖️SurfaceSpecific
fn surface_specific(input: &RoomAirInput) -> RoomAirOutput {
    let mut surface_air = input.surface_temps_c;
    for (i, &t_surf) in input.surface_temps_c.iter().enumerate() {
        surface_air[i] = 0.7 * input.zone_temp_c + 0.3 * t_surf;
    }
    let t_occ = surface_air.iter().sum::<f64>() / surface_air.len() as f64;
    RoomAirOutput { occupied_temp_c: t_occ, return_temp_c: input.zone_temp_c, exhaust_temp_c: input.zone_temp_c, floor_temp_c: surface_air[0], ceiling_temp_c: surface_air[2], surface_air_temps_c: surface_air }
}
// #endregion 🔖️SurfaceSpecific

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
