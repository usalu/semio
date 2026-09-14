//! 🪟️ Fenestration physics: angular glazing optics, the simple-glazing equivalent layer, gap and
//! room-side film conductances, rated-U coefficient adjustment, and condensation checks.
//!
//! The optical and thermal model is the EnergyPlus "detailed window" model (Engineering Reference,
//! "Window Calculation Module"): every pane's angular transmittance and reflectance follow from its
//! normal-incidence values through Fresnel optics of an equivalent uncoated sheet, panes combine
//! through multiple inter-reflection, system properties are fitted by a sixth-order polynomial in
//! the incidence cosine, and hemispherical values are the Simpson integral of the angular curve.

use crate::model::GasKind;
use crate::props::{dew_point_c, moist_air_cp_j_per_kg_k, moist_air_density, saturation_pressure_pa};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};
use serde::{Deserialize, Serialize};

// #region 🔖️Types
/// 🪟️ Largest glazing stack the window heat balance solves.
pub const MAX_PANES: usize = 4;
const ANGLES: usize = 10;
const FIT_ORDER: usize = 6;

/// 🪟️ One glass pane at normal incidence.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pane {
    pub thickness_m: f64,
    pub conductivity_w_m_k: f64,
    pub solar_transmittance: f64,
    pub solar_reflectance_front: f64,
    pub solar_reflectance_back: f64,
    pub infrared_transmittance: f64,
    pub infrared_emissivity_front: f64,
    pub infrared_emissivity_back: f64,
}

/// 🌫️ One gas-filled gap between two panes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Gap {
    pub width_m: f64,
    pub gas: GasKind,
}

/// 🪟️ Precomputed optical and thermal description of one glazing system, outside pane first.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct GlazingSystem {
    pub panes: usize,
    pub pane_conductance_w_m2k: [f64; MAX_PANES],
    pub emissivity_front: [f64; MAX_PANES],
    pub emissivity_back: [f64; MAX_PANES],
    pub gap_width_m: [f64; MAX_PANES],
    pub gap_gas: [GasKind; MAX_PANES],
    pub transmittance_fit: [f64; FIT_ORDER],
    pub front_absorptance_fit: [[f64; FIT_ORDER]; MAX_PANES],
    pub back_absorptance_fit: [[f64; FIT_ORDER]; MAX_PANES],
    pub back_transmittance_fit: [f64; FIT_ORDER],
    pub diffuse_transmittance: f64,
    pub diffuse_back_reflectance: f64,
    pub diffuse_front_absorptance: [f64; MAX_PANES],
    pub diffuse_back_absorptance: [f64; MAX_PANES],
    pub rated_u_w_m2k: Option<f64>,
}

/// 🧪️ Simple-glazing inputs converted to their equivalent single pane.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SimpleGlazingLayer {
    pub pane: Pane,
    pub u_value_w_m2k: f64,
    pub shgc: f64,
}
// #endregion 🔖️Types

// #region 🔖️Angular
fn incidence_cosines() -> [f64; ANGLES] {
    std::array::from_fn(|index| if index == 0 { 1.0 } else { (index as f64 * 10.0).to_radians().cos() })
}

