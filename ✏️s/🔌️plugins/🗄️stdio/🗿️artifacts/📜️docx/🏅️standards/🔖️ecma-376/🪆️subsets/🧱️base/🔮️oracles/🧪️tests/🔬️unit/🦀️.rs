
use super::*;
use semio_repo_test_host::law::feature_rows;

/// 🧫️ The real committed package `📜️mutate-docx-ecma-376` runs on — the WordprocessingML document
/// derived once from this repository's own `README.md`: 414 top-level body blocks, a real 37-row
/// `w:tbl`, seven declared styles and seven OPC parts.
const FIXTURE: &[u8] = include_bytes!("../../../🧫️fixtures/📜️example-readme.docx");

/// 🪢️ The four kinds that write the package plumbing (`[Content_Types].xml`, the `*.rels` parts). The jszip judge compares the typed body/styles and the other parts
/// only, so these are proven by the cases below -- explicit specs on the real package, read back through [`project_docx_ecma_376`]'s `plumbing` member -- and by the
/// `third-party-generated` evidence pairs of the manifest.
const PLUMBING_KINDS: [&str; 4] = ["set-relationship", "remove-relationship", "set-content-type", "remove-content-type"];

/// 🧾️ The case's own `Examples` rows, read rather than restated — see [`semio_repo_test_host::law::feature_rows`].
const FEATURE: &str = include_str!("../../../🧪️tests/📜️mutate-docx-ecma-376/🥒️.feature");

fn spec(kind: &str, params: &Json) -> Json {
    Json::Object(vec![("kind".to_string(), Json::String(kind.to_string())), ("params".to_string(), params.clone())])
}

/// ⚖️ The two laws `📜️mutate-docx-ecma-376`'s adapter asserts in role, proven here against the
/// real package without the runner: every declared kind moves the projection it is compared
/// through, and every declared kind's own computed inverse lands back on the untouched
/// package's projection. Nothing is exempt from either — a DOCX carries its whole typed view in
/// `word/document.xml` and `word/styles.xml`, and the OPC parts the typed view does not model
/// are projected by content-type and digest, so all fourteen kinds reach the surface.
#[test]
fn every_declared_kind_is_observable_and_its_inverse_restores_the_document() {
    let base = project_docx_ecma_376(FIXTURE).expect("the independent reader projects the real package");
    let rows = feature_rows(FEATURE);
    assert_eq!(rows.len(), KINDS.len() - 1 - PLUMBING_KINDS.len(), "one Examples row per declared kind but `replace-xml-node` (no reference model of an arbitrary XML node) and the plumbing kinds proven below");
    assert!(rows.iter().all(|(kind, _)| kind != "replace-xml-node"), "replace-xml-node carries no Examples row");
    for (kind, params) in &rows {
        assert!(KINDS.contains(&kind.as_str()), "the feature exercises {kind:?}, which the docx-ecma-376-any catalog does not declare");
        let forward = spec(kind, params);
        let mutated = oracle_apply_mutation(FIXTURE, &forward).unwrap_or_else(|error| panic!("{kind}: {error}"));
        let moved = project_docx_ecma_376(&mutated).unwrap_or_else(|error| panic!("{kind}: projecting the result failed: {error}"));
        assert_ne!(moved, base, "{kind} left the compared projection untouched, so its scenario would pass whether or not the mutation ran");
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
    let rebuilt = oracle_round_trip(FIXTURE).expect("the reference re-serializes the package");
    assert_ne!(rebuilt.as_slice(), FIXTURE, "zip+quick-xml rebuild the archive and every part from their own trees; identical bytes would mean the input was smuggled");
    assert_eq!(project_docx_ecma_376(&rebuilt).unwrap(), project_docx_ecma_376(FIXTURE).unwrap());
}

/// 🗜️ The real package with every entry `edit` returns re-stated and every other entry carried as it is.
fn repackaged(edit: impl Fn(&str, Vec<u8>) -> Vec<u8>) -> Vec<u8> {
    use std::io::{Read, Write};
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(FIXTURE)).unwrap();
    let mut writer = zip::ZipWriter::new(std::io::Cursor::new(Vec::<u8>::new()));
    let options: zip::write::FileOptions<'_, ()> = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    for index in 0..archive.len() {
        let mut entry = archive.by_index(index).unwrap();
        let name = entry.name().to_string();
        let mut bytes = Vec::new();
        entry.read_to_end(&mut bytes).unwrap();
        writer.start_file(name.as_str(), options).unwrap();
        writer.write_all(&edit(&name, bytes)).unwrap();
    }
    writer.finish().unwrap().into_inner()
}

