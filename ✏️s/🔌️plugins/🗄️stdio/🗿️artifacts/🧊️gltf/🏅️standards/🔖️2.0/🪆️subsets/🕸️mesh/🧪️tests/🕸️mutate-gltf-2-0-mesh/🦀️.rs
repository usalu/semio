//! 🦀️ glTF 2.0 `🕸️mesh` subset mutation case — Rust adapter for the 35 kinds of the
//! `gltf-2-0-mesh` catalog (`../../🔮️oracles/🔣️.json`), which own document/meshes, their primitives and morph targets, and document/accessors. Every leaf stays
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

//#region 🔖️Kinds
const KINDS: &[&str] = &[
    "bind-morph-target-attribute",
    "bind-primitive-attribute",
    "bind-primitive-indices",
    "bind-primitive-material",
    "change-mesh-extension-data",
    "change-mesh-extra-data",
    "change-mesh-morph-weights",
    "change-mesh-name",
    "change-primitive-extension-data",
    "change-primitive-extra-data",
    "change-primitive-topology-mode",
    "create-accessor",
    "create-mesh",
    "create-morph-target",
    "create-primitive",
    "delete-accessor",
    "delete-mesh",
    "delete-morph-target",
    "delete-primitive",
    "move-accessor",
    "move-mesh",
    "move-morph-target",
    "move-morph-target-attribute",
    "move-primitive",
    "move-primitive-attribute",
    "reorder-accessors",
    "reorder-meshs",
    "reorder-morph-target-attributes",
    "reorder-morph-targets",
    "reorder-primitive-attributes",
    "reorder-primitives",
    "unbind-morph-target-attribute",
    "unbind-primitive-attribute",
    "unbind-primitive-indices",
    "unbind-primitive-material",
];
//#endregion 🔖️Kinds

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
    use semio_s_artifact_stdio_gltf::standards::v2_0::subsets::any::io::{parse_gltf_document, serialize_gltf_document, GltfAccessorType, GltfComponentType};
    use semio_s_artifact_stdio_gltf::standards::v2_0::subsets::any::schema::mutations::{bind_morph_target_attribute, bind_primitive_attribute, bind_primitive_indices, bind_primitive_material, change_mesh_extension_data, change_mesh_extra_data, change_mesh_morph_weights, change_mesh_name, change_primitive_extension_data, change_primitive_extra_data, change_primitive_topology_mode, create_accessor, create_mesh, create_morph_target, create_primitive, delete_accessor, delete_mesh, delete_morph_target, delete_primitive, move_accessor, move_mesh, move_morph_target, move_morph_target_attribute, move_primitive, move_primitive_attribute, reorder_accessors, reorder_meshs, reorder_morph_target_attributes, reorder_morph_targets, reorder_primitive_attributes, reorder_primitives, unbind_morph_target_attribute, unbind_primitive_attribute, unbind_primitive_indices, unbind_primitive_material};
    use semio_s_artifact_stdio_gltf::standards::v2_0::subsets::any::schema::snapshot::{GltfJson, GltfSnapshot};
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
    /// 🔢️ A number list payload member.
    fn floats(params: &Json, key: &str) -> Result<Vec<f64>, String> {
        match params.get(key) {
            Some(Json::Array(items)) => items
                .iter()
                .map(|item| match item {
                    Json::Number(value) => Ok(*value),
                    _ => Err(format!("`{key}` must hold only numbers")),
                })
                .collect(),
            _ => Err(format!("missing or non-array `{key}`")),
        }
    }
    /// 📝️ A string payload member.
    fn text(params: &Json, key: &str) -> Result<String, String> {
        match params.get(key) {
            Some(Json::String(value)) => Ok(value.clone()),
            _ => Err(format!("missing or non-string `{key}`")),
        }
    }
    /// 📝️ A nullable string payload member.
    fn optional_text(params: &Json, key: &str) -> Result<Option<String>, String> {
        match params.get(key) {
            Some(Json::String(value)) => Ok(Some(value.clone())),
            Some(Json::Null) | None => Ok(None),
            _ => Err(format!("`{key}` must be a string or null")),
        }
    }
    /// 📝️ A string list payload member.
    fn texts(params: &Json, key: &str) -> Result<Vec<String>, String> {
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
    //#endregion 🔖️Params

    //#region 🔖️Presence
    /// 🌱️ The snapshot's own JSON value for one spec value.
    fn gltf_json(value: &Json) -> GltfJson {
        match value {
            Json::Null => GltfJson::Null,
            Json::Bool(flag) => GltfJson::Bool(*flag),
            Json::Number(number) => GltfJson::Number(*number),
            Json::String(text) => GltfJson::String(text.clone()),
            Json::Array(items) => GltfJson::Array(items.iter().map(gltf_json).collect()),
            Json::Object(entries) => GltfJson::Object(entries.iter().map(|(key, item)| (key.clone(), gltf_json(item))).collect()),
        }
    }
    /// 🧩️ `data: {state: present, value}` as `Some(value)`, `data: {state: absent}` as `None`.
    fn presence(params: &Json) -> Result<Option<GltfJson>, String> {
        let data = params.get("data").ok_or("missing `data`")?;
        match data.get("state") {
            Some(Json::String(state)) if state == "absent" => Ok(None),
            Some(Json::String(state)) if state == "present" => data.get("value").map(|value| Some(gltf_json(value))).ok_or_else(|| "present data carries no `value`".to_string()),
            _ => Err("`data.state` must be `present` or `absent`".to_string()),
        }
    }
    //#endregion 🔖️Presence

    //#region 🔖️Dispatch
    /// 📐️ One kind through its leaf's own typed `apply()`.
    fn apply_kind(before: &GltfSnapshot, kind: &str, params: &Json) -> Result<GltfSnapshot, String> {
        match kind {
            "bind-morph-target-attribute" => bind_morph_target_attribute::apply(&bind_morph_target_attribute::GltfBindMorphTargetAttributePayload { mesh: num(params, "mesh")?, primitive: num(params, "primitive")?, target: num(params, "target")?, semantic: text(params, "semantic")?, accessor: num(params, "accessor")? }, before).map_err(|error| error.detail),
            "bind-primitive-attribute" => bind_primitive_attribute::apply(&bind_primitive_attribute::GltfBindPrimitiveAttributePayload { mesh: num(params, "mesh")?, primitive: num(params, "primitive")?, semantic: text(params, "semantic")?, accessor: num(params, "accessor")? }, before).map_err(|error| error.detail),
            "bind-primitive-indices" => bind_primitive_indices::apply(&bind_primitive_indices::GltfBindPrimitiveIndicesPayload { mesh: num(params, "mesh")?, primitive: num(params, "primitive")?, accessor: num(params, "accessor")? }, before).map_err(|error| error.detail),
            "bind-primitive-material" => bind_primitive_material::apply(&bind_primitive_material::GltfBindPrimitiveMaterialPayload { mesh: num(params, "mesh")?, primitive: num(params, "primitive")?, material: num(params, "material")? }, before).map_err(|error| error.detail),
            "change-mesh-extension-data" => change_mesh_extension_data::apply(&change_mesh_extension_data::GltfChangeMeshExtensionDataPayload { mesh: num(params, "mesh")?, data: match presence(params)? { Some(value) => change_mesh_extension_data::GltfDataPresence::Present { value }, None => change_mesh_extension_data::GltfDataPresence::Absent } }, before).map_err(|error| error.detail),
            "change-mesh-extra-data" => change_mesh_extra_data::apply(&change_mesh_extra_data::GltfChangeMeshExtraDataPayload { mesh: num(params, "mesh")?, data: match presence(params)? { Some(value) => change_mesh_extra_data::GltfDataPresence::Present { value }, None => change_mesh_extra_data::GltfDataPresence::Absent } }, before).map_err(|error| error.detail),
            "change-mesh-morph-weights" => change_mesh_morph_weights::apply(&change_mesh_morph_weights::GltfChangeMeshMorphWeightsPayload { mesh: num(params, "mesh")?, weights: floats(params, "weights")? }, before).map_err(|error| error.detail),
            "change-mesh-name" => change_mesh_name::apply(&change_mesh_name::GltfChangeMeshNamePayload { mesh: num(params, "mesh")?, value: optional_text(params, "value")? }, before).map_err(|error| error.detail),
            "change-primitive-extension-data" => change_primitive_extension_data::apply(&change_primitive_extension_data::GltfChangePrimitiveExtensionDataPayload { mesh: num(params, "mesh")?, primitive: num(params, "primitive")?, data: match presence(params)? { Some(value) => change_primitive_extension_data::GltfDataPresence::Present { value }, None => change_primitive_extension_data::GltfDataPresence::Absent } }, before).map_err(|error| error.detail),
            "change-primitive-extra-data" => change_primitive_extra_data::apply(&change_primitive_extra_data::GltfChangePrimitiveExtraDataPayload { mesh: num(params, "mesh")?, primitive: num(params, "primitive")?, data: match presence(params)? { Some(value) => change_primitive_extra_data::GltfDataPresence::Present { value }, None => change_primitive_extra_data::GltfDataPresence::Absent } }, before).map_err(|error| error.detail),
            "change-primitive-topology-mode" => change_primitive_topology_mode::apply(&change_primitive_topology_mode::GltfChangePrimitiveTopologyModePayload { mesh: num(params, "mesh")?, primitive: num(params, "primitive")?, mode: num(params, "mode")? as u64 }, before).map_err(|error| error.detail),
            "create-accessor" => create_accessor::apply(&create_accessor::GltfCreateAccessorPayload { position: num(params, "position")?, component_type: GltfComponentType::from_code(num(params, "componentType")? as u64)?, count: num(params, "count")?, kind: text(params, "kind")?.parse::<GltfAccessorType>()? }, before).map_err(|error| error.detail),
            "create-mesh" => create_mesh::apply(&create_mesh::GltfCreateMeshPayload { position: num(params, "position")? }, before).map_err(|error| error.detail),
            "create-morph-target" => create_morph_target::apply(&create_morph_target::GltfCreateMorphTargetPayload { mesh: num(params, "mesh")?, primitive: num(params, "primitive")?, position: num(params, "position")? }, before).map_err(|error| error.detail),
            "create-primitive" => create_primitive::apply(&create_primitive::GltfCreatePrimitivePayload { mesh: num(params, "mesh")?, position: num(params, "position")? }, before).map_err(|error| error.detail),
            "delete-accessor" => delete_accessor::apply(&delete_accessor::GltfDeleteAccessorPayload { index: num(params, "index")? }, before).map_err(|error| error.detail),
            "delete-mesh" => delete_mesh::apply(&delete_mesh::GltfDeleteMeshPayload { index: num(params, "index")? }, before).map_err(|error| error.detail),
            "delete-morph-target" => delete_morph_target::apply(&delete_morph_target::GltfDeleteMorphTargetPayload { mesh: num(params, "mesh")?, primitive: num(params, "primitive")?, target: num(params, "target")? }, before).map_err(|error| error.detail),
            "delete-primitive" => delete_primitive::apply(&delete_primitive::GltfDeletePrimitivePayload { mesh: num(params, "mesh")?, primitive: num(params, "primitive")? }, before).map_err(|error| error.detail),
            "move-accessor" => move_accessor::apply(&move_accessor::GltfMoveAccessorPayload { index: num(params, "index")?, position: num(params, "position")? }, before).map_err(|error| error.detail),
            "move-mesh" => move_mesh::apply(&move_mesh::GltfMoveMeshPayload { index: num(params, "index")?, position: num(params, "position")? }, before).map_err(|error| error.detail),
            "move-morph-target" => move_morph_target::apply(&move_morph_target::GltfMoveMorphTargetPayload { mesh: num(params, "mesh")?, primitive: num(params, "primitive")?, target: num(params, "target")?, position: num(params, "position")? }, before).map_err(|error| error.detail),
            "move-morph-target-attribute" => move_morph_target_attribute::apply(&move_morph_target_attribute::GltfMoveMorphTargetAttributePayload { mesh: num(params, "mesh")?, primitive: num(params, "primitive")?, target: num(params, "target")?, semantic: text(params, "semantic")?, position: num(params, "position")? }, before).map_err(|error| error.detail),
            "move-primitive" => move_primitive::apply(&move_primitive::GltfMovePrimitivePayload { mesh: num(params, "mesh")?, primitive: num(params, "primitive")?, position: num(params, "position")? }, before).map_err(|error| error.detail),
            "move-primitive-attribute" => move_primitive_attribute::apply(&move_primitive_attribute::GltfMovePrimitiveAttributePayload { mesh: num(params, "mesh")?, primitive: num(params, "primitive")?, semantic: text(params, "semantic")?, position: num(params, "position")? }, before).map_err(|error| error.detail),
            "reorder-accessors" => reorder_accessors::apply(&reorder_accessors::GltfReorderAccessorsPayload { order: order(params, "order")? }, before).map_err(|error| error.detail),
            "reorder-meshs" => reorder_meshs::apply(&reorder_meshs::GltfReorderMeshsPayload { order: order(params, "order")? }, before).map_err(|error| error.detail),
            "reorder-morph-target-attributes" => reorder_morph_target_attributes::apply(&reorder_morph_target_attributes::GltfReorderMorphTargetAttributesPayload { mesh: num(params, "mesh")?, primitive: num(params, "primitive")?, target: num(params, "target")?, order: texts(params, "order")? }, before).map_err(|error| error.detail),
            "reorder-morph-targets" => reorder_morph_targets::apply(&reorder_morph_targets::GltfReorderMorphTargetsPayload { mesh: num(params, "mesh")?, primitive: num(params, "primitive")?, order: order(params, "order")? }, before).map_err(|error| error.detail),
            "reorder-primitive-attributes" => reorder_primitive_attributes::apply(&reorder_primitive_attributes::GltfReorderPrimitiveAttributesPayload { mesh: num(params, "mesh")?, primitive: num(params, "primitive")?, order: texts(params, "order")? }, before).map_err(|error| error.detail),
            "reorder-primitives" => reorder_primitives::apply(&reorder_primitives::GltfReorderPrimitivesPayload { mesh: num(params, "mesh")?, order: order(params, "order")? }, before).map_err(|error| error.detail),
            "unbind-morph-target-attribute" => unbind_morph_target_attribute::apply(&unbind_morph_target_attribute::GltfUnbindMorphTargetAttributePayload { mesh: num(params, "mesh")?, primitive: num(params, "primitive")?, target: num(params, "target")?, semantic: text(params, "semantic")? }, before).map_err(|error| error.detail),
            "unbind-primitive-attribute" => unbind_primitive_attribute::apply(&unbind_primitive_attribute::GltfUnbindPrimitiveAttributePayload { mesh: num(params, "mesh")?, primitive: num(params, "primitive")?, semantic: text(params, "semantic")? }, before).map_err(|error| error.detail),
            "unbind-primitive-indices" => unbind_primitive_indices::apply(&unbind_primitive_indices::GltfUnbindPrimitiveIndicesPayload { mesh: num(params, "mesh")?, primitive: num(params, "primitive")? }, before).map_err(|error| error.detail),
            "unbind-primitive-material" => unbind_primitive_material::apply(&unbind_primitive_material::GltfUnbindPrimitiveMaterialPayload { mesh: num(params, "mesh")?, primitive: num(params, "primitive")? }, before).map_err(|error| error.detail),
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
