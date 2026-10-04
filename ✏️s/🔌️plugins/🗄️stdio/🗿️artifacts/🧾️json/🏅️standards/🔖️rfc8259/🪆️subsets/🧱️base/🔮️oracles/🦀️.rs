//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed by the
//! registered `json` (json-rust) 0.12 reference implementation so the subject's own mutation has an
//! independent result to be compared against instead of being checked against its own reading.
//!
//! `serde_json`, not `json`, was the first choice here, and it was wrong: this subset's OWN
//! production code (`../🧬️schema/📸️snapshot/🦀️.rs`) already declares
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
//! @see ../🔣️oracle.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the mutation vocabulary itself.

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

/// 🔀️ A `path` wire value — `JsonPathSegment`s (`{"kind":"key","value":<name>}` / `{"kind":"index","value":<n>}`), the
/// leaf payload schema's own path shape — into `PathSeg`s.
#[cfg(feature = "oracles")]
pub fn path_from_spec(path: &Json) -> Vec<PathSeg> {
    match path {
        Json::Array(segments) => segments
            .iter()
            .map(|segment| match (segment.str("kind").as_str(), segment.get("value")) {
                ("index", Some(Json::Number(index))) => PathSeg::Index(index.max(0.0) as usize),
                _ => PathSeg::Key(segment.str("value")),
            })
            .collect(),
        _ => Vec::new(),
    }
}

/// 🔎️ Read-only navigation of `path` from `root`, `None` on the first unresolvable segment.
#[cfg(feature = "oracles")]
pub(crate) fn resolve<'a>(root: &'a json::JsonValue, path: &[PathSeg]) -> Option<&'a json::JsonValue> {
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
pub(crate) fn resolve_mut<'a>(root: &'a mut json::JsonValue, path: &[PathSeg]) -> Option<&'a mut json::JsonValue> {
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
pub(crate) fn read_json(input: &[u8]) -> Result<json::JsonValue, String> {
    let text = std::str::from_utf8(input).map_err(|error| format!("independent reader: input is not UTF-8: {error}"))?;
    json::parse(text).map_err(|error| format!("independent reader could not parse JSON: {error}"))
}

/// 📤️ Independent RFC 8259 write via the reference implementation — `json`'s own compact form
/// (`dump`) and number formatting, never this subset's own writer.
#[cfg(feature = "oracles")]
pub(crate) fn write_json(value: &json::JsonValue) -> Result<Vec<u8>, String> {
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
/// 🌳 A tagged `JsonValue` wire value (`{"kind":"null"|"bool"|"number"|"string"|"array"|"object", ...}`) into
/// `json::JsonValue`. A number is parsed from its verbatim `lexeme` by the reference parser itself, which is exact, so
/// no lossy `f64` ever stands between the wire and the reference implementation.
#[cfg(feature = "oracles")]
fn library_from_wire(value: &Json) -> Result<json::JsonValue, String> {
    Ok(match value.str("kind").as_str() {
        "null" => json::JsonValue::Null,
        "bool" => json::JsonValue::from(matches!(value.get("value"), Some(Json::Bool(true)))),
        "number" => json::parse(&value.str("lexeme")).map_err(|error| format!("number lexeme {:?}: {error}", value.str("lexeme")))?,
        "string" => json::JsonValue::from(value.str("value")),
        "array" => json::JsonValue::Array(value.array("items").iter().map(library_from_wire).collect::<Result<_, _>>()?),
        "object" => {
            let members = value.array("members");
            let mut object = json::object::Object::with_capacity(members.len());
            for member in &members {
                object.insert(&member.str("key"), library_from_wire(member.get("value").unwrap_or(&Json::Null))?);
            }
            json::JsonValue::Object(object)
        }
        other => return Err(format!("JsonValue wire: unrecognised kind {other:?}")),
    })
}

/// 🌲️ A `json::JsonValue` as the tagged `JsonValue` wire [`library_from_wire`] reads (a number as the lexeme json-rust prints).
#[cfg(feature = "oracles")]
fn library_to_wire(value: &json::JsonValue) -> Json {
    let tagged = |kind: &str, member: Option<(&str, Json)>| Json::Object(std::iter::once(("kind".to_string(), Json::String(kind.to_string()))).chain(member.map(|(key, value)| (key.to_string(), value))).collect());
    match value {
        json::JsonValue::Null => tagged("null", None),
        json::JsonValue::Boolean(flag) => tagged("bool", Some(("value", Json::Bool(*flag)))),
        json::JsonValue::Number(_) => tagged("number", Some(("lexeme", Json::String(value.dump())))),
        json::JsonValue::Short(_) | json::JsonValue::String(_) => tagged("string", Some(("value", Json::String(value.as_str().unwrap_or_default().to_string())))),
        json::JsonValue::Array(items) => tagged("array", Some(("items", Json::Array(items.iter().map(library_to_wire).collect())))),
        json::JsonValue::Object(object) => tagged("object", Some(("members", Json::Array(object.iter().map(|(key, member)| Json::Object(vec![("key".to_string(), Json::String(key.to_string())), ("value".to_string(), library_to_wire(member))])).collect())))),
    }
}

/// 🩹️ The reference's own `JsonSnapshot` reading of `input` (`{schema, value}`) that a `patch-snapshot` row's pointer
/// operation addresses, and that the case adapter restores the original through (`restore-snapshot`).
#[cfg(feature = "oracles")]
pub fn snapshot_wire(input: &[u8]) -> Result<Json, String> {
    Ok(Json::Object(vec![("schema".to_string(), Json::String("stdio.json".to_string())), ("value".to_string(), library_to_wire(&read_json(input)?))]))
}

/// 🌳 The `value` member of a leaf payload, read through [`library_from_wire`].
#[cfg(feature = "oracles")]
fn value_param(params: &Json) -> Result<json::JsonValue, String> {
    library_from_wire(params.get("value").unwrap_or(&Json::Null))
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
        "set-member" => {
            let path = path_param(&params);
            let key = params.str("key");
            let value = value_param(&params)?;
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
            let value = value_param(&params)?;
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
            let value = value_param(&params)?;
            let mut root = read_json(input)?;
            match resolve_mut(&mut root, &path) {
                Some(node) => *node = value,
                None => return Err("set-scalar: target path does not resolve".to_string()),
            }
            write_json(&root)
        }
        "patch-snapshot" => {
            let patched = semio_repo_test_host::law::patched_snapshot(&snapshot_wire(input)?, params.get("patch").ok_or("patch-snapshot: missing `patch`")?)?;
            write_json(&library_from_wire(patched.get("value").unwrap_or(&Json::Null))?)
        }
        "restore-snapshot" => write_json(&library_from_wire(params.get("snapshot").and_then(|snapshot| snapshot.get("value")).unwrap_or(&Json::Null))?),
        kind => Err(format!("mutation kind {kind:?} has no oracle implementation ({} input byte(s))", input.len())),
    }
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn snapshot_wire(_input: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
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
/// 🔢️ WORKED_AROUND_DEFECT — `json` 0.12's `as_f64()` is not exact (the input side never converts
/// through `f64` at all: [`library_from_wire`] parses the verbatim lexeme). It recomputes `mantissa * 10^exponent` in floating point, so a value the crate
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
pub(crate) fn project_value(value: &json::JsonValue) -> Json {
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

/// 🔎️ Reads the PROJECTED value found at `path` (a mutation spec's own wire `path` param) with `extra`
/// segments appended first — so an adapter deriving an
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
