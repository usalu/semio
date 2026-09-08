
use super::*;
use geometry::Mat4;

#[test]
fn unit_square_area() {
    let verts = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]];
    assert!((surface_area_m2(&verts) - 1.0).abs() < 1e-6);
}

#[test]
fn horizontal_roof_tilt_zero() {
    let ta = surface_tilt_azimuth([0.0, 0.0, 1.0], 0.0);
    assert!(ta.tilt_deg.abs() < 1e-6);
}

#[test]
fn vertical_wall_tilt_ninety() {
    let ta = surface_tilt_azimuth([1.0, 0.0, 0.0], 0.0);
    assert!((ta.tilt_deg - 90.0).abs() < 1e-6);
}

#[test]
fn box_volume() {
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

#[test]
fn planar_validation_ok() {
    let verts = [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [1.0, 2.0, 0.0]];
    assert_eq!(validate_polygon_planar(&verts, 1e-6), PlanarValidation::Ok);
}

#[test]
fn identity_transform_preserves_vertices() {
    let verts = [[1.0, 2.0, 3.0]];
    let out = transform_vertices(&verts, Mat4::identity());
    assert!((out[0][0] - 1.0).abs() < 1e-5);
}
