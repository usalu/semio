//! 🧱️ Opaque envelope heat transfer: surface convection correlations, exterior long-wave
//! exchange, and implicit finite-difference conduction through layered constructions.
//!
//! Convection follows the EnergyPlus defaults (Engineering Reference, "Outside Surface Heat
//! Balance" and "Inside Heat Balance"): TARP/Walton natural convection on both faces, the DOE-2
//! forced correlation (MoWiTT smooth-surface coefficients scaled by the roughness multiplier)
//! outside, and wind at the surface centroid from the power-law boundary layer.

use crate::units::STEFAN_BOLTZMANN;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};
use serde::{Deserialize, Serialize};

// #region 🔖️Convection
/// 🌬️ Lower bound of any convection coefficient [W/(m²·K)].
pub const MINIMUM_CONVECTION_W_M2K: f64 = 0.1;
/// 🌬️ Wind-profile exponent and boundary-layer thickness [m] of open country, which is also the
/// exposure of the meteorological station a TMY record is measured at (10 m mast).
pub const COUNTRY_WIND_EXPONENT: f64 = 0.14;
pub const COUNTRY_BOUNDARY_LAYER_M: f64 = 270.0;
pub const WEATHER_STATION_HEIGHT_M: f64 = 10.0;

/// 🌬️ TARP natural convection [W/(m²·K)] for a surface `delta_k = T_surface − T_air` warmer than
/// the air whose outward normal has vertical component `cos_tilt` (pass the negated value for the
/// room side of a surface).
pub fn natural_convection_w_m2k(delta_k: f64, cos_tilt: f64) -> f64 {
    let magnitude = delta_k.abs().cbrt();
    if delta_k == 0.0 || cos_tilt == 0.0 {
        1.31 * magnitude
    } else if (delta_k < 0.0 && cos_tilt < 0.0) || (delta_k > 0.0 && cos_tilt > 0.0) {
        9.482 * magnitude / (7.238 - cos_tilt.abs())
    } else {
        1.810 * magnitude / (1.382 + cos_tilt.abs())
    }
}

/// 🌬️ Room-side convection coefficient of an opaque surface [W/(m²·K)].
pub fn interior_convection_w_m2k(surface_c: f64, air_c: f64, cos_tilt: f64) -> f64 {
    natural_convection_w_m2k(surface_c - air_c, -cos_tilt).max(MINIMUM_CONVECTION_W_M2K)
}

/// 🧭️ A surface faces into the wind when the wind blows within 90° of its outward normal; near
/// horizontal surfaces are always windward.
pub fn is_windward(cos_tilt: f64, azimuth_deg: f64, wind_direction_deg: f64) -> bool {
    if cos_tilt.abs() >= 0.98 {
        return true;
    }
    let mut difference = (wind_direction_deg - azimuth_deg).abs();
    if difference - 180.0 > 0.001 {
        difference -= 360.0;
    }
    difference.abs() - 90.0 <= 0.001
}

/// 🌬️ Wind speed [m/s] at `height_m` above ground from the station wind speed, for an open-country
/// site and an open-country station; zero at or below ground.
pub fn wind_speed_at_height(station_wind_m_s: f64, height_m: f64) -> f64 {
    if height_m <= 0.0 {
        return 0.0;
    }
    station_wind_m_s * (COUNTRY_BOUNDARY_LAYER_M / WEATHER_STATION_HEIGHT_M).powf(COUNTRY_WIND_EXPONENT) * (height_m / COUNTRY_BOUNDARY_LAYER_M).powf(COUNTRY_WIND_EXPONENT)
}

/// 🌬️ DOE-2 exterior convection [W/(m²·K)]: TARP natural convection plus the roughness-scaled
/// excess of the MoWiTT windward/leeward combined coefficient over its natural part.
pub fn exterior_convection_w_m2k(surface_c: f64, air_c: f64, cos_tilt: f64, wind_at_surface_m_s: f64, windward: bool, roughness_multiplier: f64) -> f64 {
    let natural = natural_convection_w_m2k(surface_c - air_c, cos_tilt);
    let smooth_forced = if windward { 3.26 * wind_at_surface_m_s.powf(0.89) } else { 3.55 * wind_at_surface_m_s.powf(0.617) };
    natural + roughness_multiplier * ((natural * natural + smooth_forced * smooth_forced).sqrt() - natural)
}

/// 🌌️ Linearized long-wave exchange coefficients [W/(m²·K)] of an exterior face at `surface_c`
/// with the sky, the air (the near-horizon part of the sky hemisphere) and the ground (taken at
/// air temperature): `(h_sky, h_air, h_ground)`.
pub fn exterior_radiation_w_m2k(surface_c: f64, air_c: f64, sky_c: f64, emissivity: f64, cos_tilt: f64) -> (f64, f64, f64) {
    let surface_k = surface_c + 273.15;
    let view_sky = 0.5 * (1.0 + cos_tilt);
    let view_ground = 0.5 * (1.0 - cos_tilt);
    let sky_split = view_sky.sqrt();
    let secant = |other_c: f64| {
        let other_k = other_c + 273.15;
        if (surface_k - other_k).abs() < 1e-9 {
            4.0 * surface_k.powi(3)
        } else {
            (surface_k.powi(4) - other_k.powi(4)) / (surface_k - other_k)
        }
    };
    let sigma_emissivity = STEFAN_BOLTZMANN * emissivity;
    (sigma_emissivity * view_sky * sky_split * secant(sky_c), sigma_emissivity * view_sky * (1.0 - sky_split) * secant(air_c), sigma_emissivity * view_ground * secant(air_c))
}
// #endregion 🔖️Convection

