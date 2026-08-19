//! 🔢️ Numerical utilities: solvers, interpolation, integration, polynomials, lookup tables.

// #region 🔖️Interpolation
/// 📈️ Linear interpolation with clamping.
pub async fn lerp(x: f64, x0: f64, x1: f64, y0: f64, y1: f64) -> f64 {
    if (x1 - x0).abs() < 1e-12 {
        return y0;
    }
    let t = ((x - x0) / (x1 - x0)).clamp(0.0, 1.0);
    y0 + t * (y1 - y0)
}

/// 📈️ Bilinear interpolation on a regular grid.
pub async fn bilinear(x: f64, y: f64, x_vals: &[f64], y_vals: &[f64], table: &[Vec<f64>]) -> f64 {
    let xi = bracket_index(x, x_vals);
    let yi = bracket_index(y, y_vals);
    let x0 = x_vals[xi];
    let x1 = x_vals[(xi + 1).min(x_vals.len() - 1)];
    let y0 = y_vals[yi];
    let y1 = y_vals[(yi + 1).min(y_vals.len() - 1)];
    let q00 = table[yi][xi];
    let q10 = table[yi][(xi + 1).min(x_vals.len() - 1)];
    let q01 = table[(yi + 1).min(y_vals.len() - 1)][xi];
    let q11 = table[(yi + 1).min(y_vals.len() - 1)][(xi + 1).min(x_vals.len() - 1)];
    let tx = if (x1 - x0).abs() < 1e-12 { 0.0 } else { ((x - x0) / (x1 - x0)).clamp(0.0, 1.0) };
    let ty = if (y1 - y0).abs() < 1e-12 { 0.0 } else { ((y - y0) / (y1 - y0)).clamp(0.0, 1.0) };
    lerp(ty, 0.0, 1.0, lerp(tx, 0.0, 1.0, q00, q10), lerp(tx, 0.0, 1.0, q01, q11))
}

async fn bracket_index(x: f64, vals: &[f64]) -> usize {
    if vals.len() < 2 {
        return 0;
    }
    for i in 0..vals.len() - 1 {
        if x <= vals[i + 1] {
            return i;
        }
    }
    vals.len() - 2
}
// #endregion 🔖️Interpolation

// #region 🔖️Polynomial
/// 📐️ Evaluate polynomial Σ cᵢ xⁱ.
pub async fn poly_eval(coeffs: &[f64], x: f64) -> f64 {
    coeffs.iter().rev().fold(0.0, |acc, &c| acc * x + c)
}

/// 📐️ Biquadratic f(x,y) = c0 + c1*x + c2*x² + c3*y + c4*y² + c5*x*y.
pub async fn biquadratic(c: [f64; 6], x: f64, y: f64) -> f64 {
    c[0] + c[1] * x + c[2] * x * x + c[3] * y + c[4] * y * y + c[5] * x * y
}
// #endregion 🔖️Polynomial

// #region 🔖️Integration
/// ∫f(x)dx from a to b via Simpson's rule (n = even number of subintervals).
pub async fn simpson_integrate(f: impl Fn(f64) -> f64, a: f64, b: f64, n: usize) -> f64 {
    let mut n = n.max(2);
    if !n.is_multiple_of(2) {
        n += 1;
    }
    let h = (b - a) / n as f64;
    let mut sum = f(a) + f(b);
    for i in 1..n {
        let x = a + i as f64 * h;
        sum += f(x) * if i % 2 == 0 { 2.0 } else { 4.0 };
    }
    sum * h / 3.0
}

/// Explicit Euler step.
pub async fn euler_step(y: f64, dydt: f64, dt: f64) -> f64 {
    y + dydt * dt
}

/// Third-order backward difference coefficient for zone temperature.
pub async fn third_order_backward_diff(history: [f64; 3], dt: f64, dtdt: f64) -> f64 {
    let (y0, y1, y2) = (history[0], history[1], history[2]);
    let coeff = 11.0 / 6.0;
    (coeff * y0 - 3.0 * y1 + 1.5 * y2 - 0.5 * history[2]) / dt + dtdt
}
// #endregion 🔖️Integration

// #region 🔖️Solvers
/// 🔍️ Newton-Raphson root finder.
pub async fn newton_raphson(mut x: f64, f: impl Fn(f64) -> f64, df: impl Fn(f64) -> f64, max_iter: usize, tol: f64) -> Option<f64> {
    for _ in 0..max_iter {
        let fx = f(x);
        if fx.abs() < tol {
            return Some(x);
        }
        let dfx = df(x);
        if dfx.abs() < 1e-15 {
            return None;
        }
        x -= fx / dfx;
    }
    if f(x).abs() < tol * 10.0 {
        Some(x)
    } else {
        None
    }
}

/// 🔍️ Gauss-Seidel iterative solver for Ax = b (dense).
pub async fn gauss_seidel(a: &[Vec<f64>], b: &[f64], x: &mut [f64], max_iter: usize, tol: f64) -> bool {
    let n = b.len();
    for _ in 0..max_iter {
        let mut max_delta = 0.0_f64;
        for i in 0..n {
            let mut sigma = 0.0;
            for j in 0..n {
                if i != j {
                    sigma += a[i][j] * x[j];
                }
            }
            let denom = a[i][i];
            if denom.abs() < 1e-15 {
                return false;
            }
            let new_x = (b[i] - sigma) / denom;
            max_delta = max_delta.max((new_x - x[i]).abs());
            x[i] = new_x;
        }
        if max_delta < tol {
            return true;
        }
    }
    false
}
// #endregion 🔖️Solvers

// #region 🔖️LookupTable
/// 📊️ Regular-grid lookup table with linear interpolation.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LookupTable2D {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub values: Vec<Vec<f64>>,
}

impl LookupTable2D {
    pub async fn evaluate(&self, x: f64, y: f64) -> f64 {
        bilinear(x, y, &self.x, &self.y, &self.values)
    }
}
// #endregion 🔖️LookupTable

#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn lerp_endpoints() {
        assert!((lerp(0.5, 0.0, 1.0, 0.0, 10.0) - 5.0).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn newton_finds_sqrt() {
        let r = newton_raphson(2.0, |x| x * x - 2.0, |x| 2.0 * x, 20, 1e-10).unwrap();
        assert!((r - std::f64::consts::SQRT_2).abs() < 1e-8);
    }

    #[semio_framework_async_macros::async_test]
    async fn simpson_integrates_x_squared() {
        let integral = simpson_integrate(|x| x * x, 0.0, 1.0, 100);
        assert!((integral - 1.0 / 3.0).abs() < 1e-6);
    }
}
