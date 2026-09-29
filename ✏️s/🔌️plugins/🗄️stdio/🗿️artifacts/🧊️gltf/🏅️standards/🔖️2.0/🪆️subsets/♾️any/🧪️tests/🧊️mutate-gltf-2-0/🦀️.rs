//! 🦀️ glTF 2.0 mutation case — Rust adapter over the real 284 KB, 271-node `base.glb` export. Covers 7 kinds of the
//! `GltfMutation` vocabulary (`../../🧬️schema/🧬️mutations/🦀️.rs`) — `mutate-<kind>`/`inverse-<kind>` each, plus one
//! identity round trip. The judging oracle is the TypeScript reader (`🟦️.ts`, three's GLTFLoader over the committed
//! afters); this file hosts the cross-semio SUPPLEMENT `json-rust-gltf-2-0-mutate` — every kind performed by independent
//! GLB-container and JSON-tree manipulation (`semio_s_plugin_stdio_test_oracle`, `json` 0.12 as the JSON layer only,
//! never this subset's own codec) — and the SUBJECT: it decodes the GLB into `GltfSnapshot`, applies the row's
//! production mutation (and, for an inverse row, that mutation's own computed inverse) through the subset's test
//! bridges, re-encodes with `encode_glb` alone (no byte pass-through) and hands the result to the
//! `gltf-2-0-three-compare-v1` pipeline as its `actual-gltf` artifact.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::gltf::standards::v2_0::subsets::any::{oracle_apply_mutation, project_gltf, round_trip, undo_create_scene};
use semio_s_plugin_stdio_test_oracle::law::{inverse_restores_within, mutation_is_observable_within, reparsed_not_copied, round_trip_preserves_within};


//#region 🔖️Input
const INPUT: &str = "shared://🧊️mutate-gltf-2-0/🌳️base-with-nested-node/🧊️.glb";

/// 🧫️ Copies the derived-once fixture into the work directory and returns its bytes; the committed
/// fixture itself is never written to.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("input.glb"))?;
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
//#endregion 🔖️JsonBuild

//#region 🔖️Profile
/// 📏️ `semantic-gltf-v1`'s own declared writer freedom (`../../🏅️standards/🔖️2.0/🪆️subsets/♾️any/
/// 🔣️oracle.json`), mirrored here so an in-handler law check is exactly as strict as
/// the profile the case is measured by — never stricter.
const GLTF_WRITER_FREEDOM: &[&str] = &["byteLength", "fileSize", "generator", "copyright"];
//#endregion 🔖️Profile

//#region 🔖️Inverse
/// ↩️ The semantically correct inverse spec for one forward `(kind, params)` pair against the
/// derived fixture's own known real state (`../../🏅️standards/🔖️2.0/🪆️subsets/♾️any/📚️examples/
/// 🌱️metabolism/🖼️assets/🏙️base/🧊️.glb` with node 1 moved from the scene's 271-entry root list into
/// node 0's own `children` — see the feature file), computed independently here since the oracle
/// role must not link the subject crate. `create-scene` has no catalog kind of its own to invert
/// through — production dispatches its inverse via the SAME descriptor's `phase: Inverse`, not a
/// separate `delete-scene` leaf — so its caller uses `undo_create_scene` directly instead of this
/// function.
fn inverse_spec(kind: &str) -> Json {
    match kind {
        "bind-node-child" => json_spec("unbind-node-child", json_obj(vec![("parent", json_num(2.0)), ("child", json_num(3.0))])),
        "unbind-node-child" => json_spec("bind-node-child", json_obj(vec![("parent", json_num(0.0)), ("child", json_num(1.0)), ("position", json_num(0.0))])),
        "bind-scene-root-node" => json_spec("unbind-scene-root-node", json_obj(vec![("scene", json_num(0.0)), ("node", json_num(1.0))])),
        "unbind-scene-root-node" => json_spec("bind-scene-root-node", json_obj(vec![("scene", json_num(0.0)), ("node", json_num(5.0)), ("position", json_num(4.0))])),
        "change-material-alpha-mode" => json_spec("change-material-alpha-mode", json_obj(vec![("material", json_num(0.0)), ("alphaMode", json_str("OPAQUE"))])),
        "change-material-double-sided" => json_spec("change-material-double-sided", json_obj(vec![("material", json_num(0.0)), ("doubleSided", Json::Bool(false))])),
        other => json_spec(other, json_obj(vec![])),
    }
}
//#endregion 🔖️Inverse

