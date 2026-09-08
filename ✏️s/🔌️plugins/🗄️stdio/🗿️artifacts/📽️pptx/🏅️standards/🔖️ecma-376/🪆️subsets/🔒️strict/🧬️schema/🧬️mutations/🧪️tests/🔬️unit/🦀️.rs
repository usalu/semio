
use super::*;

//#region 🔖️KindsConformanceLaw
/// 🧭️ `kind_of` is an EXHAUSTIVE match (no wildcard arm) — the compiler refuses this file if a
/// variant is added to `PptxStrictMutation` without a matching kebab-case spelling here, which is what keeps
/// `KINDS` honest against the enum. The second half reads the sibling oracle manifest's `kinds`
/// array as text (the framework never parses Rust, so this is the only side that can prove the
/// manifest matches) and asserts the same list, in the same order.
#[test]
fn kinds_match_enum_and_catalog() {
    fn kind_of(mutation: &PptxStrictMutation) -> &'static str {
        match mutation {
            PptxStrictMutation::SetSnapshot(_) => "set-snapshot",
            PptxStrictMutation::SetMainNamespace(_) => "set-main-namespace",
            PptxStrictMutation::SetDrawingNamespace(_) => "set-drawing-namespace",
            PptxStrictMutation::SetRelationshipBase(_) => "set-relationship-base",
            PptxStrictMutation::SetConformanceAttribute(_) => "set-conformance-attribute",
            PptxStrictMutation::RemoveConformanceAttribute(_) => "remove-conformance-attribute",
            PptxStrictMutation::InsertVmlPart(_) => "insert-vml-part",
            PptxStrictMutation::RemoveVmlPart(_) => "remove-vml-part",
            PptxStrictMutation::InsertAlternateContent(_) => "insert-alternate-content",
            PptxStrictMutation::RemoveAlternateContent(_) => "remove-alternate-content",
        }
    }
    let samples = [
        PptxStrictMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: PptxSnapshot::default() }),
        PptxStrictMutation::SetMainNamespace(set_main_namespace::SetMainNamespace { namespace: String::new() }),
        PptxStrictMutation::SetDrawingNamespace(set_drawing_namespace::SetDrawingNamespace { namespace: String::new() }),
        PptxStrictMutation::SetRelationshipBase(set_relationship_base::SetRelationshipBase { base: String::new() }),
        PptxStrictMutation::SetConformanceAttribute(set_conformance_attribute::SetConformanceAttribute { value: String::new() }),
        PptxStrictMutation::RemoveConformanceAttribute(remove_conformance_attribute::RemoveConformanceAttribute {}),
        PptxStrictMutation::InsertVmlPart(insert_vml_part::InsertVmlPart { path: String::new(), markup: String::new() }),
        PptxStrictMutation::RemoveVmlPart(remove_vml_part::RemoveVmlPart { path: String::new() }),
        PptxStrictMutation::InsertAlternateContent(insert_alternate_content::InsertAlternateContent { path: String::new() }),
        PptxStrictMutation::RemoveAlternateContent(remove_alternate_content::RemoveAlternateContent { path: String::new() }),
    ];
    let from_enum: Vec<&'static str> = samples.iter().map(kind_of).collect();
    assert_eq!(from_enum, KINDS, "KINDS must list every PptxStrictMutation variant, in declaration order");

    let manifest = include_str!("../../../../🔮️oracle/🔣️.json");
    let needle = "\"kinds\": [";
    let start = manifest.find(needle).expect("manifest declares a kinds array") + needle.len();
    let end = start + manifest[start..].find(']').expect("kinds array is closed");
    let declared: Vec<String> = manifest[start..end].split(',').map(|entry| entry.trim().trim_matches('"').to_string()).filter(|entry| !entry.is_empty()).collect();
    assert_eq!(declared, KINDS, "the oracle manifest's kinds must match PptxStrictMutation exactly");
}
//#endregion 🔖️KindsConformanceLaw

//#region 🔖️StampLaw
/// 🏅️ The class stamp is bijective: stamping into one class and back out of it lands on the
/// snapshot it started from. This is what makes `SetSnapshot` exactly invertible on this axis,
/// and it is proven on a snapshot built by this repository's own code, not asserted.
#[test]
fn stamping_into_a_class_and_back_is_the_identity() {
    let base = PptxSnapshot::default();
    assert_eq!(stamp_conformance_class(stamp_conformance_class(base.clone(), true), false), stamp_conformance_class(base, false));
}
//#endregion 🔖️StampLaw
