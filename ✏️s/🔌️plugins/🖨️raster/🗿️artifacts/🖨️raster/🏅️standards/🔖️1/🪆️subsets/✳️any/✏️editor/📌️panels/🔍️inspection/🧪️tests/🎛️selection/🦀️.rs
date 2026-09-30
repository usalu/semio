//! 🧪️ Shared selection vectors projected into native semantic controls.
use super::*;

#[test]
fn inspector_projects_selected_properties_and_foreground_in_both_languages() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎛️selection/🔣️.json")).unwrap();
    let mut document: RasterDocument = dsl::json::from_json_str(r#"{"schema":"raster.document","id":"inspection","title":"Inspection","layers":[{"kind":"pixel","id":"paint.foreground","name":"123","mask":null,"width":32,"height":32,"imageKey":null},{"kind":"pixel","id":"paint.background","name":"Background","mask":null,"width":64,"height":64,"imageKey":null}]}"#).unwrap();
    for (locale,labels) in [("en",&RasterPlayLabels::NATIVE_EN),("de",&RasterPlayLabels::NATIVE_DE)] {
        for case in fixture["cases"].as_array().unwrap() {
            let ids: Vec<String> = serde_json::from_value(case["selection"].clone()).unwrap();
            if let crate::RasterLayerNode::Pixel { mask, .. } = &mut document.layers[0] {
                *mask = case.get("mask").map(|value| dsl::json::from_json_str(&value.to_string()).unwrap());
            }
            let node = render(&document, &RasterConfig::default(), &ids, labels).unwrap();
            let text = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).unwrap();
            let projection: serde_json::Value = serde_json::from_str(&text).unwrap();
            let mut pending = vec![&projection];
            while let Some(node) = pending.pop() {
                if node["component"]["commit"] == "blur" {
                    assert_eq!(node["bindings"][0]["trigger"], fixture["blurTrigger"], "{}", node["key"]);
                }
                if case.get("mask").is_some() {
                    for control in fixture["maskControls"].as_array().unwrap() {
                        if node["key"] == format!("raster-inspector.{}.input",control["field"].as_str().unwrap()) {
                            assert_eq!(node["bindings"][0]["trigger"],control["trigger"]);
                            assert_eq!(node["bindings"][0]["action"]["name"],"patchLayers");
                            assert_eq!(node["bindings"][0]["args"]["field"],control["field"]);
                            assert!(node.to_string().contains(control["labels"][locale].as_str().unwrap()));
                            if let Some(value)=control.get("value") {assert_eq!(&node["component"]["value"],value);}
                            if let Some(step)=control.get("step") {assert_eq!(&node["component"]["step"],step);}
                        }
                    }
                }
                pending.extend(node["children"].as_array().unwrap());
            }
            assert!(text.contains("setBrushColor"));
            assert!(text.contains(labels.foreground.as_str()));
            assert_eq!(text.contains("patchLayers"), !ids.is_empty());
            for field in case["fields"].as_array().unwrap() {
                assert!(text.contains(&format!("raster-inspector.{}.input", field.as_str().unwrap())), "{text}");
            }
            if ids.len() == 2 { assert!(text.contains(labels.mixed.as_str())); }
            if !ids.is_empty(){for mode in fixture["blendModes"].as_array().unwrap(){assert!(text.contains(mode.as_str().unwrap()),"missing blend option {mode}: {text}");}}
        }
    }
    crate::standards::v1::subsets::any::schema::mutations::binary::unit_tests::retirement::retire_raster_snapshot(document);
}

#[semio_framework_async_macros::async_test]
async fn inspector_mask_dimensions_resolve_the_attached_image() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎛️selection/🔣️.json")).unwrap();
    let width=fixture["maskImageExtent"]["width"].as_u64().unwrap() as u32;
    let height=fixture["maskImageExtent"]["height"].as_u64().unwrap() as u32;
    let mut encoder=semio_framework_pixels::png_encoding::PngEncodeJob::new(semio_framework_pixels::RasterImage::new(width,height)).unwrap();
    while !encoder.advance().unwrap().done {}
    let bytes=encoder.into_result().unwrap().data;
    let mut document=crate::standards::v1::subsets::any::schema::empty_raster_snapshot();
    document.assets.insert("mask".into(),crate::mint_raster_asset_child("mask",&crate::RasterImageAsset {mime:"image/png".into(),data:bytes})).unwrap();
    let mut layer=crate::standards::v1::subsets::any::schema::create_layer_of_kind("pixel");
    if let RasterLayerNode::Pixel {mask,..}=&mut layer {*mask=Some(crate::RasterLayerMask {enabled:true,linked:true,invert:false,width:None,height:None,image_key:Some("mask".into()),transform:crate::RasterTransform::default()});}
    document.layers.push(layer);
    let resolved=[value(&document.layers[0],"maskWidth",&document),value(&document.layers[0],"maskHeight",&document)];
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(document);
    assert_eq!(resolved,[width.to_string(),height.to_string()]);
}

