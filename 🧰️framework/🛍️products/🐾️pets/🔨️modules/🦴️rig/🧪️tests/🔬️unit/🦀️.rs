//! 🩻️ Unit tests of the rig: affine products, inverses and carried points against a homogeneous 3×3 reference and numpy's committed answers, solved skeletons against a chain built with the platform's sine and cosine, the laws of the look offset, the committed bit patterns, and the ban on platform transcendentals in the module.
//!
//! @see ../../🦀️.rs — the implementation under test
//! @see ../../../../🧫️fixtures/🦴️rig-solving/🔣️.json — numpy's answers (case 🦴️rig-solving)
//! @see ../../../../🧫️fixtures/👀️gaze-tracking/🔣️.json — numpy's answers (case 👀️gaze-tracking)

use super::*;
use crate::randomness::random_between;
use crate::schema::tests::{bits, entries, fixture, number, typed};
use serde_json::Value;

const BOUND: f64 = 1e-12;
const SEED: u32 = 20_261_002;

type Homogeneous = [[f64; 3]; 3];

fn species_of(vectors: &Value, id: &str) -> Species {
    entries(&vectors["species"]).iter().find(|candidate| candidate["id"] == id).map_or_else(|| panic!("the vectors name the unknown species {id}"), typed)
}

fn rig_of(vectors: &Value, bones: Vec<Bone>) -> Species {
    Species { id: "probe".to_string(), bones, ..species_of(vectors, "blobby") }
}

fn bone(id: &str, parent: Option<&str>, x: f64, y: f64) -> Bone {
    Bone { id: id.to_string(), parent: parent.map(str::to_string), x, y, rotation: None }
}

fn posed(species: &Species, change: impl Fn(&str, &mut BonePose)) -> Pose {
    species
        .bones
        .iter()
        .map(|bone| {
            let mut pose = REST;
            change(&bone.id, &mut pose);
            pose
        })
        .collect()
}

fn affine(value: &Value) -> Affine {
    typed(value)
}

fn numbers(value: &Value) -> Vec<f64> {
    entries(value).iter().map(number).collect()
}

fn patterns(values: &[f64]) -> Vec<Value> {
    values.iter().map(|value| Value::String(bits(*value))).collect()
}

fn point_patterns(point: Point) -> Value {
    serde_json::json!({"x": bits(point.x), "y": bits(point.y)})
}

fn drawn(stream: u32, index: u32) -> Affine {
    let entry = |slot: u32, reach: f64| random_between(&[SEED, stream, index * 6 + slot], -reach, reach);
    [entry(0, 4.0), entry(1, 4.0), entry(2, 4.0), entry(3, 4.0), entry(4, 40.0), entry(5, 40.0)]
}

fn gap(left: &[f64], right: &[f64]) -> f64 {
    assert_eq!(left.len(), right.len());
    left.iter().zip(right).map(|(left, right)| (left - right).abs() / right.abs().max(1.0)).fold(0.0, f64::max)
}

fn same(left: &[f64], right: &[f64]) -> bool {
    left.len() == right.len() && left.iter().zip(right).all(|(left, right)| left == right)
}

fn homogeneous(matrix: Affine) -> Homogeneous {
    [[matrix[0], matrix[2], matrix[4]], [matrix[1], matrix[3], matrix[5]], [0.0, 0.0, 1.0]]
}

fn flat(matrix: Homogeneous) -> Affine {
    [matrix[0][0], matrix[1][0], matrix[0][1], matrix[1][1], matrix[0][2], matrix[1][2]]
}

fn product(left: Homogeneous, right: Homogeneous) -> Homogeneous {
    let mut result = [[0.0; 3]; 3];
    for (row, cells) in result.iter_mut().enumerate() {
        for (column, cell) in cells.iter_mut().enumerate() {
            *cell = (0..3).map(|inner| left[row][inner] * right[inner][column]).sum();
        }
    }
    result
}

