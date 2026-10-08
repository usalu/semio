
use super::*;
use semio_repo_test_host::law::feature_rows;

/// 🧫️ The real committed package `🧱️mutate-pptx-ecma-376` runs on — the seven-slide subset derived
/// once from a real 62-slide conference deck, with real titles, real placeholders and real
/// `a:xfrm` geometry in EMUs.
const FIXTURE: &[u8] = include_bytes!("../../../🧫️fixtures/📽️.pptx");

/// 🪢️ The four kinds that write the package plumbing (`[Content_Types].xml`, the `*.rels` parts); the slide/shape projection does not reach them, so they are proven by
/// the cases below through `project_pptx_plumbing`.
const PLUMBING_KINDS: [&str; 4] = ["set-relationship", "remove-relationship", "set-content-type", "remove-content-type"];

/// 🧾️ The case's own `Examples` rows, read rather than restated — see [`semio_repo_test_host::law::feature_rows`].
const FEATURE: &str = include_str!("../../../🧪️tests/🧱️mutate-pptx-ecma-376/🥒️.feature");

fn spec(kind: &str, params: &Json) -> Json {
    Json::Object(vec![("kind".to_string(), Json::String(kind.to_string())), ("params".to_string(), params.clone())])
}

/// ⚖️ The two laws `🧱️mutate-pptx-ecma-376`'s adapter asserts in role, proven here against the real
/// deck without the runner: every declared kind moves the ordered slide/shape projection, and
/// every declared kind's own computed inverse lands back on the untouched deck's projection.
/// Nothing is exempt from either — every one of the nine kinds edits the slide list or a shape inside
/// it, which is precisely what the projection reports.
#[test]
fn every_declared_kind_is_observable_and_its_inverse_restores_the_presentation() {
    let base = project_pptx_mutation(FIXTURE).expect("the independent reader projects the real deck");
    let rows = feature_rows(FEATURE);
    assert_eq!(rows.len(), KINDS.len() - PLUMBING_KINDS.len(), "the feature must carry exactly one Examples row per declared kind but the plumbing kinds proven below");
    for (kind, params) in &rows {
        assert!(KINDS.contains(&kind.as_str()), "the feature exercises {kind:?}, which the pptx-ecma-376-base catalog does not declare");
        let forward = spec(kind, params);
        let mutated = oracle_apply_mutation(FIXTURE, &forward).unwrap_or_else(|error| panic!("{kind}: {error}"));
        let moved = project_pptx_mutation(&mutated).unwrap_or_else(|error| panic!("{kind}: projecting the result failed: {error}"));
        assert_ne!(moved, base, "{kind} left the compared projection untouched, so its scenario would pass whether or not the mutation ran");
        let restored = oracle_apply_mutation_inverse(FIXTURE, &forward).unwrap_or_else(|error| panic!("{kind}: inverse: {error}"));
        assert_eq!(project_pptx_mutation(&restored).unwrap(), base, "{kind}: applying the mutation and then its own inverse must restore the deck's projection");
    }
}

/// 🔒️ Both halves of the identity law, on the real deck: `oracle_round_trip` unzips, parses each slide,
/// regenerates every slide-related OPC part from the typed slide/shape list and rezips.
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
    assert_eq!(KINDS.len(), 12, "PptxMutation declares twelve kinds");
}

fn object(entries: Vec<(&str, Json)>) -> Json {
    Json::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
}

fn text(value: &str) -> Json {
    Json::String(value.to_string())
}

/// 🧱️ The two members of an ordered plumbing pair.
fn pair(row: &Json) -> (String, String) {
    match row {
        Json::Array(items) => match items.as_slice() {
            [Json::String(left), Json::String(right)] => (left.clone(), right.clone()),
            other => panic!("a plumbing pair carries two strings, got {other:?}"),
        },
        other => panic!("a plumbing pair is an array, got {other:?}"),
    }
}

