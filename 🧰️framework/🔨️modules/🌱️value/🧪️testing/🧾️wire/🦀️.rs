//! 🧾️ Canonical language-neutral operation wire witness through the independent JSON oracle.
/// 🎟️ Is the physical close authority of original native conformance laws; it equals `physicalCloseGrant` of the os-store emission fixture, which the os-store laws pin.
pub fn physical_test_close_grant() -> crate::RetainedCloneGrant {
    crate::RetainedCloneGrant { maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:64 }
}

/// 🎟️ Drives one staged original source admission to its output through an original effect bank, paying every turn its exact quote and reporting the summed receipt and actual work.
pub fn admit_owned_source<T:crate::retirement::RetireOwned+Sync,B:crate::retirement::RetireOwned>(owner:T,authority:B)->(crate::RetainedCloneSource<T>,crate::RetainedCloneProgress,u64){
    use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
    let mut admission=crate::RetainedCloneSource::admission_owned(owner,authority);
    let mut bank=EffectBank::new(Wallet::from_profile(&serde_json::json!({"items":1u64<<20,"copyBytes":1u64<<30,"capacityBytes":1u64<<30,"releaseBytes":1u64<<30,"depth":64u64,"fuel":1u64<<40})));
    let mut output=None;let mut total=crate::RetainedCloneProgress::default();let mut work=0u128;
    while output.is_none(){
        let demand=admission.next_demand().expect("staged source admission quote");
        let grant=crate::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
        let turn=if admission.phase()=="ready"{admission.take_output(&mut output,grant)}else{
            let maximum=admission.advance_work_demand().expect("staged source admission work quote");
            let poll=bank.run(grant,maximum,|effect|crate::RetainedWorkPoll::Complete(admission.advance(effect))).0;
            assert_ne!(bank.take_admitted(),Some(false),"staged source admission turn was refused by its original Work verdict");
            match poll{crate::RetainedWorkPoll::Complete(turn)=>turn,crate::RetainedWorkPoll::Pending{..}=>panic!("staged source admission turn stayed pending")}
        };
        work+=turn.work;
        let step=turn.result.expect("staged source admission turn").progress();
        assert!(step.fits(grant));
        total.copied_items+=step.copied_items;total.copied_bytes+=step.copied_bytes;total.retained_capacity_bytes+=step.retained_capacity_bytes;total.released_bytes+=step.released_bytes;
    }
    assert!(admission.terminal_is_empty());
    (output.expect("staged source admission output"),total,u64::try_from(work).expect("staged source admission work fits u64"))
}
