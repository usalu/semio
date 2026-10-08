use super::*;
use crate::brep::queries::analysis::{Convexity, SurfaceKind, Watertightness};
use parry3d::math::{Isometry, Vector};
use parry3d::query::PointQuery;
use parry3d::shape::{Ball, Cone, Cuboid, Cylinder, Shape};
use std::f64::consts::PI;

fn close(actual: f64, expected: f64, relative: f64, what: &str) {
    assert!((actual - expected).abs() <= relative * expected.abs().max(1.0), "{what}: {actual} vs {expected}");
}

fn sorted_principal(values: [f64; 3]) -> [f64; 3] {
    let mut out = values;
    out.sort_by(f64::total_cmp);
    out
}

fn parry_principal(properties: parry3d::mass_properties::MassProperties) -> [f64; 3] {
    let p = properties.principal_inertia();
    sorted_principal([p[0] as f64, p[1] as f64, p[2] as f64])
}

#[test]
fn session_mass_properties_agree_with_parry3d_for_every_analytic_primitive() {
    let mut kernel = Brep::new();
    let cases: Vec<(&str, GeometryHandle, parry3d::mass_properties::MassProperties, Box<dyn Shape>)> = vec![
        ("box", kernel.box_prim_sync(2.0, 3.0, 4.0).unwrap(), Cuboid::new(Vector::new(1.0, 1.5, 2.0)).mass_properties(1.0), Box::new(Cuboid::new(Vector::new(1.0, 1.5, 2.0)))),
        ("sphere", kernel.sphere_prim_sync(1.5).unwrap(), Ball::new(1.5).mass_properties(1.0), Box::new(Ball::new(1.5))),
        ("cylinder", kernel.cylinder_prim_sync(1.0, 3.0).unwrap(), Cylinder::new(1.5, 1.0).mass_properties(1.0), Box::new(Cylinder::new(1.5, 1.0))),
        ("cone", kernel.cone_prim_sync(2.0, 3.0).unwrap(), Cone::new(1.5, 2.0).mass_properties(1.0), Box::new(Cone::new(1.5, 2.0))),
    ];
    for (name, handle, reference, shape) in cases {
        let mass = kernel.mass_properties_sync(&handle, 1e-6).unwrap().volumetric.expect("solid mass");
        close(mass.volume, reference.mass() as f64, 1e-4, &format!("{name} volume"));
        let theirs = parry_principal(reference);
        let scale = theirs[2];
        for (ours, wanted) in mass.principal.values.iter().zip(theirs) {
            assert!((ours - wanted).abs() <= 1e-4 * scale, "{name} principal {ours} vs {wanted}");
        }
        let aabb = shape.compute_local_aabb();
        let bounds = kernel.bounds_sync(&handle).unwrap();
        let ours = sorted_principal(std::array::from_fn(|axis| bounds.max[axis] - bounds.min[axis]));
        let theirs = sorted_principal(std::array::from_fn(|axis| (aabb.maxs[axis] - aabb.mins[axis]) as f64));
        for (ours, theirs) in ours.into_iter().zip(theirs) {
            assert!((ours - theirs).abs() < 1e-5, "{name} extent {ours} vs {theirs}");
        }
    }
}

