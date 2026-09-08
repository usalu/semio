
use super::*;

#[test]
fn an_intent_at_or_ahead_of_the_current_revision_is_never_stale() {
    assert!(!is_stale_intent(ui_contract::UiRevision(5), ui_contract::UiRevision(5), DEFAULT_REVISION_TOLERANCE));
    assert!(!is_stale_intent(ui_contract::UiRevision(9), ui_contract::UiRevision(5), DEFAULT_REVISION_TOLERANCE));
}

#[test]
fn an_intent_exactly_at_the_tolerance_is_not_yet_stale() {
    assert!(!is_stale_intent(ui_contract::UiRevision(4), ui_contract::UiRevision(5), 1));
}

#[test]
fn an_intent_trailing_by_more_than_the_tolerance_is_stale() {
    assert!(is_stale_intent(ui_contract::UiRevision(3), ui_contract::UiRevision(5), 1));
}

#[test]
fn a_zero_tolerance_makes_any_trailing_revision_stale() {
    assert!(is_stale_intent(ui_contract::UiRevision(4), ui_contract::UiRevision(5), 0));
    assert!(!is_stale_intent(ui_contract::UiRevision(5), ui_contract::UiRevision(5), 0));
}
