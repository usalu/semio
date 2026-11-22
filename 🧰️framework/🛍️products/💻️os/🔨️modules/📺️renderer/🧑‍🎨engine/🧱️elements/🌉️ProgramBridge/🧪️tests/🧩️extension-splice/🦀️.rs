//! 🧩️ LAW: an extension's records join its parent's as ONE subtree under the extension node — ids shifted past the parent's, the child root its only child, every ceiling and every refusal leaving the parent untouched — and extensions nest to a bounded depth. Pure over records: no program, no JS guest.

use super::*;
use std::future::Future;
use std::task::{Context, Poll, Waker};
use ui_contract::{UiNodeId, UiNodeRecord};

fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = Box::pin(future);
    let mut cx = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(value) = future.as_mut().poll(&mut cx) {
            return value;
        }
    }
}

fn record(value: serde_json::Value) -> UiNodeRecord {
    let mut base = serde_json::json!({ "layout": { "kind": "leaf", "width": "fill", "height": "hug" }, "style": {}, "activity": "idle", "accessibility": {} });
    base.as_object_mut().unwrap().extend(value.as_object().unwrap().clone());
    serde_json::from_value(base).expect("record deserializes against the contract")
}

fn extension(id: u64, name: &str) -> UiNodeRecord {
    record(serde_json::json!({ "id": id, "key": format!("ext{id}"), "component": { "type": "extension", "extension": name, "props": { "id": "w" } } }))
}

fn text(id: u64, value: &str) -> UiNodeRecord {
    record(serde_json::json!({ "id": id, "key": format!("t{id}"), "component": { "type": "text", "value": value } }))
}

fn stack(id: u64, children: &[u64]) -> UiNodeRecord {
    record(serde_json::json!({ "id": id, "key": format!("s{id}"), "component": { "type": "container", "role": "plain" }, "children": children }))
}

fn parent() -> Vec<UiNodeRecord> {
    vec![stack(7, &[9]), extension(9, "law.outer@1")]
}

fn copy(nodes: &[UiNodeRecord]) -> Vec<UiNodeRecord> {
    nodes.iter().map(|node| node.credited_clone().expect("a small record credits")).collect()
}

fn child_ids(record: &UiNodeRecord) -> Vec<u64> {
    record.children.iter().map(|id| id.0).collect()
}

#[test]
fn a_child_document_joins_as_the_one_subtree_of_its_extension_node_with_shifted_ids() {
    let mut nodes = parent();
    assert_eq!(extension_hosts(&nodes, 0), vec![(1, "law.outer@1".to_string(), r#"{"id":"w"}"#.to_string())]);
    assert!(splice_extension_records(&mut nodes, 1, vec![stack(1, &[2, 3]), text(2, "a"), text(3, "b")], 1));
    assert_eq!(nodes.len(), 5);
    assert_eq!(child_ids(&nodes[1]), [11], "the child root, shifted past the parent's greatest id (9) by 10");
    assert_eq!(nodes[2..].iter().map(|node| node.id.0).collect::<Vec<_>>(), [11, 12, 13]);
    assert_eq!(child_ids(&nodes[2]), [12, 13], "children shift with their parent");
    assert!(extension_hosts(&nodes, 0).is_empty(), "a hosted extension is no longer a childless host");
}

#[test]
fn a_refused_splice_leaves_the_parent_untouched() {
    let mut nodes = parent();
    let before = copy(&nodes);
    assert!(!splice_extension_records(&mut nodes, 0, vec![text(1, "a")], 1), "a container is no extension");
    assert!(!splice_extension_records(&mut nodes, 1, Vec::new(), 1), "an empty document hosts nothing");
    assert!(!splice_extension_records(&mut nodes, 1, vec![text(1, "a")], 2), "the root must be among the records");
    let oversized: Vec<UiNodeRecord> = (1..=ui_contract::UI_DOCUMENT_NODES as u64).map(|id| text(id, "x")).collect();
    assert!(!splice_extension_records(&mut nodes, 1, oversized, 1), "the document ceiling holds");
    assert_eq!(nodes, before);
    assert!(splice_extension_records(&mut nodes, 1, vec![text(1, "a")], 1));
    assert!(!splice_extension_records(&mut nodes, 1, vec![text(1, "b")], 1), "an extension takes one subtree");
}

#[test]
fn extensions_inside_extensions_splice_to_a_bounded_depth() {
    let mut nodes = parent();
    block_on(splice_extension_documents(&mut nodes, |extension, _| async move {
        match extension.as_str() {
            "law.outer@1" => Some((vec![stack(1, &[2]), extension_node(2, "law.inner@1")], 1)),
            "law.inner@1" => Some((vec![text(1, "deep")], 1)),
            _ => None,
        }
    }));
    assert_eq!(nodes.len(), 5, "outer subtree, then the extension inside it");
    assert!(nodes.iter().any(|node| matches!(&node.component, ui_contract::Component::Text(_))), "the inner extension rendered its text");
    assert!(extension_hosts(&nodes, 0).is_empty());

    let mut looping = vec![extension(0, "law.self@1")];
    block_on(splice_extension_documents(&mut looping, |_, _| async { Some((vec![extension_node(1, "law.self@1")], 1)) }));
    assert_eq!(looping.len(), 1 + EXTENSION_SPLICE_DEPTH, "a self-hosting extension stops at the depth bound");
    assert_eq!(extension_hosts(&looping, 0).len(), 1, "the deepest one stays a childless node");
}

fn extension_node(id: u64, name: &str) -> UiNodeRecord {
    extension(id, name)
}

#[test]
fn an_extension_without_a_host_stays_a_childless_node() {
    let mut nodes = parent();
    let before = copy(&nodes);
    block_on(splice_extension_documents(&mut nodes, |_, _| async { None }));
    assert_eq!(nodes, before);
    let _ = UiNodeId(0);
}
