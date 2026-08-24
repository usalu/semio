//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed by the
//! registered `json` (json-rust) 0.12 reference implementation so the subject's own mutation has an
//! independent result to be compared against instead of being checked against its own reading.
//!
//! `serde_json`, not `json`, was the first choice here, and it was wrong: this subset's OWN
//! production code (`../🧬️schema/📸️snapshot/🦀️component.rs`) already declares
//! `impl From<serde_json::Value> for JsonValue` and the reverse, a real interop conversion path FROM
//! the reference's own type. A `serde_json` differential would therefore compare this
//! implementation against something it already converts from — not independent evidence, and the
//! purity gate agreed once it was pointed at the numbers (423 production files transitively reach
//! `serde_json`, correctly, since it is a genuine `workspace.dependencies` production dependency).
//! `json` (json-rust) appears nowhere in this repository's production dependency graph, so it is
//! used instead. This costs two things this subset's own codec provides and `json` does not, both
//! absorbed by design rather than worked around:
//! - Member order: `json::object::Object` stores entries in a hash-ordered binary tree (see its own
//!   source comment on `hash_key`), not insertion order — so this module still relies on the SAME
//!   `ordered-json-v1` core comparison profile the `serde_json` draft already needed for RFC 8259 §4
//!   (array order significant, key order never), rather than gaining anything back on this axis.
//! - Number precision: `json::number::Number` is a `(sign, mantissa: u64, exponent: i16)` decimal
//!   pair, not this subset's arbitrary-precision LEXEME — comparison is by parsed `f64` value here
//!   too, for the same reason `serde_json` needed it (documented in the oracle registry rationale).
//!
//! The vocabulary is per SUBSET, not per artifact: two standards of the same format declare
//! different mutations, and a subset that shares an implementation with another reaches it through
//! the shared family modules rather than by copying it.
//!
//! @see ../🧪️oracle/🔣️component.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️component.rs — the mutation vocabulary itself.

use semio_repo_test_host::Json;

//#region 🔖️Path
/// 🧭️ Test-oracle-local mirror of the subject's `JsonPathSegment` — the oracle role must not link
/// the subject crate at all, so this addresses a `json::JsonValue` tree independently. Not itself
/// gated on the `oracles` feature (unlike everything that touches `json::JsonValue`) so `read_at`'s
/// signature stays available either way.
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
pub fn resolve<'a>(root: &'a json::JsonValue, path: &[PathSeg]) -> Option<&'a json::JsonValue> {
    let mut node = root;
    for segment in path {
        node = match (segment, node) {
            (PathSeg::Key(key), json::JsonValue::Object(object)) => object.get(key)?,
            (PathSeg::Index(index), json::JsonValue::Array(items)) => items.get(*index)?,
            _ => return None,
        };
    }
    Some(node)
}

/// 🔧️ Mutable navigation of `path` from `root`, `None` on the first unresolvable segment. An empty
/// `path` resolves to `root` itself, so `set-scalar`'s whole-document replacement needs no special case.
#[cfg(feature = "oracles")]
pub fn resolve_mut<'a>(root: &'a mut json::JsonValue, path: &[PathSeg]) -> Option<&'a mut json::JsonValue> {
    let mut node = root;
    for segment in path {
        node = match (segment, node) {
            (PathSeg::Key(key), json::JsonValue::Object(object)) => object.get_mut(key)?,
            (PathSeg::Index(index), json::JsonValue::Array(items)) => items.get_mut(*index)?,
            _ => return None,
        };
    }
    Some(node)
}
//#endregion 🔖️Path

//#region 🔖️Codec
/// 📥️ Independent RFC 8259 read via the reference implementation.
#[cfg(feature = "oracles")]
pub fn read_json(input: &[u8]) -> Result<json::JsonValue, String> {
    let text = std::str::from_utf8(input).map_err(|error| format!("independent reader: input is not UTF-8: {error}"))?;
    json::parse(text).map_err(|error| format!("independent reader could not parse JSON: {error}"))
}

/// 📤️ Independent RFC 8259 write via the reference implementation — `json`'s own compact form
/// (`dump`) and number formatting, never this subset's own writer.
#[cfg(feature = "oracles")]
pub fn write_json(value: &json::JsonValue) -> Result<Vec<u8>, String> {
    Ok(value.dump().into_bytes())
}

