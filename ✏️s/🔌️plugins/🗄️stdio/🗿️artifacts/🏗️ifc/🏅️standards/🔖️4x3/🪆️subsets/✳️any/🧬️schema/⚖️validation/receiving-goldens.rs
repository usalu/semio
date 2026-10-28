//! 🧪️ Retained source receiving observes the same fifteen independently authored graph populations.
use super::{overlay::*, receive::*};
use semio_framework_plugin::app_testing::OriginalWorkFixture;
use semio_framework_value::retained_clone::RetainedCloneGrant;

#[test]
fn original_overlay_and_semantic_receiving_match_every_authoring_oracle_graph() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("../🏷️templates/✍️apply/native-cases.json")).unwrap();
    let grant = RetainedCloneGrant { maximum_items: 100_000, maximum_copy_bytes: 65_536, maximum_capacity_bytes: 1_048_576, maximum_release_bytes: 1_048_576, maximum_depth: 256 };
    for case in fixtures["cases"].as_array().unwrap() {
        let source = crate::io::parse_ifc4x3_step(case["source"].as_str().unwrap()).unwrap();
        let expected = crate::io::parse_ifc4x3_step(case["expected"].as_str().unwrap()).unwrap();
        let parsed = semio_framework_pack_json::parse(&case["application"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let mutations = crate::templates::application::work::test_plan(semio_framework_pack_json::to_dsl_value(&parsed), &source, None).unwrap();
        let mut caller = OriginalWorkFixture::with_identity(semio_framework_job::OperationId(923), semio_framework_job::Generation(1));
        let mut overlay = PopulationOverlay::default();
        for turn in 0..100_000 {
            if caller.work_turn(grant, 1, |cx| overlay.advance(&source, &mutations, cx)).unwrap() {
                break;
            }
            assert!(turn < 99_999);
        }
        assert_eq!(overlay.len(), expected.entities.len());
        for entity in &expected.entities {
            let row = overlay.find(entity.id).unwrap();
            assert_eq!(overlay.entity(&source, &mutations, row).unwrap().name, entity.name);
            for (index, value) in entity.args.iter().enumerate() {
                assert_eq!(overlay.argument(&source, &mutations, row, index).unwrap(), value);
            }
        }
        let mut receiver = PopulationReceiver::default();
        for turn in 0..100_000 {
            if caller.work_turn(grant, 1, |cx| receiver.advance(&overlay, &source, &mutations, cx)).unwrap() {
                break;
            }
            assert!(turn < 99_999);
        }
        assert_eq!(receiver.arena().unwrap().entities.len(), expected.entities.len());
        receiver.begin_close();
        overlay.begin_close();
        for turn in 0..100_000 {
            if receiver.terminal_is_empty() && overlay.terminal_is_empty() {
                break;
            }
            caller
                .turn(grant, |cx| {
                    cx.with_original_retirement(|cx| {
                        if !receiver.terminal_is_empty() {
                            semio_framework_job::close_original_retained_step(cx, receiver.close_demands(4096)?, |loan| receiver.close_step(loan))
                        } else {
                            semio_framework_job::close_original_retained_step(cx, overlay.close_demands(4096)?, |loan| overlay.close_step(loan))
                        }
                    })
                })
                .unwrap();
            assert!(turn < 99_999);
        }
        assert!(receiver.terminal_is_empty() && overlay.terminal_is_empty());
        caller.finish();
    }
}

#[test]
fn underfunded_original_overlay_birth_neither_allocates_nor_changes_coordinates() {
    let source = IfcSnapshot { entities: vec![IfcEntity { id: 1, name: "IFCCARTESIANPOINT".into(), args: vec![], complex: vec![] }], ..Default::default() };
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 65_536, maximum_capacity_bytes: 65_536, maximum_release_bytes: 65_536, maximum_depth: 256 };
    for denied in [RetainedCloneGrant { maximum_items: 0, ..grant }, RetainedCloneGrant { maximum_copy_bytes: 0, ..grant }, RetainedCloneGrant { maximum_capacity_bytes: 0, ..grant }, RetainedCloneGrant { maximum_depth: 0, ..grant }] {
        let mut caller = OriginalWorkFixture::with_identity(semio_framework_job::OperationId(924), semio_framework_job::Generation(1));
        let mut overlay = PopulationOverlay::default();
        let demand = overlay.demands(&source, &[]).unwrap();
        caller
            .work_turn(denied, 1, |cx| {
                let before = cx.retained_progress();
                let (answer, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| overlay.advance(&source, &[], cx));
                assert!(!answer?);
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                assert_eq!(before, cx.retained_progress());
                Ok(())
            })
            .unwrap();
        assert_eq!(demand, overlay.demands(&source, &[]).unwrap());
        assert_eq!(overlay.len(), 0);
        assert!(overlay.terminal_is_empty());
        caller.finish();
    }
}
