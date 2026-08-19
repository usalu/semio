//! 📐️ Surface geometry: area, orientation, zone volume, and coordinate transforms.

use crate::units::rad_to_deg;
use geometry::Vec3;

// #region 🔖️Types
/// 📐️ Surface tilt [° from horizontal] and azimuth [° clockwise from north].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TiltAzimuth {
    pub tilt_deg: f64,
    pub azimuth_deg: f64,
}

/// ✅️ Planarity validation outcome.
#[derive(Clone, Debug, PartialEq)]
pub enum PlanarValidation {
    Ok,
    TooFewVertices,
    DegenerateArea,
    NonPlanar { max_deviation_m: f64 },
}
// #endregion 🔖️Types

// #region 🔖️VecHelpers
async fn to_vec3(v: [f64; 3]) -> Vec3 {
    Vec3::new(v[0] as f32, v[1] as f32, v[2] as f32)
}

async fn from_vec3(v: Vec3) -> [f64; 3] {
    [v.x as f64, v.y as f64, v.z as f64]
}

async fn normalize(v: [f64; 3]) -> [f64; 3] {
    from_vec3(to_vec3(v).normalize())
}
// #endregion 🔖️VecHelpers

// #region 🔖️AreaNormal
/// 📏️ Signed polygon area [m²] via cross-sum (positive for CCW when viewed along outward normal).
pub async fn surface_area_m2(vertices_m: &[[f64; 3]]) -> f64 {
    if vertices_m.len() < 3 {
        return 0.0;
    }
    let origin = to_vec3(vertices_m[0]);
    let mut area = 0.0_f64;
    for i in 1..vertices_m.len() - 1 {
        let a = to_vec3(vertices_m[i]).sub(origin);
        let b = to_vec3(vertices_m[i + 1]).sub(origin);
        area += a.cross(b).length() as f64 * 0.5;
    }
    area
}

/// 🧭️ Outward unit normal from polygon winding (Newell's method).
pub async fn polygon_normal(vertices_m: &[[f64; 3]]) -> [f64; 3] {
    if vertices_m.len() < 3 {
        return [0.0, 0.0, 1.0];
    }
    let mut n = [0.0_f64; 3];
    let len = vertices_m.len();
    for i in 0..len {
        let (x0, y0, z0) = (vertices_m[i][0], vertices_m[i][1], vertices_m[i][2]);
        let (x1, y1, z1) = (vertices_m[(i + 1) % len][0], vertices_m[(i + 1) % len][1], vertices_m[(i + 1) % len][2]);
        n[0] += (y0 - y1) * (z0 + z1);
        n[1] += (z0 - z1) * (x0 + x1);
        n[2] += (x0 - x1) * (y0 + y1);
    }
    normalize(n)
}
// #endregion 🔖️AreaNormal

// #region 🔖️Orientation
/// 🧭️ Tilt from horizontal and azimuth clockwise from north (+Y) with optional north-axis offset.
pub async fn surface_tilt_azimuth(normal: [f64; 3], north_axis_deg: f64) -> TiltAzimuth {
    let n = normalize(normal);
    let tilt_deg = rad_to_deg(n[2].clamp(-1.0, 1.0).acos());
    let mut azimuth_deg = rad_to_deg(n[0].atan2(n[1]));
    if azimuth_deg < 0.0 {
        azimuth_deg += 360.0;
    }
    azimuth_deg = (azimuth_deg + north_axis_deg).rem_euclid(360.0);
    TiltAzimuth { tilt_deg, azimuth_deg }
}
// #endregion 🔖️Orientation

// #region 🔖️Volume
/// 📦️ Zone volume [m³] from closed watertight surface set (pyramid sum to interior reference point).
pub async fn zone_volume_from_surfaces(surfaces: &[&[[f64; 3]]]) -> f64 {
    let mut ref_pt = [0.0_f64; 3];
    let mut count = 0usize;
    for vertices in surfaces {
        for v in *vertices {
            ref_pt[0] += v[0];
            ref_pt[1] += v[1];
            ref_pt[2] += v[2];
            count += 1;
        }
    }
    if count == 0 {
        return 0.0;
    }
    ref_pt[0] /= count as f64;
    ref_pt[1] /= count as f64;
    ref_pt[2] /= count as f64;
    surfaces.iter().map(|face| face_pyramid_volume_m3(face, ref_pt)).sum::<f64>().abs()
}

async fn face_pyramid_volume_m3(vertices: &[[f64; 3]], ref_pt: [f64; 3]) -> f64 {
    if vertices.len() < 3 {
        return 0.0;
    }
    let mut ax = 0.0_f64;
    let mut ay = 0.0_f64;
    let mut az = 0.0_f64;
    let len = vertices.len();
    for i in 0..len {
        let v0 = vertices[i];
        let v1 = vertices[(i + 1) % len];
        ax += (v0[1] - v1[1]) * (v0[2] + v1[2]);
        ay += (v0[2] - v1[2]) * (v0[0] + v1[0]);
        az += (v0[0] - v1[0]) * (v0[1] + v1[1]);
    }
    let inv = 1.0 / len as f64;
    let cx = vertices.iter().map(|v| v[0]).sum::<f64>() * inv;
    let cy = vertices.iter().map(|v| v[1]).sum::<f64>() * inv;
    let cz = vertices.iter().map(|v| v[2]).sum::<f64>() * inv;
    let dx = cx - ref_pt[0];
    let dy = cy - ref_pt[1];
    let dz = cz - ref_pt[2];
    (ax * dx + ay * dy + az * dz) / 6.0
}
// #endregion 🔖️Volume

