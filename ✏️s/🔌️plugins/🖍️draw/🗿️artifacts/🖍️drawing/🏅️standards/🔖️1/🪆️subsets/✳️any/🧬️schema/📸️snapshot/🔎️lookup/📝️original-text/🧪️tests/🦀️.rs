//! 🧪️ Original sparse diff text receives exact UTF8 bytes and keeps partial output through denial and cancellation.
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value_clone_source_testing::FixtureSource;
use crate::test_step;
use semio_s_artifact_draw_drawing_model::snapshot::lookup::original_text::{DrawingNativeText, DrawingOriginalText};

fn grant() -> RetainedCloneGrant {
    RetainedCloneGrant {
        maximum_items: 1,
        maximum_copy_bytes: 4_096,
        maximum_capacity_bytes: 65_536,
        maximum_release_bytes: 65_536,
        maximum_depth: 64,
    }
}

#[test]
fn original_paged_text_matches_independent_serde_string_without_payload_proxy() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../🔣️.json")).unwrap();
    for case in law["cases"].as_array().unwrap() {
        for cut in [0, 1, 3, 10_000] {
            let grant = grant();
            let body = grant.maximum_copy_bytes;
            let raw: DrawingNativeText = case.as_str().unwrap().into();
            let source = FixtureSource::from_owner(raw);
            let mut owner = DrawingOriginalText::default();
            for _ in 0..cut {
                let demand = owner.demand(source.borrow(), body).unwrap();
                for axis in 0..5 {
                    let mut denied = grant;
                    match axis {
                        0 => denied.maximum_items = 0,
                        1 if demand.copy_bytes > 0 => denied.maximum_copy_bytes = 0,
                        2 if demand.capacity_bytes > 0 => denied.maximum_capacity_bytes = 0,
                        3 if demand.release_bytes > 0 => denied.maximum_release_bytes = 0,
                        4 if demand.depth > 0 => denied.maximum_depth = 0,
                        _ => continue,
                    }
                    let step = test_step(denied, |effect| owner.advance(source.borrow(), body, effect));
                    assert_eq!(step.progress(), Default::default());
                    assert_eq!(owner.demand(source.borrow(), body).unwrap(), demand);
                }
                let step = test_step(grant, |effect| owner.advance(source.borrow(), body, effect));
                assert!(step.progress().fits(grant));
                if owner.ready() {
                    break;
                }
            }
            if cut == 10_000 {
                assert!(owner.ready());
            }
            if owner.ready() {
                let (output, progress) = owner.take(grant).unwrap();
                assert!(progress.fits(grant));
                let output = FixtureSource::from_owner(output.unwrap());
                assert_eq!(serde_json::to_value(output.borrow().get()).unwrap(), *case);
            }
            owner.begin_close();
            for _ in 0..10_000 {
                if owner.terminal_is_empty() {
                    break;
                }
                test_step(grant, |effect| owner.close_step(body, effect));
            }
            assert!(owner.terminal_is_empty());
        }
    }
}
