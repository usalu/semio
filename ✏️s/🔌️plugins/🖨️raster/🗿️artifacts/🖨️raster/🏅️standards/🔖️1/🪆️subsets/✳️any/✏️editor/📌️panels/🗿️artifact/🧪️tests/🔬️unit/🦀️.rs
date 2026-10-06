use super::*;

//#region 🪟️WindowLaws
use crate::RasterTransform;
use semio_framework_plugin::{TreeWindowRequest, ViewModel, TREE_WINDOW_DEFAULT_ROWS};

fn pixel_layer(id: &str, name: &str) -> RasterLayerNode {
    RasterLayerNode::Pixel { id: id.into(), name: name.into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, width: Some(64), height: Some(64), image_key: None }
}

/// 🪟️ A document an order of magnitude past one viewport, whose FIRST layer is a group holding
/// `nested` children — so every law below covers the nested container too, not only the section.
fn oversized_document(top: usize, nested: usize) -> RasterDocument {
    let children = (0..nested).map(|index| pixel_layer(&format!("nested-{index}"), &format!("Nested {index}"))).collect();
    let group = RasterLayerNode::Group { id: "group-0".into(), name: "Group 0".into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, children };
    let mut layers = vec![group];
    layers.extend((1..top).map(|index| pixel_layer(&format!("pixel-{index}"), &format!("Pixel {index}"))));
    RasterDocument { layers, ..Default::default() }
}

