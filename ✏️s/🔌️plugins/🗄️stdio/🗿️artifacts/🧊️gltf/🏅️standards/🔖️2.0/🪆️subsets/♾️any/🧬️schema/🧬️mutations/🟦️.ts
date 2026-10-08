/** 🧬️ `GltfMutation` twin: the adjacently tagged (`mutation`/`payload`) aggregate over every glTF 2.0 leaf, branch for branch as
 * `./🔣️.json` and `./🦀️.rs` spell it; a wrapped leaf's payload is its whole phase wire.
 * @see ./🔣️.json */
import { gltfWireLiteral, gltfWireObject, gltfWireRequired, type GltfWireReader } from "../📸️snapshot/🟦️.ts";
import { parseBindDefaultSceneMutation, type BindDefaultSceneMutation } from "./🏠️default-scene/🔗️bind/🟦️.ts";
import { parseBindMorphTargetAttributeMutation, type BindMorphTargetAttributeMutation } from "./🎚️morph/🔗️bind/🟦️.ts";
import { parseBindNodeCameraMutation, type BindNodeCameraMutation } from "./📷️node-camera/🔗️bind/🟦️.ts";
import { parseBindNodeChildMutation, type BindNodeChildMutation } from "./🌿️node-child/🔗️bind/🟦️.ts";
import { parseBindNodeMeshMutation, type BindNodeMeshMutation } from "./🏗️node-mesh/🔗️bind/🟦️.ts";
import { parseBindNodeSkinMutation, type BindNodeSkinMutation } from "./🩻️node-skin/🔗️bind/🟦️.ts";
import { parseBindPrimitiveAttributeMutation, type BindPrimitiveAttributeMutation } from "./🔤️primitive/🔗️bind/🟦️.ts";
import { parseBindPrimitiveIndicesMutation, type BindPrimitiveIndicesMutation } from "./🔢️primitive/🔗️bind/🟦️.ts";
import { parseBindPrimitiveMaterialMutation, type BindPrimitiveMaterialMutation } from "./🧱️primitive/🔗️bind/🟦️.ts";
import { parseBindSceneRootNodeMutation, type BindSceneRootNodeMutation } from "./🌲️scene-root/🔗️bind/🟦️.ts";
import { parseChangeAssetDescriptiveMetadataMutation, type ChangeAssetDescriptiveMetadataMutation } from "./🪪️asset/📝️change-description/🟦️.ts";
import { parseChangeAssetExtensionDataMutation, type ChangeAssetExtensionDataMutation } from "./🪪️asset/🧩️change-extensions/🟦️.ts";
import { parseChangeAssetExtraDataMutation, type ChangeAssetExtraDataMutation } from "./🪪️asset/🧾️change-extras/🟦️.ts";
import { parseChangeAssetVersionMutation, type ChangeAssetVersionMutation } from "./🪪️asset/🔖️version/🟦️.ts";
import { parseChangeDocumentExtensionDataMutation, type ChangeDocumentExtensionDataMutation } from "./📃️document/🧩️change-extensions/🟦️.ts";
import { parseChangeDocumentExtraDataMutation, type ChangeDocumentExtraDataMutation } from "./📃️document/📝️change-extras/🟦️.ts";
import { parseChangeMaterialAlphaModeMutation, type ChangeMaterialAlphaModeMutation } from "./💎️material/🌫️change-alpha/🟦️.ts";
import { parseChangeMaterialDoubleSidedMutation, type ChangeMaterialDoubleSidedMutation } from "./💎️material/🪞️change-sides/🟦️.ts";
import { parseChangeMeshExtensionDataMutation, type ChangeMeshExtensionDataMutation } from "./🕸️mesh/🧩️change-extensions/🟦️.ts";
import { parseChangeMeshExtraDataMutation, type ChangeMeshExtraDataMutation } from "./🕸️mesh/📝️change-extras/🟦️.ts";
import { parseChangeMeshMorphWeightsMutation, type ChangeMeshMorphWeightsMutation } from "./🕸️mesh/⚖️change-weights/🟦️.ts";
import { parseChangeMeshNameMutation, type ChangeMeshNameMutation } from "./🕸️mesh/🏷️rename/🟦️.ts";
import { parseChangeNodeExtensionDataMutation, type ChangeNodeExtensionDataMutation } from "./🌳️node/🧩️change-extensions/🟦️.ts";
import { parseChangeNodeExtraDataMutation, type ChangeNodeExtraDataMutation } from "./🌳️node/📝️change-extras/🟦️.ts";
import { parseChangeNodeMorphWeightsMutation, type ChangeNodeMorphWeightsMutation } from "./🌳️node/⚖️change-weights/🟦️.ts";
import { parseChangeNodeNameMutation, type ChangeNodeNameMutation } from "./🌳️node/🏷️rename/🟦️.ts";
import { parseChangePrimitiveExtensionDataMutation, type ChangePrimitiveExtensionDataMutation } from "./🔺️primitive/🧩️change-extensions/🟦️.ts";
import { parseChangePrimitiveExtraDataMutation, type ChangePrimitiveExtraDataMutation } from "./🔺️primitive/📝️change-extras/🟦️.ts";
import { parseChangePrimitiveTopologyModeMutation, type ChangePrimitiveTopologyModeMutation } from "./🔺️primitive/📐️change-topology/🟦️.ts";
import { parseChangeSceneExtensionDataMutation, type ChangeSceneExtensionDataMutation } from "./🎬️scene/🧩️change-extensions/🟦️.ts";
import { parseChangeSceneExtraDataMutation, type ChangeSceneExtraDataMutation } from "./🎬️scene/📝️change-extras/🟦️.ts";
import { parseChangeSceneNameMutation, type ChangeSceneNameMutation } from "./🎬️scene/🏷️rename/🟦️.ts";
import { parseCreateAccessorMutation, type CreateAccessorMutation } from "./📐️accessor/🌱️create/🟦️.ts";
import { parseCreateAnimationMutation, type CreateAnimationMutation } from "./🎞️animation/🌱️create/🟦️.ts";
import { parseCreateBufferMutation, type CreateBufferMutation } from "./💿️buffer/🌱️create/🟦️.ts";
import { parseCreateBufferViewMutation, type CreateBufferViewMutation } from "./🪟️buffer-view/🌱️create/🟦️.ts";
import { parseCreateCameraMutation, type CreateCameraMutation } from "./🎥️camera/🌱️create/🟦️.ts";
import { parseCreateImageMutation, type CreateImageMutation } from "./🖼️image/🌱️create/🟦️.ts";
import { parseCreateMaterialMutation, type CreateMaterialMutation } from "./💎️material/🌱️create/🟦️.ts";
import { parseCreateMeshMutation, type CreateMeshMutation } from "./🕸️mesh/🌱️create/🟦️.ts";
import { parseCreateMorphTargetMutation, type CreateMorphTargetMutation } from "./🧬️morph/🌱️create/🟦️.ts";
import { parseCreateNodeMutation, type CreateNodeMutation } from "./🌳️node/🌱️create/🟦️.ts";
import { parseCreatePrimitiveMutation, type CreatePrimitiveMutation } from "./🔺️primitive/🌱️create/🟦️.ts";
import { parseCreateSamplerMutation, type CreateSamplerMutation } from "./🎛️sampler/🌱️create/🟦️.ts";
import { parseCreateSceneMutation, type CreateSceneMutation } from "./🎬️scene/🌱️create/🟦️.ts";
import { parseCreateSkinMutation, type CreateSkinMutation } from "./🦴️skin/🌱️create/🟦️.ts";
import { parseCreateTextureMutation, type CreateTextureMutation } from "./🎨️texture/🌱️create/🟦️.ts";
import { parseAddUsedExtensionMutation, type AddUsedExtensionMutation } from "./📣️used/➕️add/🟦️.ts";
import { parseDeleteAccessorMutation, type DeleteAccessorMutation } from "./📐️accessor/🗑️delete/🟦️.ts";
import { parseDeleteAnimationMutation, type DeleteAnimationMutation } from "./🎞️animation/🗑️delete/🟦️.ts";
import { parseDeleteBufferMutation, type DeleteBufferMutation } from "./💿️buffer/🗑️delete/🟦️.ts";
import { parseDeleteBufferViewMutation, type DeleteBufferViewMutation } from "./🪟️buffer-view/🗑️delete/🟦️.ts";
import { parseDeleteCameraMutation, type DeleteCameraMutation } from "./🎥️camera/🗑️delete/🟦️.ts";
import { parseDeleteImageMutation, type DeleteImageMutation } from "./🖼️image/🗑️delete/🟦️.ts";
import { parseDeleteMaterialMutation, type DeleteMaterialMutation } from "./💎️material/🗑️delete/🟦️.ts";
import { parseDeleteMeshMutation, type DeleteMeshMutation } from "./🕸️mesh/🗑️delete/🟦️.ts";
import { parseDeleteMorphTargetMutation, type DeleteMorphTargetMutation } from "./🧬️morph/🗑️delete/🟦️.ts";
import { parseDeleteNodeMutation, type DeleteNodeMutation } from "./🌳️node/🗑️delete/🟦️.ts";
import { parseDeletePrimitiveMutation, type DeletePrimitiveMutation } from "./🔺️primitive/🗑️delete/🟦️.ts";
import { parseDeleteSamplerMutation, type DeleteSamplerMutation } from "./🎛️sampler/🗑️delete/🟦️.ts";
import { parseDeleteSceneMutation, type DeleteSceneMutation } from "./🎬️scene/🗑️delete/🟦️.ts";
import { parseDeleteSkinMutation, type DeleteSkinMutation } from "./🦴️skin/🗑️delete/🟦️.ts";
import { parseDeleteTextureMutation, type DeleteTextureMutation } from "./🎨️texture/🗑️delete/🟦️.ts";
import { parseMoveAccessorMutation, type MoveAccessorMutation } from "./📐️accessor/🚚️move/🟦️.ts";
import { parseMoveAnimationMutation, type MoveAnimationMutation } from "./🎞️animation/🚚️move/🟦️.ts";
import { parseMoveBufferMutation, type MoveBufferMutation } from "./💿️buffer/🚚️move/🟦️.ts";
import { parseMoveBufferViewMutation, type MoveBufferViewMutation } from "./🪟️buffer-view/🚚️move/🟦️.ts";
import { parseMoveCameraMutation, type MoveCameraMutation } from "./🎥️camera/🚚️move/🟦️.ts";
import { parseMoveImageMutation, type MoveImageMutation } from "./🖼️image/🚚️move/🟦️.ts";
import { parseMoveMaterialMutation, type MoveMaterialMutation } from "./💎️material/🚚️move/🟦️.ts";
import { parseMoveMeshMutation, type MoveMeshMutation } from "./🕸️mesh/🚚️move/🟦️.ts";
import { parseMoveMorphTargetMutation, type MoveMorphTargetMutation } from "./🧬️morph/🚚️move/🟦️.ts";
import { parseMoveMorphTargetAttributeMutation, type MoveMorphTargetAttributeMutation } from "./🎚️morph/🚚️move/🟦️.ts";
import { parseMoveNodeMutation, type MoveNodeMutation } from "./🌳️node/🚚️move/🟦️.ts";
import { parseMoveNodeChildMutation, type MoveNodeChildMutation } from "./🌿️node-child/🚚️move/🟦️.ts";
import { parseMovePrimitiveMutation, type MovePrimitiveMutation } from "./🔺️primitive/🚚️move/🟦️.ts";
import { parseMovePrimitiveAttributeMutation, type MovePrimitiveAttributeMutation } from "./🔤️primitive/🚚️move/🟦️.ts";
import { parseMoveRequiredExtensionMutation, type MoveRequiredExtensionMutation } from "./✅️required/🚚️move/🟦️.ts";
import { parseMoveSamplerMutation, type MoveSamplerMutation } from "./🎛️sampler/🚚️move/🟦️.ts";
import { parseMoveSceneMutation, type MoveSceneMutation } from "./🎬️scene/🚚️move/🟦️.ts";
import { parseMoveSceneRootNodeMutation, type MoveSceneRootNodeMutation } from "./🌲️scene-root/🚚️move/🟦️.ts";
import { parseMoveSkinMutation, type MoveSkinMutation } from "./🦴️skin/🚚️move/🟦️.ts";
import { parseMoveTextureMutation, type MoveTextureMutation } from "./🎨️texture/🚚️move/🟦️.ts";
import { parseMoveUsedExtensionMutation, type MoveUsedExtensionMutation } from "./📣️used/🚚️move/🟦️.ts";
import { parseReorderAccessorsMutation, type ReorderAccessorsMutation } from "./📐️accessor/🔀️reorder/🟦️.ts";
import { parseReorderAnimationsMutation, type ReorderAnimationsMutation } from "./🎞️animation/🔀️reorder/🟦️.ts";
import { parseReorderBufferViewsMutation, type ReorderBufferViewsMutation } from "./🪟️buffer-view/🔀️reorder/🟦️.ts";
import { parseReorderBuffersMutation, type ReorderBuffersMutation } from "./💿️buffer/🔀️reorder/🟦️.ts";
import { parseReorderCamerasMutation, type ReorderCamerasMutation } from "./🎥️camera/🔀️reorder/🟦️.ts";
import { parseReorderImagesMutation, type ReorderImagesMutation } from "./🖼️image/🔀️reorder/🟦️.ts";
import { parseReorderMaterialsMutation, type ReorderMaterialsMutation } from "./💎️material/🔀️reorder/🟦️.ts";
import { parseReorderMeshsMutation, type ReorderMeshsMutation } from "./🕸️mesh/🔀️reorder/🟦️.ts";
import { parseReorderMorphTargetAttributesMutation, type ReorderMorphTargetAttributesMutation } from "./🎚️morph/🔀️reorder/🟦️.ts";
import { parseReorderMorphTargetsMutation, type ReorderMorphTargetsMutation } from "./🧬️morph/🔀️reorder/🟦️.ts";
import { parseReorderNodeChildrenMutation, type ReorderNodeChildrenMutation } from "./🌿️node-child/🔀️reorder/🟦️.ts";
import { parseReorderNodesMutation, type ReorderNodesMutation } from "./🌳️node/🔀️reorder/🟦️.ts";
import { parseReorderPrimitiveAttributesMutation, type ReorderPrimitiveAttributesMutation } from "./🔤️primitive/🔀️reorder/🟦️.ts";
import { parseReorderPrimitivesMutation, type ReorderPrimitivesMutation } from "./🔺️primitive/🔀️reorder/🟦️.ts";
import { parseReorderRequiredExtensionsMutation, type ReorderRequiredExtensionsMutation } from "./✅️required/🔀️reorder/🟦️.ts";
import { parseReorderSamplersMutation, type ReorderSamplersMutation } from "./🎛️sampler/🔀️reorder/🟦️.ts";
import { parseReorderSceneRootNodesMutation, type ReorderSceneRootNodesMutation } from "./🌲️scene-root/🔀️reorder/🟦️.ts";
import { parseReorderScenesMutation, type ReorderScenesMutation } from "./🎬️scene/🔀️reorder/🟦️.ts";
import { parseReorderSkinsMutation, type ReorderSkinsMutation } from "./🦴️skin/🔀️reorder/🟦️.ts";
import { parseReorderTexturesMutation, type ReorderTexturesMutation } from "./🎨️texture/🔀️reorder/🟦️.ts";
import { parseReorderUsedExtensionsMutation, type ReorderUsedExtensionsMutation } from "./📣️used/🔀️reorder/🟦️.ts";
import { parseMoveNodeParentMutation, type MoveNodeParentMutation } from "./🌳️node/🌿️reparent/🟦️.ts";
import { parseAddRequiredExtensionMutation, type AddRequiredExtensionMutation } from "./✅️required/➕️add/🟦️.ts";
import { parseChangeNodeTransformMutation, type ChangeNodeTransformMutation } from "./🌳️node/📐️transform/🟦️.ts";
import { parseUnbindDefaultSceneMutation, type UnbindDefaultSceneMutation } from "./🏠️default-scene/✂️unbind/🟦️.ts";
import { parseUnbindMorphTargetAttributeMutation, type UnbindMorphTargetAttributeMutation } from "./🎚️morph/✂️unbind/🟦️.ts";
import { parseUnbindNodeCameraMutation, type UnbindNodeCameraMutation } from "./📷️node-camera/✂️unbind/🟦️.ts";
import { parseUnbindNodeChildMutation, type UnbindNodeChildMutation } from "./🌿️node-child/✂️unbind/🟦️.ts";
import { parseUnbindNodeMeshMutation, type UnbindNodeMeshMutation } from "./🏗️node-mesh/✂️unbind/🟦️.ts";
import { parseUnbindNodeSkinMutation, type UnbindNodeSkinMutation } from "./🩻️node-skin/✂️unbind/🟦️.ts";
import { parseUnbindPrimitiveAttributeMutation, type UnbindPrimitiveAttributeMutation } from "./🔤️primitive/✂️unbind/🟦️.ts";
import { parseUnbindPrimitiveIndicesMutation, type UnbindPrimitiveIndicesMutation } from "./🔢️primitive/✂️unbind/🟦️.ts";
import { parseUnbindPrimitiveMaterialMutation, type UnbindPrimitiveMaterialMutation } from "./🧱️primitive/✂️unbind/🟦️.ts";
import { parseUnbindSceneRootNodeMutation, type UnbindSceneRootNodeMutation } from "./🌲️scene-root/✂️unbind/🟦️.ts";
import { parseRemoveRequiredExtensionMutation, type RemoveRequiredExtensionMutation } from "./✅️required/➖️remove/🟦️.ts";
import { parseRemoveUsedExtensionMutation, type RemoveUsedExtensionMutation } from "./📣️used/➖️remove/🟦️.ts";

