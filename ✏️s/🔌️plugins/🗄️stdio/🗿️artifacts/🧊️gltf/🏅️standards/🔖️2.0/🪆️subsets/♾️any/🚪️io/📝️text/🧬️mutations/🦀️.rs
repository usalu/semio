//! 📝️ Generic text framing for the visible glTF mutation aggregate.
use crate::schema::snapshot::GltfSnapshot;

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

use crate::schema::mutations::GltfMutation;

const GLTF_MUTATION_MAX_PAYLOAD_BYTES: usize = 64 * 1024;

/// 🎯️ One mutation applied through the central applier: the next snapshot, or the refusal's code and detail.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_gltf_mutation(base: &GltfSnapshot, mutation: &GltfMutation) -> Result<GltfSnapshot, String> {
    let outcome = <GltfMutation as protocol::Mutation<GltfSnapshot>>::diff(mutation, base);
    if let Some(message) = outcome.messages().iter().find(|message| message.level >= semio_framework_diagnostic::Severity::Error) {
        return Err(format!("{} {}", message.code.0, message.message));
    }
    protocol::apply_diff(outcome.diff(), base).map_err(|error| error.to_string())
}

/// 🌉️ Applies one mutation through the central applier; a refusal (error or fatal message) is an error.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn gltf_bridge_apply(kind: &str, step: &str, mutation: &GltfMutation, base: &GltfSnapshot) -> Result<GltfSnapshot, String> {
    apply_gltf_mutation(base, mutation).map_err(|refusal| format!("{kind}: the {step} was refused — {refusal}"))
}

fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut text = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        text.push(HEX[(byte >> 4) as usize] as char);
        text.push(HEX[(byte & 0x0f) as usize] as char);
    }
    text
}

fn decode_hex(value: &str) -> Result<Vec<u8>, String> {
    if !value.len().is_multiple_of(2) || value.len() > GLTF_MUTATION_MAX_PAYLOAD_BYTES * 2 {
        return Err("GLTF mutation text payload exceeds its budget".into());
    }
    fn nibble(value: u8) -> Option<u8> {
        if value.is_ascii_digit() {
            return Some(value - b'0');
        }
        (b'a'..=b'f').contains(&value).then_some(value - b'a' + 10)
    }
    value
        .as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| {
            let high = nibble(pair[0]).ok_or_else(|| "GLTF mutation payload must be lowercase hexadecimal".to_string())?;
            let low = nibble(pair[1]).ok_or_else(|| "GLTF mutation payload must be lowercase hexadecimal".to_string())?;
            Ok((high << 4) | low)
        })
        .collect()
}

fn text_error(detail: impl Into<String>) -> semio_framework_diagnostic::TextError {
    semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail.into(), semio_framework_diagnostic::TextSpan::at(1, 1))
}