/// 🪟️ The panel body exactly as the host reads it, for the host-known windows in `requests`.
fn window_body(document: &RasterDocument, requests: Vec<TreeWindowRequest>) -> String {
    let view = ViewModel { tree_windows: requests, ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    let node = render(document, &RasterConfig::default(), &RasterPlayLabels::NATIVE_EN, &TreeWindows::for_body(&view, RASTER_PLAY_BODY_LAYERS)).expect("render the raster layer tree");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("project the raster layer tree")
}

fn open(node_key: &str, offset: u32, rows: u32) -> TreeWindowRequest {
    TreeWindowRequest { body_key: RASTER_PLAY_BODY_LAYERS.into(), node_key: node_key.into(), open: Some(true), offset, rows }
}

fn closed(node_key: &str) -> TreeWindowRequest {
    TreeWindowRequest { body_key: RASTER_PLAY_BODY_LAYERS.into(), node_key: node_key.into(), open: Some(false), offset: 0, rows: 0 }
}

/// 🔑️ A windowed container nested inside the section is addressed by its **window path** — the
/// enclosing windowed containers' keys, outermost first, then its own key, joined by
/// `TREE_WINDOW_PATH_SEPARATOR` (`TreeWindows::path_of`). A bare node key only ever matches a
/// top-level container, so the nested laws below file the path the host really sends.
fn nested_key(node_key: &str) -> String {
    format!("{RASTER_TREE_PREFIX}{}{node_key}", ui::TREE_WINDOW_PATH_SEPARATOR)
}

/// 🪟️ Law (a): the section AND the nested group stamp their FULL extent, materialise at most one
/// viewport between them, and never grow a `+N` continuation row.
#[test]
fn oversized_document_stamps_totals_and_never_a_continuation_row() {
    let json = window_body(&oversized_document(300, 40), Vec::new());
    assert!(json.contains("\"total\":305"), "the section stamps three add rows, flattening, export and every layer: {json}");
    assert!(json.contains("\"total\":40"), "the nested group stamps its own full extent: {json}");
    assert!(!json.contains(".more"), "no continuation row survives: {json}");
    assert!(!json.contains("\"+"), "no `+N` label survives: {json}");
    assert!(json.matches("\"granularity\":\"layer\"").count() <= TREE_WINDOW_DEFAULT_ROWS as usize, "first paint materialises about one viewport: {json}");
}

/// 🪟️ Law (b): a container the host closed stamps its total and materialises nothing — at both levels.
#[test]
fn closed_containers_stamp_totals_and_materialise_no_children() {
    let document = oversized_document(300, 40);
    let json = window_body(&document, vec![closed(RASTER_TREE_PREFIX)]);
    assert!(json.contains("\"total\":305"), "a closed section still stamps its extent: {json}");
    assert!(!json.contains("raster-play-layers.add.pixel"), "a closed section materialises no rows: {json}");

    let nested = window_body(&document, vec![open(RASTER_TREE_PREFIX, 0, 6), closed(&nested_key("group-0"))]);
    assert!(nested.contains("\"total\":40"), "a closed group still stamps its extent: {nested}");
    assert!(!nested.contains("nested-0"), "a closed group materialises no children: {nested}");
}

/// 🪟️ Law (c): a host window materialises exactly `[offset, offset + rows)`, keyed by the raw row id —
/// asserted on the section and, independently, on the nested group container.
#[test]
fn host_windows_materialise_exactly_their_slice() {
    let document = oversized_document(300, 40);
    let json = window_body(&document, vec![open(RASTER_TREE_PREFIX, 105, 10)]);
    assert!(json.contains("\"offset\":105"), "the section reports its offset: {json}");
    for index in 100..110 {
        assert!(json.contains(&format!("pixel-{index}\"")), "row {index} is inside the window: {json}");
    }
    assert!(!json.contains("pixel-99\""), "the row before the window stays out: {json}");
    assert!(!json.contains("pixel-110\""), "the row after the window stays out: {json}");

    let nested = window_body(&document, vec![open(RASTER_TREE_PREFIX, 5, 1), open(&nested_key("group-0"), 12, 4)]);
    assert!(nested.contains("\"offset\":12"), "the nested group reports its offset: {nested}");
    for index in 12..16 {
        assert!(nested.contains(&format!("nested-{index}\"")), "nested child {index} is inside the window: {nested}");
    }
    assert!(!nested.contains("nested-11\""), "the nested child before the window stays out: {nested}");
    assert!(!nested.contains("nested-16\""), "the nested child after the window stays out: {nested}");
}

/// 🪟️ Law (d): the tree carries exactly ONE `interactionSelect` binding for the `"layers"` domain and
/// every layer row — nested rows included — is a pick target through its `granularity` alone, while the
/// fixed "add" rows keep their own `addLayer` action.
#[test]
fn domain_bound_rows_carry_granularity_and_one_tree_binding() {
    let json = window_body(&oversized_document(3, 2), Vec::new());
    assert!(json.contains("\"interactionDomain\":\"layers\""), "the tree binds the layers domain: {json}");
    assert_eq!(json.matches("interactionSelect").count(), 1, "exactly one tree-level pick binding exists: {json}");
    assert_eq!(json.matches("\"granularity\":\"layer\"").count(), 5, "every layer row, nested ones included, is a pick target: {json}");
    assert!(json.contains("addLayer"), "the add rows keep their own action: {json}");
}
//#endregion 🪟️WindowLaws

#[test]
fn flatten_control_uses_the_current_language_and_artifact_command() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🎮️commands/🥞️flatten-layers/🧫️fixtures/🔣️.json")).unwrap();
    for (locale,labels) in [("en",&RasterPlayLabels::NATIVE_EN),("de",&RasterPlayLabels::NATIVE_DE)] {
        let document=oversized_document(1,1);
        let view=ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
        let node=render(&document,&RasterConfig::default(),labels,&TreeWindows::for_body(&view,RASTER_PLAY_BODY_LAYERS)).unwrap();
        let json=semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).unwrap();
        assert!(json.contains(fixture["labels"][locale]["action"].as_str().unwrap()));
        assert!(json.contains(fixture["labels"][locale]["name"].as_str().unwrap()));
        assert!(json.contains("flattenLayers"));
    }
}