fn chained(species: &Species, pose: &[BonePose]) -> Vec<f64> {
    let mut worlds: Vec<(String, Homogeneous)> = Vec::new();
    let mut numbers = Vec::new();
    for (index, bone) in species.bones.iter().enumerate() {
        let entry = pose.get(index).copied().unwrap_or(REST);
        let (sine, cosine) = ((bone.rotation.unwrap_or(0.0) + entry.rotation) * std::f64::consts::PI / 180.0).sin_cos();
        let translate = [[1.0, 0.0, bone.x + entry.x], [0.0, 1.0, bone.y + entry.y], [0.0, 0.0, 1.0]];
        let rotate = [[cosine, -sine, 0.0], [sine, cosine, 0.0], [0.0, 0.0, 1.0]];
        let scale = [[entry.scale_x, 0.0, 0.0], [0.0, entry.scale_y, 0.0], [0.0, 0.0, 1.0]];
        let local = product(product(translate, rotate), scale);
        let parent = bone.parent.as_ref().and_then(|parent| worlds.iter().rev().find(|(id, _)| id == parent)).map(|(_, world)| *world);
        let world = parent.map_or(local, |parent| product(parent, local));
        worlds.push((bone.id.clone(), world));
        numbers.extend(flat(world));
    }
    numbers
}

fn composed(species: &Species, pose: &[BonePose]) -> Vec<f64> {
    let mut worlds: Vec<(String, Affine)> = Vec::new();
    let mut numbers = Vec::new();
    for (index, bone) in species.bones.iter().enumerate() {
        let entry = pose.get(index).copied().unwrap_or(REST);
        let turns = (bone.rotation.unwrap_or(0.0) + entry.rotation) / 360.0;
        let local: Affine = [cos_turns(turns) * entry.scale_x, sin_turns(turns) * entry.scale_x, (0.0 - sin_turns(turns)) * entry.scale_y, cos_turns(turns) * entry.scale_y, bone.x + entry.x, bone.y + entry.y];
        let parent = bone.parent.as_ref().and_then(|parent| worlds.iter().rev().find(|(id, _)| id == parent)).map(|(_, world)| *world);
        let world = parent.map_or(local, |parent| compose(parent, local));
        worlds.push((bone.id.clone(), world));
        numbers.extend(world);
    }
    numbers
}

#[test]
fn a_product_is_the_homogeneous_matrix_product_and_applies_the_local_matrix_first() {
    for index in 0..500 {
        let (parent, local) = (drawn(1, index), drawn(2, index));
        assert!(gap(&compose(parent, local), &flat(product(homogeneous(parent), homogeneous(local)))) <= BOUND, "{index}");
    }
    for index in 0..100 {
        let (first, second, third) = (drawn(3, index), drawn(4, index), drawn(5, index));
        assert!(same(&compose(IDENTITY, first), &first));
        assert!(same(&compose(first, IDENTITY), &first));
        assert!(gap(&compose(compose(first, second), third), &compose(first, compose(second, third))) <= BOUND);
    }
    assert_eq!(transform(compose([1.0, 0.0, 0.0, 1.0, 10.0, 0.0], [0.0, 1.0, -1.0, 0.0, 0.0, 0.0]), 1.0, 0.0), Point { x: 10.0, y: 1.0 });
    let vectors = fixture("rig-solving");
    assert!(entries(&vectors["products"]).len() > 5);
    for vector in entries(&vectors["products"]) {
        assert!(gap(&compose(affine(&vector["parent"]), affine(&vector["local"])), &numbers(&vector["expected"])) <= BOUND, "{}", vector["id"]);
    }
}

#[test]
fn an_inverse_undoes_its_matrix_and_a_matrix_without_one_yields_the_identity() {
    for index in 0..500 {
        let matrix = drawn(6, index);
        if (matrix[0] * matrix[3] - matrix[1] * matrix[2]).abs() < 0.05 {
            continue;
        }
        let inverse = invert(matrix);
        assert!(gap(&compose(matrix, inverse), &IDENTITY) <= 1e-11, "{index}");
        assert!(gap(&compose(inverse, matrix), &IDENTITY) <= 1e-11, "{index}");
    }
    for matrix in [[0.0, 0.0, 0.0, 0.0, 3.0, 4.0], [1.0, 2.0, 2.0, 4.0, 5.0, 6.0], [2.0, 0.5, 0.0, 0.0, 3.0, 4.0], [0.0, 0.0, 1.0, 1.0, 0.0, 0.0]] {
        assert_eq!(invert(matrix), IDENTITY);
    }
    assert!(same(&invert(IDENTITY), &IDENTITY));
    assert!(same(&invert([1.0, 0.0, 0.0, 1.0, 12.5, -16.0]), &[1.0, 0.0, 0.0, 1.0, -12.5, 16.0]));
    assert!(same(&invert([0.0, 1.0, -1.0, 0.0, 3.0, 4.0]), &[0.0, -1.0, 1.0, 0.0, -4.0, 3.0]));
    assert!(same(&invert([2.0, 0.0, 0.0, 4.0, 6.0, 8.0]), &[0.5, 0.0, 0.0, 0.25, -3.0, -2.0]));
    let vectors = fixture("rig-solving");
    assert!(entries(&vectors["inverses"]).len() > 5);
    for vector in entries(&vectors["inverses"]) {
        assert!(gap(&invert(affine(&vector["matrix"])), &numbers(&vector["expected"])) <= 1e-11, "{}", vector["id"]);
        if vector["singular"] == true {
            assert_eq!(invert(affine(&vector["matrix"])), IDENTITY, "{}", vector["id"]);
        }
    }
}

