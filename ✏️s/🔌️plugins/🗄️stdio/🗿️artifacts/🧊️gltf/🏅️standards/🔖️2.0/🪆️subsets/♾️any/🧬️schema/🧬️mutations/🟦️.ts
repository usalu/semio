/** 🧬 Transparent TypeScript aggregate for the complete glTF mutation vocabulary. `GltfMutation`
 * carries `#[serde(tag = "mutation", content = "payload", rename_all = "camelCase")]`, so the tag
 * values are the camelCase form of the Rust variant names (e.g. `ReorderMeshs` ->
 * `"reorderMeshs"`), NOT the kebab-case `semanticKind` slugs this previously used for the tag
 * value. */
import type { GltfBindDefaultScenePayload } from './🏠️default-scene/🔗️bind/🟦️.ts';
import type { GltfBindMorphTargetAttributePayload } from './🎚️morph-attribute/🔗️bind/🟦️.ts';
import type { GltfBindNodeCameraPayload } from './📷️node-camera/🔗️bind/🟦️.ts';
import type { GltfBindNodeChildPayload } from './🌿️node-child/🔗️bind/🟦️.ts';
import type { GltfBindNodeMeshPayload } from './🏗️node-mesh/🔗️bind/🟦️.ts';
import type { GltfBindNodeSkinPayload } from './🩻️node-skin/🔗️bind/🟦️.ts';
import type { GltfBindPrimitiveAttributePayload } from './🔤️primitive-attribute/🔗️bind/🟦️.ts';
import type { GltfBindPrimitiveIndicesPayload } from './🔢️primitive-indices/🔗️bind/🟦️.ts';
import type { GltfBindPrimitiveMaterialPayload } from './🧱️primitive-material/🔗️bind/🟦️.ts';
import type { GltfBindSceneRootNodePayload } from './🌲️scene-root/🔗️bind/🟦️.ts';
import type { GltfChangeAssetDescriptiveMetadataPayload } from './🪪️asset/📝️change-description/🟦️.ts';
import type { GltfChangeAssetExtensionDataPayload } from './🪪️asset/🧩️change-extensions/🟦️.ts';
import type { GltfChangeAssetExtraDataPayload } from './🪪️asset/🧾️change-extras/🟦️.ts';
import type { GltfChangeAssetVersionPayload } from './🪪️asset/🔖️version/🟦️.ts';
import type { GltfChangeDocumentExtensionDataPayload } from './📃️document/🧩️change-extensions/🟦️.ts';
import type { GltfChangeDocumentExtraDataPayload } from './📃️document/📝️change-extras/🟦️.ts';
import type { GltfChangeMaterialAlphaModePayload } from './💎️material/🌫️change-alpha/🟦️.ts';
import type { GltfChangeMaterialDoubleSidedPayload } from './💎️material/🪞️change-sides/🟦️.ts';
import type { GltfChangeMeshExtensionDataPayload } from './🕸️mesh/🧩️change-extensions/🟦️.ts';
import type { GltfChangeMeshExtraDataPayload } from './🕸️mesh/📝️change-extras/🟦️.ts';
import type { GltfChangeMeshMorphWeightsPayload } from './🕸️mesh/⚖️change-weights/🟦️.ts';
import type { GltfChangeMeshNamePayload } from './🕸️mesh/🏷️rename/🟦️.ts';
import type { GltfChangeNodeExtensionDataPayload } from './🌳️node/🧩️change-extensions/🟦️.ts';
import type { GltfChangeNodeExtraDataPayload } from './🌳️node/📝️change-extras/🟦️.ts';
import type { GltfChangeNodeMorphWeightsPayload } from './🌳️node/⚖️change-weights/🟦️.ts';
import type { GltfChangeNodeNameMutation } from './🌳️node/🏷️rename/🟦️.ts';
import type { GltfChangePrimitiveExtensionDataPayload } from './🔺️primitive/🧩️change-extensions/🟦️.ts';
import type { GltfChangePrimitiveExtraDataPayload } from './🔺️primitive/📝️change-extras/🟦️.ts';
import type { GltfChangePrimitiveTopologyModePayload } from './🔺️primitive/📐️change-topology/🟦️.ts';
import type { GltfChangeSceneExtensionDataPayload } from './🎬️scene/🧩️change-extensions/🟦️.ts';
import type { GltfChangeSceneExtraDataPayload } from './🎬️scene/📝️change-extras/🟦️.ts';
import type { GltfChangeSceneNamePayload } from './🎬️scene/🏷️rename/🟦️.ts';
import type { GltfCreateAccessorPayload } from './📐️accessor/🌱️create/🟦️.ts';
import type { GltfCreateAnimationPayload } from './🎞️animation/🌱️create/🟦️.ts';
import type { GltfCreateBufferPayload } from './💿️buffer/🌱️create/🟦️.ts';
import type { GltfCreateBufferViewPayload } from './🪟️buffer-view/🌱️create/🟦️.ts';
import type { GltfCreateCameraPayload } from './🎥️camera/🌱️create/🟦️.ts';
import type { GltfCreateImagePayload } from './🖼️image/🌱️create/🟦️.ts';
import type { GltfCreateMaterialPayload } from './💎️material/🌱️create/🟦️.ts';
import type { GltfCreateMeshPayload } from './🕸️mesh/🌱️create/🟦️.ts';
import type { GltfCreateMorphTargetPayload } from './🧬️morph-target/🌱️create/🟦️.ts';
import type { GltfCreateNodePayload } from './🌳️node/🌱️create/🟦️.ts';
import type { GltfCreatePrimitivePayload } from './🔺️primitive/🌱️create/🟦️.ts';
import type { GltfCreateSamplerPayload } from './🎛️sampler/🌱️create/🟦️.ts';
import type { GltfCreateScenePayload } from './🎬️scene/🌱️create/🟦️.ts';
import type { GltfCreateSkinPayload } from './🦴️skin/🌱️create/🟦️.ts';
import type { GltfCreateTexturePayload } from './🎨️texture/🌱️create/🟦️.ts';
import type { GltfDeclareUsedExtensionPayload } from './📣️used-extension/➕️add/🟦️.ts';
import type { GltfDeleteAccessorPayload } from './📐️accessor/🗑️delete/🟦️.ts';
import type { GltfDeleteAnimationPayload } from './🎞️animation/🗑️delete/🟦️.ts';
import type { GltfDeleteBufferPayload } from './💿️buffer/🗑️delete/🟦️.ts';
import type { GltfDeleteBufferViewPayload } from './🪟️buffer-view/🗑️delete/🟦️.ts';
import type { GltfDeleteCameraPayload } from './🎥️camera/🗑️delete/🟦️.ts';
import type { GltfDeleteImagePayload } from './🖼️image/🗑️delete/🟦️.ts';
import type { GltfDeleteMaterialPayload } from './💎️material/🗑️delete/🟦️.ts';
import type { GltfDeleteMeshPayload } from './🕸️mesh/🗑️delete/🟦️.ts';
import type { GltfDeleteMorphTargetPayload } from './🧬️morph-target/🗑️delete/🟦️.ts';
import type { GltfDeleteNodePayload } from './🌳️node/🗑️delete/🟦️.ts';
import type { GltfDeletePrimitivePayload } from './🔺️primitive/🗑️delete/🟦️.ts';
import type { GltfDeleteSamplerPayload } from './🎛️sampler/🗑️delete/🟦️.ts';
import type { GltfDeleteScenePayload } from './🎬️scene/🗑️delete/🟦️.ts';
import type { GltfDeleteSkinPayload } from './🦴️skin/🗑️delete/🟦️.ts';
import type { GltfDeleteTexturePayload } from './🎨️texture/🗑️delete/🟦️.ts';
import type { GltfMoveAccessorPayload } from './📐️accessor/🚚️move/🟦️.ts';
import type { GltfMoveAnimationPayload } from './🎞️animation/🚚️move/🟦️.ts';
import type { GltfMoveBufferPayload } from './💿️buffer/🚚️move/🟦️.ts';
import type { GltfMoveBufferViewPayload } from './🪟️buffer-view/🚚️move/🟦️.ts';
import type { GltfMoveCameraPayload } from './🎥️camera/🚚️move/🟦️.ts';
import type { GltfMoveImagePayload } from './🖼️image/🚚️move/🟦️.ts';
import type { GltfMoveMaterialPayload } from './💎️material/🚚️move/🟦️.ts';
import type { GltfMoveMeshPayload } from './🕸️mesh/🚚️move/🟦️.ts';
import type { GltfMoveMorphTargetPayload } from './🧬️morph-target/🚚️move/🟦️.ts';
import type { GltfMoveMorphTargetAttributePayload } from './🎚️morph-attribute/🚚️move/🟦️.ts';
import type { GltfMoveNodePayload } from './🌳️node/🚚️move/🟦️.ts';
import type { GltfMoveNodeChildPayload } from './🌿️node-child/🚚️move/🟦️.ts';
import type { GltfMovePrimitivePayload } from './🔺️primitive/🚚️move/🟦️.ts';
import type { GltfMovePrimitiveAttributePayload } from './🔤️primitive-attribute/🚚️move/🟦️.ts';
import type { GltfMoveRequiredExtensionPayload } from './✅️required-extension/🚚️move/🟦️.ts';
import type { GltfMoveSamplerPayload } from './🎛️sampler/🚚️move/🟦️.ts';
import type { GltfMoveScenePayload } from './🎬️scene/🚚️move/🟦️.ts';
import type { GltfMoveSceneRootNodePayload } from './🌲️scene-root/🚚️move/🟦️.ts';
import type { GltfMoveSkinPayload } from './🦴️skin/🚚️move/🟦️.ts';
import type { GltfMoveTexturePayload } from './🎨️texture/🚚️move/🟦️.ts';
import type { GltfMoveUsedExtensionPayload } from './📣️used-extension/🚚️move/🟦️.ts';
import type { GltfReorderAccessorsPayload } from './📐️accessor/🔀️reorder/🟦️.ts';
import type { GltfReorderAnimationsPayload } from './🎞️animation/🔀️reorder/🟦️.ts';
import type { GltfReorderBufferViewsPayload } from './🪟️buffer-view/🔀️reorder/🟦️.ts';
import type { GltfReorderBuffersPayload } from './💿️buffer/🔀️reorder/🟦️.ts';
import type { GltfReorderCamerasPayload } from './🎥️camera/🔀️reorder/🟦️.ts';
import type { GltfReorderImagesPayload } from './🖼️image/🔀️reorder/🟦️.ts';
import type { GltfReorderMaterialsPayload } from './💎️material/🔀️reorder/🟦️.ts';
import type { GltfReorderMeshsPayload } from './🕸️mesh/🔀️reorder/🟦️.ts';
import type { GltfReorderMorphTargetAttributesPayload } from './🎚️morph-attribute/🔀️reorder/🟦️.ts';
import type { GltfReorderMorphTargetsPayload } from './🧬️morph-target/🔀️reorder/🟦️.ts';
import type { GltfReorderNodeChildrenPayload } from './🌿️node-child/🔀️reorder/🟦️.ts';
import type { GltfReorderNodesPayload } from './🌳️node/🔀️reorder/🟦️.ts';
import type { GltfReorderPrimitiveAttributesPayload } from './🔤️primitive-attribute/🔀️reorder/🟦️.ts';
import type { GltfReorderPrimitivesPayload } from './🔺️primitive/🔀️reorder/🟦️.ts';
import type { GltfReorderRequiredExtensionsPayload } from './✅️required-extension/🔀️reorder/🟦️.ts';
import type { GltfReorderSamplersPayload } from './🎛️sampler/🔀️reorder/🟦️.ts';
import type { GltfReorderSceneRootNodesPayload } from './🌲️scene-root/🔀️reorder/🟦️.ts';
import type { GltfReorderScenesPayload } from './🎬️scene/🔀️reorder/🟦️.ts';
import type { GltfReorderSkinsPayload } from './🦴️skin/🔀️reorder/🟦️.ts';
import type { GltfReorderTexturesPayload } from './🎨️texture/🔀️reorder/🟦️.ts';
import type { GltfReorderUsedExtensionsPayload } from './📣️used-extension/🔀️reorder/🟦️.ts';
import type { GltfReparentNodePayload } from './🌳️node/🌿️reparent/🟦️.ts';
import type { GltfRequireExtensionPayload } from './✅️required-extension/➕️add/🟦️.ts';
import type { GltfTransformNodePayload } from './🌳️node/📐️transform/🟦️.ts';
import type { GltfUnbindDefaultScenePayload } from './🏠️default-scene/✂️unbind/🟦️.ts';
import type { GltfUnbindMorphTargetAttributePayload } from './🎚️morph-attribute/✂️unbind/🟦️.ts';
import type { GltfUnbindNodeCameraPayload } from './📷️node-camera/✂️unbind/🟦️.ts';
import type { GltfUnbindNodeChildPayload } from './🌿️node-child/✂️unbind/🟦️.ts';
import type { GltfUnbindNodeMeshPayload } from './🏗️node-mesh/✂️unbind/🟦️.ts';
import type { GltfUnbindNodeSkinPayload } from './🩻️node-skin/✂️unbind/🟦️.ts';
import type { GltfUnbindPrimitiveAttributePayload } from './🔤️primitive-attribute/✂️unbind/🟦️.ts';
import type { GltfUnbindPrimitiveIndicesPayload } from './🔢️primitive-indices/✂️unbind/🟦️.ts';
import type { GltfUnbindPrimitiveMaterialPayload } from './🧱️primitive-material/✂️unbind/🟦️.ts';
import type { GltfUnbindSceneRootNodePayload } from './🌲️scene-root/✂️unbind/🟦️.ts';
import type { GltfUnrequireExtensionPayload } from './✅️required-extension/➖️remove/🟦️.ts';
import type { GltfWithdrawUsedExtensionPayload } from './📣️used-extension/➖️remove/🟦️.ts';