/// 🧱️ The ids of one owner's ordered relationship rows in a projection.
fn owner_rows(plumbing: &Json, owner: &str) -> Vec<String> {
    plumbing.get("relationships").map(|owners| owners.array(owner).iter().map(|row| row.str("id")).collect()).unwrap_or_default()
}

/// 🪢️ The four plumbing kinds on the real deck: each moves the plumbing, lands where its position says, and its own computed inverse
/// restores the deck's projection exactly -- positions included.
#[test]
fn every_plumbing_kind_is_observable_places_its_row_and_its_inverse_restores_the_package() {
    let base = project_pptx_plumbing(FIXTURE).expect("the independent reader projects the real deck's plumbing");
    let plumbing = &base;
    let first_root = owner_rows(&base, "").first().cloned().expect("the package root owns a relationship");
    let (first_override, _) = pair(&plumbing.array("overrides")[0]);
    let hyperlink = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink";
    let cases = [
        spec("set-relationship", &object(vec![("owner", text("")), ("id", text("rIdOracleLink")), ("relType", text(hyperlink)), ("target", text("https://example.invalid/oracle")), ("external", Json::Bool(true)), ("index", Json::Number(0.0))])),
        spec("set-relationship", &object(vec![("owner", text("")), ("id", text("rIdOracleLink")), ("relType", text(hyperlink)), ("target", text("https://example.invalid/oracle")), ("external", Json::Bool(true))])),
        spec("remove-relationship", &object(vec![("owner", text("")), ("id", text(&first_root))])),
        spec("set-content-type", &object(vec![("isOverride", Json::Bool(false)), ("name", text("zzoracle")), ("contentType", text("application/x-oracle")), ("index", Json::Number(0.0))])),
        spec("set-content-type", &object(vec![("isOverride", Json::Bool(true)), ("name", text(&first_override)), ("contentType", text("application/x-oracle"))])),
        spec("remove-content-type", &object(vec![("isOverride", Json::Bool(true)), ("name", text(&first_override))])),
    ];
    for forward in &cases {
        let kind = forward.str("kind");
        assert!(PLUMBING_KINDS.contains(&kind.as_str()), "{kind} is not a plumbing kind");
        let mutated = oracle_apply_mutation(FIXTURE, forward).unwrap_or_else(|error| panic!("{kind}: {error}"));
        assert_ne!(project_pptx_plumbing(&mutated).unwrap(), base, "{kind} left the compared plumbing untouched");
        let restored = oracle_apply_mutation_inverse(FIXTURE, forward).unwrap_or_else(|error| panic!("{kind}: inverse: {error}"));
        assert_eq!(project_pptx_plumbing(&restored).unwrap(), base, "{kind}: the inverse must restore the plumbing, positions included");
    }
    let placed = project_pptx_plumbing(&oracle_apply_mutation(FIXTURE, &cases[0]).unwrap()).unwrap();
    assert_eq!(owner_rows(&placed, "").first().map(String::as_str), Some("rIdOracleLink"), "index 0 puts the relationship first");
    let appended = project_pptx_plumbing(&oracle_apply_mutation(FIXTURE, &cases[1]).unwrap()).unwrap();
    assert_eq!(owner_rows(&appended, "").last().map(String::as_str), Some("rIdOracleLink"), "no index appends the relationship");
}

/// 🚫️ A plumbing kind that targets nothing is an error, never a silent no-op.
#[test]
fn plumbing_kinds_refuse_a_missing_target() {
    let missing_relationship = spec("remove-relationship", &object(vec![("owner", text("")), ("id", text("rIdNothing"))]));
    let missing_owner = spec("remove-relationship", &object(vec![("owner", text("ppt/nothing.xml")), ("id", text("rId1"))]));
    let missing_entry = spec("remove-content-type", &object(vec![("isOverride", Json::Bool(false)), ("name", text("nothing"))]));
    for forward in [missing_relationship, missing_owner, missing_entry] {
        assert!(oracle_apply_mutation(FIXTURE, &forward).is_err(), "{} must refuse", forward.str("kind"));
    }
}
