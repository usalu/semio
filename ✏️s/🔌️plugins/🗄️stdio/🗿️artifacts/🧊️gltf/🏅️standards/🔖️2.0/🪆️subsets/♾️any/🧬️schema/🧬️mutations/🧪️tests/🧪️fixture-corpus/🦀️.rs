//! 🧫️ The committed language-neutral glTF mutation corpus (`♾️any/🧫️fixtures/🧬️mutations/<entity>/<verb>/<case>/`) against
//! the production wire, the mutation laws and the payload schemas. Every case is asserted by its own canonical implementation
//! case, `<entity>/<verb>/🧪️tests/<case>/🦀️.rs`, mounted by its leaf and calling [`assert_case`]: its `🦠️mutation` is a
//! `GltfMutation` wire fixed point that the aggregate schema admits and whose editable `payload_value()` (the `Apply` content of
//! a `#[mutation_leaf(payload = Apply)]` leaf) validates against its own leaf `input_schema()`; `📸️snapshot/⬅️before` lands on
//! `➡️after` through the committed `🔺️diff` exactly as its `🎯️outcome` declares; the inert `restore` of every computed inverse is
//! admitted by the aggregate through the shared `diff.json` and restores `before`. This module keeps the corpus-wide laws. The
//! same corpus is judged by the third-party validators of `test schema mutation-payloads` (npm `jsonschema`) and by the
//! TypeScript twins beside this file (`./🟦️.ts`, Ajv).

use super::*;
use protocol::{MutationDiff, SemanticMutation};
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

const SNAPSHOT_SCHEMA: &str = include_str!("../../../📸️snapshot/🔣️.json");
const DIFF_SCHEMA: &str = include_str!("../../../🔺️diff/🔣️.json");
const AGGREGATE_SCHEMA: &str = include_str!("../../🔣️.json");

fn subset() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️2.0/🪆️subsets/♾️any")
}

fn corpus() -> PathBuf {
    subset().join("🧫️fixtures/🧬️mutations")
}

fn cases() -> Vec<PathBuf> {
    fn walk(directory: &Path, found: &mut Vec<PathBuf>) {
        if directory.join("🦠️mutation/🔣️.json").is_file() {
            found.push(directory.to_path_buf());
            return;
        }
        for entry in std::fs::read_dir(directory).unwrap_or_else(|error| panic!("{}: {error}", directory.display())) {
            let path = entry.expect("corpus entry").path();
            if path.is_dir() {
                walk(&path, found);
            }
        }
    }
    let mut found = Vec::new();
    walk(&corpus(), &mut found);
    found.sort();
    found
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

fn json(text: &str) -> Value {
    serde_json::from_str(text).expect("committed JSON parses")
}

fn wire<T: dsl::ToValue>(value: &T) -> Value {
    json(&dsl::json::to_json_string(value))
}

/// ⚖️ JSON equality where a number is its value, not its spelling (`2` and `2.0` are one JSON number).
fn same(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => left.as_f64() == right.as_f64(),
        (Value::Array(left), Value::Array(right)) => left.len() == right.len() && left.iter().zip(right).all(|(left, right)| same(left, right)),
        (Value::Object(left), Value::Object(right)) => left.len() == right.len() && left.iter().all(|(key, value)| right.get(key).is_some_and(|other| same(value, other))),
        _ => left == right,
    }
}

fn validator(schema: &str) -> framework_schema::OwnedJsonSchemaValidator {
    framework_schema::OwnedJsonSchemaValidator::compile_with_documents(schema, &[SNAPSHOT_SCHEMA, DIFF_SCHEMA]).unwrap_or_else(|error| panic!("schema compiles: {error:?}"))
}

fn assert_valid(schema: &str, instance: &str, what: &str) {
    if let Err(error) = validator(schema).validate_json(instance) {
        panic!("{what}: {error:?}\n{instance}");
    }
}

/// 🧺️ The aggregate document compiled once beside every leaf schema and the shared snapshot and diff documents.
fn aggregate_validator() -> &'static framework_schema::OwnedJsonSchemaValidator {
    static AGGREGATE: OnceLock<framework_schema::OwnedJsonSchemaValidator> = OnceLock::new();
    AGGREGATE.get_or_init(|| {
        let documents = [SNAPSHOT_SCHEMA, DIFF_SCHEMA].into_iter().chain(<GltfMutation as protocol::Mutation<GltfSnapshot>>::INPUT_SCHEMAS.iter().copied()).collect::<Vec<_>>();
        framework_schema::OwnedJsonSchemaValidator::compile_with_documents(AGGREGATE_SCHEMA, &documents).unwrap_or_else(|error| panic!("aggregate schema compiles: {error:?}"))
    })
}

