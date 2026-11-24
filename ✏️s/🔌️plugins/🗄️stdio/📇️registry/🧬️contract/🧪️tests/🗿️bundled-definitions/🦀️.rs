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

fn plugin_definitions() -> Vec<(String, String)> {
    let plugins = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../../..");
    let mut found = Vec::new();
    for plugin in ["🗄️stdio", "📕️norm"] {
        let root = plugins.join(plugin).join("🗿️artifacts");
        for entry in std::fs::read_dir(&root).unwrap_or_else(|error| panic!("cannot list artifact definitions in {}: {error}", root.display())).filter_map(Result::ok) {
            let path = entry.path().join("📜️artifact-definition.json");
            if path.is_file() {
                let text = std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
                found.push((format!("{plugin}/🗿️artifacts/{}/📜️artifact-definition.json", entry.file_name().to_string_lossy()), text));
            }
        }
    }
    found.sort();
    found
}

fn framed_dialects(value: &str) -> Option<Vec<semio_plugin::app::ArtifactDialect>> {
    let mut rest = value;
    let mut found = Vec::new();
    for _ in 0..2 {
        let (length, tail) = rest.split_once(':')?;
        let coordinate = tail.get(..length.parse::<usize>().ok()?)?;
        let (artifact_kind, remainder) = coordinate.split_once('@')?;
        let (standard, subset) = remainder.rsplit_once('/')?;
        found.push(semio_plugin::app::ArtifactDialect { artifact_kind: artifact_kind.into(), standard: standard.into(), subset: subset.into() });
        rest = &tail[coordinate.len()..];
    }
    rest.is_empty().then_some(found)
}

fn runtime_claim_violations(file: &str, text: &str) -> Vec<String> {
    let document: serde_json::Value = serde_json::from_str(text).unwrap_or_else(|error| panic!("{file}: {error}"));
    let mut found = Vec::new();
    for capability in document["runtime_capabilities"].as_array().into_iter().flatten() {
        let identity = capability["id"].as_str().unwrap_or_default();
        let claims = capability["claims"].as_array().into_iter().flatten().collect::<Vec<_>>();
        let value_of = |namespace: &str| claims.iter().find(|claim| claim["namespace"] == namespace).and_then(|claim| claim["value"].as_str());
        let mut check = |namespace: &str, declared: &str, rebuilt: Result<ArtifactIdentityClaim, ArtifactDefinitionError>| match rebuilt {
            Err(error) => found.push(format!("{file}: {identity}: {namespace} claim {declared:?} cannot be rebuilt: {error}")),
            Ok(runtime) => {
                if runtime.value() != declared {
                    found.push(format!("{file}: {identity}: {namespace} claim declares {declared:?} but the runtime constructor produces {:?}", runtime.value()));
                }
                let descriptor = capability["descriptor"].as_str().unwrap_or_default();
                if !descriptor.contains(&format!("{namespace}:{}", runtime.value())) {
                    found.push(format!("{file}: {identity}: descriptor {descriptor:?} does not carry the runtime {namespace} claim {:?}", runtime.value()));
                }
            }
        };
        if let Some(declared) = value_of("codec-extension") {
            let schema = value_of("codec").unwrap_or_default();
            let extension = identity.rsplit_once("-extension-").and_then(|(_, tail)| tail.strip_suffix(".v1")).unwrap_or_default();
            check("codec-extension", declared, ArtifactIdentityClaim::codec_extension(schema, extension));
        }
        if let Some(declared) = value_of("composition-route") {
            match framed_dialects(declared) {
                Some(endpoints) => check("composition-route", declared, ArtifactIdentityClaim::composition_route(&endpoints[0], &endpoints[1])),
                None => found.push(format!("{file}: {identity}: composition-route claim {declared:?} is not two length-framed dialect coordinates")),
            }
        }
    }
    found
}

/// 🪪 Every declared codec-extension and composition-route claim equals what the runtime constructors rebuild from the definition's own data.
#[test]
fn every_declared_codec_extension_and_composition_route_claim_matches_the_runtime_constructors() {
    let definitions = plugin_definitions();
    assert!(definitions.len() >= 45, "expected the bundled stdio and norm artifact definitions, found {}", definitions.len());
    let report = definitions.iter().flat_map(|(file, text)| runtime_claim_violations(file, text)).collect::<Vec<_>>();
    assert!(report.is_empty(), "{} claim violation(s) across {} definitions:\n{}", report.len(), definitions.len(), report.join("\n"));
}

/// 🚫 The retired semio form with an owner prefix and the matching length frame is rejected, naming file and claim.
#[test]
fn the_retired_owner_prefixed_semio_codec_extension_claim_is_rejected() {
    let file = "🗄️stdio/🗿️artifacts/🧿️semio/📜️artifact-definition.json";
    let (_, text) = plugin_definitions().into_iter().find(|(name, _)| name == file).expect("semio definition is bundled");
    assert!(text.contains("21:stdio.semio.animation:semio"));
    let regressed = text.replace("21:stdio.semio.animation:semio", "23:s.stdio.semio.animation:semio");
    let report = runtime_claim_violations(file, &regressed);
    assert!(report.iter().any(|line| line.contains(file) && line.contains("codec-extension") && line.contains("23:s.stdio.semio.animation:semio") && line.contains("21:stdio.semio.animation:semio")), "retired form was not rejected: {report:?}");
}
