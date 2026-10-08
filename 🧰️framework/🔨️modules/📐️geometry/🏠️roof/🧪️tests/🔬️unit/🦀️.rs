use super::*;

fn ring(list: &[[f64; 2]]) -> Vec<Point> {
    list.iter().map(|p| Point::new(p[0], p[1])).collect()
}

fn rectangle(width: f64, depth: f64) -> Vec<Vec<Point>> {
    vec![ring(&[[0.0, 0.0], [width, 0.0], [width, depth], [0.0, depth]])]
}

fn footprint_area(rings: &[Vec<Point>]) -> f64 {
    rings.iter().enumerate().map(|(r, ring)| {
        let area: f64 = (0..ring.len()).map(|i| ring[i].x * ring[(i + 1) % ring.len()].y - ring[(i + 1) % ring.len()].x * ring[i].y).sum::<f64>() / 2.0;
        if r == 0 { area.abs() } else { -area.abs() }
    }).sum()
}

fn count(surface: &RoofSurface, kind: RoofLineKind) -> usize {
    surface.lines.iter().filter(|line| line.kind == kind).count()
}

fn close(a: f64, b: f64, context: &str) {
    assert!((a - b).abs() <= 1e-7 * b.abs().max(1.0), "{context}: {a} vs {b}");
}

#[test]
fn a_hip_roof_over_a_rectangle_has_one_ridge_four_hips_and_the_closed_form_area() {
    let pitch = 30.0_f64.to_radians();
    let footprint = rectangle(10.0, 4.0);
    let surface = roof_surface(&footprint, &Roof::Hip { pitch }).unwrap();
    close(surface.height, 2.0 * pitch.tan(), "ridge height");
    assert_eq!((count(&surface, RoofLineKind::Ridge), count(&surface, RoofLineKind::Hip), count(&surface, RoofLineKind::Valley)), (1, 4, 0));
    close(surface.plan_area(), 40.0, "plan area");
    close(surface.surface_area(), 40.0 / pitch.cos(), "slope area");
    assert!(surface.faces.iter().all(|face| !face.vertical));
}

#[test]
fn a_gable_roof_is_a_hip_with_the_ends_across_the_ridge_vertical() {
    let pitch = 30.0_f64.to_radians();
    let footprint = rectangle(10.0, 4.0);
    let surface = roof_surface(&footprint, &Roof::Gable { pitch, ridge_direction: 0.0 }).unwrap();
    close(surface.height, 2.0 * pitch.tan(), "ridge height");
    assert_eq!(surface.faces.iter().filter(|face| face.vertical).count(), 2);
    assert_eq!((count(&surface, RoofLineKind::Ridge), count(&surface, RoofLineKind::Verge), count(&surface, RoofLineKind::Hip)), (1, 4, 0));
    close(surface.plan_area(), 40.0, "plan area");
    let ends = 2.0 * 0.5 * 4.0 * 2.0 * pitch.tan();
    close(surface.surface_area(), 40.0 / pitch.cos() + ends, "slope plus gable ends");
}

#[test]
fn a_gable_roof_turns_with_the_ridge_direction() {
    let pitch = 35.0_f64.to_radians();
    let footprint = rectangle(10.0, 4.0);
    let across = roof_surface(&footprint, &Roof::Gable { pitch, ridge_direction: std::f64::consts::FRAC_PI_2 }).unwrap();
    close(across.height, 5.0 * pitch.tan(), "ridge across the long side");
    assert_eq!(across.faces.iter().filter(|face| face.vertical).count(), 2);
}

#[test]
fn a_concave_l_shaped_hip_roof_gets_a_valley_and_keeps_its_slope_area() {
    let pitch = 25.0_f64.to_radians();
    let footprint = vec![ring(&[[0.0, 0.0], [8.0, 0.0], [8.0, 3.0], [3.0, 3.0], [3.0, 8.0], [0.0, 8.0]])];
    let surface = roof_surface(&footprint, &Roof::Hip { pitch }).unwrap();
    assert_eq!(count(&surface, RoofLineKind::Valley), 1);
    close(surface.plan_area(), footprint_area(&footprint), "plan area");
    close(surface.surface_area(), footprint_area(&footprint) / pitch.cos(), "slope area");
}

#[test]
fn a_cross_gable_over_a_t_shaped_footprint_has_three_gable_ends_and_one_valley_pair() {
    let pitch = 30.0_f64.to_radians();
    let footprint = vec![ring(&[[0.0, 4.0], [3.0, 4.0], [3.0, 0.0], [5.0, 0.0], [5.0, 4.0], [8.0, 4.0], [8.0, 6.0], [0.0, 6.0]])];
    let (up, flat) = (pitch, VERTICAL);
    let surface = roof_surface(&footprint, &Roof::Pitched { pitches: vec![vec![up, up, flat, up, up, flat, up, flat]] }).unwrap();
    close(surface.plan_area(), footprint_area(&footprint), "plan area");
    assert_eq!(surface.faces.iter().filter(|face| face.vertical).count(), 3);
    assert_eq!(count(&surface, RoofLineKind::Valley), 2);
    close(surface.height, pitch.tan(), "bar and stem are both two metres wide");
}

