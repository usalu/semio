//! 🦀️ glTF 2.0 `💎️material` subset mutation case — Rust adapter. Covers the 18 kinds
//! `../../🔮️oracles/🔣️.json`'s `gltf-2-0-material` catalog declares: `create`/`delete`/`move`/
//! `reorder` for each of the 4 families `materials`/`textures`/`images`/`samplers`, plus
//! `change-material-alpha-mode`/`change-material-double-sided` (already the artifact-root case's
//! own 2 kinds, oracle functions reused unmodified). Every leaf's own `apply()` stays physically
//! owned by `♾️any` — `validate_mutation_leaf_source` requires a leaf's `owner` to be an immediate
//! child of its aggregate mutation root, so this case reaches it by import, never by moving the
//! directory. The oracle performs every kind by independent GLB/JSON-tree manipulation
//! (`../../../♾️any/🔮️oracles/🦀️.rs`, extended with these 16 new kinds by this same change); the
//! subject fully parses each kind's own committed fixture into `GltfSnapshot` via
//! `parse_gltf_document` and re-serializes with `serialize_gltf_document` alone, dispatching through
//! each leaf's own typed `apply()` function directly. Every `delete-*`'s inverse is special-cased on
//! both sides through a bespoke `undo_delete_*` (see the feature file's own doc comment) rather than
//! routed through a second `create-*` call, since every `create-*` payload in this subset carries no
//! field content to restore — the identical `delete-skin`/`delete-animation` shape.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_artifact_stdio_gltf_test_oracle::standards::v2_0::subsets::any::{
    oracle_apply_mutation, project_gltf, undo_delete_image, undo_delete_material, undo_delete_sampler, undo_delete_texture,
};
use semio_repo_test_host::law::{inverse_restores_within, mutation_is_observable_within};

//#region 🔖️Kinds
const DELETE_KINDS: &[&str] = &["delete-material", "delete-texture", "delete-image", "delete-sampler"];
//#endregion 🔖️Kinds

