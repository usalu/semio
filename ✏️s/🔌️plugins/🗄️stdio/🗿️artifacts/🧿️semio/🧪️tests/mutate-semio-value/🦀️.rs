//! 🦀️ Semio VALUE exhaustive mutation case — Rust SUBJECT adapter. Ticket 26/08/23/END-TO-END-
//! TESTING-REFACTOR.
//!
//! **This file no longer serves the oracle role.** The reference for `semio-v1-value-mutate` is the
//! registered oracle `semio-value-python-independent` (`../../🏅️standards/🔖️v1/🪆️subsets/✳️value/
//! 🧪️oracle/🔣️.json`) — an independent Python implementation of the semio value carrier and
//! its nine verbs, written from the committed grammar, protocol and specification vectors, living
//! beside this file as `🐍️component.py`. The runner dispatches the oracle role to that adapter and
//! the subject role here, and compares the two projections under `@comparison-ordered-json-v1`.
//! Registering oracle handlers here as well would put this repository's own answer on both sides of
//! that comparison, which is the precise failure the platform exists to prevent.
//!
//! **What the handlers assert in role.** Parity across the two implementations is the primary
//! evidence, but each side still states its own law so a scenario can fail for the right reason with
//! a readable message: `inverse-<kind>` requires the mutation's OWN computed inverse to restore the
//! building model, `spec-vector-<kind>` requires the applied snapshot to be the committed
//! after-snapshot AND the undone one to be the before-snapshot, `payload-fidelity` requires the
//! derived document to still carry exactly what this repository's own RFC 8259 reader finds in the
//! committed source, and `identity-round-trip` requires all four committed encodings to be
//! reproduced byte for byte through `law::carrier_is_exact`.
//!
//! **How the fixtures reach typed values.** The generated test host links only `semio-repo-test-host`
//! and, behind `sut`, this subset's own crate — no `serde`, no `serde_json` — so the subject module
//! below carries its own small, forward-only, structural decoder over the framework's dependency-free
//! `Json`. It decodes JSON STRUCTURE only, field by field, mirroring each committed payload's own
//! declared serde shape; it never invents or reimplements any mutation SEMANTICS, which still run
//! through the real `apply_semio_value_mutation`/`inverse_semio_value_mutation`. The DSL and pack
//! bridges this case's byte law needs — `parse_semio_value_dsl`, `print_semio_value_dsl`,
//! `encode_semio_value_pack`, `decode_semio_value_pack` — were added to the subset's own
//! `📸️snapshot/🦀️component.rs` for this wave, mirroring the ones `✳️table`, `✳️flow` and `✳️text`
//! already exported; before them this case could make no byte claim at all.

use semio_repo_test_host::Adapter;

