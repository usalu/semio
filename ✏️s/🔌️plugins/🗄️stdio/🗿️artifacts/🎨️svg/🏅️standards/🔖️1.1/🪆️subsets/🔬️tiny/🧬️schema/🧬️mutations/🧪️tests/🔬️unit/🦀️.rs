use super::*;

fn elem(name: &str, attrs: Vec<(&str, SvgAttributeValue)>, children: Vec<SvgNode>) -> SvgNode {
    SvgNode::Element { name: name.into(), attrs: attrs.into_iter().map(|(n, v)| SvgAttr { name: n.into(), value: v }).collect(), children }
}

fn document(root: SvgNode) -> SvgSnapshot {
    let mut snapshot = SvgSnapshot::default();
    snapshot.doc.root = Some(root);
    snapshot
}

/// 📇️ The one test that keeps `KINDS` honest against the enum it claims to spell. The framework
/// never parses Rust, so this is the only thing standing between a renamed variant and a catalog
/// that silently measures the wrong vocabulary.
#[test]
fn kinds_matches_enum_variants_and_manifest() {
    let every = vec![
        SvgTinyMutation::StampBaseProfile(stamp_base_profile::StampBaseProfile { base_profile: None, version: None, base_profile_index: None, version_index: None }),
        SvgTinyMutation::InsertTinyElement(insert_tiny_element::InsertTinyElement { parent: Vec::new(), index: 0, node: elem("rect", vec![], vec![]) }),
        SvgTinyMutation::RemoveElement(remove_element::RemoveElement { parent: Vec::new(), index: 0 }),
        SvgTinyMutation::SetTinyAttribute(set_tiny_attribute::SetTinyAttribute { path: Vec::new(), name: "fill".into(), value: None, index: None }),
        SvgTinyMutation::SetText(set_text::SetText { path: Vec::new(), text: String::new() }),
        SvgTinyMutation::SetViewBox(set_view_box::SetViewBox { path: Vec::new(), view_box: None, index: None }),
        SvgTinyMutation::SetTransform(set_transform::SetTransform { path: Vec::new(), transform: None, index: None }),
        SvgTinyMutation::StripNonTiny(strip_non_tiny::StripNonTiny {}),
        SvgTinyMutation::RestoreNonTiny(restore_non_tiny::RestoreNonTiny { elements: Vec::new(), attributes: Vec::new() }),
    ];
    let spelled: Vec<&'static str> = every.iter().map(kind_of).collect();
    assert_eq!(spelled, KINDS.to_vec(), "KINDS must spell every variant, in declaration order");

    let manifest = include_str!("../../../../🔮️oracles/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "the oracle manifest's catalog does not declare {kind:?}");
    }
}

/// 🔗 Holds this module's gate lists against the subset's own conformance checker, so the two
/// statements of SVG Tiny 1.1's excluded vocabulary cannot drift apart.
#[test]
fn blocklists_agree_with_the_subset_conformance_checker() {
    use crate::standards::v1_1::subsets::tiny::schema::conformance::check_svg_tiny_conformance;
    let hard = |snapshot: &SvgSnapshot| check_svg_tiny_conformance(snapshot).into_iter().any(|d| matches!(d.severity, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal));
    let root = |children: Vec<SvgNode>| document(elem("svg", vec![("baseProfile", crate::schema::snapshot::SvgAttributeValue::Text("tiny".into())), ("version", crate::schema::snapshot::SvgAttributeValue::Text("1.1".into()))], children));
    let excluded_elements: Vec<&str> = BLOCKED_ELEMENTS.iter().copied().chain(["feGaussianBlur"]).collect();
    for name in excluded_elements {
        assert!(is_blocked_element(name), "{name} must be gated by this module");
        assert!(hard(&root(vec![elem(name, vec![], vec![])])), "the subset conformance checker does not reject <{name}>");
    }
    for name in BLOCKED_ATTRS.iter().copied() {
        assert!(is_blocked_attribute(name), "{name} must be gated by this module");
        assert!(hard(&root(vec![elem("rect", vec![(name, SvgAttributeValue::Text("x".into()))], vec![])])), "the subset conformance checker does not reject the '{name}' attribute");
    }
    for name in ["rect", "circle", "path", "g", "defs", "image", "svg"] {
        assert!(!is_blocked_element(name), "{name} is retained by SVG Tiny 1.1");
        assert!(!hard(&root(vec![elem(name, vec![], vec![])])), "the subset conformance checker rejects the retained <{name}>");
    }
}