#[test]
fn a_point_is_carried_like_a_homogeneous_vector_and_returns_through_the_inverse() {
    for index in 0..500 {
        let matrix = drawn(7, index);
        let (x, y) = (random_between(&[SEED, 8, index], -2000.0, 2000.0), random_between(&[SEED, 9, index], -2000.0, 2000.0));
        let carried = transform(matrix, x, y);
        let rows = homogeneous(matrix);
        assert!(gap(&[carried.x, carried.y], &[rows[0][0] * x + rows[0][1] * y + rows[0][2], rows[1][0] * x + rows[1][1] * y + rows[1][2]]) <= BOUND);
    }
    for index in 0..100 {
        let matrix = drawn(10, index);
        assert_eq!(transform(matrix, 0.0, 0.0), Point { x: matrix[4], y: matrix[5] });
        if (matrix[0] * matrix[3] - matrix[1] * matrix[2]).abs() < 0.05 {
            continue;
        }
        let there = transform(matrix, 3.5, -7.25);
        let back = transform(invert(matrix), there.x, there.y);
        assert!(gap(&[back.x, back.y], &[3.5, -7.25]) <= 1e-10);
    }
    let vectors = fixture("rig-solving");
    assert!(entries(&vectors["points"]).len() > 5);
    for vector in entries(&vectors["points"]) {
        let carried = transform(affine(&vector["matrix"]), number(&vector["x"]), number(&vector["y"]));
        assert!(gap(&[carried.x, carried.y], &[number(&vector["expected"]["x"]), number(&vector["expected"]["y"])]) <= BOUND, "{}", vector["id"]);
    }
}

#[test]
fn a_rest_pose_holds_one_rest_entry_per_bone() {
    let vectors = fixture("rig-solving");
    for species in entries(&vectors["species"]).iter().map(typed::<Species>) {
        assert_eq!(rest_pose(&species), vec![REST; species.bones.len()]);
    }
    assert!(rest_pose(&rig_of(&vectors, Vec::new())).is_empty());
    assert_eq!(serde_json::to_value(REST).ok(), Some(serde_json::json!({"x": 0.0, "y": 0.0, "rotation": 0.0, "scaleX": 1.0, "scaleY": 1.0})));
    assert!(serde_json::from_value::<BonePose>(serde_json::json!({"x": 0, "y": 0, "rotation": 0, "scaleX": 1, "scaleY": 1, "skew": 0})).is_err());
    assert!(!entries(&vectors["restPoses"]).is_empty());
    for vector in entries(&vectors["restPoses"]) {
        let species = species_of(&vectors, vector["species"].as_str().unwrap_or_default());
        assert_eq!(rest_pose(&species), typed::<Pose>(&vector["expected"]["pose"]), "{}", vector["id"]);
        assert!(gap(&solve_rig(&species, &rest_pose(&species)), &numbers(&vector["expected"]["bones"])) <= BOUND, "{}", vector["id"]);
    }
}

