//! 🧾️ Canonical language-neutral operation wire witness through the independent JSON oracle.
/// 🎟️ Reads the independently authored physical close authority used by original native conformance laws.
pub fn physical_test_close_grant() -> crate::RetainedCloneGrant {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/📨️emission/📦️owned/🧪️tests/🧫️fixtures/🔣️.json")).expect("plain physical close policy");
    let policy=&fixture["physicalCloseGrant"];
    crate::RetainedCloneGrant { maximum_items:policy["maximumItems"].as_u64().unwrap() as usize,maximum_copy_bytes:policy["maximumCopyBytes"].as_u64().unwrap() as usize,maximum_capacity_bytes:policy["maximumCapacityBytes"].as_u64().unwrap() as usize,maximum_release_bytes:policy["maximumReleaseBytes"].as_u64().unwrap() as usize,maximum_depth:policy["maximumDepth"].as_u64().unwrap() as usize }
}

/// 🎟️ Drives one staged original source admission to its output, paying every turn its exact quote and reporting the summed receipt and actual work.
pub fn admit_owned_source<T:crate::retirement::RetireOwned+Sync,B:crate::retirement::RetireOwned>(owner:T,authority:B)->(crate::RetainedCloneSource<T>,crate::RetainedCloneProgress,u64){
    let mut admission=crate::RetainedCloneSource::admission_owned(owner,authority);
    let mut output=None;let mut total=crate::RetainedCloneProgress::default();let mut work=0;
    while output.is_none(){
        let demand=admission.next_demand().expect("staged source admission quote");
        let grant=crate::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
        let turn=if admission.phase()=="ready"{admission.take_output(&mut output,grant)}else{admission.advance(grant)};
        work+=turn.work;
        let step=turn.result.expect("staged source admission turn").progress();
        assert!(step.fits(grant));
        total.copied_items+=step.copied_items;total.copied_bytes+=step.copied_bytes;total.retained_capacity_bytes+=step.retained_capacity_bytes;total.released_bytes+=step.released_bytes;
    }
    assert!(admission.terminal_is_empty());
    (output.expect("staged source admission output"),total,work)
}
