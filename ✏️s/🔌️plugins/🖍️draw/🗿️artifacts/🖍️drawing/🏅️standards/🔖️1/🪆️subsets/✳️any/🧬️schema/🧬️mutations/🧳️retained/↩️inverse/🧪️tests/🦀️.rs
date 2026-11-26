//! 🧪️ The first actual Drawing original field inverse has exact native source, denial and cancellation receipts.
use super::*;
use semio_framework_value_clone_source_testing::{FixtureSource,drive::drive_reply};

const BODY: usize = 32768;
const WORK: u64 = 32768;

fn grant() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: BODY, maximum_capacity_bytes: 1 << 24, maximum_release_bytes: 1 << 24, maximum_depth: 128 }
}

fn advance(
    owner: &mut DrawingOriginalFieldInverse,
    before: RetainedCloneRef<'_, DrawingSnapshot>,
    authored: RetainedCloneRef<'_, DrawingMutation>,
    grant: RetainedCloneGrant,
) -> RetainedCloneStep {
    drive_reply(grant, WORK, |result| result.as_ref().map_or_else(
        semio_framework_value::OriginalRetirementFailure::retained_progress,
        |step: &RetainedCloneStep| step.progress(),
    ), |effect, reply, recover| {
        let before_work = effect.performed_work();
        let mut work = 0;
        let result = owner.advance(before, authored, BODY, WORK, effect, recover, &mut work);
        protocol::retained_conclude(effect, before_work, result, work, |step| step.progress(), reply)
    }).result.unwrap()
}

#[test]
fn original_visible_inverse_receives_original_affected_value_and_closes_every_partial() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../✅️validation/🧪️.json")).unwrap();
    let rules: serde_json::Value = serde_json::from_str(include_str!("../🔣️.json")).unwrap();
    let law = &rules["visibilityLaw"];
    let cuts = law["cancelAfterTurns"].as_array().unwrap().iter().map(|cut| cut.as_u64().unwrap());
    let factory: std::sync::Arc<dyn semio_framework_value::ArtifactOwnedValueRetirementFactory<DrawingMutation>> =
        std::sync::Arc::new(store::os_store::document_store_owners::BoundedArtifactRetirementFactory::new());
    let context = RetainedMutationCloseContext { mutations: &factory };
    let grant = grant();
    for cut in cuts.chain([100000]) {
        let snapshot: DrawingSnapshot = serde_json::from_value(corpus["base"].clone()).unwrap();
        let mutation: DrawingMutation = serde_json::from_value(law["authored"].clone()).unwrap();
        let mut before = FixtureSource::from_owner(snapshot);
        let mut authored = FixtureSource::from_owner(mutation);
        let mut owner = DrawingOriginalFieldInverse::default();

        for _ in 0..cut {
            let demand = owner.demand(before.borrow(), authored.borrow(), BODY, WORK).unwrap();
            let denied = [
                Some(RetainedCloneGrant{maximum_items:0,..grant}),
                (demand.copy_bytes>0).then_some(RetainedCloneGrant{maximum_copy_bytes:0,..grant}),
                (demand.capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:0,..grant}),
                (demand.release_bytes>0).then_some(RetainedCloneGrant{maximum_release_bytes:0,..grant}),
                (demand.depth>0).then_some(RetainedCloneGrant{maximum_depth:0,..grant}),
            ];
            for grant in denied.into_iter().flatten() {
                assert_eq!(advance(&mut owner,before.borrow(),authored.borrow(),grant).progress(),Default::default());
                assert_eq!(owner.demand(before.borrow(),authored.borrow(),BODY,WORK).unwrap(),demand);
            }
            advance(&mut owner,before.borrow(),authored.borrow(),grant);
            if owner.ready(){ break; }
        }
        if cut==100000 { assert!(owner.ready()); }
        if owner.ready() {
            assert_eq!(serde_json::to_value(owner.rows.as_ref().unwrap()).unwrap(), law["inverse"]);
        }
        owner.begin_close();
        for _ in 0..100000 {
            if owner.terminal_is_empty(){ break; }
            drive_reply(grant,WORK,|result| result.as_ref().map_or_else(
                semio_framework_value::OriginalRetirementFailure::retained_progress,
                |step: &RetainedCloneStep| step.progress(),
            ), |effect, reply, recover| {
                let start=effect.performed_work();
                let mut work=0;
                let result=owner.close_step(context,None,BODY,effect,recover,&mut work);
                protocol::retained_conclude(effect,start,result,work,|step| step.progress(),reply)
            }).result.unwrap();
        }
        assert!(owner.terminal_is_empty());
        while !before.terminal_is_empty(){ before.close_once().unwrap(); }
        while !authored.terminal_is_empty(){ authored.close_once().unwrap(); }
    }
}
