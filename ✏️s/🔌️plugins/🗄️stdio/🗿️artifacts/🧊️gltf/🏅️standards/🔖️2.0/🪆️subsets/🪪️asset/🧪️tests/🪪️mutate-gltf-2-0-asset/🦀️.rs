//! 🦀️ glTF 2.0 `🪪️asset` subset mutation case — Rust adapter. Covers the 14 kinds
//! `../../🔮️oracle/🔣️.json`'s `gltf-2-0-asset` catalog declares: `add-required-extension`,
//! `add-used-extension`, `change-asset-descriptive-metadata`, `change-asset-extension-data`,
//! `change-asset-extra-data`, `change-asset-version`, `change-document-extension-data`,
//! `change-document-extra-data`, `move-required-extension`, `move-used-extension`,
//! `remove-required-extension`, `remove-used-extension`, `reorder-required-extensions`,
//! `reorder-used-extensions`. Every leaf's own `apply()` (`../../../♾️any/🧬️schema/🧬️mutations/
//! {✅️required-extension/{➕️add,➖️remove,🚚️move,🔀️reorder},
//! 📣️used-extension/{➕️add,➖️remove,🚚️move,🔀️reorder},
//! 🪪️asset/{📝️change-description,🧩️change-extensions,🧾️change-extras,🔖️version},
//! 📃️document/{🧩️change-extensions,📝️change-extras}}/🦀️.rs`) stays physically owned by `♾️any` —
//! `validate_mutation_leaf_source` requires the exact registered domain/operation owner beneath its
//! aggregate mutation root, so this case reaches it by import. The oracle performs every kind by independent GLB/JSON-tree
//! manipulation (`../../../♾️any/🔮️oracle/🦀️.rs`, extended with these 14 kinds by this same change,
//! using `json` 0.12 as the JSON layer only, never this subset's own codec); the subject fully parses
//! each kind's own committed fixture into `GltfSnapshot` via `parse_gltf_document` and re-serializes
//! with `serialize_gltf_document` alone, dispatching through each leaf's own typed `apply()` function
//! directly. Both results are read back by the INDEPENDENT `project_gltf` reader before the
//! `semantic-gltf-v1` profile compares them.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::gltf::standards::v2_0::subsets::any::{oracle_apply_mutation, project_gltf};
use semio_s_plugin_stdio_test_oracle::law::{inverse_restores_within, mutation_is_observable_within};

//#region 🔖️Kinds
const KINDS: &[&str] = &[
    "add-required-extension",
    "add-used-extension",
    "remove-used-extension",
    "remove-required-extension",
    "move-used-extension",
    "move-required-extension",
    "reorder-used-extensions",
    "reorder-required-extensions",
    "change-asset-descriptive-metadata",
    "change-asset-version",
    "change-asset-extension-data",
    "change-asset-extra-data",
    "change-document-extension-data",
    "change-document-extra-data",
];
//#endregion 🔖️Kinds

