//! 🧭️ Every selected stdio artifact resolves from its natural file name and MIME type to exactly itself, driven by
//! the language-agnostic `🧫️fixtures/🧭️format-resolution/🔣️.json` fixture.

use semio_os_kernel::io::{resolve_format_in, FormatDescriptor, FormatQuery, FormatResolutionBasis};
use serde_json::Value;
use std::collections::BTreeSet;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../🧫️fixtures/🧭️format-resolution/🔣️.json")).expect("format resolution fixture")
}

fn descriptors() -> Vec<FormatDescriptor> {
    semio_hub_stdio_catalog::format_descriptors().expect("selected stdio formats")
}

fn artifact_of(descriptor: &FormatDescriptor) -> String {
    descriptor.kind_id.strip_prefix("s.stdio.").and_then(|rest| rest.split('.').next()).expect("stdio kind id").to_string()
}

fn query(value: &Value) -> FormatQuery<'_> {
    FormatQuery { explicit: value["explicit"].as_str(), file_name: value["fileName"].as_str(), mime: value["mime"].as_str() }
}

#[test]
fn every_roster_artifact_publishes_a_format_and_pending_artifacts_are_the_only_exceptions() {
    let fixture = fixture();
    let published = descriptors().iter().map(artifact_of).collect::<BTreeSet<_>>();
    let roster = fixture["rosterArtifacts"].as_array().unwrap().iter().map(|name| name.as_str().unwrap().to_string()).collect::<BTreeSet<_>>();
    let pending = fixture["pendingArtifacts"].as_array().unwrap().iter().map(|name| name.as_str().unwrap().to_string()).collect::<BTreeSet<_>>();
    let missing = roster.difference(&published).collect::<Vec<_>>();
    assert!(missing.is_empty(), "roster artifacts without a published format descriptor: {missing:?}");
    let undeclared = published.difference(&roster).filter(|name| !pending.contains(*name)).collect::<Vec<_>>();
    assert!(undeclared.is_empty(), "published artifacts outside the roster: {undeclared:?}");
}

#[test]
fn every_declared_extension_and_mime_resolves_to_its_own_artifact() {
    let table = descriptors();
    for case in fixture()["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let expected = case["artifact"].as_str().unwrap();
        let resolved = resolve_format_in(&table, &query(&case["query"])).unwrap_or_else(|error| panic!("{id}: {error}"));
        match resolved {
            None => assert!(case["pending"].as_bool().unwrap(), "{id}: unresolved but its artifact is published"),
            Some(resolution) => {
                assert_eq!(artifact_of(&resolution.descriptor), expected, "{id}");
                let basis = if case["basis"] == "mime" { FormatResolutionBasis::Mime } else { FormatResolutionBasis::Extension };
                assert_eq!(resolution.basis, basis, "{id}");
            }
        }
    }
}

#[test]
fn well_known_ambiguities_resolve_to_the_natural_owner() {
    let table = descriptors();
    for case in fixture()["wellKnown"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let resolution = resolve_format_in(&table, &query(&case["query"])).unwrap_or_else(|error| panic!("{id}: {error}")).unwrap_or_else(|| panic!("{id}: unresolved"));
        assert_eq!(artifact_of(&resolution.descriptor), case["artifact"].as_str().unwrap(), "{id}");
    }
}

#[test]
fn the_first_declared_extension_is_the_canonical_export_extension() {
    let table = descriptors();
    let fixture = fixture();
    for (artifact, extension) in fixture["canonicalExportExtension"].as_object().unwrap() {
        let descriptor = table.iter().find(|descriptor| &artifact_of(descriptor) == artifact).unwrap_or_else(|| panic!("{artifact} has no format"));
        assert_eq!(descriptor.extensions.first().map(String::as_str), extension.as_str(), "{artifact}");
    }
}

