//! 🦀️ PLY 1.0 mutation case — Rust adapter. Exhaustive: every declared `PlyMutation` kind
//! (`ply-1-0-any`, 10 kinds) gets a `mutate-<kind>` and an `inverse-<kind>` scenario, plus one
//! identity round trip. The oracle performs every kind by direct manipulation of `ply-rs`'s own
//! `Ply<DefaultElement>` model (`../../🏅️standards/🔖️1.0/🪆️subsets/✳️base/🦀️oracle.rs`,
//! independent of this subset's own decode/encode/mutation code); the subject fully parses into
//! `PlySnapshot` and re-serializes from it alone (no byte pass-through). Both results are read back
//! by the INDEPENDENT `ply-rs` reader before the `semantic-ply-v1` profile compares them.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::ply::standards::v1_0::subsets::any::{oracle_apply_mutation, project_ply};
use semio_s_plugin_stdio_test_oracle::law::{inverse_restores_within, mutation_is_observable_within, reparsed_not_copied, round_trip_preserves_within};

//#region 🔖️Kinds
/// 🏷️ Mirrors this subset's own `PlyMutation::KINDS` (`../../🏅️standards/🔖️1.0/🪆️subsets/✳️base/
/// 🧬️schema/🧬️mutations/🦀️.rs`). Kept as a plain literal here rather than imported since
/// this adapter's oracle-only build never links the subject crate — the contract gate (mutation
/// coverage against the `ply-1-0-any` catalog) is what keeps the two lists honest against each other.
const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-format", "insert-comment", "remove-comment", "add-element", "remove-element", "insert-row", "remove-row", "set-row-property"];
//#endregion 🔖️Kinds

//#region 🔖️Profile
/// 📏️ `semantic-ply-v1`'s own declared tolerance (`../../🏅️standards/🔖️1.0/🪆️subsets/✳️base/
/// 🔣️oracle.json`), mirrored here so an in-handler law check is exactly as strict as
/// the profile the case is measured by — never stricter.
const PLY_TOLERANCE: f64 = 1e-5;
//#endregion 🔖️Profile

//#region 🔖️Input
const INPUT: &str = "shared://🌐️pattern-sphere/🧊️.ply";

/// 🧫️ Copies the immutable committed document into the work directory and returns the mutable
/// copy's bytes; the committed fixture itself is never written to.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("📥️input.ply"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️JsonBuild
fn json_obj(entries: Vec<(&str, Json)>) -> Json {
    Json::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
}
fn json_num(value: f64) -> Json {
    Json::Number(value)
}
fn json_str(value: &str) -> Json {
    Json::String(value.to_string())
}
fn json_spec(kind: &str, params: Json) -> Json {
    json_obj(vec![("kind", json_str(kind)), ("params", params)])
}
fn json_row(values: Vec<Json>) -> Json {
    json_obj(vec![("values", Json::Array(values))])
}
fn json_property_def(name: &str, form: &str, kind_fields: Vec<(&str, Json)>) -> Json {
    let mut entries = vec![("name", json_str(name)), ("form", json_str(form))];
    entries.extend(kind_fields);
    json_obj(entries)
}
//#endregion 🔖️JsonBuild

//#region 🔖️RealEdgeTopology
/// 🕸️ The real 50 undirected edges `derive_ply.py` extracted from the fixture's own first 24 real
/// triangles (`../../🧫️fixtures/🌐️pattern-sphere/🧊️.ply`'s "edge" element) — `remove-element`'s inverse
/// re-adds this exact real element rather than an invented one.
const ORIGINAL_EDGES: [(i64, i64); 50] = [
    (1, 66), (0, 1), (0, 66), (2, 67), (1, 2), (1, 67), (66, 67), (3, 68), (2, 3), (2, 68), (67, 68), (4, 69), (3, 4), (3, 69), (68, 69), (5, 70), (4, 5), (4, 70), (69, 70), (6, 71), (5, 6), (5, 71), (70, 71), (7, 72), (6, 7), (6, 72), (71, 72), (8, 73), (7, 8), (7, 73), (72, 73), (9, 74), (8, 9), (8, 74), (73, 74), (10, 75), (9, 10), (9, 75), (74, 75), (11, 76), (10, 11), (10, 76), (75, 76), (12, 77), (11, 12), (11, 77), (76, 77), (13, 78), (12, 13), (12, 78),
];

