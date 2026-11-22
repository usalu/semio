//! 🎟️ Exact typed retirement birth reserves the copy span that moves the source and its frame.
use super::*;
use semio_framework_value_clone_source_testing::drive::drive;

#[test]
fn typed_retirement_birth_moves_source_with_exact_copy_quote() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("fixture.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let mut source = Some(fixture["value"].as_str().unwrap().to_owned());
        let mut owner = None;
        let demand = artifact_retirement_owned_birth_demands(&source).unwrap();
        let withheld_copy = case["withheldCopyBytes"].as_u64().unwrap() as usize;
        let grant = RetainedCloneGrant {
            maximum_items: 1,
            maximum_copy_bytes: demand.copy_bytes.checked_sub(withheld_copy).unwrap(),
            maximum_capacity_bytes: demand.capacity_bytes,
            maximum_release_bytes: demand.release_bytes,
            maximum_depth: demand.depth,
        };
        let turn = drive(grant, 1, |effect| {
            let turn = artifact_retirement_admit_owned(&mut source, &mut owner, effect);
            RetainedWorkPoll::Complete(RetainedWorkOutcome {
                result: turn.result.map_err(Into::into),
                work: turn.work,
            })
        });
        let moved = case["movesSource"].as_bool().unwrap();
        assert_eq!(turn.work, case["work"].as_u64().unwrap() as u128);
        assert_eq!(source.is_none(), moved);
        assert_eq!(owner.is_some(), moved);
        let receipt = turn.result.unwrap().progress();
        assert_eq!(receipt.copied_bytes, usize::from(moved) * demand.copy_bytes);
        assert_eq!(receipt.retained_capacity_bytes, usize::from(moved) * demand.capacity_bytes);
        let grant = RetainedCloneGrant {
            maximum_items: 1,
            maximum_copy_bytes: 4096,
            maximum_capacity_bytes: 4096,
            maximum_release_bytes: 4096,
            maximum_depth: 32,
        };
        for _ in 0..64 {
            if owner.is_none() {
                break;
            }
            drive(grant, 64, |effect| {
                artifact_retirement_box_close_step(&mut owner, grant.maximum_copy_bytes, effect)
            }).result.unwrap();
        }
        assert!(owner.is_none());
    }
}