#[test]
fn insert_tiny_element_rejects_an_excluded_subtree() {
    let mut snapshot = document(elem("svg", vec![], vec![]));
    let outcome = apply_svg_tiny_mutation(&mut snapshot, &SvgTinyMutation::InsertTinyElement(insert_tiny_element::InsertTinyElement { parent: Vec::new(), index: 0, node: elem("filter", vec![], vec![elem("feTurbulence", vec![], vec![])]) }));
    assert!(!outcome.messages().is_empty(), "a filter subtree must be rejected, not inserted");
    assert!(matches!(&snapshot.doc.root, Some(SvgNode::Element { children, .. }) if children.is_empty()), "the document must be untouched");
}

#[test]
fn set_tiny_attribute_rejects_a_forbidden_presentation_attribute() {
    let mut snapshot = document(elem("svg", vec![], vec![]));
    let outcome = apply_svg_tiny_mutation(&mut snapshot, &SvgTinyMutation::SetTinyAttribute(set_tiny_attribute::SetTinyAttribute { path: Vec::new(), name: "opacity".into(), value: Some(SvgAttributeValue::Text("0.5".into())), index: None }));
    assert!(!outcome.messages().is_empty(), "opacity is forbidden anywhere in SVG Tiny 1.1");
    assert!(matches!(&snapshot.doc.root, Some(SvgNode::Element { attrs, .. }) if attrs.is_empty()), "the document must be untouched");
}

#[test]
fn strip_non_tiny_removes_excluded_elements_and_attributes() {
    let mut snapshot = document(elem("svg", vec![], vec![elem("g", vec![("style", crate::schema::snapshot::SvgAttributeValue::Text("fill:#000".into()))], vec![elem("rect", vec![], vec![])]), elem("linearGradient", vec![("id", crate::schema::snapshot::SvgAttributeValue::Text("g1".into()))], vec![])]));
    apply_svg_tiny_mutation(&mut snapshot, &SvgTinyMutation::StripNonTiny(strip_non_tiny::StripNonTiny {}));
    match &snapshot.doc.root {
        Some(SvgNode::Element { children, .. }) => {
            assert_eq!(children.len(), 1, "the excluded <linearGradient> must be gone");
            assert!(matches!(&children[0], SvgNode::Element { attrs, .. } if attrs.is_empty()), "the forbidden style attribute must be gone");
        }
        other => panic!("expected an element root, got {other:?}"),
    }
}

#[test]
fn strip_non_tiny_is_invertible_through_its_own_inverse() {
    let base = document(elem("svg", vec![], vec![elem("g", vec![("style", crate::schema::snapshot::SvgAttributeValue::Text("fill:#000".into()))], vec![])]));
    let mut snapshot = base.clone();
    let mutation = SvgTinyMutation::StripNonTiny(strip_non_tiny::StripNonTiny {});
    let undo = Mutation::inverse(&mutation, &base).expect("valid retained mutation inverse fixture");
    apply_svg_tiny_mutation(&mut snapshot, &mutation);
    for step in &undo {
        apply_svg_tiny_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "strip-non-tiny followed by its own inverse must restore the document");
}

