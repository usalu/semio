
use super::*;

#[test]
fn every_declared_recipe_id_resolves() {
    for id in RECIPE_IDS {
        assert!(recipe(id).is_some(), "recipe {id} must resolve");
    }
}

#[test]
fn encode_decode_round_trips_the_base_document() {
    let doc = base_doc();
    let bytes = write_svg(&doc);
    let back = parse_svg(&bytes);
    assert_eq!(back.declaration.as_ref().unwrap().version, "1.0");
    assert_eq!(back.doctype, doc.doctype);
    match (&doc.root, &back.root) {
        (Some(QNode::Element { children: a, .. }), Some(QNode::Element { children: b, .. })) => assert_eq!(a.len(), b.len()),
        _ => panic!("root must be an element on both sides"),
    }
}

#[test]
fn each_recipe_after_differs_from_before() {
    for id in RECIPE_IDS {
        let (before, after) = recipe(id).unwrap();
        assert_ne!(write_svg(&before), write_svg(&after), "recipe {id} must actually change something");
    }
}
