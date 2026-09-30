use super::*;
use ui_contract::{SurfaceId, UiDocumentLeaseHeader, UiNodeId, UiRevision};
use ui_wgpu::wgpu::reconcile::{UiDocumentReconcileCursor, UiDocumentReconcileStep};
use ui_wgpu::wgpu::tree::{UiDocumentTree, UiTree};

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🎬️presented-media-slots/🔣️.json")).unwrap()
}

fn props() -> serde_json::Value {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/🎬️media-transport-reservation/🔣️.json")).unwrap();
    fixture["cases"][0]["props"].clone()
}

fn owner() -> PresentedMediaOwner {
    PresentedMediaOwner { plugin_id: "media".into(), controller_id: "video.viewer".into(), app_instance_id: 17, parent_document_id: "document-7".into() }
}

fn tree(props: serde_json::Value, vector: &serde_json::Value) -> UiTree {
    let rect = &vector["localRect"];
    let parent = vector.get("parentRect");
    let generation = vector.get("documentGeneration").and_then(serde_json::Value::as_u64).unwrap_or(3);
    let mut document = UiDocumentTree::new(UiDocumentLeaseHeader { generation, surface: SurfaceId::try_from("media.video.viewer").unwrap(), revision: UiRevision(7), root: UiNodeId(if parent.is_some() { 0 } else { 1 }), layout_epoch: 1, node_count: if parent.is_some() { 2 } else { 1 } }).unwrap();
    let record = serde_json::from_value(serde_json::json!({ "id":1, "key":vector.get("nodeKey").and_then(serde_json::Value::as_str).unwrap_or("media-slot"), "component": { "type":"extension", "extension":"framework.media.transport@1", "props":props }, "layout": { "kind":"leaf", "width":"fill", "height":"hug" }, "style":{}, "activity":"idle", "accessibility":{}, "children":[] })).unwrap();
    document.try_upsert_record(record).unwrap();
    if parent.is_some() {
        let record = serde_json::from_value(serde_json::json!({ "id":0, "key":"viewport", "component": { "type":"container", "role":"plain" }, "layout": { "kind":"stack", "axis":"vertical", "gap":"none", "padding":{"all":"none"}, "align":"stretch", "justify":"start", "grow":true, "wrap":false }, "style":{}, "activity":"idle", "accessibility":{}, "children":[1] })).unwrap();
        document.try_upsert_record(record).unwrap();
    }
    let mut tree = UiTree::new();
    tree.publish_document(document);
    let mut cursor = UiDocumentReconcileCursor::default();
    cursor.rearm(generation);
    let mut complete = false;
    for _ in 0..4096 {
        match tree.step_document_reconcile(&mut cursor, "media.video.viewer", "video.viewer") {
            UiDocumentReconcileStep::Pending => {}
            UiDocumentReconcileStep::Complete => { complete = true; break; }
            step => panic!("media reconcile: {step:?}"),
        }
    }
    assert!(complete);
    let root = tree.root.unwrap();
    let media = if let Some(parent) = parent {
        let media = tree.children(root).next().unwrap();
        let node = tree.node_mut(root).unwrap();
        node.layout.x = parent["x"].as_f64().unwrap() as f32;
        node.layout.y = parent["y"].as_f64().unwrap() as f32;
        node.layout.width = parent["width"].as_f64().unwrap() as f32;
        node.layout.height = parent["height"].as_f64().unwrap() as f32;
        node.flags.set(ui_wgpu::wgpu::tree::NodeFlags::SCROLLABLE, true);
        node.flags.set(ui_wgpu::wgpu::tree::NodeFlags::CLIPS_CHILDREN, true);
        node.state.scroll_offset = (vector["scrollOffset"]["x"].as_f64().unwrap() as f32, vector["scrollOffset"]["y"].as_f64().unwrap() as f32);
        media
    } else { root };
    let node = tree.node_mut(media).unwrap();
    node.layout.x = rect["x"].as_f64().unwrap() as f32;
    node.layout.y = rect["y"].as_f64().unwrap() as f32;
    node.layout.width = rect["width"].as_f64().unwrap() as f32;
    node.layout.height = rect["height"].as_f64().unwrap() as f32;
    tree
}