//#region 🔖️Kinds
/// 🏷️ Mirrors `SemioValueMutation::KINDS` (`../../🏅️standards/🔖️v1/🪆️subsets/✳️value/🧬️schema/
/// 🧬️mutations/🦀️component.rs`) — duplicated, not imported, because the generated host builds this
/// file with and without the subject crate. The contract's mutation-coverage gate keeps this list
/// honest against the catalog; `kinds_match_the_enum_and_the_catalog` in that production file keeps
/// it honest against the enum.
#[cfg(feature = "sut")]
const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-value", "set-map-entry", "remove-map-entry", "insert-list-item", "remove-list-item", "set-node", "remove-node"];
//#endregion 🔖️Kinds

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{digest, parse_json, Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::json::standards::v_rfc8259::subsets::base::schema::snapshot::{parse_json_text, JsonMember, JsonValue};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::mutations::semio_mutation_refusals;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::mutations::{
        apply_semio_value_mutation, insert_list_item, inverse_semio_value_mutation, remove_list_item, remove_map_entry, remove_node, set_map_entry, set_node, set_snapshot, set_value, SemioValueMutation, SemioValuePath, SemioValuePathSegment,
    };
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::snapshot::{
        decode_semio_value_pack, decode_semio_value_snapshot_json, encode_semio_value_pack, encode_semio_value_snapshot_json, parse_semio_value_dsl, print_semio_value_dsl, SemioValue, SemioValueEntry, SemioValueNode, SemioValueSnapshot, ValueId,
    };
    use semio_s_plugin_stdio_test_oracle::law::carrier_is_exact;

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
    /// 🦠️ Decodes every REAL mutation variant. `"noMutation"` is handled by the two call sites
    /// below instead of here: the retained `no-mutation` scenario id maps to the identity mutation
    /// `SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })`, and this decoder has no
    /// access to `base` — only the two callers do.
    fn decode_mutation(json: &Json) -> SemioValueMutation {
        match json.str("mutation").as_str() {
            "setSnapshot" => SemioValueMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: decode_snapshot(json.get("snapshot").expect("mutate-semio-value: setSnapshot fixture must carry a snapshot")) }),
            "setValue" => SemioValueMutation::SetValue(set_value::SetValue { path: decode_path(json, "path"), value: payload_value(json, "value") }),
            "setMapEntry" => SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { path: decode_path(json, "path"), key: json.str("key"), value: payload_value(json, "value") }),
            "removeMapEntry" => SemioValueMutation::RemoveMapEntry(remove_map_entry::RemoveMapEntry { path: decode_path(json, "path"), key: json.str("key") }),
            "insertListItem" => SemioValueMutation::InsertListItem(insert_list_item::InsertListItem { path: decode_path(json, "path"), index: usize_field(json, "index"), value: payload_value(json, "value") }),
            "removeListItem" => SemioValueMutation::RemoveListItem(remove_list_item::RemoveListItem { path: decode_path(json, "path"), index: usize_field(json, "index") }),
            "setNode" => SemioValueMutation::SetNode(set_node::SetNode { id: decode_id(json.get("id").expect("mutate-semio-value: setNode fixture must carry an id")), value: payload_value(json, "value") }),
            "removeNode" => SemioValueMutation::RemoveNode(remove_node::RemoveNode { id: decode_id(json.get("id").expect("mutate-semio-value: removeNode fixture must carry an id")) }),
            other => panic!("mutate-semio-value: no decoder for mutation variant {other:?}"),
        }
    }
    //#endregion 🔖️Decode
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
    //#endregion 🔖️Projection

    //#region 🔖️Derivation
    /// 🌲️ One RFC 8259 value as a `SemioValue`, by the rule the feature states and both
    /// implementations read from there: a number whose lexeme carries a `.`, an `e` or an `E` is a
    /// `Float`, every other number is an `Int`, and both keep the SOURCE LEXEME verbatim — which is
    /// the one property of `SemioValue` a native numeric type would destroy.
    fn value_of_json(node: &JsonValue) -> SemioValue {
        match node {
            JsonValue::Null => SemioValue::Null,
            JsonValue::Bool { value } => SemioValue::Bool { value: *value },
            JsonValue::Number { lexeme } => {
                if lexeme.contains(['.', 'e', 'E']) {
                    SemioValue::Float { lexeme: lexeme.clone() }
                } else {
                    SemioValue::Int { lexeme: lexeme.clone() }
                }
            }
            JsonValue::String { value } => SemioValue::Str { value: value.clone() },
            JsonValue::Array { items } => SemioValue::List { items: items.iter().map(value_of_json).collect() },
            JsonValue::Object { members } => SemioValue::Map { entries: members.iter().map(|member| SemioValueEntry { key: member.key.clone(), value: value_of_json(&member.value) }).collect() },
        }
    }

    fn member<'a>(node: &'a JsonValue, key: &str) -> Option<&'a JsonValue> {
        match node {
            JsonValue::Object { members } => members.iter().find(|member| member.key == key).map(|member| &member.value),
            _ => None,
        }
    }

    fn models_of(source: &JsonValue) -> Result<&Vec<JsonValue>, String> {
        match member(source, "models") {
            Some(JsonValue::Array { items }) => Ok(items),
            _ => Err("payload-fidelity: the committed source carries no `models` array".to_string()),
        }
    }

    fn model_id(model: &JsonValue) -> Result<String, String> {
        match member(model, "id") {
            Some(JsonValue::String { value }) => Ok(value.clone()),
            _ => Err("payload-fidelity: every source model needs a string id".to_string()),
        }
    }

    /// 🌲️ The root of the derived document: the source verbatim, except that each model's `objects`
    /// array is replaced by a `Ref` to the graph node it was lifted into.
    fn root_of(source: &JsonValue) -> Result<SemioValue, String> {
        let JsonValue::Object { members } = source else {
            return Err("payload-fidelity: the committed source is not a JSON object".to_string());
        };
        let mut entries = Vec::new();
        for entry in members {
            if entry.key != "models" {
                entries.push(SemioValueEntry { key: entry.key.clone(), value: value_of_json(&entry.value) });
                continue;
            }
            let mut models = Vec::new();
            for model in models_of(source)? {
                let id = model_id(model)?;
                let lifted = format!("{id}#objects");
                let body = match member(model, "model") {
                    Some(JsonValue::Object { members }) => members,
                    _ => return Err("payload-fidelity: every source model needs a `model` object".to_string()),
                };
                let fields = body
                    .iter()
                    .map(|field: &JsonMember| SemioValueEntry {
                        key: field.key.clone(),
                        value: if field.key == "objects" { SemioValue::Ref { id: ValueId::new(lifted.clone()) } } else { value_of_json(&field.value) },
                    })
                    .collect();
                models.push(SemioValue::Map {
                    entries: vec![SemioValueEntry { key: "id".to_string(), value: SemioValue::Str { value: id } }, SemioValueEntry { key: "model".to_string(), value: SemioValue::Map { entries: fields } }],
                });
            }
            entries.push(SemioValueEntry { key: "models".to_string(), value: SemioValue::List { items: models } });
        }
        Ok(SemioValue::Map { entries })
    }

    /// 🕸️ The graph nodes of the derived document: one per model, holding the `objects` array that
    /// was lifted out of it.
    fn nodes_of(source: &JsonValue) -> Result<Vec<SemioValueNode>, String> {
        let mut nodes = Vec::new();
        for model in models_of(source)? {
            let id = model_id(model)?;
            let body = member(model, "model").ok_or_else(|| "payload-fidelity: every source model needs a `model` object".to_string())?;
            let objects = member(body, "objects").ok_or_else(|| "payload-fidelity: every source model needs an `objects` member".to_string())?;
            nodes.push(SemioValueNode { id: ValueId::new(format!("{id}#objects")), value: value_of_json(objects) });
        }
        Ok(nodes)
    }
    //#endregion 🔖️Derivation

    //#region 🔖️Input
    /// 🕸️ The six-member demo graph, in both encodings the domain commits for it — small, but the
    /// only `stdio.semio.value` bytes in this artifact a codec other than the Python one wrote.
    const GRAPH_DSL: &str = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🕸️graph/🖼️assets/🗣️example.dsl.semio";
    const GRAPH_PACK: &str = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🕸️graph/🖼️assets/🎒️example.pack.semio";
    /// 🌲️ The real 424 KB building model, its source and its binary twin.
    const FOREST_JSON: &str = "local://🌲️hexagonal-cut-concrete-forest-left.model.json";
    const FOREST_DSL: &str = "local://🌲️hexagonal-cut-concrete-forest.dsl.semio";
    const FOREST_PACK: &str = "local://🌲️hexagonal-cut-concrete-forest.pack.semio";

    fn utf8(bytes: Vec<u8>, what: &str) -> Result<String, String> {
        String::from_utf8(bytes).map_err(|error| format!("{what} is not UTF-8: {error}"))
    }

    /// 🌲️ The real building model, parsed through this repository's own DSL codec.
    fn forest(ctx: &Context) -> Result<SemioValueSnapshot, String> {
        parse_semio_value_dsl(&utf8(ctx.fixture_bytes(FOREST_DSL)?, "the committed building model")?)
    }

    /// 📜️ The scenario's own committed mutation parameters — the feature owns the vector.
    /// `"noMutation"` maps to the identity mutation `SetSnapshot(set_snapshot::SetSnapshot { snapshot:
    /// base.clone() })` — the retained `no-mutation` scenario id's convention — since `decode_mutation`
    /// itself has no access to the real building model this call site does.
    fn mutation(ctx: &Context) -> Result<SemioValueMutation, String> {
        let json = parse_json(ctx.doc_string()?).map_err(|error| format!("{}: the scenario's mutation payload must decode: {error}", ctx.scenario.id))?;
        if json.str("mutation") == "noMutation" {
            return Ok(SemioValueMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: forest(ctx)? }));
        }
        Ok(decode_mutation(&json))
    }

    /// 🧫️ Every fixture URI the scenario's steps name, in step order and whatever scheme it uses. The
    /// feature is the single place a vector path is written down; both adapters read it from there.
    fn step_fixtures(ctx: &Context) -> Vec<String> {
        let mut found = Vec::new();
        for (_, text) in &ctx.scenario.steps {
            let mut rest = text.as_str();
            loop {
                let Some(at) = ["local://", "asset://", "shared://"].iter().filter_map(|scheme| rest.find(scheme)).min() else {
                    break;
                };
                let tail = &rest[at..];
                let end = tail.find(char::is_whitespace).unwrap_or(tail.len());
                found.push(tail[..end].to_string());
                rest = &tail[end..];
            }
        }
        found
    }

    fn vector(ctx: &Context, position: usize, label: &str) -> Result<SemioValueSnapshot, String> {
        let uri = step_fixtures(ctx).into_iter().nth(position).ok_or_else(|| format!("{}: the scenario names no {label} fixture", ctx.scenario.id))?;
        decode_semio_value_snapshot_json(&utf8(ctx.fixture_bytes(&uri)?, &uri)?)
    }

    /// `"noMutation"` maps to the identity mutation `SetSnapshot(set_snapshot::SetSnapshot { snapshot:
    /// base.clone() })`, `base` being the vector's own before-snapshot (step position 0) — same
    /// convention `mutation` above applies for the non-vector scenarios.
    fn vector_mutation(ctx: &Context, position: usize) -> Result<SemioValueMutation, String> {
        let uri = step_fixtures(ctx).into_iter().nth(position).ok_or_else(|| format!("{}: the scenario names no mutation fixture", ctx.scenario.id))?;
        let json = ctx.fixture_json(&uri)?;
        if json.str("mutation") == "noMutation" {
            return Ok(SemioValueMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: vector(ctx, 0, "before-snapshot")? }));
        }
        Ok(decode_mutation(&json))
    }

    fn apply(current: &mut SemioValueSnapshot, step: &SemioValueMutation, what: &str) -> Result<(), String> {
        let outcome = apply_semio_value_mutation(current, step);
        let refusals = semio_mutation_refusals(&outcome);
        if refusals.is_empty() {
            return Ok(());
        }
        Err(format!("{what}: the mutation was rejected: {refusals:?}"))
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same JSON both sides project, so a red
    /// scenario is readable without re-running anything.
    fn disagreement(what: &str, got: &SemioValueSnapshot, expected: &SemioValueSnapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_semio_value_snapshot_json(got), encode_semio_value_snapshot_json(expected))
    }
    //#endregion 🔖️Input

    //#region 🔖️Handlers
    /// 🎯️ One verb applied to the real building model by this repository's codec alone.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let mut current = forest(ctx)?;
        apply(&mut current, &mutation(ctx)?, &ctx.scenario.id)?;
        let projection = snapshot_json(&current);
        Ok(Outcome::with_raw(print_semio_value_dsl(&current).into_bytes(), projection))
    }

    /// ↩️ The metamorphic inverse law on the real document: applying the verb and then its OWN
    /// computed inverse must restore it exactly — map-entry POSITION, list position and node order
    /// included, which is what makes `remove-map-entry`'s multi-step undo checkable rather than
    /// merely runnable.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let base = forest(ctx)?;
        let step = mutation(ctx)?;
        let mut current = base.clone();
        apply(&mut current, &step, &ctx.scenario.id)?;
        let mutated = snapshot_json(&current);
        for undo in inverse_semio_value_mutation(&step, &base) {
            apply(&mut current, &undo, &ctx.scenario.id)?;
        }
        if current != base {
            return Err(disagreement(&format!("{}: undoing the mutation did not restore the building model", ctx.scenario.id), &current, &base));
        }
        Ok(Outcome::projection(Json::Object(vec![("mutated".to_string(), mutated), ("restored".to_string(), snapshot_json(&current))])))
    }

    /// 🧫️ The same verb on its committed `(before, mutation, after)` vector — a THIRD statement of
    /// what the verb means, independent of both implementations.
    pub fn spec_vector(ctx: &Context) -> Result<Outcome, String> {
        let base = vector(ctx, 0, "before-snapshot")?;
        let step = vector_mutation(ctx, 1)?;
        let expected = vector(ctx, 2, "after-snapshot")?;
        let mut current = base.clone();
        apply(&mut current, &step, &ctx.scenario.id)?;
        if current != expected {
            return Err(disagreement(&format!("{}: the applied document does not match the committed after-snapshot", ctx.scenario.id), &current, &expected));
        }
        let applied = snapshot_json(&current);
        for undo in inverse_semio_value_mutation(&step, &base) {
            apply(&mut current, &undo, &ctx.scenario.id)?;
        }
        if current != base {
            return Err(disagreement(&format!("{}: undoing the committed mutation did not restore its before-snapshot", ctx.scenario.id), &current, &base));
        }
        Ok(Outcome::projection(Json::Object(vec![("applied".to_string(), applied), ("restored".to_string(), snapshot_json(&current))])))
    }

    /// 🌲️ The derived fixture against the real JSON it came from, re-read on every run by THIS
    /// repository's own RFC 8259 codec while the oracle re-reads it with Python's `json` module. The
    /// derivation is a faithful transcription with one documented restructuring: each sub-model's
    /// `objects` array is lifted into a graph node keyed by `<model id>#objects` with a `Ref` left
    /// where it stood.
    pub fn payload_fidelity(ctx: &Context) -> Result<Outcome, String> {
        let source = parse_json_text(&utf8(ctx.fixture_bytes(FOREST_JSON)?, "the committed building source")?).map_err(|error| format!("payload-fidelity: the committed building source must parse as RFC 8259 JSON: {error}"))?;
        let derived = SemioValueSnapshot { schema: forest(ctx)?.schema.clone(), root: root_of(&source)?, nodes: nodes_of(&source)? };
        let committed = forest(ctx)?;
        if derived != committed {
            return Err(disagreement("payload-fidelity: the committed building document no longer matches the JSON it was derived from", &derived, &committed));
        }
        Ok(Outcome::projection(Json::Object(vec![
            ("document".to_string(), snapshot_json(&derived)),
            ("nodes".to_string(), Json::Number(derived.nodes.len() as f64)),
            ("rootEntries".to_string(), Json::Number(match &derived.root {
                SemioValue::Map { entries } => entries.len() as f64,
                _ => 0.0,
            })),
        ])))
    }

    /// 🔁️ All four committed encodings — the demo graph's two and the building model's two — each
    /// re-emitted from the parsed document.
    ///
    /// 🔒️ **The byte half of the identity law, asserted as `carrier_is_exact` and asserted in both
    /// directions.** `.dsl.semio` is a fixed-layout recursive grammar and `.pack.semio` is the same
    /// body under a binary envelope, so reproducing them BYTE FOR BYTE is the correct answer here and
    /// `law::reparsed_not_copied` would be exactly backwards. Nor is it a self-comparison: the demo
    /// graph's bytes were written by THIS codec and the Python oracle reproduces them from the
    /// grammar alone, while the building model's bytes were written by the PYTHON implementation and
    /// this codec has to reproduce THOSE.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let graph_dsl = ctx.fixture_bytes(GRAPH_DSL)?;
        let graph = parse_semio_value_dsl(&utf8(graph_dsl.clone(), "the committed demo graph")?)?;
        let graph_printed = print_semio_value_dsl(&graph);
        carrier_is_exact(graph_printed.as_bytes(), &graph_dsl)?;
        let graph_pack = ctx.fixture_bytes(GRAPH_PACK)?;
        let graph_unpacked = decode_semio_value_pack(&graph_pack)?;
        if graph_unpacked != graph {
            return Err(disagreement("identity-round-trip: the demo graph's binary twin decodes to a different document than its text", &graph_unpacked, &graph));
        }
        let graph_repacked = encode_semio_value_pack(&graph);
        carrier_is_exact(&graph_repacked, &graph_pack)?;
        let forest_dsl = ctx.fixture_bytes(FOREST_DSL)?;
        let document = parse_semio_value_dsl(&utf8(forest_dsl.clone(), "the committed building model")?)?;
        let forest_printed = print_semio_value_dsl(&document);
        carrier_is_exact(forest_printed.as_bytes(), &forest_dsl)?;
        let reparsed = parse_semio_value_dsl(&forest_printed)?;
        if reparsed != document {
            return Err(disagreement("identity-round-trip: printing the building model back to DSL and reparsing it lost content", &reparsed, &document));
        }
        let forest_pack = ctx.fixture_bytes(FOREST_PACK)?;
        let forest_unpacked = decode_semio_value_pack(&forest_pack)?;
        if forest_unpacked != document {
            return Err(disagreement("identity-round-trip: the building model's binary twin decodes to a different document than its text", &forest_unpacked, &document));
        }
        let forest_repacked = encode_semio_value_pack(&document);
        carrier_is_exact(&forest_repacked, &forest_pack)?;
        Ok(Outcome::projection(Json::Object(vec![
            ("graph".to_string(), snapshot_json(&graph)),
            ("graphDslDigest".to_string(), Json::String(digest(graph_printed.as_bytes()))),
            ("graphPackDigest".to_string(), Json::String(digest(&graph_repacked))),
            ("forestDslDigest".to_string(), Json::String(digest(forest_printed.as_bytes()))),
            ("forestPackDigest".to_string(), Json::String(digest(&forest_repacked))),
            ("forestNodes".to_string(), Json::Number(document.nodes.len() as f64)),
            ("forestDslLength".to_string(), Json::Number(forest_printed.len() as f64)),
            ("forestPackLength".to_string(), Json::Number(forest_repacked.len() as f64)),
        ])))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario
/// id, so the loop mirrors the feature's `Examples` tables exactly. Only subject handlers are
/// registered: the oracle role belongs to `🐍️component.py`.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        for kind in KINDS {
            built = built
                .subject(&format!("mutate-{kind}"), subject::mutate)
                .subject(&format!("inverse-{kind}"), subject::inverse)
                .subject(&format!("spec-vector-{kind}"), subject::spec_vector);
        }
        built = built.subject("payload-fidelity", subject::payload_fidelity).subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
