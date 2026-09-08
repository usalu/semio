#[semio_framework_async_macros::async_test]
async fn shell_fault_frame_round_trips_the_language_neutral_diagnostic() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/🔣️.json")).unwrap();
    let fault = semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.surface-render"), fixture["wire"]["fault"].as_str().unwrap());
    let semio_framework::kernel::Effect::SendMessage { target, payload } = super::shell_fault_effect(7, &fault) else { panic!("shell diagnostic effect") };
    assert!(matches!(target, semio_framework::kernel::MessageEndpoint::Shell { instance } if instance.0 == "7"));
    let protocol::AppFrame::Error { in_reply_to, fault: bytes, report } = protocol::decode_app_frame(&payload).await.expect("shell diagnostic must use the app frame protocol") else { panic!("shell diagnostic error frame") };
    assert_eq!(in_reply_to, None);
    assert!(report.is_empty());
    let decoded: semio_framework::Fault = dsl::from_dsl_value(store::pack_rt::decode_wire_value(&bytes).unwrap()).unwrap();
    assert_eq!(serde_json::Value::from(protocol::ToValue::to_value(&decoded)), serde_json::Value::from(protocol::ToValue::to_value(&fault)));
}
