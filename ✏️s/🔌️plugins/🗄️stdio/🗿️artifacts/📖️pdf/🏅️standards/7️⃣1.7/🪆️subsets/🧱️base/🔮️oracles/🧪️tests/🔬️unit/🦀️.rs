
use super::*;
use semio_repo_test_host::parse_json;

/// 🧫️ The real committed document `📑️mutate-pdf-1-7` runs on, read where the artifact already
/// keeps it — a 6.3 MB, 65-page LaTeX bachelor thesis carrying 3,173 indirect objects, a
/// six-entry outline tree and an `/OpenAction` `/GoTo` destination.
const FIXTURE: &[u8] = include_bytes!("../../../🖼️assets/🎓️bachelor-thesis/🎓️bachelor-thesis.pdf");

/// 🥒️ The case this module is the reference of — its Examples rows are the only parameter source here.
const FEATURE: &str = include_str!("../../../🧪️tests/📑️mutate-pdf-1-7/🥒️.feature");

fn json_object(pairs: Vec<(&str, Json)>) -> Json {
    Json::Object(pairs.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
}

fn text(value: &str) -> Json {
    Json::String(value.to_string())
}

/// 🧾️ The leaf wire payload the feature's first Examples row for `kind` carries — the same target against the same real
/// document, read from the feature itself, so a failure here and a failure there have the same cause and the same fix.
fn params_for(kind: &str) -> Json {
    FEATURE
        .lines()
        .find_map(|line| {
            let cells: Vec<&str> = line.split('|').map(str::trim).collect();
            (cells.len() == 4 && cells[1] == kind).then(|| parse_json(cells[2]).unwrap_or_else(|error| panic!("{kind}: the feature row is not JSON: {error}")))
        })
        .unwrap_or_else(|| panic!("the feature declares no Examples row for {kind:?}"))
}

fn spec(kind: &str) -> Json {
    json_object(vec![("kind", text(kind)), ("params", params_for(kind))])
}

fn fixture() -> Vec<u8> {
    FIXTURE.to_vec()
}

/// ⚖️ The two laws `📑️mutate-pdf-1-7`'s adapter asserts in role, proven here against the real
/// document without the runner: every declared kind moves the projection it is compared through
/// (except the one [`UNOBSERVABLE`] names, with its reason), and every declared kind's own
/// computed inverse lands back on the untouched document's projection (with
/// [`regenerates_page_content`]'s single documented axis dropped for its three kinds).
#[test]
fn every_declared_kind_is_observable_and_its_inverse_restores_the_document() {
    let original = fixture();
    let base = project_pdf_1_7(&original).expect("the independent reader projects the real document");
    for kind in KINDS {
        let forward = spec(kind);
        let mutated = oracle_apply_mutation(&original, &forward).unwrap_or_else(|error| panic!("{kind}: {error}"));
        let moved = project_pdf_1_7(&mutated).unwrap_or_else(|error| panic!("{kind}: projecting the result failed: {error}"));
        if !UNOBSERVABLE.contains(kind) {
            assert_ne!(moved, base, "{kind} left the compared projection untouched, so its scenario would pass whether or not the mutation ran");
        }
        let restored = oracle_apply_mutation_inverse(&original, &forward).unwrap_or_else(|error| panic!("{kind}: inverse: {error}"));
        let recovered = project_pdf_1_7(&restored).unwrap_or_else(|error| panic!("{kind}: projecting the restored document failed: {error}"));
        let (expected, actual) = if regenerates_page_content(kind) { (without_content_operators(&base), without_content_operators(&recovered)) } else { (base.clone(), recovered) };
        assert_eq!(actual, expected, "{kind}: applying the mutation and then its own inverse must restore the document's projection");
    }
}

/// 🚫️ The one exemption, pinned rather than merely asserted: `insert-object` is unobservable
/// BECAUSE the real document has nowhere for an unreferenced object to be seen from, not
/// because the projection is thin. The moment the vocabulary grows a linking site — or the
/// fixture grows a dangling reference — this flips red and the exemption has to be re-argued.
#[test]
fn insert_object_is_unobservable_only_because_nothing_can_reference_the_new_object() {
    let original = fixture();
    let base = project_pdf_1_7(&original).expect("the independent reader projects the real document");
    let mutated = oracle_apply_mutation(&original, &spec("insert-object")).expect("the reference inserts the object");
    assert_ne!(mutated, original, "the reference really did rewrite the file");
    assert_eq!(project_pdf_1_7(&mutated).unwrap(), base, "an object nothing references is unreachable (ISO 32000-1 §7.5.4) and must project identically");
}

/// 🕸️ The object-graph surface, checked against the values the real document actually carries,
/// so a future refactor that quietly renders it empty cannot keep the observability test green.
#[test]
fn the_object_graph_surface_reads_the_real_catalog_and_trailer() {
    let projection = project_pdf_1_7(&fixture()).expect("the independent reader projects the real document");
    let graph = projection.get("objectGraph").expect("the projection carries the object-graph surface").clone();
    let trailer = graph.get("trailer").expect("a trailer surface").clone();
    assert_eq!(trailer.get("Root"), Some(&Json::String("<indirect object>".to_string())), "trailer references render opaquely; the catalog has its own member");
    assert!(trailer.get("ID").is_some(), "the real trailer carries the /ID pair remove-trailer-entry targets");
    assert!(trailer.get("Size").is_none(), "/Size is cross-reference bookkeeping the writer recomputes on every save");
    let catalog = graph.get("catalog").expect("a catalog surface").clone();
    assert!(catalog.get("Pages").is_none(), "the page tree is projected by pageCount/pages, never twice");
    assert_eq!(catalog.get("PageMode"), Some(&Json::String("/UseOutlines".to_string())), "set-dict-entry's target axis");
    assert_eq!(catalog.get("Outlines").and_then(|outlines| outlines.get("Count")).cloned(), Some(Json::Number(6.0)), "remove-object #3015 is the outline root the catalog resolves to");
    assert_eq!(catalog.get("OpenAction").and_then(|action| action.get("S")).cloned(), Some(Json::String("/GoTo".to_string())), "set-object-value #145 is the OpenAction the catalog resolves to");
}

/// 🔒️ Both halves of the identity law, on the real document.
#[test]
fn the_round_trip_is_projection_stable_and_not_a_byte_passthrough() {
    let original = fixture();
    let rebuilt = oracle_round_trip(&original).expect("the reference re-serializes the document");
    assert_ne!(rebuilt, original, "the reference rebuilds the file from its own object graph; identical bytes would mean the input was smuggled");
    assert_eq!(project_pdf_1_7(&rebuilt).unwrap(), project_pdf_1_7(&original).unwrap());
}

#[test]
fn unknown_kind_is_an_error_never_a_silent_no_op() {
    let unknown = json_object(vec![("kind", text("not-a-real-kind")), ("params", json_object(vec![]))]);
    assert!(oracle_apply_mutation(&fixture(), &unknown).is_err());
    assert!(oracle_apply_mutation_inverse(&fixture(), &unknown).is_err());
}

/// 📇️ The three declarations that must never drift: this module's [`KINDS`], the catalog in
/// `🔣️.json`, and the `Examples` rows of the case that claims it.
#[test]
fn kinds_matches_the_catalog_and_every_feature_row() {
    let manifest = include_str!("../../🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "the pdf-1-7-base catalog is missing {kind:?}");
        assert!(FEATURE.contains(&format!("| {kind} ")) || FEATURE.contains(&format!("| {kind}\n")), "the feature declares no Examples row for {kind:?}");
    }
    assert_eq!(KINDS.len(), 16, "the pdf-1-7-base vocabulary declares sixteen direct kinds");
}