/// 🔭️ Transmittance and front/back reflectance of one uncoated pane at incidence cosine `cs`,
/// from its normal-incidence values through the equivalent index of refraction and extinction.
pub fn pane_properties_at(cs: f64, transmittance: f64, reflectance_front: f64, reflectance_back: f64) -> (f64, f64, f64) {
    if transmittance <= 0.0 {
        return (0.0, reflectance_front, reflectance_back);
    }
    let interface = |reflectance: f64| {
        let beta = transmittance * transmittance - reflectance * reflectance + 2.0 * reflectance + 1.0;
        (beta - (beta * beta - 4.0 * (2.0 - reflectance) * reflectance).sqrt()) / (2.0 * (2.0 - reflectance))
    };
    let (r0f, r0b) = (interface(reflectance_front), interface(reflectance_back));
    let asymmetry = if r0f == r0b { 0.0 } else { (r0f - r0b).abs() / (r0f + r0b) };
    if asymmetry < 0.001 {
        let absorption = |r0: f64, reflectance: f64| if r0 * transmittance != 0.0 { (r0 * transmittance / (reflectance - r0)).ln() } else { 0.0 };
        let (abf, abb) = (absorption(r0f, reflectance_front), absorption(r0b, reflectance_back));
        let index = |r0: f64| (1.0 + r0.sqrt()) / (1.0 - r0.sqrt());
        let (ngf, ngb) = (index(r0f), index(r0b));
        let refracted = |n: f64| (1.0 - (1.0 - cs * cs) / (n * n)).sqrt();
        let (cgf, cgb) = (refracted(ngf), refracted(ngb));
        let fresnel = |numerator: f64, denominator: f64| if numerator != 0.0 { (numerator / denominator).powi(2) } else { 0.0 };
        let (rpf1, rpf2) = (fresnel(ngf * cs - cgf, ngf * cs + cgf), fresnel(ngf * cgf - cs, ngf * cgf + cs));
        let (rpb1, rpb2) = (fresnel(ngb * cs - cgb, ngb * cs + cgb), fresnel(ngb * cgb - cs, ngb * cgb + cs));
        let (tpf1, tpf2) = (1.0 - rpf1, 1.0 - rpf2);
        let decay = if cgf != 0.0 { (-abf / cgf).exp() } else { 0.0 };
        let decay_twice = if cgf != 0.0 { (-2.0 * abf / cgf).exp() } else { 0.0 };
        let tfp1 = if tpf1 != 0.0 { tpf1 * tpf1 * decay / (1.0 - rpf1 * rpf1 * decay_twice) } else { 0.0 };
        let tfp2 = if tpf2 != 0.0 { tpf2 * tpf2 * decay / (1.0 - rpf2 * rpf2 * decay_twice) } else { 0.0 };
        let rfp = 0.5 * (rpf1 * (1.0 + tfp1 * decay) + rpf2 * (1.0 + tfp2 * decay));
        let back_decay = if abb != 0.0 { (-abb / cgb).exp() } else { 0.0 };
        let rbp = 0.5 * (rpb1 * (1.0 + tfp1 * back_decay) + rpb2 * (1.0 + tfp2 * back_decay));
        return (0.5 * (tfp1 + tfp2), rfp, rbp);
    }
    let (curve_t, curve_r) = if cs > 0.999 {
        (1.0, 0.0)
    } else if cs < 0.001 {
        (0.0, 1.0)
    } else if transmittance > 0.645 {
        let t = -0.0015 + (3.355 + (-3.840 + (1.460 + 0.0288 * cs) * cs) * cs) * cs;
        (t, 0.999 + (-0.563 + (2.043 + (-2.532 + 1.054 * cs) * cs) * cs) * cs - t)
    } else {
        let t = -0.002 + (2.813 + (-2.341 + (-0.05725 + 0.599 * cs) * cs) * cs) * cs;
        (t, 0.997 + (-1.868 + (6.513 + (-7.862 + 3.225 * cs) * cs) * cs) * cs - t)
    };
    (transmittance * curve_t, reflectance_front * (1.0 - curve_r) + curve_r, reflectance_back * (1.0 - curve_r) + curve_r)
}

/// 🧪️ Transmittance and reflectance of the simple-glazing equivalent pane at incidence cosine `cs`,
/// from the angular curve family its rated U-factor and SHGC select.
pub fn simple_glazing_properties_at(cs: f64, layer: &SimpleGlazingLayer) -> (f64, f64) {
    let (curve_t, curve_r) = if cs == 1.0 { (1.0, 0.0) } else { simple_glazing_curves(cs, layer.u_value_w_m2k, layer.shgc) };
    let transmittance = (layer.pane.solar_transmittance * curve_t).clamp(0.0, 1.0);
    let reflectance = (layer.pane.solar_reflectance_front * (1.0 - curve_r) + curve_r).clamp(0.0, (0.9999 - transmittance).max(0.0));
    (transmittance, reflectance)
}