#[test]
fn session_distances_agree_with_parry3d() {
    let mut kernel = Brep::new();
    let a = kernel.box_prim_sync(2.0, 3.0, 4.0).unwrap();
    let ball = kernel.sphere_prim_sync(1.0).unwrap();
    let ball = kernel.translate_sync(&ball, [5.0, 1.5, 2.0]).unwrap();
    let other = kernel.sphere_prim_sync(0.5).unwrap();
    let other = kernel.translate_sync(&other, [5.0, 4.5, 6.0]).unwrap();
    let ca = Cuboid::new(Vector::new(1.0, 1.5, 2.0));
    let pa = Isometry::translation(1.0, 1.5, 2.0);
    let gap = |center: [f64; 3]| (0..3).map(|axis| ((center[axis] - [1.0, 1.5, 2.0][axis]).abs() - [1.0, 1.5, 2.0][axis]).max(0.0).powi(2)).sum::<f64>().sqrt();
    close(kernel.shape_distance_sync(&a, &ball).unwrap().distance, gap([5.0, 1.5, 2.0]) - 1.0, 1e-9, "box to ball, closed form");
    let balls = parry3d::query::distance(&Isometry::translation(5.0, 1.5, 2.0), &Ball::new(1.0), &Isometry::translation(5.0, 4.5, 6.0), &Ball::new(0.5)).unwrap() as f64;
    close(kernel.shape_distance_sync(&ball, &other).unwrap().distance, balls, 1e-5, "ball to ball against parry3d");
    for point in [[5.0f32, 1.0, 1.0], [-2.0, -1.0, 7.0], [1.0, 1.5, 2.0], [1.0, 1.5, 3.5]] {
        let theirs = ca.distance_to_point(&pa, &parry3d::math::Point::new(point[0], point[1], point[2]), true) as f64;
        let ours = kernel.point_distance_sync(&a, [point[0] as f64, point[1] as f64, point[2] as f64]).unwrap();
        if theirs > 0.0 {
            close(ours.distance, theirs, 1e-5, "point outside against parry3d");
            assert!(ours.signed.unwrap() > 0.0);
        } else {
            assert!(ours.signed.unwrap() < 0.0, "inside points are signed negative");
        }
    }
}

#[test]
fn validate_judges_the_selected_shape_not_the_whole_session() {
    let mut kernel = Brep::new();
    let sound = kernel.box_prim_sync(1.0, 1.0, 1.0).unwrap();
    let broken = kernel.box_prim_sync(2.0, 2.0, 2.0).unwrap();
    let face = kernel.body.solid_faces(kernel.solid_id(&broken).unwrap())[0];
    kernel.body.faces.get_mut(face).unwrap().flipped ^= true;
    let ok = |json: &str| json.contains("\"ok\":true");
    assert!(ok(&kernel.validate_sync(&sound).unwrap()), "{}", kernel.validate_sync(&sound).unwrap());
    assert!(!ok(&kernel.validate_sync(&broken).unwrap()));
    assert!(kernel.validity_sync(&sound).unwrap().ok);
    let verdict = kernel.watertightness_sync(&broken).unwrap();
    assert_eq!((verdict.verdict, verdict.misoriented_edges), (Watertightness::Misoriented, 4));
    assert_eq!(kernel.watertightness_sync(&sound).unwrap().verdict, Watertightness::Watertight);
    let shells = kernel.solid_shells_sync(&sound).unwrap();
    assert!(kernel.validate_sync(&shells[0]).unwrap().contains("\"ok\":true"), "validate also answers for a shell");
}

#[test]
fn a_notched_block_has_exactly_one_concave_edge_at_three_halves_pi() {
    let mut kernel = Brep::new();
    let block = kernel.box_prim_sync(4.0, 4.0, 4.0).unwrap();
    let tool = kernel.box_prim_sync(2.0, 2.0, 6.0).unwrap();
    let tool = kernel.translate_sync(&tool, [2.0, 2.0, -1.0]).unwrap();
    let notched = kernel.cut_sync(&block, &tool).unwrap();
    close(kernel.mass_properties_sync(&notched, 1e-6).unwrap().volumetric.unwrap().volume, 48.0, 1e-9, "notched volume");
    let edges = kernel.edge_table_sync(&notched).unwrap();
    let concave: Vec<_> = edges.iter().filter_map(|row| row.dihedral).filter(|d| d.convexity == Convexity::Concave).collect();
    assert_eq!(concave.len(), 1, "{edges:?}");
    assert!((concave[0].interior_angle - 1.5 * PI).abs() < 1e-9 && (concave[0].normal_angle - PI / 2.0).abs() < 1e-9);
    let convex = edges.iter().filter_map(|row| row.dihedral).filter(|d| d.convexity == Convexity::Convex).count();
    assert_eq!(convex + 1, edges.len(), "every other edge of a notched prism is convex");
    let counts = kernel.topology_counts_sync(&notched).unwrap();
    assert_eq!((counts.euler_characteristic, counts.genus), (2, Some(0)));
}

