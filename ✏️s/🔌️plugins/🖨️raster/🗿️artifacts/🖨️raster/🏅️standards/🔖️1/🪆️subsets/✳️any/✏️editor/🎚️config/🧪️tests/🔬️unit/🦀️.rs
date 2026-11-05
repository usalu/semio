use super::*;

#[semio_framework_async_macros::async_test]
async fn session_grid_settings_follow_neutral_values_and_restore_exactly() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/image-grid.json")).unwrap();
    let base = RasterConfig::default();
    for row in fixture["cases"].as_array().unwrap() {
        let grid: RasterImageGrid = semio_framework_pack_json::from_json_str(&row.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert!(grid.valid());
        let operation = RasterConfigMutation::SetImageGrid(grid.clone());
        let next = protocol::apply_diff_cold(operation.diff(&base).diff(), &base).unwrap();
        assert_eq!(next.image_grid, grid);
        let inverse = operation.inverse(&base).unwrap();
        assert_eq!(protocol::apply_diff_cold(inverse[0].diff(&next).diff(), &next).unwrap(), base);
        store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
    }
}

#[semio_framework_async_macros::async_test]
async fn session_filter_options_follow_neutral_bounds_and_restore_exactly() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/filter-options.json")).unwrap();
    let base = RasterConfig::default();
    for row in fixture["cases"].as_array().unwrap() {
        let kind = row["filter"].as_str().unwrap();
        let amount = row["amount"].as_f64().unwrap();
        assert!(valid_filter_options(kind, amount));
        let operation = RasterConfigMutation::SetFilterOptions(SetFilterOptionsEdit { filter: kind.into(), amount });
        let next = protocol::apply_diff_cold(operation.diff(&base).diff(), &base).unwrap();
        assert_eq!(next.filter_kind, kind);
        assert_eq!(next.filter_amount, amount);
        let inverse = operation.inverse(&base).unwrap();
        let restored = protocol::apply_diff_cold(inverse[0].diff(&next).diff(), &next).unwrap();
        assert_eq!(restored, base);
        store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
    }
    for row in fixture["invalid"].as_array().unwrap() {
        let kind = row["filter"].as_str().unwrap();
        let amount = row["amount"].as_f64().unwrap();
        assert!(!valid_filter_options(kind, amount));
        assert_eq!(RasterConfigMutation::SetFilterOptions(SetFilterOptionsEdit { filter: kind.into(), amount }).diff(&base).diff(), &RasterConfigDiff::default());
    }
}

#[semio_framework_async_macros::async_test]
async fn raster_config_operation_round_trips_and_backwards_restores_snapshot() {
    let base = RasterConfig { brush_size: 24.0, ..Default::default() };
    let operation = RasterConfigMutation::SetBrushSize(SetBrushSizeEdit { value: 40.0 });
    let forward = operation.diff(&base).diff().clone();
    assert_eq!(forward, RasterConfigDiff { brush_size: Some(40.0), ..Default::default() });
    let after = protocol::apply_diff_cold(&forward, &base).unwrap();
    assert_eq!(after.brush_size, 40.0);
    let backwards = operation.inverse(&base).expect("valid retained mutation inverse fixture");
    assert_eq!(backwards, vec![RasterConfigMutation::SetBrushSize(SetBrushSizeEdit { value: 24.0 })]);
    assert_eq!(protocol::apply_diff_cold(backwards[0].diff(&after).diff(), &after).unwrap(), base);
}

#[semio_framework_async_macros::async_test]
async fn raster_config_operation_op_text_round_trips_every_variant() {
    store::os_store::test_support::assert_op_line_round_trip(&RasterConfigMutation::SetBrushSize(SetBrushSizeEdit { value: 40.0 }));
    store::os_store::test_support::assert_op_line_round_trip(&RasterConfigMutation::SetBrushOpacity(SetBrushOpacityEdit { value: 0.5 }));
    store::os_store::test_support::assert_op_line_round_trip(&RasterConfigMutation::SetCompositeViewport(SetCompositeViewportEdit { viewport: Some(RasterConfigViewportSize { width: 640.0, height: 480.0 }) }));
    store::os_store::test_support::assert_op_line_round_trip(&RasterConfigMutation::SetCompositeViewport(SetCompositeViewportEdit { viewport: None }));
    store::os_store::test_support::assert_op_line_round_trip(&RasterConfigMutation::SetCamera(SetCameraEdit { camera: RasterCamera { x: 1.0, y: -2.0, zoom: 3.0 } }));
}