fn simple_glazing_curves(cs: f64, u: f64, shgc: f64) -> (f64, f64) {
    let (c2, c3, c4) = (cs * cs, cs.powi(3), cs.powi(4));
    let transmission = [
        3.36 * cs - 3.85 * c2 + 1.49 * c3 + 0.01 * c4,
        2.83 * cs - 2.42 * c2 + 0.04 * c3 + 0.55 * c4,
        2.45 * cs - 1.58 * c2 - 0.64 * c3 + 0.77 * c4,
        2.85 * cs - 2.58 * c2 + 0.40 * c3 + 0.35 * c4,
        1.51 * cs + 2.49 * c2 - 5.87 * c3 + 2.88 * c4,
        1.21 * cs + 3.14 * c2 - 6.37 * c3 + 3.03 * c4,
        1.09 * cs + 3.54 * c2 - 6.84 * c3 + 3.23 * c4,
        0.98 * cs + 3.83 * c2 - 7.13 * c3 + 3.33 * c4,
        0.79 * cs + 3.93 * c2 - 6.86 * c3 + 3.15 * c4,
        0.08 * cs + 6.02 * c2 - 8.84 * c3 + 3.74 * c4,
    ];
    let reflection_total = [
        1.00 - 0.70 * cs + 2.57 * c2 - 3.20 * c3 + 1.33 * c4,
        1.00 - 1.87 * cs + 6.50 * c2 - 7.86 * c3 + 3.23 * c4,
        1.00 - 2.52 * cs + 8.40 * c2 - 9.86 * c3 + 3.99 * c4,
        1.00 - 1.85 * cs + 6.40 * c2 - 7.64 * c3 + 3.11 * c4,
        1.00 - 1.57 * cs + 5.60 * c2 - 6.82 * c3 + 2.80 * c4,
        1.00 - 3.15 * cs + 10.98 * c2 - 13.14 * c3 + 5.32 * c4,
        1.00 - 3.25 * cs + 11.32 * c2 - 13.54 * c3 + 5.49 * c4,
        1.00 - 3.39 * cs + 11.70 * c2 - 13.94 * c3 + 5.64 * c4,
        1.00 - 4.06 * cs + 13.55 * c2 - 15.74 * c3 + 6.27 * c4,
        1.00 - 4.35 * cs + 14.27 * c2 - 16.32 * c3 + 6.39 * c4,
    ];
    let curve = |index: usize| (transmission[index], reflection_total[index] - transmission[index]);
    let (a, b, c, d, e, f, g, h, i, j) = (curve(0), curve(1), curve(2), curve(3), curve(4), curve(5), curve(6), curve(7), curve(8), curve(9));
    let mean = |items: &[(f64, f64)]| (items.iter().map(|x| x.0).sum::<f64>() / items.len() as f64, items.iter().map(|x| x.1).sum::<f64>() / items.len() as f64);
    let fghi = mean(&[f, g, h, i]);
    let fh = mean(&[f, h]);
    let bdcd = mean(&[b, d, c, d]);
    let two = |x: f64, x0: f64, x1: f64, p: (f64, f64), q: (f64, f64)| {
        let w = (x - x0) / (x1 - x0);
        (p.0 + w * (q.0 - p.0), p.1 + w * (q.1 - p.1))
    };
    let four = |x: f64, y: f64, x0: f64, x1: f64, y0: f64, y1: f64, p00: (f64, f64), p01: (f64, f64), p10: (f64, f64), p11: (f64, f64)| {
        let blend = |v00: f64, v01: f64, v10: f64, v11: f64| (v00 * (x1 - x) * (y1 - y) + v10 * (x - x0) * (y1 - y) + v01 * (x1 - x) * (y - y0) + v11 * (x - x0) * (y - y0)) / ((x1 - x0) * (y1 - y0));
        (blend(p00.0, p01.0, p10.0, p11.0), blend(p00.1, p01.1, p10.1, p11.1))
    };
    if u < 1.4195 {
        if shgc > 0.45 {
            e
        } else if shgc >= 0.35 {
            two(shgc, 0.35, 0.45, j, e)
        } else {
            j
        }
    } else if u <= 1.7034 {
        if shgc > 0.55 {
            e
        } else if shgc > 0.5 {
            four(u, shgc, 1.4195, 1.7034, 0.50, 0.55, e, e, fghi, e)
        } else if shgc > 0.45 {
            two(u, 1.4195, 1.7034, e, fghi)
        } else if shgc > 0.35 {
            four(u, shgc, 1.4195, 1.7034, 0.35, 0.45, j, e, fghi, fghi)
        } else if shgc > 0.3 {
            two(u, 1.4195, 1.7034, j, fghi)
        } else if shgc > 0.25 {
            four(u, shgc, 1.4195, 1.7034, 0.25, 0.3, j, j, fh, fghi)
        } else {
            two(u, 1.4195, 1.7034, j, fh)
        }
    } else if u < 3.4068 {
        if shgc > 0.55 {
            e
        } else if shgc >= 0.5 {
            two(shgc, 0.5, 0.55, fghi, e)
        } else if shgc > 0.3 {
            fghi
        } else if shgc >= 0.25 {
            two(shgc, 0.25, 0.30, fh, fghi)
        } else {
            fh
        }
    } else if u <= 4.5424 {
        if shgc > 0.65 {
            two(u, 3.4068, 4.5424, e, a)
        } else if shgc > 0.6 {
            four(u, shgc, 3.4068, 4.5424, 0.6, 0.65, e, e, bdcd, a)
        } else if shgc > 0.55 {
            two(u, 3.4068, 4.5424, e, bdcd)
        } else if shgc > 0.5 {
            four(u, shgc, 3.4068, 4.5424, 0.5, 0.55, fghi, e, bdcd, bdcd)
        } else if shgc > 0.45 {
            two(u, 3.4068, 4.5424, fghi, bdcd)
        } else if shgc > 0.3 {
            four(u, shgc, 3.4068, 4.5424, 0.3, 0.45, fghi, fghi, d, bdcd)
        } else if shgc > 0.25 {
            four(u, shgc, 3.4068, 4.5424, 0.25, 0.3, fh, fghi, d, d)
        } else {
            two(u, 3.4068, 4.5424, fh, d)
        }
    } else if shgc > 0.65 {
        a
    } else if shgc >= 0.6 {
        two(shgc, 0.6, 0.65, bdcd, a)
    } else if shgc > 0.45 {
        bdcd
    } else if shgc >= 0.3 {
        two(shgc, 0.3, 0.45, d, bdcd)
    } else {
        d
    }
}

