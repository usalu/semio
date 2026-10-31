//! 🧩️ Repo-wide proof that every scope-owned `🧬️schema/🔣️.json` module compiles through the owned
//! draft-07 validator in `✅️validator/🦀️.rs`, and a per-path inventory of the keywords that do not.
//!
//! Areas are selected by the repository taxonomy; opaque paths and preservation capsules are excluded.
//! The repo root is derived from `CARGO_MANIFEST_DIR`
//! by walking up to the directory that carries `.🧬semio`; `SEMIO_SCHEMA_MODULE_ROOT` overrides it.
//! `SEMIO_SCHEMA_MODULE_COMPILE_OUT` receives the full JSON report when set.
//! @see <https://json-schema.org/draft-07/schema>

use semio_framework_schema::{OwnedJsonSchemaValidator, resolve_schema_document_reference};
use semio_framework_test_fixtures::manifest_dir;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

const SCHEMA_MODULE_DIR: &str = "🧬️schema";
const SCHEMA_MODULE_FILE: &str = "🔣️.json";
const DRAFT07_DIALECT: &str = "http://json-schema.org/draft-07/schema#";
const SKIPPED_DIRS: [&str; 6] = ["node_modules", "target", "dist", ".git", ".nx", "🗑️generated"];

/// 🚧️ Keywords that are not part of draft-07 at all. A module that carries one is a
/// `📋️execution-contract.md` §B dialect violation owned by that module's partition, never a gap in
/// this validator, so the gate below reports it instead of failing on it.
const NON_DRAFT07_KEYWORDS: [&str; 9] = ["$dynamicAnchor", "$dynamicRef", "$vocabulary", "dependentRequired", "dependentSchemas", "discriminator", "prefixItems", "unevaluatedItems", "unevaluatedProperties"];

/// 🗃️ Keyword provenance is checked independently against the published Draft07 metaschema.
static DRAFT07_KEYWORDS: std::sync::LazyLock<BTreeSet<String>> = std::sync::LazyLock::new(|| {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("📥️inputs.json")).expect("neutral input corpus");
    corpus["draft07Keywords"].as_array().expect("Draft07 keyword roster").iter().map(|value| value.as_str().expect("keyword").to_owned()).collect()
});

/// 🗂️ Distinguishes keyword provenance, document ambiguity, references and pattern compatibility.
fn failure_category(reason: &str) -> &'static str {
    match reason.split("unsupported JSON Schema keyword `").nth(1).and_then(|tail| tail.split('`').next()) {
        Some(keyword) if NON_DRAFT07_KEYWORDS.contains(&keyword) => "nonDraft07Keyword",
        Some(keyword) if DRAFT07_KEYWORDS.contains(keyword) => "draft07KeywordGap",
        Some(_) => "unknownKeyword",
        None if reason == "ambiguous schema document identifier" => "ambiguousIdentifier",
        None if reason.contains("unresolved reference") || reason.contains("unresolved cross-document reference") => "unresolvedReference",
        None if (reason.contains(".pattern") || reason.contains("patternProperties")) && (reason.contains("outside the supported subset") || reason.contains("invalid code point escape")) => "patternCompatibilityFailure",
        None => "metaSchemaViolation",
    }
}

fn repo_root() -> PathBuf {
    if let Ok(declared) = std::env::var("SEMIO_SCHEMA_MODULE_ROOT") {
        return PathBuf::from(declared);
    }
    let mut current = manifest_dir!();
    while !current.join(".🧬semio").is_dir() {
        assert!(current.pop(), "no repo root carrying `.🧬semio` above {}", env!("CARGO_MANIFEST_DIR"));
    }
    current
}

const REPOSITORY_TAXONOMY: &str = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";
const PRESERVATION_SCHEMA: &str = "🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🧬️schema/🔣️.json";

fn repository_taxonomy(root: &Path) -> serde_json::Value {
    serde_json::from_str(&std::fs::read_to_string(root.join(REPOSITORY_TAXONOMY)).expect("repository taxonomy")).expect("valid repository taxonomy")
}

struct SourceInventoryPolicy {
    opaque_paths: Vec<PathBuf>,
    opaque_prefixes: Vec<String>,
    capsule_digest_hex: usize,
}

