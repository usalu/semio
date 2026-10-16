//! 🧪️ Tool runs reach every mounted instance through the registered instance extensions.
use super::*;

#[test]
fn registered_extensions_route_every_tool_run_verb() {
    for action in TOOL_RUN_ACTION_IDS {
        assert!(ToolRunExtension::claims_action(action), "{action} is a tool run verb");
        assert!(<crate::InstanceExtensions as InstanceExtension>::claims_action(action), "{action} is routed by the registered list");
        assert!(!<crate::InstanceExtensions as InstanceExtension>::reads_history(action), "{action} never needs the document's history");
    }
}