#[test]
fn the_reference_species_rests_with_its_root_on_the_feet_and_its_limbs_on_the_body() {
    let vectors = fixture("rig-solving");
    let blobby = species_of(&vectors, "blobby");
    let bones = solve_rig(&blobby, &rest_pose(&blobby));
    let at = |id: &str| {
        let index = blobby.bones.iter().position(|bone| bone.id == id).unwrap_or_else(|| panic!("no bone {id}"));
        bones[index * 6..index * 6 + 6].to_vec()
    };
    assert_eq!(bones.len(), blobby.bones.len() * 6);
    assert_eq!(at("root"), [1.0, 0.0, 0.0, 1.0, 0.0, 0.0]);
    assert_eq!(at("body"), [1.0, 0.0, 0.0, 1.0, 0.0, -16.0]);
    assert_eq!(at("leg-left"), [1.0, 0.0, 0.0, 1.0, -6.0, -7.0]);
    assert_eq!(at("tuft"), [1.0, 0.0, 0.0, 1.0, 0.0, -29.0]);
    let turns = 20.0 / 360.0;
    assert_eq!(at("arm-left"), [cos_turns(turns), sin_turns(turns), 0.0 - sin_turns(turns), cos_turns(turns), -13.0, -16.0]);
}

#[test]
fn a_skeleton_multiplies_with_the_expressions_of_compose_bit_for_bit() {
    let vectors = fixture("rig-solving");
    assert!(entries(&vectors["skeletons"]).len() > 15);
    for vector in entries(&vectors["skeletons"]) {
        let (species, pose) = (species_of(&vectors, vector["species"].as_str().unwrap_or_default()), typed::<Pose>(&vector["pose"]));
        assert!(same(&solve_rig(&species, &pose), &composed(&species, &pose)), "{}", vector["id"]);
        assert!(gap(&solve_rig(&species, &pose), &numbers(&vector["expected"])) <= BOUND, "{}", vector["id"]);
        assert!(gap(&solve_rig(&species, &pose), &chained(&species, &pose)) <= BOUND, "{}", vector["id"]);
    }
}

#[test]
fn children_follow_exactly_when_a_parent_makes_a_quarter_turn_or_is_scaled() {
    let vectors = fixture("rig-solving");
    let blobby = species_of(&vectors, "blobby");
    let turned = solve_rig(
        &blobby,
        &posed(&blobby, |id, pose| {
            if id == "root" {
                pose.rotation = 90.0;
            }
        }),
    );
    assert_eq!(turned[..6], [0.0, 1.0, -1.0, 0.0, 0.0, 0.0]);
    assert_eq!(turned[6..12], [0.0, 1.0, -1.0, 0.0, 16.0, 0.0]);
    let squashed = solve_rig(
        &blobby,
        &posed(&blobby, |id, pose| {
            if id == "root" {
                (pose.scale_x, pose.scale_y) = (2.0, 0.5);
            }
        }),
    );
    assert_eq!(squashed[6..12], [2.0, 0.0, 0.0, 0.5, 0.0, -8.0]);
    assert_eq!(squashed[12..18], [2.0, 0.0, 0.0, 0.5, -12.0, -3.5]);
    let rest = solve_rig(&blobby, &rest_pose(&blobby));
    let spun = solve_rig(
        &blobby,
        &posed(&blobby, |id, pose| match id {
            "root" => pose.rotation = 360.0,
            "body" => pose.rotation = -720.0,
            "leg-left" => pose.rotation = 1080.0,
            _ => {}
        }),
    );
    assert!(same(&spun[..18], &rest[..18]));
    assert!(gap(&spun, &rest) <= BOUND);
}

#[test]
fn bones_beyond_the_pose_stay_at_rest_and_an_unlisted_parent_counts_as_none() {
    let vectors = fixture("rig-solving");
    let blobby = species_of(&vectors, "blobby");
    let full = posed(&blobby, |id, pose| {
        if id == "body" {
            (pose.rotation, pose.scale_y) = (30.0, 1.1);
        }
    });
    assert_eq!(solve_rig(&blobby, &full[..2]), solve_rig(&blobby, &full));
    assert_eq!(solve_rig(&blobby, &[]), solve_rig(&blobby, &rest_pose(&blobby)));
    assert!(solve_rig(&rig_of(&vectors, Vec::new()), &[]).is_empty());
    let orphan = rig_of(&vectors, vec![bone("root", None, 1.0, 2.0), bone("stray", Some("later"), 5.0, 6.0), bone("later", Some("root"), 10.0, 20.0)]);
    assert_eq!(solve_rig(&orphan, &[]), [1.0, 0.0, 0.0, 1.0, 1.0, 2.0, 1.0, 0.0, 0.0, 1.0, 5.0, 6.0, 1.0, 0.0, 0.0, 1.0, 11.0, 22.0]);
    let twice = rig_of(&vectors, vec![bone("root", None, 1.0, 2.0), bone("root", None, 100.0, 200.0), bone("leaf", Some("root"), 10.0, 20.0)]);
    assert_eq!(solve_rig(&twice, &[])[12..], [1.0, 0.0, 0.0, 1.0, 110.0, 220.0]);
}