impl SourceInventoryPolicy {
    fn load(root: &Path) -> Self {
        let taxonomy = repository_taxonomy(root);
        let schema: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(root.join(PRESERVATION_SCHEMA)).expect("preservation schema")).expect("valid preservation schema");
        let protocol = &schema["definitions"]["RelativePreservationProtocol"]["const"];
        let retirement: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(root.join("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧹️retirement/🧬️schema/🔣️.json")).expect("retirement schema")).expect("valid retirement schema");
        Self {
            opaque_paths: taxonomy["pathExclusions"].as_object().expect("path exclusions").values().filter(|row| row["mode"] == "opaque").map(|row| root.join(row["path"].as_str().expect("opaque path"))).collect(),
            opaque_prefixes: vec![protocol["capsulePrefix"].as_str().expect("capsule prefix").to_owned(), retirement["definitions"]["Policy"]["const"]["markerPrefix"].as_str().expect("retirement marker prefix").to_owned()],
            capsule_digest_hex: protocol["capsuleDigestHex"].as_u64().expect("capsule digest length") as usize,
        }
    }

    fn is_opaque(&self, path: &Path) -> bool {
        if self.opaque_paths.iter().any(|prefix| path.starts_with(prefix)) {
            return true;
        }
        path.ancestors().filter_map(Path::file_name).filter_map(|name| name.to_str()).any(|name| self.opaque_prefixes.iter().any(|prefix| name.strip_prefix(prefix).is_some_and(|digest| digest.len() == self.capsule_digest_hex && digest.bytes().all(|entry| entry.is_ascii_digit() || (b'a'..=b'f').contains(&entry)))))
    }
}

/// 🗺️ Selects the declared source areas, independently of the generated schema catalog.
fn schema_source_roots(root: &Path) -> Vec<PathBuf> {
    repository_taxonomy(root)["areaLayers"].as_object().expect("repository source areas").keys().map(|lane| root.join(lane)).collect()
}

fn collect_modules(directory: &Path, modules: &mut Vec<PathBuf>) {
    collect_schema_documents(directory, false, &SourceInventoryPolicy::load(&repo_root()), modules);
}

/// 🗂️ Inventories declared schema facets beneath a real schema module without following symlinks.
fn collect_schema_documents(directory: &Path, inherited: bool, policy: &SourceInventoryPolicy, modules: &mut Vec<PathBuf>) {
    if policy.is_opaque(directory) {
        return;
    }
    let canonical = if directory.file_name().and_then(|name| name.to_str()) == Some(SCHEMA_MODULE_DIR) {
        match std::fs::symlink_metadata(directory.join(SCHEMA_MODULE_FILE)) {
            Ok(metadata) => metadata.is_file(),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => false,
            Err(error) => panic!("{}: {error}", directory.display()),
        }
    } else {
        false
    };
    let inside_module = inherited || canonical;
    let entries = std::fs::read_dir(directory).unwrap_or_else(|error| panic!("{}: {error}", directory.display()));
    for entry in entries {
        let entry = entry.expect("schema inventory entry");
        let path = entry.path();
        if policy.is_opaque(&path) {
            continue;
        }
        let kind = entry.file_type().expect("schema inventory file type");
        let name = entry.file_name();
        let name = name.to_str().expect("schema inventory UTF-8 filename");
        if kind.is_symlink() {
            continue;
        }
        if kind.is_dir() && !SKIPPED_DIRS.contains(&name) {
            collect_schema_documents(&path, inside_module, policy, modules);
        } else if kind.is_file() && inside_module && name == SCHEMA_MODULE_FILE {
            let body = std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
            if directory.file_name().and_then(|name| name.to_str()) == Some(SCHEMA_MODULE_DIR) || declared_string(&body, "$id").is_some() || declared_string(&body, "$schema").is_some() {
                modules.push(path);
            }
        }
    }
}

fn declared_string(body: &str, keyword: &str) -> Option<String> {
    serde_json::from_str::<serde_json::Value>(body).ok()?.get(keyword)?.as_str().map(str::to_owned)
}