fn assert_aggregate_admits(instance: &str, what: &str) {
    if let Err(error) = aggregate_validator().validate_json(instance) {
        panic!("{what}: the aggregate schema refuses the wire: {error:?}\n{instance}");
    }
}

fn camel(kind: &str) -> String {
    kind.split('-').enumerate().map(|(index, word)| if index == 0 { word.to_string() } else { word[..1].to_uppercase() + &word[1..] }).collect()
}

fn case_name(case: &Path) -> String {
    case.strip_prefix(corpus()).unwrap_or(case).display().to_string()
}

/// 🧫️ Asserts one committed case, `<entity>/<verb>/<case>` under the corpus, against every corpus law; called by exactly one
/// canonical implementation case per committed fixture bundle.
pub(crate) fn assert_case(name: &str) {
    let case = corpus().join(name);
    assert!(case.join("🦠️mutation/🔣️.json").is_file(), "{name}: no committed case");
    let text = read(&case.join("🦠️mutation/🔣️.json"));
    let mutation: GltfMutation = dsl::json::from_json_str(&text).unwrap_or_else(|error| panic!("{name}: the committed mutation decodes: {error}"));
    assert_committed_wire(name, &text, &mutation);
    if let Some(before) = assert_committed_outcome(name, &case, &mutation) {
        assert_inverse_restores(name, &mutation, &before);
    }
}

/// 🧬️ The committed mutation is the production wire the aggregate admits, and its editable payload is the value its leaf schema describes.
fn assert_committed_wire(name: &str, text: &str, mutation: &GltfMutation) {
    assert!(same(&wire(mutation), &json(text)), "{name}: decode→encode is not a fixed point");
    assert_aggregate_admits(text, name);
    let schema = <GltfMutation as protocol::Mutation<GltfSnapshot>>::input_schema(mutation).unwrap_or_else(|| panic!("{name}: the leaf publishes its payload schema"));
    let payload = <GltfMutation as protocol::Mutation<GltfSnapshot>>::payload_value(mutation);
    let content = &json(text)["payload"];
    let editable = if content.get("phase").is_some() { &content["value"] } else { content };
    assert!(same(&wire(&payload), editable), "{name}: payload_value is the editable content of the aggregate member");
    assert_valid(schema, &dsl::json::to_json_string(&payload), name);
    assert_eq!(&<GltfMutation as protocol::Mutation<GltfSnapshot>>::with_payload_value(mutation, payload).expect("the payload rebuilds its own kind"), mutation, "{name}");
}

/// 🎯️ The case lands `before` on `after` through the committed diff, or refuses with its committed code and no change; an
/// applied case answers its `before`.
fn assert_committed_outcome(name: &str, case: &Path, mutation: &GltfMutation) -> Option<GltfSnapshot> {
    let mut snapshots = Vec::new();
    for side in ["⬅️before", "➡️after"] {
        let text = read(&case.join("📸️snapshot").join(side).join("🔣️.json"));
        assert_valid(SNAPSHOT_SCHEMA, &text, &format!("{name} {side}"));
        let snapshot: GltfSnapshot = dsl::json::from_json_str(&text).unwrap_or_else(|error| panic!("{name} {side}: {error}"));
        assert!(same(&wire(&snapshot), &json(&text)), "{name} {side}: decode→encode is not a fixed point");
        snapshots.push(snapshot);
    }
    let [before, after] = <[GltfSnapshot; 2]>::try_from(snapshots).expect("two sides");
    let outcome = <GltfMutation as protocol::Mutation<GltfSnapshot>>::diff(mutation, &before);
    let declared = json(&read(&case.join("🎯️outcome/🔣️.json")));
    match declared["status"].as_str() {
        Some("applied") => {
            assert!(outcome.messages().is_empty(), "{name}: {:?}", outcome.messages());
            let committed = read(&case.join("🔺️diff/🔣️.json"));
            assert_valid(DIFF_SCHEMA, &committed, &format!("{name} diff"));
            assert!(same(&wire(outcome.diff()), &json(&committed)), "{name}: the produced diff differs from the committed one:\n{}", dsl::json::to_json_string(outcome.diff()));
            let decoded: GltfDiff = dsl::json::from_json_str(&committed).expect("committed diff decodes");
            assert_eq!(decoded.apply(&before).expect("committed diff applies"), after, "{name}: the committed diff does not carry before to after");
            Some(before)
        }
        Some("rejected") => {
            let code = declared["code"].as_str().expect("a rejection names its outcome code");
            let rejection = declared["rejection"].as_str().expect("a rejection names its glTF refusal");
            assert_eq!(crate::schema::modules::mutation_support::top_level::rejection_outcome_code(rejection), code, "{name}: the declared outcome code is the one the glTF refusal maps to");
            assert!(outcome.diff().is_empty_diff(), "{name}: a rejection changes nothing");
            assert_eq!(outcome.messages().iter().map(|message| message.code.0.as_str()).collect::<Vec<_>>(), [code], "{name}");
            assert!(outcome.messages().iter().all(|message| message.message.starts_with(rejection)), "{name}: {:?}", outcome.messages());
            assert!(case.join("🔺️diff/🚫️.absent").is_file(), "{name}: a rejected case commits no diff");
            assert_eq!(after, before, "{name}: a rejected case keeps its snapshot");
            None
        }
        other => panic!("{name}: unknown outcome status {other:?}"),
    }
}