//#region 🔖️Input
/// 🧫️ Each kind owns its own committed `before.gltf` (`../../🧫️fixtures/<kind>-applied/`, shared
/// against this case's own owner — `shared://` resolves there since `🧪️tests` sits directly under
/// `💎️material`). Copies into the work directory; the committed fixture itself is never written to.
fn mutable_input(ctx: &Context, kind: &str) -> Result<Vec<u8>, String> {
    let uri = format!("shared://{kind}-applied/before.gltf");
    let copy = ctx.copy_input(&uri, Some("input.gltf"))?;
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
    use super::mutable_input;
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_gltf::standards::v2_0::subsets::any::io::{parse_gltf_document, serialize_gltf_document};
    use semio_s_artifact_stdio_gltf::standards::v2_0::subsets::any::io::text::mutations::apply_gltf_mutation;
    use semio_s_artifact_stdio_gltf::standards::v2_0::subsets::any::schema::mutations::{change_material_alpha_mode,change_material_double_sided,create_image,create_material,create_sampler,create_texture,delete_image,delete_material,delete_sampler,delete_texture,move_image,move_material,move_sampler,move_texture,reorder_images,reorder_materials,reorder_samplers,reorder_textures};
use semio_s_artifact_stdio_gltf::standards::v2_0::subsets::any::io::text::mutations::{gltf_inverse_restored_document};
    use semio_s_artifact_stdio_gltf::standards::v2_0::subsets::any::schema::snapshot::{GltfAlphaMode, GltfSnapshot};
    use semio_s_artifact_stdio_gltf_test_oracle::standards::v2_0::subsets::any::project_gltf;

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
    /// 📐️ Full parse → typed leaf mutation through the central applier → re-serialize from the model alone — the
    /// no-byte-pass-through rule this wave exists to enforce. Dispatches through each of the 18
    /// leaves' own mutation and the central applier, same shape `🎥️camera`/`🦴️skin`'s adapters already
    /// established.
    fn apply_kind(before: &GltfSnapshot, kind: &str, params: &Json) -> Result<GltfSnapshot, String> {
        match kind {
            "create-material" => apply_gltf_mutation(before, &create_material::mutation(create_material::GltfCreateMaterialPayload { position: num(params, "position")?, material: None })),
            "delete-material" => apply_gltf_mutation(before, &delete_material::mutation(delete_material::GltfDeleteMaterialPayload { index: num(params, "index")? })),
            "move-material" => apply_gltf_mutation(before, &move_material::mutation(move_material::GltfMoveMaterialPayload { index: num(params, "index")?, position: num(params, "position")? })),
            "reorder-materials" => apply_gltf_mutation(before, &reorder_materials::mutation(reorder_materials::GltfReorderMaterialsPayload { order: order(params, "order")? })),
            "create-texture" => apply_gltf_mutation(before, &create_texture::mutation(create_texture::GltfCreateTexturePayload { position: num(params, "position")?, texture: None })),
            "delete-texture" => apply_gltf_mutation(before, &delete_texture::mutation(delete_texture::GltfDeleteTexturePayload { index: num(params, "index")? })),
            "move-texture" => apply_gltf_mutation(before, &move_texture::mutation(move_texture::GltfMoveTexturePayload { index: num(params, "index")?, position: num(params, "position")? })),
            "reorder-textures" => apply_gltf_mutation(before, &reorder_textures::mutation(reorder_textures::GltfReorderTexturesPayload { order: order(params, "order")? })),
            "create-image" => apply_gltf_mutation(before, &create_image::mutation(create_image::GltfCreateImagePayload { position: num(params, "position")?, image: None })),
            "delete-image" => apply_gltf_mutation(before, &delete_image::mutation(delete_image::GltfDeleteImagePayload { index: num(params, "index")? })),
            "move-image" => apply_gltf_mutation(before, &move_image::mutation(move_image::GltfMoveImagePayload { index: num(params, "index")?, position: num(params, "position")? })),
            "reorder-images" => apply_gltf_mutation(before, &reorder_images::mutation(reorder_images::GltfReorderImagesPayload { order: order(params, "order")? })),
            "create-sampler" => apply_gltf_mutation(before, &create_sampler::mutation(create_sampler::GltfCreateSamplerPayload { position: num(params, "position")?, sampler: None })),
            "delete-sampler" => apply_gltf_mutation(before, &delete_sampler::mutation(delete_sampler::GltfDeleteSamplerPayload { index: num(params, "index")? })),
            "move-sampler" => apply_gltf_mutation(before, &move_sampler::mutation(move_sampler::GltfMoveSamplerPayload { index: num(params, "index")?, position: num(params, "position")? })),
            "reorder-samplers" => apply_gltf_mutation(before, &reorder_samplers::mutation(reorder_samplers::GltfReorderSamplersPayload { order: order(params, "order")? })),
            "change-material-alpha-mode" => apply_gltf_mutation(before, &change_material_alpha_mode::mutation(change_material_alpha_mode::GltfChangeMaterialAlphaModePayload { material: num(params, "material")?, alpha_mode: alpha_mode(params, "alphaMode")? })),
            "change-material-double-sided" => apply_gltf_mutation(before, &change_material_double_sided::mutation(change_material_double_sided::GltfChangeMaterialDoubleSidedPayload { material: num(params, "material")?, double_sided: boolean(params, "doubleSided")? })),
            other => Err(format!("unrecognised mutation kind {other:?}")),
        }
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
        actual(ctx, bytes, projection)
    }

    /// 📦️ The produced document as the `actual-gltf` artifact the `gltf-2-0-three-compare-v1` pipeline reads.
    fn actual(ctx: &Context, bytes: Vec<u8>, projection: Json) -> Result<Outcome, String> {
        let path = ctx.artifact("actual-gltf", "actual.gltf")?;
        std::fs::write(&path, &bytes).map_err(|error| error.to_string())?;
        Ok(Outcome::with_raw(bytes, projection).artifact("actual-gltf", &path, "model/gltf+json"))
    }

    /// ↩️ The production inverse, never a hand-written one: `gltf_inverse_restored_document` applies the row's mutation and
    /// replays that mutation's OWN computed `inverse(base)` through the production codec, and three's GLTFLoader then
    /// judges the restored document against the committed `⬅️before.gltf`.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let kind = spec.str("kind");
        let input = mutable_input(ctx, &kind)?;
        let empty = Json::Object(Vec::new());
        let bytes = gltf_inverse_restored_document(&input, &kind, &spec.get("params").unwrap_or(&empty).to_string())?;
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
    built
}
//#endregion 🔖️Registration
