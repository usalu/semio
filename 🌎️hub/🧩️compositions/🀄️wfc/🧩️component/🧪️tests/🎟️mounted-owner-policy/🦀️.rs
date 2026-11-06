//! 🎛️ The actual guest runtime selects the portable WFC lifecycle grants.

const FIXTURE: &str = include_str!("../../../🧪️tests/🧫️fixtures/🎟️mounted-owner-policy/🔣️.json");

#[test]
fn guest_runtime_uses_the_portable_wfc_policy() {
    let fixture: serde_json::Value = serde_json::from_str(FIXTURE).expect("portable mounted policy");
    let selected = super::__SEMIO_PLUGIN_RUNTIME.with(|runtime| runtime.mounted_owner_policy());
    for grant in [selected.preparation, selected.maintenance, selected.close] {
        let wire: std::collections::BTreeMap<_, _> = grant.wire_fields().into_iter().collect();
        assert_eq!(serde_json::to_value(wire).expect("native policy fields"), fixture["grant"]);
        assert_eq!(serde_json::to_value(grant).expect("third-party policy output"), fixture["grant"]);
    }
}
