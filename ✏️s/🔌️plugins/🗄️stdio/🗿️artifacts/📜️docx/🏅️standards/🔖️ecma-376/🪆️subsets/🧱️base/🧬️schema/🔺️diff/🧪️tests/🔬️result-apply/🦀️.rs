use super::*;

#[test]
fn xml_part_reorder_round_trip_is_exact() {
    let base = snapshot_a();
    let last = base.xml_parts.len() - 1;
    assert!(last >= 1, "the reorder needs two XML parts");
    let first_path = base.xml_parts.get(0).unwrap().path.clone();
    let diff = DocxDiff { xml_parts: Some(DocxXmlPartsDelta { moved: vec![DocxXmlPartRelocation { id: first_path, from: 0, to: last }], ..Default::default() }), ..Default::default() };
    let mut reordered = base.clone();
    let first = reordered.xml_parts.remove(0);
    reordered.xml_parts.push(first);
    assert!(!diff.is_empty());
    assert_eq!(protocol::apply_diff(&diff, &base).expect("order diff applies"), reordered);
    assert_eq!(protocol::apply_diff(&diff.inverse(&base), &reordered).expect("inverse applies"), base);
}

#[test]
fn forward_diff_replays_and_its_inverse_restores_the_base() {
    let base = snapshot_a();
    let after = protocol::apply_diff(&demo_forward_diff(), &base).expect("forward diff applies");
    assert_eq!(after, snapshot_b());
    let backward = demo_forward_diff().inverse(&base);
    assert_eq!(protocol::apply_diff(&backward, &after).expect("inverse applies"), base);
    let mut summed = demo_forward_diff();
    summed.absorb(backward);
    assert_eq!(protocol::apply_diff(&summed, &base).expect("forward plus inverse applies"), base);
}

#[test]
fn removal_row_must_sit_at_its_base_index() {
    let base = snapshot_a();
    let path = base.xml_parts.get(0).unwrap().path.clone();
    let diff = DocxDiff { xml_parts: Some(DocxXmlPartsDelta { removed: vec![DocxXmlPartRemoval { id: path, index: 1 }], ..Default::default() }), ..Default::default() };
    assert_eq!(protocol::apply_diff(&diff, &base).unwrap_err().code, "mutation.apply.missing-target");
}

#[semio_framework_async_macros::async_test]
async fn rejects_missing_xml_part_target_without_mutating_base() {
    let base = DocxSnapshot::default();
    let diff = DocxDiff { xml_parts: Some(DocxXmlPartsDelta { modified: vec![DocxXmlPartModification { id: "missing.xml".into(), patch: DocxXmlPartDiff::default() }], ..Default::default() }), ..Default::default() };
    let result = protocol::apply_diff(&diff, &base);
    assert_eq!(result.unwrap_err().code, "mutation.apply.missing-target");
    assert_eq!(base, DocxSnapshot::default());
}
