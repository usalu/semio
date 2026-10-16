//! ️tests for example `🌙️capsule-dream`.

#[test]
fn dsl_asset_parses_and_round_trips() {
    let text = super::capsule_dream::dsl_text();
    assert!(text.len() > 64, "dsl fixture must carry real payload");
    let projection = crate::standards::v1::subsets::any::io::text::snapshot::parse_dsl(text).expect("example dsl parses");
    assert_eq!(projection.parts.len(), 2880);
    assert_eq!(projection.fasteners.len(), 2864);
    semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&projection);
}

#[test]
fn flatten_matches_golden_poses_to_1e4() {
    let text = super::capsule_dream::dsl_text();
    let mut projection = crate::standards::v1::subsets::any::io::text::snapshot::parse_dsl(text).expect("example dsl parses");
    crate::standards::v1::subsets::any::schema::inferences::flat_position::flatten_snapshot_inplace(&mut projection);
    let golden: serde_json::Map<String, serde_json::Value> = serde_json::from_slice(&std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌙️capsule-dream/🖼️assets/🔣️.json")).expect("the committed golden poses are readable")).expect("golden json");
    assert_eq!(golden.len(), 2880);
    let mut center_mismatches = 0usize;
    let mut origin_mismatches = 0usize;
    for part in &projection.parts {
        let expected = golden.get(&part.id).expect("golden");
        let origin = expected.get("origin").and_then(|v| v.as_array()).unwrap();
        let center = expected.get("center").unwrap();
        let ex = [origin[0].as_f64().unwrap(), origin[1].as_f64().unwrap(), origin[2].as_f64().unwrap()];
        let cx = center.get("x").and_then(|v| v.as_f64()).unwrap();
        let cy = center.get("y").and_then(|v| v.as_f64()).unwrap();
        if (part.part_2d.x - cx).abs() > 1e-4 || (part.part_2d.y - cy).abs() > 1e-4 {
            center_mismatches += 1;
        }
        let err = (part.part_3d.origin[0] - ex[0]).abs().max((part.part_3d.origin[1] - ex[1]).abs()).max((part.part_3d.origin[2] - ex[2]).abs());
        if err > 1e-4 {
            origin_mismatches += 1;
        }
    }
    assert_eq!(center_mismatches, 0, "{center_mismatches} center mismatches");
    assert_eq!(origin_mismatches, 0, "{origin_mismatches} origin mismatches");
}

#[test]
fn op_pack_and_spr_assets_are_nonempty() {
    assert!(include_str!("../../🖼️assets/🔧️dream.op.semio").len() > 64);
    assert!(include_bytes!("../../🖼️assets/🎒️.pack.semio").len() > 64);
    assert!(include_bytes!("../../🖼️assets/📡️dream.spr.semio").len() > 64);
}

#[test]
fn inference_default_law() {
    use crate::standards::v1::subsets::any::schema::inferences::Puzzle5dInference;
    use protocol::Inference;
    assert_eq!(Puzzle5dInference::infer(&crate::Puzzle5dSnapshot::default()).expect("valid materialized inference fixture"), Puzzle5dInference::default());
}

#[test]
fn inference_determinism_law() {
    use crate::standards::v1::subsets::any::schema::inferences::Puzzle5dInference;
    use protocol::Inference;
    let text = super::capsule_dream::dsl_text();
    let projection = crate::standards::v1::subsets::any::io::text::snapshot::parse_dsl(text).expect("example dsl parses");
    assert_eq!(Puzzle5dInference::infer(&projection).expect("valid materialized inference fixture"), Puzzle5dInference::infer(&projection).expect("valid materialized inference fixture"));
}

/// 🔏️ The digest sidecar declares the DSL the host would deliver: a repository-relative name, the exact length of the bytes and a
/// well-formed SHA-256 (recomputed independently by the `📦️package-asset-constants` law); the delivery declaration repeats it.
#[test]
fn the_digest_sidecar_declares_the_dsl_it_stands_for() {
    let (name, size_bytes, sha256) = super::capsule_dream::dsl_bytes_digest();
    assert!(!name.starts_with('/') && !name.contains(".."), "{name}");
    assert_eq!(super::capsule_dream::dsl_bytes().len() as u64, size_bytes);
    assert_eq!(sha256.len(), 64);
    assert!(sha256.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)));
    let declaration = super::capsule_dream::dsl_asset_declaration();
    assert_eq!((declaration.name.as_str(), declaration.size_bytes, declaration.sha256.as_str()), (name, size_bytes, sha256));
}