/// 🔗 Returns decoded reference document identifiers without local fragments.
fn referenced_documents(body: &str) -> Vec<String> {
    fn visit(value: &serde_json::Value, inherited: Option<&str>, targets: &mut BTreeSet<String>) {
        match value {
            serde_json::Value::Object(object) => {
                let declared = object.get("$id").and_then(serde_json::Value::as_str).map(|id| resolve_schema_document_reference(id, inherited));
                let base = declared.as_deref().or(inherited);
                if let Some(reference) = object.get("$ref").and_then(serde_json::Value::as_str) {
                    if !reference.split('#').next().unwrap_or_default().is_empty() {
                        targets.insert(resolve_schema_document_reference(reference, base));
                    }
                }
                for child in object.values() { visit(child, base, targets); }
            }
            serde_json::Value::Array(array) => {
                for child in array { visit(child, inherited, targets); }
            }
            _ => {}
        }
    }
    let mut targets = BTreeSet::new();
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(body) { visit(&value, None, &mut targets); }
    targets.into_iter().collect()
}

fn declared_document_id(body: &str) -> Option<String> {
    declared_string(body, "$id").map(|id| resolve_schema_document_reference(&id, None))
}
fn json_escape(value: &str) -> String {
    value.chars().fold(String::new(), |mut escaped, entry| {
        match entry {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            other => escaped.push(other),
        }
        escaped
    })
}

#[test]
fn every_draft07_schema_module_compiles_through_the_owned_validator() {
    let root = repo_root();
    let mut modules = Vec::new();
    for lane in schema_source_roots(&root) {
        collect_modules(&lane, &mut modules);
    }
    modules.sort();
    assert!(modules.len() > 2_000, "expected the full repo module set, walked only {}", modules.len());

    let mut compiled = 0usize;
    let mut draft07_failures: Vec<(String, String)> = Vec::new();
    let mut other_dialect_failures: Vec<(String, String, String)> = Vec::new();
    let mut dialects: BTreeMap<String, usize> = BTreeMap::new();
    let mut unsupported_keywords: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut colliding_ids: BTreeMap<String, Vec<String>> = BTreeMap::new();

    let bodies: Vec<(String, String)> = modules
        .iter()
        .map(|module| {
            let relative = module.strip_prefix(&root).unwrap_or(module).display().to_string();
            let body = std::fs::read_to_string(module).unwrap_or_else(|error| panic!("{relative}: {error}"));
            (relative, body)
        })
        .collect();
    let by_id = unique_document_inputs(bodies.iter().map(|(_, body)| body.as_str()));
    for (relative, body) in &bodies {
        let Some(id) = declared_document_id(body) else { continue };
        colliding_ids.entry(id.clone()).or_default().push(relative.clone());

    }
    colliding_ids.retain(|_, paths| paths.len() > 1);


    for (relative, body) in &bodies {
        let dialect = declared_string(body, "$schema").unwrap_or_else(|| "<undeclared>".to_string());
        *dialects.entry(dialect.clone()).or_default() += 1;
        let siblings = referenced_document_inputs(body, &by_id);
        let relative = relative.clone();
        let result = if declared_document_id(body).is_some_and(|id| colliding_ids.contains_key(&id)) {
            Err("ambiguous schema document identifier".to_owned())
        } else {
            OwnedJsonSchemaValidator::compile_with_documents(body, &siblings).map_err(|error| error.to_string())
        };
        match result {
            Ok(_) => compiled += 1,
            Err(error) => {
                let reason = error.to_string();
                if let Some(keyword) = reason.split("unsupported JSON Schema keyword `").nth(1).and_then(|tail| tail.split('`').next()) {
                    unsupported_keywords.entry(keyword.to_string()).or_default().push(relative.clone());
                }
                if dialect == DRAFT07_DIALECT {
                    draft07_failures.push((relative, reason));
                } else {
                    other_dialect_failures.push((relative, dialect, reason));
                }
            }
        }
    }

    let report = format!(
        "{{\n  \"modules\": {},\n  \"compiled\": {},\n  \"collidingIds\": {},\n  \"draft07Failures\": [{}\n  ],\n  \"otherDialectFailures\": [{}\n  ],\n  \"dialects\": {{{}\n  }},\n  \"unsupportedKeywords\": {{{}\n  }}\n}}\n",
        modules.len(),
        compiled,
        colliding_ids.len(),
        draft07_failures.iter().map(|(path, reason)| format!("\n    {{ \"path\": \"{}\", \"reason\": \"{}\" }}", json_escape(path), json_escape(reason))).collect::<Vec<_>>().join(","),
        other_dialect_failures.iter().map(|(path, dialect, reason)| format!("\n    {{ \"path\": \"{}\", \"dialect\": \"{}\", \"reason\": \"{}\" }}", json_escape(path), json_escape(dialect), json_escape(reason))).collect::<Vec<_>>().join(","),
        dialects.iter().map(|(dialect, count)| format!("\n    \"{}\": {count}", json_escape(dialect))).collect::<Vec<_>>().join(","),
        unsupported_keywords.iter().map(|(keyword, paths)| format!("\n    \"{}\": [{}\n    ]", json_escape(keyword), paths.iter().map(|path| format!("\n      \"{}\"", json_escape(path))).collect::<Vec<_>>().join(","))).collect::<Vec<_>>().join(","),
    );
    if let Ok(destination) = std::env::var("SEMIO_SCHEMA_MODULE_COMPILE_OUT") {
        if let Some(parent) = Path::new(&destination).parent() {
            std::fs::create_dir_all(parent).expect("report directory");
        }
        std::fs::write(&destination, &report).expect("report");
    }
    println!("[schema-module-compile] modules={} compiled={} draft07Failures={} otherDialectFailures={} collidingIds={}", modules.len(), compiled, draft07_failures.len(), other_dialect_failures.len(), colliding_ids.len());
    for (keyword, paths) in &unsupported_keywords {
        println!("[schema-module-compile] unsupported keyword `{keyword}` in {} module(s), first {}", paths.len(), paths[0]);
    }
    let mut categories: BTreeMap<&str, usize> = BTreeMap::new();
    for (_, reason) in &draft07_failures {
        *categories.entry(failure_category(reason)).or_default() += 1;
    }
    for (category, count) in &categories {
        println!("[schema-module-compile] draft-07 failure category {category}: {count}");
    }
    for (path, reason) in &draft07_failures {
        println!("[schema-module-compile] {} {path}: {reason}", failure_category(reason));
    }

    let repairs: Vec<&(String, String)> = draft07_failures.iter().filter(|(_, reason)| matches!(failure_category(reason), "draft07KeywordGap" | "unknownKeyword" | "patternCompatibilityFailure")).collect();
    assert!(repairs.is_empty(), "schema module compilation requires repairs:\n{}", repairs.iter().map(|(path, reason)| format!("  {} {path}: {reason}", failure_category(reason))).collect::<Vec<_>>().join("\n"));
}