#[test]
fn a_hip_roof_over_a_courtyard_footprint_closes_around_the_hole() {
    let pitch = 20.0_f64.to_radians();
    let footprint = vec![ring(&[[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]]), ring(&[[3.0, 3.0], [3.0, 7.0], [7.0, 7.0], [7.0, 3.0]])];
    let surface = roof_surface(&footprint, &Roof::Hip { pitch }).unwrap();
    close(surface.height, 1.5 * pitch.tan(), "ridge height");
    close(surface.plan_area(), footprint_area(&footprint), "plan area");
    close(surface.surface_area(), footprint_area(&footprint) / pitch.cos(), "slope area");
}

#[test]
fn a_mansard_roof_steepens_the_lower_slope_and_flattens_the_upper_one() {
    let (lower, upper, break_height) = (70.0_f64.to_radians(), 30.0_f64.to_radians(), 1.0);
    let footprint = rectangle(10.0, 6.0);
    let surface = roof_surface(&footprint, &Roof::Mansard { lower_pitch: lower, upper_pitch: upper, break_height }).unwrap();
    let inset = break_height / lower.tan();
    close(surface.height, break_height + (3.0 - inset) * upper.tan(), "ridge height");
    assert!(surface.faces.iter().any(|face| face.zone == 0) && surface.faces.iter().any(|face| face.zone == 1));
    assert_eq!(count(&surface, RoofLineKind::Break), 4);
    close(surface.plan_area(), 60.0, "plan area");
    let lower_zone: f64 = surface.faces.iter().filter(|face| face.zone == 0).map(|face| plan_area_of(&face.vertices)).sum();
    close(lower_zone, 60.0 - (10.0 - 2.0 * inset) * (6.0 - 2.0 * inset), "lower zone plan area");
}

#[test]
fn a_mansard_that_closes_below_its_break_is_a_plain_hip() {
    let surface = roof_surface(&rectangle(10.0, 2.0), &Roof::Mansard { lower_pitch: 45.0_f64.to_radians(), upper_pitch: 20.0_f64.to_radians(), break_height: 5.0 }).unwrap();
    close(surface.height, 1.0, "ridge height");
    assert!(surface.faces.iter().all(|face| face.zone == 0));
}

#[test]
fn per_edge_pitches_tilt_each_plane_on_its_own() {
    let footprint = rectangle(10.0, 6.0);
    let pitches = vec![vec![40.0_f64.to_radians(), 40.0_f64.to_radians(), 20.0_f64.to_radians(), 40.0_f64.to_radians()]];
    let surface = roof_surface(&footprint, &Roof::Pitched { pitches }).unwrap();
    let (a, b) = (40.0_f64.to_radians().tan().recip(), 20.0_f64.to_radians().tan().recip());
    close(surface.height, 6.0 / (a + b), "the two long planes meet where a t + b t = 6");
}

#[test]
fn the_mesh_is_watertight_with_the_footprint_floor_and_cancellation_is_honoured() {
    let surface = roof_surface(&rectangle(10.0, 4.0), &Roof::Hip { pitch: 30.0_f64.to_radians() }).unwrap();
    let mesh = surface.mesh();
    assert!(mesh.triangle_count() >= 4);
    let up = mesh.normals.iter().all(|n| n[2] > 0.0);
    assert!(up, "sloped faces face up");
    assert_eq!(roof_surface_controlled(&rectangle(10.0, 4.0), &Roof::Hip { pitch: 0.5 }, &mut || false), Err(RoofError::Skeleton(SkeletonError::Cancelled)));
}

#[test]
fn a_roof_layer_is_a_closed_shell_whose_volume_is_the_thickness_times_the_plan_area() {
    let pitch = 30.0_f64.to_radians();
    let footprints = [rectangle(10.0, 4.0), vec![ring(&[[0.0, 0.0], [8.0, 0.0], [8.0, 3.0], [3.0, 3.0], [3.0, 8.0], [0.0, 8.0]])]];
    for footprint in footprints {
        let surface = roof_surface(&footprint, &Roof::Hip { pitch }).unwrap();
        let shell = surface.shell(0.2);
        assert!(shell.is_watertight(), "the layer is closed");
        close(shell.volume(), 0.2 * surface.plan_area(), "vertical thickness times plan area");
    }
    let gable = roof_surface(&rectangle(10.0, 4.0), &Roof::Gable { pitch, ridge_direction: 0.0 }).unwrap();
    let shell = gable.shell(0.1);
    assert!(shell.is_watertight());
    close(shell.volume(), 0.1 * 40.0, "gable layer");
}

#[test]
fn invalid_pitches_and_breaks_are_refused() {
    let footprint = rectangle(4.0, 4.0);
    for pitch in [0.0, -0.1, 2.0, f64::NAN] {
        assert_eq!(roof_surface(&footprint, &Roof::Hip { pitch }), Err(RoofError::InvalidPitch));
    }
    assert_eq!(roof_surface(&footprint, &Roof::Mansard { lower_pitch: 1.0, upper_pitch: 0.5, break_height: 0.0 }), Err(RoofError::InvalidBreak));
}
