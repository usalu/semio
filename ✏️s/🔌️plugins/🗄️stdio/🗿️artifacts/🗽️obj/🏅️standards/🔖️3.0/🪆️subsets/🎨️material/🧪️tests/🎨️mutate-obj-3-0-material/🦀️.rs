//! 🦀️ OBJ 3.0 material-subset mutation case — Rust adapter. Exhaustive for THIS subset's own
//! 2-kind vocabulary (`set-mtllib`, `set-usemtl` — `../../🔮️oracle/🔣️.json`'s `obj-3.0-material` catalog): every declared kind gets a `mutate-<kind>` and
//! an `inverse-<kind>` scenario, plus one identity round trip. The oracle performs each kind by
//! direct OBJ-grammar manipulation (`../../../📐️geometry/🔮️oracle/🦀️.rs`,
//! independent of this subset's own decode/encode/mutation code); the subject fully parses into
//! `ObjSnapshot` and re-serializes from it alone (no byte pass-through). Both results are read back
//! by the INDEPENDENT `tobj` reader before the `semantic-obj-document-v1` profile compares them.
//!
//! 🩹️ Ticket `26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-
//! MUTATION`, shard F3: before this fix (and before this case's own relocation out of artifact
//! level into this subset), this file was a copy-pasted duplicate of `✳️any/🧪️tests/📐️mutate-obj-3-0/
//! 🦀️.rs` (the geometry subset's own 22-kind exhaustive case) — it declared and registered handlers
//! for all 22 `ObjMutation` kinds while the `.feature` beside it only ever names 2
//! (`set-mtllib`/`set-usemtl`), so 20 of the 44 registered handlers were unreachable dead code, and
//! every vertex/texcoord/normal/face/group/object helper existed only to serve them. Trimmed to the
//! 2 kinds this subset actually owns; `SetMtllib`/`SetUsemtl` still come from `subsets::any::schema::
//! mutations` because that's genuinely where they're implemented (OBJ's mutation vocabulary has not
//! been physically split per subset — the same shared-implementation-with-manifest-level-ownership
//! shape this ticket's own gltf 2.0 subset split already established, see `📓️a6-gltf-png-bmp-
//! subsets.md`), not a second copy.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::obj::standards::v3_0::subsets::any::{oracle_apply_mutation, oracle_document_projection};
use semio_s_plugin_stdio_test_oracle::law::{inverse_restores_within, mutation_is_observable_within, reparsed_not_copied, round_trip_preserves_within};
use semio_s_plugin_stdio_test_oracle::mesh::project_obj;

//#region 🔖️Kinds
/// 🏷️ Mirrors this subset's own `obj-3.0-material` catalog `kinds` (`../../🔮️oracle/🔣️.json`). Kept as a plain literal here rather than imported since
/// this adapter's oracle-only build never links the subject crate — the contract gate (mutation
/// coverage against that catalog) is what keeps the two lists honest against each other.
const KINDS: &[&str] = &["set-mtllib", "set-usemtl"];
//#endregion 🔖️Kinds

//#region 🔖️Input
const INPUT: &str = "shared://🧪️pattern-sphere/🧊️.obj";

/// 🧫️ Copies the immutable committed mesh into the work directory and returns the mutable copy's
/// bytes; the committed fixture itself is never written to.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("input.obj"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️JsonBuild
fn json_obj(entries: Vec<(&str, Json)>) -> Json {
    Json::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
}
fn json_str(value: &str) -> Json {
    Json::String(value.to_string())
}
fn json_spec(kind: &str, params: Json) -> Json {
    json_obj(vec![("kind", json_str(kind)), ("params", params)])
}
//#endregion 🔖️JsonBuild

//#region 🔖️Profile
/// 📏️ `semantic-obj-document-v1`'s own declared tolerances (`../../🔮️oracle/🔣️.json`), mirrored here so an in-handler law check is exactly as strict as
/// the profile the case is measured by — never stricter, which would invent a failure the
/// comparison itself would forgive.
const OBJ_WRITER_FREEDOM: &[&str] = &["byteLength", "fileSize", "precision"];
const OBJ_TOLERANCE: f64 = 1e-5;
//#endregion 🔖️Profile

