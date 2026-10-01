use super::*;

#[semio_framework_async_macros::async_test]
async fn service_operation_preserves_its_exact_owner_action_and_payload() {
    let payload = semio_framework::DslValue::Object(vec![("jobId".into(), semio_framework::DslValue::String("job-alpha".into()))]);
    let effect = wit_effects::Effect::RequestServiceOperation(wit_effects::RequestServiceOperationEffect {
        owner: "alpha".into(), service_id: "alpha.document.compute".into(), action: "cancel".into(), payload: store::pack_rt::encode_wire_value(&payload),
    });
    let converted = wit_effect_to_kernel(effect).await.unwrap();
    assert_eq!(converted, semio_framework::kernel::Effect::RequestServiceOperation { owner: "alpha".into(), service_id: "alpha.document.compute".into(), action: "cancel".into(), payload });
}

#[semio_framework_async_macros::async_test]
async fn service_operation_rejects_a_malformed_payload() {
    let effect = wit_effects::Effect::RequestServiceOperation(wit_effects::RequestServiceOperationEffect {
        owner: "alpha".into(), service_id: "alpha.document.compute".into(), action: "cancel".into(), payload: vec![255],
    });
    assert!(wit_effect_to_kernel(effect).await.is_err());
}