/// 🔤️ One entry's text with `from` replaced by `to` once.
fn replaced(bytes: Vec<u8>, from: &str, to: &str) -> Vec<u8> {
    let text = String::from_utf8(bytes).unwrap();
    assert!(text.contains(from), "the real package carries {from:?}");
    text.replacen(from, to, 1).into_bytes()
}

/// 🌿️ An XML-bearing part is compared by its logical content: laid out the way this repository's own XML writer
/// materializes it (LF after the declaration, a start tag wrapped between attributes) the package projects exactly as
/// the committed one does, while a content change inside the same part still moves the projection.
#[test]
fn an_xml_part_is_compared_by_its_content_never_its_layout() {
    let base = project_docx_ecma_376(FIXTURE).unwrap();
    let relaid = repackaged(|name, bytes| if name == "docProps/core.xml" { replaced(replaced(bytes, "?>\r\n", "?>\n"), " xmlns:dc=", "\n    xmlns:dc=") } else { bytes });
    assert_eq!(project_docx_ecma_376(&relaid).unwrap(), base, "writer layout is not document content");
    let edited = repackaged(|name, bytes| if name == "docProps/app.xml" { replaced(bytes, "semio-e2e-wave7-derivation", "another-application") } else { bytes });
    assert_ne!(project_docx_ecma_376(&edited).unwrap(), base, "a content change in an XML part is observed");
}

/// 🔘️ Run flags are ECMA-376 ST_OnOff toggles: `<w:b w:val="0"/>` switches bold off exactly as leaving `<w:b/>` out does.
#[test]
fn a_run_flag_switched_off_by_its_value_reads_as_off() {
    let base = project_docx_ecma_376(FIXTURE).unwrap();
    let switched = repackaged(|name, bytes| if name == "word/document.xml" { replaced(bytes, "<w:rPr><w:b/></w:rPr>", "<w:rPr><w:b w:val=\"0\"/></w:rPr>") } else { bytes });
    let dropped = repackaged(|name, bytes| if name == "word/document.xml" { replaced(bytes, "<w:rPr><w:b/></w:rPr>", "<w:rPr></w:rPr>") } else { bytes });
    assert_ne!(project_docx_ecma_376(&dropped).unwrap(), base, "dropping the bold toggle is observed");
    assert_eq!(project_docx_ecma_376(&switched).unwrap(), project_docx_ecma_376(&dropped).unwrap(), "w:val=\"0\" reads as not bold");
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
    assert_eq!(KINDS.len(), 16, "the docx-ecma-376-any catalog declares sixteen kinds");
}

/// 🔤️ A JSON object from `(key, value)` pairs.
fn object(entries: Vec<(&str, Json)>) -> Json {
    Json::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
}

fn text(value: &str) -> Json {
    Json::String(value.to_string())
}

/// 🧱️ The ids of one owner's ordered plumbing rows in a projection.
fn owner_rows(projection: &Json, owner: &str) -> Vec<String> {
    let owners = projection.get("plumbing").and_then(|plumbing| plumbing.get("relationships")).cloned().unwrap_or(Json::Null);
    owners.array(owner).iter().map(|row| row.str("id")).collect()
}

