//! 🧪️ History editing reaches every mounted instance through the registered instance extensions.
use super::*;

#[test]
fn registered_extensions_route_every_history_edit_verb() {
    for action in [HISTORY_EDIT_BEGIN_ACTION_ID, HISTORY_EDIT_COMMIT_ACTION_ID, HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID] {
        assert!(TimeTravelExtension::claims_action(action), "{action} is a history-edit verb");
        assert!(<crate::InstanceExtensions as InstanceExtension>::claims_action(action), "{action} is routed by the registered list");
        assert!(<crate::InstanceExtensions as InstanceExtension>::reads_history(action), "{action} needs the document's history");
    }
}

#[test]
fn only_replay_cancellation_is_admitted_while_loading() {
    assert!(<crate::InstanceExtensions as InstanceExtension>::admitted_while_loading(HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID));
    assert!(!<crate::InstanceExtensions as InstanceExtension>::admitted_while_loading(HISTORY_EDIT_BEGIN_ACTION_ID));
}