/// 🔁️ System transmittance, front/back reflectance and per-pane absorptance of a stack of panes
/// (outside first) for radiation incident on the outside pane.
fn stack_properties(panes: &[(f64, f64, f64)]) -> (f64, f64, f64, [f64; MAX_PANES]) {
    let n = panes.len();
    let mut transmittance = [[0.0_f64; MAX_PANES]; MAX_PANES];
    let mut reflectance_front = [[0.0_f64; MAX_PANES]; MAX_PANES];
    let mut reflectance_back = [[0.0_f64; MAX_PANES]; MAX_PANES];
    for (index, &(t, rf, rb)) in panes.iter().enumerate() {
        transmittance[index][index] = t;
        reflectance_front[index][index] = rf;
        reflectance_back[index][index] = rb;
    }
    for i in 0..n.saturating_sub(1) {
        for j in (i + 1)..n {
            let denominator = 1.0 - reflectance_front[j][j] * reflectance_back[i][j - 1];
            if denominator == 0.0 {
                transmittance[j][i] = 0.0;
                reflectance_front[j][i] = 1.0;
                reflectance_back[i][j] = 1.0;
            } else {
                transmittance[j][i] = transmittance[j - 1][i] * transmittance[j][j] / denominator;
                reflectance_front[j][i] = reflectance_front[j - 1][i] + transmittance[j - 1][i].powi(2) * reflectance_front[j][j] / denominator;
                reflectance_back[i][j] = reflectance_back[j][j] + transmittance[j][j].powi(2) * reflectance_back[i][j - 1] / denominator;
            }
        }
    }
    let mut absorptance = [0.0; MAX_PANES];
    for j in 0..n {
        let (t0, rb0) = if j == 0 { (1.0, 0.0) } else { (transmittance[j - 1][0], reflectance_back[0][j - 1]) };
        let rf0 = if j == n - 1 { 0.0 } else { reflectance_front[n - 1][j + 1] };
        let af = 1.0 - transmittance[j][j] - reflectance_front[j][j];
        let ab = 1.0 - transmittance[j][j] - reflectance_back[j][j];
        let d1 = 1.0 - reflectance_front[n - 1][j] * rb0;
        let d2 = 1.0 - reflectance_back[0][j] * rf0;
        absorptance[j] = if d1 == 0.0 || d2 == 0.0 { 0.0 } else { t0 * af / d1 + transmittance[j][0] * rf0 * ab / d2 };
    }
    (transmittance[n - 1][0], reflectance_front[n - 1][0], reflectance_back[0][n - 1], absorptance)
}

/// 📈️ Least-squares fit of `y(x) = Σ c_k x^(k+1)`, `k = 0..5`, through the ten incidence angles.
fn polynomial_fit(x: &[f64; ANGLES], y: &[f64; ANGLES]) -> [f64; FIT_ORDER] {
    let mut matrix = [[0.0_f64; FIT_ORDER]; FIT_ORDER];
    let mut rhs = [0.0_f64; FIT_ORDER];
    for sample in 0..ANGLES {
        let powers: [f64; FIT_ORDER] = std::array::from_fn(|k| x[sample].powi(k as i32 + 1));
        for row in 0..FIT_ORDER {
            rhs[row] += y[sample] * powers[row];
            for column in 0..FIT_ORDER {
                matrix[row][column] += powers[row] * powers[column];
            }
        }
    }
    for pivot in 0..FIT_ORDER - 1 {
        for row in pivot + 1..FIT_ORDER {
            let factor = matrix[row][pivot] / matrix[pivot][pivot];
            rhs[row] -= rhs[pivot] * factor;
            for column in pivot..FIT_ORDER {
                matrix[row][column] -= matrix[pivot][column] * factor;
            }
        }
    }
    let mut coefficients = [0.0; FIT_ORDER];
    for row in (0..FIT_ORDER).rev() {
        let tail: f64 = (row + 1..FIT_ORDER).map(|column| matrix[row][column] * coefficients[column]).sum();
        coefficients[row] = (rhs[row] - tail) / matrix[row][row];
    }
    coefficients
}

/// 📈️ Evaluates a fitted angular property at incidence cosine `x` (zero outside `0..=1`).
pub fn evaluate_fit(x: f64, coefficients: &[f64; FIT_ORDER]) -> f64 {
    if !(0.0..=1.0).contains(&x) {
        return 0.0;
    }
    x * (coefficients[0] + x * (coefficients[1] + x * (coefficients[2] + x * (coefficients[3] + x * (coefficients[4] + x * coefficients[5])))))
}

/// 🌐️ Hemispherical average of an angular property, `∫ 2 p(φ) cos φ sin φ dφ` by the trapezoid rule
/// over the ten ten-degree samples.
fn diffuse_average(values: &[f64; ANGLES]) -> f64 {
    let step = 10.0_f64.to_radians();
    let sum: f64 = (0..ANGLES - 1).map(|index| 0.5 * step * (values[index] * (2.0 * index as f64 * step).sin() + values[index + 1] * (2.0 * (index + 1) as f64 * step).sin())).sum();
    sum.max(0.0)
}
// #endregion 🔖️Angular

