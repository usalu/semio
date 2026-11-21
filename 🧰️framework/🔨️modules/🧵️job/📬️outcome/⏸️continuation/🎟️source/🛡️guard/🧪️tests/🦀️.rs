//! 🧪️ A full Source escrow can mint a header guard using its paid acknowledged return.

use super::*;
use semio_framework_job_context::{Generation, OperationId, RetainedCloneGrant};
use semio_framework_job_testing_caller::OriginalJobFixtureCaller;

#[test]
fn full_escrow_reclaims_one_acknowledged_row_before_minting_its_guard() {
    let law: serde_json::Value = serde_json::from_str(include_str!("🧫️.json")).unwrap();
    let cells = law["cells"].as_u64().unwrap();
    let header = law["headerBytes"].as_u64().unwrap() as usize;
    let stock = RetainedCloneGrant {
        maximum_items: 4096,
        maximum_copy_bytes: 1 << 20,
        maximum_capacity_bytes: 1 << 20,
        maximum_release_bytes: 1 << 20,
        maximum_depth: 8,
    };
    for (index, row) in law["cases"].as_array().unwrap().iter().enumerate() {
        let mut caller = OriginalJobFixtureCaller::admit(
            OperationId(8831 + index as u64), Generation(1), stock, || Some(0), 100_000, 100_000,
        );
        let mut ready = false;
        for _ in 0..64 {
            caller.turn(stock, |cx| {
                cx.bind_original_source_epoch(1)?;
                ready = cx.admit_original_core_wire_delivery()? && cx.admit_original_escrow_publisher()?;
                Ok(())
            }).unwrap();
            if ready { break; }
        }
        assert!(ready);
        let mut permit = None;
        caller.refreshed_turn(stock, |cx| {
            cx.bind_original_source_epoch(2)?;
            let (source, source_birth, recipient) = source(cx)?;
            let budget = cx.original_return_budget_mut().unwrap().get_mut();
            let table = budget.table_mut().ok_or_else(bad)?;
            for serial in 1..=cells {
                let identity = OriginalReturnEscrowIdentity {
                    source: source as u64, source_birth, recipient: recipient as u64,
                    destination: serial, extent: 0, serial, origin_epoch: 1,
                };
                assert!(table.reserve(identity, Default::default())?.is_some());
                table.redeem(identity, Default::default(), Default::default())?;
                if serial <= row["acknowledged"].as_u64().unwrap() {
                    table.mark_local_received(identity)?;
                }
            }
            assert_eq!(table.free_rows(), 0);
            budget.serial = cells;
            let items = cx.retained_progress().copied_items;
            let work = cx.original_work_units_spent();
            let (result, allocations) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| {
                prepare_original_header_guard(cx, &mut permit, header)
            });
            assert_eq!(Some(result?), row["admitted"].as_bool());
            assert_eq!((allocations.requested_bytes, allocations.released_bytes), (0, 0));
            assert_eq!(cx.retained_progress().copied_items - items, row["chargedItems"].as_u64().unwrap() as usize);
            assert_eq!(cx.original_work_units_spent() - work, u128::from(row["chargedWork"].as_u64().unwrap()));
            Ok(())
        }).unwrap();
        for _ in 0..16 {
            if permit.is_none() { break; }
            caller.refreshed_turn(stock, |cx| {
                cx.bind_original_source_epoch(3)?;
                release_original_header_guard(cx, &mut permit)?;
                Ok(())
            }).unwrap();
        }
        assert!(permit.is_none());
        caller.refreshed_turn(stock, |cx| {
            cx.original_return_budget_mut().unwrap().get_mut().table_mut().ok_or_else(bad)?.acknowledge_settled();
            Ok(())
        }).unwrap();
        caller.retire(stock);
    }
}
