
use super::*;

fn elem(name: &str, attrs: Vec<(&str, &str)>, children: Vec<XmlNode>) -> XmlNode {
    XmlNode::Element { name: name.into(), attrs: attrs.into_iter().map(|(n, v)| XmlAttr { name: n.into(), value: v.into() }).collect(), children }
}

fn document() -> SvgSnapshot {
    let mut snapshot = SvgSnapshot::default();
    snapshot.doc.root = Some(elem(
        "svg",
        vec![],
        vec![
            elem("defs", vec![], vec![elem("clipPath", vec![("id", "shape")], vec![elem("path", vec![("d", "M0 0H8V8H0Z")], vec![])]), elem("clipPath", vec![("id", "lettering")], vec![elem("text", vec![], vec![])])]),
            elem("g", vec![], vec![elem("rect", vec![], vec![])]),
        ],
    ));
    snapshot
}

/// 📇️ The one test that keeps `KINDS` honest against the enum it claims to spell.
#[test]
fn kinds_matches_enum_variants_and_manifest() {
    let every = vec![
        SvgBasicMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: SvgSnapshot::default() }),
        SvgBasicMutation::StampBaseProfile(stamp_base_profile::StampBaseProfile { base_profile: None, version: None }),
        SvgBasicMutation::InsertBasicElement(insert_basic_element::InsertBasicElement { parent: Vec::new(), index: 0, node: elem("rect", vec![], vec![]) }),
        SvgBasicMutation::RemoveElement(remove_element::RemoveElement { parent: Vec::new(), index: 0 }),
        SvgBasicMutation::SetBasicAttribute(set_basic_attribute::SetBasicAttribute { path: Vec::new(), name: "fill".into(), value: None }),
        SvgBasicMutation::SetClipPathReference(set_clip_path_reference::SetClipPathReference { path: Vec::new(), clip_path_id: None }),
        SvgBasicMutation::InsertClipPathShape(insert_clip_path_shape::InsertClipPathShape { clip_path_id: "shape".into(), index: 0, node: elem("circle", vec![], vec![]) }),
        SvgBasicMutation::SetText(set_text::SetText { path: Vec::new(), text: String::new() }),
        SvgBasicMutation::SetViewBox(set_view_box::SetViewBox { path: Vec::new(), view_box: None }),
        SvgBasicMutation::SetTransform(set_transform::SetTransform { path: Vec::new(), transform: None }),
    ];
    let spelled: Vec<&'static str> = every.iter().map(kind_of).collect();
    assert_eq!(spelled, KINDS.to_vec(), "KINDS must spell every variant, in declaration order");

    let manifest = include_str!("../../../../🔮️oracle/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "the oracle manifest's catalog does not declare {kind:?}");
    }
}

/// 🔗 Holds this module's gate lists against the subset's own conformance checker, so the two
/// statements of SVG Basic 1.1's excluded vocabulary cannot drift apart.
#[test]
fn blocklists_agree_with_the_subset_conformance_checker() {
    use crate::standards::v1_1::subsets::basic::schema::check_svg_basic_conformance;
    let hard = |snapshot: &SvgSnapshot| check_svg_basic_conformance(snapshot).into_iter().any(|d| matches!(d.severity, dsl::Severity::Error | dsl::Severity::Fatal));
    let root = |children: Vec<XmlNode>| {
        let mut snapshot = SvgSnapshot::default();
        snapshot.doc.root = Some(elem("svg", vec![("baseProfile", "basic"), ("version", "1.1")], children));
        snapshot
    };
    for name in BLOCKED_FILTER_PRIMITIVES {
        assert!(is_blocked_filter_primitive(name), "{name} must be gated by this module");
        assert!(hard(&root(vec![elem("filter", vec![("id", "f1")], vec![elem(name, vec![], vec![])])])), "the subset conformance checker does not reject <{name}>");
    }
    for name in ["feGaussianBlur", "feBlend", "feFlood", "feOffset", "linearGradient", "clipPath", "mask"] {
        assert!(!is_blocked_filter_primitive(name), "{name} is retained by SVG Basic 1.1");
        assert!(!hard(&root(vec![elem("filter", vec![("id", "f1")], vec![elem(name, vec![], vec![])])])), "the subset conformance checker rejects the retained <{name}>");
    }
    for name in TEXT_ELEMENTS {
        assert!(carries_text(&elem(name, vec![], vec![])), "{name} must count as text for this module's clip gate");
    }
    assert!(hard(&root(vec![elem("clipPath", vec![("id", "c1")], vec![elem("text", vec![], vec![])]), elem("rect", vec![("clip-path", "url(#c1)")], vec![])])), "the subset conformance checker does not reject clipping to text");
    assert!(!hard(&root(vec![elem("clipPath", vec![("id", "c1")], vec![elem("path", vec![], vec![])]), elem("rect", vec![("clip-path", "url(#c1)")], vec![])])), "the subset conformance checker rejects a shape-only clip path");
}