// #region 🔖️System
impl GlazingSystem {
    /// 🪟️ Layered glazing: `panes` outside first, `gaps[i]` between pane `i` and `i + 1`.
    pub fn layered(panes: &[Pane], gaps: &[Gap]) -> Option<Self> {
        let n = panes.len();
        if n == 0 || n > MAX_PANES || gaps.len() + 1 != n {
            return None;
        }
        let cosines = incidence_cosines();
        let mut transmittance = [0.0; ANGLES];
        let mut reflectance_back = [0.0; ANGLES];
        let mut front = [[0.0; ANGLES]; MAX_PANES];
        let mut back = [[0.0; ANGLES]; MAX_PANES];
        let mut back_transmittance = [0.0; ANGLES];
        for (angle, &cs) in cosines.iter().enumerate() {
            let at: [(f64, f64, f64); MAX_PANES] = std::array::from_fn(|index| panes.get(index).map_or((0.0, 0.0, 0.0), |pane| pane_properties_at(cs, pane.solar_transmittance, pane.solar_reflectance_front, pane.solar_reflectance_back)));
            let (t, _, rb, absorbed) = stack_properties(&at[..n]);
            transmittance[angle] = t;
            reflectance_back[angle] = rb;
            let reversed: [(f64, f64, f64); MAX_PANES] = std::array::from_fn(|index| if index < n { let (t, rf, rb) = at[n - 1 - index]; (t, rb, rf) } else { (0.0, 0.0, 0.0) });
            let (tb, _, _, absorbed_back) = stack_properties(&reversed[..n]);
            back_transmittance[angle] = tb;
            for pane in 0..n {
                front[pane][angle] = absorbed[pane];
                back[pane][angle] = absorbed_back[n - 1 - pane];
            }
        }
        let mut system = Self::empty(n, &cosines, &transmittance, &reflectance_back, &front, &back, &back_transmittance);
        for (index, pane) in panes.iter().enumerate() {
            system.pane_conductance_w_m2k[index] = pane.conductivity_w_m_k / pane.thickness_m.max(1e-6);
            system.emissivity_front[index] = pane.infrared_emissivity_front;
            system.emissivity_back[index] = pane.infrared_emissivity_back;
        }
        for (index, gap) in gaps.iter().enumerate() {
            system.gap_width_m[index] = gap.width_m;
            system.gap_gas[index] = gap.gas;
        }
        Some(system)
    }

    /// 🧪️ Simple glazing: the rated U-factor and SHGC become one equivalent pane with the angular
    /// curve family those two ratings select.
    pub fn simple(u_value_w_m2k: f64, shgc: f64) -> Self {
        let layer = simple_glazing_layer(u_value_w_m2k, shgc);
        let cosines = incidence_cosines();
        let mut transmittance = [0.0; ANGLES];
        let mut reflectance = [0.0; ANGLES];
        let mut absorbed = [[0.0; ANGLES]; MAX_PANES];
        for (angle, &cs) in cosines.iter().enumerate() {
            let (t, r) = simple_glazing_properties_at(cs, &layer);
            transmittance[angle] = t;
            reflectance[angle] = r;
            absorbed[0][angle] = 1.0 - t - r;
        }
        let mut system = Self::empty(1, &cosines, &transmittance, &reflectance, &absorbed, &absorbed, &transmittance);
        system.pane_conductance_w_m2k[0] = layer.pane.conductivity_w_m_k / layer.pane.thickness_m;
        system.emissivity_front[0] = layer.pane.infrared_emissivity_front;
        system.emissivity_back[0] = layer.pane.infrared_emissivity_back;
        system.rated_u_w_m2k = Some(u_value_w_m2k);
        system
    }

    fn empty(n: usize, cosines: &[f64; ANGLES], transmittance: &[f64; ANGLES], reflectance_back: &[f64; ANGLES], front: &[[f64; ANGLES]; MAX_PANES], back: &[[f64; ANGLES]; MAX_PANES], back_transmittance: &[f64; ANGLES]) -> Self {
        Self {
            panes: n,
            pane_conductance_w_m2k: [0.0; MAX_PANES],
            emissivity_front: [0.84; MAX_PANES],
            emissivity_back: [0.84; MAX_PANES],
            gap_width_m: [0.0; MAX_PANES],
            gap_gas: [GasKind::Air; MAX_PANES],
            transmittance_fit: polynomial_fit(cosines, transmittance),
            front_absorptance_fit: std::array::from_fn(|pane| if pane < n { polynomial_fit(cosines, &front[pane]) } else { [0.0; FIT_ORDER] }),
            back_absorptance_fit: std::array::from_fn(|pane| if pane < n { polynomial_fit(cosines, &back[pane]) } else { [0.0; FIT_ORDER] }),
            back_transmittance_fit: polynomial_fit(cosines, back_transmittance),
            diffuse_transmittance: diffuse_average(transmittance),
            diffuse_back_reflectance: diffuse_average(reflectance_back),
            diffuse_front_absorptance: std::array::from_fn(|pane| if pane < n { diffuse_average(&front[pane]) } else { 0.0 }),
            diffuse_back_absorptance: std::array::from_fn(|pane| if pane < n { diffuse_average(&back[pane]) } else { 0.0 }),
            rated_u_w_m2k: None,
        }
    }

    /// ☀️ Beam transmittance at incidence cosine `cs`.
    pub fn beam_transmittance(&self, cs: f64) -> f64 {
        evaluate_fit(cs, &self.transmittance_fit)
    }

