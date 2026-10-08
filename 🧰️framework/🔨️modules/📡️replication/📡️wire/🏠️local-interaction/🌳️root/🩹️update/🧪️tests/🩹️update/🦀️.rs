use super::*;
use crate::value::{ordered::SharedOwner,retained_clone::{RetainedCloneGrant,RetainedCloneStep}};

fn fixture()->serde_json::Value{serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap()}
fn cases()->serde_json::Value{serde_json::from_str(include_str!("../../../../🧫️fixtures/🏠️local-interaction/🔣️.json")).unwrap()}
fn grant(cursor:&LocalInteractionRootUpdate,bytes:usize)->RetainedCloneGrant{RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:bytes,maximum_capacity_bytes:cursor.next_capacity_byte_demand(bytes).unwrap(),maximum_release_bytes:cursor.next_release_byte_demand().unwrap(),maximum_depth:cursor.next_depth_demand().unwrap()}}
fn account(step:RetainedCloneStep,grant:RetainedCloneGrant,work:LocalInteractionWork)->usize{let progress=step.progress();assert!(progress.fits(grant));assert!(progress.copied_items<=1);assert!(progress.copied_bytes<=grant.maximum_copy_bytes);if work==LocalInteractionWork::Retirement{progress.copied_bytes}else{0}}
fn advance(cursor:&mut LocalInteractionRootUpdate,bytes:usize)->(RetainedCloneStep,usize){
    let grant=grant(cursor,bytes);let work=cursor.work_kind();let(step,birth,free)=crate::test_allocation::observe_backing(||cursor.advance(grant).unwrap());let progress=step.progress();assert_eq!(birth,progress.retained_capacity_bytes);assert_eq!(free,progress.released_bytes);let retired=account(step,grant,work);(step,retired)
}
fn close(mut cursor:LocalInteractionRootUpdate,bytes:usize)->usize{
    cursor.begin_close().unwrap();let mut released=0;
    for _ in 0..500000{if cursor.terminal_is_empty(){return released;}let grant=grant(&cursor,bytes);let(step,birth,free)=crate::test_allocation::observe_backing(||cursor.close_step(grant).unwrap());let progress=step.progress();assert_eq!(birth,progress.retained_capacity_bytes);assert_eq!(free,progress.released_bytes);released+=account(step,grant,LocalInteractionWork::Retirement);}
    panic!("update owner failed to reach exact terminal")
}
fn retire(cursor:Result<crate::value::retirement::controlled::ControlledRetirement<LocalInteractionRoot>,(crate::value::ValueError,LocalInteractionRoot)>,bytes:usize)->usize{super::retained_root_tests::drain(cursor,bytes)}

#[test]
fn local_interaction_retained_update_three_fields_are_atomic_and_exact() {
    let fixture = fixture(); let cases = cases();
    for name in fixture["cases"].as_array().unwrap() {
        let case = cases["cases"].as_array().unwrap().iter().find(|row| &row["id"] == name).unwrap();
        for bytes in [1, 64, 4096] {
            let root = LocalInteractionRoot::from_cold(from_json(case["before"].clone()));
            let patch = LocalInteractionRootPatch::from_cold(from_json(case["restore"]["domains"]["graph"].clone()));
            let pointer = patch.selection().map(SharedOwner::as_ptr);
            let mut update = root.begin_domain_patch(SharedOwner::from_cold("graph".into()), patch);
            let admitted=grant(&update,bytes);assert_eq!(update.advance(RetainedCloneGrant {maximum_items:0,..admitted}).unwrap().progress(),crate::value::retained_clone::RetainedCloneProgress::default());
            let(error,birth,free)=crate::test_allocation::observe_backing(||update.advance(RetainedCloneGrant {maximum_items:1,..Default::default()}).unwrap_err());assert_eq!(error.kind,crate::value::ValueRefusalKind::DepthLimit);assert_eq!((birth,free),(0,0));
            for _ in 0..10_000 {
                if update.is_complete() { break; }
                assert!(update.take().is_none());
                advance(&mut update,bytes);
                assert_eq!(dsl_to_json(&crate::value::ToValue::to_value(&root)), case["before"]);
            }
            assert!(update.is_complete());
            let result = update.take().unwrap();
            assert_eq!(dsl_to_json(&crate::value::ToValue::to_value(&result)), case["expected"]);
            if let Some(pointer) = pointer { assert_eq!(result.selection().get("graph").unwrap() as *const _, pointer); }
            close(update, bytes); retire(root.retire(), bytes); retire(result.retire(), bytes);
        }
    }
}

#[test]
fn local_interaction_retained_update_every_cancel_frontier_retires_exact_owners() {
    let fixture = fixture(); let cases = cases();
    let case = cases["cases"].as_array().unwrap().iter().find(|row| row["id"] == fixture["cancelSourceCase"]).unwrap();
    for bytes in [1, 64, 4096] {
        let mut last_cut = false;
        for cut in 0..10_000 {
            let root = LocalInteractionRoot::from_cold(from_json(case["before"].clone()));
            let patch = LocalInteractionRootPatch::from_cold(from_json(case["restore"]["domains"]["graph"].clone()));
            let mut update = root.begin_domain_patch(SharedOwner::from_cold("graph".into()), patch);
            let mut released = retire(root.retire(), bytes);
            for _ in 0..cut { released += advance(&mut update,bytes).1; }
            if update.is_complete() { last_cut = true; }
            update.begin_close().unwrap(); assert!(update.take().is_none());
            released += close(update, bytes);
            assert_eq!(released as u64, fixture["cancelOwnedStringBytes"].as_u64().unwrap(), "cut {cut}, grant {bytes}");
            if last_cut { break; }
        }
        assert!(last_cut);
    }
}

#[test]
fn local_interaction_retained_update_long_keys_are_compared_under_each_byte_grant() {
    let fixture = fixture(); let cases = cases();
    let state: LocalInteractionState = from_json(cases["cases"].as_array().unwrap().iter().find(|row| row["id"] == fixture["largeSourceCase"]).unwrap()["expected"].clone());
    for bytes in [1, 64, 4096] {
        let domain = SharedOwner::from_cold(state.selection.first_key_value().unwrap().0.clone());
        let key_bytes = domain.len(); assert!(key_bytes > 4096);
        let root = LocalInteractionRoot::from_cold(state.clone());
        let patch = LocalInteractionRootPatch::from_cold(LocalInteractionDomainPatch { selection: None, active_mode: None, active_granularity: None });
        let mut update = root.begin_domain_patch(domain, patch);
        let mut compared = 0;
        for _ in 0..500_000 {
            if update.is_complete() { break; }
            let work=update.work_kind();let(step,_)=advance(&mut update,bytes);
            if work==LocalInteractionWork::Comparison{compared+=step.progress().copied_bytes;}
        }
        assert!(update.is_complete()); assert!(compared >= 2 * key_bytes);
        let result = update.take().unwrap(); assert!(result.selection().is_empty());
        close(update, bytes); retire(root.retire(), bytes); retire(result.retire(), bytes);
    }
}
