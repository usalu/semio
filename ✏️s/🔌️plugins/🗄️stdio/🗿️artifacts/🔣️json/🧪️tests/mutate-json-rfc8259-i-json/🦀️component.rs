//! 🦀️ RFC 7493 I-JSON exhaustive mutation case — SUBJECT adapter.
//!
//! The ORACLE for this case runs in Python (`🐍️component.py`, `simplejson`), because RFC 7493
//! restricts the JSON value space and the reference has to surface member order, duplicate names and
//! exact number lexemes — see that file's own header and the subset's oracle manifest. This adapter
//! therefore carries the SUBJECT half only: this repository's own
//! `JsonSnapshot`/`JsonIJsonMutation`/`apply_json_i_json_mutation` over the full ten-kind vocabulary,
//! decoded and re-encoded through the subset's own codec alone. It is gated behind the generated
//! host's `sut` feature, so the oracle-only run never compiles the local implementation.
//!
//! Both roles are read back through an INDEPENDENT reader before the `semantic-i-json-v1` profile
//! compares them: the oracle projects through `simplejson`, the subject through `project_json_value`
//! (json-rust, in the stdio oracle crate — NOT `serde_json`, which is production-reachable in this
//! repository and was rejected on those grounds) — never through the subject's own model.

use semio_repo_test_host::Adapter;

//#region 🔖️Kinds
/// 📇️ Case-local mirror of the `json-rfc8259-i-json` catalog, duplicated rather than imported: the
/// oracle-only build of this adapter must never link `semio-s-plugin-stdio`. The production side's
/// own `kinds_match_the_enum_and_the_catalog` keeps `KINDS` honest against the enum AND the manifest;
/// a drift HERE is caught structurally instead — the contract phase fails with
/// `mutation-kind-uncovered`/`mutation-kind-undeclared`, and the runner fails every unregistered
/// scenario id outright.
const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-top-level", "upsert-member", "remove-member", "rename-member", "set-safe-number", "set-string", "insert-array-element", "remove-array-element"];