    /// ☀️ Beam absorptance of pane `pane` for radiation arriving from outside.
    pub fn beam_front_absorptance(&self, pane: usize, cs: f64) -> f64 {
        evaluate_fit(cs, &self.front_absorptance_fit[pane])
    }

    /// ☀️ Beam absorptance of pane `pane` for radiation arriving from the room.
    pub fn beam_back_absorptance(&self, pane: usize, cs: f64) -> f64 {
        evaluate_fit(cs, &self.back_absorptance_fit[pane])
    }

    /// ☀️ Beam transmittance for radiation arriving from the room.
    pub fn beam_back_transmittance(&self, cs: f64) -> f64 {
        evaluate_fit(cs, &self.back_transmittance_fit)
    }

    /// ☀️ Room-side diffuse absorptance summed over panes.
    pub fn diffuse_back_absorptance_total(&self) -> f64 {
        self.diffuse_back_absorptance[..self.panes].iter().sum()
    }

    /// 🌡️ Face count of the thermal chain (two per pane).
    pub fn faces(&self) -> usize {
        2 * self.panes
    }
}

/// 🧪️ Equivalent single pane of a simple glazing system (EnergyPlus
/// `WindowMaterial:SimpleGlazingSystem`): film resistances from the rated U-factor, solar
/// transmittance from U and SHGC, and a reflectance that closes the SHGC through the inward-flowing
/// fraction of absorbed solar.
pub fn simple_glazing_layer(u_value_w_m2k: f64, shgc: f64) -> SimpleGlazingLayer {
    let u = u_value_w_m2k;
    let r_inside_winter = if u < 5.85 { 1.0 / (0.359073 * u.ln() + 6.949915) } else { 1.0 / (1.788041 * u - 2.886625) };
    let r_outside_winter = 1.0 / (0.025342 * u + 29.163853);
    let r_layer = (1.0 / u - r_inside_winter - r_outside_winter).max(0.001);
    let thickness_m = if 1.0 / r_layer > 7.0 { 0.002 } else { 0.05914 - 0.00714 / r_layer };
    let high = |g: f64| if g < 0.7206 { 0.939998 * g * g + 0.20332 * g } else { 1.30415 * g - 0.30515 };
    let low = |g: f64| if g <= 0.15 { 0.41040 * g } else { 0.085775 * g * g + 0.963954 * g - 0.084958 };
    let interpolation = (u - 3.4) / (4.5 - 3.4);
    let transmittance = if u > 4.5 {
        high(shgc)
    } else if u < 3.4 {
        low(shgc)
    } else {
        interpolation * (high(shgc) - low(shgc)) + low(shgc)
    }
    .max(0.0);
    let delta = shgc - transmittance;
    let r_inside_low = 1.0 / (199.8208128 * delta.powi(3) - 90.639733 * delta * delta + 19.737055 * delta + 6.766575);
    let r_outside_low = 1.0 / (5.763355 * delta + 20.541528);
    let r_inside_high = 1.0 / (29.436546 * delta.powi(3) - 21.943415 * delta * delta + 9.945872 * delta + 7.426151);
    let r_outside_high = 1.0 / (2.225824 * delta + 20.577080);
    let (r_inside_summer, r_outside_summer) = if u > 4.5 {
        (r_inside_high, r_outside_high)
    } else if u < 3.4 {
        (r_inside_low, r_outside_low)
    } else {
        (interpolation * (r_inside_low - r_inside_high) + r_inside_low, interpolation * (r_outside_low - r_outside_high) + r_outside_low)
    };
    let inflow = (r_outside_summer + 0.5 * r_layer) / (r_outside_summer + r_layer + r_inside_summer);
    let absorptance = (shgc - transmittance) / inflow;
    let reflectance = 1.0 - transmittance - absorptance;
    SimpleGlazingLayer {
        pane: Pane {
            thickness_m,
            conductivity_w_m_k: thickness_m / r_layer,
            solar_transmittance: transmittance,
            solar_reflectance_front: reflectance,
            solar_reflectance_back: reflectance,
            infrared_transmittance: 0.0,
            infrared_emissivity_front: 0.84,
            infrared_emissivity_back: 0.84,
        },
        u_value_w_m2k,
        shgc,
    }
}
// #endregion 🔖️System

// #region 🔖️Conductance
/// 🌫️ Conductivity [W/(m·K)], dynamic viscosity [kg/(m·s)], specific heat [J/(kg·K)] and molar
/// mass [kg/kmol] of a fill gas at `t_k`.
pub fn gas_properties(gas: GasKind, t_k: f64) -> (f64, f64, f64, f64) {
    let (conductivity, viscosity, specific_heat, molar_mass) = match gas {
        GasKind::Air => ((2.873e-3, 7.760e-5), (3.723e-6, 4.940e-8), (1002.737, 1.2324e-2), 28.97),
        GasKind::Argon => ((2.285e-3, 5.149e-5), (3.379e-6, 6.451e-8), (521.929, 0.0), 39.948),
        GasKind::Krypton => ((9.443e-4, 2.826e-5), (2.213e-6, 7.777e-8), (248.091, 0.0), 83.8),
        GasKind::Xenon => ((4.538e-4, 1.723e-5), (1.069e-6, 7.414e-8), (158.340, 0.0), 131.3),
    };
    (conductivity.0 + conductivity.1 * t_k, viscosity.0 + viscosity.1 * t_k, specific_heat.0 + specific_heat.1 * t_k, molar_mass)
}

