#[cfg(test)]
fn collect_fixture_actions(input: &mut InputState<ActionDescriptor>) -> Vec<ActionDescriptor> {
    let mut actions = Vec::new();
    while let Some(action) = input.take_action_step().expect("fixture action authority remains live") {
        actions.push(action.into_descriptor().expect("bounded fixture action materializes"));
    }
    actions
}

#[cfg(test)]
#[test]
fn realize_fault_remains_scheduled_until_the_aborted_cursor_is_terminal() {
    let glue = include_str!("../../🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs");
    let native = include_str!("../../🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs");
    assert!(glue.contains("cursor.phase = AppPresentPhase::Aborted;\n                            if self.retained_fault.is_none()"));
    assert!(glue.contains("return Ok(AppPresentStep::Pending);"));
    assert!(native.contains("if self.presenter.has_pending_presentation()"));
    assert!(native.contains("self.scheduler.invalidate(InvalidationReason::RESOURCE_READY);"));
}

#[cfg(all(test, not(target_arch = "wasm32")))]
#[test]
fn native_socket_probe_codec_encodes_the_fixture_shape_and_agrees_with_the_third_party_serializer() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧊️wgpu-native-socket-probe-codec/🔣️.json")).expect("language-neutral codec fixture parses");
    let probe = &fixture["nativeSocketProbeCodec"];
    let value = probe["value"].as_str().expect("fixture value is a string").to_string();
    let snapshot = NativeSocketProbeSnapshot(value.clone());
    let diff = NativeSocketProbeDiff(value.clone());
    let mutation = NativeSocketProbeMutation::Set(value);

    assert_eq!(serde_json::Value::from(store::ToValue::to_value(&snapshot)), probe["snapshotEncoding"]);
    assert_eq!(serde_json::Value::from(store::ToValue::to_value(&diff)), probe["diffEncoding"]);
    assert_eq!(serde_json::Value::from(store::ToValue::to_value(&mutation)), probe["mutationEncoding"]);
    assert_eq!(serde_json::to_value(&snapshot).expect("serde oracle"), probe["snapshotEncoding"]);
    assert_eq!(serde_json::to_value(&diff).expect("serde oracle"), probe["diffEncoding"]);
    assert_eq!(serde_json::to_value(&mutation).expect("serde oracle"), probe["mutationEncoding"]);

    assert_eq!(<NativeSocketProbeSnapshot as store::FromValue>::from_value(store::ToValue::to_value(&snapshot)).expect("snapshot round trip"), snapshot);
    assert_eq!(<NativeSocketProbeDiff as store::FromValue>::from_value(store::ToValue::to_value(&diff)).expect("diff round trip"), diff);
    assert_eq!(<NativeSocketProbeMutation as store::FromValue>::from_value(store::ToValue::to_value(&mutation)).expect("mutation round trip"), mutation);

    for rejected in probe["rejectedSnapshotEncodings"].as_array().expect("fixture snapshot rejections") {
        assert!(<NativeSocketProbeSnapshot as store::FromValue>::from_value(store::DslValue::from(rejected)).is_err(), "snapshot must reject {rejected}");
    }
    for rejected in probe["rejectedMutationEncodings"].as_array().expect("fixture mutation rejections") {
        assert!(<NativeSocketProbeMutation as store::FromValue>::from_value(store::DslValue::from(rejected)).is_err(), "mutation must reject {rejected}");
    }
}
