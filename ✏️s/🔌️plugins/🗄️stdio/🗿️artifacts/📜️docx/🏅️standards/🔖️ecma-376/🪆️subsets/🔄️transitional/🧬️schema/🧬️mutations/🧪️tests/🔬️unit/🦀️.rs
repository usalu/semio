
use super::*;

//#region 🔖️KindsConformanceLaw
/// 🧭️ `kind_of` is an EXHAUSTIVE match (no wildcard arm) — the compiler refuses this file if a
/// variant is added to `DocxTransitionalMutation` without a matching kebab-case spelling here, which is what keeps
/// `KINDS` honest against the enum. The second half reads the sibling oracle manifest's `kinds`
/// array as text (the framework never parses Rust, so this is the only side that can prove the
/// manifest matches) and asserts the same list, in the same order.
#[test]
fn kinds_match_enum_and_catalog() {
    fn kind_of(mutation: &DocxTransitionalMutation) -> &'static str {
        match mutation {
            DocxTransitionalMutation::SetSnapshot(_) => "set-snapshot",
            DocxTransitionalMutation::SetMainNamespace(_) => "set-main-namespace",
            DocxTransitionalMutation::SetRelationshipBase(_) => "set-relationship-base",
            DocxTransitionalMutation::SetConformanceAttribute(_) => "set-conformance-attribute",
            DocxTransitionalMutation::RemoveConformanceAttribute(_) => "remove-conformance-attribute",
        }
    }
    let samples = [
        DocxTransitionalMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: DocxSnapshot::default() }),
        DocxTransitionalMutation::SetMainNamespace(set_main_namespace::SetMainNamespace { namespace: String::new() }),
        DocxTransitionalMutation::SetRelationshipBase(set_relationship_base::SetRelationshipBase { base: String::new() }),
        DocxTransitionalMutation::SetConformanceAttribute(set_conformance_attribute::SetConformanceAttribute { value: String::new() }),
        DocxTransitionalMutation::RemoveConformanceAttribute(remove_conformance_attribute::RemoveConformanceAttribute {}),
    ];
    let from_enum: Vec<&'static str> = samples.iter().map(kind_of).collect();
    assert_eq!(from_enum, KINDS, "KINDS must list every DocxTransitionalMutation variant, in declaration order");

    let manifest = include_str!("../../../../🔮️oracle/🔣️.json");
    let needle = "\"kinds\": [";
    let start = manifest.find(needle).expect("manifest declares a kinds array") + needle.len();
    let end = start + manifest[start..].find(']').expect("kinds array is closed");
    let declared: Vec<String> = manifest[start..end].split(',').map(|entry| entry.trim().trim_matches('"').to_string()).filter(|entry| !entry.is_empty()).collect();
    assert_eq!(declared, KINDS, "the oracle manifest's kinds must match DocxTransitionalMutation exactly");
}
//#endregion 🔖️KindsConformanceLaw

//#region 🔖️StampLaw
/// 🏅️ The class stamp is bijective: stamping into one class and back out of it lands on the
/// snapshot it started from. This is what makes `SetSnapshot` exactly invertible on this axis,
/// and it is proven on a snapshot built by this repository's own code, not asserted.
#[test]
fn stamping_into_a_class_and_back_is_the_identity() {
    let base = DocxSnapshot::default();
    assert_eq!(stamp_conformance_class(stamp_conformance_class(base.clone(), true), false), stamp_conformance_class(base, false));
}
//#endregion 🔖️StampLaw
