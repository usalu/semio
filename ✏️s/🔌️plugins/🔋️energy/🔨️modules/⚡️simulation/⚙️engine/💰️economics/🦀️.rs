//! 💰️ Utility tariffs and life-cycle costing (non-physics post-pass).

use crate::meters::FuelType;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

// #region 🔖️Tariff
/// 💰️ Time-of-use period.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct TouPeriod {
    pub name: String,
    pub start_hour: u8,
    pub end_hour: u8,
    pub months: Vec<u8>,
    pub energy_rate_per_kwh: f64,
    pub demand_rate_per_kw: f64,
}

/// 💰️ Utility tariff definition.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct UtilityTariff {
    pub name: String,
    pub fuel: FuelType,
    pub periods: Vec<TouPeriod>,
    pub fixed_monthly_charge: f64,
    pub ratchet_percent: f64,
}

// #endregion 🔖️Tariff

// #region 🔖️Lcca
/// 💰️ Life-cycle cost parameters.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct LccaParameters {
    pub study_period_years: u32,
    pub discount_rate: f64,
    pub inflation_rate: f64,
    pub initial_cost: f64,
    pub annual_maintenance: f64,
    pub replacement_cost: f64,
    pub replacement_interval_years: u32,
}

/// 💰️ Life-cycle cost result.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct LccaResult {
    pub present_value_energy: f64,
    pub present_value_maintenance: f64,
    pub present_value_total: f64,
    pub simple_payback_years: f64,
}

/// 💰️ Compute present value of annual cost over study period.
#[cfg(test)]
pub(crate) fn present_value(annual_cost: f64, discount_rate: f64, years: u32) -> f64 {
    let mut pv = 0.0;
    for y in 1..=years {
        pv += annual_cost / (1.0 + discount_rate).powi(y as i32);
    }
    pv
}

/// 💰️ Run LCCA from annual energy cost and parameters.
#[cfg(test)]
pub(crate) fn compute_lcca(annual_energy_cost: f64, params: &LccaParameters) -> LccaResult {
    let pv_energy = present_value(annual_energy_cost, params.discount_rate, params.study_period_years);
    let pv_maint = present_value(params.annual_maintenance, params.discount_rate, params.study_period_years);
    let pv_total = params.initial_cost + pv_energy + pv_maint;
    let simple_payback = if annual_energy_cost > 0.0 { params.initial_cost / annual_energy_cost } else { f64::INFINITY };
    LccaResult { present_value_energy: pv_energy, present_value_maintenance: pv_maint, present_value_total: pv_total, simple_payback_years: simple_payback }
}
// #endregion 🔖️Lcca

// #region 🔖️Economics
/// 💰️ Economics post-pass over meter results.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct EconomicsResult {
    pub annual_energy_cost: f64,
    pub annual_demand_cost: f64,
    pub lcca: Option<LccaResult>,
}

// #endregion 🔖️Economics

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