//#region 🔖️Oracle
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_gltf(&bytes)?;
    mutation_is_observable_within(&spec.str("kind"), &projection, &project_gltf(&input)?, &[], GLTF_WRITER_FREEDOM, 0.0)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ The inverse law, asserted HERE rather than deferred to the parity phase: the reference applies
/// the leaf forward and then its own inverse (for `create-scene`, the SAME descriptor's `Inverse`
/// phase, which is why it routes through `undo_create_scene` instead of a separate kind), and the
/// restored document's independent projection must equal the REAL original's own, within
/// `semantic-gltf-v1`'s own declared writer freedom and no stricter.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let restored = if kind == "create-scene" { undo_create_scene(&mutated, 0)? } else { oracle_apply_mutation(&mutated, &inverse_spec(&kind))? };
    let projection = project_gltf(&restored)?;
    inverse_restores_within(&kind, &projection, &project_gltf(&input)?, GLTF_WRITER_FREEDOM, 0.0)?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔁️ The identity law, both halves asserted in role: reading the GLB container and re-writing it
/// from the parsed JSON document plus the BIN chunk alone must preserve the scene/node/material
/// projection, and must NOT hand back the input bytes — the writer re-serializes the JSON chunk in
/// its own compact form and recomputes every chunk length and pad, so bit-identical output would
/// mean the container was copied rather than parsed.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let bytes = round_trip(&input)?;
    reparsed_not_copied(&bytes, &input)?;
    let projection = project_gltf(&bytes)?;
    round_trip_preserves_within(&projection, &project_gltf(&input)?, GLTF_WRITER_FREEDOM, 0.0)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::mutable_input;
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_gltf::standards::v2_0::subsets::any::io::{decode_glb, encode_glb};
    use semio_s_artifact_stdio_gltf::standards::v2_0::subsets::any::schema::mutations::{gltf_inverse_restored_document, gltf_mutated_document};
    use semio_s_plugin_stdio_test_oracle::artifacts::gltf::standards::v2_0::subsets::any::project_gltf;

    //#region 🔖️Handlers
    /// 📐️ Full GLB parse → the row's production mutation (`GltfMutation` through `Mutation::diff(..).apply_to`) → GLB
    /// re-encoded from the model alone; bit-identical output would be a byte pass-through.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let spec = ctx.doc_json()?;
        let empty = Json::Object(Vec::new());
        let bytes = gltf_mutated_document(&input, &spec.str("kind"), &spec.get("params").unwrap_or(&empty).to_string())?;
        if bytes == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_gltf(&bytes)?;
        actual(ctx, bytes, projection)
    }

    /// 📦️ The produced GLB as the `actual-gltf` artifact the `gltf-2-0-three-compare-v1` pipeline reads.
    fn actual(ctx: &Context, bytes: Vec<u8>, projection: Json) -> Result<Outcome, String> {
        let path = ctx.artifact("actual-gltf", "actual.glb")?;
        std::fs::write(&path, &bytes).map_err(|error| error.to_string())?;
        Ok(Outcome::with_raw(bytes, projection).artifact("actual-gltf", &path, "model/gltf-binary"))
    }

    /// ↩️ The production inverse, never a hand-written one: the row's mutation, then that mutation's OWN computed
    /// `inverse(base)` replayed in order.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let spec = ctx.doc_json()?;
        let empty = Json::Object(Vec::new());
        let bytes = gltf_inverse_restored_document(&input, &spec.str("kind"), &spec.get("params").unwrap_or(&empty).to_string())?;
        let projection = project_gltf(&bytes)?;
        actual(ctx, bytes, projection)
    }

    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode_glb(&input)?;
        let bytes = encode_glb(&snapshot)?;
        if bytes == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_gltf(&bytes)?;
        actual(ctx, bytes, projection)
    }
    //#endregion 🔖️Handlers
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
