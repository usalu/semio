//! ♿️ LAW: closed retained disclosures remove descendants from the projected accessibility tree.

use super::*;
use crate::wgpu::reconcile::{UiDocumentReconcileCursor, UiDocumentReconcileStep};
use crate::wgpu::tree::UiDocumentTree;
use ui_contract::{SurfaceId, UiDocumentLeaseHeader, UiNodeId, UiNodeRecord, UiRevision};

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/📂️retained-section-collapse/🔣️.json")).expect("retained Section fixture")
}

fn mounted_disclosure(fixture: &serde_json::Value) -> UiTree {
    let section = &fixture["section"];
    let records = [
        serde_json::json!({
            "id": 0,
            "key": section["id"],
            "component": { "type": "container", "role": "section", "label": section["label"], "defaultOpen": section["defaultOpen"] },
            "layout": { "kind": "leaf", "width": "hug", "height": "hug" },
            "style": {},
            "activity": "idle",
            "accessibility": { "label": section["label"] },
            "children": [1]
        }),
        serde_json::json!({
            "id": 1,
            "key": section["childId"],
            "component": { "type": "button", "icon": "circle-dot", "label": section["childLabel"] },
            "layout": { "kind": "leaf", "width": "hug", "height": "hug" },
            "style": {},
            "activity": "idle",
            "accessibility": { "label": section["childLabel"] },
            "bindings": [{ "trigger": "activate", "action": { "scope": "fixture", "name": "activateChild", "version": 1 } }]
        }),
    ];
    let header = UiDocumentLeaseHeader { generation: 1, surface: SurfaceId::try_from("fixture").expect("surface id"), revision: UiRevision(1), root: UiNodeId(0), layout_epoch: 0, node_count: records.len() };
    let mut document = UiDocumentTree::new(header).expect("document header");
    for record in records {
        document.try_upsert_record(serde_json::from_value::<UiNodeRecord>(record).expect("document record")).expect("record admits");
    }
    let mut tree = UiTree::new();
    tree.publish_document(document);
    let mut cursor = UiDocumentReconcileCursor::default();
    cursor.rearm(1);
    for _ in 0..64 {
        match tree.step_document_reconcile(&mut cursor, "fixture", "fixture") {
            UiDocumentReconcileStep::Pending => {}
            UiDocumentReconcileStep::Complete => return tree,
            UiDocumentReconcileStep::Fault(fault) => panic!("disclosure document faulted: {fault:?}"),
        }
    }
    panic!("disclosure document did not reconcile");
}

#[test]
fn accessibility_reaches_a_disclosure_child_only_while_open() {
    let fixture = fixture();
    let mut tree = mounted_disclosure(&fixture);
    let closed = accessibility_projection(&tree);
    assert_eq!(closed.len(), 1, "closed accessibility projection omits descendants");
    assert_eq!(closed[0].expanded, Some(fixture["states"]["closed"]["expanded"].as_bool().unwrap()));
    let root = tree.root.expect("mounted root");
    assert_eq!(tree.toggle_disclosure(root), Some(true));
    let open = accessibility_projection(&tree);
    assert_eq!(open.len(), 2, "opening restores the child to accessibility reading order");
    assert_eq!(open[0].expanded, Some(fixture["states"]["open"]["expanded"].as_bool().unwrap()));
    assert_eq!(open[1].key, fixture["section"]["childId"].as_str().unwrap());
}