#[test]
fn inspector_adjustment_parameters_are_localized_bounded_commit_controls() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../🧬️schema/🧬️mutations/🎛️change-layer-adjustment-parameter/🧪️tests/🔣️.json")).unwrap();
    let mut document=crate::standards::v1::subsets::any::schema::empty_raster_snapshot();
    let layer=crate::standards::v1::subsets::any::schema::create_layer_of_kind("adjustment");
    let id=layer_node_id(&layer).to_owned();document.layers.push(layer);
    for (locale,labels) in [("en",&RasterPlayLabels::NATIVE_EN),("de",&RasterPlayLabels::NATIVE_DE)] {
        let tree=render(&document,&RasterConfig::default(),&[id.clone()],labels).unwrap();
        let text=semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(tree)).unwrap();
        let projection:serde_json::Value=serde_json::from_str(&text).unwrap();let mut pending=vec![&projection];let mut controls=0;
        while let Some(node)=pending.pop() {
            if ["raster-inspector.brightness.input","raster-inspector.contrast.input"].contains(&node["key"].as_str().unwrap_or("")) {
                assert_eq!(node["component"]["min"],-1.0);assert_eq!(node["component"]["max"],1.0);assert_eq!(node["component"]["step"],0.01);assert_eq!(node["component"]["value"],"0");assert_eq!(node["bindings"][0]["trigger"],"commit");controls+=1;
            }
            pending.extend(node["children"].as_array().unwrap());
        }
        assert_eq!(controls,2);assert!(!text.contains("raster-inspector.transformX.input"));
        for key in ["brightness","contrast"] {assert!(text.contains(fixture["labels"][locale][key].as_str().unwrap()));}
    }
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(document);
}

#[test]
fn inspector_offers_localized_merge_only_for_supported_sibling_selection() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🎮️commands/🫳️merge-down/🧫️fixtures/🔣️.json")).unwrap();
    let mut document=crate::standards::v1::subsets::any::schema::empty_raster_snapshot();document.layers=dsl::json::from_json_str(&fixture["cases"][0]["layers"].to_string()).unwrap();
    for (locale,labels) in [("en",&RasterPlayLabels::NATIVE_EN),("de",&RasterPlayLabels::NATIVE_DE)] {
        for ids in [vec!["upper".to_owned()],vec!["outside".to_owned()],vec!["lower".to_owned(),"upper".to_owned()],vec![]] {
            let tree=render(&document,&RasterConfig::default(),&ids,labels).unwrap();
            let mut pending=vec![&tree];let mut disabled=None;
            while let Some(node)=pending.pop() {if node.key.as_str()=="raster-inspector.merge-down" {disabled=Some(node.disabled);}pending.extend(node.children.iter());}
            let text=semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(tree)).unwrap();
            assert_eq!(text.contains("mergeDown"),ids.len()==1);
            if ids.len()==1 {
                assert!(text.contains(fixture["labels"][locale].as_str().unwrap()));assert!(text.contains(fixture["hints"][locale].as_str().unwrap()));
                assert_eq!(disabled,Some(ids!=["upper"]));
            }
        }
    }
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(document);
}

#[test]
fn inspector_protection_controls_follow_the_neutral_capabilities() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🗺️surface/🎨️paint/🧫️fixtures/🔒️protection/🔣️.json")).unwrap();
    let mut document=crate::standards::v1::subsets::any::schema::empty_raster_snapshot();document.layers=dsl::json::from_json_str(&fixture["layers"].to_string()).unwrap();
    for labels in [&RasterPlayLabels::NATIVE_EN,&RasterPlayLabels::NATIVE_DE] {
        for case in fixture["cases"].as_array().unwrap() {
            let tree=render(&document,&RasterConfig::default(),&[case["id"].as_str().unwrap().into()],labels).unwrap();
            let mut pending=vec![&tree];let mut lock_found=false;
            while let Some(node)=pending.pop() {
                let key=node.key.as_str();
                let enabled=match key {
                    "raster-inspector.locked.input"=>{lock_found=true;Some(case["expected"]["canChangeLock"].as_bool().unwrap())},
                    "raster-inspector.visible.input"=>Some(true),
                    "raster-inspector.name.input"|"raster-inspector.opacity.input"|"raster-inspector.maskPresent.input"=>Some(case["expected"]["editable"].as_bool().unwrap()),
                    "raster-inspector.transformX.input"|"raster-inspector.transformScaleX.input"|"raster-inspector.transformScaleY.input"|"raster-inspector.transformRotation.input"|"raster-inspector.transformShearX.input"|"raster-inspector.width.input"=>Some(case["expected"]["structural"].as_bool().unwrap()),
                    _=>None,
                };
                if let Some(enabled)=enabled {assert_eq!(node.disabled,!enabled,"{} {key}",case["id"]);}
                pending.extend(node.children.iter());
            }
            let text=semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(tree)).unwrap();
            assert!(lock_found);assert!(text.contains(labels.locked.as_str()));
        }
    }
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(document);
}

