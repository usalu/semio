//! 🦀️ Semio VALUE exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-
//! REFACTOR. Recorded no-oracle decision `semio-value-mutation-semantics` (`../../🏅️standards/
//! 🔖️v1/🪆️subsets/✳️value/🧪️oracle/🔣️component.json`, which also records the `json` 0.12 candidate
//! and why it is not reachable this wave): `s.stdio.semio.value` is a semio-NATIVE format with no
//! third-party reader or writer, so `oracle` here reads committed specification vectors —
//! `set-snapshot`'s from its own leaf under `🧬️mutations/📄set-snapshot/🧪️tests/`, the other eight
//! from this case's own `🧫️fixtures/` — literally, through the host's `Context::fixture_json`, with
//! no recomputation and no reimplementation of mutation semantics. `subject` drives this
//! repository's own `apply_semio_value_mutation`/`inverse_semio_value_mutation` over the full 9-kind
//! `SemioValueMutation` vocabulary. Both sides project to structural JSON and `ordered-json-v1`
//! compares them.
//!
//! The oracle-only build must never link the subject crate (fleet brief §5.3), so the subject module
//! below carries its own small, forward-only, hand-written JSON decoder turning the SAME fixture
//! bytes into real `SemioValueSnapshot`/`SemioValueMutation` values — a mechanical structural
//! decode, recursive because `SemioValue` is, but never a reimplementation of mutation semantics and
//! never a hand-transcribed Rust-literal COPY that could silently drift from the committed file. The
//! generated test-host crate carries no `serde_json` dependency, so the decoder is built on the
//! framework's own dependency-free `protocol::Json`. The subject half is gated behind the generated
//! host's `sut` feature so the oracle-only run never compiles the local implementation; the Rust
//! SUBJECT phase is blocked this wave by a concurrent os-kernel refactor, so it is written and gated
//! but not run.

use semio_repo_test_host::{Adapter, Context, Outcome};

//#region 🔖️Kinds
/// 🏷️ Mirrors `SemioValueMutation::KINDS` (`../../🏅️standards/🔖️v1/🪆️subsets/✳️value/🧬️schema/
/// 🧬️mutations/🦀️component.rs`) — duplicated, not imported, because the oracle-only build must not
/// link the subject crate. The contract's mutation-coverage gate keeps this list honest against the
/// catalog; `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against
/// the enum.
const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-value", "set-map-entry", "remove-map-entry", "insert-list-item", "remove-list-item", "set-node", "remove-node"];

/// 🌱 The one kind that owns a committed leaf of its own; every other kind's vector lives beside
/// this case because its leaf directory does not exist in the taxonomy.
const LEAF_KIND: &str = "set-snapshot";
const LEAF_DIR: &str = "🏅️standards/🔖️v1/🪆️subsets/✳️value/🧬️schema/🧬️mutations/📄set-snapshot/🧪️tests/retypes-a-map-member-and-repoints-a-graph-node";
//#endregion 🔖️Kinds

//#region 🔖️OracleFixtures
/// 🗂️ The SAME URI strings `component.feature` declares, rebuilt here so both roles read one set of
/// committed bytes. `no-mutation` is the identity, so its after-state IS the before-document.
fn before_uri(kind: &str) -> String {
    if kind == LEAF_KIND {
        format!("asset://{LEAF_DIR}/📸️snapshot/⬅️before/🔣️component.json")
    } else {
        "local://⬅️before.json".to_string()
    }
}
fn mutation_uri(kind: &str) -> String {
    if kind == LEAF_KIND {
        format!("asset://{LEAF_DIR}/🦠️mutation/🔣️component.json")
    } else {
        format!("local://{kind}.mutation.json")
    }
}
fn after_uri(kind: &str) -> String {
    match kind {
        LEAF_KIND => format!("asset://{LEAF_DIR}/📸️snapshot/➡️after/🔣️component.json"),
        "no-mutation" => "local://⬅️before.json".to_string(),
        other => format!("local://{other}.after.json"),
    }
}
//#endregion 🔖️OracleFixtures

