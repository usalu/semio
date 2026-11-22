//! 🎯️ Locates a longitudinal support crossing strictly inside a prism profile.

use super::{LongitudinalSource, SourceVertex, cross, difference, dot, height};

type Sample = ([f32; 3], SourceVertex);

pub(super) struct Crossing {
    base: [f32; 3],
    offset: [f64; 3],
    pub(super) points: [[f32; 3]; 2],
}

pub(super) fn profile_contains(profile: &[[f32; 3]], base: [f32; 3], normal: [f64; 3], epsilon: f64) -> Option<bool> {
    let axis = (0..3).max_by(|&a, &b| normal[a].abs().total_cmp(&normal[b].abs())).unwrap();
    let project = |point: [f32; 3]| [(axis + 1) % 3, (axis + 2) % 3].map(|i| f64::from(point[i]));
    let [x, y] = project(base);
    let mut inside = false;
    for (&a, &b) in profile.iter().zip(profile.iter().cycle().skip(1)) {
        let ([ax, ay], [bx, by]) = (project(a), project(b));
        let (dx, dy) = (bx - ax, by - ay);
        let length2 = dx * dx + dy * dy;
        if length2 <= epsilon * epsilon { return None; }
        let t = (((x - ax) * dx + (y - ay) * dy) / length2).clamp(0.0, 1.0);
        if (x - ax - t * dx).hypot(y - ay - t * dy) <= epsilon { return None; }
        if (ay > y) != (by > y) && x < ax + (y - ay) * dx / dy {
            inside = !inside;
        }
    }
    Some(inside)
}

pub(super) fn crossing(input: &LongitudinalSource<'_>, supports: [usize; 2], sweep: [f64; 3]) -> Option<Crossing> {
    let normal = input.stock_normal;
    let planes = supports.map(|support| input.planes[support]);
    let [a, b] = planes.map(|plane| plane.normal.0.map(f64::from));
    let determinant = dot(cross(a, b), normal);
    if determinant.abs() <= 1e-6 { return None; }
    let stock_cap = &input.source.faces[input.prism.caps()[0]];
    let anchor = input.source.positions[stock_cap[0] as usize];
    let [da, db] = planes.map(|plane| -height(anchor, plane) / determinant);
    let [u, v] = [cross(b, normal), cross(normal, a)];
    let offset: [f64; 3] = std::array::from_fn(|i| da * u[i] + db * v[i]);
    let at = |t| std::array::from_fn(|i| (f64::from(anchor[i]) + offset[i] + t * sweep[i]) as f32);
    let points = input.interval.map(at);
    if !points.iter().flatten().all(|coordinate| coordinate.is_finite()) { return None; }
    Some(Crossing { base: at(0.0), offset, points })
}

pub(super) fn sample(input: &LongitudinalSource<'_>, supports: [usize; 2], crossing: Crossing) -> Option<[Sample; 2]> {
    let Crossing { base, offset, points } = crossing;
    let caps = input.caps.map(|cap| cap.map(|i| input.planes[i]));
    let planes = supports.map(|support| input.planes[support]);
    let normal = input.stock_normal;
    let ring = &input.source.faces[input.prism.caps()[0]];
    let profile: Vec<_> = ring.iter().map(|&id| input.source.positions[id as usize]).collect();
    let anchor = profile[0];
    if !profile_contains(&profile, base, normal, input.epsilon)? { return None; }
    let count = profile.len();
    let (basis, weights) = (1..count - 1).find_map(|i| {
        let [p, q] = [profile[i], profile[i + 1]].map(|point| difference(point, anchor));
        let area = dot(cross(p, q), normal);
        if area.abs() <= input.epsilon * input.epsilon { return None; }
        let b = dot(cross(offset, q), normal) / area;
        let c = dot(cross(p, offset), normal) / area;
        let weights = [1.0 - b - c, b, c];
        weights.iter().all(|weight| *weight >= 0.0).then_some(([ring[0], ring[i], ring[i + 1]], weights))
    })?;
    let pairs = basis.map(|id| (0..count).map(|i| input.prism.pair(input.source, i)).find(|pair| pair[0] == id).unwrap());
    let make_sample = |end: usize| {
        let mut remaining = input.interval[end];
        let p = points[end];
        if planes.into_iter().chain(caps[end]).any(|plane| height(p, plane).abs() > input.epsilon) {
            return None;
        }
        let mut vertices = [basis[0]; 4];
        let mut fractions = [0.0; 4];
        let mut index = 0;
        for ([a, b], weight) in pairs.into_iter().zip(weights) {
            let upper = if caps[end].is_none() { weight * end as f64 } else { weight.min(remaining) };
            remaining -= upper;
            for (id, fraction) in [(a, weight - upper), (b, upper)] {
                if fraction > 0.0 {
                    *vertices.get_mut(index)? = id;
                    fractions[index] = fraction;
                    index += 1;
                }
            }
        }
        for (i, coordinate) in p.into_iter().enumerate() {
            let reconstructed: f64 = vertices.into_iter().zip(fractions)
                .map(|(id, weight)| f64::from(input.source.positions[id as usize][i]) * weight).sum();
            if (reconstructed - f64::from(coordinate)).abs() > input.epsilon { return None; }
        }
        Some((p, SourceVertex::Weighted { vertices, weights: fractions }))
    };
    Some([make_sample(0)?, make_sample(1)?])
}
