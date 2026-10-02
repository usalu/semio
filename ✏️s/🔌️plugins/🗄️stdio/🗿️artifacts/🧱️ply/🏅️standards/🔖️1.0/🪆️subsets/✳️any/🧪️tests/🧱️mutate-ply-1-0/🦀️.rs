//! 🦀️ PLY 1.0 mutation case — Rust adapter. Exhaustive: every declared `PlyMutation` kind
//! (`ply-1-0-any`, 9 kinds) gets a `mutate-<kind>` and an `inverse-<kind>` scenario, plus one
//! identity round trip. The oracle performs every kind by direct manipulation of `ply-rs`'s own
//! `Ply<DefaultElement>` model (`../../🏅️standards/🔖️1.0/🪆️subsets/✳️base/🦀️oracle.rs`,
//! independent of this subset's own decode/encode/mutation code); the subject fully parses into
//! `PlySnapshot` and re-serializes from it alone (no byte pass-through). Both results are read back
//! by the INDEPENDENT `ply-rs` reader before the `semantic-ply-v1` profile compares them.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_artifact_stdio_ply_test_oracle::standards::v1_0::subsets::any::{oracle_apply_mutation, oracle_round_trip, ply_snapshot_wire, project_ply};
use semio_repo_test_host::law::{inverse_restores_within, mutation_is_observable_within, reparsed_not_copied, round_trip_preserves_within};


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
fn json_spec(kind: &str, params: Json) -> Json {
    json_obj(vec![("kind", Json::String(kind.to_string())), ("params", params)])
}
//#endregion 🔖️JsonBuild