export type GltfMutation =
  | { readonly mutation: 'bindDefaultScene'; readonly payload: GltfBindDefaultScenePayload }
  | { readonly mutation: 'bindMorphTargetAttribute'; readonly payload: GltfBindMorphTargetAttributePayload }
  | { readonly mutation: 'bindNodeCamera'; readonly payload: GltfBindNodeCameraPayload }
  | { readonly mutation: 'bindNodeChild'; readonly payload: GltfBindNodeChildPayload }
  | { readonly mutation: 'bindNodeMesh'; readonly payload: GltfBindNodeMeshPayload }
  | { readonly mutation: 'bindNodeSkin'; readonly payload: GltfBindNodeSkinPayload }
  | { readonly mutation: 'bindPrimitiveAttribute'; readonly payload: GltfBindPrimitiveAttributePayload }
  | { readonly mutation: 'bindPrimitiveIndices'; readonly payload: GltfBindPrimitiveIndicesPayload }
  | { readonly mutation: 'bindPrimitiveMaterial'; readonly payload: GltfBindPrimitiveMaterialPayload }
  | { readonly mutation: 'bindSceneRootNode'; readonly payload: GltfBindSceneRootNodePayload }
  | { readonly mutation: 'changeAssetDescriptiveMetadata'; readonly payload: GltfChangeAssetDescriptiveMetadataPayload }
  | { readonly mutation: 'changeAssetExtensionData'; readonly payload: GltfChangeAssetExtensionDataPayload }
  | { readonly mutation: 'changeAssetExtraData'; readonly payload: GltfChangeAssetExtraDataPayload }
  | { readonly mutation: 'changeAssetVersion'; readonly payload: GltfChangeAssetVersionPayload }
  | { readonly mutation: 'changeDocumentExtensionData'; readonly payload: GltfChangeDocumentExtensionDataPayload }
  | { readonly mutation: 'changeDocumentExtraData'; readonly payload: GltfChangeDocumentExtraDataPayload }
  | { readonly mutation: 'changeMaterialAlphaMode'; readonly payload: GltfChangeMaterialAlphaModePayload }
  | { readonly mutation: 'changeMaterialDoubleSided'; readonly payload: GltfChangeMaterialDoubleSidedPayload }
  | { readonly mutation: 'changeMeshExtensionData'; readonly payload: GltfChangeMeshExtensionDataPayload }
  | { readonly mutation: 'changeMeshExtraData'; readonly payload: GltfChangeMeshExtraDataPayload }
  | { readonly mutation: 'changeMeshMorphWeights'; readonly payload: GltfChangeMeshMorphWeightsPayload }
  | { readonly mutation: 'changeMeshName'; readonly payload: GltfChangeMeshNamePayload }
  | { readonly mutation: 'changeNodeExtensionData'; readonly payload: GltfChangeNodeExtensionDataPayload }
  | { readonly mutation: 'changeNodeExtraData'; readonly payload: GltfChangeNodeExtraDataPayload }
  | { readonly mutation: 'changeNodeMorphWeights'; readonly payload: GltfChangeNodeMorphWeightsPayload }
  | { readonly mutation: 'changeNodeName'; readonly payload: GltfChangeNodeNameMutation }
  | { readonly mutation: 'changePrimitiveExtensionData'; readonly payload: GltfChangePrimitiveExtensionDataPayload }
  | { readonly mutation: 'changePrimitiveExtraData'; readonly payload: GltfChangePrimitiveExtraDataPayload }
  | { readonly mutation: 'changePrimitiveTopologyMode'; readonly payload: GltfChangePrimitiveTopologyModePayload }
  | { readonly mutation: 'changeSceneExtensionData'; readonly payload: GltfChangeSceneExtensionDataPayload }
  | { readonly mutation: 'changeSceneExtraData'; readonly payload: GltfChangeSceneExtraDataPayload }
  | { readonly mutation: 'changeSceneName'; readonly payload: GltfChangeSceneNamePayload }
  | { readonly mutation: 'createAccessor'; readonly payload: GltfCreateAccessorPayload }
  | { readonly mutation: 'createAnimation'; readonly payload: GltfCreateAnimationPayload }
  | { readonly mutation: 'createBuffer'; readonly payload: GltfCreateBufferPayload }
  | { readonly mutation: 'createBufferView'; readonly payload: GltfCreateBufferViewPayload }
  | { readonly mutation: 'createCamera'; readonly payload: GltfCreateCameraPayload }
  | { readonly mutation: 'createImage'; readonly payload: GltfCreateImagePayload }
  | { readonly mutation: 'createMaterial'; readonly payload: GltfCreateMaterialPayload }
  | { readonly mutation: 'createMesh'; readonly payload: GltfCreateMeshPayload }
  | { readonly mutation: 'createMorphTarget'; readonly payload: GltfCreateMorphTargetPayload }
  | { readonly mutation: 'createNode'; readonly payload: GltfCreateNodePayload }
  | { readonly mutation: 'createPrimitive'; readonly payload: GltfCreatePrimitivePayload }
  | { readonly mutation: 'createSampler'; readonly payload: GltfCreateSamplerPayload }
  | { readonly mutation: 'createScene'; readonly payload: GltfCreateScenePayload }
  | { readonly mutation: 'createSkin'; readonly payload: GltfCreateSkinPayload }
  | { readonly mutation: 'createTexture'; readonly payload: GltfCreateTexturePayload }
  | { readonly mutation: 'addUsedExtension'; readonly payload: GltfDeclareUsedExtensionPayload }
  | { readonly mutation: 'deleteAccessor'; readonly payload: GltfDeleteAccessorPayload }
  | { readonly mutation: 'deleteAnimation'; readonly payload: GltfDeleteAnimationPayload }
  | { readonly mutation: 'deleteBuffer'; readonly payload: GltfDeleteBufferPayload }
  | { readonly mutation: 'deleteBufferView'; readonly payload: GltfDeleteBufferViewPayload }
  | { readonly mutation: 'deleteCamera'; readonly payload: GltfDeleteCameraPayload }
  | { readonly mutation: 'deleteImage'; readonly payload: GltfDeleteImagePayload }
  | { readonly mutation: 'deleteMaterial'; readonly payload: GltfDeleteMaterialPayload }
  | { readonly mutation: 'deleteMesh'; readonly payload: GltfDeleteMeshPayload }
  | { readonly mutation: 'deleteMorphTarget'; readonly payload: GltfDeleteMorphTargetPayload }
  | { readonly mutation: 'deleteNode'; readonly payload: GltfDeleteNodePayload }
  | { readonly mutation: 'deletePrimitive'; readonly payload: GltfDeletePrimitivePayload }
  | { readonly mutation: 'deleteSampler'; readonly payload: GltfDeleteSamplerPayload }
  | { readonly mutation: 'deleteScene'; readonly payload: GltfDeleteScenePayload }
  | { readonly mutation: 'deleteSkin'; readonly payload: GltfDeleteSkinPayload }
  | { readonly mutation: 'deleteTexture'; readonly payload: GltfDeleteTexturePayload }
  | { readonly mutation: 'moveAccessor'; readonly payload: GltfMoveAccessorPayload }
  | { readonly mutation: 'moveAnimation'; readonly payload: GltfMoveAnimationPayload }
  | { readonly mutation: 'moveBuffer'; readonly payload: GltfMoveBufferPayload }
  | { readonly mutation: 'moveBufferView'; readonly payload: GltfMoveBufferViewPayload }
  | { readonly mutation: 'moveCamera'; readonly payload: GltfMoveCameraPayload }
  | { readonly mutation: 'moveImage'; readonly payload: GltfMoveImagePayload }
  | { readonly mutation: 'moveMaterial'; readonly payload: GltfMoveMaterialPayload }
  | { readonly mutation: 'moveMesh'; readonly payload: GltfMoveMeshPayload }
  | { readonly mutation: 'moveMorphTarget'; readonly payload: GltfMoveMorphTargetPayload }
  | { readonly mutation: 'moveMorphTargetAttribute'; readonly payload: GltfMoveMorphTargetAttributePayload }
  | { readonly mutation: 'moveNode'; readonly payload: GltfMoveNodePayload }
  | { readonly mutation: 'moveNodeChild'; readonly payload: GltfMoveNodeChildPayload }
  | { readonly mutation: 'movePrimitive'; readonly payload: GltfMovePrimitivePayload }
  | { readonly mutation: 'movePrimitiveAttribute'; readonly payload: GltfMovePrimitiveAttributePayload }
  | { readonly mutation: 'moveRequiredExtension'; readonly payload: GltfMoveRequiredExtensionPayload }
  | { readonly mutation: 'moveSampler'; readonly payload: GltfMoveSamplerPayload }
  | { readonly mutation: 'moveScene'; readonly payload: GltfMoveScenePayload }
  | { readonly mutation: 'moveSceneRootNode'; readonly payload: GltfMoveSceneRootNodePayload }
  | { readonly mutation: 'moveSkin'; readonly payload: GltfMoveSkinPayload }
  | { readonly mutation: 'moveTexture'; readonly payload: GltfMoveTexturePayload }
  | { readonly mutation: 'moveUsedExtension'; readonly payload: GltfMoveUsedExtensionPayload }
  | { readonly mutation: 'reorderAccessors'; readonly payload: GltfReorderAccessorsPayload }
  | { readonly mutation: 'reorderAnimations'; readonly payload: GltfReorderAnimationsPayload }
  | { readonly mutation: 'reorderBufferViews'; readonly payload: GltfReorderBufferViewsPayload }
  | { readonly mutation: 'reorderBuffers'; readonly payload: GltfReorderBuffersPayload }
  | { readonly mutation: 'reorderCameras'; readonly payload: GltfReorderCamerasPayload }
  | { readonly mutation: 'reorderImages'; readonly payload: GltfReorderImagesPayload }
  | { readonly mutation: 'reorderMaterials'; readonly payload: GltfReorderMaterialsPayload }
  | { readonly mutation: 'reorderMeshs'; readonly payload: GltfReorderMeshsPayload }
  | { readonly mutation: 'reorderMorphTargetAttributes'; readonly payload: GltfReorderMorphTargetAttributesPayload }
  | { readonly mutation: 'reorderMorphTargets'; readonly payload: GltfReorderMorphTargetsPayload }
  | { readonly mutation: 'reorderNodeChildren'; readonly payload: GltfReorderNodeChildrenPayload }
  | { readonly mutation: 'reorderNodes'; readonly payload: GltfReorderNodesPayload }
  | { readonly mutation: 'reorderPrimitiveAttributes'; readonly payload: GltfReorderPrimitiveAttributesPayload }
  | { readonly mutation: 'reorderPrimitives'; readonly payload: GltfReorderPrimitivesPayload }
  | { readonly mutation: 'reorderRequiredExtensions'; readonly payload: GltfReorderRequiredExtensionsPayload }
  | { readonly mutation: 'reorderSamplers'; readonly payload: GltfReorderSamplersPayload }
  | { readonly mutation: 'reorderSceneRootNodes'; readonly payload: GltfReorderSceneRootNodesPayload }
  | { readonly mutation: 'reorderScenes'; readonly payload: GltfReorderScenesPayload }
  | { readonly mutation: 'reorderSkins'; readonly payload: GltfReorderSkinsPayload }
  | { readonly mutation: 'reorderTextures'; readonly payload: GltfReorderTexturesPayload }
  | { readonly mutation: 'reorderUsedExtensions'; readonly payload: GltfReorderUsedExtensionsPayload }
  | { readonly mutation: 'moveNodeParent'; readonly payload: GltfReparentNodePayload }
  | { readonly mutation: 'addRequiredExtension'; readonly payload: GltfRequireExtensionPayload }
  | { readonly mutation: 'changeNodeTransform'; readonly payload: GltfTransformNodePayload }
  | { readonly mutation: 'unbindDefaultScene'; readonly payload: GltfUnbindDefaultScenePayload }
  | { readonly mutation: 'unbindMorphTargetAttribute'; readonly payload: GltfUnbindMorphTargetAttributePayload }
  | { readonly mutation: 'unbindNodeCamera'; readonly payload: GltfUnbindNodeCameraPayload }
  | { readonly mutation: 'unbindNodeChild'; readonly payload: GltfUnbindNodeChildPayload }
  | { readonly mutation: 'unbindNodeMesh'; readonly payload: GltfUnbindNodeMeshPayload }
  | { readonly mutation: 'unbindNodeSkin'; readonly payload: GltfUnbindNodeSkinPayload }
  | { readonly mutation: 'unbindPrimitiveAttribute'; readonly payload: GltfUnbindPrimitiveAttributePayload }
  | { readonly mutation: 'unbindPrimitiveIndices'; readonly payload: GltfUnbindPrimitiveIndicesPayload }
  | { readonly mutation: 'unbindPrimitiveMaterial'; readonly payload: GltfUnbindPrimitiveMaterialPayload }
  | { readonly mutation: 'unbindSceneRootNode'; readonly payload: GltfUnbindSceneRootNodePayload }
  | { readonly mutation: 'removeRequiredExtension'; readonly payload: GltfUnrequireExtensionPayload }
  | { readonly mutation: 'removeUsedExtension'; readonly payload: GltfWithdrawUsedExtensionPayload };