#[test]
fn a_look_offset_points_at_the_target_inside_the_unit_disc() {
    let point = |x: f64, y: f64| Point { x, y };
    assert_eq!(look_offset(point(12.5, -3.0), point(12.5, -3.0), 40.0), point(0.0, 0.0));
    assert_eq!(look_offset(point(0.0, 0.0), point(0.0, 0.0), 0.0), point(0.0, 0.0));
    assert_eq!(look_offset(point(0.0, 0.0), point(40.0, 0.0), 40.0), point(0.5, 0.0));
    assert_eq!(look_offset(point(0.0, 0.0), point(0.0, -40.0), 40.0), point(0.0, -0.5));
    assert_eq!(look_offset(point(10.0, 20.0), point(13.0, 24.0), 5.0), point(0.3, 0.4));
    assert_eq!(look_offset(point(3.0, 4.0), point(0.0, 0.0), 0.0), point(-0.6, -0.8));
    assert_eq!(look_offset(point(3.0, 4.0), point(0.0, 0.0), -25.0), point(-0.6, -0.8));
    let mut last = 0.0;
    for distance in [0.001, 1.0, 10.0, 40.0, 100.0, 1000.0, 1e6, 1e12] {
        let offset = look_offset(point(5.0, 5.0), point(5.0 + distance, 5.0), 40.0);
        assert!(offset.x > last && offset.x < 1.0, "{distance}");
        last = offset.x;
    }
    for index in 0..2000 {
        let eye = point(random_between(&[SEED, 16, index], 0.0, 1920.0), random_between(&[SEED, 17, index], 0.0, 1080.0));
        let target = point(random_between(&[SEED, 18, index], -200.0, 2120.0), random_between(&[SEED, 19, index], -200.0, 1280.0));
        let reach = random_between(&[SEED, 20, index], 1.0, 200.0);
        let offset = look_offset(eye, target, reach);
        let distance = ((target.x - eye.x) * (target.x - eye.x) + (target.y - eye.y) * (target.y - eye.y)).sqrt();
        let length = (offset.x * offset.x + offset.y * offset.y).sqrt();
        assert!(length < 1.0);
        assert!((length - distance / (distance + reach)).abs() <= BOUND);
        assert!((offset.x * (target.y - eye.y) - offset.y * (target.x - eye.x)).abs() <= 1e-9);
        assert!(offset.x * (target.x - eye.x) + offset.y * (target.y - eye.y) > 0.0);
    }
}

#[test]
fn a_pupil_travels_up_to_the_outline_of_its_white_and_never_backwards() {
    let eye = |radius: f64, pupil: f64| Eye { id: "eye".to_string(), bone: "head".to_string(), x: 0.0, y: 0.0, radius, pupil };
    assert!(pupil_reach(&eye(4.0, 1.5)) == 2.25 && pupil_reach(&eye(3.0, 2.75)) == 0.0);
    assert!(pupil_reach(&eye(2.0, 2.0)).to_bits() == 0.0_f64.to_bits() && pupil_reach(&eye(1.0, 3.0)).to_bits() == 0.0_f64.to_bits());
    assert!(pupil_reach(&eye(f64::NAN, 1.0)).to_bits() == 0.0_f64.to_bits());
}

#[test]
fn numpy_agrees_on_every_committed_offset_and_every_committed_eye_on_a_posed_bone() {
    let vectors = fixture("gaze-tracking");
    assert!(entries(&vectors["offsets"]).len() > 10);
    for vector in entries(&vectors["offsets"]) {
        let offset = look_offset(typed(&vector["eye"]), typed(&vector["target"]), number(&vector["reach"]));
        assert!(gap(&[offset.x, offset.y], &[number(&vector["expected"]["x"]), number(&vector["expected"]["y"])]) <= BOUND, "{}", vector["id"]);
    }
    assert!(entries(&vectors["eyes"]).len() > 3);
    for vector in entries(&vectors["eyes"]) {
        let resting: Point = typed(&vector["eye"]);
        let eye = transform(affine(&vector["bone"]), resting.x, resting.y);
        let offset = look_offset(eye, typed(&vector["target"]), number(&vector["reach"]));
        let expected = &vector["expected"];
        assert!(gap(&[eye.x, eye.y, offset.x, offset.y], &[number(&expected["eye"]["x"]), number(&expected["eye"]["y"]), number(&expected["offset"]["x"]), number(&expected["offset"]["y"])]) <= BOUND, "{}", vector["id"]);
    }
}