#[test]
fn protected_tree_rows_remain_selectable_and_disable_structural_actions() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🗺️surface/🎨️paint/🧫️fixtures/🔒️protection/🔣️.json")).unwrap();
    let mut document=crate::standards::v1::subsets::any::io::text::snapshot::empty_raster_snapshot();document.layers=semio_framework_pack_json::from_json_str(&fixture["layers"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let view=ViewModel {tree_windows:vec![open(RASTER_TREE_PREFIX,0,20),open(&nested_key("container"),0,20),open(&nested_key("locked-group"),0,20)],..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)};
    for labels in [&RasterPlayLabels::NATIVE_EN,&RasterPlayLabels::NATIVE_DE] {
        let tree=render(&document,&RasterConfig::default(),labels,&TreeWindows::for_body(&view,RASTER_PLAY_BODY_LAYERS)).unwrap();
        let mut pending=vec![&tree];let mut seen=0;
        while let Some(node)=pending.pop() {
            if node.key.as_str()==format!("{RASTER_TREE_PREFIX}.flatten") {assert!(node.disabled);}
            if let Some(case)=fixture["cases"].as_array().unwrap().iter().find(|case|case["id"].as_str()==Some(node.key.as_str())) {
                assert!(!node.disabled);
                let semio_framework_plugin::Component::TreeItem(props)=&node.component else {panic!("layer row")};
                assert_eq!(props.draggable,Some(case["expected"]["structural"].as_bool().unwrap()));seen+=1;
            }
            pending.extend(node.children.iter());
        }
        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(tree)).unwrap();
        assert_eq!(seen,fixture["cases"].as_array().unwrap().len());
    }
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(document);
}

#[test]
fn layer_creation_controls_include_nondestructive_adjustments_in_both_languages() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/➕️layer-creation/🔣️.json")).unwrap();
    for (locale,labels) in [("en",&RasterPlayLabels::NATIVE_EN),("de",&RasterPlayLabels::NATIVE_DE)] {
        let document=crate::standards::v1::subsets::any::io::text::snapshot::empty_raster_snapshot();let view=ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
        let tree=render(&document,&RasterConfig::default(),labels,&TreeWindows::for_body(&view,RASTER_PLAY_BODY_LAYERS)).unwrap();
        let json=semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(tree)).unwrap();
        let projection:serde_json::Value=serde_json::from_str(&json).unwrap();let mut pending=vec![&projection];let mut found=0;
        while let Some(node)=pending.pop() {
            for action in fixture["actions"].as_array().unwrap() {
                if node["key"]==format!("{RASTER_TREE_PREFIX}.add.{}",action["kind"].as_str().unwrap()) {
                    let row=node.to_string();assert!(row.contains(action[locale].as_str().unwrap()));assert_eq!(node["component"]["target"]["activation"],fixture["command"]);assert_eq!(node["component"]["target"]["args"]["kind"],action["kind"]);found+=1;
                }
            }
            pending.extend(node["children"].as_array().unwrap());
        }
        assert_eq!(found,fixture["actions"].as_array().unwrap().len());
        crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(document);
    }
}

#[test]
fn png_download_control_is_localized_and_not_disabled_by_layer_locks() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../📤️export/🧫️fixtures/🔣️.json")).unwrap();
    for (locale,labels) in [("en",&RasterPlayLabels::NATIVE_EN),("de",&RasterPlayLabels::NATIVE_DE)] {
        let mut document=oversized_document(1,1);if let RasterLayerNode::Group {locked,..}=&mut document.layers[0]{*locked=true;}
        let view=ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);let node=render(&document,&RasterConfig::default(),labels,&TreeWindows::for_body(&view,RASTER_PLAY_BODY_LAYERS)).unwrap();
        let mut pending=vec![&node];let mut enabled=false;while let Some(item)=pending.pop(){if item.key.as_str()==format!("{RASTER_TREE_PREFIX}.export-png"){enabled=!item.disabled;}pending.extend(item.children.iter());}assert!(enabled);
        let json=semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).unwrap();assert!(json.contains(fixture["download"][locale].as_str().unwrap()));assert!(json.contains("exportPng"));
    }
}