#[test]
fn no_extension_or_mime_is_claimed_by_two_artifacts() {
    let table = descriptors();
    let mut extensions: std::collections::BTreeMap<&str, BTreeSet<String>> = Default::default();
    let mut mimes: std::collections::BTreeMap<&str, BTreeSet<String>> = Default::default();
    for descriptor in &table {
        for extension in &descriptor.extensions {
            extensions.entry(extension).or_default().insert(artifact_of(descriptor));
        }
        for mime in &descriptor.mimes {
            mimes.entry(mime).or_default().insert(artifact_of(descriptor));
        }
    }
    let shared = extensions.iter().chain(mimes.iter()).filter(|(_, owners)| owners.len() > 1).collect::<Vec<_>>();
    assert!(shared.is_empty(), "claims held by several artifacts: {shared:?}");
}

#[test]
fn the_generated_shell_format_claims_equal_the_published_formats() {
    let claims: Value = serde_json::from_str(include_str!("../../../../../✏️s/🧑‍💻dev/🚀️entry/🧭️format-catalog/🤖️generated/stdio-format-claims.json")).expect("generated format claims");
    let strings = |value: &Value| value.as_array().unwrap().iter().map(|item| item.as_str().unwrap().to_string()).collect::<Vec<_>>();
    let table = descriptors();
    for row in claims["formats"].as_array().unwrap() {
        let kind = row["kindId"].as_str().unwrap();
        let published = table.iter().find(|descriptor| descriptor.kind_id == kind).unwrap_or_else(|| panic!("{kind} is not published by the registry"));
        assert_eq!(published.short_id, row["shortId"].as_str().unwrap(), "{kind}");
        assert_eq!(published.extensions, strings(&row["extensions"]), "{kind} keeps its declared extension order");
        let mut mimes = published.mimes.clone();
        mimes.sort();
        assert_eq!(mimes, strings(&row["mimes"]), "{kind}");
        assert!(row["artifactRef"].as_str().unwrap().starts_with(&format!("s.stdio.{}@", artifact_of(published))), "{kind}");
    }
    let claimed = claims["formats"].as_array().unwrap().iter().map(|row| row["kindId"].as_str().unwrap().to_string()).collect::<BTreeSet<_>>();
    let roster = fixture()["rosterArtifacts"].as_array().unwrap().iter().map(|name| name.as_str().unwrap().to_string()).collect::<BTreeSet<_>>();
    let covered = table.iter().filter(|descriptor| claimed.contains(&descriptor.kind_id)).map(artifact_of).collect::<BTreeSet<_>>();
    assert_eq!(covered, roster, "every roster artifact has a shell-routable format");
}

#[test]
fn a_multi_revision_artifact_resolves_to_its_declared_owner_revision() {
    let table = descriptors();
    let claims: Value = serde_json::from_str(include_str!("../../../../../✏️s/🧑‍💻dev/🚀️entry/🧭️format-catalog/🤖️generated/stdio-format-claims.json")).unwrap();
    for (artifact, expected) in fixture()["ownerCoordinates"].as_object().unwrap() {
        let extension = table.iter().find(|descriptor| &artifact_of(descriptor) == artifact).and_then(|descriptor| descriptor.extensions.first()).unwrap_or_else(|| panic!("{artifact} has no format"));
        let name = format!("sample{extension}");
        let resolution = resolve_format_in(&table, &FormatQuery { file_name: Some(&name), ..FormatQuery::default() }).unwrap().unwrap_or_else(|| panic!("{artifact} unresolved"));
        assert!(resolution.descriptor.kind_id.contains(expected["kindIdContains"].as_str().unwrap()), "{artifact} resolves to {}", resolution.descriptor.kind_id);
        let row = claims["formats"].as_array().unwrap().iter().find(|row| row["kindId"] == resolution.descriptor.kind_id.as_str()).unwrap_or_else(|| panic!("{artifact} has no shell route"));
        assert_eq!(row["artifactRef"].as_str(), expected["coordinate"].as_str(), "{artifact}");
    }
}