#[test]
fn every_committed_product_inverse_point_skeleton_offset_and_eye_has_the_committed_bit_pattern() {
    let rig = fixture("rig-solving");
    let mut compared = 0;
    for vector in entries(&rig["products"]) {
        assert_eq!(patterns(&compose(affine(&vector["parent"]), affine(&vector["local"]))), entries(&vector["bits"]), "{}", vector["id"]);
        compared += 6;
    }
    for vector in entries(&rig["inverses"]) {
        assert_eq!(patterns(&invert(affine(&vector["matrix"]))), entries(&vector["bits"]), "{}", vector["id"]);
        compared += 6;
    }
    for vector in entries(&rig["points"]) {
        assert_eq!(point_patterns(transform(affine(&vector["matrix"]), number(&vector["x"]), number(&vector["y"]))), vector["bits"], "{}", vector["id"]);
        compared += 2;
    }
    for vector in entries(&rig["skeletons"]) {
        let (species, pose) = (species_of(&rig, vector["species"].as_str().unwrap_or_default()), typed::<Pose>(&vector["pose"]));
        assert_eq!(patterns(&solve_rig(&species, &pose)), entries(&vector["bits"]), "{}", vector["id"]);
        compared += species.bones.len() * 6;
    }
    let gaze = fixture("gaze-tracking");
    for vector in entries(&gaze["offsets"]) {
        assert_eq!(point_patterns(look_offset(typed(&vector["eye"]), typed(&vector["target"]), number(&vector["reach"]))), vector["bits"], "{}", vector["id"]);
        compared += 2;
    }
    for vector in entries(&gaze["eyes"]) {
        let resting: Point = typed(&vector["eye"]);
        let eye = transform(affine(&vector["bone"]), resting.x, resting.y);
        let offset = look_offset(eye, typed(&vector["target"]), number(&vector["reach"]));
        assert_eq!(serde_json::json!({"eye": point_patterns(eye), "offset": point_patterns(offset)}), vector["bits"], "{}", vector["id"]);
        compared += 4;
    }
    assert!(compared > 900, "{compared}");
}

#[test]
fn the_module_calls_no_platform_transcendental_and_fuses_no_product() {
    let source = include_str!("../../🦀️.rs");
    for call in [".sin(", ".cos(", ".tan(", ".atan2(", ".exp(", ".powf(", ".powi(", ".hypot(", ".ln(", ".mul_add(", ".sin_cos(", "Instant", "SystemTime"] {
        assert!(!source.contains(call), "{call}");
    }
}

mod quick {
    use super::*;

    #[test]
    fn a_chain_built_with_the_platform_sine_and_cosine_agrees_for_drawn_poses() {
        let vectors = fixture("rig-solving");
        for species in entries(&vectors["species"]).iter().map(typed::<Species>) {
            for index in 0..50u32 {
                let pose: Pose = (0..species.bones.len() as u32)
                    .map(|bone| BonePose {
                        x: random_between(&[SEED, 11, index * 64 + bone], -8.0, 8.0),
                        y: random_between(&[SEED, 12, index * 64 + bone], -8.0, 8.0),
                        rotation: random_between(&[SEED, 13, index * 64 + bone], -720.0, 720.0),
                        scale_x: random_between(&[SEED, 14, index * 64 + bone], -1.5, 1.5),
                        scale_y: random_between(&[SEED, 15, index * 64 + bone], 0.5, 1.5),
                    })
                    .collect();
                assert!(gap(&solve_rig(&species, &pose), &chained(&species, &pose)) <= 1e-11, "{} {index}", species.id);
                assert!(same(&solve_rig(&species, &pose), &composed(&species, &pose)), "{} {index}", species.id);
            }
        }
    }
}