// #region 🔖️Conduction
/// 🧱️ One layer of an opaque construction, outside first.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ConductionLayer {
    pub thickness_m: f64,
    pub conductivity_w_m_k: f64,
    pub volumetric_heat_capacity_j_m3k: f64,
}

/// 🧱️ Finite-difference node chain of a construction per unit area: `conductance_w_m2k[k]`
/// couples node `k` and `k + 1`, `capacitance_j_m2k[k]` is node `k`'s lumped heat capacity. Node
/// `0` is the outside face, the last node the inside face.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct NodeChain {
    pub conductance_w_m2k: Vec<f64>,
    pub capacitance_j_m2k: Vec<f64>,
}

/// 🧱️ Most sub-layers any single material layer is divided into.
pub const MAXIMUM_SUBLAYERS: usize = 16;

/// 🧮️ Number of equal sub-layers a massive layer needs so each is no thicker than the thermal
/// penetration depth `√(α·Δt)` of one time step; massless layers stay one resistance.
pub fn sublayer_count(layer: &ConductionLayer, time_step_s: f64) -> usize {
    if layer.volumetric_heat_capacity_j_m3k <= 0.0 {
        return 1;
    }
    let penetration = (layer.conductivity_w_m_k / layer.volumetric_heat_capacity_j_m3k * time_step_s).sqrt();
    ((layer.thickness_m / penetration).ceil() as usize).clamp(1, MAXIMUM_SUBLAYERS)
}

impl NodeChain {
    /// 🔢️ Node count of `layers` discretized for `time_step_s`.
    pub fn node_count(layers: &[ConductionLayer], time_step_s: f64) -> usize {
        layers.iter().map(|layer| sublayer_count(layer, time_step_s)).sum::<usize>() + 1
    }

    /// 🧱️ Appends one layer's sub-layers to a chain whose node vectors are already reserved.
    pub fn push_layer(&mut self, layer: &ConductionLayer, time_step_s: f64) {
        if self.capacitance_j_m2k.is_empty() {
            self.capacitance_j_m2k.push(0.0);
        }
        let count = sublayer_count(layer, time_step_s);
        let thickness = layer.thickness_m / count as f64;
        let half_capacity = 0.5 * layer.volumetric_heat_capacity_j_m3k.max(0.0) * thickness;
        for _ in 0..count {
            self.conductance_w_m2k.push(layer.conductivity_w_m_k / thickness.max(1e-12));
            *self.capacitance_j_m2k.last_mut().expect("chain has a node") += half_capacity;
            self.capacitance_j_m2k.push(half_capacity);
        }
    }

    /// 🔥️ Steady-state face-to-face conductance [W/(m²·K)].
    pub fn conductance_w_m2k(&self) -> f64 {
        1.0 / self.conductance_w_m2k.iter().map(|g| 1.0 / g).sum::<f64>().max(1e-12)
    }

    /// 🔢️ Node count.
    pub fn nodes(&self) -> usize {
        self.capacitance_j_m2k.len()
    }
}

/// ➗️ Thomas forward elimination of an implicit chain from its outside face, returning for every
/// node but the inside face the relation `T_k = a_k + b_k·T_{k+1}` written into `a`/`b`.
///
/// The outside face exchanges `h_outside·(T_equivalent − T_0)` with its environment, written as the
/// linear source `q_outside = h_outside·T_equivalent`; `source(k)` is heat deposited at node `k`.
pub fn eliminate_chain(conductance: &[f64], capacitance: &[f64], previous: &[f64], source: impl Fn(usize) -> f64, time_step_s: f64, h_outside: f64, q_outside: f64, a: &mut [f64], b: &mut [f64]) {
    let nodes = capacitance.len();
    for k in 0..nodes - 1 {
        let storage = capacitance[k] / time_step_s;
        let (diagonal, rhs) = if k == 0 {
            (storage + h_outside + conductance[0], storage * previous[0] + q_outside + source(0))
        } else {
            (storage + conductance[k - 1] + conductance[k] - conductance[k - 1] * b[k - 1], storage * previous[k] + conductance[k - 1] * a[k - 1] + source(k))
        };
        a[k] = rhs / diagonal;
        b[k] = conductance[k] / diagonal;
    }
}

/// ➗️ Back substitution after [`eliminate_chain`] once the inside-face temperature is known.
pub fn substitute_chain(a: &[f64], b: &[f64], inside_c: f64, temperatures: &mut [f64]) {
    let nodes = temperatures.len();
    temperatures[nodes - 1] = inside_c;
    for k in (0..nodes - 1).rev() {
        temperatures[k] = a[k] + b[k] * temperatures[k + 1];
    }
}
// #endregion 🔖️Conduction

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