fn original_edge_element() -> Json {
    json_obj(vec![
        ("name", json_str("edge")),
        ("count", json_num(ORIGINAL_EDGES.len() as f64)),
        ("properties", Json::Array(vec![json_property_def("v1", "scalar", vec![("kind", json_str("int"))]), json_property_def("v2", "scalar", vec![("kind", json_str("int"))])])),
        ("rows", Json::Array(ORIGINAL_EDGES.iter().map(|(a, b)| json_row(vec![json_num(*a as f64), json_num(*b as f64)])).collect())),
    ])
}

/// 📸️ Vertex 0's own real row (`x y z nx ny nz s t`) — the trailing orphan vertex (index 8448) is a
/// literal duplicate of this same real row, so `remove-row`'s inverse restores it exactly.
const VERTEX_0_ROW: [f64; 8] = [0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.00390625, 0.0];
//#endregion 🔖️RealEdgeTopology

//#region 🔖️Inverse
/// ↩️ The semantically correct inverse spec for one forward `(kind, params)` pair against the
/// pristine fixture's own known real values — index/name-aware, mirroring the same per-variant
/// `PlyMutation::inverse()` semantics `../../🏅️standards/🔖️1.0/🪆️subsets/✳️base/🧬️schema/🧬️mutations/
/// 🦀️.rs` documents, computed independently here since neither the oracle nor this adapter
/// can reach that subject-side method. `set-snapshot`'s inverse is a REAL `set-snapshot` carrying
/// the original document's own independent projection — which is exactly the payload shape this
/// subset's oracle consumes (`ply_from_json` reads the `{format, comments, elements}` object
/// `project_ply` emits) — never a hand-back of the pristine input bytes, which would let the
/// scenario pass without `ply-rs` re-serializing anything at all.
fn inverse_spec(kind: &str, base: &[u8]) -> Result<Json, String> {
    Ok(match kind {
        "set-snapshot" => json_spec("set-snapshot", json_obj(vec![("snapshot", project_ply(base)?)])),
        "no-mutation" => json_spec("no-mutation", json_obj(vec![])),
        "set-format" => json_spec("set-format", json_obj(vec![("format", json_str("ascii"))])),
        "insert-comment" => json_spec("remove-comment", json_obj(vec![("index", json_num(0.0))])),
        "remove-comment" => json_spec("insert-comment", json_obj(vec![("index", json_num(0.0)), ("comment", json_str("stdio.ply 1.0 real-world fixture, derived once from real committed geometry."))])),
        "add-element" => json_spec("remove-element", json_obj(vec![("name", json_str("material"))])),
        "remove-element" => json_spec("add-element", json_obj(vec![("index", json_num(2.0)), ("element", original_edge_element())])),
        "insert-row" => json_spec("remove-row", json_obj(vec![("elementName", json_str("vertex")), ("index", json_num(8449.0))])),
        "remove-row" => json_spec("insert-row", json_obj(vec![("elementName", json_str("vertex")), ("index", json_num(8448.0)), ("row", json_row(VERTEX_0_ROW.iter().map(|v| json_num(*v)).collect()))])),
        "set-row-property" => json_spec("set-row-property", json_obj(vec![("elementName", json_str("vertex")), ("rowIndex", json_num(0.0)), ("propertyName", json_str("x")), ("value", json_num(0.0))])),
        other => json_spec(other, json_obj(vec![])),
    })
}
//#endregion 🔖️Inverse

//#region 🔖️Oracle
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_ply(&bytes)?;
    mutation_is_observable_within(&spec.str("kind"), &projection, &project_ply(&input)?, &[], &[], PLY_TOLERANCE)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ The inverse law, asserted HERE rather than deferred to the parity phase: every kind —
