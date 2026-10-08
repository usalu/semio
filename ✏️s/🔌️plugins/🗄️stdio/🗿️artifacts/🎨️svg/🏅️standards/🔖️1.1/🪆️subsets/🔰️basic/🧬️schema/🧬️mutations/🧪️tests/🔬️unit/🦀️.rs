use super::*;

fn elem(name: &str, attrs: Vec<(&str, SvgAttributeValue)>, children: Vec<SvgNode>) -> SvgNode {
    SvgNode::Element { name: name.into(), attrs: attrs.into_iter().map(|(n, v)| SvgAttr { name: n.into(), value: v }).collect(), children }
}

fn document() -> SvgSnapshot {
    let mut snapshot = SvgSnapshot::default();
    snapshot.doc.root = Some(elem(
        "svg",
        vec![],
        vec![
            elem("defs", vec![], vec![elem("clipPath", vec![("id", crate::schema::snapshot::SvgAttributeValue::Text("shape".into()))], vec![elem("path", vec![("d", SvgAttributeValue::PathData(vec![crate::schema::snapshot::PathCommand::MoveTo{x:0.0,y:0.0,relative:false},crate::schema::snapshot::PathCommand::HorizontalLineTo{x:8.0,relative:false},crate::schema::snapshot::PathCommand::VerticalLineTo{y:8.0,relative:false},crate::schema::snapshot::PathCommand::HorizontalLineTo{x:0.0,relative:false},crate::schema::snapshot::PathCommand::ClosePath]))], vec![])]), elem("clipPath", vec![("id", crate::schema::snapshot::SvgAttributeValue::Text("lettering".into()))], vec![elem("text", vec![], vec![])])]),
            elem("g", vec![], vec![elem("rect", vec![], vec![])]),
        ],
    ));
    snapshot
}

/// 📇️ The one test that keeps `KINDS` honest against the enum it claims to spell.
#[test]
fn kinds_matches_enum_variants_and_manifest() {
    let every = vec![
        SvgBasicMutation::StampBaseProfile(stamp_base_profile::StampBaseProfile { base_profile: None, version: None, base_profile_index: None, version_index: None }),
        SvgBasicMutation::InsertBasicElement(insert_basic_element::InsertBasicElement { parent: Vec::new(), index: 0, node: elem("rect", vec![], vec![]) }),
        SvgBasicMutation::RemoveElement(remove_element::RemoveElement { parent: Vec::new(), index: 0 }),
        SvgBasicMutation::SetBasicAttribute(set_basic_attribute::SetBasicAttribute { path: Vec::new(), name: "fill".into(), value: None, index: None }),
        SvgBasicMutation::SetClipPathReference(set_clip_path_reference::SetClipPathReference { path: Vec::new(), clip_path_id: None }),
        SvgBasicMutation::InsertClipPathShape(insert_clip_path_shape::InsertClipPathShape { clip_path_id: "shape".into(), index: 0, node: elem("circle", vec![], vec![]) }),
        SvgBasicMutation::SetText(set_text::SetText { path: Vec::new(), text: String::new() }),
        SvgBasicMutation::SetViewBox(set_view_box::SetViewBox { path: Vec::new(), view_box: None, index: None }),
        SvgBasicMutation::SetTransform(set_transform::SetTransform { path: Vec::new(), transform: None, index: None }),
    ];
    let spelled: Vec<&'static str> = every.iter().map(kind_of).collect();
    assert_eq!(spelled, KINDS.to_vec(), "KINDS must spell every variant, in declaration order");

    let manifest = include_str!("../../../../🔮️oracles/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "the oracle manifest's catalog does not declare {kind:?}");
    }
}

/// 🔗 Holds this module's gate lists against the subset's own conformance checker, so the two
/// statements of SVG Basic 1.1's excluded vocabulary cannot drift apart.
#[test]
fn blocklists_agree_with_the_subset_conformance_checker() {
    use crate::standards::v1_1::subsets::basic::schema::conformance::check_svg_basic_conformance;
    let hard = |snapshot: &SvgSnapshot| check_svg_basic_conformance(snapshot).into_iter().any(|d| matches!(d.severity, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal));
    let root = |children: Vec<SvgNode>| {
        let mut snapshot = SvgSnapshot::default();
        snapshot.doc.root = Some(elem("svg", vec![("baseProfile", crate::schema::snapshot::SvgAttributeValue::Text("basic".into())), ("version", crate::schema::snapshot::SvgAttributeValue::Text("1.1".into()))], children));
        snapshot
    };
    for name in BLOCKED_FILTER_PRIMITIVES {
        assert!(is_blocked_filter_primitive(name), "{name} must be gated by this module");
        assert!(hard(&root(vec![elem("filter", vec![("id", crate::schema::snapshot::SvgAttributeValue::Text("f1".into()))], vec![elem(name, vec![], vec![])])])), "the subset conformance checker does not reject <{name}>");
    }
    for name in ["feGaussianBlur", "feBlend", "feFlood", "feOffset", "linearGradient", "clipPath", "mask"] {
        assert!(!is_blocked_filter_primitive(name), "{name} is retained by SVG Basic 1.1");
        assert!(!hard(&root(vec![elem("filter", vec![("id", crate::schema::snapshot::SvgAttributeValue::Text("f1".into()))], vec![elem(name, vec![], vec![])])])), "the subset conformance checker rejects the retained <{name}>");
    }
    for name in TEXT_ELEMENTS {
        assert!(carries_text(&elem(name, vec![], vec![])), "{name} must count as text for this module's clip gate");
    }
    assert!(hard(&root(vec![elem("clipPath", vec![("id", crate::schema::snapshot::SvgAttributeValue::Text("c1".into()))], vec![elem("text", vec![], vec![])]), elem("rect", vec![("clip-path", crate::schema::snapshot::SvgAttributeValue::LocalReference("c1".into()))], vec![])])), "the subset conformance checker does not reject clipping to text");
    assert!(!hard(&root(vec![elem("clipPath", vec![("id", crate::schema::snapshot::SvgAttributeValue::Text("c1".into()))], vec![elem("path", vec![], vec![])]), elem("rect", vec![("clip-path", crate::schema::snapshot::SvgAttributeValue::LocalReference("c1".into()))], vec![])])), "the subset conformance checker rejects a shape-only clip path");
}