#[test]
fn a_hemisphere_is_bounded_by_its_equator_and_its_pole() {
    let mut kernel = Brep::new();
    let ball = kernel.sphere_prim_sync(2.0).unwrap();
    let slab = kernel.box_prim_sync(6.0, 6.0, 3.0).unwrap();
    let slab = kernel.translate_sync(&slab, [-3.0, -3.0, 0.0]).unwrap();
    let half = kernel.intersect_sync(&ball, &slab).unwrap();
    let bounds = kernel.bounds_sync(&half).unwrap();
    for (axis, (lo, hi)) in [(-2.0, 2.0), (-2.0, 2.0), (0.0, 2.0)].into_iter().enumerate() {
        assert!((bounds.min[axis] - lo).abs() < 1e-9 && (bounds.max[axis] - hi).abs() < 1e-9, "axis {axis}: {:?} {:?}", bounds.min, bounds.max);
    }
    close(kernel.mass_properties_sync(&half, 1e-4).unwrap().volumetric.unwrap().volume, 2.0 / 3.0 * PI * 8.0, 1e-4, "hemisphere volume");
}

#[test]
fn shells_wires_and_faces_answer_through_their_handles() {
    let mut kernel = Brep::new();
    let cylinder = kernel.cylinder_prim_sync(1.0, 3.0).unwrap();
    let shell = kernel.solid_shells_sync(&cylinder).unwrap().remove(0);
    let mass = kernel.mass_properties_sync(&shell, 1e-6).unwrap();
    close(mass.volumetric.expect("a closed shell encloses volume").volume, 3.0 * PI, 1e-5, "shell volume");
    let topology = kernel.deconstruct_sync(&cylinder).unwrap();
    let rows = kernel.face_table_sync(&cylinder, 1e-6).unwrap();
    assert_eq!(rows.iter().filter(|row| row.surface_kind == SurfaceKind::Plane).count(), 2);
    let lateral = topology.faces.iter().find(|face| kernel.mass_properties_sync(face, 1e-6).map_or(false, |m| (m.area - 6.0 * PI).abs() < 1e-3)).expect("lateral face");
    let curvature = kernel.surface_differential_sync(lateral, 1.0, 1.5).unwrap();
    assert!((curvature.principal_curvatures[0] - 1.0).abs() < 1e-9 && curvature.principal_curvatures[1].abs() < 1e-9);
    assert!(curvature.mean > 0.0 && curvature.gaussian.abs() < 1e-12);
    let wire = kernel.rectangle_wire_sync(2.0, 3.0).unwrap();
    let counts = kernel.topology_counts_sync(&wire).unwrap();
    assert_eq!((counts.wires, counts.edges, counts.faces), (1, 4, 0));
    let bounds = kernel.bounds_sync(&wire).unwrap();
    assert!((bounds.max[0] - bounds.min[0] - 2.0).abs() < 1e-9 && (bounds.max[1] - bounds.min[1] - 3.0).abs() < 1e-9);
    assert!(kernel.mass_properties_sync(&wire, 1e-6).is_err());
}

#[test]
fn a_bare_curve_has_nothing_to_analyse() {
    let mut kernel = Brep::new();
    let line = kernel.line_curve_sync([0.0; 3], [1.0, 0.0, 0.0]).unwrap();
    assert!(kernel.shape_scope(&line).is_err());
    assert!(kernel.analyze_sync(&line, 1e-6).is_err());
}

#[test]
fn the_whole_analysis_of_a_handle_is_one_value() {
    let mut kernel = Brep::new();
    let torus = kernel.torus_prim_sync(3.0, 1.0).unwrap();
    let analysis = kernel.analyze_sync(&torus, 1e-6).unwrap();
    assert_eq!(analysis.topology.genus, Some(1));
    assert!(analysis.unavailable.is_empty(), "{:?}", analysis.unavailable);
    assert_eq!((analysis.faces.len(), analysis.edges.len()), (1, 2));
    let bounds = analysis.bounds.unwrap();
    assert!(bounds.exact && (bounds.max[0] - 4.0).abs() < 1e-9 && (bounds.max[2] - 1.0).abs() < 1e-9);
    close(analysis.mass.unwrap().volumetric.unwrap().volume, 2.0 * PI * PI * 3.0, 1e-5, "torus volume");
}
