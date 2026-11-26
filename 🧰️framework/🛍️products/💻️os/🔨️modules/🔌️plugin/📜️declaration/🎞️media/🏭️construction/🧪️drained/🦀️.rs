use semio_framework_job::{Generation, OperationId, OriginalJobFixtureCaller, close_original_effect_step};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneStep, RetainedWorkOutcome, RetainedWorkPoll};
use semio_plugin_decl_media::ArtifactOutputChunks;

#[test]
fn original_drained_output_returns_the_exact_admitted_page_table() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let stock = RetainedCloneGrant { maximum_items: 1 << 20, maximum_copy_bytes: 1 << 24, maximum_capacity_bytes: 1 << 24, maximum_release_bytes: 1 << 24, maximum_depth: 64 };
    for row in law["cases"].as_array().unwrap().iter().filter(|row| row["slots"].as_u64().is_some_and(|slots| slots <= 64)) {
        let maximum = row["maximum"].as_u64().unwrap() as usize;
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(44852), Generation(1), stock, || Some(1), u64::MAX, 65_536);
        caller.audit_heap();
        let mut owner = None;
        assert!(caller.turn(stock, |cx| ArtifactOutputChunks::original_birth_admit(maximum, &mut owner, cx)).unwrap());
        let capacity = ArtifactOutputChunks::original_birth_demands(maximum).unwrap().capacity_bytes;
        assert!(capacity <= caller.receipt().retained_capacity_bytes);
        let alias = owner.clone();
        assert!(owner.as_ref().unwrap().drained_close_demands().is_none());
        drop(alias);
        let (demand, work) = owner.as_ref().unwrap().drained_close_demands().unwrap();
        assert_eq!(demand.release_bytes, capacity);
        assert_eq!(work, 2 + row["slots"].as_u64().unwrap());
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth, ..Default::default() };
        let denied = RetainedCloneGrant { maximum_release_bytes: grant.maximum_release_bytes - 1, ..grant };
        for original_grant in [denied, grant] {
            let step = caller.retirement_turn(original_grant, |cx| close_original_effect_step(cx, demand, |effect| {
                let result = ArtifactOutputChunks::close_drained_original(&mut owner, effect).map(RetainedCloneStep::Progress);
                RetainedWorkPoll::Complete(RetainedWorkOutcome { work: effect.performed_work(), result })
            })).unwrap();
            if original_grant == denied {
                assert_eq!(step.progress(), Default::default());
                assert!(owner.is_some());
            } else {
                assert!(owner.is_none());
                assert_eq!(step.progress().copied_items, 1);
                assert_eq!(step.progress().retained_capacity_bytes, 0);
                assert_eq!(step.progress().released_bytes, capacity);
            }
        }
        caller.retire(stock);
    }
}