impl protocol::OpText for GltfMutation {
    fn print_op(&self) -> String {
        format!("gltf-mutation payload={}", encode_hex(semio_framework_pack_json::to_json_string(self).as_bytes()))
    }

    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let payload = line.strip_prefix("gltf-mutation payload=").ok_or_else(|| text_error("expected canonical GLTF mutation aggregate"))?;
        let bytes = decode_hex(payload).map_err(text_error)?;
        let text = std::str::from_utf8(&bytes).map_err(|error| text_error(error.to_string()))?;
        semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|cause| semio_framework_diagnostic::TextError::from_value_error(cause, semio_framework_diagnostic::TextSpan::at(1,1)))
    }
}

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v2_0::subsets::any::schema::mutations::*;
use crate::schema::diff::GltfDiff;
use crate::GltfSnapshot;
use crate::standards::v2_0::subsets::any::schema::mutations::add_required_extension::AddRequiredExtensionMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::add_used_extension::AddUsedExtensionMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::bind_default_scene::BindDefaultSceneMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::bind_morph_target_attribute::BindMorphTargetAttributeMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::bind_node_camera::BindNodeCameraMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::bind_node_child::BindNodeChildMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::bind_node_mesh::BindNodeMeshMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::bind_node_skin::BindNodeSkinMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::bind_primitive_attribute::BindPrimitiveAttributeMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::bind_primitive_indices::BindPrimitiveIndicesMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::bind_primitive_material::BindPrimitiveMaterialMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::bind_scene_root_node::BindSceneRootNodeMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::change_asset_descriptive_metadata::ChangeAssetDescriptiveMetadataMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::change_asset_extension_data::ChangeAssetExtensionDataMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::change_asset_extra_data::ChangeAssetExtraDataMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::change_asset_version::ChangeAssetVersionMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::change_document_extension_data::ChangeDocumentExtensionDataMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::change_document_extra_data::ChangeDocumentExtraDataMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::change_material_alpha_mode::ChangeMaterialAlphaModeMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::change_material_double_sided::ChangeMaterialDoubleSidedMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::change_mesh_extension_data::ChangeMeshExtensionDataMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::change_mesh_extra_data::ChangeMeshExtraDataMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::change_mesh_morph_weights::ChangeMeshMorphWeightsMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::change_mesh_name::ChangeMeshNameMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::change_node_extension_data::ChangeNodeExtensionDataMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::change_node_extra_data::ChangeNodeExtraDataMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::change_node_morph_weights::ChangeNodeMorphWeightsMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::change_node_name::ChangeNodeNameMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::change_node_transform::ChangeNodeTransformMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::change_primitive_extension_data::ChangePrimitiveExtensionDataMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::change_primitive_extra_data::ChangePrimitiveExtraDataMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::change_primitive_topology_mode::ChangePrimitiveTopologyModeMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::change_scene_extension_data::ChangeSceneExtensionDataMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::change_scene_extra_data::ChangeSceneExtraDataMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::change_scene_name::ChangeSceneNameMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::create_accessor::CreateAccessorMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::create_animation::CreateAnimationMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::create_buffer::CreateBufferMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::create_buffer_view::CreateBufferViewMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::create_camera::CreateCameraMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::create_image::CreateImageMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::create_material::CreateMaterialMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::create_mesh::CreateMeshMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::create_morph_target::CreateMorphTargetMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::create_node::CreateNodeMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::create_primitive::CreatePrimitiveMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::create_sampler::CreateSamplerMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::create_scene::CreateSceneMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::create_skin::CreateSkinMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::create_texture::CreateTextureMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::delete_accessor::DeleteAccessorMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::delete_animation::DeleteAnimationMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::delete_buffer::DeleteBufferMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::delete_buffer_view::DeleteBufferViewMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::delete_camera::DeleteCameraMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::delete_image::DeleteImageMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::delete_material::DeleteMaterialMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::delete_mesh::DeleteMeshMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::delete_morph_target::DeleteMorphTargetMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::delete_node::DeleteNodeMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::delete_primitive::DeletePrimitiveMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::delete_sampler::DeleteSamplerMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::delete_scene::DeleteSceneMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::delete_skin::DeleteSkinMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::delete_texture::DeleteTextureMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::move_accessor::MoveAccessorMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::move_animation::MoveAnimationMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::move_buffer::MoveBufferMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::move_buffer_view::MoveBufferViewMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::move_camera::MoveCameraMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::move_image::MoveImageMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::move_material::MoveMaterialMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::move_mesh::MoveMeshMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::move_morph_target::MoveMorphTargetMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::move_morph_target_attribute::MoveMorphTargetAttributeMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::move_node::MoveNodeMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::move_node_child::MoveNodeChildMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::move_node_parent::MoveNodeParentMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::move_primitive::MovePrimitiveMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::move_primitive_attribute::MovePrimitiveAttributeMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::move_required_extension::MoveRequiredExtensionMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::move_sampler::MoveSamplerMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::move_scene::MoveSceneMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::move_scene_root_node::MoveSceneRootNodeMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::move_skin::MoveSkinMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::move_texture::MoveTextureMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::move_used_extension::MoveUsedExtensionMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::remove_required_extension::RemoveRequiredExtensionMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::remove_used_extension::RemoveUsedExtensionMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::reorder_accessors::ReorderAccessorsMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::reorder_animations::ReorderAnimationsMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::reorder_buffer_views::ReorderBufferViewsMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::reorder_buffers::ReorderBuffersMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::reorder_cameras::ReorderCamerasMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::reorder_images::ReorderImagesMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::reorder_materials::ReorderMaterialsMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::reorder_meshs::ReorderMeshsMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::reorder_morph_target_attributes::ReorderMorphTargetAttributesMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::reorder_morph_targets::ReorderMorphTargetsMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::reorder_node_children::ReorderNodeChildrenMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::reorder_nodes::ReorderNodesMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::reorder_primitive_attributes::ReorderPrimitiveAttributesMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::reorder_primitives::ReorderPrimitivesMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::reorder_required_extensions::ReorderRequiredExtensionsMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::reorder_samplers::ReorderSamplersMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::reorder_scene_root_nodes::ReorderSceneRootNodesMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::reorder_scenes::ReorderScenesMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::reorder_skins::ReorderSkinsMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::reorder_textures::ReorderTexturesMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::reorder_used_extensions::ReorderUsedExtensionsMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::unbind_default_scene::UnbindDefaultSceneMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::unbind_morph_target_attribute::UnbindMorphTargetAttributeMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::unbind_node_camera::UnbindNodeCameraMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::unbind_node_child::UnbindNodeChildMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::unbind_node_mesh::UnbindNodeMeshMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::unbind_node_skin::UnbindNodeSkinMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::unbind_primitive_attribute::UnbindPrimitiveAttributeMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::unbind_primitive_indices::UnbindPrimitiveIndicesMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::unbind_primitive_material::UnbindPrimitiveMaterialMutation;
use crate::standards::v2_0::subsets::any::schema::mutations::unbind_scene_root_node::UnbindSceneRootNodeMutation;