#[test]
fn insert_basic_element_accepts_a_retained_filter_primitive() {
    let mut snapshot = document();
    let outcome = apply_svg_basic_mutation(
        &mut snapshot,
        &SvgBasicMutation::InsertBasicElement(insert_basic_element::InsertBasicElement { parent: vec![0], index: 0, node: elem("filter", vec![("id", "blur")], vec![elem("feGaussianBlur", vec![("stdDeviation", "2")], vec![])]) }),
    );
    assert!(outcome.messages().is_empty(), "feGaussianBlur is retained by SVG Basic 1.1: {:?}", outcome.messages());
}

#[test]
fn insert_basic_element_rejects_an_excluded_primitive() {
    let mut snapshot = document();
    let before = snapshot.clone();
    let outcome = apply_svg_basic_mutation(&mut snapshot, &SvgBasicMutation::InsertBasicElement(insert_basic_element::InsertBasicElement { parent: vec![0], index: 0, node: elem("filter", vec![], vec![elem("feTurbulence", vec![], vec![])]) }));
    assert!(!outcome.messages().is_empty(), "feTurbulence is outside SVG Basic 1.1");
    assert_eq!(snapshot, before, "the document must be untouched");
}

#[test]
fn set_clip_path_reference_rejects_a_clip_path_that_clips_to_text() {
    let mut snapshot = document();
    let outcome = apply_svg_basic_mutation(&mut snapshot, &SvgBasicMutation::SetClipPathReference(set_clip_path_reference::SetClipPathReference { path: vec![1], clip_path_id: Some("lettering".into()) }));
    assert!(!outcome.messages().is_empty(), "SVG Basic 1.1 does not support clipping to text");
}

#[test]
fn set_clip_path_reference_accepts_a_shape_only_clip_path_and_inverts() {
    let base = document();
    let mut snapshot = base.clone();
    let mutation = SvgBasicMutation::SetClipPathReference(set_clip_path_reference::SetClipPathReference { path: vec![1], clip_path_id: Some("shape".into()) });
    let undo = Mutation::inverse(&mutation, &base);
    let outcome = apply_svg_basic_mutation(&mut snapshot, &mutation);
    assert!(outcome.messages().is_empty(), "a shape-only clipPath is legal: {:?}", outcome.messages());
    assert_eq!(element_attr(node_at(&snapshot.doc, &[1]).unwrap(), "clip-path"), Some("url(#shape)"));
    for step in &undo {
        apply_svg_basic_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "setting and clearing the clip-path reference must restore the document");
}

#[test]
fn insert_clip_path_shape_rejects_a_text_shape() {
    let mut snapshot = document();
    let before = snapshot.clone();
    let outcome = apply_svg_basic_mutation(&mut snapshot, &SvgBasicMutation::InsertClipPathShape(insert_clip_path_shape::InsertClipPathShape { clip_path_id: "shape".into(), index: 0, node: elem("text", vec![], vec![]) }));
    assert!(!outcome.messages().is_empty(), "adding text to a clip path would clip to text");
    assert_eq!(snapshot, before, "the document must be untouched");
}

#[test]
fn insert_clip_path_shape_adds_a_real_shape_and_inverts() {
    let base = document();
    let mut snapshot = base.clone();
    let mutation = SvgBasicMutation::InsertClipPathShape(insert_clip_path_shape::InsertClipPathShape { clip_path_id: "shape".into(), index: 0, node: elem("circle", vec![("cx", "4"), ("cy", "4"), ("r", "4")], vec![]) });
    let undo = Mutation::inverse(&mutation, &base);
    apply_svg_basic_mutation(&mut snapshot, &mutation);
    match node_at(&snapshot.doc, &[0, 0]) {
        Ok(XmlNode::Element { children, .. }) => assert_eq!(children.len(), 2, "the clip path must have gained the shape"),
        other => panic!("expected the clipPath element, got {other:?}"),
    }
    for step in &undo {
        apply_svg_basic_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "adding and removing the clip shape must restore the document");
}
