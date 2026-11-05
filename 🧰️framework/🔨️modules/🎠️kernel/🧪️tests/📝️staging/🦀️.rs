//! 📨️ The explicit staged donor transports original bytes independently of typed view context.
#[test]
fn staged_request_event_round_trips_the_neutral_donor_without_retyping_view_context() {
    let law:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
    for key in ["event","separate"] {
        let event:crate::Event=serde_json::from_value(law[key].clone()).expect("declared event variant");
        assert_eq!(serde_json::to_value(&event).unwrap(),law[key]);
    }
}
