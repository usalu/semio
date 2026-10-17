//! 🧬️ Transparent glTF mutation aggregate. Every concrete payload, outcome, diff, inverse, and test lives in its direct semantic leaf.

use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::diff::GltfDiff;

use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;

pub use crate::standards::v2_0::subsets::any::schema::mutations::add_required_extension::AddRequiredExtensionMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::add_used_extension::AddUsedExtensionMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::bind_default_scene::BindDefaultSceneMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::bind_morph_target_attribute::BindMorphTargetAttributeMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::bind_node_camera::BindNodeCameraMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::bind_node_child::BindNodeChildMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::bind_node_mesh::BindNodeMeshMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::bind_node_skin::BindNodeSkinMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::bind_primitive_attribute::BindPrimitiveAttributeMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::bind_primitive_indices::BindPrimitiveIndicesMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::bind_primitive_material::BindPrimitiveMaterialMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::bind_scene_root_node::BindSceneRootNodeMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::change_asset_descriptive_metadata::ChangeAssetDescriptiveMetadataMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::change_asset_extension_data::ChangeAssetExtensionDataMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::change_asset_extra_data::ChangeAssetExtraDataMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::change_asset_version::ChangeAssetVersionMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::change_document_extension_data::ChangeDocumentExtensionDataMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::change_document_extra_data::ChangeDocumentExtraDataMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::change_material_alpha_mode::ChangeMaterialAlphaModeMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::change_material_double_sided::ChangeMaterialDoubleSidedMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::change_mesh_extension_data::ChangeMeshExtensionDataMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::change_mesh_extra_data::ChangeMeshExtraDataMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::change_mesh_morph_weights::ChangeMeshMorphWeightsMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::change_mesh_name::ChangeMeshNameMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::change_node_extension_data::ChangeNodeExtensionDataMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::change_node_extra_data::ChangeNodeExtraDataMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::change_node_morph_weights::ChangeNodeMorphWeightsMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::change_node_name::ChangeNodeNameMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::change_node_transform::ChangeNodeTransformMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::change_primitive_extension_data::ChangePrimitiveExtensionDataMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::change_primitive_extra_data::ChangePrimitiveExtraDataMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::change_primitive_topology_mode::ChangePrimitiveTopologyModeMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::change_scene_extension_data::ChangeSceneExtensionDataMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::change_scene_extra_data::ChangeSceneExtraDataMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::change_scene_name::ChangeSceneNameMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::create_accessor::CreateAccessorMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::create_animation::CreateAnimationMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::create_buffer::CreateBufferMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::create_buffer_view::CreateBufferViewMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::create_camera::CreateCameraMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::create_image::CreateImageMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::create_material::CreateMaterialMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::create_mesh::CreateMeshMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::create_morph_target::CreateMorphTargetMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::create_node::CreateNodeMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::create_primitive::CreatePrimitiveMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::create_sampler::CreateSamplerMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::create_scene::CreateSceneMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::create_skin::CreateSkinMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::create_texture::CreateTextureMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::delete_accessor::DeleteAccessorMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::delete_animation::DeleteAnimationMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::delete_buffer::DeleteBufferMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::delete_buffer_view::DeleteBufferViewMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::delete_camera::DeleteCameraMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::delete_image::DeleteImageMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::delete_material::DeleteMaterialMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::delete_mesh::DeleteMeshMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::delete_morph_target::DeleteMorphTargetMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::delete_node::DeleteNodeMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::delete_primitive::DeletePrimitiveMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::delete_sampler::DeleteSamplerMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::delete_scene::DeleteSceneMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::delete_skin::DeleteSkinMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::delete_texture::DeleteTextureMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::move_accessor::MoveAccessorMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::move_animation::MoveAnimationMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::move_buffer::MoveBufferMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::move_buffer_view::MoveBufferViewMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::move_camera::MoveCameraMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::move_image::MoveImageMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::move_material::MoveMaterialMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::move_mesh::MoveMeshMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::move_morph_target::MoveMorphTargetMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::move_morph_target_attribute::MoveMorphTargetAttributeMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::move_node::MoveNodeMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::move_node_child::MoveNodeChildMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::move_node_parent::MoveNodeParentMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::move_primitive::MovePrimitiveMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::move_primitive_attribute::MovePrimitiveAttributeMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::move_required_extension::MoveRequiredExtensionMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::move_sampler::MoveSamplerMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::move_scene::MoveSceneMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::move_scene_root_node::MoveSceneRootNodeMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::move_skin::MoveSkinMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::move_texture::MoveTextureMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::move_used_extension::MoveUsedExtensionMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::remove_required_extension::RemoveRequiredExtensionMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::remove_used_extension::RemoveUsedExtensionMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::reorder_accessors::ReorderAccessorsMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::reorder_animations::ReorderAnimationsMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::reorder_buffer_views::ReorderBufferViewsMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::reorder_buffers::ReorderBuffersMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::reorder_cameras::ReorderCamerasMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::reorder_images::ReorderImagesMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::reorder_materials::ReorderMaterialsMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::reorder_meshs::ReorderMeshsMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::reorder_morph_target_attributes::ReorderMorphTargetAttributesMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::reorder_morph_targets::ReorderMorphTargetsMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::reorder_node_children::ReorderNodeChildrenMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::reorder_nodes::ReorderNodesMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::reorder_primitive_attributes::ReorderPrimitiveAttributesMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::reorder_primitives::ReorderPrimitivesMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::reorder_required_extensions::ReorderRequiredExtensionsMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::reorder_samplers::ReorderSamplersMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::reorder_scene_root_nodes::ReorderSceneRootNodesMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::reorder_scenes::ReorderScenesMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::reorder_skins::ReorderSkinsMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::reorder_textures::ReorderTexturesMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::reorder_used_extensions::ReorderUsedExtensionsMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::unbind_default_scene::UnbindDefaultSceneMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::unbind_morph_target_attribute::UnbindMorphTargetAttributeMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::unbind_node_camera::UnbindNodeCameraMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::unbind_node_child::UnbindNodeChildMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::unbind_node_mesh::UnbindNodeMeshMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::unbind_node_skin::UnbindNodeSkinMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::unbind_primitive_attribute::UnbindPrimitiveAttributeMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::unbind_primitive_indices::UnbindPrimitiveIndicesMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::unbind_primitive_material::UnbindPrimitiveMaterialMutation;
pub use crate::standards::v2_0::subsets::any::schema::mutations::unbind_scene_root_node::UnbindSceneRootNodeMutation;