//#region 🔖️Oracle
/// 🔮️ The forward reference answer: the committed AFTER snapshot, read literally through the host.
fn mutate_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |ctx: &Context| {
        let after = ctx.fixture_json(&after_uri(kind))?;
        let bytes = after.to_string().into_bytes();
        Ok(Outcome::with_raw(bytes, after))
    }
}

/// 🔮️ The inverse reference answer: the committed BEFORE snapshot — undoing any mutation must
/// return to exactly where the specification vector started, member order included.
fn inverse_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |ctx: &Context| {
        let before = ctx.fixture_json(&before_uri(kind))?;
        let bytes = before.to_string().into_bytes();
        Ok(Outcome::with_raw(bytes, before))
    }
}

/// 🔮️ The completeness reference answer: rebuilding the committed document from an empty snapshot
/// must land on that same committed document.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let before = ctx.fixture_json(&before_uri("no-mutation"))?;
    let bytes = before.to_string().into_bytes();
    Ok(Outcome::with_raw(bytes, before))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{before_uri, mutation_uri};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::mutations::{apply_semio_value_mutation, inverse_semio_value_mutation, SemioValueMutation, SemioValuePath, SemioValuePathSegment};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueEntry, SemioValueNode, SemioValueSnapshot, ValueId};

    //#region 🔖️Decode
    /// 🧫️ A small, forward-only, hand-written structural decoder — turns the fixture bytes
    /// `Context::fixture_json` reads STRAIGHT from the committed file into real
    /// `SemioValueSnapshot`/`SemioValueMutation` values. It decodes JSON STRUCTURE only, field by
    /// field, mirroring each payload's own declared serde shape; it never invents or reimplements
    /// any mutation SEMANTICS, which still run through the real entry points below.
    fn usize_field(json: &Json, key: &str) -> usize {
        match json.get(key) {
            Some(Json::Number(value)) => *value as usize,
            other => panic!("mutate-semio-value: expected a numeric field {key:?}, found {other:?}"),
        }
    }
    fn bytes_field(json: &Json, key: &str) -> Vec<u8> {
        json.array(key)
            .iter()
            .map(|entry| match entry {
                Json::Number(value) => *value as u8,
                other => panic!("mutate-semio-value: expected a byte number, found {other:?}"),
            })
            .collect()
    }
    fn decode_id(json: &Json) -> ValueId {
        ValueId::new(json.str("value"))
    }
    /// 🌳 `SemioValue` is internally tagged on `kind` with camelCase variant names and is
    /// recursive through `list`/`map`, so the decoder is too.
    fn decode_value(json: &Json) -> SemioValue {
        match json.str("kind").as_str() {
            "null" => SemioValue::Null,
            "bool" => SemioValue::Bool { value: matches!(json.get("value"), Some(Json::Bool(true))) },
            "int" => SemioValue::Int { lexeme: json.str("lexeme") },
            "float" => SemioValue::Float { lexeme: json.str("lexeme") },
            "str" => SemioValue::Str { value: json.str("value") },
            "bytes" => SemioValue::Bytes { value: bytes_field(json, "value") },
            "list" => SemioValue::List { items: json.array("items").iter().map(decode_value).collect() },
            "map" => SemioValue::Map { entries: json.array("entries").iter().map(decode_entry).collect() },
            "ref" => SemioValue::Ref { id: decode_id(json.get("id").expect("mutate-semio-value: a ref value must carry an id")) },
            other => panic!("mutate-semio-value: unknown value kind {other:?}"),
        }
    }
    fn decode_entry(json: &Json) -> SemioValueEntry {
        SemioValueEntry { key: json.str("key"), value: decode_value(json.get("value").expect("mutate-semio-value: a map entry must carry a value")) }
    }
    fn decode_node(json: &Json) -> SemioValueNode {
        SemioValueNode { id: decode_id(json.get("id").expect("mutate-semio-value: a graph node must carry an id")), value: decode_value(json.get("value").expect("mutate-semio-value: a graph node must carry a value")) }
    }
    fn decode_snapshot(json: &Json) -> SemioValueSnapshot {
        SemioValueSnapshot { schema: json.str("schema"), root: decode_value(json.get("root").expect("mutate-semio-value: a snapshot must carry a root")), nodes: json.array("nodes").iter().map(decode_node).collect() }
    }
    fn decode_path(json: &Json, key: &str) -> SemioValuePath {
        json.array(key)
            .iter()
            .map(|segment| match segment.str("kind").as_str() {
                "key" => SemioValuePathSegment::Key { key: segment.str("key") },
                "index" => SemioValuePathSegment::Index { index: usize_field(segment, "index") },
                other => panic!("mutate-semio-value: unknown path segment kind {other:?}"),
            })
            .collect()
    }
    fn payload_value(json: &Json, key: &str) -> SemioValue {
        decode_value(json.get(key).unwrap_or_else(|| panic!("mutate-semio-value: mutation fixture must carry a {key:?} value")))
    }
    fn decode_mutation(json: &Json) -> SemioValueMutation {
        match json.str("mutation").as_str() {
            "noMutation" => SemioValueMutation::NoMutation,
            "setSnapshot" => SemioValueMutation::SetSnapshot { snapshot: decode_snapshot(json.get("snapshot").expect("mutate-semio-value: setSnapshot fixture must carry a snapshot")) },
            "setValue" => SemioValueMutation::SetValue { path: decode_path(json, "path"), value: payload_value(json, "value") },
            "setMapEntry" => SemioValueMutation::SetMapEntry { path: decode_path(json, "path"), key: json.str("key"), value: payload_value(json, "value") },
            "removeMapEntry" => SemioValueMutation::RemoveMapEntry { path: decode_path(json, "path"), key: json.str("key") },
            "insertListItem" => SemioValueMutation::InsertListItem { path: decode_path(json, "path"), index: usize_field(json, "index"), value: payload_value(json, "value") },
            "removeListItem" => SemioValueMutation::RemoveListItem { path: decode_path(json, "path"), index: usize_field(json, "index") },
            "setNode" => SemioValueMutation::SetNode { id: decode_id(json.get("id").expect("mutate-semio-value: setNode fixture must carry an id")), value: payload_value(json, "value") },
            "removeNode" => SemioValueMutation::RemoveNode { id: decode_id(json.get("id").expect("mutate-semio-value: removeNode fixture must carry an id")) },
            other => panic!("mutate-semio-value: no decoder for mutation variant {other:?}"),
        }
    }
    //#endregion 🔖️Decode

    //#region 🔖️Fixtures
    /// 🧫️ Reads the SAME committed fixture bytes the oracle role reads, decoded once into real
    /// typed values through the structural decoders above.
    fn fixture_for(kind: &str, ctx: &Context) -> Result<(SemioValueSnapshot, SemioValueMutation), String> {
        let before = decode_snapshot(&ctx.fixture_json(&before_uri(kind))?);
        let mutation = decode_mutation(&ctx.fixture_json(&mutation_uri(kind))?);
        Ok((before, mutation))
    }
    //#endregion 🔖️Fixtures

    //#region 🔖️Projection
    fn tagged(kind: &str, rest: Vec<(String, Json)>) -> Json {
        let mut entries = vec![("kind".to_string(), Json::String(kind.to_string()))];
        entries.extend(rest);
        Json::Object(entries)
    }
    fn bytes_json(bytes: &[u8]) -> Json {
        Json::Array(bytes.iter().map(|byte| Json::Number(*byte as f64)).collect())
    }
    fn id_json(id: &ValueId) -> Json {
        Json::Object(vec![("value".to_string(), Json::String(id.value.clone()))])
    }
    /// 🎯️ Mirrors `SemioValue`'s internally-tagged (`tag = "kind"`, camelCase) serde shape variant
    /// for variant, recursing through `list` and `map` exactly as the type does.
    fn value_json(value: &SemioValue) -> Json {
        match value {
            SemioValue::Null => tagged("null", Vec::new()),
            SemioValue::Bool { value } => tagged("bool", vec![("value".to_string(), Json::Bool(*value))]),
            SemioValue::Int { lexeme } => tagged("int", vec![("lexeme".to_string(), Json::String(lexeme.clone()))]),
            SemioValue::Float { lexeme } => tagged("float", vec![("lexeme".to_string(), Json::String(lexeme.clone()))]),
            SemioValue::Str { value } => tagged("str", vec![("value".to_string(), Json::String(value.clone()))]),
            SemioValue::Bytes { value } => tagged("bytes", vec![("value".to_string(), bytes_json(value))]),
            SemioValue::List { items } => tagged("list", vec![("items".to_string(), Json::Array(items.iter().map(value_json).collect()))]),
            SemioValue::Map { entries } => tagged("map", vec![("entries".to_string(), Json::Array(entries.iter().map(entry_json).collect()))]),
            SemioValue::Ref { id } => tagged("ref", vec![("id".to_string(), id_json(id))]),
        }
    }
    fn entry_json(entry: &SemioValueEntry) -> Json {
        Json::Object(vec![("key".to_string(), Json::String(entry.key.clone())), ("value".to_string(), value_json(&entry.value))])
    }
    fn node_json(node: &SemioValueNode) -> Json {
        Json::Object(vec![("id".to_string(), id_json(&node.id)), ("value".to_string(), value_json(&node.value))])
    }
    /// 🎯️ The projection every scenario compares under `ordered-json-v1` — array order is
    /// significant there, which is exactly what makes the `remove-map-entry` inverse's
    /// position-restoring multi-step undo checkable.
    fn snapshot_json(snapshot: &SemioValueSnapshot) -> Json {
        Json::Object(vec![
            ("schema".to_string(), Json::String(snapshot.schema.clone())),
            ("root".to_string(), value_json(&snapshot.root)),
            ("nodes".to_string(), Json::Array(snapshot.nodes.iter().map(node_json).collect())),
        ])
    }
    fn outcome_of(snapshot: &SemioValueSnapshot) -> Outcome {
        let projection = snapshot_json(snapshot);
        let bytes = projection.to_string().into_bytes();
        Outcome::with_raw(bytes, projection)
    }
    //#endregion 🔖️Projection

    //#region 🔖️Handlers
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let (mut base, mutation) = fixture_for(kind, ctx)?;
            let outcome = apply_semio_value_mutation(&mut base, &mutation);
            if !outcome.messages().is_empty() {
                return Err(format!("mutate-{kind}: mutation rejected: {:?}", outcome.messages()));
            }
            Ok(outcome_of(&base))
        }
    }

    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let (base, mutation) = fixture_for(kind, ctx)?;
            let mut current = base.clone();
            let outcome = apply_semio_value_mutation(&mut current, &mutation);
            if !outcome.messages().is_empty() {
                return Err(format!("inverse-{kind}: forward mutation rejected: {:?}", outcome.messages()));
            }
            for step in &inverse_semio_value_mutation(&mutation, &base) {
                let step_outcome = apply_semio_value_mutation(&mut current, step);
                if !step_outcome.messages().is_empty() {
                    return Err(format!("inverse-{kind}: inverse step rejected: {:?}", step_outcome.messages()));
                }
            }
            Ok(outcome_of(&current))
        }
    }

    /// 🔁️ The completeness law: the subset's own full-replace `set-snapshot` diff must carry an
    /// empty snapshot all the way to the committed document, with no node of the recursive typed
    /// model silently dropped on the way through.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let committed = decode_snapshot(&ctx.fixture_json(&before_uri("no-mutation"))?);
        let mut rebuilt = SemioValueSnapshot::default();
        let outcome = apply_semio_value_mutation(&mut rebuilt, &SemioValueMutation::SetSnapshot { snapshot: committed });
        if !outcome.messages().is_empty() {
            return Err(format!("identity-round-trip: full-replace rejected: {:?}", outcome.messages()));
        }
        Ok(outcome_of(&rebuilt))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario
/// id, so every kind is registered in a loop over `KINDS`.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle_for(kind)).oracle(&format!("inverse-{kind}"), inverse_oracle_for(kind));
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind)).subject(&format!("inverse-{kind}"), subject::inverse(kind));
        }
    }
    built = built.oracle("identity-round-trip", round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
