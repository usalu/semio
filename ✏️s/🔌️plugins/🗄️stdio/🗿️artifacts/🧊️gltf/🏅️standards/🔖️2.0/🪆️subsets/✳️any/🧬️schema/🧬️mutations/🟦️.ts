/** 🧬 Transparent TypeScript aggregate for the complete glTF mutation vocabulary. `GltfMutation`
 * carries `#[serde(tag = "mutation", content = "payload", rename_all = "camelCase")]`, so the tag
 * values are the camelCase form of the Rust variant names (e.g. `ReorderMeshs` ->
 * `"reorderMeshs"`), NOT the kebab-case `semanticKind` slugs this previously used for the tag
 * value. */
import type { GltfBindDefaultScenePayload } from './🔗️🎬️bind-default-scene/🟦️.ts';
import type { GltfBindMorphTargetAttributePayload } from './🔗️🧬️bind-morph-target-attribute/🟦️.ts';
import type { GltfBindNodeCameraPayload } from './🔗️🔘️bind-node-camera/🟦️.ts';
import type { GltfBindNodeChildPayload } from './🔗️🔘️bind-node-child/🟦️.ts';
import type { GltfBindNodeMeshPayload } from './🔗️🔘️bind-node-mesh/🟦️.ts';
import type { GltfBindNodeSkinPayload } from './🔗️🔘️bind-node-skin/🟦️.ts';
import type { GltfBindPrimitiveAttributePayload } from './🔗️🔺️bind-primitive-attribute/🟦️.ts';
import type { GltfBindPrimitiveIndicesPayload } from './🔗️🔺️bind-primitive-indices/🟦️.ts';
import type { GltfBindPrimitiveMaterialPayload } from './🔗️🔺️bind-primitive-material/🟦️.ts';
import type { GltfBindSceneRootNodePayload } from './🔗️🎬️bind-scene-root-node/🟦️.ts';
import type { GltfChangeAssetDescriptiveMetadataPayload } from './✏️📦️change-asset-descriptive-metadata/🟦️.ts';
import type { GltfChangeAssetExtensionDataPayload } from './✏️📦️change-asset-extension-data/🟦️.ts';
import type { GltfChangeAssetExtraDataPayload } from './✏️📦️change-asset-extra-data/🟦️.ts';
import type { GltfChangeAssetVersionPayload } from './✏️📦️change-asset-version/🟦️.ts';
import type { GltfChangeDocumentExtensionDataPayload } from './✏️📄️change-document-extension-data/🟦️.ts';
import type { GltfChangeDocumentExtraDataPayload } from './✏️📄️change-document-extra-data/🟦️.ts';
import type { GltfChangeMaterialAlphaModePayload } from './✏️💎️change-material-alpha-mode/🟦️.ts';
import type { GltfChangeMaterialDoubleSidedPayload } from './✏️💎️change-material-double-sided/🟦️.ts';
import type { GltfChangeMeshExtensionDataPayload } from './✏️🕸️change-mesh-extension-data/🟦️.ts';
import type { GltfChangeMeshExtraDataPayload } from './✏️🕸️change-mesh-extra-data/🟦️.ts';
import type { GltfChangeMeshMorphWeightsPayload } from './✏️🕸️change-mesh-morph-weights/🟦️.ts';
import type { GltfChangeMeshNamePayload } from './✏️🕸️change-mesh-name/🟦️.ts';
import type { GltfChangeNodeExtensionDataPayload } from './✏️🔘️change-node-extension-data/🟦️.ts';
import type { GltfChangeNodeExtraDataPayload } from './✏️🔘️change-node-extra-data/🟦️.ts';
import type { GltfChangeNodeMorphWeightsPayload } from './✏️🔘️change-node-morph-weights/🟦️.ts';
import type { GltfChangeNodeNameMutation } from './✏️🔘️change-node-name/🟦️.ts';
import type { GltfChangePrimitiveExtensionDataPayload } from './✏️🔺️change-primitive-extension-data/🟦️.ts';
import type { GltfChangePrimitiveExtraDataPayload } from './✏️🔺️change-primitive-extra-data/🟦️.ts';
import type { GltfChangePrimitiveTopologyModePayload } from './✏️🔺️change-primitive-topology-mode/🟦️.ts';
import type { GltfChangeSceneExtensionDataPayload } from './✏️🎬️change-scene-extension-data/🟦️.ts';
import type { GltfChangeSceneExtraDataPayload } from './✏️🎬️change-scene-extra-data/🟦️.ts';
import type { GltfChangeSceneNamePayload } from './✏️🎬️change-scene-name/🟦️.ts';
import type { GltfCreateAccessorPayload } from './🌱️📐️create-accessor/🟦️.ts';
import type { GltfCreateAnimationPayload } from './🌱️🎞️create-animation/🟦️.ts';
import type { GltfCreateBufferPayload } from './🌱️💾️create-buffer/🟦️.ts';
import type { GltfCreateBufferViewPayload } from './🌱️👁️create-buffer-view/🟦️.ts';
import type { GltfCreateCameraPayload } from './🌱️🎥️create-camera/🟦️.ts';
import type { GltfCreateImagePayload } from './🌱️🖼️create-image/🟦️.ts';
import type { GltfCreateMaterialPayload } from './🌱️💎️create-material/🟦️.ts';
import type { GltfCreateMeshPayload } from './🌱️🕸️create-mesh/🟦️.ts';
import type { GltfCreateMorphTargetPayload } from './🌱️🧬️create-morph-target/🟦️.ts';
import type { GltfCreateNodePayload } from './🌱️🔘️create-node/🟦️.ts';
import type { GltfCreatePrimitivePayload } from './🌱️🔺️create-primitive/🟦️.ts';
import type { GltfCreateSamplerPayload } from './🌱️🎛️create-sampler/🟦️.ts';
import type { GltfCreateScenePayload } from './🌱️🎬️create-scene/🟦️.ts';
import type { GltfCreateSkinPayload } from './🌱️🧥️create-skin/🟦️.ts';
import type { GltfCreateTexturePayload } from './🌱️🎨️create-texture/🟦️.ts';
import type { GltfDeclareUsedExtensionPayload } from './📣️🧩️add-used-extension/🟦️.ts';
import type { GltfDeleteAccessorPayload } from './🗑️📐️delete-accessor/🟦️.ts';
import type { GltfDeleteAnimationPayload } from './🗑️🎞️delete-animation/🟦️.ts';
import type { GltfDeleteBufferPayload } from './🗑️💾️delete-buffer/🟦️.ts';
import type { GltfDeleteBufferViewPayload } from './🗑️👁️delete-buffer-view/🟦️.ts';
import type { GltfDeleteCameraPayload } from './🗑️🎥️delete-camera/🟦️.ts';
import type { GltfDeleteImagePayload } from './🗑️🖼️delete-image/🟦️.ts';
import type { GltfDeleteMaterialPayload } from './🗑️💎️delete-material/🟦️.ts';
import type { GltfDeleteMeshPayload } from './🗑️🕸️delete-mesh/🟦️.ts';
import type { GltfDeleteMorphTargetPayload } from './🗑️🧬️delete-morph-target/🟦️.ts';
import type { GltfDeleteNodePayload } from './🗑️🔘️delete-node/🟦️.ts';
import type { GltfDeletePrimitivePayload } from './🗑️🔺️delete-primitive/🟦️.ts';
import type { GltfDeleteSamplerPayload } from './🗑️🎛️delete-sampler/🟦️.ts';
import type { GltfDeleteScenePayload } from './🗑️🎬️delete-scene/🟦️.ts';
import type { GltfDeleteSkinPayload } from './🗑️🧥️delete-skin/🟦️.ts';
import type { GltfDeleteTexturePayload } from './🗑️🎨️delete-texture/🟦️.ts';
import type { GltfMoveAccessorPayload } from './🚚️📐️move-accessor/🟦️.ts';
import type { GltfMoveAnimationPayload } from './🚚️🎞️move-animation/🟦️.ts';
import type { GltfMoveBufferPayload } from './🚚️💾️move-buffer/🟦️.ts';
import type { GltfMoveBufferViewPayload } from './🚚️👁️move-buffer-view/🟦️.ts';
import type { GltfMoveCameraPayload } from './🚚️🎥️move-camera/🟦️.ts';
import type { GltfMoveImagePayload } from './🚚️🖼️move-image/🟦️.ts';
import type { GltfMoveMaterialPayload } from './🚚️💎️move-material/🟦️.ts';
import type { GltfMoveMeshPayload } from './🚚️🕸️move-mesh/🟦️.ts';
import type { GltfMoveMorphTargetPayload } from './🚚️🧬️move-morph-target/🟦️.ts';
import type { GltfMoveMorphTargetAttributePayload } from './🚚️🧬️move-morph-target-attribute/🟦️.ts';
import type { GltfMoveNodePayload } from './🚚️🔘️move-node/🟦️.ts';
import type { GltfMoveNodeChildPayload } from './🚚️🔘️move-node-child/🟦️.ts';
import type { GltfMovePrimitivePayload } from './🚚️🔺️move-primitive/🟦️.ts';
import type { GltfMovePrimitiveAttributePayload } from './🚚️🔺️move-primitive-attribute/🟦️.ts';
import type { GltfMoveRequiredExtensionPayload } from './🚚️🧩️move-required-extension/🟦️.ts';
import type { GltfMoveSamplerPayload } from './🚚️🎛️move-sampler/🟦️.ts';
import type { GltfMoveScenePayload } from './🚚️🎬️move-scene/🟦️.ts';
import type { GltfMoveSceneRootNodePayload } from './🚚️🎬️move-scene-root-node/🟦️.ts';
import type { GltfMoveSkinPayload } from './🚚️🧥️move-skin/🟦️.ts';
import type { GltfMoveTexturePayload } from './🚚️🎨️move-texture/🟦️.ts';
import type { GltfMoveUsedExtensionPayload } from './🚚️🧩️move-used-extension/🟦️.ts';
import type { GltfReorderAccessorsPayload } from './🔀️📐️reorder-accessors/🟦️.ts';
import type { GltfReorderAnimationsPayload } from './🔀️🎞️reorder-animations/🟦️.ts';
import type { GltfReorderBufferViewsPayload } from './🔀️👁️reorder-buffer-views/🟦️.ts';
import type { GltfReorderBuffersPayload } from './🔀️💾️reorder-buffers/🟦️.ts';
import type { GltfReorderCamerasPayload } from './🔀️🎥️reorder-cameras/🟦️.ts';
import type { GltfReorderImagesPayload } from './🔀️🖼️reorder-images/🟦️.ts';
import type { GltfReorderMaterialsPayload } from './🔀️💎️reorder-materials/🟦️.ts';
import type { GltfReorderMeshsPayload } from './🔀️🕸️reorder-meshs/🟦️.ts';
import type { GltfReorderMorphTargetAttributesPayload } from './🔀️🧬️reorder-morph-target-attributes/🟦️.ts';
import type { GltfReorderMorphTargetsPayload } from './🔀️🧬️reorder-morph-targets/🟦️.ts';
import type { GltfReorderNodeChildrenPayload } from './🔀️🔘️reorder-node-children/🟦️.ts';
import type { GltfReorderNodesPayload } from './🔀️🔘️reorder-nodes/🟦️.ts';
import type { GltfReorderPrimitiveAttributesPayload } from './🔀️🔺️reorder-primitive-attributes/🟦️.ts';
import type { GltfReorderPrimitivesPayload } from './🔀️🔺️reorder-primitives/🟦️.ts';
import type { GltfReorderRequiredExtensionsPayload } from './🔀️🧩️reorder-required-extensions/🟦️.ts';
import type { GltfReorderSamplersPayload } from './🔀️🎛️reorder-samplers/🟦️.ts';
import type { GltfReorderSceneRootNodesPayload } from './🔀️🎬️reorder-scene-root-nodes/🟦️.ts';
import type { GltfReorderScenesPayload } from './🔀️🎬️reorder-scenes/🟦️.ts';
import type { GltfReorderSkinsPayload } from './🔀️🧥️reorder-skins/🟦️.ts';
import type { GltfReorderTexturesPayload } from './🔀️🎨️reorder-textures/🟦️.ts';
import type { GltfReorderUsedExtensionsPayload } from './🔀️🧩️reorder-used-extensions/🟦️.ts';
import type { GltfReparentNodePayload } from './👪️🔘️move-node-parent/🟦️.ts';
import type { GltfRequireExtensionPayload } from './✅️🧩️add-required-extension/🟦️.ts';
import type { GltfTransformNodePayload } from './🔄️🔘️change-node-transform/🟦️.ts';
import type { GltfUnbindDefaultScenePayload } from './✂️🎬️unbind-default-scene/🟦️.ts';
import type { GltfUnbindMorphTargetAttributePayload } from './✂️🧬️unbind-morph-target-attribute/🟦️.ts';
import type { GltfUnbindNodeCameraPayload } from './✂️🔘️unbind-node-camera/🟦️.ts';
import type { GltfUnbindNodeChildPayload } from './✂️🔘️unbind-node-child/🟦️.ts';
import type { GltfUnbindNodeMeshPayload } from './✂️🔘️unbind-node-mesh/🟦️.ts';
import type { GltfUnbindNodeSkinPayload } from './✂️🔘️unbind-node-skin/🟦️.ts';
import type { GltfUnbindPrimitiveAttributePayload } from './✂️🔺️unbind-primitive-attribute/🟦️.ts';
import type { GltfUnbindPrimitiveIndicesPayload } from './✂️🔺️unbind-primitive-indices/🟦️.ts';
import type { GltfUnbindPrimitiveMaterialPayload } from './✂️🔺️unbind-primitive-material/🟦️.ts';
import type { GltfUnbindSceneRootNodePayload } from './✂️🎬️unbind-scene-root-node/🟦️.ts';
import type { GltfUnrequireExtensionPayload } from './🚫️🧩️remove-required-extension/🟦️.ts';
import type { GltfWithdrawUsedExtensionPayload } from './🔙️🧩️remove-used-extension/🟦️.ts';

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
