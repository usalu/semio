//! 📐️ SI unit helpers and physical constants for BEM computations.

// #region 🔖️Constants
/// 🌡️ Standard atmospheric pressure [Pa].
pub const P_STD: f64 = 101_325.0;
/// 💨️ Dry air gas constant [J/(kg·K)].
pub const R_DRY_AIR: f64 = 287.055;
/// 💧️ Water vapor gas constant [J/(kg·K)].
pub const R_WATER_VAPOR: f64 = 461.52;
/// 🌡️ Triple-point temperature of water [K].
pub const T_TRIPLE_WATER: f64 = 273.16;
/// 🔥️ Specific heat of dry air at constant pressure [J/(kg·K)].
pub const CP_DRY_AIR: f64 = 1006.0;
/// 🔥️ Latent heat of vaporization at 0°C [J/kg].
pub const H_FG_0C: f64 = 2_501_000.0;
/// 💧️ Density of water [kg/m³].
pub const RHO_WATER: f64 = 998.0;
/// 🧊️ Density of dry air at reference [kg/m³].
pub const RHO_AIR_REF: f64 = 1.2;
/// ⚡️ Stefan-Boltzmann constant [W/(m²·K⁴)].
pub const STEFAN_BOLTZMANN: f64 = 5.670_374_419e-8;
/// 🌍️ Standard gravity [m/s²].
pub const GRAVITY: f64 = 9.806_65;
// #endregion 🔖️Constants

// #region 🔖️Conversions
/// 🌡️ Celsius to Kelvin.
pub fn c_to_k(t_c: f64) -> f64 {
    t_c + 273.15
}

/// 🌡️ Kelvin to Celsius.
pub fn k_to_c(t_k: f64) -> f64 {
    t_k - 273.15
}

/// 📐️ Degrees to radians.
pub fn deg_to_rad(deg: f64) -> f64 {
    deg * std::f64::consts::PI / 180.0
}

/// 📐️ Radians to degrees.
pub fn rad_to_deg(rad: f64) -> f64 {
    rad * 180.0 / std::f64::consts::PI
}
// #endregion 🔖️Conversions

// #region 🔖️Quantity
/// 📊️ Tagged SI scalar for results and limits.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Unit {
    Dimensionless,
    Meters,
    SquareMeters,
    CubicMeters,
    Kelvin,
    Celsius,
    Pascals,
    Watts,
    Joules,
    KilogramsPerSecond,
    CubicMetersPerSecond,
    KilowattHours,
    HumidityRatio,
    Percent,
}

/// 📏️ Physical quantity with unit tag.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Quantity {
    pub unit: Unit,
    pub value: f64,
}

impl Quantity {
    pub const fn new(unit: Unit, value: f64) -> Self {
        Self { unit, value }
    }

    pub fn watts(v: f64) -> Self {
        Self::new(Unit::Watts, v)
    }

    pub fn joules(v: f64) -> Self {
        Self::new(Unit::Joules, v)
    }

    pub fn celsius(v: f64) -> Self {
        Self::new(Unit::Celsius, v)
    }
}
// #endregion 🔖️Quantity

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn celsius_kelvin_roundtrip() {
        assert!((k_to_c(c_to_k(20.0)) - 20.0).abs() < 1e-9);
    }
}
