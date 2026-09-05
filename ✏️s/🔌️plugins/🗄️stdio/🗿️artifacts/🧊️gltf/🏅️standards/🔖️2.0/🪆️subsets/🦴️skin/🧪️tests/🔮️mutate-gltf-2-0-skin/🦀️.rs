//! 🦀️ glTF 2.0 `🦴️skin` subset mutation case — Rust adapter. Covers the 4 kinds `../../🔮️oracle/
//! 🔣️.json`'s `gltf-2-0-skin` catalog declares: `create-skin`, `delete-skin`, `move-skin`,
//! `reorder-skins`. Every leaf's own `apply()` (`../../../♾️any/🧬️schema/🧬️mutations/
//! 🦴️skin/{🌱️create,🗑️delete,🚚️move,🔀️reorder}/🦀️.rs`) stays physically owned by `♾️any` —
//! `validate_mutation_leaf_source` requires the exact registered domain/operation owner beneath its
//! aggregate mutation root, so this case reaches it by import. The
//! oracle performs every kind by independent GLB/JSON-tree manipulation
//! (`../../../♾️any/🔮️oracle/🦀️.rs`, extended with these 4 kinds by this same change); the subject
//! fully parses each kind's own committed fixture into `GltfSnapshot` via `parse_gltf_document` and
//! re-serializes with `serialize_gltf_document` alone, dispatching through each leaf's own typed
//! `apply()` function directly. `delete-skin`'s inverse is special-cased on both sides (see the
//! feature file's own doc comment and `undo_delete_skin`) rather than routed through a second
//! `create-skin` call, since `create-skin`'s payload carries no field content to restore.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::gltf::standards::v2_0::subsets::any::{oracle_apply_mutation, project_gltf, undo_delete_skin};
use semio_s_plugin_stdio_test_oracle::law::{inverse_restores_within, mutation_is_observable_within};

//#region 🔖️Kinds
const KINDS: &[&str] = &["create-skin", "delete-skin", "move-skin", "reorder-skins"];
//#endregion 🔖️Kinds

