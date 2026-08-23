//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed by the
//! registered `serde_json` reference implementation so the subject's own mutation has an independent
//! result to be compared against instead of being checked against its own reading.
//!
//! The vocabulary is per SUBSET, not per artifact: two standards of the same format declare
//! different mutations, and a subset that shares an implementation with another reaches it through
//! the shared family modules rather than by copying it. This subset does not share — RFC 8259
//! declares object member order insignificant (§4), and `serde_json`'s default (non-`preserve_order`)
//! `Map` is a `BTreeMap` that re-sorts keys alphabetically on every parse/serialize — real, deliberate
//! producer freedom absorbed by the `ordered-json-v1` core comparison profile (array order
//! significant, key order never), not by anything hand-rolled here.
//!
//! @see ../🧪️oracle/🔣️component.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️component.rs — the mutation vocabulary itself.

use semio_repo_test_host::Json;

//#region 🔖️Path
/// 🧭️ Test-oracle-local mirror of the subject's `JsonPathSegment` — the oracle role must not link
/// the subject crate at all, so this addresses a `serde_json::Value` tree independently. Not itself
/// gated on the `oracles` feature (unlike everything that touches `serde_json::Value`) so
/// `read_at`'s signature stays available either way.
#[derive(Clone, Debug)]
pub enum PathSeg {
    Key(String),
    Index(usize),
}

/// 🔀️ A mutation spec's `path` param (`["models", 0, "model"]`) into `PathSeg`s — a string entry
/// is an object key, a number entry is an array index.
#[cfg(feature = "oracles")]
pub fn path_from_spec(path: &Json) -> Vec<PathSeg> {
    match path {
        Json::Array(segments) => segments
            .iter()
            .map(|segment| match segment {
                Json::String(key) => PathSeg::Key(key.clone()),
                Json::Number(index) => PathSeg::Index(*index as usize),
                _ => PathSeg::Key(String::new()),
            })
            .collect(),
        _ => Vec::new(),
    }
}

/// 🔎️ Read-only navigation of `path` from `root`, `None` on the first unresolvable segment.
#[cfg(feature = "oracles")]
pub fn resolve<'a>(root: &'a serde_json::Value, path: &[PathSeg]) -> Option<&'a serde_json::Value> {
    let mut node = root;
    for segment in path {
        node = match (segment, node) {
            (PathSeg::Key(key), serde_json::Value::Object(map)) => map.get(key)?,
            (PathSeg::Index(index), serde_json::Value::Array(items)) => items.get(*index)?,
            _ => return None,
        };
    }
    Some(node)
}

/// 🔧️ Mutable navigation of `path` from `root`, `None` on the first unresolvable segment. An empty
/// `path` resolves to `root` itself, so `set-scalar`'s whole-document replacement needs no special case.
#[cfg(feature = "oracles")]
pub fn resolve_mut<'a>(root: &'a mut serde_json::Value, path: &[PathSeg]) -> Option<&'a mut serde_json::Value> {
    let mut node = root;
    for segment in path {
        node = match (segment, node) {
            (PathSeg::Key(key), serde_json::Value::Object(map)) => map.get_mut(key)?,
            (PathSeg::Index(index), serde_json::Value::Array(items)) => items.get_mut(*index)?,
            _ => return None,
        };
    }
    Some(node)
}
//#endregion 🔖️Path

//#region 🔖️Codec
/// 📥️ Independent RFC 8259 read via the reference implementation.
#[cfg(feature = "oracles")]
pub fn read_json(input: &[u8]) -> Result<serde_json::Value, String> {
    serde_json::from_slice(input).map_err(|error| format!("independent reader could not parse JSON: {error}"))
}

/// 📤️ Independent RFC 8259 write via the reference implementation — `serde_json`'s own compact
/// form and number formatting, never this subset's own writer.
#[cfg(feature = "oracles")]
pub fn write_json(value: &serde_json::Value) -> Result<Vec<u8>, String> {
    serde_json::to_vec(value).map_err(|error| format!("independent writer could not serialize JSON: {error}"))
}

/// 🔁️ The oracle's own decode/re-encode, entirely through the reference implementation — an
/// adapter never has to name `serde_json::Value` itself to ask for this.
#[cfg(feature = "oracles")]
pub fn round_trip(bytes: &[u8]) -> Result<Vec<u8>, String> {
    write_json(&read_json(bytes)?)
}
#[cfg(not(feature = "oracles"))]
pub fn round_trip(_bytes: &[u8]) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Codec