#[semio_framework_async_macros::async_test]
async fn raster_config_default_matches_brush_controls() {
    let config = RasterConfig::default();
    assert_eq!(config.brush_size, 24.0);
    assert_eq!(config.brush_opacity, 1.0);
}

#[semio_framework_async_macros::async_test]
async fn raster_config_dsl_round_trips() {
    let config = RasterConfig { brush_size: 40.0, brush_opacity: 0.5, brush_color: "#e07020".into(), brush_hardness: 0.25,paint_target:"mask".into(),mask_value:96,fill_tolerance:40,filter_kind:"gamma".into(),filter_amount:1.25,image_grid:RasterImageGrid::default(),pixel_selection:None,composite_viewport: Some(RasterConfigViewportSize { width: 640.0, height: 480.0 }), camera: RasterCamera { x: 5.0, y: -3.0, zoom: 2.0 } };
    store::os_store::test_support::assert_dsl_round_trip(&config);
}

#[semio_framework_async_macros::async_test]
async fn brush_style_shared_vectors_preserve_session_state_and_reject_invalid_values() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🖌️brush-style/🔣️.json")).unwrap();
    let base = RasterConfig::default();
    for row in fixture["cases"].as_array().unwrap() {
        let color = row["color"].as_str().unwrap().to_string();
        let hardness = row["hardness"].as_f64().unwrap();
        let color_op = RasterConfigMutation::SetBrushColor(SetBrushColorEdit { value: color.clone() });
        let colored = protocol::apply_diff_cold(color_op.diff(&base).diff(),&base).unwrap();
        let hardness_op=RasterConfigMutation::SetBrushHardness(SetBrushHardnessEdit{value:hardness});
        let next = protocol::apply_diff_cold(hardness_op.diff(&colored).diff(),&colored).unwrap();
        assert_eq!(next.brush_color, color);
        assert_eq!(next.brush_hardness, hardness);
        assert_eq!(next.brush_rgba().to_vec(), row["rgba"].as_array().unwrap().iter().map(|v| v.as_u64().unwrap() as u8).collect::<Vec<_>>());
        let mut oracle: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&base)).unwrap();
        oracle["brushColor"] = row["color"].clone();
        oracle["brushHardness"] = row["hardness"].clone();
        let expected: RasterConfig = semio_framework_pack_json::from_json_str(&oracle.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert_eq!(next, expected);
        let restored=protocol::apply_diff_cold(hardness_op.inverse(&colored).unwrap()[0].diff(&next).diff(),&next).unwrap();
        assert_eq!(protocol::apply_diff_cold(color_op.inverse(&base).unwrap()[0].diff(&restored).diff(),&restored).unwrap(),base);
        store::os_store::test_support::assert_op_line_round_trip(&color_op);
        store::os_store::test_support::assert_dsl_round_trip(&next);
    }
    for color in fixture["invalidColors"].as_array().unwrap() {
        assert!(!valid_brush_color(color.as_str().unwrap()));
    }
    for value in [f64::NAN, f64::INFINITY, -0.1, 1.1] {
        assert_eq!(RasterConfigMutation::SetBrushHardness(SetBrushHardnessEdit { value }).diff(&base).diff(), &RasterConfigDiff::default());
    }
}

#[semio_framework_async_macros::async_test]
async fn mask_paint_config_shared_vectors_round_trip_and_restore(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🎭️mask-paint/🔣️.json")).unwrap();let base=RasterConfig::default();
    for row in fixture["cases"].as_array().unwrap(){
        let target=RasterConfigMutation::SetPaintTarget(SetPaintTargetEdit {value:row["target"].as_str().unwrap().into()});let value=RasterConfigMutation::SetMaskValue(SetMaskValueEdit {value:row["value"].as_u64().unwrap() as u32});let targeted=protocol::apply_diff_cold(target.diff(&base).diff(),&base).unwrap();let next=protocol::apply_diff_cold(value.diff(&targeted).diff(),&targeted).unwrap();assert_eq!(next.paint_target,row["target"].as_str().unwrap());assert_eq!(next.mask_value,row["value"].as_u64().unwrap() as u32);let restored=protocol::apply_diff_cold(value.inverse(&targeted).unwrap()[0].diff(&next).diff(),&next).unwrap();assert_eq!(protocol::apply_diff_cold(target.inverse(&base).unwrap()[0].diff(&restored).diff(),&restored).unwrap(),base);store::os_store::test_support::assert_dsl_round_trip(&next);store::os_store::test_support::assert_op_line_round_trip(&target);store::os_store::test_support::assert_op_line_round_trip(&value);
    }
    for value in fixture["invalidTargets"].as_array().unwrap(){assert_eq!(RasterConfigMutation::SetPaintTarget(SetPaintTargetEdit {value:value.as_str().unwrap().into()}).diff(&base).diff(),&RasterConfigDiff::default());}
    assert_eq!(RasterConfigMutation::SetMaskValue(SetMaskValueEdit {value:256}).diff(&base).diff(),&RasterConfigDiff::default());
}