/// INCLUDING `set-snapshot`, which now inverts through a real `set-snapshot` of the original
/// document instead of returning the pristine bytes — is applied forward and then undone, and the
/// restored document's independent `ply-rs` projection must equal the REAL original's own, within
/// `semantic-ply-v1`'s own declared tolerance and no stricter.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let restored = oracle_apply_mutation(&mutated, &inverse_spec(&kind, &input)?)?;
    let projection = project_ply(&restored)?;
    inverse_restores_within(&kind, &projection, &project_ply(&input)?, &[], PLY_TOLERANCE)?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔁️ The identity law, both halves asserted in role: `ply-rs` parses the real document into its
/// own `Ply<DefaultElement>` and re-serializes from that model alone, so the projection must be
/// preserved AND the output must not be the input bytes back — the writer re-derives the whole
/// header and re-formats every ASCII payload value, so bit-identical output would mean nothing was
/// parsed.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let bytes = oracle_apply_mutation(&input, &json_spec("no-mutation", json_obj(vec![])))?;
    reparsed_not_copied(&bytes, &input)?;
    let projection = project_ply(&bytes)?;
    round_trip_preserves_within(&projection, &project_ply(&input)?, &[], PLY_TOLERANCE)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{inverse_spec, mutable_input};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::ply::standards::v1_0::subsets::any::io::{decode_ply, encode_ply_with_format};
    use semio_s_plugin_stdio::artifacts::ply::standards::v1_0::subsets::any::schema::mutations::{add_element, apply_ply_mutation, insert_comment, insert_row, remove_comment, remove_element, remove_row, set_format, set_row_property, set_snapshot, PlyMutation};
    use semio_s_plugin_stdio::artifacts::ply::standards::v1_0::subsets::any::schema::snapshot::{PlyElement, PlyFormat, PlyProperty, PlyRow, PlyScalarType, PlySnapshot, PlyValue};
    use semio_s_plugin_stdio_test_oracle::artifacts::ply::standards::v1_0::subsets::any::project_ply;

    //#region 🔖️SpecReading
    fn json_num(value: &Json, key: &str) -> Option<f64> {
        match value.get(key) {
            Some(Json::Number(number)) => Some(*number),
            _ => None,
        }
    }
    fn json_str(value: &Json, key: &str) -> Option<String> {
        match value.get(key) {
            Some(Json::String(text)) => Some(text.clone()),
            _ => None,
        }
    }
    fn json_usize(value: &Json, key: &str) -> Result<usize, String> {
        json_num(value, key).map(|number| number as usize).ok_or_else(|| format!("expected numeric field {key:?}"))
    }
    //#endregion 🔖️SpecReading

    //#region 🔖️TypeParsing
    fn scalar_type_from_json(value: &str) -> Result<PlyScalarType, String> {
        Ok(match value {
            "char" => PlyScalarType::Char,
            "uChar" => PlyScalarType::UChar,
            "short" => PlyScalarType::Short,
            "uShort" => PlyScalarType::UShort,
            "int" => PlyScalarType::Int,
            "uInt" => PlyScalarType::UInt,
            "float" => PlyScalarType::Float,
            "double" => PlyScalarType::Double,
            other => return Err(format!("unknown ply scalar type {other:?}")),
        })
    }

    fn format_from_json(value: &str) -> Result<PlyFormat, String> {
        Ok(match value {
            "ascii" => PlyFormat::Ascii,
            "binaryLittleEndian" => PlyFormat::BinaryLittleEndian,
            "binaryBigEndian" => PlyFormat::BinaryBigEndian,
            other => return Err(format!("unknown ply format {other:?}")),
        })
    }

    fn property_from_json(value: &Json) -> Result<PlyProperty, String> {
        let name = json_str(value, "name").ok_or("property requires a name")?;
        match json_str(value, "form").ok_or("property requires a form")?.as_str() {
            "scalar" => Ok(PlyProperty::Scalar { name, kind: scalar_type_from_json(&json_str(value, "kind").ok_or("scalar property requires a kind")?)? }),
            "list" => Ok(PlyProperty::List { name, count_kind: scalar_type_from_json(&json_str(value, "countKind").ok_or("list property requires a countKind")?)?, value_kind: scalar_type_from_json(&json_str(value, "valueKind").ok_or("list property requires a valueKind")?)? }),
            other => Err(format!("unknown property form {other:?}")),
        }
    }

    fn value_from_json(value: &Json, kind: &PlyScalarType) -> Result<PlyValue, String> {
        let number = match value {
            Json::Number(n) => *n,
            other => return Err(format!("expected a number, found {other:?}")),
        };
        Ok(match kind {
            PlyScalarType::Char => PlyValue::Char(number as i8),
            PlyScalarType::UChar => PlyValue::UChar(number as u8),
            PlyScalarType::Short => PlyValue::Short(number as i16),
            PlyScalarType::UShort => PlyValue::UShort(number as u16),
            PlyScalarType::Int => PlyValue::Int(number as i32),
            PlyScalarType::UInt => PlyValue::UInt(number as u32),
            PlyScalarType::Float => PlyValue::Float(number as f32),
            PlyScalarType::Double => PlyValue::Double(number),
        })
    }

    fn cell_from_json(value: &Json, property: &PlyProperty) -> Result<PlyValue, String> {
        match property {
            PlyProperty::Scalar { kind, .. } => value_from_json(value, kind),
            PlyProperty::List { value_kind, .. } => {
                let Json::Array(items) = value else { return Err(format!("expected an array for a list property, found {value:?}")) };
                Ok(PlyValue::List(items.iter().map(|item| value_from_json(item, value_kind)).collect::<Result<Vec<_>, String>>()?))
            }
        }
    }

    fn row_from_json(value: &Json, properties: &[PlyProperty]) -> Result<PlyRow, String> {
        let values = match value.get("values") {
            Some(Json::Array(items)) => items.clone(),
            _ => return Err("row requires a values array".to_string()),
        };
        if values.len() != properties.len() {
            return Err(format!("row expects {} values, got {}", properties.len(), values.len()));
        }
        Ok(PlyRow { values: properties.iter().zip(values.iter()).map(|(property, cell)| cell_from_json(cell, property)).collect::<Result<Vec<_>, String>>()? })
    }

    fn element_from_json(value: &Json) -> Result<PlyElement, String> {
        let name = json_str(value, "name").ok_or("element requires a name")?;
        let properties = match value.get("properties") {
            Some(Json::Array(items)) => items.iter().map(property_from_json).collect::<Result<Vec<_>, String>>()?,
            _ => Vec::new(),
        };
        let rows = match value.get("rows") {
            Some(Json::Array(items)) => items.iter().map(|row| row_from_json(row, &properties)).collect::<Result<Vec<_>, String>>()?,
            _ => Vec::new(),
        };
        Ok(PlyElement { name, count: rows.len(), properties, rows })
    }
    //#endregion 🔖️TypeParsing

    //#region 🔖️MutationFromSpec
    /// 🦠️ The same `(kind, params)` wire shape the oracle dispatcher reads, translated into a real
    /// `PlyMutation` value for this subset's own `apply_ply_mutation`. `insert-row`/`set-row-property`
    /// resolve their target property's declared type against `snapshot` (the document the mutation is
    /// about to be applied to) — the same way the oracle module resolves it against `ply.header`
    /// before decoding a cell — rather than guessing a type from the JSON value alone.
    fn mutation_from_spec(spec: &Json, snapshot: &PlySnapshot) -> Result<PlyMutation, String> {
        let kind = json_str(spec, "kind").unwrap_or_default();
        let empty = Json::Object(Vec::new());
        let params = spec.get("params").unwrap_or(&empty);
        Ok(match kind.as_str() {
            // 🧭️ `PlyMutation::NoMutation` is gone (mutation-leaf migration,
            // `../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/S-END-TO-END/📓️plan-mutation-leaf-migration.md`),
            // so `no-mutation` maps to a real `SetSnapshot(snapshot)` — a genuine identity mutation.
            "no-mutation" => PlyMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: snapshot.clone() }),
            "set-snapshot" => {
                let snapshot_json = params.get("snapshot").ok_or("set-snapshot requires a snapshot field")?;
                let elements = match snapshot_json.get("elements") {
                    Some(Json::Array(items)) => items.iter().map(element_from_json).collect::<Result<Vec<_>, String>>()?,
                    _ => Vec::new(),
                };
                let mut next = semio_s_plugin_stdio::artifacts::ply::standards::v1_0::subsets::any::schema::snapshot::PlySnapshot::default();
                next.format = format_from_json(&json_str(snapshot_json, "format").unwrap_or_else(|| "ascii".to_string()))?;
                next.comments = string_array(snapshot_json, "comments");
                next.elements = elements;
                PlyMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: next })
            }
            "set-format" => PlyMutation::SetFormat(set_format::SetFormat { format: format_from_json(&json_str(params, "format").ok_or("set-format requires a format field")?)? }),
            "insert-comment" => PlyMutation::InsertComment(insert_comment::InsertComment { index: json_usize(params, "index")?, comment: json_str(params, "comment").ok_or("insert-comment requires a comment field")? }),
            "remove-comment" => PlyMutation::RemoveComment(remove_comment::RemoveComment { index: json_usize(params, "index")? }),
            "add-element" => PlyMutation::AddElement(add_element::AddElement { index: json_usize(params, "index")?, element: element_from_json(params.get("element").ok_or("add-element requires an element field")?)? }),
            "remove-element" => PlyMutation::RemoveElement(remove_element::RemoveElement { name: json_str(params, "name").ok_or("remove-element requires a name field")? }),
            "insert-row" => {
                let element_name = json_str(params, "elementName").ok_or("insert-row requires an elementName field")?;
                let properties = element_properties(snapshot, &element_name)?;
                let row = row_from_json(params.get("row").ok_or("insert-row requires a row field")?, &properties)?;
                PlyMutation::InsertRow(insert_row::InsertRow { element_name, index: json_usize(params, "index")?, row })
            }
            "remove-row" => PlyMutation::RemoveRow(remove_row::RemoveRow { element_name: json_str(params, "elementName").ok_or("remove-row requires an elementName field")?, index: json_usize(params, "index")? }),
            "set-row-property" => {
                let element_name = json_str(params, "elementName").ok_or("set-row-property requires an elementName field")?;
                let property_name = json_str(params, "propertyName").ok_or("set-row-property requires a propertyName field")?;
                let properties = element_properties(snapshot, &element_name)?;
                let property = properties.iter().find(|property| property_name_of(property) == property_name).ok_or_else(|| format!("no property named {property_name:?} on element {element_name:?}"))?;
                let value_json = params.get("value").cloned().unwrap_or(Json::Null);
                PlyMutation::SetRowProperty(set_row_property::SetRowProperty { element_name, row_index: json_usize(params, "rowIndex")?, property_name, value: cell_from_json(&value_json, property)? })
            }
            other => return Err(format!("unrecognised mutation kind {other:?}")),
        })
    }

    fn property_name_of(property: &PlyProperty) -> &str {
        match property {
            PlyProperty::Scalar { name, .. } => name,
            PlyProperty::List { name, .. } => name,
        }
    }

    fn element_properties(snapshot: &PlySnapshot, element_name: &str) -> Result<Vec<PlyProperty>, String> {
        snapshot.elements.iter().find(|element| element.name == element_name).map(|element| element.properties.clone()).ok_or_else(|| format!("no element named {element_name:?}"))
    }

    fn string_array(value: &Json, key: &str) -> Vec<String> {
        match value.get(key) {
            Some(Json::Array(items)) => items
                .iter()
                .map(|item| match item {
                    Json::String(text) => text.clone(),
                    _ => String::new(),
                })
                .collect(),
            _ => Vec::new(),
        }
    }
    //#endregion 🔖️MutationFromSpec

    //#region 🔖️Codec
    /// 📐️ Full parse → typed mutation → re-serialize from the model alone, IN THE SNAPSHOT'S OWN
    /// DECLARED FORMAT. `encode_ply` is the ascii-forcing convenience (`encode_ply_with_format(…,
    /// PlyFormat::Ascii)`) and calling it here discarded `snapshot.format` outright, so
    /// `set-format` → `binaryLittleEndian` moved the typed model and then wrote plain ascii with
    /// an `format ascii 1.0` header: the mutation was unobservable in the document and the
    /// reference's real binary output had nothing on our side to be compared against. The
    /// production pack path already reads it this way — `🧬️schema/📸️snapshot/🦀️.rs`
    /// calls `encode_ply_with_format(self, self.format)` and names the ascii-forcing call as the
    /// hazard — and this adapter now mirrors it (ticket `26/08/23/END-TO-END-TESTING-REFACTOR`,
    /// subject scenario `inverse-set-format`).
    fn encode_in_declared_format(snapshot: &PlySnapshot) -> Result<Vec<u8>, String> {
        encode_ply_with_format(snapshot, snapshot.format).map_err(|error| format!("encode_ply_with_format failed: {error}"))
    }

    /// 📐️ One step of the pipeline: parse, apply, re-serialize. Carries NO byte tripwire — see
    /// [`apply_and_encode`] for where that rule belongs.
    fn mutate_and_encode(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
        let mut snapshot = decode_ply(input).map_err(|error| format!("decode_ply failed: {error}"))?;
        let mutation = mutation_from_spec(spec, &snapshot)?;
        apply_ply_mutation(&mut snapshot, &mutation);
        encode_in_declared_format(&snapshot)
    }

    /// 📐️ [`mutate_and_encode`] plus the no-byte-pass-through rule this wave exists to enforce,
    /// applied to the step that reads the REAL COMMITTED FIXTURE — a foreign writer's bytes, which
    /// this codec's normal form cannot reproduce, so bit-identical output there means the input was
    /// smuggled rather than parsed. It is deliberately NOT applied to the undo step: that step
    /// re-encodes THIS codec's own first-generation output, and `🚪️io/🦀️.rs` documents
    /// decode/encode as a true fixed point from the second generation onward, so an undo that
    /// restores the same model is REQUIRED to reproduce those bytes. Asserting the tripwire there
    /// failed `inverse-no-mutation` for the codec behaving exactly as its own retention law says it
    /// must. The pristine-input half of the law still runs, unweakened, in [`round_trip`] and in
    /// every forward step below.
    fn apply_and_encode(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
        let bytes = mutate_and_encode(input, spec)?;
        if bytes == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        Ok(bytes)
    }
    //#endregion 🔖️Codec

    //#region 🔖️Handlers
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let spec = ctx.doc_json()?;
        let bytes = apply_and_encode(&input, &spec)?;
        let projection = project_ply(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// ↩️ Every kind, INCLUDING `set-snapshot`, is genuinely applied forward and then undone through
    /// this repository's own `PlyMutation` pipeline — `set-snapshot` inverts through a real
    /// `set-snapshot` carrying the original document's independent projection, never through a
    /// hand-back of the pristine input bytes.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let spec = ctx.doc_json()?;
        let kind = json_str(&spec, "kind").unwrap_or_default();
        let mutated = apply_and_encode(&input, &spec)?;
        let restored = mutate_and_encode(&mutated, &inverse_spec(&kind, &input)?)?;
        let projection = project_ply(&restored)?;
        Ok(Outcome::with_raw(restored, projection))
    }

    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode_ply(&input).map_err(|error| format!("decode_ply failed: {error}"))?;
        let bytes = encode_in_declared_format(&snapshot)?;
        if bytes == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_ply(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle).oracle(&format!("inverse-{kind}"), inverse_oracle);
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate).subject(&format!("inverse-{kind}"), subject::inverse);
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
