//! 🧭️ Format resolution law driven by the language-agnostic `🧫️fixtures/🧭️format-resolution/🔣️.json` fixture.

use semio_framework_os_io::{canonicalize_format_descriptor, pick_revision, RevisionSniff, resolve_format_in, FormatDescriptor, FormatQuery, FormatResolutionBasis, FormatResolutionError};
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../🧫️fixtures/🧭️format-resolution/🔣️.json")).expect("format resolution fixture")
}

fn strings(value: &Value) -> Vec<String> {
    value.as_array().expect("array").iter().map(|item| item.as_str().expect("string").to_string()).collect()
}

fn descriptors(rows: &Value) -> Vec<FormatDescriptor> {
    rows.as_array()
        .expect("descriptor rows")
        .iter()
        .map(|row| FormatDescriptor {
            kind_id: row["kindId"].as_str().unwrap().into(),
            short_id: row["shortId"].as_str().unwrap().into(),
            aliases: strings(&row["aliases"]),
            mimes: strings(&row["mimes"]),
            extensions: strings(&row["extensions"]),
            name: row["shortId"].as_str().unwrap().into(),
            full_name: row["shortId"].as_str().unwrap().into(),
            neutral: true,
            dir_name: row["shortId"].as_str().unwrap().into(),
            is_binary: true,
        })
        .collect()
}

fn query(value: &Value) -> FormatQuery<'_> {
    FormatQuery { explicit: value["explicit"].as_str(), file_name: value["fileName"].as_str(), mime: value["mime"].as_str() }
}

fn basis(name: &str) -> FormatResolutionBasis {
    match name {
        "explicit" => FormatResolutionBasis::Explicit,
        "extension" => FormatResolutionBasis::Extension,
        "mime" => FormatResolutionBasis::Mime,
        other => panic!("unknown basis {other}"),
    }
}

#[test]
fn every_fixture_case_resolves_with_the_frozen_precedence() {
    let fixture = fixture();
    assert_eq!(strings(&fixture["precedence"]), ["explicit", "extension", "mime", "unresolved"]);
    let table = descriptors(&fixture["descriptors"]);
    for case in fixture["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let result = resolve_format_in(&table, &query(&case["query"]));
        if let Some(error) = case["error"].as_str() {
            match error {
                "unknownExplicit" => assert!(matches!(result, Err(FormatResolutionError::UnknownExplicit { .. })), "{id}: {result:?}"),
                "ambiguous" => assert!(matches!(result, Err(FormatResolutionError::Ambiguous { .. })), "{id}: {result:?}"),
                other => panic!("{id}: unknown error {other}"),
            }
            continue;
        }
        let resolved = result.unwrap_or_else(|error| panic!("{id}: {error}"));
        match case["kindId"].as_str() {
            None => assert!(resolved.is_none(), "{id}: {resolved:?}"),
            Some(kind) => {
                let resolved = resolved.unwrap_or_else(|| panic!("{id}: unresolved"));
                assert_eq!(resolved.descriptor.kind_id, kind, "{id}");
                assert_eq!(resolved.basis, basis(case["basis"].as_str().unwrap()), "{id}");
                assert_eq!(resolved.mime_disagrees_with.as_deref(), case["mimeDisagreesWith"].as_str(), "{id}");
            }
        }
    }
}

#[test]
fn single_extension_names_agree_with_the_standard_library_path_oracle() {
    let fixture = fixture();
    let table = descriptors(&fixture["descriptors"]);
    for case in fixture["cases"].as_array().unwrap() {
        let Some(name) = case["query"]["fileName"].as_str().filter(|name| !name.contains(['/', '\\']) && name.matches('.').count() == 1) else { continue };
        let Some(extension) = std::path::Path::new(name).extension().and_then(|extension| extension.to_str()) else { continue };
        let oracle = table.iter().find(|descriptor| descriptor.extensions.contains(&format!(".{}", extension.to_ascii_lowercase())));
        let ours = resolve_format_in(&table, &FormatQuery { file_name: Some(name), ..FormatQuery::default() }).unwrap();
        assert_eq!(ours.map(|resolution| resolution.descriptor.kind_id), oracle.map(|descriptor| descriptor.kind_id.clone()), "{name}");
    }
}

#[test]
fn a_claim_held_by_two_formats_is_refused_instead_of_guessed() {
    let fixture = fixture();
    let table = descriptors(&fixture["ambiguousDescriptors"]);
    let case = &fixture["ambiguousCase"];
    let error = resolve_format_in(&table, &query(&case["query"])).expect_err("ambiguous claim");
    assert_eq!(error, FormatResolutionError::Ambiguous { claim: case["claim"].as_str().unwrap().into(), kind_ids: strings(&case["kindIds"]) });
}

#[test]
fn declared_extension_order_is_the_canonical_export_precedence() {
    let fixture = fixture();
    for row in descriptors(&fixture["descriptors"]) {
        let declared = row.extensions.clone();
        let canonical = canonicalize_format_descriptor(row).expect("fixture descriptor canonicalizes");
        assert_eq!(canonical.extensions, declared, "{} keeps its declared extension order, so the first claim stays the export extension", canonical.kind_id);
    }
}

#[test]
fn a_revision_is_picked_by_its_declared_header_and_otherwise_the_owner_opens() {
    let fixture = fixture();
    let revisions = fixture["revisionSniffs"]["revisions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| RevisionSniff { kind_id: row["kindId"].as_str().unwrap().into(), window: row["window"].as_u64().unwrap() as usize, contains: strings(&row["contains"]) })
        .collect::<Vec<_>>();
    for case in fixture["revisionSniffs"]["cases"].as_array().unwrap() {
        let head = case["head"].as_array().unwrap().iter().map(|byte| byte.as_u64().unwrap() as u8).collect::<Vec<_>>();
        assert_eq!(pick_revision(case["owner"].as_str().unwrap(), &revisions, &head), case["kindId"].as_str().unwrap(), "{}", case["id"]);
    }
}
