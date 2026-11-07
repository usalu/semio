//! 🧾️ Canonical language-neutral operation wire witness through the independent JSON oracle.
/// 🎟️ Is the physical close authority of original native conformance laws; it equals `physicalCloseGrant` of the os-store emission fixture, which the os-store laws pin.
pub fn physical_test_close_grant() -> crate::RetainedCloneGrant {
    crate::RetainedCloneGrant { maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:64 }
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
