//! 🦀️ glTF 2.0 `🎥️camera` subset mutation case — Rust adapter. Covers the 4 kinds
//! `../../🔮️oracle/🔣️.json`'s `gltf-2-0-camera` catalog declares: `create-camera`, `delete-camera`,
//! `move-camera`, `reorder-cameras`. Every leaf's own `apply()` (`../../../♾️any/🧬️schema/
//! 🧬️mutations/🎥️camera/{🌱️create,🗑️delete,🚚️move,🔀️reorder}/🦀️.rs`) stays physically owned by
//! `♾️any` — `validate_mutation_leaf_source` requires the exact registered domain/operation owner
//! beneath its aggregate mutation root, so this case reaches it by import. The oracle performs every kind by independent GLB/JSON-tree
//! manipulation (`../../../♾️any/🔮️oracle/🦀️.rs`, extended with these 4 kinds by this same change,
//! using `json` 0.12 as the JSON layer only, never this subset's own codec); the subject fully parses
//! each kind's own committed fixture into `GltfSnapshot` via `parse_gltf_document` and re-serializes
//! with `serialize_gltf_document` alone, dispatching through each leaf's own typed `apply()` function
//! directly. Both results are read back by the INDEPENDENT `project_gltf` reader before the
//! `semantic-gltf-v1` profile compares them.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::gltf::standards::v2_0::subsets::any::{oracle_apply_mutation, project_gltf};
use semio_s_plugin_stdio_test_oracle::law::{inverse_restores_within, mutation_is_observable_within};

//#region 🔖️Kinds
const KINDS: &[&str] = &["create-camera", "delete-camera", "move-camera", "reorder-cameras"];
//#endregion 🔖️Kinds

