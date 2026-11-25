/// 🚨️ A retained output fault arms the reconcile drive until the reactor has reported it to the shell and never again, so one faulted surface cannot hold every turn in more-work.
///
/// 🐛️ The fault stays on the tracker until its instance closes, and `has_drivable_work`, `has_publishable_work` and `has_work` answered `true` for as long as it stayed. FEM 3D measured 9 186 `reconcile` turns with no patch and no effect after a staged `addMaterial` while the new material never reached the tree. Browser measurement 2026-11-25.
#[test]
fn a_reported_output_fault_stops_arming_the_reconcile_drive() {
    let tracker = PatchTracker::new();
    assert!(!tracker.has_drivable_work() && !tracker.has_publishable_work() && !has_work(&tracker.state.borrow()));
    tracker.state.borrow_mut().output_fault = Some((NativeCloseKey::fixture(77, 1), "fixture output fault", false));
    assert!(tracker.has_drivable_work() && tracker.has_publishable_work() && has_work(&tracker.state.borrow()), "an unreported fault still owes its report");
    assert_eq!(tracker.take_render_fault(), Some((77, "fixture output fault".to_owned())));
    assert!(!tracker.has_drivable_work() && !tracker.has_publishable_work() && !has_work(&tracker.state.borrow()), "a reported fault owes the reactor nothing");
    assert_eq!(tracker.take_render_fault(), None, "the fault is reported once");
    assert!(tracker.state.borrow().output_fault.is_some(), "the fault itself stays until its instance closes");
}
