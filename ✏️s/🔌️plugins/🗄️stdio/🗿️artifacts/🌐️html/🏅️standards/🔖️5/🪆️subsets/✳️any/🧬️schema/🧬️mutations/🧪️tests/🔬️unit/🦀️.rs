
use super::*;
use crate::standards::v5::subsets::any::schema::diff::{HtmlChildAdded as HtmlChildAddedT, HtmlNodeDiff as HtmlNodeDiffT};
use crate::standards::v5::subsets::any::schema::snapshot::{HtmlAttr, STDIO_HTML_DOCUMENT_SCHEMA, write_html_document};
use protocol::MutationDiff;
use protocol::command::DiffAlgebra;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn el(name: &str, attrs: Vec<HtmlAttr>, children: Vec<HtmlNode>) -> HtmlNode {
    HtmlNode::Element { name: name.into(), attributes: attrs, children }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn fixture() -> HtmlSnapshot {
    <HtmlSnapshot as store::ArtifactDsl>::parse_dsl("<!DOCTYPE html>\n<html><body><p id=\"x\" width=\"5\">hi</p></body></html>\n").unwrap()
}

#[semio_framework_async_macros::async_test]
async fn insert_then_remove_node_apply_and_inverse() {
    let base = fixture();
    let insert = HtmlMutation::InsertNode(insert_node::InsertNode { parent: vec![0], index: 1, node: el("span", vec![HtmlAttr::new("class", "x")], vec![]) });
    let mut after = base.clone();
    apply_html_mutation(&mut after, &insert);
    match node_at(&after, &[0]).unwrap() {
        HtmlNode::Element { children, .. } => assert_eq!(children.len(), 2),
        other => panic!("unexpected node {other:?}"),
    }
    let inverses = Mutation::inverse(&insert, &base);
    let mut restored = after.clone();
    for inv in &inverses {
        apply_html_mutation(&mut restored, inv);
    }
    assert_eq!(restored, base);
}

#[semio_framework_async_macros::async_test]
async fn set_attribute_tristate_apply_and_inverse_round_trip() {
    let base = fixture();
    // Some(Some(v)): modify existing value.
    let m1 = HtmlMutation::SetAttribute(set_attribute::SetAttribute { path: vec![0, 0], name: "width".into(), value: Some(Some("99".into())) });
    let d1 = Mutation::diff(&m1, &base);
    let after1 = <HtmlDiff as MutationDiff<HtmlSnapshot>>::apply(d1.diff(), &base).unwrap();
    assert_eq!(element_attr(node_at(&after1, &[0, 0]).unwrap(), "width"), Some(&Some("99".to_string())));
    let mut restored1 = after1.clone();
    for inv in Mutation::inverse(&m1, &base) {
        apply_html_mutation(&mut restored1, &inv);
    }
    assert_eq!(write_html_document(&restored1), write_html_document(&base));

    // Some(None): make valueless.
    let m2 = HtmlMutation::SetAttribute(set_attribute::SetAttribute { path: vec![0, 0], name: "width".into(), value: Some(None) });
    let mut after2 = base.clone();
    apply_html_mutation(&mut after2, &m2);
    assert_eq!(element_attr(node_at(&after2, &[0, 0]).unwrap(), "width"), Some(&None));
    for inv in Mutation::inverse(&m2, &base) {
        apply_html_mutation(&mut after2, &inv);
    }
    assert_eq!(write_html_document(&after2), write_html_document(&base));

    // None: remove entirely.
    let m3 = HtmlMutation::SetAttribute(set_attribute::SetAttribute { path: vec![0, 0], name: "width".into(), value: None });
    let mut after3 = base.clone();
    apply_html_mutation(&mut after3, &m3);
    assert_eq!(element_attr(node_at(&after3, &[0, 0]).unwrap(), "width"), None);
    for inv in Mutation::inverse(&m3, &base) {
        apply_html_mutation(&mut after3, &inv);
    }
    assert_eq!(write_html_document(&after3), write_html_document(&base));

    // None -> Some: add a brand new attribute.
    let m4 = HtmlMutation::SetAttribute(set_attribute::SetAttribute { path: vec![0, 0], name: "hidden".into(), value: Some(None) });
    let mut after4 = base.clone();
    apply_html_mutation(&mut after4, &m4);
    assert_eq!(element_attr(node_at(&after4, &[0, 0]).unwrap(), "hidden"), Some(&None));
    for inv in Mutation::inverse(&m4, &base) {
        apply_html_mutation(&mut after4, &inv);
    }
    assert_eq!(write_html_document(&after4), write_html_document(&base));
}

#[semio_framework_async_macros::async_test]
async fn remove_node_inverse_restores_removed_node() {
    let base = fixture();
    let remove = HtmlMutation::RemoveNode(remove_node::RemoveNode { parent: vec![0], index: 0 });
    let mut after = base.clone();
    apply_html_mutation(&mut after, &remove);
    match node_at(&after, &[0]).unwrap() {
        HtmlNode::Element { children, .. } => assert!(children.is_empty()),
        other => panic!("unexpected node {other:?}"),
    }
    for inv in Mutation::inverse(&remove, &base) {
        apply_html_mutation(&mut after, &inv);
    }
    assert_eq!(write_html_document(&after), write_html_document(&base));
}

#[semio_framework_async_macros::async_test]
async fn set_element_name_apply_and_inverse() {
    let base = fixture();
    let mutation = HtmlMutation::SetElementName(set_element_name::SetElementName { path: vec![0, 0], name: "div".into() });
    let mut after = base.clone();
    apply_html_mutation(&mut after, &mutation);
    match node_at(&after, &[0, 0]).unwrap() {
        HtmlNode::Element { name, .. } => assert_eq!(name, "div"),
        other => panic!("unexpected node {other:?}"),
    }
    for inv in Mutation::inverse(&mutation, &base) {
        apply_html_mutation(&mut after, &inv);
    }
    assert_eq!(write_html_document(&after), write_html_document(&base));
}

//#region 🔖️Fixtures
/// 🌱 `sweep_a`/`sweep_b`: differ in EVERY mutable field. `doctype` goes `Some(x) -> None`
/// (tri-state `Some(None)`). `root`'s attrs (name-keyed) exercise removed+modified+added
/// simultaneously. The naive positional `children_diff_between` can only ever show ONE of
/// {removed-tail, added-tail} per instance, so `removed` is exercised at the top-level
/// children triple and `added` at the nested triple inside the modified child, while that
/// modified child's OWN diff (name+attributes+children all `Some`) is the
/// "modified-in-every-field" collection entry.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_a() -> HtmlSnapshot {
    HtmlSnapshot {
        schema: STDIO_HTML_DOCUMENT_SCHEMA.into(),
        doctype: Some("DOCTYPE html".into()),
        root: el(
            "html",
            vec![HtmlAttr::new("keep", "k"), HtmlAttr::new("toRemove", "r"), HtmlAttr::new("toModify", "old")],
            vec![el("g", vec![HtmlAttr::new("x", "1")], vec![el("rect", vec![], vec![])]), HtmlNode::Text { text: "stay".into() }, el("toDrop", vec![], vec![])],
        ),
    }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_b() -> HtmlSnapshot {
    HtmlSnapshot {
        schema: STDIO_HTML_DOCUMENT_SCHEMA.into(),
        doctype: None,
        root: el(
            "htmlRenamed",
            vec![HtmlAttr::new("keep", "k"), HtmlAttr::new("toModify", "new"), HtmlAttr::boolean("added")],
            vec![el("gModified", vec![HtmlAttr::new("x", "2"), HtmlAttr::new("y", "3")], vec![el("rect", vec![], vec![]), el("circle", vec![], vec![])]), HtmlNode::Text { text: "stay".into() }],
        ),
    }
}
//#endregion 🔖️Fixtures

//#region 🔖️MutationDiffLaw
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_mutations() -> Vec<HtmlMutation> {
    vec![
        HtmlMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: sweep_b() }),
        HtmlMutation::SetDoctype(set_doctype::SetDoctype { doctype: Some("DOCTYPE html PUBLIC".into()) }),
        HtmlMutation::SetDoctype(set_doctype::SetDoctype { doctype: None }),
        HtmlMutation::InsertNode(insert_node::InsertNode { parent: vec![0], index: 1, node: el("span", vec![], vec![]) }),
        HtmlMutation::RemoveNode(remove_node::RemoveNode { parent: vec![0], index: 0 }),
        HtmlMutation::SetElementName(set_element_name::SetElementName { path: vec![0, 0], name: "div".into() }),
        HtmlMutation::SetAttribute(set_attribute::SetAttribute { path: vec![0, 0], name: "width".into(), value: Some(Some("99".into())) }),
        HtmlMutation::SetAttribute(set_attribute::SetAttribute { path: vec![0, 0], name: "width".into(), value: None }),
        HtmlMutation::SetText(set_text::SetText { path: vec![0, 0, 0], text: "hi".into() }),
    ]
}