/// 🔁️ The oracle's own decode/re-encode, entirely through the reference implementation — an
/// adapter never has to name `json::JsonValue` itself to ask for this.
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
/// 🔢️ WORKED_AROUND_DEFECT — `json` 0.12's `impl From<f64> for JsonValue` is not round-trip exact.
/// Its `Number` is a `(sign, mantissa: u64, exponent: i16)` decimal pair, and the conversion INTO it
/// rounds: `JsonValue::from(2.7000102824824506_f64).dump()` yields `2.7000102824824507`, one ULP up,
/// and `-8.881784197001252e-16` becomes `…253e-16`. Reproduced standalone against the crate alone —
/// 2 of 9 probed values were moved — and the crate's own PARSER is exact on all of them, so the fix
/// is to reach the same `Number` through the half that works: format the `f64` with Rust's own
/// shortest-round-trip `{:?}` and let `json::parse` build the value. Non-finite doubles are not JSON
/// numbers at all and become `null`, which is what the crate's own conversion does with them too.
/// Without this, a `set-snapshot` carrying a real 8,449-vertex model back through the reference
/// perturbs its coordinates and the inverse law fails on a defect that is not this repository's.
#[cfg(feature = "oracles")]
fn library_number(number: f64) -> json::JsonValue {
    if !number.is_finite() {
        return json::JsonValue::Null;
    }
    json::parse(&format!("{number:?}")).unwrap_or(json::JsonValue::Null)
}

