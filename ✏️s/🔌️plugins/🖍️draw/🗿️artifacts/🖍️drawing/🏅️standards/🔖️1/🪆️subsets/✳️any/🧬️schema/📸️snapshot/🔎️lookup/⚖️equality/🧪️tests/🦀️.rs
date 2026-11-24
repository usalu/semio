//! 🧪️ Original native field equality matches independent serde values with denial and every partial cut.
use semio_framework_value::{paged::PagedUtf8, retained_clone::RetainedCloneGrant};
use semio_s_artifact_draw_drawing_model::snapshot::lookup::equality::DrawingOriginalEquality;
use crate::test_step;
use semio_framework_value_clone_source_testing::FixtureSource;

fn grant() -> RetainedCloneGrant {
    RetainedCloneGrant {
        maximum_items: 1,
        maximum_copy_bytes: 1 << 20,
        maximum_capacity_bytes: 1 << 20,
        maximum_release_bytes: 1 << 20,
        maximum_depth: 128,
    }
}

#[test]
fn original_text_equality_matches_serde_without_payload_copy() {
    let cases = [("雪\0a", "雪\0a"), ("snow", "snow!"), ("", "")];
    for (left, right) in cases {
        for cut in [0, 1, 4, 10_000] {
            let grant = grant();
            let left = FixtureSource::from_owner(PagedUtf8::<{usize::MAX}>::from(left));
            let right = FixtureSource::from_owner(PagedUtf8::<{usize::MAX}>::from(right));
            let expected = serde_json::to_value(left.borrow().get()).unwrap()
                == serde_json::to_value(right.borrow().get()).unwrap();
            let mut comparison = DrawingOriginalEquality::default();
            for _ in 0..cut {
                let demand = comparison.demand(left.borrow(), right.borrow()).unwrap();
                if demand.copy_bytes > 0 {
                    let denied = RetainedCloneGrant {
                        maximum_copy_bytes: demand.copy_bytes - 1,
                        ..grant
                    };
                    let step = test_step(denied, |effect| {
                        comparison.advance(left.borrow(), right.borrow(), effect)
                    });
                    assert_eq!(step.progress(), Default::default());
                    assert_eq!(comparison.demand(left.borrow(), right.borrow()).unwrap(), demand);
                }
                let step = test_step(grant, |effect| {
                    comparison.advance(left.borrow(), right.borrow(), effect)
                });
                assert!(step.progress().fits(grant));
                if let Some(equal) = comparison.result() {
                    assert_eq!(equal, expected);
                    break;
                }
            }
            if cut == 10_000 {
                assert_eq!(comparison.result(), Some(expected));
            }
            comparison.begin_close();
            for _ in 0..10_000 {
                if comparison.terminal_is_empty() {
                    break;
                }
                test_step(grant, |effect| comparison.close_step(grant.maximum_copy_bytes, effect));
            }
            assert!(comparison.terminal_is_empty());
        }
    }
}
