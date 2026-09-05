//! 🦀️ glTF 2.0 `💎️material` subset mutation case — Rust adapter. Covers the 18 kinds
//! `../../🔮️oracle/🔣️.json`'s `gltf-2-0-material` catalog declares: `create`/`delete`/`move`/
//! `reorder` for each of the 4 families `materials`/`textures`/`images`/`samplers`, plus
//! `change-material-alpha-mode`/`change-material-double-sided` (already the artifact-root case's
//! own 2 kinds, oracle functions reused unmodified). Every leaf's own `apply()` stays physically
//! owned by `♾️any` — `validate_mutation_leaf_source` requires a leaf's `owner` to be an immediate
//! child of its aggregate mutation root, so this case reaches it by import, never by moving the
//! directory. The oracle performs every kind by independent GLB/JSON-tree manipulation
//! (`../../../♾️any/🔮️oracle/🦀️.rs`, extended with these 16 new kinds by this same change); the
//! subject fully parses each kind's own committed fixture into `GltfSnapshot` via
//! `parse_gltf_document` and re-serializes with `serialize_gltf_document` alone, dispatching through
//! each leaf's own typed `apply()` function directly. Every `delete-*`'s inverse is special-cased on
//! both sides through a bespoke `undo_delete_*` (see the feature file's own doc comment) rather than
//! routed through a second `create-*` call, since every `create-*` payload in this subset carries no
//! field content to restore — the identical `delete-skin`/`delete-animation` shape.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::gltf::standards::v2_0::subsets::any::{
    oracle_apply_mutation, project_gltf, undo_delete_image, undo_delete_material, undo_delete_sampler, undo_delete_texture,
};
use semio_s_plugin_stdio_test_oracle::law::{inverse_restores_within, mutation_is_observable_within};

//#region 🔖️Kinds
const KINDS: &[&str] = &[
    "create-material",
    "delete-material",
    "move-material",
    "reorder-materials",
    "create-texture",
    "delete-texture",
    "move-texture",
    "reorder-textures",
    "create-image",
    "delete-image",
    "move-image",
    "reorder-images",
    "create-sampler",
    "delete-sampler",
    "move-sampler",
    "reorder-samplers",
    "change-material-alpha-mode",
    "change-material-double-sided",
];
const DELETE_KINDS: &[&str] = &["delete-material", "delete-texture", "delete-image", "delete-sampler"];
//#endregion 🔖️Kinds