//#region 🔖️Input
/// 🧫️ Each kind owns its own committed `⬅️before.gltf` (the exact fixture coordinates below, shared
/// against this case's own owner — `shared://` resolves there since `🧪️tests` sits directly under
/// `🦴️skin`). Copies into the work directory; the committed fixture itself is never written to.
fn mutable_input(ctx: &Context, kind: &str) -> Result<Vec<u8>, String> {
    let uri = match kind {
        "create-skin" => "shared://🌱️create-skin-applied/⬅️before.gltf",
        "delete-skin" => "shared://🗑️delete-skin-applied/⬅️before.gltf",
        "move-skin" => "shared://🚚️move-skin-applied/⬅️before.gltf",
        "reorder-skins" => "shared://🔀️reorder-skins-applied/⬅️before.gltf",
        other => return Err(format!("unrecognised fixture kind {other:?}")),
    };
    let copy = ctx.copy_fixture(uri, Some("input.gltf"))?;
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
fn json_arr(values: Vec<f64>) -> Json {
    Json::Array(values.into_iter().map(Json::Number).collect())
}
fn json_spec(kind: &str, params: Json) -> Json {
    json_obj(vec![("kind", Json::String(kind.to_string())), ("params", params)])
}
//#endregion 🔖️JsonBuild

//#region 🔖️Profile
/// 📏️ Mirrors `../../../♾️any/🧪️tests/🐞️mutate-gltf-2-0/🦀️.rs`'s own `GLTF_WRITER_FREEDOM` — the
/// SAME `semantic-gltf-v1` profile this case is measured under.
const GLTF_WRITER_FREEDOM: &[&str] = &["byteLength", "fileSize", "generator", "copyright"];
//#endregion 🔖️Profile

//#region 🔖️Inverse
/// ↩️ The semantically correct inverse spec for `create-skin`/`move-skin`/`reorder-skins` against
/// each kind's own committed fixture (the `mutable_input` coordinates above) — computed
/// independently since the oracle role must not link the subject crate. `delete-skin` has no entry
/// here: its inverse is special-cased in both `mutate_oracle`/`inverse_oracle` below and the subject
/// module, via `undo_delete_skin` — see the feature file's own doc comment for why.
fn inverse_spec(kind: &str) -> Json {
    match kind {
        "create-skin" => json_spec("delete-skin", json_obj(vec![("index", json_num(1.0))])),
        "move-skin" => json_spec("move-skin", json_obj(vec![("index", json_num(0.0)), ("position", json_num(1.0))])),
        "reorder-skins" => json_spec("reorder-skins", json_obj(vec![("order", json_arr(vec![1.0, 0.0]))])),
        other => json_spec(other, json_obj(vec![])),
    }
}
//#endregion 🔖️Inverse

//#region 🔖️Oracle
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let input = mutable_input(ctx, &kind)?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_gltf(&bytes)?;
    mutation_is_observable_within(&kind, &projection, &project_gltf(&input)?, &[], GLTF_WRITER_FREEDOM, 0.0)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ The inverse law, asserted HERE rather than deferred to the parity phase — see
/// `../../../♾️any/🧪️tests/🐞️mutate-gltf-2-0/🦀️.rs`'s identical structure for the artifact-root case.
/// `delete-skin` is special-cased through `undo_delete_skin` (the original document's own real
/// content, not a same-shaped substitute) rather than the generic `inverse_spec` dispatch.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let input = mutable_input(ctx, &kind)?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let restored = if kind == "delete-skin" { undo_delete_skin(&mutated, &input)? } else { oracle_apply_mutation(&mutated, &inverse_spec(&kind))? };
    let projection = project_gltf(&restored)?;
    inverse_restores_within(&kind, &projection, &project_gltf(&input)?, GLTF_WRITER_FREEDOM, 0.0)?;
    Ok(Outcome::with_raw(restored, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::mutable_input;
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::gltf::standards::v2_0::subsets::any::io::{parse_gltf_document, serialize_gltf_document};
    use semio_s_plugin_stdio::artifacts::gltf::standards::v2_0::subsets::any::schema::mutations::{create_skin, delete_skin, move_skin, reorder_skins};
    use semio_s_plugin_stdio::artifacts::gltf::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
    use semio_s_plugin_stdio_test_oracle::artifacts::gltf::standards::v2_0::subsets::any::project_gltf;

    //#region 🔖️Params
    fn num(params: &Json, key: &str) -> Result<usize, String> {
        match params.get(key) {
            Some(Json::Number(value)) => Ok(*value as usize),
            _ => Err(format!("missing or non-numeric `{key}`")),
        }
    }
    fn order(params: &Json, key: &str) -> Result<Vec<usize>, String> {
        match params.get(key) {
            Some(Json::Array(items)) => items
                .iter()
                .map(|item| match item {
                    Json::Number(value) => Ok(*value as usize),
                    _ => Err(format!("`{key}` must hold only numbers")),
                })
                .collect(),
            _ => Err(format!("missing or non-array `{key}`")),
        }
    }
    //#endregion 🔖️Params

    //#region 🔖️Dispatch
    /// 📐️ Full parse → typed leaf `apply()` → re-serialize from the model alone — the
    /// no-byte-pass-through rule this wave exists to enforce. Dispatches through each of the 4
    /// leaves' own real `apply()` directly, same shape `🎥️camera`'s adapter already established.
    fn apply_kind(before: &GltfSnapshot, kind: &str, params: &Json) -> Result<GltfSnapshot, String> {
        match kind {
            "create-skin" => create_skin::apply(&create_skin::GltfCreateSkinPayload { position: num(params, "position")? }, before).map_err(|error| error.detail),
            "delete-skin" => delete_skin::apply(&delete_skin::GltfDeleteSkinPayload { index: num(params, "index")? }, before).map_err(|error| error.detail),
            "move-skin" => move_skin::apply(&move_skin::GltfMoveSkinPayload { index: num(params, "index")?, position: num(params, "position")? }, before).map_err(|error| error.detail),
            "reorder-skins" => reorder_skins::apply(&reorder_skins::GltfReorderSkinsPayload { order: order(params, "order")? }, before).map_err(|error| error.detail),
            other => Err(format!("unrecognised mutation kind {other:?}")),
        }
    }

    /// ↩️ `delete-skin`'s own inverse, restoring `document/skins` and every `nodes[].skin`
    /// reference DIRECTLY from `before` — the exact typed values this snapshot already holds, not a
    /// same-shaped substitute a second `create-skin` call could only approximate (its own payload
    /// carries no field content — see the feature file's own doc comment). Mirrors
    /// `../../../♾️any/🔮️oracle/🦀️.rs`'s `undo_delete_skin` on the independent-reader side.
    fn undo_delete_skin(before: &GltfSnapshot, mutated: &GltfSnapshot) -> GltfSnapshot {
        let mut restored = mutated.clone();
        restored.document.skins = before.document.skins.clone();
        for (index, node) in restored.document.nodes.iter_mut().enumerate() {
            node.skin = before.document.nodes[index].skin;
        }
        restored
    }
    //#endregion 🔖️Dispatch

    //#region 🔖️Handlers
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let kind = spec.str("kind");
        let input = mutable_input(ctx, &kind)?;
        let before = parse_gltf_document(&input)?;
        let empty = Json::Object(Vec::new());
        let params = spec.get("params").unwrap_or(&empty);
        let after = apply_kind(&before, &kind, params)?;
        let bytes = serialize_gltf_document(&after);
        if bytes == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_gltf(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let kind = spec.str("kind");
        let input = mutable_input(ctx, &kind)?;
        let before = parse_gltf_document(&input)?;
        let empty = Json::Object(Vec::new());
        let params = spec.get("params").unwrap_or(&empty);
        let mutated = apply_kind(&before, &kind, params)?;
        let restored = if kind == "delete-skin" {
            undo_delete_skin(&before, &mutated)
        } else {
            let inverse = super::inverse_spec(&kind);
            let inverse_kind = inverse.str("kind");
            let inverse_empty = Json::Object(Vec::new());
            let inverse_params = inverse.get("params").unwrap_or(&inverse_empty);
            apply_kind(&mutated, &inverse_kind, inverse_params)?
        };
        let bytes = serialize_gltf_document(&restored);
        let projection = project_gltf(&bytes)?;
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
    built
}
//#endregion 🔖️Registration
