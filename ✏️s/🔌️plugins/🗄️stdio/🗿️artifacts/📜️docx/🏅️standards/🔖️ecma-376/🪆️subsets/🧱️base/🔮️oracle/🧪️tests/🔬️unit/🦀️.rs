
use super::*;
use crate::law::feature_rows;

/// 🧫️ The real committed package `📜️mutate-docx-ecma-376` runs on — the WordprocessingML document
/// derived once from this repository's own `README.md`: 414 top-level body blocks, a real 37-row
/// `w:tbl`, seven declared styles and seven OPC parts.
const FIXTURE: &[u8] = include_bytes!("../../../🧫️fixtures/📜️example-readme.docx");

/// 🧾️ The case's own `Examples` rows, read rather than restated — see [`crate::law::feature_rows`].
const FEATURE: &str = include_str!("../../../🧪️tests/📜️mutate-docx-ecma-376/🥒️.feature");

fn spec(kind: &str, params: &Json) -> Json {
    Json::Object(vec![("kind".to_string(), Json::String(kind.to_string())), ("params".to_string(), params.clone())])
}

/// ⚖️ The two laws `📜️mutate-docx-ecma-376`'s adapter asserts in role, proven here against the
/// real package without the runner: every declared kind moves the projection it is compared
/// through, and every declared kind's own computed inverse lands back on the untouched
/// package's projection. Nothing is exempt from either — a DOCX carries its whole typed view in
/// `word/document.xml` and `word/styles.xml`, and the OPC parts the typed view does not model
/// are projected by content-type and digest, so all thirteen kinds reach the surface.
#[test]
fn every_declared_kind_is_observable_and_its_inverse_restores_the_document() {
    let base = project_docx_ecma_376(FIXTURE).expect("the independent reader projects the real package");
    let rows = feature_rows(FEATURE);
    assert_eq!(rows.len(), KINDS.len(), "the feature must carry exactly one Examples row per declared kind");
    for (kind, params) in &rows {
        assert!(KINDS.contains(&kind.as_str()), "the feature exercises {kind:?}, which the docx-ecma-376-any catalog does not declare");
        let forward = spec(kind, params);
        let mutated = oracle_apply_mutation(FIXTURE, &forward).unwrap_or_else(|error| panic!("{kind}: {error}"));
        let moved = project_docx_ecma_376(&mutated).unwrap_or_else(|error| panic!("{kind}: projecting the result failed: {error}"));
        if kind != "no-mutation" {
            assert_ne!(moved, base, "{kind} left the compared projection untouched, so its scenario would pass whether or not the mutation ran");
        }
        let restored = oracle_apply_mutation_inverse(FIXTURE, &forward).unwrap_or_else(|error| panic!("{kind}: inverse: {error}"));
        assert_eq!(project_docx_ecma_376(&restored).unwrap(), base, "{kind}: applying the mutation and then its own inverse must restore the package's projection");
    }
}

/// 🚫️ The one inverse this vocabulary genuinely cannot express, refused rather than faked.
/// `DocxMutation::InsertStyle` carries a style and APPENDS, so removing an INTERIOR style can
/// never be undone — the oracle rejects the request outright instead of returning an undo that
/// leaves `Heading1` where `Title` was. The Examples row removes the LAST style, which append
/// genuinely restores.
#[test]
fn removing_an_interior_style_is_refused_because_append_cannot_put_it_back() {
    let interior = spec("remove-style", &Json::Object(vec![("id".to_string(), Json::String("Title".to_string()))]));
    assert!(oracle_apply_mutation_inverse(FIXTURE, &interior).is_err(), "Title is the second of seven declared styles; no declared kind can reinsert it there");
    let last = spec("remove-style", &Json::Object(vec![("id".to_string(), Json::String("TableCell".to_string()))]));
    assert!(oracle_apply_mutation_inverse(FIXTURE, &last).is_ok(), "TableCell is the last declared style, so append restores it exactly");
}

/// 🔒️ Both halves of the identity law, on the real package.
#[test]
fn the_round_trip_is_projection_stable_and_not_a_byte_passthrough() {
    let rebuilt = oracle_apply_mutation(FIXTURE, &spec("no-mutation", &Json::Object(Vec::new()))).expect("the reference re-serializes the package");
    assert_ne!(rebuilt.as_slice(), FIXTURE, "zip+quick-xml rebuild the archive and every part from their own trees; identical bytes would mean the input was smuggled");
    assert_eq!(project_docx_ecma_376(&rebuilt).unwrap(), project_docx_ecma_376(FIXTURE).unwrap());
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
        assert!(manifest.contains(&format!("\"{kind}\"")), "the docx-ecma-376-any catalog is missing {kind:?}");
    }
    assert_eq!(KINDS.len(), 12, "DocxMutation declares twelve kinds");
}
