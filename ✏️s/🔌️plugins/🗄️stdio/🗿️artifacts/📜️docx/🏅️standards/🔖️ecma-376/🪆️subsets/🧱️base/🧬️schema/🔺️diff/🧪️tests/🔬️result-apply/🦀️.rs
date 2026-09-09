use super::*;

/// 🧪️ `SetSnapshot` is a TOTAL replacement, so `DocxDiff::between(base, next)` applied to
/// `base` has to land on `next` EXACTLY — the ORDER of the name-keyed style list included,
/// because `w:styles`' declaration order is what `semantic-docx-ecma-376-mutate-v1` projects
/// by index. Until wave 14 the named triple was order-blind (survivors kept their base order,
/// additions were appended), so undoing `set-snapshot` on the real `📜️example-readme.docx`
/// returned all seven real styles with six of them in the wrong place — 12 differences
/// against the `zip`+`quick-xml` oracle in `mutate-docx-ecma-376::inverse-set-snapshot`. The
/// fixture's own seven styles and the case's own three-style `set-snapshot` target are used
/// here verbatim, so this test fails for the same reason the case did.
#[test]
fn set_snapshot_and_its_inverse_reproduce_the_exact_style_order() {
    let of = |ids: &[&str]| DocxSnapshot::from_parts(OpcPackage::empty(), DocxDocument { body: Vec::new(), styles: ids.iter().map(|id| DocxStyle { id: (*id).into(), name: (*id).into(), based_on: None }).collect() });
    let base = of(&["Normal", "Title", "Heading1", "Heading2", "Heading3", "Code", "TableCell"]);
    let next = of(&["Normal", "Heading1", "TableCell"]);

    let forward = DocxDiff::between(&base, &next);
    assert_eq!(forward.apply(&base).expect("the forward diff applies"), next, "set-snapshot must land on exactly the snapshot it carries");
    assert_eq!(forward.inverse(&base).apply(&next).expect("the inverse applies"), base, "undoing set-snapshot must restore the style order it found");

    // A pure REORDER carries no removal, no modification and no addition whatsoever, so the
    // order field is the only thing in the triple that can express it at all.
    let shuffled = of(&["TableCell", "Normal", "Heading1"]);
    let reorder = DocxDiff::between(&next, &shuffled);
    assert!(!reorder.is_empty(), "a pure reorder must not diff to nothing");
    assert_eq!(reorder.apply(&next).expect("the reorder applies"), shuffled);
}

#[semio_framework_async_macros::async_test]
async fn rejects_missing_style_target_without_mutating_base() {
    let base = DocxSnapshot::default();
    let diff =
        DocxDiff { document: Some(DocxDocumentDiff { styles: Some(DocxStylesDiff { modified: vec![NamedModified { key: "missing".into(), diff: DocxStyleDiff::default() }], ..Default::default() }), ..Default::default() }), ..Default::default() };
    let result = diff.apply(&base);
    assert_eq!(result.unwrap_err().code, "mutation.apply.missing-target");
    assert_eq!(base, DocxSnapshot::default());
}