/// 🪢️ The four plumbing kinds on the real package: each moves the projection through its `plumbing` member, lands where its position says, and its own computed
/// inverse restores the package's projection exactly -- including the position an entry held.
#[test]
fn every_plumbing_kind_is_observable_places_its_row_and_its_inverse_restores_the_document() {
    let base = project_docx_ecma_376(FIXTURE).expect("the independent reader projects the real package");
    let hyperlink = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink";
    let cases = [
        spec("set-relationship", &object(vec![("owner", text("")), ("id", text("rIdOracleLink")), ("relType", text(hyperlink)), ("target", text("https://example.invalid/oracle")), ("external", Json::Bool(true)), ("index", Json::Number(0.0))])),
        spec("set-relationship", &object(vec![("owner", text("word/document.xml")), ("id", text("rIdOracleLink")), ("relType", text(hyperlink)), ("target", text("https://example.invalid/oracle")), ("external", Json::Bool(true))])),
        spec("remove-relationship", &object(vec![("owner", text("")), ("id", text("rId2"))])),
        spec("set-content-type", &object(vec![("isOverride", Json::Bool(false)), ("name", text("zzoracle")), ("contentType", text("application/x-oracle")), ("index", Json::Number(0.0))])),
        spec("set-content-type", &object(vec![("isOverride", Json::Bool(true)), ("name", text("/docProps/core.xml")), ("contentType", text("application/x-oracle"))])),
        spec("remove-content-type", &object(vec![("isOverride", Json::Bool(true)), ("name", text("/docProps/core.xml"))])),
    ];
    for forward in &cases {
        let kind = forward.str("kind");
        assert!(PLUMBING_KINDS.contains(&kind.as_str()), "{kind} is not a plumbing kind");
        let mutated = oracle_apply_mutation(FIXTURE, forward).unwrap_or_else(|error| panic!("{kind}: {error}"));
        let moved = project_docx_ecma_376(&mutated).unwrap();
        assert_ne!(moved, base, "{kind} left the compared projection untouched");
        let restored = oracle_apply_mutation_inverse(FIXTURE, forward).unwrap_or_else(|error| panic!("{kind}: inverse: {error}"));
        assert_eq!(project_docx_ecma_376(&restored).unwrap(), base, "{kind}: the inverse must restore the plumbing, positions included");
    }
    let placed = project_docx_ecma_376(&oracle_apply_mutation(FIXTURE, &cases[0]).unwrap()).unwrap();
    assert_eq!(owner_rows(&placed, "").first().map(String::as_str), Some("rIdOracleLink"), "index 0 puts the relationship first");
    let appended = project_docx_ecma_376(&oracle_apply_mutation(FIXTURE, &cases[1]).unwrap()).unwrap();
    assert_eq!(owner_rows(&appended, "word/document.xml").last().map(String::as_str), Some("rIdOracleLink"), "no index appends the relationship");
    let dropped = project_docx_ecma_376(&oracle_apply_mutation(FIXTURE, &cases[2]).unwrap()).unwrap();
    assert!(!owner_rows(&dropped, "").contains(&"rId2".to_string()), "the relationship is gone");
}

/// 🚫️ A plumbing kind that targets nothing is an error, never a silent no-op.
#[test]
fn plumbing_kinds_refuse_a_missing_target() {
    let missing_relationship = spec("remove-relationship", &object(vec![("owner", text("")), ("id", text("rIdNothing"))]));
    let missing_owner = spec("remove-relationship", &object(vec![("owner", text("word/nothing.xml")), ("id", text("rId1"))]));
    let missing_entry = spec("remove-content-type", &object(vec![("isOverride", Json::Bool(false)), ("name", text("nothing"))]));
    for forward in [missing_relationship, missing_owner, missing_entry] {
        assert!(oracle_apply_mutation(FIXTURE, &forward).is_err(), "{} must refuse", forward.str("kind"));
    }
}