//#region 🔖️Projection
/// 🔍️ The projection both roles are compared through: `tobj`'s triangle mesh (which cannot see
/// `mtllib`/`usemtl` at all — it triangulates and re-indexes, dropping material references), plus
/// the document surface that reader cannot see. Both come from readers independent of `decode_obj`.
fn project(bytes: &[u8]) -> Result<Json, String> {
    let mut projection = project_obj(bytes)?;
    match &mut projection {
        Json::Object(members) => members.push(("document".to_string(), oracle_document_projection(bytes)?)),
        other => return Err(format!("the mesh reader returned {other:?} rather than an object")),
    }
    Ok(projection)
}

/// 👁️ Both declared kinds only ever move the document half of the composed projection —
/// `set-mtllib`/`set-usemtl` touch no vertex/face geometry — which is exactly what the document
/// half was added to make observable at all.
fn moved_the_document(kind: &str, mutated: &Json, base: &Json) -> Result<(), String> {
    mutation_is_observable_within(kind, mutated, base, &[], OBJ_WRITER_FREEDOM, OBJ_TOLERANCE)
}
//#endregion 🔖️Projection

//#region 🔖️Inverse
/// ↩️ The semantically correct inverse for one forward `(kind, params)` pair against the pristine
/// fixture's own real value — mirroring `ObjMutation::inverse()`'s `SetMtllib`/`SetUsemtl` semantics
/// (`../../../✳️any/🧬️schema/🧬️mutations/🦀️.rs`), computed independently
/// here since neither the oracle nor this adapter can reach that subject-side method. Both kinds
/// invert to a single re-`set-*` of the pristine fixture's own real value: `set-mtllib` back to the
/// real `pattern-sphere.mtl` it already declares, `set-usemtl` back to the real single default-band
/// usemtl range this ticket's own fixtures (`../../🧫️fixtures/🖌️set-usemtl/⬅️before.obj`) carry.
fn inverse_specs(spec: &Json) -> Result<Vec<Json>, String> {
    let kind = spec.str("kind");
    Ok(match kind.as_str() {
        "set-mtllib" => vec![json_spec("set-mtllib", json_obj(vec![("mtllib", json_str("pattern-sphere.mtl"))]))],
        "set-usemtl" => vec![json_spec("set-usemtl", json_obj(vec![("usemtl", Json::Array(vec![json_obj(vec![("faceIndexFrom", Json::Number(0.0)), ("material", json_str("pattern"))])]))]))],
        other => return Err(format!("mutate-obj-3-0-material: no inverse registered for kind {other:?} — this case owns only set-mtllib/set-usemtl")),
    })
}
//#endregion 🔖️Inverse

//#region 🔖️Oracle
/// 🦠️ The forward half, with observability asserted in role: the reference applies the kind to the
/// real mesh and the result has to differ from the untouched document under the very profile the
/// case is measured by.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project(&bytes)?;
    moved_the_document(&spec.str("kind"), &projection, &project(&input)?)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ The inverse law, asserted HERE rather than deferred to the parity phase: each kind is applied
/// forward and then undone, and the restored mesh's composed projection must equal the REAL
/// original's own. `semantic-obj-document-v1`'s own tolerance (1e-5, byte length and decimal
/// precision the only writer freedom) is what the comparison uses, never a stricter one.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let mut restored = oracle_apply_mutation(&input, &spec)?;
    for undo in inverse_specs(&spec)? {
        restored = oracle_apply_mutation(&restored, &undo)?;
    }
    let projection = project(&restored)?;
    inverse_restores_within(&kind, &projection, &project(&input)?, OBJ_WRITER_FREEDOM, OBJ_TOLERANCE)?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔁️ The identity law, both halves asserted in role: parsing the real fixture and re-rendering the