/// ↩️ The computed inverse restores `before`; a `restore` is inert (not editable) and the aggregate admits its wire.
fn assert_inverse_restores(name: &str, mutation: &GltfMutation, before: &GltfSnapshot) {
    let mut restored = <GltfMutation as protocol::Mutation<GltfSnapshot>>::diff(mutation, before).diff().apply(before).expect("forward applies");
    let inverse = <GltfMutation as protocol::Mutation<GltfSnapshot>>::inverse(mutation, before);
    assert!(!inverse.is_empty(), "{name}: an applied change has an inverse");
    for step in inverse {
        let text = dsl::json::to_json_string(&step);
        assert_aggregate_admits(&text, &format!("{name} inverse"));
        let restore = json(&text)["payload"].get("phase").is_some();
        assert_eq!(<GltfMutation as protocol::Mutation<GltfSnapshot>>::input_schema(&step).is_none(), restore, "{name}: only a wrapped leaf's restore is inert");
        let decoded: GltfMutation = dsl::json::from_json_str(&text).expect("inverse wire decodes");
        let outcome = <GltfMutation as protocol::Mutation<GltfSnapshot>>::diff(&decoded, &restored);
        assert!(outcome.messages().is_empty(), "{name}: {:?}", outcome.messages());
        restored = outcome.diff().apply(&restored).expect("restore applies");
    }
    assert_eq!(&restored, before, "{name}: the inverse does not restore before");
}

/// 🗂️ Every leaf owns a committed case, and every case names a leaf of the aggregate.
#[test]
fn every_leaf_kind_owns_a_committed_case() {
    let wires = cases().iter().map(|case| json(&read(&case.join("🦠️mutation/🔣️.json")))["mutation"].as_str().expect("wire names its mutation").to_string()).collect::<std::collections::BTreeSet<_>>();
    let kinds = GltfMutation::kinds().iter().map(|descriptor| camel(descriptor.kind)).collect::<std::collections::BTreeSet<_>>();
    assert_eq!(wires, kinds);
}

/// 🪢️ Every committed case is asserted by exactly its canonical implementation case: `<entity>/<verb>/🧪️tests/<case>/🦀️.rs`,
/// mounted by the leaf and calling [`assert_case`] with that case, so a new fixture bundle can never go unasserted.
#[test]
fn every_committed_case_is_mounted_by_its_leaf_implementation_case() {
    for case in cases() {
        let name = case_name(&case);
        let (leaf, scenario) = name.rsplit_once('/').expect("<entity>/<verb>/<case>");
        let owner = subset().join("🧬️schema/🧬️mutations").join(leaf);
        let implementation = read(&owner.join("🧪️tests").join(scenario).join("🦀️.rs"));
        assert!(read(&owner.join("🦀️.rs")).contains(&format!("#[path = \"🧪️tests/{scenario}/🦀️.rs\"]")), "{name}: the leaf does not mount its implementation case");
        assert!(implementation.contains(&format!("assert_case(\"{name}\")")), "{name}: the implementation case asserts another case");
    }
}

/// 🎛️ The framework reader declares every leaf's inputs, resolving the shared documents by `$id`.
#[test]
fn every_leaf_schema_declares_its_inputs_to_the_framework_reader() {
    let documents = [SNAPSHOT_SCHEMA, DIFF_SCHEMA].map(|text| dsl::os_pack::json::to_dsl_value(&dsl::os_pack::json::parse(text).expect("shared document parses")));
    let resolve = |id: &str| documents.iter().find(|document| document.get("$id").and_then(dsl::DslValue::as_str) == Some(id)).cloned();
    assert_eq!(<GltfMutation as protocol::Mutation<GltfSnapshot>>::INPUT_SCHEMAS.len(), 121);
    for schema in <GltfMutation as protocol::Mutation<GltfSnapshot>>::INPUT_SCHEMAS {
        let title = json(schema)["title"].as_str().unwrap_or_default().to_string();
        let inputs = semio_framework::mutation_input_defs(schema, &resolve).unwrap_or_else(|error| panic!("{title}: {error:?}"));
        assert_eq!(inputs.len(), json(schema)["properties"].as_object().map_or(0, |properties| properties.len()), "{title}: one input per payload member");
    }
}