//#region 🔖️Inverse
/// ↩️ The semantically correct inverse spec for one forward `(kind, params)` pair, mirroring the per-variant
/// `PlyMutation::inverse()` semantics independently: whatever pre-mutation state it needs is read out of `base` through
/// the oracle's own `ply-rs` reading ([`ply_snapshot_wire`]), and every spec it returns carries the leaf wire payload
/// like the feature's rows. `set-snapshot`'s inverse is a REAL `set-snapshot` carrying the original document — never a
/// hand-back of the pristine input bytes, which would let the scenario pass without `ply-rs` re-serializing anything.
fn inverse_spec(spec: &Json, base: &[u8]) -> Result<Json, String> {
    let kind = spec.str("kind");
    let params = spec.get("params").cloned().unwrap_or(Json::Null);
    let snapshot = ply_snapshot_wire(base)?;
    let elements = snapshot.array("elements");
    let number = |key: &str| match params.get(key) {
        Some(Json::Number(number)) => Ok(*number as usize),
        _ => Err(format!("{kind} requires a numeric {key:?}")),
    };
    let element = |name: &str| elements.iter().position(|element| element.str("name") == name).map(|at| (at, elements[at].clone())).ok_or_else(|| format!("the real document declares no element {name:?}"));
    let at = |items: Vec<Json>, index: usize, what: &str| items.get(index).cloned().ok_or_else(|| format!("the real document has no {what} {index}"));
    Ok(match kind.as_str() {
        "set-snapshot" => json_spec("set-snapshot", json_obj(vec![("snapshot", snapshot.clone())])),
        "set-format" => json_spec("set-format", json_obj(vec![("format", Json::String(snapshot.str("format")))])),
        "insert-comment" => json_spec("remove-comment", json_obj(vec![("index", Json::Number(number("index")?.min(snapshot.array("comments").len()) as f64))])),
        "remove-comment" => {
            let index = number("index")?;
            json_spec("insert-comment", json_obj(vec![("index", Json::Number(index as f64)), ("comment", at(snapshot.array("comments"), index, "comment")?)]))
        }
        "add-element" => json_spec("remove-element", json_obj(vec![("name", Json::String(params.get("element").map(|element| element.str("name")).unwrap_or_default()))])),
        "remove-element" => {
            let (index, removed) = element(&params.str("name"))?;
            json_spec("add-element", json_obj(vec![("index", Json::Number(index as f64)), ("element", removed)]))
        }
        "insert-row" => {
            let rows = element(&params.str("elementName"))?.1.array("rows");
            json_spec("remove-row", json_obj(vec![("elementName", Json::String(params.str("elementName"))), ("index", Json::Number(number("index")?.min(rows.len()) as f64))]))
        }
        "remove-row" => {
            let index = number("index")?;
            let row = at(element(&params.str("elementName"))?.1.array("rows"), index, "row")?;
            json_spec("insert-row", json_obj(vec![("elementName", Json::String(params.str("elementName"))), ("index", Json::Number(index as f64)), ("row", row)]))
        }
        "set-row-property" => {
            let (_, target) = element(&params.str("elementName"))?;
            let row_index = number("rowIndex")?;
            let property_name = params.str("propertyName");
            let column = target.array("properties").iter().position(|property| property.str("name") == property_name).ok_or_else(|| format!("the real element declares no property {property_name:?}"))?;
            let value = at(at(target.array("rows"), row_index, "row")?.array("values"), column, "column")?;
            json_spec("set-row-property", json_obj(vec![("elementName", Json::String(params.str("elementName"))), ("rowIndex", Json::Number(row_index as f64)), ("propertyName", Json::String(property_name)), ("value", value)]))
        }
        other => return Err(format!("no inverse rule for kind {other:?}")),
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
    let restored = oracle_apply_mutation(&mutated, &inverse_spec(&spec, &input)?)?;
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
    let bytes = oracle_round_trip(&input)?;
    reparsed_not_copied(&bytes, &input)?;
    let projection = project_ply(&bytes)?;
    round_trip_preserves_within(&projection, &project_ply(&input)?, &[], PLY_TOLERANCE)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::mutable_input;
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_ply::standards::v1_0::subsets::any::io::{decode_ply, encode_ply_with_format};
    use semio_repo_test_host::law::wire_operation;
    use semio_s_artifact_stdio_ply::{mutation_from_payload_json, mutation_inverse, mutation_payload_json};
    use semio_s_artifact_stdio_ply::standards::v1_0::subsets::any::schema::mutations::{apply_ply_mutation, PlyMutation};
    use semio_s_artifact_stdio_ply::standards::v1_0::subsets::any::schema::snapshot::PlySnapshot;
    use semio_s_artifact_stdio_ply_test_oracle::standards::v1_0::subsets::any::project_ply;

    /// 🔀️ The spec's wire payload, decoded by the aggregate's own generic payload constructor.
    fn mutation_of(spec: &Json) -> Result<PlyMutation, String> {
        wire_operation(&spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null), mutation_from_payload_json, mutation_payload_json)
    }

    fn decode(input: &[u8]) -> Result<PlySnapshot, String> {
        decode_ply(input).map_err(|error| format!("decode_ply failed: {error}"))
    }

    /// 📐️ Re-serializes from the model alone IN THE SNAPSHOT'S OWN DECLARED FORMAT — the production pack path's
    /// `encode_ply_with_format(self, self.format)`; the ascii-forcing `encode_ply` would make `set-format` unobservable.
    fn encode_in_declared_format(snapshot: &PlySnapshot) -> Result<Vec<u8>, String> {
        encode_ply_with_format(snapshot, snapshot.format).map_err(|error| format!("encode_ply_with_format failed: {error}"))
    }

    /// 📐️ The forward step reads the REAL COMMITTED FIXTURE — a foreign writer's bytes this codec's normal form cannot
    /// reproduce — so bit-identical output there means the input was smuggled rather than parsed.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let mut snapshot = decode(&input)?;
        apply_ply_mutation(&mut snapshot, &mutation_of(&ctx.doc_json()?)?);
        let bytes = encode_in_declared_format(&snapshot)?;
        if bytes == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_ply(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// ↩️ Every kind, INCLUDING `set-snapshot`, is applied forward and then undone by the production inverse computed
    /// against the pre-mutation snapshot.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let mut snapshot = decode(&mutable_input(ctx)?)?;
        let forward = mutation_of(&ctx.doc_json()?)?;
        let backward = mutation_inverse(&forward, &snapshot);
        apply_ply_mutation(&mut snapshot, &forward);
        for mutation in &backward {
            apply_ply_mutation(&mut snapshot, mutation);
        }
        let restored = encode_in_declared_format(&snapshot)?;
        let projection = project_ply(&restored)?;
        Ok(Outcome::with_raw(restored, projection))
    }

    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let bytes = encode_in_declared_format(&decode(&input)?)?;
        if bytes == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_ply(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    built = built.oracle("mutate", mutate_oracle).oracle("inverse", inverse_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("mutate", subject::mutate).subject("inverse", subject::inverse);
    }
    built = built.oracle("identity-round-trip", round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