//#region 🔖️SpecReaders
#[cfg(feature = "oracles")]
fn mutation_params(spec: &Json) -> Json {
    spec.get("params").cloned().unwrap_or(Json::Null)
}
#[cfg(feature = "oracles")]
fn number(value: &Json, key: &str) -> Option<f64> {
    match value.get(key) {
        Some(Json::Number(number)) => Some(*number),
        _ => None,
    }
}
#[cfg(feature = "oracles")]
fn path_param(params: &Json) -> Vec<PathSeg> {
    path_from_spec(&params.get("path").cloned().unwrap_or(Json::Array(Vec::new())))
}
/// 🔀️ The host's own minimal `Json` (single `f64` number kind, no `preserve_order` distinction)
/// into `serde_json::Value`, so a mutation's literal `value`/`snapshot` param can be written by the
/// reference implementation.
#[cfg(feature = "oracles")]
fn json_to_serde(value: &Json) -> serde_json::Value {
    match value {
        Json::Null => serde_json::Value::Null,
        Json::Bool(flag) => serde_json::Value::Bool(*flag),
        Json::Number(number) => serde_json::Number::from_f64(*number).map(serde_json::Value::Number).unwrap_or(serde_json::Value::Null),
        Json::String(text) => serde_json::Value::String(text.clone()),
        Json::Array(items) => serde_json::Value::Array(items.iter().map(json_to_serde).collect()),
        Json::Object(entries) => serde_json::Value::Object(entries.iter().map(|(key, value)| (key.clone(), json_to_serde(value))).collect()),
    }
}
//#endregion 🔖️SpecReaders

//#region 🔖️Dispatch
/// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized bytes.
/// An unrecognised kind is an error, never a silent no-op — a mutation that is quietly skipped
/// reports as a passing test. Navigation that fails to resolve is likewise an error here (unlike
/// this subset's own `JsonMutation::diff`, which treats it as a no-op at the diff-algebra level):
/// every scenario in this case targets a real, resolvable path, so a resolution failure here can
/// only mean the spec itself is wrong.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let params = mutation_params(spec);
    match spec.str("kind").as_str() {
        "" => Err("mutation spec carries no `kind`".to_string()),
        "no-mutation" => Ok(input.to_vec()),
        "set-snapshot" => {
            let value = json_to_serde(&params.get("value").cloned().unwrap_or(Json::Null));
            write_json(&value)
        }
        "set-member" => {
            let path = path_param(&params);
            let key = params.str("key");
            let value = json_to_serde(&params.get("value").cloned().unwrap_or(Json::Null));
            let mut root = read_json(input)?;
            match resolve_mut(&mut root, &path) {
                Some(serde_json::Value::Object(map)) => {
                    map.insert(key, value);
                }
                _ => return Err("set-member: target at path is not an object".to_string()),
            }
            write_json(&root)
        }
        "remove-member" => {
            let path = path_param(&params);
            let key = params.str("key");
            let mut root = read_json(input)?;
            match resolve_mut(&mut root, &path) {
                Some(serde_json::Value::Object(map)) => {
                    map.remove(&key);
                }
                _ => return Err("remove-member: target at path is not an object".to_string()),
            }
            write_json(&root)
        }
        "insert-array-element" => {
            let path = path_param(&params);
            let index = number(&params, "index").ok_or("insert-array-element: missing `index`")? as usize;
            let value = json_to_serde(&params.get("value").cloned().unwrap_or(Json::Null));
            let mut root = read_json(input)?;
            match resolve_mut(&mut root, &path) {
                Some(serde_json::Value::Array(items)) => {
                    let clamped = index.min(items.len());
                    items.insert(clamped, value);
                }
                _ => return Err("insert-array-element: target at path is not an array".to_string()),
            }
            write_json(&root)
        }
        "remove-array-element" => {
            let path = path_param(&params);
            let index = number(&params, "index").ok_or("remove-array-element: missing `index`")? as usize;
            let mut root = read_json(input)?;
            match resolve_mut(&mut root, &path) {
                Some(serde_json::Value::Array(items)) if index < items.len() => {
                    items.remove(index);
                }
                Some(serde_json::Value::Array(_)) => return Err(format!("remove-array-element: index {index} out of bounds")),
                _ => return Err("remove-array-element: target at path is not an array".to_string()),
            }
            write_json(&root)
        }
        "set-scalar" => {
            let path = path_param(&params);
            let value = json_to_serde(&params.get("value").cloned().unwrap_or(Json::Null));
            let mut root = read_json(input)?;
            match resolve_mut(&mut root, &path) {
                Some(node) => *node = value,
                None => return Err("set-scalar: target path does not resolve".to_string()),
            }
            write_json(&root)
        }
        kind => Err(format!("mutation kind {kind:?} has no oracle implementation ({} input byte(s))", input.len())),
    }
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch

//#region 🔖️Projection
/// 🔀️ `serde_json::Value` into the host's own minimal `Json` (`Number` is always `f64`, matching
/// how the comparison engine reads projections) — keys keep whatever order `serde_json`'s `Map`
/// iterates in, which is IGNORED at comparison time by the `ordered-json-v1` profile. Exposed (not
/// just `project_json_value`'s wrapped form) so an adapter's own inverse-spec derivation can embed a
/// raw pre-mutation VALUE — e.g. `set-member`'s old value, or the whole document for `set-snapshot`
/// — inside a mutation spec's `value`/`snapshot` param.
#[cfg(feature = "oracles")]
pub fn project_value(value: &serde_json::Value) -> Json {
    match value {
        serde_json::Value::Null => Json::Null,
        serde_json::Value::Bool(flag) => Json::Bool(*flag),
        serde_json::Value::Number(number) => Json::Number(number.as_f64().unwrap_or(f64::NAN)),
        serde_json::Value::String(text) => Json::String(text.clone()),
        serde_json::Value::Array(items) => Json::Array(items.iter().map(project_value).collect()),
        serde_json::Value::Object(map) => Json::Object(map.iter().map(|(key, value)| (key.clone(), project_value(value))).collect()),
    }
}

/// 👁️ Projects JSON bytes with the INDEPENDENT `serde_json` reader onto the `ordered-json-v1` shape
/// this case's oracle and subject are both compared through — key order is real in the wire form
/// but not in this projection (see the module doc comment).
#[cfg(feature = "oracles")]
pub fn project_json_value(bytes: &[u8]) -> Result<Json, String> {
    let value = read_json(bytes)?;
    Ok(Json::Object(vec![("format".to_string(), Json::String("json".to_string())), ("value".to_string(), project_value(&value))]))
}

