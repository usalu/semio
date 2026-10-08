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
            DocxTransitionalMutation::SetMainNamespace(_) => "set-main-namespace",
            DocxTransitionalMutation::SetRelationshipBase(_) => "set-relationship-base",
            DocxTransitionalMutation::SetConformanceAttribute(_) => "set-conformance-attribute",
            DocxTransitionalMutation::RemoveConformanceAttribute(_) => "remove-conformance-attribute",
        }
    }
    let samples = [
        DocxTransitionalMutation::SetMainNamespace(set_main_namespace::SetMainNamespace { namespace: String::new() }),
        DocxTransitionalMutation::SetRelationshipBase(set_relationship_base::SetRelationshipBase { base: String::new() }),
        DocxTransitionalMutation::SetConformanceAttribute(set_conformance_attribute::SetConformanceAttribute { value: String::new() }),
        DocxTransitionalMutation::RemoveConformanceAttribute(remove_conformance_attribute::RemoveConformanceAttribute {}),
    ];
    let from_enum: Vec<&'static str> = samples.iter().map(kind_of).collect();
    assert_eq!(from_enum, KINDS, "KINDS must list every DocxTransitionalMutation variant, in declaration order");

    let manifest = include_str!("../../../../🔮️oracles/🔣️.json");
    let needle = "\"kinds\": [";
    let start = manifest.find(needle).expect("manifest declares a kinds array") + needle.len();
    let end = start + manifest[start..].find(']').expect("kinds array is closed");
    let declared: Vec<String> = manifest[start..end].split(',').map(|entry| entry.trim().trim_matches('"').to_string()).filter(|entry| !entry.is_empty()).collect();
    assert_eq!(declared, KINDS, "the oracle manifest's kinds must match DocxTransitionalMutation exactly");
}
//#endregion 🔖️KindsConformanceLaw

//#region 🔖️StampLaw
/// 🏅️ The class stamp is bijective: moving a package out of a class and back into it with the concrete stamp mutations lands on the
/// snapshot it started from, which is what makes each stamp mutation exactly invertible on its axis.
#[test]
fn stamping_out_of_a_class_and_back_is_the_identity() {
    use semio_framework_plugin::ArtifactBuilder;
    let started = crate::standards::v_ecma_376::subsets::transitional::io::DocxTransitionalBuilderConstruction::empty().add_text_paragraph("clean").build().unwrap();
    let mut state = started.clone();
    for mutation in stamp_conformance_class_mutations(true) {
        assert!(apply_docx_transitional_mutation(&mut state, &mutation).messages().is_empty());
    }
    for mutation in stamp_conformance_class_mutations(false) {
        assert!(apply_docx_transitional_mutation(&mut state, &mutation).messages().is_empty());
    }
    assert_eq!(state, started);
}
//#endregion 🔖️StampLaw
