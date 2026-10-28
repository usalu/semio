/// 🔲️ Owned scalar convex boundary declarations share the language-neutral case contract.
#[derive(Clone, Copy, Debug)]
pub enum ConvexProfileBoundary {
    Circle { radius: f64, chords: u64 },
    RoundedRectangle { width: f64, height: f64, radius: f64, chords: u64 },
}

/// 🧮️ Count exact straight joins without rounding a capsule or circle into another profile.
pub fn convex_profile_size(source: ConvexProfileBoundary) -> Option<u64> {
    let (radius, chords) = match source {
        ConvexProfileBoundary::Circle { radius, chords } => (radius, chords),
        ConvexProfileBoundary::RoundedRectangle { radius, chords, .. } => (radius, chords),
    };
    if !radius.is_finite() || radius <= 0.0 || chords < 4 || chords > 9_007_199_254_740_991 || chords % 4 != 0 { return None; }
    match source {
        ConvexProfileBoundary::Circle { .. } => Some(chords),
        ConvexProfileBoundary::RoundedRectangle { width, height, .. } => {
            if !width.is_finite() || !height.is_finite() || width <= 0.0 || height <= 0.0 || radius > width / 2.0 || radius > height / 2.0 { return None; }
            let size = chords.checked_add(if radius < width / 2.0 { 2 } else { 0 })?.checked_add(if radius < height / 2.0 { 2 } else { 0 })?;
            (size <= 9_007_199_254_740_991).then_some(size)
        }
    }
}

/// 🧭️ Borrow one bounded indexed CCW sample; no array, source admission or funding is implied.
pub fn convex_profile_point(source: ConvexProfileBoundary, mut index: u64) -> Option<[f64; 2]> {
    if index >= convex_profile_size(source)? { return None; }
    match source {
        ConvexProfileBoundary::Circle { radius, chords } => {
            let q = chords / 4;
            if index % q == 0 { let cardinal = [[1.0, 0.0], [0.0, 1.0], [-1.0, 0.0], [0.0, -1.0]][(index / q) as usize]; return Some([radius * cardinal[0], radius * cardinal[1]]); }
            let angle = 2.0 * std::f64::consts::PI * index as f64 / chords as f64;
            Some([radius * angle.cos(), radius * angle.sin()])
        }
        ConvexProfileBoundary::RoundedRectangle { width, height, radius, chords } => {
            let q = chords / 4; let x = width / 2.0; let y = height / 2.0; let a = x - radius; let b = y - radius;
            for quadrant in 0..4 { let count = q + if (if quadrant % 2 == 0 { a } else { b }) > 0.0 { 1 } else { 0 }; if index >= count { index -= count; continue; }
                if index == 0 { return Some([[x, b], [-a, y], [-x, -b], [a, -y]][quadrant]); }
                if index == q { return Some([[a, y], [-x, b], [-a, -y], [x, -b]][quadrant]); }
                let angle = std::f64::consts::PI / 2.0 * (quadrant as f64 + index as f64 / q as f64);
                return Some([(if quadrant == 0 || quadrant == 3 { a } else { -a }) + radius * angle.cos(), (if quadrant < 2 { b } else { -b }) + radius * angle.sin()]);
            }
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_neutral_convex_profile_cases() {
        let data: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/cases.json")).unwrap();
        for row in data["cases"].as_array().unwrap() {
            let source = &row["boundary"];
            let radius = source["radius"].as_f64().unwrap(); let chords = source["chords"].as_u64().unwrap();
            let boundary = match source["kind"].as_str().unwrap() {
                "circle" => ConvexProfileBoundary::Circle { radius, chords },
                "rounded-rectangle" => ConvexProfileBoundary::RoundedRectangle { width: source["width"].as_f64().unwrap(), height: source["height"].as_f64().unwrap(), radius, chords },
                _ => panic!("Unknown neutral boundary"),
            };
            let size = convex_profile_size(boundary);
            if let Some(expected) = row["expectedSize"].as_u64() {
                assert_eq!(size, Some(expected), "{}", row["id"]);
                let points: Vec<_> = (0..expected).map(|index| convex_profile_point(boundary, index).unwrap()).collect();
                let area: f64 = points.iter().enumerate().map(|(index, p)| { let q = points[(index + 1) % points.len()]; (p[0] * q[1] - p[1] * q[0]) / 2.0 }).sum();
                assert!((area - row["expectedArea"].as_f64().unwrap()).abs() < 1e-12, "{}", row["id"]);
                assert_eq!(convex_profile_point(boundary, expected), None);
            } else {
                assert_eq!(size, None, "{}", row["id"]);
                assert_eq!(convex_profile_point(boundary, 0), None, "{}", row["id"]);
            }
        }
    }
}
