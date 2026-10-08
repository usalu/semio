use super::*;

#[semio_framework_async_macros::async_test]
async fn raster_config_operation_round_trips_and_backwards_restores_snapshot() {
    let base = RasterConfig { brush_size: 24.0, ..Default::default() };
    let operation = RasterConfigMutation::SetBrushSize { value: 40.0 };
    let forward = operation.diff(&base).diff().clone();
    assert_eq!(forward, RasterConfigDiff { brush_size: Some(40.0), ..Default::default() });
    let after = protocol::apply_diff(&forward, &base).unwrap();
    assert_eq!(after.brush_size, 40.0);
    let backwards = operation.inverse(&base).expect("valid retained mutation inverse fixture");
    assert_eq!(backwards, vec![RasterConfigMutation::SetBrushSize { value: 24.0 }]);
    assert_eq!(protocol::apply_diff(backwards[0].diff(&after).diff(), &after).unwrap(), base);
}

#[semio_framework_async_macros::async_test]
async fn raster_config_operation_op_text_round_trips_every_variant() {
    store::os_store::test_support::assert_op_line_round_trip(&RasterConfigMutation::SetBrushSize { value: 40.0 });
    store::os_store::test_support::assert_op_line_round_trip(&RasterConfigMutation::SetBrushOpacity { value: 0.5 });
    store::os_store::test_support::assert_op_line_round_trip(&RasterConfigMutation::SetCompositeViewport { viewport: Some(RasterConfigViewportSize { width: 640.0, height: 480.0 }) });
    store::os_store::test_support::assert_op_line_round_trip(&RasterConfigMutation::SetCompositeViewport { viewport: None });
    store::os_store::test_support::assert_op_line_round_trip(&RasterConfigMutation::SetCamera { camera: RasterCamera { x: 1.0, y: -2.0, zoom: 3.0 } });
}

#[semio_framework_async_macros::async_test]
async fn raster_config_default_matches_brush_controls() {
    let config = RasterConfig::default();
    assert_eq!(config.brush_size, 24.0);
    assert_eq!(config.brush_opacity, 1.0);
}

#[semio_framework_async_macros::async_test]
async fn raster_config_dsl_round_trips() {
    let config = RasterConfig { brush_size: 40.0, brush_opacity: 0.5, brush_color: "#e07020".into(), brush_hardness: 0.25,paint_target:"mask".into(),mask_value:96,fill_tolerance:40,pixel_selection:None,composite_viewport: Some(RasterConfigViewportSize { width: 640.0, height: 480.0 }), camera: RasterCamera { x: 5.0, y: -3.0, zoom: 2.0 } };
    store::os_store::test_support::assert_dsl_round_trip(&config);
}

#[semio_framework_async_macros::async_test]
async fn brush_style_shared_vectors_preserve_session_state_and_reject_invalid_values() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🖌️brush-style/🔣️.json")).unwrap();
    let base = RasterConfig::default();
    for row in fixture["cases"].as_array().unwrap() {
        let color = row["color"].as_str().unwrap().to_string();
        let hardness = row["hardness"].as_f64().unwrap();
        let color_op = RasterConfigMutation::SetBrushColor { value: color.clone() };
        let next = color_op.diff(&base).diff().clone();
        let next = RasterConfigMutation::SetBrushHardness { value: hardness }.diff(&next).diff().clone();
        assert_eq!(next.brush_color, color);
        assert_eq!(next.brush_hardness, hardness);
        assert_eq!(next.brush_rgba().to_vec(), row["rgba"].as_array().unwrap().iter().map(|v| v.as_u64().unwrap() as u8).collect::<Vec<_>>());
        let mut oracle: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&base)).unwrap();
        oracle["brushColor"] = row["color"].clone();
        oracle["brushHardness"] = row["hardness"].clone();
        let expected: RasterConfig = semio_framework_pack_json::from_json_str(&oracle.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert_eq!(next, expected);
        assert_eq!(color_op.inverse(&base).expect("valid retained mutation inverse fixture")[0].diff(&next).diff(), &base);
        store::os_store::test_support::assert_op_line_round_trip(&color_op);
        store::os_store::test_support::assert_dsl_round_trip(&next);
    }
    for color in fixture["invalidColors"].as_array().unwrap() {
        assert!(!valid_brush_color(color.as_str().unwrap()));
    }
    for value in [f64::NAN, f64::INFINITY, -0.1, 1.1] {
        assert_eq!(RasterConfigMutation::SetBrushHardness { value }.diff(&base).diff(), &base);
    }
}

#[semio_framework_async_macros::async_test]
async fn mask_paint_config_shared_vectors_round_trip_and_restore(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎭️mask-paint/🔣️.json")).unwrap();let base=RasterConfig::default();
    for row in fixture["cases"].as_array().unwrap(){
        let target=RasterConfigMutation::SetPaintTarget{value:row["target"].as_str().unwrap().into()};let value=RasterConfigMutation::SetMaskValue{value:row["value"].as_u64().unwrap() as u32};let next=target.diff(&base).diff().clone();let next=value.diff(&next).diff().clone();assert_eq!(next.paint_target,row["target"].as_str().unwrap());assert_eq!(next.mask_value,row["value"].as_u64().unwrap() as u32);assert_eq!(target.inverse(&base).expect("valid retained mutation inverse fixture")[0].diff(&next).diff(),&base);store::os_store::test_support::assert_dsl_round_trip(&next);store::os_store::test_support::assert_op_line_round_trip(&target);store::os_store::test_support::assert_op_line_round_trip(&value);
    }
    for value in fixture["invalidTargets"].as_array().unwrap(){assert_eq!(RasterConfigMutation::SetPaintTarget{value:value.as_str().unwrap().into()}.diff(&base).diff(),&base);}
    assert_eq!(RasterConfigMutation::SetMaskValue{value:256}.diff(&base).diff(),&base);
}