#[semio_framework_async_macros::async_test]
async fn mutation_diff_law() {
    for mutation in sample_mutations() {
        let base = fixture();
        let diff_direct = Mutation::diff(&mutation, &base);
        let applied_via_diff = MutationDiff::apply(diff_direct.diff(), &base).unwrap();

        let mut via_apply = base.clone();
        let diff_from_apply = apply_html_mutation(&mut via_apply, &mutation);

        assert_eq!(applied_via_diff, via_apply, "mutation_diff_law: apply mismatch for {mutation:?}");
        assert_eq!(diff_direct, diff_from_apply, "mutation_diff_law: diff mismatch for {mutation:?}");
    }
}
//#endregion 🔖️MutationDiffLaw

//#region 🔖️InverseLaw
#[semio_framework_async_macros::async_test]
async fn inverse_law() {
    for mutation in sample_mutations() {
        let base = fixture();

        let mut round_tripped = base.clone();
        apply_html_mutation(&mut round_tripped, &mutation);
        for inverse_mutation in <HtmlMutation as Mutation<HtmlSnapshot>>::inverse(&mutation, &base) {
            apply_html_mutation(&mut round_tripped, &inverse_mutation);
        }
        assert_eq!(round_tripped, base, "inverse_law (mutation-level).await failed for {mutation:?}");

        let diff = Mutation::diff(&mutation, &base);
        let next = MutationDiff::apply(diff.diff(), &base).unwrap();
        let inverse_diff = DiffAlgebra::inverse(diff.diff(), &base);
        let restored = MutationDiff::apply(&inverse_diff, &next).unwrap();
        assert_eq!(restored, base, "inverse_law (diff-level).await failed for {mutation:?}");
    }
}
//#endregion 🔖️InverseLaw