/// whole OBJ grammar from the parsed model alone must preserve the mesh projection, and must NOT
/// hand back the input bytes.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let bytes = oracle_apply_mutation(&input, &json_spec("no-mutation", json_obj(vec![])))?;
    reparsed_not_copied(&bytes, &input)?;
    let projection = project(&bytes)?;
    round_trip_preserves_within(&projection, &project(&input)?, OBJ_WRITER_FREEDOM, OBJ_TOLERANCE)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{inverse_specs, moved_the_document, mutable_input, project};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::obj::standards::v3_0::subsets::any::io::{decode_obj, encode_obj};
    use semio_s_plugin_stdio::artifacts::obj::standards::v3_0::subsets::any::schema::mutations::{apply_obj_mutation, set_mtllib, set_usemtl, ObjMutation};
    use semio_s_plugin_stdio::artifacts::obj::standards::v3_0::subsets::any::schema::snapshot::ObjUsemtlRange;

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

    //#region 🔖️MutationFromSpec
    /// 🦠️ The same `(kind, params)` wire shape the oracle dispatcher reads, translated into a real
    /// `ObjMutation` value for this subset's own `apply_obj_mutation` — only the 2 kinds this case
    /// owns; every other kind is a hard error rather than silently falling through to a vocabulary
    /// this subset does not claim.
    fn mutation_from_spec(spec: &Json) -> Result<ObjMutation, String> {
        let kind = spec.str("kind");
        let empty = Json::Object(Vec::new());
        let params = spec.get("params").unwrap_or(&empty);
        Ok(match kind.as_str() {
            "set-mtllib" => ObjMutation::SetMtllib(set_mtllib::SetMtllib { mtllib: json_str(params, "mtllib") }),
            "set-usemtl" => ObjMutation::SetUsemtl(set_usemtl::SetUsemtl {
                usemtl: params.array("usemtl").iter().map(|entry| Ok(ObjUsemtlRange { face_index_from: json_usize(entry, "faceIndexFrom")?, material: json_str(entry, "material").ok_or("usemtl.material")? })).collect::<Result<Vec<_>, String>>()?,
            }),
            other => return Err(format!("mutate-obj-3-0-material: unrecognised mutation kind {other:?} — this case owns only set-mtllib/set-usemtl")),
        })
    }
    //#endregion 🔖️MutationFromSpec

    //#region 🔖️Codec
    /// 📐️ Full parse → typed mutation → re-serialize from the model alone.
    fn mutate_and_encode(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
        let text = std::str::from_utf8(input).map_err(|error| format!("input is not UTF-8: {error}"))?;
        let mut snapshot = decode_obj(text).map_err(|error| format!("decode_obj failed: {error}"))?;
        let mutation = mutation_from_spec(spec)?;
        apply_obj_mutation(&mut snapshot, &mutation);
        Ok(encode_obj(&snapshot).into_bytes())
    }

    /// 📐️ [`mutate_and_encode`] plus the no-byte-pass-through rule this wave exists to enforce: our
    /// encoder cannot reproduce another writer's statement layout, so bit-identical output means the
    /// input was smuggled rather than parsed.
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
        let projection = project(&bytes)?;
        moved_the_document(&spec.str("kind"), &projection, &project(&input)?)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// ↩️ Each kind is genuinely applied forward and then undone through this repository's own
    /// `ObjMutation` pipeline.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let spec = ctx.doc_json()?;
        let mut restored = apply_and_encode(&input, &spec)?;
        for undo in inverse_specs(&spec)? {
            restored = mutate_and_encode(&restored, &undo)?;
        }
        let projection = project(&restored)?;
        Ok(Outcome::with_raw(restored, projection))
    }

    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let text = std::str::from_utf8(&input).map_err(|error| format!("input is not UTF-8: {error}"))?;
        let snapshot = decode_obj(text).map_err(|error| format!("decode_obj failed: {error}"))?;
        let bytes = encode_obj(&snapshot).into_bytes();
        if bytes == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project(&bytes)?;
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