/// 🌫️ Convective-conductive conductance [W/(m²·K)] of a sealed gap between faces at `t_outer_k`
/// and `t_inner_k` (ISO 15099 Nusselt correlations for an enclosed cavity of `height_m` at `tilt_deg`).
pub fn gap_conductance_w_m2k(t_outer_k: f64, t_inner_k: f64, width_m: f64, height_m: f64, tilt_deg: f64, gas: GasKind) -> f64 {
    let mean = 0.5 * (t_outer_k + t_inner_k);
    let (conductivity, viscosity, specific_heat, molar_mass) = gas_properties(gas, mean);
    let density = 1.0e5 * molar_mass / (8314.51 * mean);
    let prandtl = specific_heat * viscosity / conductivity;
    let grashof = 9.807 * width_m.powi(3) * (t_outer_k - t_inner_k).abs() * density * density / (mean * viscosity * viscosity);
    let rayleigh = grashof * prandtl;
    let aspect = height_m / width_m;
    let vertical_1 = if rayleigh <= 1.0e4 {
        1.0 + 1.759_667_8e-10 * rayleigh.powf(2.298_475_5)
    } else if rayleigh <= 5.0e4 {
        0.028154 * rayleigh.powf(0.4134)
    } else {
        0.0673838 * rayleigh.powf(1.0 / 3.0)
    };
    let vertical = vertical_1.max(0.242 * (rayleigh / aspect).powf(0.272));
    let tilt = tilt_deg.to_radians();
    let nusselt = if t_outer_k > t_inner_k {
        1.0 + (vertical - 1.0) * tilt.sin()
    } else if tilt_deg >= 60.0 {
        let g = if rayleigh >= 0.001 { 0.5 * (1.0 + (rayleigh / 3160.0).powf(20.6)).powf(-0.1) } else { 0.5 };
        let sixty = (1.0 + (0.0936 * rayleigh.powf(0.314) / (1.0 + g)).powi(7)).powf(0.142857).max((0.104 + 0.175 / aspect) * rayleigh.powf(0.283));
        ((90.0 - tilt_deg) * sixty + (tilt_deg - 60.0) * vertical) / 30.0
    } else {
        let rayleigh_cos = rayleigh * tilt.cos();
        let a = 1.0 - 1708.0 / rayleigh_cos;
        let b = (rayleigh_cos / 5830.0).powf(0.33333) - 1.0;
        let angle = 1708.0 * (1.8 * tilt).sin().powf(1.6);
        1.0 + 1.44 * 0.5 * (a.abs() + a) * (1.0 - angle / rayleigh_cos) + 0.5 * (b.abs() + b)
    };
    conductivity / width_m * nusselt
}

/// 🌬️ Room-side natural convection coefficient [W/(m²·K)] of a glazing face (ISO 15099 §8.3.2)
/// of `height_m` at `tilt_deg`, never below `0.1`.
pub fn glazing_interior_convection_w_m2k(surface_c: f64, air_c: f64, humidity_ratio: f64, pressure_pa: f64, height_m: f64, tilt_deg: f64) -> f64 {
    let (surface_k, air_k) = (surface_c + 273.15, air_c + 273.15);
    if !(200.0..=400.0).contains(&air_k) || !(180.0..=450.0).contains(&surface_k) {
        return 0.1;
    }
    let delta = surface_c - air_c;
    let film_k = air_k + 0.25 * delta;
    let density = moist_air_density(film_k - 273.15, humidity_ratio, pressure_pa);
    let conductivity = 2.873e-3 + 7.76e-5 * film_k;
    let viscosity = 3.723e-6 + 4.94e-8 * film_k;
    let tilt = if delta > 0.0 { 180.0 - tilt_deg } else { tilt_deg };
    let rayleigh = density * density * height_m.powi(3) * 9.81 * moist_air_cp_j_per_kg_k(humidity_ratio) * delta.abs() / (film_k * viscosity * conductivity);
    let sine = tilt_deg.to_radians().sin();
    let nusselt = if tilt < 15.0 {
        0.13 * rayleigh.cbrt()
    } else if tilt <= 90.0 {
        let critical = 2.5e5 * ((0.72 * tilt).min(700.0).exp() / sine).powf(0.2);
        if rayleigh <= critical {
            0.56 * (rayleigh * sine).powf(0.25)
        } else {
            0.13 * (rayleigh.cbrt() - critical.cbrt()) + 0.56 * (critical * sine).powf(0.25)
        }
    } else if tilt <= 179.0 {
        0.56 * (rayleigh * sine).clamp(1.0e5, 1.0e11).powf(0.25)
    } else {
        0.58 * rayleigh.min(1.0e11).powf(0.2)
    };
    (nusselt * conductivity / height_m).max(0.1)
}

