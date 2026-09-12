
use super::*;
use crate::law::feature_rows;

/// 🧫️ The real committed package `🧱️mutate-pptx-ecma-376` runs on — the seven-slide subset derived
/// once from a real 62-slide conference deck, with real titles, real placeholders and real
/// `a:xfrm` geometry in EMUs.
const FIXTURE: &[u8] = include_bytes!("../../../🧫️fixtures/📽️.pptx");

/// 🧾️ The case's own `Examples` rows, read rather than restated — see [`crate::law::feature_rows`].
const FEATURE: &str = include_str!("../../../🧪️tests/🧱️mutate-pptx-ecma-376/🥒️.feature");

fn spec(kind: &str, params: &Json) -> Json {
    Json::Object(vec![("kind".to_string(), Json::String(kind.to_string())), ("params".to_string(), params.clone())])
}

/// ⚖️ The two laws `🧱️mutate-pptx-ecma-376`'s adapter asserts in role, proven here against the real
/// deck without the runner: every declared kind moves the ordered slide/shape projection, and
/// every declared kind's own computed inverse lands back on the untouched deck's projection.
/// Nothing is exempt from either — every one of the nine kinds is defined on the slide list or
/// on a shape inside it, which is precisely what the projection reports.
#[test]
fn every_declared_kind_is_observable_and_its_inverse_restores_the_presentation() {
    let base = project_pptx_mutation(FIXTURE).expect("the independent reader projects the real deck");
    let rows = feature_rows(FEATURE);
    assert_eq!(rows.len(), KINDS.len(), "the feature must carry exactly one Examples row per declared kind");
    for (kind, params) in &rows {
        assert!(KINDS.contains(&kind.as_str()), "the feature exercises {kind:?}, which the pptx-ecma-376-base catalog does not declare");
        let forward = spec(kind, params);
        let mutated = oracle_apply_mutation(FIXTURE, &forward).unwrap_or_else(|error| panic!("{kind}: {error}"));
        let moved = project_pptx_mutation(&mutated).unwrap_or_else(|error| panic!("{kind}: projecting the result failed: {error}"));
        if kind != "no-mutation" {
            assert_ne!(moved, base, "{kind} left the compared projection untouched, so its scenario would pass whether or not the mutation ran");
        }
        let restored = oracle_apply_mutation_inverse(FIXTURE, &forward).unwrap_or_else(|error| panic!("{kind}: inverse: {error}"));
        assert_eq!(project_pptx_mutation(&restored).unwrap(), base, "{kind}: applying the mutation and then its own inverse must restore the deck's projection");
    }
}

/// 🔒️ Both halves of the identity law, on the real deck. `oracle_round_trip` is the same rebuild
/// path every kind takes: unzip, parse each slide, regenerate every slide-related OPC part from
/// the typed slide/shape list, rezip.
#[test]
fn the_round_trip_is_projection_stable_and_not_a_byte_passthrough() {
    let rebuilt = oracle_round_trip(FIXTURE).expect("the reference re-serializes the deck");
    assert_ne!(rebuilt.as_slice(), FIXTURE, "the slide parts and the archive are both rebuilt from the parsed model; identical bytes would mean the input was smuggled");
    assert_eq!(project_pptx_mutation(&rebuilt).unwrap(), project_pptx_mutation(FIXTURE).unwrap());
}

#[test]
fn unknown_kind_is_an_error_never_a_silent_no_op() {
    let unknown = spec("not-a-real-kind", &Json::Object(Vec::new()));
    assert!(oracle_apply_mutation(FIXTURE, &unknown).is_err());
    assert!(oracle_apply_mutation_inverse(FIXTURE, &unknown).is_err());
    assert!(oracle_apply_mutation(FIXTURE, &Json::Object(vec![("params".to_string(), Json::Object(Vec::new()))])).is_err(), "a spec with no kind at all is an error too");
}

/// 📇️ [`KINDS`] against the catalog that declares it.
#[test]
fn kinds_matches_the_catalog() {
    let manifest = include_str!("../../🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "the pptx-ecma-376-base catalog is missing {kind:?}");
    }
    assert_eq!(KINDS.len(), 9, "PptxMutation declares nine kinds");
}