// #region 🔖️Validation
/// ✅️ Check polygon planarity within tolerance [m].
pub async fn validate_polygon_planar(vertices_m: &[[f64; 3]], tolerance_m: f64) -> PlanarValidation {
    if vertices_m.len() < 3 {
        return PlanarValidation::TooFewVertices;
    }
    if surface_area_m2(vertices_m) < 1e-9 {
        return PlanarValidation::DegenerateArea;
    }
    let n = polygon_normal(vertices_m);
    let anchor = vertices_m[0];
    let mut max_dev = 0.0_f64;
    for v in &vertices_m[1..] {
        let d = (v[0] - anchor[0]) * n[0] + (v[1] - anchor[1]) * n[1] + (v[2] - anchor[2]) * n[2];
        max_dev = max_dev.max(d.abs());
    }
    if max_dev > tolerance_m {
        PlanarValidation::NonPlanar { max_deviation_m: max_dev }
    } else {
        PlanarValidation::Ok
    }
}
// #endregion 🔖️Validation

// #region 🔖️Transform
/// 🔄️ Apply 4×4 transform to polygon vertices (building ↔ world).
pub async fn transform_vertices(vertices_m: &[[f64; 3]], transform: geometry::Mat4) -> Vec<[f64; 3]> {
    vertices_m.iter().map(|v| from_vec3(transform.transform_point(to_vec3(*v)))).collect()
}

/// 🔄️ Rotate direction vector (no translation).
pub async fn transform_direction(direction: [f64; 3], transform: geometry::Mat4) -> [f64; 3] {
    from_vec3(transform.transform_direction(to_vec3(direction)))
}
// #endregion 🔖️Transform

#[cfg(test)]
mod tests {
    use super::*;
    use geometry::Mat4;

    #[semio_framework_async_macros::async_test]
    async fn unit_square_area() {
        let verts = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]];
        assert!((surface_area_m2(&verts) - 1.0).abs() < 1e-6);
    }

    #[semio_framework_async_macros::async_test]
    async fn horizontal_roof_tilt_zero() {
        let ta = surface_tilt_azimuth([0.0, 0.0, 1.0], 0.0);
        assert!(ta.tilt_deg.abs() < 1e-6);
    }

    #[semio_framework_async_macros::async_test]
    async fn vertical_wall_tilt_ninety() {
        let ta = surface_tilt_azimuth([1.0, 0.0, 0.0], 0.0);
        assert!((ta.tilt_deg - 90.0).abs() < 1e-6);
    }

    #[semio_framework_async_macros::async_test]
    async fn box_volume() {
        let floor = [[0.0, 0.0, 0.0], [0.0, 3.0, 0.0], [4.0, 3.0, 0.0], [4.0, 0.0, 0.0]];
        let ceiling = [[0.0, 0.0, 3.0], [4.0, 0.0, 3.0], [4.0, 3.0, 3.0], [0.0, 3.0, 3.0]];
        let walls = [
            [[0.0, 0.0, 0.0], [0.0, 0.0, 3.0], [0.0, 3.0, 3.0], [0.0, 3.0, 0.0]],
            [[4.0, 0.0, 0.0], [4.0, 3.0, 0.0], [4.0, 3.0, 3.0], [4.0, 0.0, 3.0]],
            [[0.0, 0.0, 0.0], [4.0, 0.0, 0.0], [4.0, 0.0, 3.0], [0.0, 0.0, 3.0]],
            [[0.0, 3.0, 0.0], [0.0, 3.0, 3.0], [4.0, 3.0, 3.0], [4.0, 3.0, 0.0]],
        ];
        let mut surfaces: Vec<&[[f64; 3]]> = vec![&floor, &ceiling];
        for w in &walls {
            surfaces.push(w);
        }
        let vol = zone_volume_from_surfaces(&surfaces);
        assert!((vol - 36.0).abs() < 0.5);
    }

    #[semio_framework_async_macros::async_test]
    async fn planar_validation_ok() {
        let verts = [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [1.0, 2.0, 0.0]];
        assert_eq!(validate_polygon_planar(&verts, 1e-6), PlanarValidation::Ok);
    }

    #[semio_framework_async_macros::async_test]
    async fn identity_transform_preserves_vertices() {
        let verts = [[1.0, 2.0, 3.0]];
        let out = transform_vertices(&verts, Mat4::identity());
        assert!((out[0][0] - 1.0).abs() < 1e-5);
    }
}
