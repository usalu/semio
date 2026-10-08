use super::*;

fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap() }
fn cases() -> serde_json::Value { serde_json::from_str(include_str!("../../../🧫️fixtures/🏠️local-interaction/🔣️.json")).unwrap() }

#[test]
fn local_interaction_root_composes_real_typed_payload_and_physical_retirement() {
    use crate::value::{retirement::controlled::ControlledRetirement,retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};
    let fixture=fixture();let cases=cases();
    for name in ["sourceCase","largeSourceCase"]{let case=cases["cases"].as_array().unwrap().iter().find(|case|case["id"]==fixture[name]).unwrap();let source=&case[if name=="sourceCase"{"before"}else{"expected"}];
        for maximum_copy_bytes in [1,64,4096]{
            let(root,birth,free)=crate::test_allocation::observe_backing(||LocalInteractionRoot::from_cold(from_json(source.clone())));let original=birth-free;
            assert_eq!(dsl_to_json(&crate::value::ToValue::to_value(&root)),*source);
            let(mut owner,birth,free)=crate::test_allocation::observe_backing(||ControlledRetirement::new(root).unwrap_or_else(|_|panic!("local interaction root requires defining typed retirement")));assert_eq!((birth,free),(0,0));let mut total=RetainedCloneProgress::default();let mut turns=0;
            while !owner.terminal_is_empty(){turns+=1;assert!(turns<500000);let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes,maximum_capacity_bytes:owner.next_capacity_byte_demand(maximum_copy_bytes).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
                let(step,birth,free)=crate::test_allocation::observe_backing(||owner.step(RetainedCloneGrant {maximum_items:0,..grant}).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!((birth,free),(0,0));
                for denied in [RetainedCloneGrant {maximum_capacity_bytes:grant.maximum_capacity_bytes.saturating_sub(1),..grant},RetainedCloneGrant {maximum_release_bytes:grant.maximum_release_bytes.saturating_sub(1),..grant}]{if denied.maximum_capacity_bytes==grant.maximum_capacity_bytes&&denied.maximum_release_bytes==grant.maximum_release_bytes{continue;}let(step,birth,free)=crate::test_allocation::observe_backing(||owner.step(denied).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!((birth,free),(0,0));}
                let(step,birth,free)=crate::test_allocation::observe_backing(||owner.step(grant).unwrap());let progress=step.progress();assert!(progress.fits(grant));assert_eq!(birth,progress.retained_capacity_bytes);assert_eq!(free,progress.released_bytes);total=total.checked_add(progress).unwrap();
            }
            assert_eq!(total.released_bytes,original+total.retained_capacity_bytes);eprintln!("[DEBUG] actual local interaction typed {} copy{} turns{} original{} born{} physically released{}",name,maximum_copy_bytes,turns,original,total.retained_capacity_bytes,total.released_bytes);
        }
    }
}

pub(super) fn drain(owner:Result<crate::value::retirement::controlled::ControlledRetirement<LocalInteractionRoot>,(crate::value::ValueError,LocalInteractionRoot)>,bytes:usize)->usize {
    use crate::value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress};
    let mut owner=owner.unwrap_or_else(|_|panic!("defining Root retirement must be supported"));
    let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:bytes,maximum_capacity_bytes:owner.next_capacity_byte_demand(bytes).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
    assert_eq!(owner.step(RetainedCloneGrant {maximum_items:0,..grant}).unwrap().progress(),RetainedCloneProgress::default());
    assert_eq!(owner.step(RetainedCloneGrant {maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,..grant}).unwrap().progress(),RetainedCloneProgress::default());
    let mut retired=0;
    for _ in 0..200000 {
        if owner.terminal_is_empty(){return retired;}
        let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:bytes,maximum_capacity_bytes:owner.next_capacity_byte_demand(bytes).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
        let(step,birth,free)=crate::test_allocation::observe_backing(||owner.step(grant).unwrap());let progress=step.progress();
        assert!(progress.fits(grant));assert_eq!(birth,progress.retained_capacity_bytes);assert_eq!(free,progress.released_bytes);assert!(progress.copied_items!=0);
        retired+=progress.copied_bytes;
    }
    panic!("retained root failed to close")
}

#[test]
fn local_interaction_retained_root_preserves_exact_wire_and_shared_payloads() {
    let fixture = fixture();
    let cases = cases();
    let source = &cases["cases"].as_array().unwrap().iter().find(|case| case["id"] == fixture["sourceCase"]).unwrap()["before"];
    for bytes in [1, 64, 4096] {
        let root = LocalInteractionRoot::from_cold(from_json(source.clone()));
        let shared = root.clone();
        assert!(std::ptr::eq(root.selection().first_key_value().unwrap().1, shared.selection().first_key_value().unwrap().1));
        assert_eq!(dsl_to_json(&crate::value::ToValue::to_value(&root)), *source);
        assert_eq!(drain(root.retire(), bytes), fixture["sharedOwnerRetiredBytes"].as_u64().unwrap() as usize);
        assert_eq!(dsl_to_json(&crate::value::ToValue::to_value(&shared)), *source);
        assert_eq!(drain(shared.retire(), bytes), fixture["finalOwnerRetiredBytes"].as_u64().unwrap() as usize);
    }
}

#[test]
fn local_interaction_retained_root_large_semantic_content_closes_under_actual_grants() {
    let fixture = fixture();
    let cases = cases();
    let source = &cases["cases"].as_array().unwrap().iter().find(|case| case["id"] == fixture["largeSourceCase"]).unwrap()["expected"];
    for bytes in [1, 64, 4096] {
        let root = LocalInteractionRoot::from_cold(from_json(source.clone()));
        assert_eq!(dsl_to_json(&crate::value::ToValue::to_value(&root)), *source);
        assert_eq!(drain(root.retire(), bytes), fixture["largeFinalOwnerRetiredBytes"].as_u64().unwrap() as usize);
    }
}
