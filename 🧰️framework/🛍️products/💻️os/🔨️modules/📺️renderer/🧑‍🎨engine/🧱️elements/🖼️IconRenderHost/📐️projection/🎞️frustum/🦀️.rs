//! 🎞️ Homogeneous clipping against all six camera frustum planes.

fn frustum_distance(point: [f32; 4], plane: usize) -> f32 {
    if plane.is_multiple_of(2) { point[3] + point[plane / 2] } else { point[3] - point[plane / 2] }
}

pub fn clip_polygon(points: &[[f32; 4]; 3]) -> Vec<[f32; 4]> {
    let mut input = points.to_vec();
    for plane in 0..6 {
        let mut output = Vec::with_capacity(input.len() + 1);
        for index in 0..input.len() {
            let a = input[index];
            let b = input[(index + 1) % input.len()];
            let da = frustum_distance(a, plane);
            let db = frustum_distance(b, plane);
            if da >= 0.0 {
                output.push(a);
            }
            if (da > 0.0 && db < 0.0) || (da < 0.0 && db > 0.0) {
                output.push(lerp4(a, b, da / (da - db)));
            }
        }
        input = output;
        if input.is_empty() {
            break;
        }
    }
    input
}

pub fn clip_line(mut points: [[f32; 4]; 2]) -> Option<[[f32; 4]; 2]> {
    for plane in 0..6 {
        let da = frustum_distance(points[0], plane);
        let db = frustum_distance(points[1], plane);
        if da < 0.0 && db < 0.0 {
            return None;
        }
        if da < 0.0 {
            points[0] = lerp4(points[0], points[1], da / (da - db));
        } else if db < 0.0 {
            points[1] = lerp4(points[0], points[1], da / (da - db));
        }
    }
    Some(points)
}

fn lerp4(a: [f32; 4], b: [f32; 4], t: f32) -> [f32; 4] {
    std::array::from_fn(|axis| a[axis] + (b[axis] - a[axis]) * t)
}
