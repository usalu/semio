//! ️tests for example `🌙️capsule-dream`.

#[test]
async fn dsl_asset_parses_and_round_trips() {
    let text = include_str!("../🖼️assets/🗣️dream.dsl.semio");
    assert!(text.len() > 64, "dsl fixture must carry real payload");
    let projection = crate::artifacts::puzzle5d::dsl::parse_dsl(text).expect("example dsl parses");
    assert_eq!(projection.parts.len(), 2880);
    assert_eq!(projection.fasteners.len(), 2864);
    semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&projection);
}

#[test]
async fn flatten_matches_golden_poses_to_1e4() {
    let text = include_str!("../🖼️assets/🗣️dream.dsl.semio");
    let mut projection = crate::artifacts::puzzle5d::dsl::parse_dsl(text).expect("example dsl parses");
    crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::inferences::flat_position::flatten_snapshot_inplace(&mut projection);
    let golden: serde_json::Map<String, serde_json::Value> =
        serde_json::from_str(include_str!("../🖼️assets/🏅golden-poses.json")).expect("golden json");
    assert_eq!(golden.len(), 2880);
    let mut center_mismatches = 0usize;
    let mut origin_mismatches = 0usize;
    for part in &projection.parts {
        let expected = golden.get(&part.id).expect("golden");
        let origin = expected.get("origin").and_then(|v| v.as_array()).unwrap();
        let center = expected.get("center").unwrap();
        let ex = [
            origin[0].as_f64().unwrap(),
            origin[1].as_f64().unwrap(),
            origin[2].as_f64().unwrap(),
        ];
        let cx = center.get("x").and_then(|v| v.as_f64()).unwrap();
        let cy = center.get("y").and_then(|v| v.as_f64()).unwrap();
        if (part.part_2d.x - cx).abs() > 1e-4 || (part.part_2d.y - cy).abs() > 1e-4 {
            center_mismatches += 1;
        }
        let err = (part.part_3d.origin[0] - ex[0]).abs()
            .max((part.part_3d.origin[1] - ex[1]).abs())
            .max((part.part_3d.origin[2] - ex[2]).abs());
        if err > 1e-4 {
            origin_mismatches += 1;
        }
    }
    assert_eq!(center_mismatches, 0, "{center_mismatches} center mismatches");
    assert_eq!(origin_mismatches, 0, "{origin_mismatches} origin mismatches");
}

#[test]
async fn op_pack_and_spr_assets_are_nonempty() {
    assert!(include_str!("../🖼️assets/🔧️dream.op.semio").len() > 64);
    assert!(include_bytes!("../🖼️assets/🎒️dream.pack.semio").len() > 64);
    assert!(include_bytes!("../🖼️assets/📡️dream.spr.semio").len() > 64);
}

#[test]
async fn inference_default_law() {
    use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::inferences::Puzzle5dInference;
    use protocol::Inference;
    assert_eq!(
        Puzzle5dInference::infer(&crate::artifacts::puzzle5d::Puzzle5dSnapshot::default()),
        Puzzle5dInference::default()
    );
}

#[test]
async fn inference_determinism_law() {
    use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::inferences::Puzzle5dInference;
    use protocol::Inference;
    let text = include_str!("../🖼️assets/🗣️dream.dsl.semio");
    let projection = crate::artifacts::puzzle5d::dsl::parse_dsl(text).expect("example dsl parses");
    assert_eq!(Puzzle5dInference::infer(&projection), Puzzle5dInference::infer(&projection));
}