fn retire(mut tree: UiTree) {
    if let Some(mut document) = tree.take_document() {
        while !document.close_step() {}
    }
}

#[test]
fn media_slot_fixture_composes_body_clip_and_exact_owner() {
    let fixture = fixture();
    for vector in fixture["cases"].as_array().unwrap() {
        let mut props = props();
        if let Some(resource) = vector.get("resourceOverride") {
            for (key, value) in resource.as_object().unwrap() {
                props["resource"][key] = value.clone();
            }
        }
        let tree = tree(props, vector);
        let body = &vector["body"];
        let body = Rect::new(body["x"].as_f64().unwrap() as f32, body["y"].as_f64().unwrap() as f32, body["width"].as_f64().unwrap() as f32, body["height"].as_f64().unwrap() as f32);
        let mut slots = Vec::new();
        assert!(collect_tree_slots(&tree, "media.video.viewer", body, &owner(), &mut slots));
        assert_eq!(slots.len(), vector["expectedCount"].as_u64().unwrap() as usize, "{}", vector["name"]);
        if let Some(slot) = slots.first() {
            assert_eq!(slot.rect, serde_json::from_value::<PresentedMediaRect>(vector["expectedRect"].clone()).unwrap());
            assert_eq!(slot.clip, serde_json::from_value::<PresentedMediaRect>(vector["expectedClip"].clone()).unwrap());
            assert_eq!(slot.occluded, vector["expectedOccluded"].as_bool().unwrap());
            assert_eq!(slot.node_id, "1");
            assert_eq!(slot.node_key, "media-slot");
            assert_eq!(slot.token, fixture["token"].as_str().unwrap());
        }
        retire(tree);
    }
}

#[test]
fn media_slot_caps_refuse_the_entire_overflow_publication() {
    let fixture = fixture();
    let tree = tree(props(), &fixture["cases"][0]);
    let mut slots = Vec::new();
    let body = Rect::new(0.0, 0.0, 500.0, 500.0);
    for _ in 0..PRESENTED_MEDIA_SLOT_CAPACITY {
        assert!(collect_tree_slots(&tree, "media.video.viewer", body, &owner(), &mut slots));
    }
    assert!(!collect_tree_slots(&tree, "media.video.viewer", body, &owner(), &mut slots));
    retire(tree);
}

#[test]
fn media_slot_descriptor_byte_budget_refuses_oversized_publication() {
    let fixture = fixture();
    let mut props = props();
    let budget = &fixture["descriptorBudget"];
    for label in props["labels"].as_object_mut().unwrap().values_mut() {
        *label = serde_json::Value::String("x".repeat(budget["labelBytes"].as_u64().unwrap() as usize));
    }
    let mut vector = fixture["cases"][0].clone();
    vector["nodeKey"] = serde_json::Value::String("x".repeat(budget["nodeKeyBytes"].as_u64().unwrap() as usize));
    let tree = tree(props, &vector);
    let mut slots = Vec::new();
    let body = Rect::new(0.0, 0.0, 500.0, 500.0);
    for _ in 0..budget["expectedAcceptedSlots"].as_u64().unwrap() {
        assert!(collect_tree_slots(&tree, "media.video.viewer", body, &owner(), &mut slots));
    }
    assert!(serde_json::to_vec(&slots).unwrap().len() <= PRESENTED_MEDIA_DESCRIPTOR_BYTES);
    assert!(!collect_tree_slots(&tree, "media.video.viewer", body, &owner(), &mut slots));
    assert!(serde_json::to_vec(&slots).unwrap().len() > PRESENTED_MEDIA_DESCRIPTOR_BYTES);
    assert!(slots.len() < PRESENTED_MEDIA_SLOT_CAPACITY);
    retire(tree);
}