/// 🌊️ The bucket tolerance vectors shared with the TypeScript schema twin: every admitted value lands and inverts, a value
/// past 255 leaves the session untouched.
#[semio_framework_async_macros::async_test]
async fn fill_tolerance_shared_vectors_round_trip_and_restore() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🌊️fill-tolerance/🔣️.json")).unwrap();
    let base = RasterConfig::default();
    assert_eq!(base.fill_tolerance, 24);
    for row in fixture["cases"].as_array().unwrap() {
        let value = row.as_u64().unwrap() as u32;
        let op = RasterConfigMutation::SetFillTolerance(SetFillToleranceEdit { value });
        let next = protocol::apply_diff_cold(op.diff(&base).diff(),&base).unwrap();
        assert_eq!(next.fill_tolerance, value);
        assert_eq!(protocol::apply_diff_cold(op.inverse(&base).unwrap()[0].diff(&next).diff(),&next).unwrap(),base);
        store::os_store::test_support::assert_op_line_round_trip(&op);
        store::os_store::test_support::assert_dsl_round_trip(&next);
    }
    for value in fixture["invalidValues"].as_array().unwrap().iter().filter_map(serde_json::Value::as_u64) {
        assert_eq!(RasterConfigMutation::SetFillTolerance(SetFillToleranceEdit { value: value as u32 }).diff(&base).diff(), &RasterConfigDiff::default());
    }
}

#[semio_framework_async_macros::async_test]
async fn completed_selection_contract_round_trips_and_survives_style_changes(){
    use semio_framework_plugin::artifact_app_laws::OriginalWorkFixture;
    use store::snapshot_clone_preparation::config_apply::ConfigApplyMutation;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🎯️pixel-selection/🔣️.json")).unwrap();
    let mut caller=OriginalWorkFixture::new();let grant=OriginalWorkFixture::stock();
    for row in fixture["cases"].as_array().unwrap(){
        let selection:RasterPixelSelection=caller.receive_native(semio_framework_value::DslValue::from(row["selection"].clone()),grant);
        let mut coverage=vec![0;selection.validate().unwrap()];
        for (start,end,alpha) in selection.spans().unwrap(){coverage[start..end].fill(alpha);}
        assert_eq!(coverage,row["coverage"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap() as u8).collect::<Vec<_>>());
        let mut next=RasterConfig{paint_target:selection.target.clone(),..Default::default()};
        let inverse=RasterConfigMutation::SetPixelSelection(SetPixelSelectionEdit{selection:Some(selection)}).exchange(&mut next).unwrap();
        let original=next.pixel_selection.clone();
        let style_inverse=RasterConfigMutation::SetBrushSize(SetBrushSizeEdit{value:12.0}).exchange(&mut next).unwrap();assert_eq!(next.pixel_selection,original);
        let same=RasterConfigMutation::SetPaintTarget(SetPaintTargetEdit{value:next.paint_target.clone()}).exchange(&mut next).unwrap();assert!(same.take_followup_inverse(&mut next).is_none());assert_eq!(next.pixel_selection,original);
        let decoded:RasterConfig=caller.receive_native(semio_framework_value::ToValue::to_value(&next),grant);assert_eq!(decoded,next);caller.retire_owned(decoded,grant);
        let target=if next.paint_target=="pixels"{"mask"}else{"pixels"};
        let target_inverse=RasterConfigMutation::SetPaintTarget(SetPaintTargetEdit{value:target.into()}).exchange(&mut next).unwrap();let selection_inverse=target_inverse.take_followup_inverse(&mut next).unwrap();assert!(next.pixel_selection.is_none());
        caller.retire_owned((original,inverse,style_inverse,same,target_inverse,selection_inverse,next),grant);
    }
    let base=&fixture["cases"][0]["selection"];
    for invalid in fixture["invalid"].as_array().unwrap(){
        let mut value=base.clone();for(key,item)in invalid.as_object().unwrap(){value[key]=item.clone();}
        match caller.try_receive_native::<_,RasterPixelSelection>(semio_framework_value::DslValue::from(value.clone()),grant){
            Ok(selection)=>{assert!(selection.validate().is_err(),"{value}");caller.retire_owned(selection,grant);},
            Err(error)=>caller.retire_owned(error,grant),
        }
    }
    caller.finish();
}