/// 🌉️ The `(kind, params)` row a mutation case states (`delete-camera`, `{"index":0}`) as the [`GltfMutation`] wire form
/// `{"mutation": "deleteCamera", "payload": {"phase": "apply", "value": params}}`, decoded by the production codec.
pub(crate) fn gltf_row_mutation(kind: &str, params_json: &str) -> Result<GltfMutation, String> {
    let params: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(params_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| format!("{kind}: the parameters are not JSON: {error}"))?;
    let variant: String = kind
        .split('-')
        .enumerate()
        .map(|(index, word)| {
            let mut letters = word.chars();
            match letters.next() {
                Some(first) if index > 0 => first.to_uppercase().chain(letters).collect::<String>(),
                _ => word.to_string(),
            }
        })
        .collect();
    let payload = semio_framework_value::DslValue::object(vec![("phase".to_string(), semio_framework_value::DslValue::String("apply".to_string())), ("value".to_string(), params)]);
    let wire = semio_framework_value::DslValue::object(vec![("mutation".to_string(), semio_framework_value::DslValue::String(variant)), ("payload".to_string(), payload)]);
    <GltfMutation as semio_framework_value::FromValue>::from_value(wire).map_err(|error| format!("{kind}: not a glTF mutation of this vocabulary: {error}"))
}
}
pub use mutations_codec::*;

mod node_name_facades {
use crate::standards::v2_0::subsets::any::schema::mutations::change_node_name::*;
use semio_framework_value::DslValue;
//#region 🔗️Facade
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GltfChangeNodeNameFacadeError {
    pub code: &'static str,
    pub path: String,
}

impl std::fmt::Display for GltfChangeNodeNameFacadeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{} at {}", self.code, self.path)
    }
}

impl std::error::Error for GltfChangeNodeNameFacadeError {}

pub(crate) type FacadeResult<T> = Result<T, GltfChangeNodeNameFacadeError>;

pub(crate) fn facade_error<T>(code: &'static str, path: impl Into<String>) -> FacadeResult<T> {
    Err(GltfChangeNodeNameFacadeError { code, path: path.into() })
}

fn exact_object<'a>(value: &'a DslValue, allowed: &[&str], path: &str) -> FacadeResult<&'a [(String, DslValue)]> {
    let object = match value.as_object() {
        Some(object) => object,
        None => return facade_error("object", path),
    };
    if object.iter().any(|(key, _)| !allowed.contains(&key.as_str())) {
        return facade_error("unknown", path);
    }
    if object.iter().enumerate().any(|(index, (key, _))| object[..index].iter().any(|(prior, _)| prior == key)) {
        return facade_error("duplicate", path);
    }
    Ok(object)
}

fn object_field<'a>(object: &'a [(String, DslValue)], key: &str) -> Option<&'a DslValue> {
    object.iter().find(|(name, _)| name == key).map(|(_, value)| value)
}

fn json_node(value: &DslValue, path: &str) -> FacadeResult<u32> {
    match value.as_f64() {
        Some(node) if node.is_finite() && node.fract() == 0.0 && (0.0..=f64::from(u32::MAX)).contains(&node) => Ok(node as u32),
        _ => facade_error("node", path),
    }
}