#[test]
fn stamp_base_profile_is_invertible_when_the_root_declared_neither_attribute() {
    let base = document(elem("svg", vec![("id", crate::schema::snapshot::SvgAttributeValue::Text("Layer_1".into()))], vec![]));
    let mut snapshot = base.clone();
    let mutation = SvgTinyMutation::StampBaseProfile(stamp_base_profile::StampBaseProfile { base_profile: Some("tiny".into()), version: Some("1.1".into()), base_profile_index: None, version_index: None });
    let undo = Mutation::inverse(&mutation, &base).expect("valid retained mutation inverse fixture");
    apply_svg_tiny_mutation(&mut snapshot, &mutation);
    assert_eq!(element_attr(snapshot.doc.root.as_ref().unwrap(), "baseProfile"), Some(&SvgAttributeValue::Text("tiny".into())));
    for step in &undo {
        apply_svg_tiny_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "stamping and unstamping the profile must restore the document");
}

#[semio_framework_async_macros::async_test]
async fn strip_and_restore_non_tiny_satisfy_the_inverse_sum_law() {
    let base = document(elem("svg", vec![("style", SvgAttributeValue::Text("x".into())), ("id", SvgAttributeValue::Text("a".into()))], vec![elem("g", vec![("opacity", SvgAttributeValue::Text("0.5".into()))], vec![elem("linearGradient", vec![], vec![]), elem("rect", vec![], vec![])]), elem("filter", vec![], vec![])]));
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&SvgTinyMutation::StripNonTiny(strip_non_tiny::StripNonTiny {}), &base).await;
    let stripped = document(elem("svg", vec![("id", SvgAttributeValue::Text("a".into()))], vec![elem("g", vec![], vec![elem("rect", vec![], vec![])])]));
    let restore = SvgTinyMutation::RestoreNonTiny(restore_non_tiny::RestoreNonTiny {
        elements: vec![restore_non_tiny::RestoredElement { parent: vec![0], index: 0, node: elem("linearGradient", vec![], vec![]) }, restore_non_tiny::RestoredElement { parent: Vec::new(), index: 1, node: elem("filter", vec![], vec![]) }],
        attributes: vec![restore_non_tiny::RestoredAttribute { path: Vec::new(), index: 0, name: "style".into(), value: SvgAttributeValue::Text("x".into()) }, restore_non_tiny::RestoredAttribute { path: vec![0], index: 0, name: "opacity".into(), value: SvgAttributeValue::Text("0.5".into()) }],
    });
    let mut restored = stripped.clone();
    apply_svg_tiny_mutation(&mut restored, &restore);
    assert_eq!(restored, base, "restore-non-tiny must put every row back at its position");
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&restore, &stripped).await;
}

#[semio_framework_async_macros::async_test]
async fn every_authoring_leaf_satisfies_the_inverse_sum_law() {
    let base = document(elem("svg", vec![("id", SvgAttributeValue::Text("a".into())), ("viewBox", SvgAttributeValue::ViewBox(crate::schema::snapshot::ViewBox { min_x: 0.0, min_y: 0.0, width: 5.0, height: 5.0 })), ("class", SvgAttributeValue::Text("c".into()))], vec![elem("g", vec![], vec![]), SvgNode::Text { text: "t".into() }]));
    for mutation in [
        SvgTinyMutation::StampBaseProfile(stamp_base_profile::StampBaseProfile { base_profile: Some("tiny".into()), version: Some("1.1".into()), base_profile_index: None, version_index: None }),
        SvgTinyMutation::InsertTinyElement(insert_tiny_element::InsertTinyElement { parent: Vec::new(), index: 1, node: elem("rect", vec![], vec![]) }),
        SvgTinyMutation::RemoveElement(remove_element::RemoveElement { parent: Vec::new(), index: 0 }),
        SvgTinyMutation::SetTinyAttribute(set_tiny_attribute::SetTinyAttribute { path: Vec::new(), name: "viewBox".into(), value: None, index: None }),
        SvgTinyMutation::SetText(set_text::SetText { path: vec![1], text: "u".into() }),
        SvgTinyMutation::SetViewBox(set_view_box::SetViewBox { path: Vec::new(), view_box: None, index: None }),
        SvgTinyMutation::SetTransform(set_transform::SetTransform { path: vec![0], transform: Some(vec![crate::schema::snapshot::TransformOp::Scale { x: 2.0, y: None }]), index: None }),
    ] {
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
    }
}
