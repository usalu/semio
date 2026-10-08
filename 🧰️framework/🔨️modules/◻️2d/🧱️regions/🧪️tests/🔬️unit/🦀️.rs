use super::*;
use serde_json::Value;

fn fixtures() -> Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap()
}

fn ring(v: &Value) -> Vec<Vec2> {
    v.as_array().unwrap().iter().map(|p| [p[0].as_f64().unwrap(), p[1].as_f64().unwrap()]).collect()
}

fn regions_of(v: &Value) -> Vec<Region> {
    v.as_array().unwrap().iter().map(|r| Region::new(&ring(r), &[])).collect()
}

fn join(v: &Value) -> OffsetJoin {
    match v["kind"].as_str().unwrap() {
        "miter" => OffsetJoin::Miter { limit: v["limit"].as_f64().unwrap() },
        "bevel" => OffsetJoin::Bevel,
        _ => OffsetJoin::Round { tolerance: v["tolerance"].as_f64().unwrap() },
    }
}

fn total_area(regions: &[Region]) -> f64 {
    regions.iter().map(Region::area).sum()
}

fn hole_count(regions: &[Region]) -> usize {
    regions.iter().map(|r| r.holes.len()).sum()
}

#[test]
fn offsets_match_closed_form_fixtures() {
    for case in fixtures()["offsets"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let holes: Vec<Vec<Vec2>> = case["holes"].as_array().unwrap().iter().map(ring).collect();
        let region = Region::new(&ring(&case["outer"]), &holes);
        let got = offset_regions(&[region], case["distance"].as_f64().unwrap(), join(&case["join"]), &mut |_| true).unwrap();
        let e = &case["expected"];
        assert_eq!(got.len(), e["regions"].as_u64().unwrap() as usize, "{name}: regions");
        assert_eq!(hole_count(&got), e["holes"].as_u64().unwrap() as usize, "{name}: holes");
        assert!((total_area(&got) - e["area"].as_f64().unwrap()).abs() <= e["tolerance"].as_f64().unwrap(), "{name}: area {}", total_area(&got));
        for r in &got {
            assert!(signed_area(&r.outer) > 0.0 && r.holes.iter().all(|h| signed_area(h) < 0.0), "{name}: orientation");
        }
    }
}

#[test]
fn booleans_match_closed_form_fixtures() {
    for case in fixtures()["booleans"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let operation = BooleanOperation::parse(case["operation"].as_str().unwrap()).unwrap();
        let got = region_boolean(operation, &regions_of(&case["subject"]), &regions_of(&case["clip"]), &mut |_| true).unwrap();
        let e = &case["expected"];
        assert_eq!(got.len(), e["regions"].as_u64().unwrap() as usize, "{name}: regions");
        assert_eq!(hole_count(&got), e["holes"].as_u64().unwrap() as usize, "{name}: holes");
        assert!((total_area(&got) - e["area"].as_f64().unwrap()).abs() <= 1e-9, "{name}: area {}", total_area(&got));
    }
}

#[test]
fn control_can_cancel_without_a_partial_result() {
    let big: Vec<Vec2> = (0..400).map(|k| [10.0 * (k as f64 * 0.0157).cos(), 10.0 * (k as f64 * 0.0157).sin()]).collect();
    let region = Region::new(&big, &[]);
    let mut calls = 0;
    let result = offset_regions(&[region.clone()], 0.5, OffsetJoin::Round { tolerance: 1e-3 }, &mut |progress| {
        calls += 1;
        assert!(!progress.done);
        false
    });
    assert_eq!(result, Err(BooleanError::Cancelled));
    assert_eq!(calls, 1);
    let complete = offset_regions(&[region], 0.5, OffsetJoin::Round { tolerance: 1e-3 }, &mut |_| true).unwrap();
    assert_eq!(complete.len(), 1);
}

#[test]
fn invalid_distances_are_rejected_and_zero_is_the_identity() {
    let region = Region::new(&[[0.0, 0.0], [1.0, 0.0], [1.0, 1.0]], &[]);
    assert!(offset_regions(&[region.clone()], f64::NAN, OffsetJoin::Bevel, &mut |_| true).is_err());
    assert_eq!(offset_regions(&[region.clone()], 0.0, OffsetJoin::Bevel, &mut |_| true).unwrap(), vec![region]);
}

#[test]
fn sharp_corners_obey_the_miter_limit() {
    let spike = Region::new(&[[0.0, 0.0], [10.0, 0.0], [0.0, 0.5]], &[]);
    let long = offset_regions(&[spike.clone()], 0.1, OffsetJoin::Miter { limit: 100.0 }, &mut |_| true).unwrap();
    let cut = offset_regions(&[spike], 0.1, OffsetJoin::Miter { limit: 2.0 }, &mut |_| true).unwrap();
    assert!(total_area(&cut) < total_area(&long));
}