//#region 🔖️Input
/// 🧫️ Each kind declares its own committed `⬅️before.gltf` fixture coordinate below, shared
/// against this case's own owner — `shared://` resolves there since `🧪️tests` sits directly under
/// `🎥️camera`). Copies into the work directory; the committed fixture itself is never written to.
fn mutable_input(ctx: &Context, kind: &str) -> Result<Vec<u8>, String> {
    let uri = match kind {
        "create-camera" => "shared://🌱️create-camera-applied/⬅️before.gltf",
        "delete-camera" => "shared://🗑️delete-camera-applied/⬅️before.gltf",
        "move-camera" => "shared://🚚️move-camera-applied/⬅️before.gltf",
        "reorder-cameras" => "shared://🔀️reorder-cameras-applied/⬅️before.gltf",
        other => return Err(format!("unknown camera fixture kind: {other}")),
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
/// 🎥️ The `create-camera` fixture's own inserted camera (`../../🧫️fixtures/🌱️create-camera-applied/
/// ➡️after.gltf`, `cameras[1]`) — a perspective camera with only `yfov`/`znear` set, matching the
/// scenario's own Examples row.
fn perspective_projection(yfov: f64, znear: f64) -> Json {
    json_obj(vec![("type", Json::String("perspective".to_string())), ("perspective", json_obj(vec![("yfov", json_num(yfov)), ("znear", json_num(znear))]))])
}
//#endregion 🔖️JsonBuild

//#region 🔖️Profile
/// 📏️ Mirrors `../../../♾️any/🧪️tests/🧊️mutate-gltf-2-0/🦀️.rs`'s own `GLTF_WRITER_FREEDOM` — the
/// SAME `semantic-gltf-v1` profile this case is measured under.
const GLTF_WRITER_FREEDOM: &[&str] = &["byteLength", "fileSize", "generator", "copyright"];
//#endregion 🔖️Profile

//#region 🔖️Inverse
/// ↩️ The semantically correct inverse spec for one forward `(kind, params)` pair against the
/// kind's own committed `⬅️before.gltf` fixture selected by `mutable_input`, computed
/// independently here since the oracle role must not link the subject crate. Mirrors
/// `../../../♾️any/🔮️oracle/🦀️.rs`'s own `apply_camera_change`/remap arithmetic by construction
/// (each spec below was derived from the committed fixture's own before/after diff, documented in
/// the feature file).
fn inverse_spec(kind: &str) -> Json {
    match kind {
        "create-camera" => json_spec("delete-camera", json_obj(vec![("index", json_num(1.0))])),
        "delete-camera" => json_spec("create-camera", json_obj(vec![("position", json_num(0.0)), ("projection", perspective_projection(0.8726646259971648, 0.1))])),
        "move-camera" => json_spec("move-camera", json_obj(vec![("index", json_num(0.0)), ("position", json_num(1.0))])),
        "reorder-cameras" => json_spec("reorder-cameras", json_obj(vec![("order", json_arr(vec![1.0, 0.0]))])),
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
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let input = mutable_input(ctx, &kind)?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let restored = oracle_apply_mutation(&mutated, &inverse_spec(&kind))?;
    let projection = project_gltf(&restored)?;
    inverse_restores_within(&kind, &projection, &project_gltf(&input)?, GLTF_WRITER_FREEDOM, 0.0)?;
    Ok(Outcome::with_raw(restored, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{inverse_spec, mutable_input};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::gltf::standards::v2_0::subsets::any::io::{parse_gltf_document, serialize_gltf_document};
    use semio_s_plugin_stdio::artifacts::gltf::standards::v2_0::subsets::any::schema::mutations::{create_camera, delete_camera, move_camera, reorder_cameras};
    use semio_s_plugin_stdio::artifacts::gltf::standards::v2_0::subsets::any::schema::snapshot::{GltfCameraProjection, GltfOrthographic, GltfPerspective};
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
    fn number_field(object: &Json, key: &str) -> Option<f64> {
        match object.get(key) {
            Some(Json::Number(value)) => Some(*value),
            _ => None,
        }
    }
    /// 🎥️ `projection`'s own tagged-union shape (`{"type":"perspective"|"orthographic",
    /// "perspective"|"orthographic": {…}}` — see the schema's own hand-rolled
    /// `FromValue`/`Deserialize`), read directly off the spec's `Json` rather than through
    /// `dsl::FromValue` (this adapter never needs the generic value machinery for a shape this
    /// small).
    fn projection(params: &Json, key: &str) -> Result<GltfCameraProjection, String> {
        let value = params.get(key).ok_or_else(|| format!("missing `{key}`"))?;
        match value.str("type").as_str() {
            "perspective" => {
                let inner = value.get("perspective").ok_or("projection.perspective missing")?;
                Ok(GltfCameraProjection::Perspective(GltfPerspective {
                    aspect_ratio: number_field(inner, "aspectRatio"),
                    yfov: number_field(inner, "yfov").ok_or("perspective.yfov missing")?,
                    zfar: number_field(inner, "zfar"),
                    znear: number_field(inner, "znear").ok_or("perspective.znear missing")?,
                    extensions: None,
                    extras: None,
                }))
            }
            "orthographic" => {
                let inner = value.get("orthographic").ok_or("projection.orthographic missing")?;
                Ok(GltfCameraProjection::Orthographic(GltfOrthographic {
                    xmag: number_field(inner, "xmag").ok_or("orthographic.xmag missing")?,
                    ymag: number_field(inner, "ymag").ok_or("orthographic.ymag missing")?,
                    zfar: number_field(inner, "zfar").ok_or("orthographic.zfar missing")?,
                    znear: number_field(inner, "znear").ok_or("orthographic.znear missing")?,
                    extensions: None,
                    extras: None,
                }))
            }
            other => Err(format!("projection.type must be perspective or orthographic, got {other:?}")),
        }
    }
    //#endregion 🔖️Params

    //#region 🔖️Dispatch
    /// 📐️ Full parse → typed leaf `apply()` → re-serialize from the model alone — the
    /// no-byte-pass-through rule this wave exists to enforce. Dispatches through each of the 4
    /// leaves' own real `apply()` directly, the same simple typed-payload shape every camera leaf
    /// exposes (no descriptor-table indirection needed here, unlike the artifact-root case's
    /// 7 older-style kinds).
    fn apply_kind(before: &GltfSnapshot, kind: &str, params: &Json) -> Result<GltfSnapshot, String> {
        match kind {
            "create-camera" => create_camera::apply(&create_camera::GltfCreateCameraPayload { position: num(params, "position")?, projection: projection(params, "projection")? }, before).map_err(|error| error.detail),
            "delete-camera" => delete_camera::apply(&delete_camera::GltfDeleteCameraPayload { index: num(params, "index")? }, before).map_err(|error| error.detail),
            "move-camera" => move_camera::apply(&move_camera::GltfMoveCameraPayload { index: num(params, "index")?, position: num(params, "position")? }, before).map_err(|error| error.detail),
            "reorder-cameras" => reorder_cameras::apply(&reorder_cameras::GltfReorderCamerasPayload { order: order(params, "order")? }, before).map_err(|error| error.detail),
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
        let inverse = inverse_spec(&kind);
        let inverse_kind = inverse.str("kind");
        let inverse_empty = Json::Object(Vec::new());
        let inverse_params = inverse.get("params").unwrap_or(&inverse_empty);
        let restored = apply_kind(&mutated, &inverse_kind, inverse_params)?;
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