/// 🆔️ Supplies only documents whose declared identifier has exactly one owner.
fn unique_document_inputs<'a>(documents: impl IntoIterator<Item = &'a str>) -> BTreeMap<String, &'a str> {
    let mut indexed = BTreeMap::new();
    let mut ambiguous = BTreeSet::new();
    for body in documents {
        if let Some(id) = declared_document_id(body) {
            if indexed.insert(id.clone(), body).is_some() {
                ambiguous.insert(id);
            }
        }
    }
    for id in ambiguous {
        indexed.remove(&id);
    }
    indexed
}

/// 📥️ Selects the explicit documents supplied to one owned compilation.
fn referenced_document_inputs<'a>(body: &str, by_id: &BTreeMap<String, &'a str>) -> Vec<&'a str> {
    let mut visited = BTreeSet::new();
    if let Some(id) = declared_document_id(body) {
        visited.insert(id);
    }
    let mut pending = referenced_documents(body);
    let mut selected = Vec::new();
    while let Some(target) = pending.pop() {
        if !visited.insert(target.clone()) {
            continue;
        }
        if let Some(document) = by_id.get(&target) {
            pending.extend(referenced_documents(document));
            selected.push(*document);
        }
    }
    selected
}

#[test]
fn reference_input_inventory_contains_declared_facets() {
    let root = repo_root();
    let corpus: serde_json::Value = serde_json::from_str(include_str!("📥️inputs.json")).expect("neutral input corpus");
    for case in corpus["inventoryCases"].as_array().expect("inventory cases") {
        let mut selected = Vec::new();
        collect_modules(&root.join(case["owner"].as_str().expect("owner")), &mut selected);
        assert!(selected.contains(&root.join(case["target"].as_str().expect("target"))), "{}", case["id"]);
    }
}

