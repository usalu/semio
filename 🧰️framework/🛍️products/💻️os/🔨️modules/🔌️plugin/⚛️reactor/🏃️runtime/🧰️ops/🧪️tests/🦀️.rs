//! ⚙️ Unit tests of runtime-ops; the work package moves the tests of its sources here.

#[test]
fn a_lease_claim_posting_without_a_family_receipt_owes_no_receiving_and_with_one_binds_that_family() {
    use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress};
    use crate::ops_ingress::{OriginalPublicationFamily, PluginExchangeOutput};
    let claim = RetainedCloneProgress { copied_items: 1, copied_bytes: 2 * std::mem::size_of::<Option<(u64, (u64, u64))>>(), ..Default::default() };
    let (before, grant) = (RetainedCloneProgress { copied_bytes: 23, ..Default::default() }, RetainedCloneGrant { maximum_items: 8, maximum_copy_bytes: 4096, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: 4 });
    let after = before.checked_add(claim).unwrap();
    let mut direct = PluginExchangeOutput::default();
    direct.bind_window_posting(before, after, Default::default(), grant, 7).unwrap();
    assert!(direct.original_receiving.is_none() && !direct.ownership_receipt_pending());
    let mut served = PluginExchangeOutput::default();
    served.tool_run_receipt = Some((grant, claim));
    served.bind_window_posting(before, after, Default::default(), grant, 7).unwrap();
    let receiving = served.original_receiving.unwrap();
    assert_eq!((receiving.family, receiving.before, receiving.posted, receiving.recipient), (Some(OriginalPublicationFamily::Tool), before, claim, 7));
    let mut idle = PluginExchangeOutput::default();
    idle.tool_run_receipt = Some((grant, claim));
    idle.bind_window_posting(after, after, Default::default(), grant, 7).unwrap();
    assert!(idle.original_receiving.is_none());
    assert!(idle.bind_window_posting(after, before, Default::default(), grant, 7).is_err());
}

#[test]
fn a_worker_receipt_excludes_the_independently_redeemed_return() {
    use crate::ops_ingress::PluginExchangeOutput;
    use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress};
    let before = RetainedCloneProgress { copied_items: 2, copied_bytes: 212, ..Default::default() };
    let producer = RetainedCloneProgress { copied_items: 4, copied_bytes: 55, ..Default::default() };
    let reserved = RetainedCloneProgress { copied_items: 1, copied_bytes: 106, ..Default::default() };
    let paid = before.checked_add(reserved).unwrap();
    let after = paid.checked_add(producer).unwrap();
    let grant = RetainedCloneGrant { maximum_items: 64, maximum_copy_bytes: 32768, ..Default::default() };
    let mut output = PluginExchangeOutput::default();
    output.preparation_receipt = Some((1, grant, producer));
    output.bind_window_posting(before, after, reserved, grant, 7).unwrap();
    let receiving = output.original_receiving.unwrap();
    assert_eq!(receiving.posted, producer);
    assert_eq!(receiving.before.checked_add(receiving.posted).unwrap(), after);
}
