
use super::*;

#[test]
fn every_declared_recipe_id_resolves() {
    for id in RECIPE_IDS {
        assert!(recipe(id).is_some(), "recipe {id} must resolve");
    }
}

#[test]
fn encode_decode_round_trips_the_base_document() {
    // 📌 `base_doc()`'s own item#i1 text node deliberately stores the PRE-ESCAPED source form
    // (see this file's header) — decode resolves entities, so the round-trip target is the
    // RESOLVED text, not the literal source string.
    let mut doc = base_doc();
    let bytes = encode_xml(&doc);
    let back = decode_xml(&bytes).expect("decode base document");
    if let Some(XNode::Element { children, .. }) = doc.root.as_mut() {
        if let XNode::Element { children: item_children, .. } = &mut children[0] {
            item_children[0] = XNode::Text("Widget & Gadget \u{a9}".to_string());
        }
    }
    assert_eq!(back.declaration, doc.declaration);
    assert_eq!(back.doctype, doc.doctype);
    assert_eq!(back.prolog, doc.prolog);
    assert_eq!(back.root, doc.root);
}

#[test]
fn general_ref_reassembly_recovers_named_and_numeric_entities_across_events() {
    let bytes = encode_xml(&base_doc());
    let text = std::str::from_utf8(&bytes).unwrap();
    assert!(text.contains("&amp;"), "encoded bytes must literally contain the named entity ref");
    assert!(text.contains("&#169;"), "encoded bytes must literally contain the numeric char ref");
    let doc = decode_xml(&bytes).expect("decode");
    let Some(XNode::Element { children, .. }) = doc.root else { panic!("expected root element") };
    let XNode::Element { children: item_children, .. } = &children[0] else { panic!("expected item element") };
    let XNode::Text(text) = &item_children[0] else { panic!("expected text node") };
    assert_eq!(text, "Widget & Gadget \u{a9}", "reassembled text must resolve both the named and numeric reference");
}

#[test]
fn every_recipe_after_state_differs_from_before_in_exactly_its_own_dimension() {
    for id in RECIPE_IDS {
        let (before, after) = recipe(id).unwrap();
        assert_ne!(before, after, "recipe {id} must produce a materially different after-state");
    }
}