//#region 🔖️Input
/// 🧫️ Each kind owns its own committed `before.gltf` (`../../🧫️fixtures/<kind>-applied/`, shared
/// against this case's own owner — `shared://` resolves there since `🧪️tests` sits directly under
/// `💎️material`). Copies into the work directory; the committed fixture itself is never written to.
fn mutable_input(ctx: &Context, kind: &str) -> Result<Vec<u8>, String> {
    let uri = format!("shared://{kind}-applied/before.gltf");
    let copy = ctx.copy_fixture(&uri, Some("input.gltf"))?;
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
fn json_bool(value: bool) -> Json {
    Json::Bool(value)
}
fn json_str(value: &str) -> Json {
    Json::String(value.to_string())
}
fn json_arr(values: Vec<f64>) -> Json {
    Json::Array(values.into_iter().map(Json::Number).collect())
}
fn json_spec(kind: &str, params: Json) -> Json {
    json_obj(vec![("kind", Json::String(kind.to_string())), ("params", params)])
}
//#endregion 🔖️JsonBuild

//#region 🔖️Profile
/// 📏️ Mirrors `../../../♾️any/🧪️tests/🧊️mutate-gltf-2-0/🦀️.rs`'s own `GLTF_WRITER_FREEDOM` — the
/// SAME `semantic-gltf-v1` profile this case is measured under.
const GLTF_WRITER_FREEDOM: &[&str] = &["byteLength", "fileSize", "generator", "copyright"];
//#endregion 🔖️Profile

//#region 🔖️Inverse
/// ↩️ The semantically correct inverse spec for every kind but `delete-*` against each kind's own
/// committed fixture (`../../🧫️fixtures/<kind>-applied/before.gltf`), computed independently here
/// since the oracle role must not link the subject crate. Every `delete-*` kind has no entry here:
/// its inverse is special-cased in both `mutate_oracle`/`inverse_oracle` below and the subject
/// module, via `undo_delete_{material,texture,image,sampler}` — see the feature file's own doc
/// comment for why.
fn inverse_spec(kind: &str) -> Json {
    match kind {
        "create-material" => json_spec("delete-material", json_obj(vec![("index", json_num(1.0))])),
        "move-material" => json_spec("move-material", json_obj(vec![("index", json_num(1.0)), ("position", json_num(0.0))])),
        "reorder-materials" => json_spec("reorder-materials", json_obj(vec![("order", json_arr(vec![1.0, 0.0]))])),
        "create-texture" => json_spec("delete-texture", json_obj(vec![("index", json_num(1.0))])),
        "move-texture" => json_spec("move-texture", json_obj(vec![("index", json_num(0.0)), ("position", json_num(2.0))])),
        "reorder-textures" => json_spec("reorder-textures", json_obj(vec![("order", json_arr(vec![2.0, 1.0, 0.0]))])),
        "create-image" => json_spec("delete-image", json_obj(vec![("index", json_num(1.0))])),
        "move-image" => json_spec("move-image", json_obj(vec![("index", json_num(0.0)), ("position", json_num(2.0))])),
        "reorder-images" => json_spec("reorder-images", json_obj(vec![("order", json_arr(vec![2.0, 1.0, 0.0]))])),
        "create-sampler" => json_spec("delete-sampler", json_obj(vec![("index", json_num(1.0))])),
        "move-sampler" => json_spec("move-sampler", json_obj(vec![("index", json_num(0.0)), ("position", json_num(2.0))])),
        "reorder-samplers" => json_spec("reorder-samplers", json_obj(vec![("order", json_arr(vec![2.0, 1.0, 0.0]))])),
        "change-material-alpha-mode" => json_spec("change-material-alpha-mode", json_obj(vec![("material", json_num(1.0)), ("alphaMode", json_str("OPAQUE"))])),
        "change-material-double-sided" => json_spec("change-material-double-sided", json_obj(vec![("material", json_num(1.0)), ("doubleSided", json_bool(false))])),
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
/// `../../../♾️any/🧪️tests/🧊️mutate-gltf-2-0/🦀️.rs`'s identical structure for the artifact-root case.
/// Every `delete-*` kind is special-cased through its own `undo_delete_*` (the original document's
/// own real content, not a same-shaped substitute) rather than the generic `inverse_spec` dispatch.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let input = mutable_input(ctx, &kind)?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let restored = match kind.as_str() {
        "delete-material" => undo_delete_material(&mutated, &input)?,
        "delete-texture" => undo_delete_texture(&mutated, &input)?,
        "delete-image" => undo_delete_image(&mutated, &input)?,
        "delete-sampler" => undo_delete_sampler(&mutated, &input)?,
        _ => oracle_apply_mutation(&mutated, &inverse_spec(&kind))?,
    };
    let projection = project_gltf(&restored)?;
    inverse_restores_within(&kind, &projection, &project_gltf(&input)?, GLTF_WRITER_FREEDOM, 0.0)?;
    Ok(Outcome::with_raw(restored, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{mutable_input, DELETE_KINDS};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::gltf::standards::v2_0::subsets::any::io::{parse_gltf_document, serialize_gltf_document};
    use semio_s_plugin_stdio::artifacts::gltf::standards::v2_0::subsets::any::schema::mutations::{
        change_material_alpha_mode, change_material_double_sided, create_image, create_material, create_sampler, create_texture, delete_image, delete_material, delete_sampler, delete_texture, move_image, move_material,
        move_sampler, move_texture, reorder_images, reorder_materials, reorder_samplers, reorder_textures,
    };
    use semio_s_plugin_stdio::artifacts::gltf::standards::v2_0::subsets::any::schema::snapshot::{GltfAlphaMode, GltfSnapshot};
    use semio_s_plugin_stdio_test_oracle::artifacts::gltf::standards::v2_0::subsets::any::project_gltf;

    //#region 🔖️Params
    fn num(params: &Json, key: &str) -> Result<usize, String> {
        match params.get(key) {
            Some(Json::Number(value)) => Ok(*value as usize),
            _ => Err(format!("missing or non-numeric `{key}`")),
        }
    }
    fn boolean(params: &Json, key: &str) -> Result<bool, String> {
        match params.get(key) {
            Some(Json::Bool(value)) => Ok(*value),
            _ => Err(format!("missing or non-boolean `{key}`")),
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
    /// 🎨️ `alphaMode`'s own three-spelling enum, read directly off the spec's `Json` — this
    /// adapter never needs the generic value machinery for a shape this small, the same choice
    /// `🎥️camera`'s own `projection` param already makes.
    fn alpha_mode(params: &Json, key: &str) -> Result<GltfAlphaMode, String> {
        match params.get(key) {
            Some(Json::String(value)) => match value.as_str() {
                "OPAQUE" => Ok(GltfAlphaMode::Opaque),
                "MASK" => Ok(GltfAlphaMode::Mask),
                "BLEND" => Ok(GltfAlphaMode::Blend),
                other => Err(format!("`{key}` must be OPAQUE, MASK or BLEND, got {other:?}")),
            },
            _ => Err(format!("missing or non-string `{key}`")),
        }
    }
    //#endregion 🔖️Params

    //#region 🔖️Dispatch
    /// 📐️ Full parse → typed leaf `apply()` → re-serialize from the model alone — the
    /// no-byte-pass-through rule this wave exists to enforce. Dispatches through each of the 18
    /// leaves' own real `apply()` directly, same shape `🎥️camera`/`🦴️skin`'s adapters already
    /// established.
    fn apply_kind(before: &GltfSnapshot, kind: &str, params: &Json) -> Result<GltfSnapshot, String> {
        match kind {
            "create-material" => create_material::apply(&create_material::GltfCreateMaterialPayload { position: num(params, "position")? }, before).map_err(|error| error.detail),
            "delete-material" => delete_material::apply(&delete_material::GltfDeleteMaterialPayload { index: num(params, "index")? }, before).map_err(|error| error.detail),
            "move-material" => move_material::apply(&move_material::GltfMoveMaterialPayload { index: num(params, "index")?, position: num(params, "position")? }, before).map_err(|error| error.detail),
            "reorder-materials" => reorder_materials::apply(&reorder_materials::GltfReorderMaterialsPayload { order: order(params, "order")? }, before).map_err(|error| error.detail),
            "create-texture" => create_texture::apply(&create_texture::GltfCreateTexturePayload { position: num(params, "position")? }, before).map_err(|error| error.detail),
            "delete-texture" => delete_texture::apply(&delete_texture::GltfDeleteTexturePayload { index: num(params, "index")? }, before).map_err(|error| error.detail),
            "move-texture" => move_texture::apply(&move_texture::GltfMoveTexturePayload { index: num(params, "index")?, position: num(params, "position")? }, before).map_err(|error| error.detail),
            "reorder-textures" => reorder_textures::apply(&reorder_textures::GltfReorderTexturesPayload { order: order(params, "order")? }, before).map_err(|error| error.detail),
            "create-image" => create_image::apply(&create_image::GltfCreateImagePayload { position: num(params, "position")? }, before).map_err(|error| error.detail),
            "delete-image" => delete_image::apply(&delete_image::GltfDeleteImagePayload { index: num(params, "index")? }, before).map_err(|error| error.detail),
            "move-image" => move_image::apply(&move_image::GltfMoveImagePayload { index: num(params, "index")?, position: num(params, "position")? }, before).map_err(|error| error.detail),
            "reorder-images" => reorder_images::apply(&reorder_images::GltfReorderImagesPayload { order: order(params, "order")? }, before).map_err(|error| error.detail),
            "create-sampler" => create_sampler::apply(&create_sampler::GltfCreateSamplerPayload { position: num(params, "position")? }, before).map_err(|error| error.detail),
            "delete-sampler" => delete_sampler::apply(&delete_sampler::GltfDeleteSamplerPayload { index: num(params, "index")? }, before).map_err(|error| error.detail),
            "move-sampler" => move_sampler::apply(&move_sampler::GltfMoveSamplerPayload { index: num(params, "index")?, position: num(params, "position")? }, before).map_err(|error| error.detail),
            "reorder-samplers" => reorder_samplers::apply(&reorder_samplers::GltfReorderSamplersPayload { order: order(params, "order")? }, before).map_err(|error| error.detail),
            "change-material-alpha-mode" => change_material_alpha_mode::apply(&change_material_alpha_mode::GltfChangeMaterialAlphaModePayload { material: num(params, "material")?, alpha_mode: alpha_mode(params, "alphaMode")? }, before).map_err(|error| error.detail),
            "change-material-double-sided" => change_material_double_sided::apply(&change_material_double_sided::GltfChangeMaterialDoubleSidedPayload { material: num(params, "material")?, double_sided: boolean(params, "doubleSided")? }, before).map_err(|error| error.detail),
            other => Err(format!("unrecognised mutation kind {other:?}")),
        }
    }

    /// ↩️ Every `delete-*` kind's own inverse, restoring the removed collection AND every
    /// reference DIRECTLY from `before` — the exact typed values this snapshot already holds, not a
    /// same-shaped substitute a second `create-*` call could only approximate (its own payload
    /// carries no field content — see the feature file's own doc comment). Mirrors
    /// `../../../♾️any/🔮️oracle/🦀️.rs`'s own `undo_delete_{material,texture,image,sampler}` on the
    /// independent-reader side.
    fn undo_delete(before: &GltfSnapshot, mutated: &GltfSnapshot, kind: &str) -> GltfSnapshot {
        let mut restored = mutated.clone();
        match kind {
            "delete-material" => {
                restored.document.materials = before.document.materials.clone();
                for (mesh_index, mesh) in restored.document.meshes.iter_mut().enumerate() {
                    for (primitive_index, primitive) in mesh.primitives.iter_mut().enumerate() {
                        primitive.material = before.document.meshes[mesh_index].primitives[primitive_index].material;
                    }
                }
            }
            "delete-texture" => {
                restored.document.textures = before.document.textures.clone();
                for (index, material) in restored.document.materials.iter_mut().enumerate() {
                    let source = &before.document.materials[index];
                    if let (Some(pbr), Some(source_pbr)) = (&mut material.pbr_metallic_roughness, &source.pbr_metallic_roughness) {
                        pbr.base_color_texture = source_pbr.base_color_texture.clone();
                        pbr.metallic_roughness_texture = source_pbr.metallic_roughness_texture.clone();
                    } else {
                        material.pbr_metallic_roughness = source.pbr_metallic_roughness.clone();
                    }
                    material.normal_texture = source.normal_texture.clone();
                    material.occlusion_texture = source.occlusion_texture.clone();
                    material.emissive_texture = source.emissive_texture.clone();
                }
            }
            "delete-image" => {
                restored.document.images = before.document.images.clone();
                for (index, texture) in restored.document.textures.iter_mut().enumerate() {
                    texture.source = before.document.textures[index].source;
                }
            }
            "delete-sampler" => {
                restored.document.samplers = before.document.samplers.clone();
                for (index, texture) in restored.document.textures.iter_mut().enumerate() {
                    texture.sampler = before.document.textures[index].sampler;
                }
            }
            other => unreachable!("undo_delete called for a non-delete kind {other:?}"),
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
        let restored = if DELETE_KINDS.contains(&kind.as_str()) {
            undo_delete(&before, &mutated, &kind)
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
