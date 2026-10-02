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

const INPUT: &str = "shared://🔣️.json";
//#endregion 🔖️Kinds

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::INPUT;
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::base::schema::snapshot::{parse_json_text, write_json_text, JsonSnapshot, JsonValue};
    use semio_repo_test_host::law::wire_operation;
    use semio_s_artifact_stdio_json::{mutation_from_payload_json, mutation_inverse, mutation_payload_json};
    use semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::i_json::schema::mutations::{apply_json_i_json_mutation, is_safe_number_lexeme, is_unicode_noncharacter, JsonIJsonMutation};
    use semio_s_artifact_stdio_json_test_oracle::standards::v_rfc8259::subsets::base::project_json_value;

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
    /// 📄️ The scenario's `<id>`/`<params>` spec decoded as the leaf wire payload it is, through the aggregate's own
    /// derive-generated payload constructor — never re-declared field by field here.
    fn mutation_from_spec(spec: &Json) -> Result<JsonIJsonMutation, String> {
        wire_operation(&spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null), mutation_from_payload_json, mutation_payload_json)
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
        let undo = mutation_inverse(&mutation, &base);
        let mut snapshot = base;
        apply_json_i_json_mutation(&mut snapshot, &mutation);
        for step in &undo {
            apply_json_i_json_mutation(&mut snapshot, step);
        }
        let bytes = emit(&snapshot)?;
        let projection = project_json_value(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
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
        built = built.subject("mutate", subject::mutate).subject("inverse", subject::inverse);
        built = built.subject("i-json-conformance", subject::i_json_conformance).subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
