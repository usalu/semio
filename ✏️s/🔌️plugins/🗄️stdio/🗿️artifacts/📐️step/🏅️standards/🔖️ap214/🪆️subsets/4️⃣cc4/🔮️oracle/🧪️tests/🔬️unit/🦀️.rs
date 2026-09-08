
use super::*;

fn fixture() -> Vec<u8> {
    include_bytes!("../../../🧫️fixtures/🌲️hexagonal-cut-concrete-forest-left-ap214/📐️.stp").to_vec()
}

fn object(entries: Vec<(&str, Json)>) -> Json {
    Json::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
}

fn spec(kind: &str, params: Json) -> Json {
    object(vec![("kind", Json::String(kind.to_string())), ("params", params)])
}

fn project(bytes: &[u8]) -> Result<Json, String> {
    project_step_ap214_cc4(bytes)
}

/// 🧫️ What this class actually sees in the real committed export, read off the file rather than
/// assumed — this is the number every scenario's observability depends on.
#[test]
fn the_real_export_reads_the_way_this_class_predicts() {
    let before = project(&fixture()).unwrap();
    assert_eq!(before.get("aboveCeiling"), Some(&Json::Number(1.0)), "the rung-6 #13 is two rungs above CC4");
    assert_eq!(before.get("conformsToClass"), Some(&Json::Bool(false)));
    assert_eq!(before.get("hasProductChain"), Some(&Json::Bool(true)), "the formation rung is the ISO 10303-41 subtype a real exporter writes");
    assert_eq!(before.get("fileSchema").unwrap().clone(), Json::Array(vec![Json::String("AUTOMOTIVE_DESIGN".to_string())]));
}

/// ⚖️ Every declared kind must MOVE this class's projection, and its own inverse must put it
/// back exactly. A kind that leaves the projection where it was is a scenario that proves
/// nothing, so the observability half is asserted here as well as in the case adapter.
#[test]
fn every_kind_is_observable_and_its_own_inverse_restores_the_projection() {
    let input = fixture();
    let original = project(&input).unwrap();
    for case in exercised_specs() {
        let kind = case.str("kind");
        let mutated = oracle_apply_mutation(&input, &case).unwrap_or_else(|error| panic!("{kind} failed: {error}"));
        let after = project(&mutated).unwrap();
        if kind != "no-mutation" {
            assert_ne!(after, original, "{kind} left the conformance projection unchanged -- a mutation that is not observable proves nothing");
        }
        let inverse = oracle_inverse_spec(&input, &case).unwrap();
        let restored = oracle_apply_mutation(&mutated, &inverse).unwrap_or_else(|error| panic!("{kind} inverse failed: {error}"));
        assert_eq!(project(&restored).unwrap(), original, "applying {kind} and then its own inverse must restore the original projection");
    }
}

/// 📇️ One spec per declared kind, with parameters chosen against the REAL fixture's own content:
/// `#13` is a real representation, `#827`/`#822`/`#821` are the real product chain.
fn exercised_specs() -> Vec<Json> {
    vec![
        spec("no-mutation", Json::Object(Vec::new())),
        spec(
            "set-snapshot",
            object(vec![
                ("fileSchema", Json::Array(vec![Json::String("AUTOMOTIVE_DESIGN".to_string())])),
                (
                    "productIdentity",
                    object(vec![
                        ("product", Json::Number(1.0)),
                        ("productName", Json::String("Document".to_string())),
                        ("formation", Json::Number(2.0)),
                        ("formationId", Json::String("A".to_string())),
                        ("definition", Json::Number(3.0)),
                        ("definitionId", Json::String("A".to_string())),
                    ]),
                ),
            ]),
        ),
        spec("set-file-schema", object(vec![("schemas", Json::Array(vec![Json::String("CONFIG_CONTROL_DESIGN".to_string())]))])),
        spec("set-product-identity", object(vec![("identity", Json::Null)])),
        spec(
            "set-shape-representation",
            object(vec![
                ("id", Json::Number(836.0)),
                (
                    "representation",
                    object(vec![
                        ("typeName", Json::String("MANIFOLD_SURFACE_SHAPE_REPRESENTATION".to_string())),
                        ("name", Json::String("Document".to_string())),
                        ("items", Json::Array(vec![Json::Number(837.0), Json::Number(895.0)])),
                        ("context", Json::Number(835.0)),
                    ]),
                ),
            ]),
        ),
        spec("demote-shape-representation", object(vec![("id", Json::Number(13.0))])),
    ]
}

#[test]
fn the_round_trip_reparses_rather_than_copies() {
    let input = fixture();
    let output = oracle_round_trip(&input).unwrap();
    assert_ne!(output, input, "ISO 10303-21 clear text is regenerated from the parsed model, so identical bytes would mean the input was copied");
    assert_eq!(project(&output).unwrap(), project(&input).unwrap());
}

#[test]
fn a_ladder_edit_refuses_an_entity_that_is_not_on_the_ladder() {
    let refusal = oracle_apply_mutation(&fixture(), &spec("set-shape-representation", object(vec![("id", Json::Number(827.0)), ("representation", Json::Null)]))).unwrap_err();
    assert!(refusal.contains("never an arbitrary entity"), "a conformance repair must never delete a product record: {refusal}");
}

#[test]
fn an_unknown_kind_is_an_error_not_a_silent_no_op() {
    assert!(oracle_apply_mutation(&fixture(), &spec("insert-entity", Json::Object(Vec::new()))).is_err(), "the Part-21 grammar verbs belong to the 🧱️base subset, not to a conformance class");
}

/// 🏷️ `KINDS` must equal the committed catalog AND the committed vocabulary. The framework never
/// parses Rust, so this reads both files as text and fails the moment they drift apart.
#[test]
fn kinds_match_the_catalog_and_the_vocabulary() {
    let manifest = include_str!("../../🔣️.json");
    let vocabulary = include_str!("../../../🧬️schema/🧬️mutations/🦀️.rs");
    let feature = include_str!("../../../🧪️tests/🔬️4-mutate-step-ap214-cc4/🥒️.feature");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "the catalog is missing kind {kind:?}");
        assert!(vocabulary.contains(&format!("\"{kind}\"")), "StepCc4Mutation::KINDS is missing {kind:?}");
        assert!(feature.contains(&format!("| {kind} ")), "the case's Examples table is missing kind {kind:?}");
    }
    assert_eq!(KINDS.len(), 6);
}
