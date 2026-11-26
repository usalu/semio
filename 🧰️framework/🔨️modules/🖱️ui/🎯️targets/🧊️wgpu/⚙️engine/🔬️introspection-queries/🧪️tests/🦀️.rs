use super::*;
use semio_framework_job::{Generation, OperationId, OriginalJobFixtureCaller};
use semio_framework_value::RetainedCloneGrant;

fn clock() -> Option<u64> { Some(1000) }

#[test]
fn presented_tree_alias_returns_under_a_new_work_slice_deadline() {
    let stock = RetainedCloneGrant { maximum_items: 10000, maximum_copy_bytes: 1_000_000, maximum_capacity_bytes: 1_000_000, maximum_release_bytes: 1_000_000, maximum_depth: 64 };
    let mut ui = Ui::new(semio_ui_locale::Locale::En);
    let surface = ui.try_admit_surface("accepted-window").unwrap();
    let mut tree = Some(UiTree::new());
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(97146), Generation(17), stock, clock, 3_600_000_000, 2_000_000);
    caller.turn(stock, |cx| {
        assert!(ui.donate_original_presented_fixture_tree(surface, &mut tree, cx)?);
        let mut admitted = None;
        cx.with_original_work_loan(cx.fuel_remaining(), 2000, |same| { admitted = ui.admit_presented_tree_source(surface, same)?; Ok(()) })?;
        let mut loan = admitted.unwrap();
        let returned = cx.with_original_work_loan(cx.fuel_remaining(), 3000, |same| ui.return_presented_tree_source(&mut loan, same));
        if returned.is_err() { assert!(cx.with_original_work_loan(cx.fuel_remaining(), 2000, |same| ui.return_presented_tree_source(&mut loan, same))?); }
        assert!(returned?);
        assert!(ui.take_original_presented_fixture_tree(surface, &mut tree, cx)?);
        Ok(())
    }).unwrap();
    caller.retire(stock);
    drop(tree);
}

#[test]
fn presented_tree_alias_rejects_a_different_original_caller() {
    let stock = RetainedCloneGrant { maximum_items: 10000, maximum_copy_bytes: 1_000_000, maximum_capacity_bytes: 1_000_000, maximum_release_bytes: 1_000_000, maximum_depth: 64 };
    let mut ui = Ui::new(semio_ui_locale::Locale::En);
    let surface = ui.try_admit_surface("accepted-window").unwrap();
    let mut tree = Some(UiTree::new());
    let mut owner = OriginalJobFixtureCaller::admit(OperationId(97146), Generation(17), stock, clock, 3_600_000_000, 2_000_000);
    let mut other = OriginalJobFixtureCaller::admit(OperationId(97147), Generation(17), stock, clock, 3_600_000_000, 2_000_000);
    owner.turn(stock, |cx| {
        assert!(ui.donate_original_presented_fixture_tree(surface, &mut tree, cx)?);
        let mut loan = ui.admit_presented_tree_source(surface, cx)?.unwrap();
        other.turn(stock, |foreign| { assert!(ui.return_presented_tree_source(&mut loan, foreign).is_err()); Ok(()) }).unwrap();
        assert!(ui.return_presented_tree_source(&mut loan, cx)?);
        assert!(ui.take_original_presented_fixture_tree(surface, &mut tree, cx)?);
        Ok(())
    }).unwrap();
    owner.retire(stock);
    other.retire(stock);
    drop(tree);
}
