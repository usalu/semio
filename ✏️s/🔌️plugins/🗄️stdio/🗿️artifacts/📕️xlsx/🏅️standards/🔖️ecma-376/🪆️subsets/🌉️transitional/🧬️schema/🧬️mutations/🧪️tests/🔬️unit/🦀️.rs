use super::*;

//#region 🔖️KindsConformanceLaw
/// 🧭️ `kind_of` is an EXHAUSTIVE match (no wildcard arm) — the compiler refuses this file if a
/// variant is added to `XlsxTransitionalMutation` without a matching kebab-case spelling here, which is what keeps
/// `KINDS` honest against the enum. The second half reads the sibling oracle manifest's `kinds`
/// array as text (the framework never parses Rust, so this is the only side that can prove the
/// manifest matches) and asserts the same list, in the same order.
#[test]
fn kinds_match_enum_and_catalog() {
    fn kind_of(mutation: &XlsxTransitionalMutation) -> &'static str {
        match mutation {
            XlsxTransitionalMutation::SetMainNamespace(_) => "set-main-namespace",
            XlsxTransitionalMutation::SetRelationshipsNamespace(_) => "set-relationships-namespace",
            XlsxTransitionalMutation::SetRelationshipBase(_) => "set-relationship-base",
            XlsxTransitionalMutation::SetConformanceAttribute(_) => "set-conformance-attribute",
            XlsxTransitionalMutation::RemoveConformanceAttribute(_) => "remove-conformance-attribute",
            XlsxTransitionalMutation::SetWorksheetContentType(_) => "set-worksheet-content-type",
        }
    }
    let samples = [
        XlsxTransitionalMutation::SetMainNamespace(set_main_namespace::SetMainNamespace { namespace: String::new() }),
        XlsxTransitionalMutation::SetRelationshipsNamespace(set_relationships_namespace::SetRelationshipsNamespace { namespace: String::new() }),
        XlsxTransitionalMutation::SetRelationshipBase(set_relationship_base::SetRelationshipBase { base: String::new() }),
        XlsxTransitionalMutation::SetConformanceAttribute(set_conformance_attribute::SetConformanceAttribute { value: String::new() }),
        XlsxTransitionalMutation::RemoveConformanceAttribute(remove_conformance_attribute::RemoveConformanceAttribute {}),
        XlsxTransitionalMutation::SetWorksheetContentType(set_worksheet_content_type::SetWorksheetContentType { path: String::new(), content_type: String::new(), override_index: None }),
    ];
    let from_enum: Vec<&'static str> = samples.iter().map(kind_of).collect();
    assert_eq!(from_enum, KINDS, "KINDS must list every XlsxTransitionalMutation variant, in declaration order");

    let manifest = include_str!("../../../../🔮️oracles/🔣️.json");
    let needle = "\"kinds\": [";
    let start = manifest.find(needle).expect("manifest declares a kinds array") + needle.len();
    let end = start + manifest[start..].find(']').expect("kinds array is closed");
    let declared: Vec<String> = manifest[start..end].split(',').map(|entry| entry.trim().trim_matches('"').to_string()).filter(|entry| !entry.is_empty()).collect();
    assert_eq!(declared, KINDS, "the oracle manifest's kinds must match XlsxTransitionalMutation exactly");
}
//#endregion 🔖️KindsConformanceLaw

//#region 🔖️StampLaw
/// 🏗️ A three-sheet workbook declaring the strict families, so every stamp kind has something to retarget.
fn stampable() -> XlsxSnapshot {
    use crate::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_xlsx;
    use crate::standards::v_ecma_376::subsets::base::schema::snapshot::{XlsxSheet, XlsxWorkbook};
    let workbook = XlsxWorkbook { sheets: ["One", "Two", "Three"].into_iter().map(|name| XlsxSheet { name: name.into(), cells: vec![] }).collect(), shared_strings: vec![] };
    crate::standards::v_ecma_376::subsets::strict::schema::stamp_strict_namespace(build_minimal_xlsx(workbook))
}

/// 🏅️ Every stamp kind satisfies the inverse sum law on a workbook declaring the opposite class, and stamping into the class and back out restores the workbook.
#[semio_framework_async_macros::async_test]
async fn every_stamp_kind_satisfies_the_inverse_sum_law_and_stamping_round_trips() {
    let base = stampable();
    for mutation in stamp_conformance_class_mutations(false) {
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
    }
    let mut stamped = base.clone();
    for mutation in stamp_conformance_class_mutations(false) {
        apply_xlsx_transitional_mutation(&mut stamped, &mutation);
    }
    assert_ne!(stamped, base);
    for mutation in stamp_conformance_class_mutations(true) {
        apply_xlsx_transitional_mutation(&mut stamped, &mutation);
    }
    assert_eq!(stamped, base);
}
//#endregion 🔖️StampLaw

//#region 🔖️AuthorityLaw
/// 📄️ Every class axis lives in the snapshot's authoritative logical XML parts: retargeting the main namespace and
/// stamping the conformance attribute move the main part itself — an edit of `opc.parts` alone would move nothing, because
/// XML parts are never opaque OPC bytes.
#[test]
fn class_edits_move_the_authoritative_main_part() {
    let mut snapshot = XlsxSnapshot::default();
    let main = main_part_path(&snapshot).expect("the default workbook has a main part");
    apply_xlsx_transitional_mutation(&mut snapshot, &XlsxTransitionalMutation::SetMainNamespace(set_main_namespace::SetMainNamespace { namespace: STRICT_MAIN_NS.into() }));
    apply_xlsx_transitional_mutation(&mut snapshot, &XlsxTransitionalMutation::SetConformanceAttribute(set_conformance_attribute::SetConformanceAttribute { value: "strict".into() }));
    let root = snapshot.xml_part(&main).and_then(|part| part.document.root.as_ref()).expect("the main part keeps its root");
    assert!(declares_namespace(root, STRICT_MAIN_NS), "the strict main namespace must land on the logical main part");
    assert_eq!(conformance_attribute(&snapshot).as_deref(), Some("strict"));
}
//#endregion 🔖️AuthorityLaw
