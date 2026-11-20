use super::*;
use std::path::PathBuf;

fn bundled() -> Vec<(String, &'static str)> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../../🗿️artifacts");
    let mut found = std::fs::read_dir(&root)
        .unwrap_or_else(|error| panic!("cannot list bundled artifact definitions in {}: {error}", root.display()))
        .filter_map(Result::ok)
        .map(|entry| entry.path().join("📜️artifact-definition.json"))
        .filter(|path| path.is_file())
        .map(|path| {
            let text: &'static str = Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display())).into_boxed_str());
            (path.parent().and_then(|parent| parent.file_name()).map(|name| name.to_string_lossy().into_owned()).unwrap_or_default(), text)
        })
        .collect::<Vec<_>>();
    found.sort();
    found
}

fn placeholder() {}

fn independent_ownership_violations(directory: &str, text: &str) -> Vec<String> {
    let document: serde_json::Value = serde_json::from_str(text).unwrap_or_else(|error| panic!("{directory}: {error}"));
    let owner = document["id"].as_str().unwrap_or_default();
    let mut found = Vec::new();
    for (collection, categories) in [("mutations", "mutation"), ("inferences", "inference"), ("resources", "resource"), ("conformance_suites", "conformance-suite")] {
        for item in document[collection].as_array().into_iter().flatten() {
            let identity = item["id"].as_str().unwrap_or_default();
            if !identity.starts_with(&format!("{owner}.{categories}.")) {
                found.push(format!("{directory}: {collection} id {identity:?} is not owned by {owner}.{categories}."));
            }
        }
    }
    for item in document["runtime_capabilities"].as_array().into_iter().flatten() {
        let identity = item["id"].as_str().unwrap_or_default();
        if !identity.starts_with(&format!("{owner}.")) && !item["category"].as_str().is_some_and(|category| matches!(category, "codec" | "representation") && identity.starts_with("s.stdio.")) {
            found.push(format!("{directory}: runtime capability {identity:?} is not owned by {owner}"));
        }
    }
    found
}

/// 🗿️ Runs the full assembly over every bundled stdio artifact definition and reports all violations at once.
#[test]
fn every_bundled_artifact_definition_assembles_without_violations() {
    let definitions = bundled();
    assert!(definitions.len() >= 30, "expected the bundled stdio artifact definitions, found {}", definitions.len());
    let mut report = Vec::new();
    for (directory, text) in &definitions {
        match source(text) {
            Err(error) => report.push(format!("{directory}: {}", error.message)),
            Ok(parsed) => {
                let executables = expected_executable_ids(&parsed).into_iter().map(|identity| ArtifactExecutable::from_function_pointer(identity, placeholder as *const ()));
                if let Err(error) = definition_from_schema_with_executables(text, executables) {
                    report.push(format!("{directory}: build: {}", error.message));
                }
            }
        }
        report.extend(independent_ownership_violations(directory, text));
    }
    assert!(report.is_empty(), "{} violation(s) across {} definitions:\n{}", report.len(), definitions.len(), report.join("\n"));
}
