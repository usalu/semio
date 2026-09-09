use super::*;

fn elem(name: &str, attrs: Vec<(&str, &str)>, children: Vec<XmlNode>) -> XmlNode {
    XmlNode::Element { name: name.into(), attrs: attrs.into_iter().map(|(n, v)| XmlAttr { name: n.into(), value: v.into() }).collect(), children }
}

fn document(root: XmlNode) -> SvgSnapshot {
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
        SvgTinyMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: SvgSnapshot::default() }),
        SvgTinyMutation::StampBaseProfile(stamp_base_profile::StampBaseProfile { base_profile: None, version: None }),
        SvgTinyMutation::InsertTinyElement(insert_tiny_element::InsertTinyElement { parent: Vec::new(), index: 0, node: elem("rect", vec![], vec![]) }),
        SvgTinyMutation::RemoveElement(remove_element::RemoveElement { parent: Vec::new(), index: 0 }),
        SvgTinyMutation::SetTinyAttribute(set_tiny_attribute::SetTinyAttribute { path: Vec::new(), name: "fill".into(), value: None }),
        SvgTinyMutation::SetText(set_text::SetText { path: Vec::new(), text: String::new() }),
        SvgTinyMutation::SetViewBox(set_view_box::SetViewBox { path: Vec::new(), view_box: None }),
        SvgTinyMutation::SetTransform(set_transform::SetTransform { path: Vec::new(), transform: None }),
        SvgTinyMutation::StripNonTiny(strip_non_tiny::StripNonTiny {}),
    ];
    let spelled: Vec<&'static str> = every.iter().map(kind_of).collect();
    assert_eq!(spelled, KINDS.to_vec(), "KINDS must spell every variant, in declaration order");

    let manifest = include_str!("../../../../🔮️oracle/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "the oracle manifest's catalog does not declare {kind:?}");
    }
}

/// 🔗 Holds this module's gate lists against the subset's own conformance checker, so the two
/// statements of SVG Tiny 1.1's excluded vocabulary cannot drift apart.
#[test]
fn blocklists_agree_with_the_subset_conformance_checker() {
    use crate::standards::v1_1::subsets::tiny::schema::check_svg_tiny_conformance;
    let hard = |snapshot: &SvgSnapshot| check_svg_tiny_conformance(snapshot).into_iter().any(|d| matches!(d.severity, dsl::Severity::Error | dsl::Severity::Fatal));
    let root = |children: Vec<XmlNode>| document(elem("svg", vec![("baseProfile", "tiny"), ("version", "1.1")], children));
    let excluded_elements: Vec<&str> = BLOCKED_ELEMENTS.iter().copied().chain(["feGaussianBlur"]).collect();
    for name in excluded_elements {
        assert!(is_blocked_element(name), "{name} must be gated by this module");
        assert!(hard(&root(vec![elem(name, vec![], vec![])])), "the subset conformance checker does not reject <{name}>");
    }
    for name in BLOCKED_ATTRS.iter().copied() {
        assert!(is_blocked_attribute(name), "{name} must be gated by this module");
        assert!(hard(&root(vec![elem("rect", vec![(name, "x")], vec![])])), "the subset conformance checker does not reject the '{name}' attribute");
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
    assert!(matches!(&snapshot.doc.root, Some(XmlNode::Element { children, .. }) if children.is_empty()), "the document must be untouched");
}

#[test]
fn set_tiny_attribute_rejects_a_forbidden_presentation_attribute() {
    let mut snapshot = document(elem("svg", vec![], vec![]));
    let outcome = apply_svg_tiny_mutation(&mut snapshot, &SvgTinyMutation::SetTinyAttribute(set_tiny_attribute::SetTinyAttribute { path: Vec::new(), name: "opacity".into(), value: Some("0.5".into()) }));
    assert!(!outcome.messages().is_empty(), "opacity is forbidden anywhere in SVG Tiny 1.1");
    assert!(matches!(&snapshot.doc.root, Some(XmlNode::Element { attrs, .. }) if attrs.is_empty()), "the document must be untouched");
}

#[test]
fn strip_non_tiny_removes_excluded_elements_and_attributes() {
    let mut snapshot = document(elem("svg", vec![], vec![elem("g", vec![("style", "fill:#000")], vec![elem("rect", vec![], vec![])]), elem("linearGradient", vec![("id", "g1")], vec![])]));
    apply_svg_tiny_mutation(&mut snapshot, &SvgTinyMutation::StripNonTiny(strip_non_tiny::StripNonTiny {}));
    match &snapshot.doc.root {
        Some(XmlNode::Element { children, .. }) => {
            assert_eq!(children.len(), 1, "the excluded <linearGradient> must be gone");
            assert!(matches!(&children[0], XmlNode::Element { attrs, .. } if attrs.is_empty()), "the forbidden style attribute must be gone");
        }
        other => panic!("expected an element root, got {other:?}"),
    }
}

#[test]
fn strip_non_tiny_is_invertible_through_its_own_inverse() {
    let base = document(elem("svg", vec![], vec![elem("g", vec![("style", "fill:#000")], vec![])]));
    let mut snapshot = base.clone();
    let mutation = SvgTinyMutation::StripNonTiny(strip_non_tiny::StripNonTiny {});
    let undo = Mutation::inverse(&mutation, &base);
    apply_svg_tiny_mutation(&mut snapshot, &mutation);
    for step in &undo {
        apply_svg_tiny_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "strip-non-tiny followed by its own inverse must restore the document");
}

#[test]
fn stamp_base_profile_is_invertible_when_the_root_declared_neither_attribute() {
    let base = document(elem("svg", vec![("id", "Layer_1")], vec![]));
    let mut snapshot = base.clone();
    let mutation = SvgTinyMutation::StampBaseProfile(stamp_base_profile::StampBaseProfile { base_profile: Some("tiny".into()), version: Some("1.1".into()) });
    let undo = Mutation::inverse(&mutation, &base);
    apply_svg_tiny_mutation(&mut snapshot, &mutation);
    assert_eq!(element_attr(snapshot.doc.root.as_ref().unwrap(), "baseProfile"), Some("tiny"));
    for step in &undo {
        apply_svg_tiny_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "stamping and unstamping the profile must restore the document");
}