export type GltfMutation =
  | { readonly mutation: "bindDefaultScene"; readonly payload: BindDefaultSceneMutation }
  | { readonly mutation: "bindMorphTargetAttribute"; readonly payload: BindMorphTargetAttributeMutation }
  | { readonly mutation: "bindNodeCamera"; readonly payload: BindNodeCameraMutation }
  | { readonly mutation: "bindNodeChild"; readonly payload: BindNodeChildMutation }
  | { readonly mutation: "bindNodeMesh"; readonly payload: BindNodeMeshMutation }
  | { readonly mutation: "bindNodeSkin"; readonly payload: BindNodeSkinMutation }
  | { readonly mutation: "bindPrimitiveAttribute"; readonly payload: BindPrimitiveAttributeMutation }
  | { readonly mutation: "bindPrimitiveIndices"; readonly payload: BindPrimitiveIndicesMutation }
  | { readonly mutation: "bindPrimitiveMaterial"; readonly payload: BindPrimitiveMaterialMutation }
  | { readonly mutation: "bindSceneRootNode"; readonly payload: BindSceneRootNodeMutation }
  | { readonly mutation: "changeAssetDescriptiveMetadata"; readonly payload: ChangeAssetDescriptiveMetadataMutation }
  | { readonly mutation: "changeAssetExtensionData"; readonly payload: ChangeAssetExtensionDataMutation }
  | { readonly mutation: "changeAssetExtraData"; readonly payload: ChangeAssetExtraDataMutation }
  | { readonly mutation: "changeAssetVersion"; readonly payload: ChangeAssetVersionMutation }
  | { readonly mutation: "changeDocumentExtensionData"; readonly payload: ChangeDocumentExtensionDataMutation }
  | { readonly mutation: "changeDocumentExtraData"; readonly payload: ChangeDocumentExtraDataMutation }
  | { readonly mutation: "changeMaterialAlphaMode"; readonly payload: ChangeMaterialAlphaModeMutation }
  | { readonly mutation: "changeMaterialDoubleSided"; readonly payload: ChangeMaterialDoubleSidedMutation }
  | { readonly mutation: "changeMeshExtensionData"; readonly payload: ChangeMeshExtensionDataMutation }
  | { readonly mutation: "changeMeshExtraData"; readonly payload: ChangeMeshExtraDataMutation }
  | { readonly mutation: "changeMeshMorphWeights"; readonly payload: ChangeMeshMorphWeightsMutation }
  | { readonly mutation: "changeMeshName"; readonly payload: ChangeMeshNameMutation }
  | { readonly mutation: "changeNodeExtensionData"; readonly payload: ChangeNodeExtensionDataMutation }
  | { readonly mutation: "changeNodeExtraData"; readonly payload: ChangeNodeExtraDataMutation }
  | { readonly mutation: "changeNodeMorphWeights"; readonly payload: ChangeNodeMorphWeightsMutation }
  | { readonly mutation: "changeNodeName"; readonly payload: ChangeNodeNameMutation }
  | { readonly mutation: "changePrimitiveExtensionData"; readonly payload: ChangePrimitiveExtensionDataMutation }
  | { readonly mutation: "changePrimitiveExtraData"; readonly payload: ChangePrimitiveExtraDataMutation }
  | { readonly mutation: "changePrimitiveTopologyMode"; readonly payload: ChangePrimitiveTopologyModeMutation }
  | { readonly mutation: "changeSceneExtensionData"; readonly payload: ChangeSceneExtensionDataMutation }
  | { readonly mutation: "changeSceneExtraData"; readonly payload: ChangeSceneExtraDataMutation }
  | { readonly mutation: "changeSceneName"; readonly payload: ChangeSceneNameMutation }
  | { readonly mutation: "createAccessor"; readonly payload: CreateAccessorMutation }
  | { readonly mutation: "createAnimation"; readonly payload: CreateAnimationMutation }
  | { readonly mutation: "createBuffer"; readonly payload: CreateBufferMutation }
  | { readonly mutation: "createBufferView"; readonly payload: CreateBufferViewMutation }
  | { readonly mutation: "createCamera"; readonly payload: CreateCameraMutation }
  | { readonly mutation: "createImage"; readonly payload: CreateImageMutation }
  | { readonly mutation: "createMaterial"; readonly payload: CreateMaterialMutation }
  | { readonly mutation: "createMesh"; readonly payload: CreateMeshMutation }
  | { readonly mutation: "createMorphTarget"; readonly payload: CreateMorphTargetMutation }
  | { readonly mutation: "createNode"; readonly payload: CreateNodeMutation }
  | { readonly mutation: "createPrimitive"; readonly payload: CreatePrimitiveMutation }
  | { readonly mutation: "createSampler"; readonly payload: CreateSamplerMutation }
  | { readonly mutation: "createScene"; readonly payload: CreateSceneMutation }
  | { readonly mutation: "createSkin"; readonly payload: CreateSkinMutation }
  | { readonly mutation: "createTexture"; readonly payload: CreateTextureMutation }
  | { readonly mutation: "addUsedExtension"; readonly payload: AddUsedExtensionMutation }
  | { readonly mutation: "deleteAccessor"; readonly payload: DeleteAccessorMutation }
  | { readonly mutation: "deleteAnimation"; readonly payload: DeleteAnimationMutation }
  | { readonly mutation: "deleteBuffer"; readonly payload: DeleteBufferMutation }
  | { readonly mutation: "deleteBufferView"; readonly payload: DeleteBufferViewMutation }
  | { readonly mutation: "deleteCamera"; readonly payload: DeleteCameraMutation }
  | { readonly mutation: "deleteImage"; readonly payload: DeleteImageMutation }
  | { readonly mutation: "deleteMaterial"; readonly payload: DeleteMaterialMutation }
  | { readonly mutation: "deleteMesh"; readonly payload: DeleteMeshMutation }
  | { readonly mutation: "deleteMorphTarget"; readonly payload: DeleteMorphTargetMutation }
  | { readonly mutation: "deleteNode"; readonly payload: DeleteNodeMutation }
  | { readonly mutation: "deletePrimitive"; readonly payload: DeletePrimitiveMutation }
  | { readonly mutation: "deleteSampler"; readonly payload: DeleteSamplerMutation }
  | { readonly mutation: "deleteScene"; readonly payload: DeleteSceneMutation }
  | { readonly mutation: "deleteSkin"; readonly payload: DeleteSkinMutation }
  | { readonly mutation: "deleteTexture"; readonly payload: DeleteTextureMutation }
  | { readonly mutation: "moveAccessor"; readonly payload: MoveAccessorMutation }
  | { readonly mutation: "moveAnimation"; readonly payload: MoveAnimationMutation }
  | { readonly mutation: "moveBuffer"; readonly payload: MoveBufferMutation }
  | { readonly mutation: "moveBufferView"; readonly payload: MoveBufferViewMutation }
  | { readonly mutation: "moveCamera"; readonly payload: MoveCameraMutation }
  | { readonly mutation: "moveImage"; readonly payload: MoveImageMutation }
  | { readonly mutation: "moveMaterial"; readonly payload: MoveMaterialMutation }
  | { readonly mutation: "moveMesh"; readonly payload: MoveMeshMutation }
  | { readonly mutation: "moveMorphTarget"; readonly payload: MoveMorphTargetMutation }
  | { readonly mutation: "moveMorphTargetAttribute"; readonly payload: MoveMorphTargetAttributeMutation }
  | { readonly mutation: "moveNode"; readonly payload: MoveNodeMutation }
  | { readonly mutation: "moveNodeChild"; readonly payload: MoveNodeChildMutation }
  | { readonly mutation: "movePrimitive"; readonly payload: MovePrimitiveMutation }
  | { readonly mutation: "movePrimitiveAttribute"; readonly payload: MovePrimitiveAttributeMutation }
  | { readonly mutation: "moveRequiredExtension"; readonly payload: MoveRequiredExtensionMutation }
  | { readonly mutation: "moveSampler"; readonly payload: MoveSamplerMutation }
  | { readonly mutation: "moveScene"; readonly payload: MoveSceneMutation }
  | { readonly mutation: "moveSceneRootNode"; readonly payload: MoveSceneRootNodeMutation }
  | { readonly mutation: "moveSkin"; readonly payload: MoveSkinMutation }
  | { readonly mutation: "moveTexture"; readonly payload: MoveTextureMutation }
  | { readonly mutation: "moveUsedExtension"; readonly payload: MoveUsedExtensionMutation }
  | { readonly mutation: "reorderAccessors"; readonly payload: ReorderAccessorsMutation }
  | { readonly mutation: "reorderAnimations"; readonly payload: ReorderAnimationsMutation }
  | { readonly mutation: "reorderBufferViews"; readonly payload: ReorderBufferViewsMutation }
  | { readonly mutation: "reorderBuffers"; readonly payload: ReorderBuffersMutation }
  | { readonly mutation: "reorderCameras"; readonly payload: ReorderCamerasMutation }
  | { readonly mutation: "reorderImages"; readonly payload: ReorderImagesMutation }
  | { readonly mutation: "reorderMaterials"; readonly payload: ReorderMaterialsMutation }
  | { readonly mutation: "reorderMeshs"; readonly payload: ReorderMeshsMutation }
  | { readonly mutation: "reorderMorphTargetAttributes"; readonly payload: ReorderMorphTargetAttributesMutation }
  | { readonly mutation: "reorderMorphTargets"; readonly payload: ReorderMorphTargetsMutation }
  | { readonly mutation: "reorderNodeChildren"; readonly payload: ReorderNodeChildrenMutation }
  | { readonly mutation: "reorderNodes"; readonly payload: ReorderNodesMutation }
  | { readonly mutation: "reorderPrimitiveAttributes"; readonly payload: ReorderPrimitiveAttributesMutation }
  | { readonly mutation: "reorderPrimitives"; readonly payload: ReorderPrimitivesMutation }
  | { readonly mutation: "reorderRequiredExtensions"; readonly payload: ReorderRequiredExtensionsMutation }
  | { readonly mutation: "reorderSamplers"; readonly payload: ReorderSamplersMutation }
  | { readonly mutation: "reorderSceneRootNodes"; readonly payload: ReorderSceneRootNodesMutation }
  | { readonly mutation: "reorderScenes"; readonly payload: ReorderScenesMutation }
  | { readonly mutation: "reorderSkins"; readonly payload: ReorderSkinsMutation }
  | { readonly mutation: "reorderTextures"; readonly payload: ReorderTexturesMutation }
  | { readonly mutation: "reorderUsedExtensions"; readonly payload: ReorderUsedExtensionsMutation }
  | { readonly mutation: "moveNodeParent"; readonly payload: MoveNodeParentMutation }
  | { readonly mutation: "addRequiredExtension"; readonly payload: AddRequiredExtensionMutation }
  | { readonly mutation: "changeNodeTransform"; readonly payload: ChangeNodeTransformMutation }
  | { readonly mutation: "unbindDefaultScene"; readonly payload: UnbindDefaultSceneMutation }
  | { readonly mutation: "unbindMorphTargetAttribute"; readonly payload: UnbindMorphTargetAttributeMutation }
  | { readonly mutation: "unbindNodeCamera"; readonly payload: UnbindNodeCameraMutation }
  | { readonly mutation: "unbindNodeChild"; readonly payload: UnbindNodeChildMutation }
  | { readonly mutation: "unbindNodeMesh"; readonly payload: UnbindNodeMeshMutation }
  | { readonly mutation: "unbindNodeSkin"; readonly payload: UnbindNodeSkinMutation }
  | { readonly mutation: "unbindPrimitiveAttribute"; readonly payload: UnbindPrimitiveAttributeMutation }
  | { readonly mutation: "unbindPrimitiveIndices"; readonly payload: UnbindPrimitiveIndicesMutation }
  | { readonly mutation: "unbindPrimitiveMaterial"; readonly payload: UnbindPrimitiveMaterialMutation }
  | { readonly mutation: "unbindSceneRootNode"; readonly payload: UnbindSceneRootNodeMutation }
  | { readonly mutation: "removeRequiredExtension"; readonly payload: RemoveRequiredExtensionMutation }
  | { readonly mutation: "removeUsedExtension"; readonly payload: RemoveUsedExtensionMutation };

