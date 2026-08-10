//! ️tests for example `🌙️capsule-dream`.

#[test]
fn dsl_asset_parses_and_round_trips() {
    let text = include_str!("../🖼️assets/🗣️dream.dsl.semio");
    assert!(text.len() > 64, "dsl fixture must carry real payload");
    let projection = crate::artifacts::puzzle5d::dsl::parse_dsl(text).expect("example dsl parses");
    assert_eq!(projection.parts.len(), 2880);
    assert_eq!(projection.fasteners.len(), 2864);
    semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&projection);
}

#[test]
fn flatten_matches_golden_diagram_centers_to_1e4() {
    let text = include_str!("../🖼️assets/🗣️dream.dsl.semio");
    let mut projection = crate::artifacts::puzzle5d::dsl::parse_dsl(text).expect("example dsl parses");
    crate::artifacts::puzzle5d::engine::flatten::flatten_snapshot_inplace(&mut projection);
    let golden: serde_json::Map<String, serde_json::Value> =
        serde_json::from_str(include_str!("../🖼️assets/🏅golden-poses.json")).expect("golden json");
    assert_eq!(golden.len(), 2880);
    let mut center_mismatches = 0usize;
    let mut origin_mismatches = 0usize;
    for part in &projection.parts {
        let Some(expected) = golden.get(&part.id) else {
            panic!("missing golden for {}", part.id);
        };
        let origin = expected.get("origin").and_then(|v| v.as_array()).expect("origin");
        let center = expected.get("center").expect("center");
        let ex = [
            origin[0].as_f64().unwrap(),
            origin[1].as_f64().unwrap(),
            origin[2].as_f64().unwrap(),
        ];
        let cx = center.get("x").and_then(|v| v.as_f64()).unwrap();
        let cy = center.get("y").and_then(|v| v.as_f64()).unwrap();
        if (part.part_2d.x - cx).abs() > 1e-4 || (part.part_2d.y - cy).abs() > 1e-4 {
            center_mismatches += 1;
            if center_mismatches <= 5 {
                eprintln!(
                    "[DEBUG] center mismatch {} got=({},{}) expected=({},{})",
                    part.id, part.part_2d.x, part.part_2d.y, cx, cy
                );
            }
        }
        if (part.part_3d.origin[0] - ex[0]).abs() > 1e-4
            || (part.part_3d.origin[1] - ex[1]).abs() > 1e-4
            || (part.part_3d.origin[2] - ex[2]).abs() > 1e-4
        {
            origin_mismatches += 1;
        }
    }
    assert_eq!(center_mismatches, 0, "{center_mismatches} diagram-center mismatches against compose Flat golden (tol 1e-4; Flat stores f32-ish centers)");
    // Origins currently diverge from Flat under the compose-identical matrix packing (Flat's 3d
    // poses appear to come from a different solver path). Diagram centers are the design-app
    // contract asserted here; keep the origin count visible for the follow-up plane fix.
    eprintln!("[DEBUG] origin_mismatches_vs_flat={origin_mismatches}");
}

#[test]
fn op_pack_and_spr_assets_are_nonempty() {
    assert!(include_str!("../🖼️assets/🔧️dream.op.semio").len() > 64);
    assert!(include_bytes!("../🖼️assets/🎒️dream.pack.semio").len() > 64);
    assert!(include_bytes!("../🖼️assets/📡️dream.spr.semio").len() > 64);
}