/// 📏️ Ratio by which a simple glazing system's film coefficients are scaled so its centre-of-glass
/// conductance under NFRC winter rating conditions (−18 °C out, 21 °C in, `h_out` 26 W/(m²·K), 1 m
/// high) equals the rated U-factor — bisection on `[1, 2]`, `1` when already within 0.01.
pub fn rated_coefficient_adjustment(system: &GlazingSystem) -> f64 {
    let Some(rated) = system.rated_u_w_m2k else { return 1.0 };
    let mut conductance = rating_conductance(system, 1.0);
    let (mut low, mut high, mut ratio) = (1.0, 2.0, 1.0);
    for _ in 0..100 {
        if (rated - conductance).abs() <= 0.01 {
            break;
        }
        ratio = 0.5 * (low + high);
        conductance = rating_conductance(system, ratio);
        if conductance < rated {
            low = ratio;
        } else {
            high = ratio;
        }
    }
    ratio
}

fn rating_conductance(system: &GlazingSystem, ratio: f64) -> f64 {
    let (t_in, t_out) = (294.15, 255.15);
    let h_out = 26.0 * ratio;
    let sigma = crate::units::STEFAN_BOLTZMANN;
    let scon = system.pane_conductance_w_m2k[0];
    let (e_out, e_in) = (system.emissivity_front[0], system.emissivity_back[0]);
    let resistances = [1.0 / (26.0 + 5.3), 1.0 / scon, 1.0 / (3.2 + 5.3)];
    let total: f64 = resistances.iter().sum();
    let mut faces = [t_out + resistances[0] / total * (t_in - t_out), t_out + (resistances[0] + resistances[1]) / total * (t_in - t_out)];
    let mut h_in = 0.0;
    for iteration in 0..100 {
        let film = t_in + 0.25 * (faces[1] - t_in);
        let density = 101_325.0 / (287.0 * film * (1.0 + 1.6077687e-5));
        let conductivity = 2.873e-3 + 7.76e-5 * film;
        let viscosity = 3.723e-6 + 4.94e-8 * film;
        let specific_heat = 1002.737 + 1.2324e-2 * film;
        let rayleigh = density * density * 9.81 * specific_heat * (faces[1] - t_in).abs() / (film * viscosity * conductivity);
        let fresh = 0.56 * rayleigh.powf(0.25) * conductivity;
        h_in = if iteration >= 1 { 0.5 * (h_in + fresh) } else { fresh } * ratio;
        let hr = [e_out * sigma * faces[0].powi(3), e_in * sigma * faces[1].powi(3)];
        let a = [[hr[0] + scon + h_out, -scon], [-scon, hr[1] + scon + h_in]];
        let b = [sigma * t_out.powi(4) * e_out + h_out * t_out, sigma * t_in.powi(4) * e_in + h_in * t_in];
        let determinant = a[0][0] * a[1][1] - a[0][1] * a[1][0];
        let solved = [(b[0] * a[1][1] - a[0][1] * b[1]) / determinant, (a[0][0] * b[1] - b[0] * a[1][0]) / determinant];
        let error = 0.5 * ((faces[0] - solved[0]).abs() + (faces[1] - solved[1]).abs());
        faces = [0.5 * (faces[0] + solved[0]), 0.5 * (faces[1] + solved[1])];
        if error <= 0.02 {
            break;
        }
    }
    let h_out_rad = e_out * sigma * 0.5 * (t_out + faces[0]).powi(3);
    let h_in_rad = e_in * sigma * 0.5 * (t_in + faces[1]).powi(3);
    1.0 / (1.0 / (h_out_rad + h_out) + 1.0 / scon + 1.0 / (h_in_rad + h_in))
}
// #endregion 🔖️Conductance

// #region 🔖️Condensation
/// 💧️ Condensation risk on interior glazing surface.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CondensationRisk {
    None,
    Risk { margin_k: f64 },
    Condensing,
}

/// 💧️ Assess interior surface condensation vs zone dew point.
pub fn condensation_risk(interior_surface_temp_c: f64, humidity_ratio: f64, atmospheric_pressure_pa: f64) -> CondensationRisk {
    let margin = interior_surface_temp_c - dew_point_c(humidity_ratio, atmospheric_pressure_pa);
    if margin <= 0.0 {
        CondensationRisk::Condensing
    } else if margin < 2.0 {
        CondensationRisk::Risk { margin_k: margin }
    } else {
        CondensationRisk::None
    }
}

/// 💧️ Interior surface RH given surface temperature.
pub fn interior_surface_rh(surface_temp_c: f64, zone_air_temp_c: f64, zone_rh: f64) -> f64 {
    let p_w = zone_rh * saturation_pressure_pa(zone_air_temp_c);
    (p_w / saturation_pressure_pa(surface_temp_c).max(1.0)).clamp(0.0, 1.5)
}
// #endregion 🔖️Condensation

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
