#[cfg(test)]
fn collect_fixture_actions(input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Vec<ActionDescriptor> {
    let mut actions = Vec::new();
    while let Some(action) = input.take_action_step().expect("fixture action authority remains live") {
        let queued = action.into_envelope().expect("bounded fixture action materializes");
        if let Some(receipt) = queued.receipt { settle_renderer_action_receipt(receipt, engine_canvas::TextEditorActionOutcome::Accepted); }
        actions.push(queued.descriptor);
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

    assert_eq!(serde_json::Value::from(semio_framework_value::ToValue::to_value(&snapshot)), probe["snapshotEncoding"]);
    assert_eq!(serde_json::Value::from(semio_framework_value::ToValue::to_value(&diff)), probe["diffEncoding"]);
    assert_eq!(serde_json::Value::from(semio_framework_value::ToValue::to_value(&mutation)), probe["mutationEncoding"]);
    assert_eq!(serde_json::to_value(&snapshot).expect("serde oracle"), probe["snapshotEncoding"]);
    assert_eq!(serde_json::to_value(&diff).expect("serde oracle"), probe["diffEncoding"]);
    assert_eq!(serde_json::to_value(&mutation).expect("serde oracle"), probe["mutationEncoding"]);

    assert_eq!(<NativeSocketProbeSnapshot as semio_framework_value::FromValue>::from_value(semio_framework_value::ToValue::to_value(&snapshot)).expect("snapshot round trip"), snapshot);
    assert_eq!(<NativeSocketProbeDiff as semio_framework_value::FromValue>::from_value(semio_framework_value::ToValue::to_value(&diff)).expect("diff round trip"), diff);
    assert_eq!(<NativeSocketProbeMutation as semio_framework_value::FromValue>::from_value(semio_framework_value::ToValue::to_value(&mutation)).expect("mutation round trip"), mutation);

    for rejected in probe["rejectedSnapshotEncodings"].as_array().expect("fixture snapshot rejections") {
        assert!(<NativeSocketProbeSnapshot as semio_framework_value::FromValue>::from_value(semio_framework_value::DslValue::from(rejected)).is_err(), "snapshot must reject {rejected}");
    }
    for rejected in probe["rejectedMutationEncodings"].as_array().expect("fixture mutation rejections") {
        assert!(<NativeSocketProbeMutation as semio_framework_value::FromValue>::from_value(semio_framework_value::DslValue::from(rejected)).is_err(), "mutation must reject {rejected}");
    }
}

#[cfg(test)]
#[test]
fn glb_outline_accumulator_matches_the_neutral_three_geometry_contract() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/🎨️world3d-glb-outline/🔣️.json")).expect("GLB outline fixture");
    let identity: GlbMatrix = [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]];
    let compare_point = |left: &[f32; 3], right: &[f32; 3]| left.iter().zip(right).find_map(|(left, right)| (left != right).then(|| left.total_cmp(right))).unwrap_or(std::cmp::Ordering::Equal);
    for record in fixture["cases"].as_array().expect("outline cases") {
        let positions = record["positions"]
            .as_array()
            .expect("positions")
            .iter()
            .map(|point| point.as_array().expect("point").iter().map(|value| value.as_f64().expect("coordinate") as f32).collect::<Vec<_>>().try_into().expect("three coordinates"))
            .collect::<Vec<[f32; 3]>>();
        let indices = record["indices"].as_array().expect("indices").iter().map(|value| value.as_u64().expect("index") as usize).collect::<Vec<_>>();
        let mut outline = GlbOutlineAccumulator::new(indices.len()).expect("bounded outline owner");
        for triangle in indices.chunks_exact(3) {
            outline.push_triangle(0, identity, [positions[triangle[0]], positions[triangle[1]], positions[triangle[2]]]).expect("one triangle step");
        }
        while !outline.flush_step().expect("one retained edge flush") {}
        let mut actual = outline.segments;
        for segment in &mut actual {
            if compare_point(&segment[0], &segment[1]).is_gt() {
                segment.swap(0, 1);
            }
        }
        actual.sort_by(|left, right| compare_point(&left[0], &right[0]).then_with(|| compare_point(&left[1], &right[1])));
        let mut expected = record["expectedSegments"]
            .as_array()
            .expect("expected segments")
            .iter()
            .map(|segment| {
                let points = segment.as_array().expect("segment");
                std::array::from_fn(|point| {
                    let coordinates = points[point].as_array().expect("point");
                    std::array::from_fn(|axis| coordinates[axis].as_f64().expect("coordinate") as f32 * GLB_OUTLINE_SCALE)
                })
            })
            .collect::<Vec<[[f32; 3]; 2]>>();
        expected.sort_by(|left, right| compare_point(&left[0], &right[0]).then_with(|| compare_point(&left[1], &right[1])));
        assert_eq!(actual, expected, "{}", record["id"].as_str().expect("case id"));
    }
}