/// 🔀️ The host's own minimal `Json` (single `f64` number kind, no order distinction) into
/// `json::JsonValue`, so a mutation's literal `value`/`snapshot` param can be written by the
/// reference implementation.
#[cfg(feature = "oracles")]
fn json_to_library(value: &Json) -> json::JsonValue {
    match value {
        Json::Null => json::JsonValue::Null,
        Json::Bool(flag) => json::JsonValue::from(*flag),
        Json::Number(number) => library_number(*number),
        Json::String(text) => json::JsonValue::from(text.clone()),
        Json::Array(items) => json::JsonValue::Array(items.iter().map(json_to_library).collect()),
        Json::Object(entries) => {
            let mut object = json::object::Object::with_capacity(entries.len());
            for (key, value) in entries {
                object.insert(key, json_to_library(value));
            }
            json::JsonValue::Object(object)
        }
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
            let value = json_to_library(&params.get("value").cloned().unwrap_or(Json::Null));
            write_json(&value)
        }
        "set-member" => {
            let path = path_param(&params);
            let key = params.str("key");
            let value = json_to_library(&params.get("value").cloned().unwrap_or(Json::Null));
            let mut root = read_json(input)?;
            match resolve_mut(&mut root, &path) {
                Some(json::JsonValue::Object(object)) => {
                    object.insert(&key, value);
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
                Some(json::JsonValue::Object(object)) => {
                    object.remove(&key);
                }
                _ => return Err("remove-member: target at path is not an object".to_string()),
            }
            write_json(&root)
        }
        "insert-array-element" => {
            let path = path_param(&params);
            let index = number(&params, "index").ok_or("insert-array-element: missing `index`")? as usize;
            let value = json_to_library(&params.get("value").cloned().unwrap_or(Json::Null));
            let mut root = read_json(input)?;
            match resolve_mut(&mut root, &path) {
                Some(json::JsonValue::Array(items)) => {
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
                Some(json::JsonValue::Array(items)) if index < items.len() => {
                    items.remove(index);
                }
                Some(json::JsonValue::Array(_)) => return Err(format!("remove-array-element: index {index} out of bounds")),
                _ => return Err("remove-array-element: target at path is not an array".to_string()),
            }
            write_json(&root)
        }
        "set-scalar" => {
            let path = path_param(&params);
            let value = json_to_library(&params.get("value").cloned().unwrap_or(Json::Null));
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
/// 🔀️ `json::JsonValue` into the host's own minimal `Json` (`Number` is always `f64`, matching how
/// the comparison engine reads projections) — keys keep whatever order `json::object::Object`'s
/// hash-ordered tree iterates in, which is IGNORED at comparison time by the `ordered-json-v1`
/// profile (see the module doc comment: `json` does not preserve insertion order at all).
/// 🔢️ WORKED_AROUND_DEFECT, the mirror of [`library_number`] — `json` 0.12's `as_f64()` is not
/// exact either. It recomputes `mantissa * 10^exponent` in floating point, so a value the crate
/// parsed and stores correctly comes back rounded: the real fixture's
/// `-1.3283902924697095e-17` surface normal reads out as `…097e-17`. Reproduced standalone against
/// the crate alone. The crate's own `dump()` of the same value is exact — it prints the stored
/// decimal — so the conversion goes through that text and the standard library's correctly-rounded
/// `str::parse::<f64>`, falling back to the crate's accessor only if the text is not a Rust float
/// literal at all. Left as `as_f64()`, the reading drifts by one ULP per cycle and the inverse law
/// fails on a defect that is not this repository's.
#[cfg(feature = "oracles")]
fn host_number(value: &json::JsonValue) -> f64 {
    value.dump().parse::<f64>().unwrap_or_else(|_| value.as_f64().unwrap_or(f64::NAN))
}

#[cfg(feature = "oracles")]
pub fn project_value(value: &json::JsonValue) -> Json {
    match value {
        json::JsonValue::Null => Json::Null,
        json::JsonValue::Boolean(flag) => Json::Bool(*flag),
        json::JsonValue::Number(_) => Json::Number(host_number(value)),
        json::JsonValue::Short(_) | json::JsonValue::String(_) => Json::String(value.as_str().unwrap_or("").to_string()),
        json::JsonValue::Array(items) => Json::Array(items.iter().map(project_value).collect()),
        json::JsonValue::Object(object) => Json::Object(object.iter().map(|(key, value)| (key.to_string(), project_value(value))).collect()),
    }
}

/// 👁️ Projects JSON bytes with the INDEPENDENT `json` reader onto the `ordered-json-v1` shape this
/// case's oracle and subject are both compared through — key order is real in the wire form but not
/// in this projection (see the module doc comment).
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
/// "element `index` under `path`" (`extra = [Index(index)]`) without ever naming `json::JsonValue`
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

    /// 🔢️ Pins the worked-around `json` 0.12 defect and the fix together: the crate's own
    /// `From<f64>` moves these two real fixture coordinates by one ULP, and `library_number` does
    /// not. If a later release fixes the conversion, the first half of this test starts failing and
    /// the workaround can go.
    #[test]
    fn the_library_number_conversion_survives_a_round_trip_the_crates_own_does_not() {
        for value in [2.7000102824824506_f64, -8.881784197001252e-16] {
            assert_ne!(json::JsonValue::from(value).as_f64().unwrap().to_bits(), value.to_bits(), "json 0.12's own From<f64> is documented here as lossy for {value:?}");
            assert_eq!(library_number(value).as_f64().unwrap().to_bits(), value.to_bits(), "the workaround has to be exact for {value:?}");
        }
        for value in [0.1_f64, 1.0, 0.0, -0.0, 1e300, 1.0 / 3.0] {
            assert_eq!(library_number(value).as_f64().unwrap().to_bits(), value.to_bits(), "and exact for {value:?} as well");
        }
        assert!(library_number(f64::NAN).is_null(), "a non-finite double is not a JSON number");
    }

    /// 🔢️ The mirror defect and its fix: reading a real fixture coordinate back out of the crate.
    #[test]
    fn the_host_number_reading_is_exact_where_the_crates_own_accessor_is_not() {
        let document = json::parse("{\"v\": -1.3283902924697095e-17}").unwrap();
        assert_ne!(document["v"].as_f64().unwrap(), -1.3283902924697095e-17_f64, "json 0.12's own as_f64 is documented here as lossy for this real fixture coordinate");
        assert_eq!(host_number(&document["v"]), -1.3283902924697095e-17_f64, "the workaround has to read it back exactly");
        for text in ["0.1", "1", "-0.0", "1e300", "3.141592653589793", "4503599627370497"] {
            let probe = json::parse(&format!("{{\"v\": {text}}}")).unwrap();
            assert_eq!(host_number(&probe["v"]).to_bits(), text.parse::<f64>().unwrap().to_bits(), "and stay exact for {text}");
        }
    }
    fn obj(pairs: Vec<(&str, Json)>) -> Json {
        Json::Object(pairs.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
    }

    #[test]
    fn no_mutation_is_a_true_byte_identity() {
        let input = br#"{"a":1,"b":2}"#;
        let output = oracle_apply_mutation(input, &spec("no-mutation", Json::Object(vec![]))).unwrap();
        assert_eq!(output, input);
    }

    #[test]
    fn set_member_upserts_and_remove_member_deletes() {
        let input = br#"{"a":1,"nested":{"b":2}}"#;
        let updated = oracle_apply_mutation(input, &spec("set-member", obj(vec![("path", Json::Array(vec![Json::String("nested".into())])), ("key", Json::String("c".into())), ("value", Json::Number(3.0))]))).unwrap();
        let value = read_json(&updated).unwrap();
        assert_eq!(resolve(&value, &[PathSeg::Key("nested".into()), PathSeg::Key("c".into())]).and_then(|v| v.as_f64()), Some(3.0));

        let removed = oracle_apply_mutation(&updated, &spec("remove-member", obj(vec![("path", Json::Array(vec![Json::String("nested".into())])), ("key", Json::String("c".into()))]))).unwrap();
        let value = read_json(&removed).unwrap();
        assert!(resolve(&value, &[PathSeg::Key("nested".into()), PathSeg::Key("c".into())]).is_none());
        assert_eq!(resolve(&value, &[PathSeg::Key("nested".into()), PathSeg::Key("b".into())]).and_then(|v| v.as_f64()), Some(2.0));
    }

    #[test]
    fn insert_and_remove_array_element_are_inverse_on_a_real_shaped_array() {
        let input = br#"{"items":[1,2,3]}"#;
        let inserted = oracle_apply_mutation(input, &spec("insert-array-element", obj(vec![("path", Json::Array(vec![Json::String("items".into())])), ("index", Json::Number(1.0)), ("value", Json::Number(99.0))]))).unwrap();
        assert_eq!(project_json_value(&inserted).unwrap(), project_json_value(br#"{"items":[1,99,2,3]}"#).unwrap());

        let removed = oracle_apply_mutation(&inserted, &spec("remove-array-element", obj(vec![("path", Json::Array(vec![Json::String("items".into())])), ("index", Json::Number(1.0))]))).unwrap();
        assert_eq!(project_json_value(&removed).unwrap(), project_json_value(input).unwrap());
    }

    #[test]
    fn set_scalar_replaces_regardless_of_kind_incl_whole_document() {
        let input = br#"{"a":{"b":1}}"#;
        let output = oracle_apply_mutation(input, &spec("set-scalar", obj(vec![("path", Json::Array(vec![Json::String("a".into())])), ("value", Json::String("replaced".into()))]))).unwrap();
        assert_eq!(project_json_value(&output).unwrap(), project_json_value(br#"{"a":"replaced"}"#).unwrap());

        let whole = oracle_apply_mutation(input, &spec("set-scalar", obj(vec![("path", Json::Array(vec![])), ("value", Json::Bool(true))]))).unwrap();
        assert_eq!(project_json_value(&whole).unwrap(), project_json_value(b"true").unwrap());
    }

    #[test]
    fn set_snapshot_replaces_the_whole_document() {
        let input = br#"{"old":true}"#;
        let output = oracle_apply_mutation(input, &spec("set-snapshot", obj(vec![("value", obj(vec![("fresh", Json::Array(vec![Json::Number(1.0), Json::Number(2.0), Json::String("x".into())]))]))]))).unwrap();
        assert_eq!(project_json_value(&output).unwrap(), project_json_value(br#"{"fresh":[1,2,"x"]}"#).unwrap());
    }

    /// 🔤️ Where order-insensitivity actually comes from. The projection is a faithful record of what
    /// was parsed, so it PRESERVES member order — json-rust's `Object` is insertion-ordered, contrary
    /// to an earlier assumption here. RFC 8259 §4 declares member order insignificant, and what
    /// discharges that is the case's `ordered-json-v1` comparison profile, which ignores key order at
    /// compare time. Array order is significant per §5 and is preserved by both.
    ///
    /// The original form of this test asserted the projection itself normalized member order. It did
    /// not, and the test had never run — the crate's whole test target failed to build, so the claim
    /// went unchecked. Recorded here so the distinction is not quietly re-lost.
    #[test]
    fn projection_preserves_member_order_and_array_order() {
        let a = project_json_value(br#"{"a":1,"b":2}"#).unwrap();
        let b = project_json_value(br#"{"b":2,"a":1}"#).unwrap();
        assert_ne!(a, b, "the projection records what was parsed; the comparison profile is what makes member order insignificant");

        let arr_a = project_json_value(br#"[1,2]"#).unwrap();
        let arr_b = project_json_value(br#"[2,1]"#).unwrap();
        assert_ne!(arr_a, arr_b, "array order IS significant per RFC 8259 §5");
    }

    #[test]
    fn unknown_kind_is_an_error_never_a_silent_no_op() {
        let input = b"{}";
        let result = oracle_apply_mutation(input, &spec("not-a-real-kind", Json::Object(vec![])));
        assert!(result.is_err(), "an unrecognised kind must fail loudly");
    }
}
//#endregion 🧪️Tests