#[semio_framework_async_macros::async_test]
async fn raster_config_inverses_sum_to_the_negative_diff() {
    use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;
    let base = RasterConfig { composite_viewport: Some(RasterConfigViewportSize { width: 640.0, height: 480.0 }), ..RasterConfig::default() };
    for mutation in [
        RasterConfigMutation::SetBrushSize(SetBrushSizeEdit { value: 40.0 }),
        RasterConfigMutation::SetBrushOpacity(SetBrushOpacityEdit { value: 0.5 }),
        RasterConfigMutation::SetBrushColor(SetBrushColorEdit { value: "#112233".into() }),
        RasterConfigMutation::SetBrushHardness(SetBrushHardnessEdit { value: 0.25 }),
        RasterConfigMutation::SetPaintTarget(SetPaintTargetEdit { value: "mask".into() }),
        RasterConfigMutation::SetMaskValue(SetMaskValueEdit { value: 7 }),
        RasterConfigMutation::SetFillTolerance(SetFillToleranceEdit { value: 3 }),
        RasterConfigMutation::SetCompositeViewport(SetCompositeViewportEdit { viewport: None }),
        RasterConfigMutation::SetCamera(SetCameraEdit { camera: RasterCamera { x: 1.0, y: -2.0, zoom: 3.0 } }),
    ] {
        assert_mutation_inverse_sum_law(&mutation, &base).await;
    }
}

/// 🧬️ The newtype payload variants keep the exact externally tagged JSON of the former named variants (compared as parsed serde_json values, in both directions).
#[semio_framework_async_macros::async_test]
async fn raster_config_mutation_wire_is_identical_to_the_named_variant_shape() {
    let rows: Vec<(RasterConfigMutation, &str)> = vec![
        (RasterConfigMutation::SetBrushSize(SetBrushSizeEdit { value: 40.0 }), r#"{"SetBrushSize":{"value":40.0}}"#),
        (RasterConfigMutation::SetBrushOpacity(SetBrushOpacityEdit { value: 0.5 }), r#"{"SetBrushOpacity":{"value":0.5}}"#),
        (RasterConfigMutation::SetBrushColor(SetBrushColorEdit { value: "#e07020".into() }), r##"{"SetBrushColor":{"value":"#e07020"}}"##),
        (RasterConfigMutation::SetBrushHardness(SetBrushHardnessEdit { value: 0.25 }), r#"{"SetBrushHardness":{"value":0.25}}"#),
        (RasterConfigMutation::SetPaintTarget(SetPaintTargetEdit { value: "mask".into() }), r#"{"SetPaintTarget":{"value":"mask"}}"#),
        (RasterConfigMutation::SetMaskValue(SetMaskValueEdit { value: 96 }), r#"{"SetMaskValue":{"value":96}}"#),
        (RasterConfigMutation::SetFillTolerance(SetFillToleranceEdit { value: 40 }), r#"{"SetFillTolerance":{"value":40}}"#),
        (RasterConfigMutation::SetPixelSelection(SetPixelSelectionEdit { selection: None }), r#"{"SetPixelSelection":{"selection":null}}"#),
        (RasterConfigMutation::SetCompositeViewport(SetCompositeViewportEdit { viewport: Some(RasterConfigViewportSize { width: 640.0, height: 480.0 }) }), r#"{"SetCompositeViewport":{"viewport":{"width":640.0,"height":480.0}}}"#),
        (RasterConfigMutation::SetCamera(SetCameraEdit { camera: RasterCamera { x: 1.0, y: -2.0, zoom: 3.0 } }), r#"{"SetCamera":{"camera":{"x":1.0,"y":-2.0,"zoom":3.0}}}"#),
    ];
    for (mutation, legacy) in rows {
        let encoded: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&mutation)).unwrap();
        assert_eq!(encoded, serde_json::from_str::<serde_json::Value>(legacy).unwrap(), "{legacy}");
        let decoded: RasterConfigMutation = semio_framework_pack_json::from_json_str(legacy, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert_eq!(decoded, mutation, "{legacy}");
    }
}
