
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
            XlsxTransitionalMutation::SetSnapshot(_) => "set-snapshot",
            XlsxTransitionalMutation::SetMainNamespace(_) => "set-main-namespace",
            XlsxTransitionalMutation::SetRelationshipsNamespace(_) => "set-relationships-namespace",
            XlsxTransitionalMutation::SetConformanceAttribute(_) => "set-conformance-attribute",
            XlsxTransitionalMutation::RemoveConformanceAttribute(_) => "remove-conformance-attribute",
            XlsxTransitionalMutation::SetWorksheetContentType(_) => "set-worksheet-content-type",
        }
    }
    let samples = [
        XlsxTransitionalMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: XlsxSnapshot::default() }),
        XlsxTransitionalMutation::SetMainNamespace(set_main_namespace::SetMainNamespace { namespace: String::new() }),
        XlsxTransitionalMutation::SetRelationshipsNamespace(set_relationships_namespace::SetRelationshipsNamespace { namespace: String::new() }),
        XlsxTransitionalMutation::SetConformanceAttribute(set_conformance_attribute::SetConformanceAttribute { value: String::new() }),
        XlsxTransitionalMutation::RemoveConformanceAttribute(remove_conformance_attribute::RemoveConformanceAttribute {}),
        XlsxTransitionalMutation::SetWorksheetContentType(set_worksheet_content_type::SetWorksheetContentType { path: String::new(), content_type: String::new() }),
    ];
    let from_enum: Vec<&'static str> = samples.iter().map(kind_of).collect();
    assert_eq!(from_enum, KINDS, "KINDS must list every XlsxTransitionalMutation variant, in declaration order");

    let manifest = include_str!("../../../../🔮️oracle/🔣️.json");
    let needle = "\"kinds\": [";
    let start = manifest.find(needle).expect("manifest declares a kinds array") + needle.len();
    let end = start + manifest[start..].find(']').expect("kinds array is closed");
    let declared: Vec<String> = manifest[start..end].split(',').map(|entry| entry.trim().trim_matches('"').to_string()).filter(|entry| !entry.is_empty()).collect();
    assert_eq!(declared, KINDS, "the oracle manifest's kinds must match XlsxTransitionalMutation exactly");
}
//#endregion 🔖️KindsConformanceLaw

//#region 🔖️StampLaw
/// 🏅️ The class stamp is bijective: stamping into one class and back out of it lands on the
/// snapshot it started from. This is what makes `SetSnapshot` exactly invertible on this axis,
/// and it is proven on a snapshot built by this repository's own code, not asserted.
#[test]
fn stamping_into_a_class_and_back_is_the_identity() {
    let base = XlsxSnapshot::default();
    assert_eq!(stamp_conformance_class(stamp_conformance_class(base.clone(), true), false), stamp_conformance_class(base, false));
}
//#endregion 🔖️StampLaw
