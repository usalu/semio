//! 🦀️ glTF 2.0 `💿️buffer` subset mutation case — Rust adapter for the 8 kinds of the
//! `gltf-2-0-buffer` catalog (`../../🔮️oracles/🔣️.json`), which own document/buffers and document/bufferViews. Every leaf stays
//! physically owned by `♾️any`'s aggregate mutation root and is reached by import.
//!
//! The oracle performs each kind by independent GLB/JSON-tree manipulation
//! (`../../../♾️any/🔮️oracles/🦀️.rs`); the subject fully parses the committed `⬅️before.gltf` into
//! `GltfSnapshot`, dispatches through the leaf's own typed `apply()`, and re-serializes from the
//! model alone. The feature's spec names the input fixture and the inverse: either the kinds that
//! undo the mutation, applied the same way on both sides, or — for a kind whose payload cannot
//! carry what it removed — the top-level members restored from the original document.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::gltf::standards::v2_0::subsets::any::{oracle_apply_mutation, project_gltf, restore_members};
use semio_s_plugin_stdio_test_oracle::law::{inverse_restores_within, mutation_is_observable_within};


//#region 🔖️Spec
/// 📏️ The `semantic-gltf-v1` writer freedom every glTF mutation case is measured under.
const GLTF_WRITER_FREEDOM: &[&str] = &["byteLength", "fileSize", "generator", "copyright"];

/// 🧫️ A work-directory copy of the spec's own `fixture`; the committed fixture is never written to.
fn mutable_input(ctx: &Context, spec: &Json) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(&spec.str("fixture"), Some("input.gltf"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}

/// ↩️ The spec's inverse kinds, in application order.
fn inverse_specs(spec: &Json) -> Vec<Json> {
    match spec.get("inverse") {
        Some(Json::Array(items)) => items.clone(),
        _ => Vec::new(),
    }
}

/// ↩️ The top-level members the inverse restores from the original document, when it does.
fn restored_members(spec: &Json) -> Option<Vec<String>> {
    match spec.get("restore") {
        Some(Json::Array(items)) => Some(
            items
                .iter()
                .filter_map(|item| match item {
                    Json::String(member) => Some(member.clone()),
                    _ => None,
                })
                .collect(),
        ),
        _ => None,
    }
}
//#endregion 🔖️Spec

//#region 🔖️Oracle
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let input = mutable_input(ctx, &spec)?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_gltf(&bytes)?;
    mutation_is_observable_within(&spec.str("kind"), &projection, &project_gltf(&input)?, &[], GLTF_WRITER_FREEDOM, 0.0)?;
    Ok(Outcome::with_raw(bytes, projection))
}

fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let input = mutable_input(ctx, &spec)?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let restored = match restored_members(&spec) {
        Some(members) => restore_members(&mutated, &input, &members)?,
        None => inverse_specs(&spec).iter().try_fold(mutated, |current, inverse| oracle_apply_mutation(&current, inverse))?,
    };
    let projection = project_gltf(&restored)?;
    inverse_restores_within(&spec.str("kind"), &projection, &project_gltf(&input)?, GLTF_WRITER_FREEDOM, 0.0)?;
    Ok(Outcome::with_raw(restored, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{inverse_specs, mutable_input, restored_members};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_gltf::standards::v2_0::subsets::any::io::{parse_gltf_document, serialize_gltf_document};
    use semio_s_artifact_stdio_gltf::standards::v2_0::subsets::any::schema::mutations::{create_buffer, create_buffer_view, delete_buffer, delete_buffer_view, move_buffer, move_buffer_view, reorder_buffer_views, reorder_buffers};
    use semio_s_artifact_stdio_gltf::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
    use semio_s_plugin_stdio_test_oracle::artifacts::gltf::standards::v2_0::subsets::any::project_gltf;

    //#region 🔖️Params
    /// 🔢️ A non-negative integer payload member.
    fn num(params: &Json, key: &str) -> Result<usize, String> {
        match params.get(key) {
            Some(Json::Number(value)) if *value >= 0.0 && value.fract() == 0.0 => Ok(*value as usize),
            _ => Err(format!("missing or non-integer `{key}`")),
        }
    }
    /// 🔢️ An index list payload member.
    fn order(params: &Json, key: &str) -> Result<Vec<usize>, String> {
        match params.get(key) {
            Some(Json::Array(items)) => items
                .iter()
                .map(|item| match item {
                    Json::Number(value) if *value >= 0.0 && value.fract() == 0.0 => Ok(*value as usize),
                    _ => Err(format!("`{key}` must hold only non-negative integers")),
                })
                .collect(),
            _ => Err(format!("missing or non-array `{key}`")),
        }
    }
    /// 💾️ A byte list payload member.
    fn bytes(params: &Json, key: &str) -> Result<Vec<u8>, String> {
        order(params, key)?.into_iter().map(|byte| u8::try_from(byte).map_err(|_| format!("`{key}` holds a value above 255"))).collect()
    }
    //#endregion 🔖️Params

    //#region 🔖️Dispatch
    /// 📐️ One kind through its leaf's own typed `apply()`.
    fn apply_kind(before: &GltfSnapshot, kind: &str, params: &Json) -> Result<GltfSnapshot, String> {
        match kind {
            "create-buffer" => create_buffer::apply(&create_buffer::GltfCreateBufferPayload { position: num(params, "position")?, bytes: bytes(params, "bytes")? }, before).map_err(|error| error.detail),
            "create-buffer-view" => create_buffer_view::apply(&create_buffer_view::GltfCreateBufferViewPayload { position: num(params, "position")?, buffer: num(params, "buffer")?, byte_offset: num(params, "byteOffset")?, byte_length: num(params, "byteLength")? }, before).map_err(|error| error.detail),
            "delete-buffer" => delete_buffer::apply(&delete_buffer::GltfDeleteBufferPayload { index: num(params, "index")? }, before).map_err(|error| error.detail),
            "delete-buffer-view" => delete_buffer_view::apply(&delete_buffer_view::GltfDeleteBufferViewPayload { index: num(params, "index")? }, before).map_err(|error| error.detail),
            "move-buffer" => move_buffer::apply(&move_buffer::GltfMoveBufferPayload { index: num(params, "index")?, position: num(params, "position")? }, before).map_err(|error| error.detail),
            "move-buffer-view" => move_buffer_view::apply(&move_buffer_view::GltfMoveBufferViewPayload { index: num(params, "index")?, position: num(params, "position")? }, before).map_err(|error| error.detail),
            "reorder-buffer-views" => reorder_buffer_views::apply(&reorder_buffer_views::GltfReorderBufferViewsPayload { order: order(params, "order")? }, before).map_err(|error| error.detail),
            "reorder-buffers" => reorder_buffers::apply(&reorder_buffers::GltfReorderBuffersPayload { order: order(params, "order")? }, before).map_err(|error| error.detail),
            other => Err(format!("unrecognised mutation kind {other:?}")),
        }
    }

    /// ↩️ The named top-level members copied back from `before`.
    fn restore(before: &GltfSnapshot, mutated: &GltfSnapshot, members: &[String]) -> Result<GltfSnapshot, String> {
        let mut restored = mutated.clone();
        for member in members {
            match member.as_str() {
                "scene" => restored.document.scene = before.document.scene,
                "scenes" => restored.document.scenes = before.document.scenes.clone(),
                "nodes" => restored.document.nodes = before.document.nodes.clone(),
                "skins" => restored.document.skins = before.document.skins.clone(),
                "animations" => restored.document.animations = before.document.animations.clone(),
                "meshes" => restored.document.meshes = before.document.meshes.clone(),
                "accessors" => restored.document.accessors = before.document.accessors.clone(),
                "images" => restored.document.images = before.document.images.clone(),
                "bufferViews" => restored.document.buffer_views = before.document.buffer_views.clone(),
                "buffers" => {
                    restored.document.buffers = before.document.buffers.clone();
                    restored.buffers = before.buffers.clone();
                }
                other => return Err(format!("no restore rule for member {other:?}")),
            }
        }
        Ok(restored)
    }
    //#endregion 🔖️Dispatch

    //#region 🔖️Handlers
    fn params(spec: &Json) -> Json {
        spec.get("params").cloned().unwrap_or(Json::Object(Vec::new()))
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let input = mutable_input(ctx, &spec)?;
        let before = parse_gltf_document(&input)?;
        let after = apply_kind(&before, &spec.str("kind"), &params(&spec))?;
        let bytes = serialize_gltf_document(&after);
        if bytes == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_gltf(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let input = mutable_input(ctx, &spec)?;
        let before = parse_gltf_document(&input)?;
        let mutated = apply_kind(&before, &spec.str("kind"), &params(&spec))?;
        let restored = match restored_members(&spec) {
            Some(members) => restore(&before, &mutated, &members)?,
            None => inverse_specs(&spec).iter().try_fold(mutated, |current, inverse| apply_kind(&current, &inverse.str("kind"), &params(inverse)))?,
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
    built = built.oracle("mutate", mutate_oracle).oracle("inverse", inverse_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("mutate", subject::mutate).subject("inverse", subject::inverse);
    }
    built
}
//#endregion 🔖️Registration