#[cfg(not(feature = "oracles"))]
pub fn project_json_value(_bytes: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

/// 🔎️ Reads the PROJECTED value found at `path` (the same string-key/number-index shape a mutation
/// spec's own `path` param uses) with `extra` segments appended first — so an adapter deriving an
/// independent inverse spec can address "member `key` under `path`" (`extra = [Key(key)]`) or
/// "element `index` under `path`" (`extra = [Index(index)]`) without ever naming `serde_json::Value`
/// itself. `Ok(None)` on the first unresolvable segment.
#[cfg(feature = "oracles")]
pub fn read_at(bytes: &[u8], path: &Json, extra: &[PathSeg]) -> Result<Option<Json>, String> {
    let root = read_json(bytes)?;
    let mut segments = path_from_spec(path);
    segments.extend_from_slice(extra);
    Ok(resolve(&root, &segments).map(project_value))
}
#[cfg(not(feature = "oracles"))]
pub fn read_at(_bytes: &[u8], _path: &Json, _extra: &[PathSeg]) -> Result<Option<Json>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Projection

//#region 🧪️Tests
#[cfg(all(test, feature = "oracles"))]
mod tests {
    use super::*;

    fn spec(kind: &str, params: Json) -> Json {
        Json::Object(vec![("kind".to_string(), Json::String(kind.to_string())), ("params".to_string(), params)])
    }
    fn obj(pairs: Vec<(&str, Json)>) -> Json {
        Json::Object(pairs.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
    }

    #[semio_framework_async_macros::async_test]
    async fn no_mutation_is_a_true_byte_identity() {
        let input = br#"{"a":1,"b":2}"#;
        let output = oracle_apply_mutation(input, &spec("no-mutation", Json::Object(vec![]))).unwrap();
        assert_eq!(output, input);
    }

    #[semio_framework_async_macros::async_test]
    async fn set_member_upserts_and_remove_member_deletes() {
        let input = br#"{"a":1,"nested":{"b":2}}"#;
        let updated = oracle_apply_mutation(input, &spec("set-member", obj(vec![("path", Json::Array(vec![Json::String("nested".into())])), ("key", Json::String("c".into())), ("value", Json::Number(3.0))]))).unwrap();
        let value = read_json(&updated).unwrap();
        assert_eq!(resolve(&value, &[PathSeg::Key("nested".into()), PathSeg::Key("c".into())]), Some(&serde_json::json!(3.0)));

        let removed = oracle_apply_mutation(&updated, &spec("remove-member", obj(vec![("path", Json::Array(vec![Json::String("nested".into())])), ("key", Json::String("c".into()))]))).unwrap();
        let value = read_json(&removed).unwrap();
        assert_eq!(resolve(&value, &[PathSeg::Key("nested".into()), PathSeg::Key("c".into())]), None);
        assert_eq!(resolve(&value, &[PathSeg::Key("nested".into()), PathSeg::Key("b".into())]), Some(&serde_json::json!(2)));
    }

    #[semio_framework_async_macros::async_test]
    async fn insert_and_remove_array_element_are_inverse_on_a_real_shaped_array() {
        let input = br#"{"items":[1,2,3]}"#;
        let inserted = oracle_apply_mutation(input, &spec("insert-array-element", obj(vec![("path", Json::Array(vec![Json::String("items".into())])), ("index", Json::Number(1.0)), ("value", Json::Number(99.0))]))).unwrap();
        assert_eq!(read_json(&inserted).unwrap(), serde_json::json!({"items": [1, 99, 2, 3]}));

        let removed = oracle_apply_mutation(&inserted, &spec("remove-array-element", obj(vec![("path", Json::Array(vec![Json::String("items".into())])), ("index", Json::Number(1.0))]))).unwrap();
        assert_eq!(read_json(&removed).unwrap(), read_json(input).unwrap());
    }

    #[semio_framework_async_macros::async_test]
    async fn set_scalar_replaces_regardless_of_kind_incl_whole_document() {
        let input = br#"{"a":{"b":1}}"#;
        let output = oracle_apply_mutation(input, &spec("set-scalar", obj(vec![("path", Json::Array(vec![Json::String("a".into())])), ("value", Json::String("replaced".into()))]))).unwrap();
        assert_eq!(read_json(&output).unwrap(), serde_json::json!({"a": "replaced"}));

        let whole = oracle_apply_mutation(input, &spec("set-scalar", obj(vec![("path", Json::Array(vec![])), ("value", Json::Bool(true))]))).unwrap();
        assert_eq!(read_json(&whole).unwrap(), serde_json::json!(true));
    }

    #[semio_framework_async_macros::async_test]
    async fn set_snapshot_replaces_the_whole_document() {
        let input = br#"{"old":true}"#;
        let output = oracle_apply_mutation(input, &spec("set-snapshot", obj(vec![("value", serde_to_host(&serde_json::json!({"fresh": [1, 2, "x"]})))]))).unwrap();
        assert_eq!(read_json(&output).unwrap(), serde_json::json!({"fresh": [1, 2, "x"]}));
    }

    #[semio_framework_async_macros::async_test]
    async fn projection_is_insensitive_to_member_order_but_not_array_order() {
        let a = project_json_value(br#"{"a":1,"b":2}"#).unwrap();
        let b = project_json_value(br#"{"b":2,"a":1}"#).unwrap();
        assert_eq!(a, b, "member order must not affect the projection — RFC 8259 §4 declares it insignificant");

        let arr_a = project_json_value(br#"[1,2]"#).unwrap();
        let arr_b = project_json_value(br#"[2,1]"#).unwrap();
        assert_ne!(arr_a, arr_b, "array order IS significant per RFC 8259 §5");
    }

    #[semio_framework_async_macros::async_test]
    async fn unknown_kind_is_an_error_never_a_silent_no_op() {
        let input = b"{}";
        let result = oracle_apply_mutation(input, &spec("not-a-real-kind", Json::Object(vec![])));
        assert!(result.is_err(), "an unrecognised kind must fail loudly");
    }

    /// 🧪️ Test-local helper: a `serde_json::Value` literal into the host's `Json`, for building
    /// `set-snapshot` params from a `serde_json::json!` literal instead of hand-nesting `Json` variants.
    fn serde_to_host(value: &serde_json::Value) -> Json {
        match value {
            serde_json::Value::Null => Json::Null,
            serde_json::Value::Bool(flag) => Json::Bool(*flag),
            serde_json::Value::Number(number) => Json::Number(number.as_f64().unwrap_or(f64::NAN)),
            serde_json::Value::String(text) => Json::String(text.clone()),
            serde_json::Value::Array(items) => Json::Array(items.iter().map(serde_to_host).collect()),
            serde_json::Value::Object(map) => Json::Object(map.iter().map(|(key, value)| (key.clone(), serde_to_host(value))).collect()),
        }
    }
}
//#endregion 🧪️Tests
