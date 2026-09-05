//! 🧬️ Transparent glTF mutation aggregate. Every concrete payload, outcome, diff, inverse, and test lives in its direct semantic leaf.

use crate::artifacts::gltf::schema::diff::GltfDiff;
use crate::artifacts::gltf::GltfSnapshot;

pub use super::bind_default_scene::BindDefaultSceneMutation;
pub use super::bind_morph_target_attribute::BindMorphTargetAttributeMutation;
pub use super::bind_node_camera::BindNodeCameraMutation;
pub use super::bind_node_child::BindNodeChildMutation;
pub use super::bind_node_mesh::BindNodeMeshMutation;
pub use super::bind_node_skin::BindNodeSkinMutation;
pub use super::bind_primitive_attribute::BindPrimitiveAttributeMutation;
pub use super::bind_primitive_indices::BindPrimitiveIndicesMutation;
pub use super::bind_primitive_material::BindPrimitiveMaterialMutation;
pub use super::bind_scene_root_node::BindSceneRootNodeMutation;
pub use super::change_asset_descriptive_metadata::ChangeAssetDescriptiveMetadataMutation;
pub use super::change_asset_extension_data::ChangeAssetExtensionDataMutation;
pub use super::change_asset_extra_data::ChangeAssetExtraDataMutation;
pub use super::change_asset_version::ChangeAssetVersionMutation;
pub use super::change_document_extension_data::ChangeDocumentExtensionDataMutation;
pub use super::change_document_extra_data::ChangeDocumentExtraDataMutation;
pub use super::change_material_alpha_mode::ChangeMaterialAlphaModeMutation;
pub use super::change_material_double_sided::ChangeMaterialDoubleSidedMutation;
pub use super::change_mesh_extension_data::ChangeMeshExtensionDataMutation;
pub use super::change_mesh_extra_data::ChangeMeshExtraDataMutation;
pub use super::change_mesh_morph_weights::ChangeMeshMorphWeightsMutation;
pub use super::change_mesh_name::ChangeMeshNameMutation;
pub use super::change_node_extension_data::ChangeNodeExtensionDataMutation;
pub use super::change_node_extra_data::ChangeNodeExtraDataMutation;
pub use super::change_node_morph_weights::ChangeNodeMorphWeightsMutation;
pub use super::change_node_name::ChangeNodeNameMutation;
pub use super::change_primitive_extension_data::ChangePrimitiveExtensionDataMutation;
pub use super::change_primitive_extra_data::ChangePrimitiveExtraDataMutation;
pub use super::change_primitive_topology_mode::ChangePrimitiveTopologyModeMutation;
pub use super::change_scene_extension_data::ChangeSceneExtensionDataMutation;
pub use super::change_scene_extra_data::ChangeSceneExtraDataMutation;
pub use super::change_scene_name::ChangeSceneNameMutation;
pub use super::create_accessor::CreateAccessorMutation;
pub use super::create_animation::CreateAnimationMutation;
pub use super::create_buffer::CreateBufferMutation;
pub use super::create_buffer_view::CreateBufferViewMutation;
pub use super::create_camera::CreateCameraMutation;
pub use super::create_image::CreateImageMutation;
pub use super::create_material::CreateMaterialMutation;
pub use super::create_mesh::CreateMeshMutation;
pub use super::create_morph_target::CreateMorphTargetMutation;
pub use super::create_node::CreateNodeMutation;
pub use super::create_primitive::CreatePrimitiveMutation;
pub use super::create_sampler::CreateSamplerMutation;
pub use super::create_scene::CreateSceneMutation;
pub use super::create_skin::CreateSkinMutation;
pub use super::create_texture::CreateTextureMutation;
pub use super::add_used_extension::AddUsedExtensionMutation;
pub use super::delete_accessor::DeleteAccessorMutation;
pub use super::delete_animation::DeleteAnimationMutation;
pub use super::delete_buffer::DeleteBufferMutation;
pub use super::delete_buffer_view::DeleteBufferViewMutation;
pub use super::delete_camera::DeleteCameraMutation;
pub use super::delete_image::DeleteImageMutation;
pub use super::delete_material::DeleteMaterialMutation;
pub use super::delete_mesh::DeleteMeshMutation;
pub use super::delete_morph_target::DeleteMorphTargetMutation;
pub use super::delete_node::DeleteNodeMutation;
pub use super::delete_primitive::DeletePrimitiveMutation;
pub use super::delete_sampler::DeleteSamplerMutation;
pub use super::delete_scene::DeleteSceneMutation;
pub use super::delete_skin::DeleteSkinMutation;
pub use super::delete_texture::DeleteTextureMutation;
pub use super::move_accessor::MoveAccessorMutation;
pub use super::move_animation::MoveAnimationMutation;
pub use super::move_buffer::MoveBufferMutation;
pub use super::move_buffer_view::MoveBufferViewMutation;
pub use super::move_camera::MoveCameraMutation;
pub use super::move_image::MoveImageMutation;
pub use super::move_material::MoveMaterialMutation;
pub use super::move_mesh::MoveMeshMutation;
pub use super::move_morph_target::MoveMorphTargetMutation;
pub use super::move_morph_target_attribute::MoveMorphTargetAttributeMutation;
pub use super::move_node::MoveNodeMutation;
pub use super::move_node_child::MoveNodeChildMutation;
pub use super::move_primitive::MovePrimitiveMutation;
pub use super::move_primitive_attribute::MovePrimitiveAttributeMutation;
pub use super::move_required_extension::MoveRequiredExtensionMutation;
pub use super::move_sampler::MoveSamplerMutation;
pub use super::move_scene::MoveSceneMutation;
pub use super::move_scene_root_node::MoveSceneRootNodeMutation;
pub use super::move_skin::MoveSkinMutation;
pub use super::move_texture::MoveTextureMutation;
pub use super::move_used_extension::MoveUsedExtensionMutation;
pub use super::reorder_accessors::ReorderAccessorsMutation;
pub use super::reorder_animations::ReorderAnimationsMutation;
pub use super::reorder_buffer_views::ReorderBufferViewsMutation;
pub use super::reorder_buffers::ReorderBuffersMutation;
pub use super::reorder_cameras::ReorderCamerasMutation;
pub use super::reorder_images::ReorderImagesMutation;
pub use super::reorder_materials::ReorderMaterialsMutation;
pub use super::reorder_meshs::ReorderMeshsMutation;
pub use super::reorder_morph_target_attributes::ReorderMorphTargetAttributesMutation;
pub use super::reorder_morph_targets::ReorderMorphTargetsMutation;
pub use super::reorder_node_children::ReorderNodeChildrenMutation;
pub use super::reorder_nodes::ReorderNodesMutation;
pub use super::reorder_primitive_attributes::ReorderPrimitiveAttributesMutation;
pub use super::reorder_primitives::ReorderPrimitivesMutation;
pub use super::reorder_required_extensions::ReorderRequiredExtensionsMutation;
pub use super::reorder_samplers::ReorderSamplersMutation;
pub use super::reorder_scene_root_nodes::ReorderSceneRootNodesMutation;
pub use super::reorder_scenes::ReorderScenesMutation;
pub use super::reorder_skins::ReorderSkinsMutation;
pub use super::reorder_textures::ReorderTexturesMutation;
pub use super::reorder_used_extensions::ReorderUsedExtensionsMutation;
pub use super::move_node_parent::MoveNodeParentMutation;
pub use super::add_required_extension::AddRequiredExtensionMutation;
pub use super::change_node_transform::ChangeNodeTransformMutation;
pub use super::unbind_default_scene::UnbindDefaultSceneMutation;
pub use super::unbind_morph_target_attribute::UnbindMorphTargetAttributeMutation;
pub use super::unbind_node_camera::UnbindNodeCameraMutation;
pub use super::unbind_node_child::UnbindNodeChildMutation;
pub use super::unbind_node_mesh::UnbindNodeMeshMutation;
pub use super::unbind_node_skin::UnbindNodeSkinMutation;
pub use super::unbind_primitive_attribute::UnbindPrimitiveAttributeMutation;
pub use super::unbind_primitive_indices::UnbindPrimitiveIndicesMutation;
pub use super::unbind_primitive_material::UnbindPrimitiveMaterialMutation;
pub use super::unbind_scene_root_node::UnbindSceneRootNodeMutation;
pub use super::remove_required_extension::RemoveRequiredExtensionMutation;
pub use super::remove_used_extension::RemoveUsedExtensionMutation;

/// 🧬️ The complete glTF 2.0 semantic mutation vocabulary.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
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

pub fn apply_gltf_mutation(snapshot: &mut GltfSnapshot, mutation: &GltfMutation) -> protocol::MutationOutcome<GltfDiff> {
    let outcome = <GltfMutation as protocol::Mutation<GltfSnapshot>>::diff(mutation, snapshot);
    if let Ok(next) = protocol::MutationDiff::apply(outcome.diff(), snapshot) { *snapshot = next; }
    outcome
}

//#region 🧪️StructuralTests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::SemanticMutation;

    #[test]
    fn aggregate_descriptor_roster_is_exactly_the_direct_leaf_roster() {
        assert_eq!(GltfMutation::kinds().len(), 120);
        assert_eq!(GltfMutation::kinds().iter().map(|descriptor| descriptor.kind).collect::<std::collections::BTreeSet<_>>().len(), 120);
    }
}
//#endregion 🧪️StructuralTests