//#region 🔖️Input
/// 🧫️ Each kind owns its own committed `before.gltf` (`../../🧫️fixtures/<kind>-applied/`, shared
/// against this case's own owner — `shared://` resolves there since `🧪️tests` sits directly under
/// `🪪️asset`). Copies into the work directory; the committed fixture itself is never written to.
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
fn json_str(value: &str) -> Json {
    Json::String(value.to_string())
}
fn json_str_arr(values: &[&str]) -> Json {
    Json::Array(values.iter().map(|value| json_str(value)).collect())
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
/// ↩️ The semantically correct inverse spec for one forward `(kind, params)` pair against the
/// kind's own committed fixture (`../../🧫️fixtures/<kind>-applied/before.gltf`), computed
/// independently here since the oracle role must not link the subject crate. Each spec below was
/// derived from the committed fixture's own before/after diff, documented in the feature file. An
/// object member's inverse omits a key entirely (rather than writing `null`) whenever the BEFORE
/// document never carried that key — the same `Option::None`-is-absence convention
/// `optional_str_param`/`optional_object_param` (`../../../♾️any/🔮️oracle/🦀️.rs`) read on the way in.
fn inverse_spec(kind: &str) -> Json {
    match kind {
        "add-required-extension" => json_spec("remove-required-extension", json_obj(vec![("extension", json_str("KHR_materials_unlit"))])),
        "add-used-extension" => json_spec("remove-used-extension", json_obj(vec![("extension", json_str("ACME_marker"))])),
        "remove-used-extension" => json_spec("add-used-extension", json_obj(vec![("extension", json_str("KHR_materials_unlit")), ("position", json_num(0.0))])),
        "remove-required-extension" => json_spec("add-required-extension", json_obj(vec![("extension", json_str("KHR_materials_unlit")), ("position", json_num(0.0))])),
        "move-used-extension" => json_spec("move-used-extension", json_obj(vec![("extension", json_str("ACME_marker")), ("position", json_num(1.0))])),
        "move-required-extension" => json_spec("move-required-extension", json_obj(vec![("extension", json_str("ACME_marker")), ("position", json_num(1.0))])),
        "reorder-used-extensions" => json_spec("reorder-used-extensions", json_obj(vec![("order", json_str_arr(&["KHR_materials_unlit", "ACME_marker"]))])),
        "reorder-required-extensions" => json_spec("reorder-required-extensions", json_obj(vec![("order", json_str_arr(&["KHR_materials_unlit", "ACME_marker"]))])),
        "change-asset-descriptive-metadata" => json_spec("change-asset-descriptive-metadata", json_obj(vec![("generator", json_str("three.js GLTFExporter (semio oracle fixture base)")), ("copyright", json_str("2026 Ueli Saluz — CC0 fixture data"))])),
        "change-asset-version" => json_spec("change-asset-version", json_obj(vec![("version", json_str("2.0"))])),
        "change-asset-extension-data" => json_spec("change-asset-extension-data", json_obj(vec![])),
        "change-asset-extra-data" => json_spec("change-asset-extra-data", json_obj(vec![("data", json_obj(vec![("fixtureBase", json_str("gltf-2-0-any-reader-oracle")), ("revision", json_num(1.0))]))])),
        "change-document-extension-data" => json_spec("change-document-extension-data", json_obj(vec![])),
        "change-document-extra-data" => json_spec("change-document-extra-data", json_obj(vec![("data", json_obj(vec![("documentPurpose", json_str("semio reader-oracle base document"))]))])),
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
    use semio_s_plugin_stdio::artifacts::gltf::standards::v2_0::subsets::any::schema::mutations::{add_required_extension, add_used_extension, change_asset_descriptive_metadata, change_asset_extension_data, change_asset_extra_data, change_asset_version, change_document_extension_data, change_document_extra_data, move_required_extension, move_used_extension, remove_required_extension, remove_used_extension, reorder_required_extensions, reorder_used_extensions};
    use semio_s_plugin_stdio::artifacts::gltf::standards::v2_0::subsets::any::schema::snapshot::{GltfJson, GltfSnapshot};
    use semio_s_plugin_stdio_test_oracle::artifacts::gltf::standards::v2_0::subsets::any::project_gltf;

    //#region 🔖️Params
    fn str_field(params: &Json, key: &str) -> Result<String, String> {
        match params.get(key) {
            Some(Json::String(value)) => Ok(value.clone()),
            _ => Err(format!("missing or non-string `{key}`")),
        }
    }
    /// 🔎️ `None` for an absent or non-string key — the SAME "absent means `None`" convention the
    /// oracle's `optional_str_param` reads, matching an `Option<String>` payload field.
    fn optional_str_field(params: &Json, key: &str) -> Option<String> {
        match params.get(key) {
            Some(Json::String(value)) => Some(value.clone()),
            _ => None,
        }
    }
    fn num(params: &Json, key: &str) -> Result<usize, String> {
        match params.get(key) {
            Some(Json::Number(value)) => Ok(*value as usize),
            _ => Err(format!("missing or non-numeric `{key}`")),
        }
    }
    fn string_array(params: &Json, key: &str) -> Result<Vec<String>, String> {
        match params.get(key) {
            Some(Json::Array(items)) => items
                .iter()
                .map(|item| match item {
                    Json::String(value) => Ok(value.clone()),
                    _ => Err(format!("`{key}` must hold only strings")),
                })
                .collect(),
            _ => Err(format!("missing or non-array `{key}`")),
        }
    }
    /// 🌉️ This host's own `Json` → `GltfJson` — structural only, the production twin of the
    /// oracle's `from_host_json`/`to_host_json` bridge (`../../../♾️any/🔮️oracle/🦀️.rs`), used
    /// solely to carry `change-{asset,document}-{extension,extra}-data`'s opaque `data` param into
    /// a typed payload without this adapter needing any of `GltfJson`'s own variant names beyond
    /// its shape (`Null`/`Bool`/`Number`/`String`/`Array`/`Object` — verified structurally identical
    /// to this host's `Json` before writing this, not assumed).
    fn to_gltf_json(value: &Json) -> GltfJson {
        match value {
            Json::Null => GltfJson::Null,
            Json::Bool(flag) => GltfJson::Bool(*flag),
            Json::Number(number) => GltfJson::Number(*number),
            Json::String(text) => GltfJson::String(text.clone()),
            Json::Array(items) => GltfJson::Array(items.iter().map(to_gltf_json).collect()),
            Json::Object(entries) => GltfJson::Object(entries.iter().map(|(key, item)| (key.clone(), to_gltf_json(item))).collect()),
        }
    }
    /// 🔎️ `None` for an absent, `null`, or non-object key — the same convention the oracle's
    /// `optional_object_param` reads, matching an `Option<GltfJson>` payload field.
    fn optional_gltf_json(params: &Json, key: &str) -> Option<GltfJson> {
        match params.get(key) {
            Some(value @ Json::Object(_)) => Some(to_gltf_json(value)),
            _ => None,
        }
    }
    //#endregion 🔖️Params

    //#region 🔖️Dispatch
    /// 📐️ Full parse → typed leaf `apply()` → re-serialize from the model alone — the
    /// no-byte-pass-through rule this wave exists to enforce. Dispatches through each of the 14
    /// leaves' own real `apply()` directly, the same simple typed-payload shape every camera/skin/
    /// animation leaf exposes.
    fn apply_kind(before: &GltfSnapshot, kind: &str, params: &Json) -> Result<GltfSnapshot, String> {
        match kind {
            "add-required-extension" => add_required_extension::apply(&add_required_extension::GltfRequireExtensionPayload { extension: str_field(params, "extension")?, position: num(params, "position")? }, before).map_err(|error| error.detail),
            "add-used-extension" => add_used_extension::apply(&add_used_extension::GltfDeclareUsedExtensionPayload { extension: str_field(params, "extension")?, position: num(params, "position")? }, before).map_err(|error| error.detail),
            "remove-used-extension" => remove_used_extension::apply(&remove_used_extension::GltfWithdrawUsedExtensionPayload { extension: str_field(params, "extension")? }, before).map_err(|error| error.detail),
            "remove-required-extension" => remove_required_extension::apply(&remove_required_extension::GltfUnrequireExtensionPayload { extension: str_field(params, "extension")? }, before).map_err(|error| error.detail),
            "move-used-extension" => move_used_extension::apply(&move_used_extension::GltfMoveUsedExtensionPayload { extension: str_field(params, "extension")?, position: num(params, "position")? }, before).map_err(|error| error.detail),
            "move-required-extension" => move_required_extension::apply(&move_required_extension::GltfMoveRequiredExtensionPayload { extension: str_field(params, "extension")?, position: num(params, "position")? }, before).map_err(|error| error.detail),
            "reorder-used-extensions" => reorder_used_extensions::apply(&reorder_used_extensions::GltfReorderUsedExtensionsPayload { order: string_array(params, "order")? }, before).map_err(|error| error.detail),
            "reorder-required-extensions" => reorder_required_extensions::apply(&reorder_required_extensions::GltfReorderRequiredExtensionsPayload { order: string_array(params, "order")? }, before).map_err(|error| error.detail),
            "change-asset-descriptive-metadata" => change_asset_descriptive_metadata::apply(&change_asset_descriptive_metadata::GltfChangeAssetDescriptiveMetadataPayload { generator: optional_str_field(params, "generator"), copyright: optional_str_field(params, "copyright"), min_version: optional_str_field(params, "minVersion") }, before).map_err(|error| error.detail),
            "change-asset-version" => change_asset_version::apply(&change_asset_version::GltfChangeAssetVersionPayload { version: str_field(params, "version")? }, before).map_err(|error| error.detail),
            "change-asset-extension-data" => change_asset_extension_data::apply(&change_asset_extension_data::GltfChangeAssetExtensionDataPayload { data: optional_gltf_json(params, "data") }, before).map_err(|error| error.detail),
            "change-asset-extra-data" => change_asset_extra_data::apply(&change_asset_extra_data::GltfChangeAssetExtraDataPayload { data: optional_gltf_json(params, "data") }, before).map_err(|error| error.detail),
            "change-document-extension-data" => change_document_extension_data::apply(&change_document_extension_data::GltfChangeDocumentExtensionDataPayload { data: optional_gltf_json(params, "data") }, before).map_err(|error| error.detail),
            "change-document-extra-data" => change_document_extra_data::apply(&change_document_extra_data::GltfChangeDocumentExtraDataPayload { data: optional_gltf_json(params, "data") }, before).map_err(|error| error.detail),
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
