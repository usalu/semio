//! 📐️ Exact signed doubled areas between a conic arc and its chord.

/// 🌙️ Returns the signed doubled elliptic segment area.
pub(super) fn elliptic([a, b]: [f64; 2], sweep: f64) -> f64 {
    let residual = if sweep.abs() < 0.125 {
        let y = sweep * sweep;
        sweep * y * (1.0 / 6.0 + y * (-1.0 / 120.0 + y * (1.0 / 5040.0 + y * (-1.0 / 362880.0 + y / 39916800.0))))
    } else { sweep - sweep.sin() };
    a.min(b) * (a.max(b) * residual)
}

/// 🧮️ Scales the signed control triangle for a rational quadratic with equal end weights.
pub(super) fn rational(weight: f64) -> f64 {
    let a = 0.5 * (1.0 + weight);
    let ratio = (1.0 - weight) / (1.0 + weight);
    if ratio.abs() <= 0.25 {
        let mut power = 1.0;
        let mut integral = 0.0;
        for i in 0..32 {
            let n = f64::from(i);
            integral += power * (n + 1.0) * (1.0 / (2.0 * n + 1.0) + 1.0 / (2.0 * n + 3.0));
            power *= -ratio;
        }
        return weight * integral / (2.0 * a * a);
    }
    if weight < 1.0 {
        let gap = (1.0 - weight) * (1.0 + weight);
        let integral = weight.acos() / gap.sqrt();
        weight * (integral - weight) / gap
    } else if weight < 1e150 {
        let inverse = 1.0 / weight;
        let gap = (1.0 - inverse) * (1.0 + inverse);
        (1.0 - weight.acosh() * (inverse * inverse) / gap.sqrt()) / gap
    } else { 1.0 }
}