/// 🌊️ The bucket tolerance vectors shared with the TypeScript schema twin: every admitted value lands and inverts, a value
/// past 255 leaves the session untouched.
#[semio_framework_async_macros::async_test]
async fn fill_tolerance_shared_vectors_round_trip_and_restore() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🌊️fill-tolerance/🔣️.json")).unwrap();
    let base = RasterConfig::default();
    assert_eq!(base.fill_tolerance, 24);
    for row in fixture["cases"].as_array().unwrap() {
        let value = row.as_u64().unwrap() as u32;
        let op = RasterConfigMutation::SetFillTolerance { value };
        let next = op.diff(&base).diff().clone();
        assert_eq!(next.fill_tolerance, value);
        assert_eq!(op.inverse(&base).expect("valid retained mutation inverse fixture")[0].diff(&next).diff(), &base);
        store::os_store::test_support::assert_op_line_round_trip(&op);
        store::os_store::test_support::assert_dsl_round_trip(&next);
    }
    for value in fixture["invalidValues"].as_array().unwrap().iter().filter_map(serde_json::Value::as_u64) {
        assert_eq!(RasterConfigMutation::SetFillTolerance { value: value as u32 }.diff(&base).diff(), &base);
    }
}

#[semio_framework_async_macros::async_test]
async fn completed_selection_contract_round_trips_and_survives_style_changes(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎯️pixel-selection/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap(){
        let selection:RasterPixelSelection=semio_framework_pack_json::from_json_str(&row["selection"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let mut coverage=vec![0;selection.validate().unwrap()];
        for (start,end,alpha) in selection.spans().unwrap(){coverage[start..end].fill(alpha);}
        assert_eq!(coverage,row["coverage"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap() as u8).collect::<Vec<_>>());
        let base=RasterConfig{paint_target:selection.target.clone(),..Default::default()};
        let op=RasterConfigMutation::SetPixelSelection{selection:Some(selection.clone())};
        let next=op.diff(&base).diff().clone();assert_eq!(next.pixel_selection,Some(selection));
        assert_eq!(RasterConfigMutation::SetBrushSize{value:12.0}.diff(&next).diff().pixel_selection,next.pixel_selection);
        assert_eq!(RasterConfigMutation::SetPaintTarget{value:next.paint_target.clone()}.diff(&next).diff(),&next);
        let target=if next.paint_target=="pixels"{"mask"}else{"pixels"};
        assert!(RasterConfigMutation::SetPaintTarget{value:target.into()}.diff(&next).diff().pixel_selection.is_none());
        assert!(RasterConfigMutation::SetPixelSelection{selection:None}.diff(&next).diff().pixel_selection.is_none());
        assert_eq!(op.inverse(&base).expect("valid retained mutation inverse fixture")[0].diff(&next).diff(),&base);
        store::os_store::test_support::assert_dsl_round_trip(&next);
        store::os_store::test_support::assert_op_line_round_trip(&op);
        store::os_store::test_support::assert_op_line_round_trip(&RasterConfigMutation::SetPixelSelection{selection:None});
    }
    let base=&fixture["cases"][0]["selection"];
    for invalid in fixture["invalid"].as_array().unwrap(){
        let mut value=base.clone();for(key,item)in invalid.as_object().unwrap(){value[key]=item.clone();}
        let selection:RasterPixelSelection=semio_framework_pack_json::from_json_str(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert!(selection.validate().is_err(),"{value}");
        let config=RasterConfig::default();assert_eq!(RasterConfigMutation::SetPixelSelection{selection:Some(selection)}.diff(&config).diff(),&config);
    }
}

#[semio_framework_async_macros::async_test]
async fn raster_config_inverses_sum_to_the_negative_diff() {
    use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;
    let base = RasterConfig { composite_viewport: Some(RasterConfigViewportSize { width: 640.0, height: 480.0 }), ..RasterConfig::default() };
    for mutation in [
        RasterConfigMutation::SetBrushSize { value: 40.0 },
        RasterConfigMutation::SetBrushOpacity { value: 0.5 },
        RasterConfigMutation::SetBrushColor { value: "#112233".into() },
        RasterConfigMutation::SetBrushHardness { value: 0.25 },
        RasterConfigMutation::SetPaintTarget { value: "mask".into() },
        RasterConfigMutation::SetMaskValue { value: 7 },
        RasterConfigMutation::SetFillTolerance { value: 3 },
        RasterConfigMutation::SetCompositeViewport { viewport: None },
        RasterConfigMutation::SetCamera { camera: RasterCamera { x: 1.0, y: -2.0, zoom: 3.0 } },
    ] {
        assert_mutation_inverse_sum_law(&mutation, &base).await;
    }
    protocol::os_spr::protocol_laws::assert_diff_algebra_between_law::<RasterConfig, RasterConfigDiff>(&base, &RasterConfig::default()).await;
}
