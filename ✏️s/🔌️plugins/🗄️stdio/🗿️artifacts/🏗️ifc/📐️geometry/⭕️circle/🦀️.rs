//! ⭕️ Fixed-size represented circle basis radius shared with the TypeScript contract.
pub type CircleAxis = [f64; 3];

/// 📏️ Bound a positive normal finite world radius without allocating or owning a work context.
pub fn circle_basis_radius(radius: f64, u: CircleAxis, v: CircleAxis) -> Option<f64> {
    if !(radius > 0.0) || !radius.is_finite() || !u.iter().chain(v.iter()).all(|n| n.is_finite()) {
        return None;
    }
    let largest = u.iter().chain(v.iter()).fold(0.0_f64, |high, n| high.max(n.abs()));
    if !(largest > 0.0) {
        return None;
    }
    let (mut aa, mut bb, mut ab) = (0.0, 0.0, 0.0);
    for axis in 0..3 {
        let (a, b) = (u[axis] / largest, v[axis] / largest);
        aa += a * a;
        bb += b * b;
        ab += a * b;
    }
    let lambda = (aa + bb + (aa - bb).hypot(2.0 * ab)) / 2.0 + 64.0 * f64::EPSILON;
    let result = radius * largest * lambda.sqrt() * (1.0 + 16.0 * f64::EPSILON);
    (result.is_finite() && result >= f64::MIN_POSITIVE).then_some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_neutral_basis_radius_cases() {
        let data: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/radius-cases.json")).unwrap();
        for row in data["cases"].as_array().unwrap() {
            let axis = |key: &str| std::array::from_fn(|index| row[key][index].as_f64().unwrap());
            let actual = circle_basis_radius(row["radius"].as_f64().unwrap(), axis("u"), axis("v"));
            if let Some(expected) = row["expected"].as_f64() {
                let actual = actual.unwrap();
                assert!(actual >= expected * (1.0 - 1e-14), "{}", row["id"]);
                assert!(actual <= expected * (1.0 + 1e-12), "{}", row["id"]);
            } else {
                assert_eq!(actual, None, "{}", row["id"]);
            }
        }
    }
}
