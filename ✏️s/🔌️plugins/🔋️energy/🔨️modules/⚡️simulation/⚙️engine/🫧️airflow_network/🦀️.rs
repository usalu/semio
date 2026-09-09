//! 🌐️ Pressure-driven multizone airflow network with stack and wind effects.

use crate::props::moist_air_density;
use crate::units::{GRAVITY, RHO_AIR_REF};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};
use serde::{Deserialize, Serialize};

// #region 🔖️AfNode
/// 🔵️ Airflow network node (zone or outdoor reference).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct AfNode {
    pub id: u32,
    pub elevation_m: f64,
    pub temperature_c: f64,
    pub humidity_ratio: f64,
    pub is_reference: bool,
}

impl AfNode {
    pub fn density(&self, p_atm: f64) -> f64 {
        moist_air_density(self.temperature_c, self.humidity_ratio, p_atm)
    }
}
// #endregion 🔖️AfNode

// #region 🔖️AfLinkKind
/// 🔗️ Airflow link type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub enum AfLinkKind {
    Crack,
    Opening,
    Door,
    Duct,
}
// #endregion 🔖️AfLinkKind

// #region 🔖️AfLink
/// ↔ Pressure-flow link between two nodes (power-law Q = C·|ΔP|ⁿ).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct AfLink {
    pub id: u32,
    pub node_a: u32,
    pub node_b: u32,
    pub kind: AfLinkKind,
    pub flow_coefficient: f64,
    pub flow_exponent: f64,
    pub area_m2: f64,
    pub discharge_coefficient: f64,
    pub orientation_deg: f64,
    pub wind_exposure_factor: f64,
}
// #endregion 🔖️AfLink

// #region 🔖️AirflowNetwork
/// 🌐️ Multizone airflow network.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct AirflowNetwork {
    pub nodes: Vec<AfNode>,
    pub links: Vec<AfLink>,
    pub wind_speed_m_s: f64,
    pub wind_direction_deg: f64,
    pub outdoor_temp_c: f64,
    pub outdoor_humidity_ratio: f64,
}

impl AirflowNetwork {
    pub fn node_index(&self, id: u32) -> Option<usize> {
        self.nodes.iter().position(|n| n.id == id)
    }

    /// 🌬️ Volumetric flow [m³/s] through link from node_a toward node_b (positive = a→b).
    pub fn link_flow_m3_s(&self, link: &AfLink, pressures_pa: &[f64], p_atm: f64) -> f64 {
        let ia = self.node_index(link.node_a).unwrap_or(0);
        let ib = self.node_index(link.node_b).unwrap_or(0);
        let node_a = &self.nodes[ia];
        let node_b = &self.nodes[ib];
        let dp_stack = stack_pressure_pa(node_a, node_b, p_atm);
        let dp_wind = wind_pressure_pa(link, self.wind_speed_m_s, self.wind_direction_deg);
        let dp = pressures_pa[ia] - pressures_pa[ib] + dp_stack + dp_wind;
        power_law_flow(link, dp, node_a.density(p_atm))
    }

    /// 🔍️ Solve zone pressures [Pa] relative to reference node via Gauss-Seidel mass balance.
    pub fn solve_pressures(&self, p_atm: f64, max_iter: usize, tol: f64) -> Option<Vec<f64>> {
        let n = self.nodes.len();
        if n == 0 {
            return Some(Vec::new());
        }
        let ref_idx = self.nodes.iter().position(|node| node.is_reference)?;
        let mut pressures = vec![0.0; n];
        pressures[ref_idx] = 0.0;

        for _ in 0..max_iter {
            let mut max_delta = 0.0_f64;
            for i in 0..n {
                if i == ref_idx {
                    continue;
                }
                let (sum_q, sum_g) = node_mass_balance(self, i, &pressures, p_atm);
                if sum_g.abs() < 1e-12 {
                    continue;
                }
                let dp = -sum_q / sum_g;
                let new_p = pressures[i] + dp;
                max_delta = max_delta.max((new_p - pressures[i]).abs());
                pressures[i] = new_p;
            }
            if max_delta < tol {
                return Some(pressures);
            }
        }
        Some(pressures)
    }

    /// 📊️ Flow rates [m³/s] for all links after pressure solve.
    pub fn solve_flows(&self, p_atm: f64) -> Option<Vec<f64>> {
        let pressures = self.solve_pressures(p_atm, 200, 1e-4)?;
        Some(self.links.iter().map(|link| self.link_flow_m3_s(link, &pressures, p_atm)).collect())
    }
}
// #endregion 🔖️AirflowNetwork

// #region 🔖️Physics
fn stack_pressure_pa(node_a: &AfNode, node_b: &AfNode, p_atm: f64) -> f64 {
    let rho_a = node_a.density(p_atm);
    let rho_b = node_b.density(p_atm);
    GRAVITY * (node_a.elevation_m - node_b.elevation_m) * (rho_a - rho_b) * 0.5
}

fn wind_pressure_pa(link: &AfLink, wind_speed_m_s: f64, wind_direction_deg: f64) -> f64 {
    let angle = (wind_direction_deg - link.orientation_deg).to_radians();
    let cp = angle.cos();
    0.5 * RHO_AIR_REF * wind_speed_m_s * wind_speed_m_s * cp * link.wind_exposure_factor
}

fn power_law_flow(link: &AfLink, dp_pa: f64, rho: f64) -> f64 {
    let n = link.flow_exponent.clamp(0.5, 1.0);
    let c = link.flow_coefficient.max(1e-12);
    let sign = if dp_pa >= 0.0 { 1.0 } else { -1.0 };
    sign * c * dp_pa.abs().powf(n) / rho.sqrt()
}

fn link_conductance(link: &AfLink, dp_pa: f64, rho: f64) -> f64 {
    let n = link.flow_exponent.clamp(0.5, 1.0);
    let c = link.flow_coefficient.max(1e-12);
    if dp_pa.abs() < 1e-6 {
        n * c / rho.sqrt()
    } else {
        n * c * dp_pa.abs().powf(n - 1.0) / rho.sqrt()
    }
}

fn node_mass_balance(network: &AirflowNetwork, node_i: usize, pressures: &[f64], p_atm: f64) -> (f64, f64) {
    let mut sum_q = 0.0;
    let mut sum_g = 0.0;
    for link in &network.links {
        let ia = network.node_index(link.node_a).unwrap_or(0);
        let ib = network.node_index(link.node_b).unwrap_or(0);
        if ia != node_i && ib != node_i {
            continue;
        }
        let node_a = &network.nodes[ia];
        let node_b = &network.nodes[ib];
        let dp_stack = stack_pressure_pa(node_a, node_b, p_atm);
        let dp_wind = wind_pressure_pa(link, network.wind_speed_m_s, network.wind_direction_deg);
        let dp = pressures[ia] - pressures[ib] + dp_stack + dp_wind;
        let rho = node_a.density(p_atm);
        let g = link_conductance(link, dp, rho);
        let q = power_law_flow(link, dp, rho);
        if node_i == ia {
            sum_q -= q;
            sum_g -= g;
        }
        if node_i == ib {
            sum_q += q;
            sum_g += g;
        }
    }
    (sum_q, sum_g)
}
// #endregion 🔖️Physics

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