#[test]
fn media_slot_occlusion_vectors_include_foreground_window_chrome() {
    let rect = |value: &serde_json::Value| Rect::new(value["x"].as_f64().unwrap() as f32, value["y"].as_f64().unwrap() as f32, value["width"].as_f64().unwrap() as f32, value["height"].as_f64().unwrap() as f32);
    for vector in fixture()["occlusionCases"].as_array().unwrap() {
        let frame = body_occluder(rect(&vector["body"]), vector["frames"].as_array().unwrap().iter().map(rect));
        assert_eq!(rects_overlap(rect(&vector["media"]), frame), vector["occluded"].as_bool().unwrap(), "{}", vector["name"]);
    }
}


#[test]
fn media_slot_concealment_preserves_the_accepted_transport_identity() {
    let fixture = fixture();
    let vectors = fixture["cases"].as_array().unwrap();
    let visible = vectors.iter().find(|vector| vector["expectedCount"] == 1 && vector["expectedOccluded"] == false).unwrap();
    let hidden = vectors.iter().find(|vector| vector["expectedOccluded"] == true).unwrap();
    let mut tokens = Vec::new();
    for vector in [visible, hidden, visible] {
        let tree = tree(props(), vector);
        let body = &vector["body"];
        let body = Rect::new(body["x"].as_f64().unwrap() as f32, body["y"].as_f64().unwrap() as f32, body["width"].as_f64().unwrap() as f32, body["height"].as_f64().unwrap() as f32);
        let mut slots = Vec::new();
        assert!(collect_tree_slots(&tree, "media.video.viewer", body, &owner(), &mut slots));
        assert_eq!(slots.len(), 1);
        assert_eq!(slots[0].occluded, vector["expectedOccluded"].as_bool().unwrap());
        tokens.push(slots[0].token.clone());
        retire(tree);
    }
    assert!(tokens.windows(2).all(|pair| pair[0] == pair[1]));
}


#[test]
fn media_slot_identity_registry_requeries_none_and_rejects_retired_or_foreign_replies() {
    let mut registry = DocumentIdentityRegistry::default();
    let key = ("media".to_string(), 17);
    for row in fixture()["identityTransitions"].as_array().unwrap() {
        match row["operation"].as_str().unwrap() {
            "begin" => assert_eq!(registry.begin(key.clone()), row["request"].as_u64()),
            "accept" => assert_eq!(registry.accept(&key, row["request"].as_u64().unwrap(), row["instance"].as_u64().unwrap() as u32, row["replyDocument"].as_str().map(ToOwned::to_owned)), row["accepted"].as_bool().unwrap()),
            "retire" => registry.remove(&key),
            operation => panic!("unknown identity fixture operation {operation}"),
        }
        assert_eq!(registry.document(&key), row["document"].as_str());
    }
}


#[test]
fn media_slot_tokens_survive_ui_generation_but_change_with_document_and_resource_authority() {
    let fixture = fixture();
    let vector = &fixture["cases"][0];
    let collect = |props, owner: &PresentedMediaOwner, vector: &serde_json::Value| {
        let tree = tree(props, vector);
        let mut slots = Vec::new();
        assert!(collect_tree_slots(&tree, "media.video.viewer", Rect::new(0.0, 0.0, 500.0, 500.0), owner, &mut slots));
        let token = slots[0].token.clone();
        retire(tree);
        token
    };
    let initial = collect(props(), &owner(), vector);
    let mut rerender = vector.clone();
    rerender["documentGeneration"] = 99.into();
    rerender["localRect"]["x"] = 23.into();
    assert_eq!(initial, collect(props(), &owner(), &rerender));
    let mut resource = props();
    resource["resource"]["generation"] = "4".into();
    assert_ne!(initial, collect(resource, &owner(), vector));
    let mut resource = props();
    resource["resource"]["revision"] = "8".into();
    resource["revision"] = "8".into();
    assert_ne!(initial, collect(resource, &owner(), vector));
    let mut resource = props();
    resource["resource"]["parentDocumentId"] = "document-B".into();
    let mut new_owner = owner();
    new_owner.parent_document_id = "document-B".into();
    assert_ne!(initial, collect(resource, &new_owner, vector));
}