#[test]
fn inspector_layer_actions_are_localized_and_protection_aware() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎛️selection/🔣️.json")).unwrap();
    let protection:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🗺️surface/🎨️paint/🧫️fixtures/🔒️protection/🔣️.json")).unwrap();
    let mut document=crate::standards::v1::subsets::any::schema::empty_raster_snapshot();document.layers=dsl::json::from_json_str(&protection["layers"].to_string()).unwrap();
    for (locale,labels) in [("en",&RasterPlayLabels::NATIVE_EN),("de",&RasterPlayLabels::NATIVE_DE)] {
        for case in protection["cases"].as_array().unwrap() {
            let id=case["id"].as_str().unwrap();
            let tree=render(&document,&RasterConfig::default(),&[id.into()],labels).unwrap();
            for action in fixture["layerActions"].as_array().unwrap() {
                let key=format!("raster-inspector.{}",action["key"].as_str().unwrap());let mut pending=vec![&tree];let mut found=false;
                while let Some(node)=pending.pop() {
                    if node.key.as_str()==key {found=true;let enabled=if action["key"]=="duplicate" {!case["expected"]["inherited"].as_bool().unwrap()}else{case["expected"]["structural"].as_bool().unwrap()};assert_eq!(node.disabled,!enabled,"{id} {key}");}
                    pending.extend(node.children.iter());
                }
                assert!(found,"{key}");
            }
            let text=semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(tree)).unwrap();
            let projection:serde_json::Value=serde_json::from_str(&text).unwrap();let mut pending=vec![&projection];
            while let Some(node)=pending.pop() {
                for action in fixture["layerActions"].as_array().unwrap() {if node["key"]==format!("raster-inspector.{}",action["key"].as_str().unwrap()) {assert_eq!(node["component"]["target"]["activation"],action["command"]);assert_eq!(node["component"]["target"]["args"]["layerId"],id);assert!(node.to_string().contains(action["labels"][locale].as_str().unwrap()));}}
                pending.extend(node["children"].as_array().unwrap());
            }
        }
        for ids in [vec![],vec!["outside".into(),"editable-pixel".into()]] {
            let tree=render(&document,&RasterConfig::default(),&ids,labels).unwrap();let text=semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(tree)).unwrap();assert!(!text.contains("duplicateLayer"));assert!(!text.contains("deleteLayer"));
        }
    }
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(document);
}

#[test]
fn inspector_layer_transform_controls_use_localized_commit_bindings(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../🧬️schema/🧬️mutations/📐️change-layer-transform/🧪️tests/🔣️.json")).unwrap();
    for kind in ["pixel","group"] {
        let mut document=crate::standards::v1::subsets::any::schema::empty_raster_snapshot();document.layers.push(crate::standards::v1::subsets::any::schema::create_layer_of_kind(kind));let id=layer_node_id(&document.layers[0]).to_owned();
        for (locale,labels) in [("en",&RasterPlayLabels::NATIVE_EN),("de",&RasterPlayLabels::NATIVE_DE)] {
            let tree=render(&document,&RasterConfig::default(),&[id.clone()],labels).unwrap();
            let text=semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(tree)).unwrap();
            let projection:serde_json::Value=serde_json::from_str(&text).unwrap();let mut pending=vec![&projection];let mut found=0;
            while let Some(node)=pending.pop(){
                for row in fixture["controls"].as_array().unwrap(){let field=row["field"].as_str().unwrap();if node["key"]==format!("raster-inspector.{field}.input"){
                    found+=1;assert_eq!(node["bindings"][0]["trigger"],"commit");assert_eq!(node["bindings"][0]["action"]["name"],"patchLayers");assert_eq!(node["bindings"][0]["args"]["field"],field);assert!(node.to_string().contains(fixture["labels"][locale][field].as_str().unwrap()));
                }}
                pending.extend(node["children"].as_array().unwrap());
            }
            assert_eq!(found,4);
        }
        crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(document);
    }
}