//#region 🔖️AbsorbLaw
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn two_child_root(a_name: &str, b_name: &str) -> HtmlSnapshot {
    HtmlSnapshot { schema: STDIO_HTML_DOCUMENT_SCHEMA.into(), doctype: None, root: el("html", vec![], vec![el(a_name, vec![], vec![]), el(b_name, vec![], vec![])]) }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn assert_absorb_matches_sequential(base: &HtmlSnapshot, d1: &HtmlDiff, d2: &HtmlDiff) -> HtmlDiff {
    let sequential = MutationDiff::apply(d2, &MutationDiff::apply(d1, base).unwrap()).unwrap();
    let mut absorbed = d1.clone();
    MutationDiff::absorb(&mut absorbed, d2.clone());
    assert_eq!(MutationDiff::apply(&absorbed, base).unwrap(), sequential, "absorb_law: apply(absorb(d1,d2), base) != sequential");
    absorbed
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn root_children_diff(diff: &HtmlDiff) -> &HtmlChildrenDiff {
    match diff.root.as_ref().expect("root diff present") {
        HtmlNodeDiffT::Element(e) => e.children.as_ref().expect("children diff present"),
        other => panic!("expected element diff, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn absorb_law() {
    {
        let base = two_child_root("a", "b");
        let d1 = Mutation::diff(&HtmlMutation::InsertNode(insert_node::InsertNode { parent: vec![], index: 2, node: el("f", vec![], vec![]) }), &base);
        let mid = MutationDiff::apply(d1.diff(), &base).unwrap();
        let d2 = Mutation::diff(&HtmlMutation::RemoveNode(remove_node::RemoveNode { parent: vec![], index: 0 }), &mid);
        let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
        let triple = root_children_diff(&absorbed);
        assert_eq!(triple.removed, vec![0]);
        assert_eq!(triple.added.len(), 1);
        assert_eq!(triple.added[0].index, 1);
        let HtmlNode::Element { name, .. } = &triple.added[0].item else { panic!("expected element") };
        assert_eq!(name, "f");
    }
    {
        let base = two_child_root("a", "b");
        let d1 = Mutation::diff(&HtmlMutation::InsertNode(insert_node::InsertNode { parent: vec![], index: 2, node: el("f", vec![], vec![]) }), &base);
        let mid = MutationDiff::apply(d1.diff(), &base).unwrap();
        let d2 = Mutation::diff(&HtmlMutation::InsertNode(insert_node::InsertNode { parent: vec![], index: 2, node: el("g", vec![], vec![]) }), &mid);
        let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
        let triple = root_children_diff(&absorbed);
        assert_eq!(triple.added.len(), 2, "both inserts must survive absorb, not LWW-clobber");
        let names: Vec<&str> = triple
            .added
            .iter()
            .map(|a| match &a.item {
                HtmlNode::Element { name, .. } => name.as_str(),
                _ => "",
            })
            .collect();
        assert!(names.contains(&"f"));
        assert!(names.contains(&"g"));
    }
    {
        let base = two_child_root("a", "b");
        let d1 = Mutation::diff(&HtmlMutation::InsertNode(insert_node::InsertNode { parent: vec![], index: 1, node: el("f", vec![], vec![]) }), &base);
        let mid = MutationDiff::apply(d1.diff(), &base).unwrap();
        let d2 = Mutation::diff(&HtmlMutation::SetAttribute(set_attribute::SetAttribute { path: vec![1], name: "k".into(), value: Some(Some("v".into())) }), &mid);
        let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
        let triple = root_children_diff(&absorbed);
        assert!(triple.modified.is_empty(), "patch-into-added must not surface as a separate modified entry");
        assert_eq!(triple.added.len(), 1);
        let HtmlNode::Element { attributes, .. } = &triple.added[0].item else { panic!("expected element") };
        assert!(attributes.iter().any(|a| a.name == "k" && a.value.as_deref() == Some("v")));
    }
    {
        let base = two_child_root("a", "b");
        let d1 = Mutation::diff(&HtmlMutation::SetAttribute(set_attribute::SetAttribute { path: vec![1], name: "k".into(), value: Some(Some("v".into())) }), &base);
        let mid = MutationDiff::apply(d1.diff(), &base).unwrap();
        let d2 = Mutation::diff(&HtmlMutation::RemoveNode(remove_node::RemoveNode { parent: vec![], index: 1 }), &mid);
        let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
        let triple = root_children_diff(&absorbed);
        assert!(triple.modified.is_empty(), "modify of a since-removed item must not survive absorb");
        assert_eq!(triple.removed, vec![1]);
    }
    {
        let base = two_child_root("a", "b");
        let d1 = Mutation::diff(&HtmlMutation::InsertNode(insert_node::InsertNode { parent: vec![], index: 2, node: el("f", vec![], vec![]) }), &base);
        let mid1 = MutationDiff::apply(d1.diff(), &base).unwrap();
        let d2 = Mutation::diff(&HtmlMutation::InsertNode(insert_node::InsertNode { parent: vec![], index: 2, node: el("g", vec![], vec![]) }), &mid1);
        let mid2 = MutationDiff::apply(d2.diff(), &mid1).unwrap();
        let d3 = Mutation::diff(&HtmlMutation::RemoveNode(remove_node::RemoveNode { parent: vec![], index: 0 }), &mid2);
        let sequential = MutationDiff::apply(d3.diff(), &mid2).unwrap();

        let mut left = d1.diff().clone();
        MutationDiff::absorb(&mut left, d2.diff().clone());
        MutationDiff::absorb(&mut left, d3.diff().clone());

        let mut d2_then_d3 = d2.diff().clone();
        MutationDiff::absorb(&mut d2_then_d3, d3.diff().clone());
        let mut right = d1.diff().clone();
        MutationDiff::absorb(&mut right, d2_then_d3);

        assert_eq!(MutationDiff::apply(&left, &base).unwrap(), sequential, "absorb associativity (left) failed");
        assert_eq!(MutationDiff::apply(&right, &base).unwrap(), sequential, "absorb associativity (right) failed");
    }
}
//#endregion 🔖️AbsorbLaw

//#region 🔖️BetweenRoundtripLaw
#[semio_framework_async_macros::async_test]
async fn between_roundtrip_law() {
    let a = sweep_a();
    let b = sweep_b();
    assert_eq!(MutationDiff::apply(&<HtmlDiff as DiffAlgebra<HtmlSnapshot>>::between(&a, &b), &a).unwrap(), b);
    assert_eq!(MutationDiff::apply(&<HtmlDiff as DiffAlgebra<HtmlSnapshot>>::between(&b, &a), &b).unwrap(), a);

    let sample = fixture();
    assert_eq!(MutationDiff::apply(&<HtmlDiff as DiffAlgebra<HtmlSnapshot>>::between(&sample, &sample), &sample).unwrap(), sample);

    let real = <HtmlSnapshot as store::ArtifactDsl>::parse_dsl("<!DOCTYPE html>\n<html><body><div id=\"layer1\"><p>a</p><span>b</span></div></body></html>\n").unwrap();
    let mut mutated = real.clone();
    apply_html_mutation(&mut mutated, &HtmlMutation::SetAttribute(set_attribute::SetAttribute { path: vec![0, 0], name: "id".into(), value: Some(Some("root".into())) }));
    assert_ne!(real, mutated);
    assert_eq!(MutationDiff::apply(&<HtmlDiff as DiffAlgebra<HtmlSnapshot>>::between(&real, &mutated), &real).unwrap(), mutated);
    assert_eq!(MutationDiff::apply(&<HtmlDiff as DiffAlgebra<HtmlSnapshot>>::between(&mutated, &real), &mutated).unwrap(), real);
}
//#endregion 🔖️BetweenRoundtripLaw

//#region 🔖️FieldSweep
#[semio_framework_async_macros::async_test]
async fn field_sweep() {
    let a = sweep_a();
    let b = sweep_b();

    let diff_ab = <HtmlDiff as DiffAlgebra<HtmlSnapshot>>::between(&a, &b);
    assert_eq!(MutationDiff::apply(&diff_ab, &a).unwrap(), b);
    let diff_ba = <HtmlDiff as DiffAlgebra<HtmlSnapshot>>::between(&b, &a);
    assert_eq!(MutationDiff::apply(&diff_ba, &b).unwrap(), a);
    assert!(<HtmlDiff as DiffAlgebra<HtmlSnapshot>>::between(&a, &a).is_empty());

    assert_eq!(diff_ab.doctype, Some(None));
    assert!(diff_ab.root.is_some());

    let HtmlNodeDiffT::Element(root_diff) = diff_ab.root.as_ref().unwrap() else { panic!("expected element diff") };
    assert!(root_diff.name.is_some());
    let attrs_diff = root_diff.attributes.as_ref().expect("attrs diff present");
    assert!(!attrs_diff.removed.is_empty(), "attrs: removed not exercised");
    assert!(!attrs_diff.modified.is_empty(), "attrs: modified not exercised");
    assert!(!attrs_diff.added.is_empty(), "attrs: added not exercised");

    let children_diff = root_diff.children.as_ref().expect("children diff present");
    assert!(!children_diff.removed.is_empty(), "children: removed not exercised");
    assert_eq!(children_diff.modified.len(), 1);
    let modified_entry = &children_diff.modified[0];
    let HtmlNodeDiffT::Element(modified_element) = &modified_entry.diff else { panic!("expected element diff") };
    assert!(modified_element.name.is_some(), "modified child: name not exercised");
    assert!(modified_element.attributes.is_some(), "modified child: attributes not exercised");
    let nested_children = modified_element.children.as_ref().expect("nested children diff present");
    let nested_added: &Vec<HtmlChildAddedT> = &nested_children.added;
    assert!(!nested_added.is_empty(), "children: added (nested) not exercised");
}
//#endregion 🔖️FieldSweep

/// 🧪️ op_text_binary_roundtrip_law: round-trip laws for the hand-rolled `HtmlMutation` grammar
/// — exercises every variant incl. `InsertNode`'s bare `HtmlNode` payload and `SetAttribute`'s
/// tri-state value.
#[semio_framework_async_macros::async_test]
async fn op_text_binary_roundtrip_law() {
    let base = fixture();
    let mutations = vec![
        HtmlMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() }),
        HtmlMutation::SetDoctype(set_doctype::SetDoctype { doctype: Some("DOCTYPE html".into()) }),
        HtmlMutation::SetDoctype(set_doctype::SetDoctype { doctype: None }),
        HtmlMutation::InsertNode(insert_node::InsertNode { parent: vec![0], index: 1, node: el("span", vec![HtmlAttr::new("r", "1")], vec![]) }),
        HtmlMutation::RemoveNode(remove_node::RemoveNode { parent: vec![0], index: 2 }),
        HtmlMutation::SetElementName(set_element_name::SetElementName { path: vec![0], name: "g".into() }),
        HtmlMutation::SetAttribute(set_attribute::SetAttribute { path: vec![0], name: "width".into(), value: Some(Some("99".into())) }),
        HtmlMutation::SetAttribute(set_attribute::SetAttribute { path: vec![0], name: "width".into(), value: Some(None) }),
        HtmlMutation::SetAttribute(set_attribute::SetAttribute { path: vec![0], name: "width".into(), value: None }),
        HtmlMutation::SetText(set_text::SetText { path: vec![0, 1], text: "hello world".into() }),
        HtmlMutation::SetComment(set_comment::SetComment { path: vec![0, 1], text: " comment ".into() }),
        HtmlMutation::SetRawText(set_raw_text::SetRawText { path: vec![0, 1], text: "console.log(1);".into() }),
    ];
    for mutation in mutations {
        let printed = mutation.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
        let parsed = HtmlMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
        assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

        let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
        let decoded = HtmlMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
        assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
    }
}

//#region kinds_law
/// 🎯️ kinds_const_matches_enum_variants_in_declaration_order: `KINDS` is the ONLY thing the
/// wave 7 catalog contract (`../🔣️oracle.json`'s `kinds` array) is checked against,
/// so it must genuinely enumerate every variant, in order, with the exact `OpText` keyword the
/// case's adapter registers `mutate-<kind>`/`inverse-<kind>` scenario ids under.
#[semio_framework_async_macros::async_test]
async fn kinds_const_matches_enum_variants_in_declaration_order() {
    let base = fixture();
    let one_per_variant = vec![
        HtmlMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() }),
        HtmlMutation::SetDoctype(set_doctype::SetDoctype { doctype: Some("DOCTYPE html".into()) }),
        HtmlMutation::InsertNode(insert_node::InsertNode { parent: vec![0], index: 0, node: el("span", vec![], vec![]) }),
        HtmlMutation::RemoveNode(remove_node::RemoveNode { parent: vec![0], index: 0 }),
        HtmlMutation::SetElementName(set_element_name::SetElementName { path: vec![0], name: "div".into() }),
        HtmlMutation::SetAttribute(set_attribute::SetAttribute { path: vec![0], name: "id".into(), value: Some(Some("x".into())) }),
        HtmlMutation::SetText(set_text::SetText { path: vec![0, 0], text: "x".into() }),
        HtmlMutation::SetComment(set_comment::SetComment { path: vec![0, 0], text: "x".into() }),
        HtmlMutation::SetRawText(set_raw_text::SetRawText { path: vec![0, 0], text: "x".into() }),
    ];
    assert_eq!(one_per_variant.len(), KINDS.len(), "one_per_variant must cover every KINDS entry exactly once");
    for (mutation, kind) in one_per_variant.iter().zip(KINDS.iter()) {
        let printed = mutation.print_op();
        let keyword = printed.split(' ').next().unwrap_or(&printed);
        assert_eq!(keyword, *kind, "KINDS order must match the enum's own OpText keyword order for {mutation:?}");
    }
}
//#endregion kinds_law