const payloads: { readonly [K in GltfMutation["mutation"]]: GltfWireReader<Extract<GltfMutation, { readonly mutation: K }>["payload"]> } = {
  bindDefaultScene: parseBindDefaultSceneMutation,
  bindMorphTargetAttribute: parseBindMorphTargetAttributeMutation,
  bindNodeCamera: parseBindNodeCameraMutation,
  bindNodeChild: parseBindNodeChildMutation,
  bindNodeMesh: parseBindNodeMeshMutation,
  bindNodeSkin: parseBindNodeSkinMutation,
  bindPrimitiveAttribute: parseBindPrimitiveAttributeMutation,
  bindPrimitiveIndices: parseBindPrimitiveIndicesMutation,
  bindPrimitiveMaterial: parseBindPrimitiveMaterialMutation,
  bindSceneRootNode: parseBindSceneRootNodeMutation,
  changeAssetDescriptiveMetadata: parseChangeAssetDescriptiveMetadataMutation,
  changeAssetExtensionData: parseChangeAssetExtensionDataMutation,
  changeAssetExtraData: parseChangeAssetExtraDataMutation,
  changeAssetVersion: parseChangeAssetVersionMutation,
  changeDocumentExtensionData: parseChangeDocumentExtensionDataMutation,
  changeDocumentExtraData: parseChangeDocumentExtraDataMutation,
  changeMaterialAlphaMode: parseChangeMaterialAlphaModeMutation,
  changeMaterialDoubleSided: parseChangeMaterialDoubleSidedMutation,
  changeMeshExtensionData: parseChangeMeshExtensionDataMutation,
  changeMeshExtraData: parseChangeMeshExtraDataMutation,
  changeMeshMorphWeights: parseChangeMeshMorphWeightsMutation,
  changeMeshName: parseChangeMeshNameMutation,
  changeNodeExtensionData: parseChangeNodeExtensionDataMutation,
  changeNodeExtraData: parseChangeNodeExtraDataMutation,
  changeNodeMorphWeights: parseChangeNodeMorphWeightsMutation,
  changeNodeName: parseChangeNodeNameMutation,
  changePrimitiveExtensionData: parseChangePrimitiveExtensionDataMutation,
  changePrimitiveExtraData: parseChangePrimitiveExtraDataMutation,
  changePrimitiveTopologyMode: parseChangePrimitiveTopologyModeMutation,
  changeSceneExtensionData: parseChangeSceneExtensionDataMutation,
  changeSceneExtraData: parseChangeSceneExtraDataMutation,
  changeSceneName: parseChangeSceneNameMutation,
  createAccessor: parseCreateAccessorMutation,
  createAnimation: parseCreateAnimationMutation,
  createBuffer: parseCreateBufferMutation,
  createBufferView: parseCreateBufferViewMutation,
  createCamera: parseCreateCameraMutation,
  createImage: parseCreateImageMutation,
  createMaterial: parseCreateMaterialMutation,
  createMesh: parseCreateMeshMutation,
  createMorphTarget: parseCreateMorphTargetMutation,
  createNode: parseCreateNodeMutation,
  createPrimitive: parseCreatePrimitiveMutation,
  createSampler: parseCreateSamplerMutation,
  createScene: parseCreateSceneMutation,
  createSkin: parseCreateSkinMutation,
  createTexture: parseCreateTextureMutation,
  addUsedExtension: parseAddUsedExtensionMutation,
  deleteAccessor: parseDeleteAccessorMutation,
  deleteAnimation: parseDeleteAnimationMutation,
  deleteBuffer: parseDeleteBufferMutation,
  deleteBufferView: parseDeleteBufferViewMutation,
  deleteCamera: parseDeleteCameraMutation,
  deleteImage: parseDeleteImageMutation,
  deleteMaterial: parseDeleteMaterialMutation,
  deleteMesh: parseDeleteMeshMutation,
  deleteMorphTarget: parseDeleteMorphTargetMutation,
  deleteNode: parseDeleteNodeMutation,
  deletePrimitive: parseDeletePrimitiveMutation,
  deleteSampler: parseDeleteSamplerMutation,
  deleteScene: parseDeleteSceneMutation,
  deleteSkin: parseDeleteSkinMutation,
  deleteTexture: parseDeleteTextureMutation,
  moveAccessor: parseMoveAccessorMutation,
  moveAnimation: parseMoveAnimationMutation,
  moveBuffer: parseMoveBufferMutation,
  moveBufferView: parseMoveBufferViewMutation,
  moveCamera: parseMoveCameraMutation,
  moveImage: parseMoveImageMutation,
  moveMaterial: parseMoveMaterialMutation,
  moveMesh: parseMoveMeshMutation,
  moveMorphTarget: parseMoveMorphTargetMutation,
  moveMorphTargetAttribute: parseMoveMorphTargetAttributeMutation,
  moveNode: parseMoveNodeMutation,
  moveNodeChild: parseMoveNodeChildMutation,
  movePrimitive: parseMovePrimitiveMutation,
  movePrimitiveAttribute: parseMovePrimitiveAttributeMutation,
  moveRequiredExtension: parseMoveRequiredExtensionMutation,
  moveSampler: parseMoveSamplerMutation,
  moveScene: parseMoveSceneMutation,
  moveSceneRootNode: parseMoveSceneRootNodeMutation,
  moveSkin: parseMoveSkinMutation,
  moveTexture: parseMoveTextureMutation,
  moveUsedExtension: parseMoveUsedExtensionMutation,
  reorderAccessors: parseReorderAccessorsMutation,
  reorderAnimations: parseReorderAnimationsMutation,
  reorderBufferViews: parseReorderBufferViewsMutation,
  reorderBuffers: parseReorderBuffersMutation,
  reorderCameras: parseReorderCamerasMutation,
  reorderImages: parseReorderImagesMutation,
  reorderMaterials: parseReorderMaterialsMutation,
  reorderMeshs: parseReorderMeshsMutation,
  reorderMorphTargetAttributes: parseReorderMorphTargetAttributesMutation,
  reorderMorphTargets: parseReorderMorphTargetsMutation,
  reorderNodeChildren: parseReorderNodeChildrenMutation,
  reorderNodes: parseReorderNodesMutation,
  reorderPrimitiveAttributes: parseReorderPrimitiveAttributesMutation,
  reorderPrimitives: parseReorderPrimitivesMutation,
  reorderRequiredExtensions: parseReorderRequiredExtensionsMutation,
  reorderSamplers: parseReorderSamplersMutation,
  reorderSceneRootNodes: parseReorderSceneRootNodesMutation,
  reorderScenes: parseReorderScenesMutation,
  reorderSkins: parseReorderSkinsMutation,
  reorderTextures: parseReorderTexturesMutation,
  reorderUsedExtensions: parseReorderUsedExtensionsMutation,
  moveNodeParent: parseMoveNodeParentMutation,
  addRequiredExtension: parseAddRequiredExtensionMutation,
  changeNodeTransform: parseChangeNodeTransformMutation,
  unbindDefaultScene: parseUnbindDefaultSceneMutation,
  unbindMorphTargetAttribute: parseUnbindMorphTargetAttributeMutation,
  unbindNodeCamera: parseUnbindNodeCameraMutation,
  unbindNodeChild: parseUnbindNodeChildMutation,
  unbindNodeMesh: parseUnbindNodeMeshMutation,
  unbindNodeSkin: parseUnbindNodeSkinMutation,
  unbindPrimitiveAttribute: parseUnbindPrimitiveAttributeMutation,
  unbindPrimitiveIndices: parseUnbindPrimitiveIndicesMutation,
  unbindPrimitiveMaterial: parseUnbindPrimitiveMaterialMutation,
  unbindSceneRootNode: parseUnbindSceneRootNodeMutation,
  removeRequiredExtension: parseRemoveRequiredExtensionMutation,
  removeUsedExtension: parseRemoveUsedExtensionMutation,
};
const mutations = Object.keys(payloads) as GltfMutation["mutation"][];

/** 📥️ Reads one aggregate wire: the tag names the leaf, whose own reader decodes the payload. */
export const parseGltfMutation: GltfWireReader<GltfMutation> = (value, at = "$") => {
  const row = gltfWireObject<{ mutation: GltfMutation["mutation"]; payload: unknown }>({ mutation: gltfWireRequired(gltfWireLiteral(...mutations)), payload: gltfWireRequired((payload) => payload) })(value, at);
  return { mutation: row.mutation, payload: payloads[row.mutation](row.payload, `${at}.payload`) } as GltfMutation;
};