fn graphql_optional(value: &DslValue, path: &str) -> FacadeResult<Option<String>> {
    let object = exact_object(value, &["present", "absent"], path)?;
    match (object_field(object, "present"), object_field(object, "absent")) {
        (Some(semio_framework_value::DslValue::String(value)), None) => Ok(Some(value.clone())),
        (None, Some(semio_framework_value::DslValue::Bool(true))) => Ok(None),
        _ => facade_error("nullable", path),
    }
}

fn proto_optional(value: &DslValue, path: &str) -> FacadeResult<Option<String>> {
    let object = exact_object(value, &["present", "absent"], path)?;
    match (object_field(object, "present"), object_field(object, "absent")) {
        (Some(semio_framework_value::DslValue::String(value)), None) => Ok(Some(value.clone())),
        (None, Some(absent)) if exact_object(absent, &[], path)?.is_empty() => Ok(None),
        _ => facade_error("nullable", path),
    }
}

fn decode_object(value: &DslValue, optional: fn(&DslValue, &str) -> FacadeResult<Option<String>>, path: &str) -> FacadeResult<ChangeNodeNameMutation> {
    let root = exact_object(value, &["apply"], path)?;
    let Some(apply) = object_field(root, "apply") else {
        return facade_error("phase", path);
    };
    let apply = exact_object(apply, &["node", "value"], &format!("{path}.apply"))?;
    let node = match object_field(apply, "node") {
        Some(node) => json_node(node, &format!("{path}.apply.node"))?,
        None => return facade_error("node", format!("{path}.apply.node")),
    };
    let value = match object_field(apply, "value") {
        Some(value) => optional(value, &format!("{path}.apply.value"))?,
        None => return facade_error("nullable", format!("{path}.apply.value")),
    };
    Ok(ChangeNodeNameMutation::Apply(GltfChangeNodeNamePayload { node, value }))
}

pub fn decode_gltf_change_node_name_graphql(value: &DslValue) -> FacadeResult<ChangeNodeNameMutation> {
    decode_object(value, graphql_optional, "graphql")
}

pub fn decode_gltf_change_node_name_proto(value: &DslValue) -> FacadeResult<ChangeNodeNameMutation> {
    decode_object(value, proto_optional, "proto")
}


}
pub use node_name_facades::{GltfChangeNodeNameFacadeError,decode_gltf_change_node_name_graphql,decode_gltf_change_node_name_proto};
pub(crate) use node_name_facades::{FacadeResult,facade_error};

fn gltf_bridge_read(document: &[u8]) -> Result<GltfSnapshot, String> {
    if document.starts_with(b"glTF") { crate::engine::decode_glb(document) } else { crate::engine::parse_gltf_document(document) }
}

fn gltf_bridge_write(snapshot: &GltfSnapshot) -> Result<Vec<u8>, String> {
    match snapshot.source_form {
        crate::schema::snapshot::GltfSourceForm::Glb => crate::engine::encode_glb(snapshot),
        crate::schema::snapshot::GltfSourceForm::Json => Ok(crate::engine::serialize_gltf_document(snapshot)),
    }
}

pub fn gltf_mutated_document(document: &[u8], kind: &str, params_json: &str) -> Result<Vec<u8>, String> {
    let mutation = gltf_row_mutation(kind, params_json)?;
    let snapshot = gltf_bridge_read(document)?;
    let mutated = gltf_bridge_apply(kind, "mutation", &mutation, &snapshot)?;
    gltf_bridge_write(&mutated)
}

pub fn gltf_inverse_restored_document(document: &[u8], kind: &str, params_json: &str) -> Result<Vec<u8>, String> {
    let mutation = gltf_row_mutation(kind, params_json)?;
    let base = gltf_bridge_read(document)?;
    let mut restored = gltf_bridge_apply(kind, "mutation", &mutation, &base)?;
    for step in <GltfMutation as protocol::Mutation<GltfSnapshot>>::inverse(&mutation, &base).map_err(semio_framework_value::ValueError::into_message)?.into_iter().rev() {
        restored = gltf_bridge_apply(kind, "inverse", &step, &restored)?;
    }
    gltf_bridge_write(&restored)
}