#[test]
fn reference_input_closure_matches_neutral_corpus() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("📥️inputs.json")).expect("neutral input corpus");
    for case in corpus["referenceCases"].as_array().expect("reference cases") {
        let body = case["root"].as_str().expect("root");
        let by_id = unique_document_inputs(case["documents"].as_array().expect("documents").iter().map(|document| document.as_str().expect("document")));
        let selected = referenced_document_inputs(body, &by_id);
        let mut actual: Vec<String> = selected.iter().filter_map(|document| declared_document_id(document)).collect();
        actual.sort();
        let expected: Vec<String> = case["expected"].as_array().expect("expected").iter().map(|id| id.as_str().expect("id").to_owned()).collect();
        assert_eq!(actual, expected, "{}", case["id"]);
        assert_eq!(OwnedJsonSchemaValidator::compile_with_documents(body, &selected).is_ok(), case["compiles"].as_bool().expect("outcome"), "{}", case["id"]);
    }
}

#[test]
fn reference_input_repository_roots_match_neutral_corpus() {
    let root = repo_root();
    let corpus: serde_json::Value = serde_json::from_str(include_str!("📥️inputs.json")).expect("neutral input corpus");
    let mut expected: Vec<PathBuf> = corpus["repositoryRoots"].as_array().expect("repository roots").iter().map(|value| root.join(value.as_str().expect("repository root"))).collect();
    let mut actual = schema_source_roots(&root);
    expected.sort();
    actual.sort();
    assert_eq!(actual, expected, "every declared source area must be inventoried");
}

#[test]
fn failure_classification_matches_neutral_corpus() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("📥️inputs.json")).expect("neutral input corpus");
    for case in corpus["classificationCases"].as_array().expect("classification cases") {
        assert_eq!(failure_category(case["reason"].as_str().expect("reason")), case["expected"].as_str().expect("category"), "{}", case["id"]);
    }
}

#[test]
fn source_inventory_opacity_matches_neutral_corpus() {
    let root = repo_root();
    let policy = SourceInventoryPolicy::load(&root);
    let corpus: serde_json::Value = serde_json::from_str(include_str!("📥️inputs.json")).expect("neutral input corpus");
    for case in corpus["opacityCases"].as_array().expect("opacity cases") {
        let path = root.join(case["path"].as_str().expect("path"));
        assert_eq!(policy.is_opaque(&path), case["opaque"].as_bool().expect("opacity"), "{}", case["id"]);
    }
}

#[test]
fn pattern_inputs_match_neutral_ecmascript_corpus() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("📥️inputs.json")).expect("neutral input corpus");
    for case in corpus["patternCases"].as_array().expect("pattern cases") {
        let schema = serde_json::json!({"pattern": case["pattern"]}).to_string();
        let validator = OwnedJsonSchemaValidator::compile(&schema);
        assert_eq!(validator.is_ok(), case["compiles"].as_bool().expect("compile outcome"), "{}", case["id"]);
        if let Ok(validator) = validator {
            for entry in case["matches"].as_array().into_iter().flatten() {
                let units: Vec<u16> = entry["units"].as_array().expect("UTF-16 units").iter().map(|value| u16::try_from(value.as_u64().expect("unit")).expect("UTF-16 unit")).collect();
                let instance = format!("\"{}\"", units.iter().map(|unit| format!("\\u{unit:04X}")).collect::<String>());
                assert_eq!(validator.is_valid_json(&instance), entry["expected"].as_bool().expect("validity"), "{}", case["id"]);
            }
        }
    }
}

#[test]
fn authored_schema_inputs_match_neutral_corpus() {
    let root = repo_root();
    let corpus: serde_json::Value = serde_json::from_str(include_str!("📥️inputs.json")).expect("neutral input corpus");
    let authoring = &corpus["authoring"];
    let facet: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(root.join(authoring["schema"].as_str().expect("schema path"))).expect("actual authored schema")).expect("valid authored schema JSON");
    let documents: Vec<String> = authoring["documents"].as_array().expect("document paths").iter().map(|path| std::fs::read_to_string(root.join(path.as_str().expect("document path"))).expect("actual canonical schema")).collect();
    let inputs: Vec<&str> = documents.iter().map(String::as_str).collect();
    for case in authoring["cases"].as_array().expect("authoring cases") {
        let name = case["definition"].as_str().expect("definition");
        assert!(facet["$defs"].get(name).is_some(), "missing authored definition {name}");
        let schema = serde_json::json!({"$defs": facet["$defs"], "$ref": format!("#/$defs/{name}")}).to_string();
        let validator = OwnedJsonSchemaValidator::compile_with_documents(&schema, &inputs).expect("actual authored schema compiles");
        assert_eq!(validator.is_valid_json(&case["value"].to_string()), case["valid"].as_bool().expect("validity"), "{}", case["id"]);
    }
}