#[test]
fn insert_basic_element_accepts_a_retained_filter_primitive() {
    let mut snapshot = document();
    let outcome = apply_svg_basic_mutation(
        &mut snapshot,
        &SvgBasicMutation::InsertBasicElement(insert_basic_element::InsertBasicElement { parent: vec![0], index: 0, node: elem("filter", vec![("id", crate::schema::snapshot::SvgAttributeValue::Text("blur".into()))], vec![elem("feGaussianBlur", vec![("stdDeviation", crate::schema::snapshot::SvgAttributeValue::Text("2".into()))], vec![])]) }),
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
    let undo = Mutation::inverse(&mutation, &base).expect("valid retained mutation inverse fixture");
    let outcome = apply_svg_basic_mutation(&mut snapshot, &mutation);
    assert!(outcome.messages().is_empty(), "a shape-only clipPath is legal: {:?}", outcome.messages());
    assert_eq!(element_attr(node_at(&snapshot.doc, &[1]).unwrap(), "clip-path"), Some(&SvgAttributeValue::LocalReference("shape".into())));
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
    let mutation = SvgBasicMutation::InsertClipPathShape(insert_clip_path_shape::InsertClipPathShape { clip_path_id: "shape".into(), index: 0, node: elem("circle", vec![("cx", crate::schema::snapshot::SvgAttributeValue::Length(crate::schema::snapshot::SvgLength{magnitude:4.0,unit:"".into()})), ("cy", crate::schema::snapshot::SvgAttributeValue::Length(crate::schema::snapshot::SvgLength{magnitude:4.0,unit:"".into()})), ("r", crate::schema::snapshot::SvgAttributeValue::Length(crate::schema::snapshot::SvgLength{magnitude:4.0,unit:"".into()}))], vec![]) });
    let undo = Mutation::inverse(&mutation, &base).expect("valid retained mutation inverse fixture");
    apply_svg_basic_mutation(&mut snapshot, &mutation);
    match node_at(&snapshot.doc, &[0, 0]) {
        Ok(SvgNode::Element { children, .. }) => assert_eq!(children.len(), 2, "the clip path must have gained the shape"),
        other => panic!("expected the clipPath element, got {other:?}"),
    }
    for step in &undo {
        apply_svg_basic_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "adding and removing the clip shape must restore the document");
}

#[semio_framework_async_macros::async_test]
async fn every_leaf_satisfies_the_inverse_sum_law() {
    let base = document();
    let view_box = crate::schema::snapshot::ViewBox { min_x: 0.0, min_y: 0.0, width: 10.0, height: 20.0 };
    for mutation in [
        SvgBasicMutation::StampBaseProfile(stamp_base_profile::StampBaseProfile { base_profile: Some("basic".into()), version: Some("1.1".into()), base_profile_index: None, version_index: None }),
        SvgBasicMutation::InsertBasicElement(insert_basic_element::InsertBasicElement { parent: Vec::new(), index: 1, node: elem("rect", vec![], vec![]) }),
        SvgBasicMutation::RemoveElement(remove_element::RemoveElement { parent: Vec::new(), index: 1 }),
        SvgBasicMutation::SetBasicAttribute(set_basic_attribute::SetBasicAttribute { path: Vec::new(), name: "fill".into(), value: Some(SvgAttributeValue::Text("red".into())), index: None }),
        SvgBasicMutation::SetClipPathReference(set_clip_path_reference::SetClipPathReference { path: vec![1], clip_path_id: Some("shape".into()) }),
        SvgBasicMutation::InsertClipPathShape(insert_clip_path_shape::InsertClipPathShape { clip_path_id: "shape".into(), index: 1, node: elem("circle", vec![], vec![]) }),
        SvgBasicMutation::SetViewBox(set_view_box::SetViewBox { path: Vec::new(), view_box: Some(view_box), index: None }),
        SvgBasicMutation::SetTransform(set_transform::SetTransform { path: vec![1], transform: Some(vec![TransformOp::Scale { x: 2.0, y: None }]), index: None }),
    ] {
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
    }
}