const INPUT: &str = "shared://🔣️hexagonal-cut-concrete-forest-left.model.json";
//#endregion 🔖️Kinds

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::INPUT;
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::json::standards::v_rfc8259::subsets::any::schema::mutations::{JsonPath, JsonPathSegment};
    use semio_s_plugin_stdio::artifacts::json::standards::v_rfc8259::subsets::any::schema::snapshot::{parse_json_text, write_json_text, JsonMember, JsonSnapshot, JsonValue};
    use semio_s_plugin_stdio::artifacts::json::standards::v_rfc8259::subsets::i_json::schema::mutations::{apply_json_i_json_mutation, is_safe_number_lexeme, is_unicode_noncharacter, JsonIJsonMutation, JsonIJsonRoot};
    use semio_s_plugin_stdio_test_oracle::artifacts::json::standards::v_rfc8259::subsets::any::project_json_value;

    //#region 🔖️Input
    /// 🧫️ Copies the immutable real fixture into the work directory and returns the mutable copy's bytes.
    pub fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
        let copy = ctx.copy_fixture(INPUT, Some("input.json"))?;
        std::fs::read(&copy).map_err(|error| error.to_string())
    }

    fn snapshot_of(bytes: &[u8]) -> Result<JsonSnapshot, String> {
        let text = std::str::from_utf8(bytes).map_err(|error| format!("the fixture is not UTF-8: {error}"))?;
        let value = parse_json_text(text).map_err(|error| format!("parse_json_text failed: {error}"))?;
        Ok(JsonSnapshot { value, ..JsonSnapshot::default() })
    }

    fn emit(snapshot: &JsonSnapshot) -> Result<Vec<u8>, String> {
        Ok(write_json_text(&snapshot.value).into_bytes())
    }
    //#endregion 🔖️Input

    //#region 🔖️SpecCodec
    /// 🔢️ A `Json::Number` back to an RFC 8259 lexeme. An integral value prints without a fractional
    /// part so `99` stays `99` rather than becoming `99.0`, which the `set-safe-number` clause reads
    /// as an integer and the profile compares by value either way.
    fn lexeme_of(value: f64) -> String {
        if value.fract() == 0.0 && value.abs() < 9.007_199_254_740_992e15 {
            format!("{}", value as i64)
        } else {
            format!("{value}")
        }
    }

    /// 🔀️ The scenario's JSON payload into this repository's own `JsonValue`.
    fn json_to_value(value: &Json) -> JsonValue {
        match value {
            Json::Null => JsonValue::Null,
            Json::Bool(flag) => JsonValue::Bool { value: *flag },
            Json::Number(number) => JsonValue::Number { lexeme: lexeme_of(*number) },
            Json::String(text) => JsonValue::String { value: text.clone() },
            Json::Array(items) => JsonValue::Array { items: items.iter().map(json_to_value).collect() },
            Json::Object(members) => JsonValue::Object { members: members.iter().map(|(key, item)| JsonMember { key: key.clone(), value: json_to_value(item) }).collect() },
        }
    }

    /// 🧭️ A `["models", 0, "model"]` spec path into a typed `JsonPath` — a string entry is an object
    /// member name, a number entry is an array index.
    fn path_of(params: &Json) -> JsonPath {
        params
            .array("path")
            .iter()
            .map(|segment| match segment {
                Json::Number(index) => JsonPathSegment::Index(index.max(0.0) as usize),
                other => JsonPathSegment::Key(match other {
                    Json::String(text) => text.clone(),
                    _ => String::new(),
                }),
            })
            .collect()
    }

    fn usize_field(params: &Json, key: &str) -> usize {
        match params.get(key) {
            Some(Json::Number(number)) => number.max(0.0) as usize,
            _ => 0,
        }
    }

    /// 📄️ The scenario's `<id>`/`<params>` spec turned into the ONE typed `JsonIJsonMutation` this
    /// subset declares for it.
    fn mutation_from_spec(spec: &Json) -> Result<JsonIJsonMutation, String> {
        let params = spec.get("params").cloned().unwrap_or(Json::Null);
        match spec.str("kind").as_str() {
            "no-mutation" => Ok(JsonIJsonMutation::NoMutation),
            "set-snapshot" => Ok(JsonIJsonMutation::SetSnapshot { snapshot: JsonSnapshot { value: json_to_value(&params.get("value").cloned().unwrap_or(Json::Null)), ..JsonSnapshot::default() } }),
            "set-top-level" => match (params.get("object"), params.get("array")) {
                (Some(object), _) => match json_to_value(object) {
                    JsonValue::Object { members } => Ok(JsonIJsonMutation::SetTopLevel { root: JsonIJsonRoot::Object { members } }),
                    _ => Err("set-top-level: the `object` payload is not an object".to_string()),
                },
                (_, Some(array)) => match json_to_value(array) {
                    JsonValue::Array { items } => Ok(JsonIJsonMutation::SetTopLevel { root: JsonIJsonRoot::Array { items } }),
                    _ => Err("set-top-level: the `array` payload is not an array".to_string()),
                },
                _ => Err("set-top-level: RFC 7493 §2.1 — neither an `object` nor an `array` payload, and a scalar root is unrepresentable".to_string()),
            },
            "upsert-member" => Ok(JsonIJsonMutation::UpsertMember { path: path_of(&params), key: params.str("key"), value: json_to_value(&params.get("value").cloned().unwrap_or(Json::Null)) }),
            "remove-member" => Ok(JsonIJsonMutation::RemoveMember { path: path_of(&params), key: params.str("key") }),
            "rename-member" => Ok(JsonIJsonMutation::RenameMember { path: path_of(&params), from: params.str("from"), to: params.str("to") }),
            "set-safe-number" => Ok(JsonIJsonMutation::SetSafeNumber { path: path_of(&params), lexeme: params.str("lexeme") }),
            "set-string" => Ok(JsonIJsonMutation::SetString { path: path_of(&params), value: params.str("value") }),
            "insert-array-element" => Ok(JsonIJsonMutation::InsertArrayElement { path: path_of(&params), index: usize_field(&params, "index"), value: json_to_value(&params.get("value").cloned().unwrap_or(Json::Null)) }),
            "remove-array-element" => Ok(JsonIJsonMutation::RemoveArrayElement { path: path_of(&params), index: usize_field(&params, "index") }),
            other => Err(format!("mutation kind {other:?} has no subject implementation")),
        }
    }
    //#endregion 🔖️SpecCodec

    //#region 🔖️Handlers
    /// 🎯️ One handler shared by every `mutate-<kind>` scenario id.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let mut snapshot = snapshot_of(&mutable_input(ctx)?)?;
        let mutation = mutation_from_spec(&ctx.doc_json()?)?;
        let outcome = apply_json_i_json_mutation(&mut snapshot, &mutation);
        if !outcome.messages().is_empty() {
            return Err(format!("the subject refused the mutation: {:?}", outcome.messages()));
        }
        let bytes = emit(&snapshot)?;
        let projection = project_json_value(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// ↩️ One handler shared by every `inverse-<kind>` scenario id. The undo comes from the subset's
    /// own `Mutation::inverse` — which is the very law under test, so the oracle recomputes its own
    /// undo independently from the pre-mutation document rather than being handed this one.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let base = snapshot_of(&mutable_input(ctx)?)?;
        let mutation = mutation_from_spec(&ctx.doc_json()?)?;
        let undo = protocol_inverse(&mutation, &base);
        let mut snapshot = base;
        apply_json_i_json_mutation(&mut snapshot, &mutation);
        for step in &undo {
            apply_json_i_json_mutation(&mut snapshot, step);
        }
        let bytes = emit(&snapshot)?;
        let projection = project_json_value(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// ↩️ `Mutation::inverse` reached without naming the `protocol` crate in this adapter's own
    /// dependency list — the trait is re-exported through the subject crate's own public surface.
    fn protocol_inverse(mutation: &JsonIJsonMutation, base: &JsonSnapshot) -> Vec<JsonIJsonMutation> {
        use semio_s_plugin_stdio::protocol::Mutation;
        Mutation::inverse(mutation, base)
    }

    /// 🛡️ The same four RFC 7493 clauses the oracle checks, computed from the subject's own decoded
    /// snapshot so the two counts can be compared rather than merely both being green.
    pub fn i_json_conformance(ctx: &Context) -> Result<Outcome, String> {
        let raw = mutable_input(ctx)?;
        let snapshot = snapshot_of(&raw)?;
        let mut duplicates = 0usize;
        let mut integers = 0usize;
        let mut unsafe_integers = 0usize;
        let mut strings = 0usize;
        let mut noncharacter_strings = 0usize;
        walk(&snapshot.value, &mut duplicates, &mut integers, &mut unsafe_integers, &mut strings, &mut noncharacter_strings);
        let top_level = match &snapshot.value {
            JsonValue::Object { .. } => "object",
            JsonValue::Array { .. } => "array",
            _ => return Err("RFC 7493 §2.1: the top-level value is a bare scalar".to_string()),
        };
        Ok(Outcome::projection(Json::Object(vec![
            ("topLevel".to_string(), Json::String(top_level.to_string())),
            ("duplicateMemberNames".to_string(), Json::Number(duplicates as f64)),
            ("integers".to_string(), Json::Number(integers as f64)),
            ("unsafeIntegers".to_string(), Json::Number(unsafe_integers as f64)),
            ("strings".to_string(), Json::Number(strings as f64)),
            ("noncharacterStrings".to_string(), Json::Number(noncharacter_strings as f64)),
            ("bytes".to_string(), Json::Number(raw.len() as f64)),
        ])))
    }

    fn walk(value: &JsonValue, duplicates: &mut usize, integers: &mut usize, unsafe_integers: &mut usize, strings: &mut usize, noncharacter_strings: &mut usize) {
        match value {
            JsonValue::Number { lexeme } => {
                if !lexeme.contains('.') && !lexeme.contains('e') && !lexeme.contains('E') {
                    *integers += 1;
                    if !is_safe_number_lexeme(lexeme) {
                        *unsafe_integers += 1;
                    }
                }
            }
            JsonValue::String { value } => {
                *strings += 1;
                if value.chars().any(is_unicode_noncharacter) {
                    *noncharacter_strings += 1;
                }
            }
            JsonValue::Array { items } => {
                for item in items {
                    walk(item, duplicates, integers, unsafe_integers, strings, noncharacter_strings);
                }
            }
            JsonValue::Object { members } => {
                let mut seen: std::collections::HashSet<&str> = std::collections::HashSet::new();
                for member in members {
                    if !seen.insert(member.key.as_str()) {
                        *duplicates += 1;
                    }
                    walk(&member.value, duplicates, integers, unsafe_integers, strings, noncharacter_strings);
                }
            }
            _ => {}
        }
    }

    /// 🔒️ The no-byte-pass-through rule: the subject fully parses the real artifact into its typed
    /// snapshot and re-serializes from the model alone — `parse_json_text`/`write_json_text` are this
    /// subset's ONLY channel from input to output.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = snapshot_of(&input)?;
        let output = emit(&snapshot)?;
        if output == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_json_value(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }
    //#endregion 🔖️Handlers

    /// 🧭️ Re-exported so `super::adapter()` registers the same ten-kind sweep without a third copy of
    /// the list.
    pub const SUBJECT_KINDS: &[&str] = super::KINDS;
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Only the SUBJECT role is registered here —
/// this case's oracle role is served by `🐍️component.py`, which the coordinator selects from the
/// registered oracle's own `"ecosystem": "python"`.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        for kind in subject::SUBJECT_KINDS {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate).subject(&format!("inverse-{kind}"), subject::inverse);
        }
        built = built.subject("i-json-conformance", subject::i_json_conformance).subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