/// 🧬️ The complete glTF 2.0 semantic mutation vocabulary.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "mutation", content = "payload", rename_all = "camelCase")]
#[mutations(snapshot = GltfSnapshot, diff = GltfDiff, schema = "s.stdio.gltf")]
pub enum GltfMutation {
    BindDefaultScene(BindDefaultSceneMutation),
    BindMorphTargetAttribute(BindMorphTargetAttributeMutation),
    BindNodeCamera(BindNodeCameraMutation),
    BindNodeChild(BindNodeChildMutation),
    BindNodeMesh(BindNodeMeshMutation),
    BindNodeSkin(BindNodeSkinMutation),
    BindPrimitiveAttribute(BindPrimitiveAttributeMutation),
    BindPrimitiveIndices(BindPrimitiveIndicesMutation),
    BindPrimitiveMaterial(BindPrimitiveMaterialMutation),
    BindSceneRootNode(BindSceneRootNodeMutation),
    ChangeAssetDescriptiveMetadata(ChangeAssetDescriptiveMetadataMutation),
    ChangeAssetExtensionData(ChangeAssetExtensionDataMutation),
    ChangeAssetExtraData(ChangeAssetExtraDataMutation),
    ChangeAssetVersion(ChangeAssetVersionMutation),
    ChangeDocumentExtensionData(ChangeDocumentExtensionDataMutation),
    ChangeDocumentExtraData(ChangeDocumentExtraDataMutation),
    ChangeMaterialAlphaMode(ChangeMaterialAlphaModeMutation),
    ChangeMaterialDoubleSided(ChangeMaterialDoubleSidedMutation),
    ChangeMeshExtensionData(ChangeMeshExtensionDataMutation),
    ChangeMeshExtraData(ChangeMeshExtraDataMutation),
    ChangeMeshMorphWeights(ChangeMeshMorphWeightsMutation),
    ChangeMeshName(ChangeMeshNameMutation),
    ChangeNodeExtensionData(ChangeNodeExtensionDataMutation),
    ChangeNodeExtraData(ChangeNodeExtraDataMutation),
    ChangeNodeMorphWeights(ChangeNodeMorphWeightsMutation),
    ChangeNodeName(ChangeNodeNameMutation),
    ChangePrimitiveExtensionData(ChangePrimitiveExtensionDataMutation),
    ChangePrimitiveExtraData(ChangePrimitiveExtraDataMutation),
    ChangePrimitiveTopologyMode(ChangePrimitiveTopologyModeMutation),
    ChangeSceneExtensionData(ChangeSceneExtensionDataMutation),
    ChangeSceneExtraData(ChangeSceneExtraDataMutation),
    ChangeSceneName(ChangeSceneNameMutation),
    CreateAccessor(CreateAccessorMutation),
    CreateAnimation(CreateAnimationMutation),
    CreateBuffer(CreateBufferMutation),
    CreateBufferView(CreateBufferViewMutation),
    CreateCamera(CreateCameraMutation),
    CreateImage(CreateImageMutation),
    CreateMaterial(CreateMaterialMutation),
    CreateMesh(CreateMeshMutation),
    CreateMorphTarget(CreateMorphTargetMutation),
    CreateNode(CreateNodeMutation),
    CreatePrimitive(CreatePrimitiveMutation),
    CreateSampler(CreateSamplerMutation),
    CreateScene(CreateSceneMutation),
    CreateSkin(CreateSkinMutation),
    CreateTexture(CreateTextureMutation),
    AddUsedExtension(AddUsedExtensionMutation),
    DeleteAccessor(DeleteAccessorMutation),
    DeleteAnimation(DeleteAnimationMutation),
    DeleteBuffer(DeleteBufferMutation),
    DeleteBufferView(DeleteBufferViewMutation),
    DeleteCamera(DeleteCameraMutation),
    DeleteImage(DeleteImageMutation),
    DeleteMaterial(DeleteMaterialMutation),
    DeleteMesh(DeleteMeshMutation),
    DeleteMorphTarget(DeleteMorphTargetMutation),
    DeleteNode(DeleteNodeMutation),
    DeletePrimitive(DeletePrimitiveMutation),
    DeleteSampler(DeleteSamplerMutation),
    DeleteScene(DeleteSceneMutation),
    DeleteSkin(DeleteSkinMutation),
    DeleteTexture(DeleteTextureMutation),
    MoveAccessor(MoveAccessorMutation),
    MoveAnimation(MoveAnimationMutation),
    MoveBuffer(MoveBufferMutation),
    MoveBufferView(MoveBufferViewMutation),
    MoveCamera(MoveCameraMutation),
    MoveImage(MoveImageMutation),
    MoveMaterial(MoveMaterialMutation),
    MoveMesh(MoveMeshMutation),
    MoveMorphTarget(MoveMorphTargetMutation),
    MoveMorphTargetAttribute(MoveMorphTargetAttributeMutation),
    MoveNode(MoveNodeMutation),
    MoveNodeChild(MoveNodeChildMutation),
    MovePrimitive(MovePrimitiveMutation),
    MovePrimitiveAttribute(MovePrimitiveAttributeMutation),
    MoveRequiredExtension(MoveRequiredExtensionMutation),
    MoveSampler(MoveSamplerMutation),
    MoveScene(MoveSceneMutation),
    MoveSceneRootNode(MoveSceneRootNodeMutation),
    MoveSkin(MoveSkinMutation),
    MoveTexture(MoveTextureMutation),
    MoveUsedExtension(MoveUsedExtensionMutation),
    ReorderAccessors(ReorderAccessorsMutation),
    ReorderAnimations(ReorderAnimationsMutation),
    ReorderBufferViews(ReorderBufferViewsMutation),
    ReorderBuffers(ReorderBuffersMutation),
    ReorderCameras(ReorderCamerasMutation),
    ReorderImages(ReorderImagesMutation),
    ReorderMaterials(ReorderMaterialsMutation),
    ReorderMeshs(ReorderMeshsMutation),
    ReorderMorphTargetAttributes(ReorderMorphTargetAttributesMutation),
    ReorderMorphTargets(ReorderMorphTargetsMutation),
    ReorderNodeChildren(ReorderNodeChildrenMutation),
    ReorderNodes(ReorderNodesMutation),
    ReorderPrimitiveAttributes(ReorderPrimitiveAttributesMutation),
    ReorderPrimitives(ReorderPrimitivesMutation),
    ReorderRequiredExtensions(ReorderRequiredExtensionsMutation),
    ReorderSamplers(ReorderSamplersMutation),
    ReorderSceneRootNodes(ReorderSceneRootNodesMutation),
    ReorderScenes(ReorderScenesMutation),
    ReorderSkins(ReorderSkinsMutation),
    ReorderTextures(ReorderTexturesMutation),
    ReorderUsedExtensions(ReorderUsedExtensionsMutation),
    MoveNodeParent(MoveNodeParentMutation),
    AddRequiredExtension(AddRequiredExtensionMutation),
    ChangeNodeTransform(ChangeNodeTransformMutation),
    UnbindDefaultScene(UnbindDefaultSceneMutation),
    UnbindMorphTargetAttribute(UnbindMorphTargetAttributeMutation),
    UnbindNodeCamera(UnbindNodeCameraMutation),
    UnbindNodeChild(UnbindNodeChildMutation),
    UnbindNodeMesh(UnbindNodeMeshMutation),
    UnbindNodeSkin(UnbindNodeSkinMutation),
    UnbindPrimitiveAttribute(UnbindPrimitiveAttributeMutation),
    UnbindPrimitiveIndices(UnbindPrimitiveIndicesMutation),
    UnbindPrimitiveMaterial(UnbindPrimitiveMaterialMutation),
    UnbindSceneRootNode(UnbindSceneRootNodeMutation),
    RemoveRequiredExtension(RemoveRequiredExtensionMutation),
    RemoveUsedExtension(RemoveUsedExtensionMutation),
}

//#region 🌉️TestBridge


/// 🌉️ Reads a `.glb` container or `.gltf` JSON text through the production codec — whichever the bytes are.


/// 🌉️ Writes a snapshot back in the form it was read from, through the production codec.


/// 🌉️ One glTF document (`.glb` or `.gltf`) after the row's mutation, applied by the production codec and written back
/// in its own form — the subject half of a mutation case's forward rows. Its signature names only bytes and `str`, so
/// a generated test host reaches it.


/// 🌉️ One glTF document after the row's mutation and then its OWN computed inverse: every step of
/// `Mutation::inverse(base)` is replayed onto the mutated document in order, and the result is written back in the
/// document's own form. A mutation case's inverse rows hand exactly these bytes to their judge against the untouched
/// input, so a lossy production inverse cannot hide behind a hand-written one. Its signature names only bytes and
/// `str`, so a generated test host reaches it.

//#endregion 🌉️TestBridge

//#region 🧪️StructuralTests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#endregion 🧪️StructuralTests
