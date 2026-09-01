/** 🧬 Transparent TypeScript aggregate for the complete glTF mutation vocabulary. `GltfMutation`
 * carries `#[serde(tag = "mutation", content = "payload", rename_all = "camelCase")]`, so the tag
 * values are the camelCase form of the Rust variant names (e.g. `ReorderMeshs` ->
 * `"reorderMeshs"`), NOT the kebab-case `semanticKind` slugs this previously used for the tag
 * value. */
import type { GltfBindDefaultScenePayload } from './🔗️🎬️bind-default-scene/🟦️component.ts';
import type { GltfBindMorphTargetAttributePayload } from './🔗️🧬️bind-morph-target-attribute/🟦️component.ts';
import type { GltfBindNodeCameraPayload } from './🔗️🔘️bind-node-camera/🟦️component.ts';
import type { GltfBindNodeChildPayload } from './🔗️🔘️bind-node-child/🟦️component.ts';
import type { GltfBindNodeMeshPayload } from './🔗️🔘️bind-node-mesh/🟦️component.ts';
import type { GltfBindNodeSkinPayload } from './🔗️🔘️bind-node-skin/🟦️component.ts';
import type { GltfBindPrimitiveAttributePayload } from './🔗️🔺️bind-primitive-attribute/🟦️component.ts';
import type { GltfBindPrimitiveIndicesPayload } from './🔗️🔺️bind-primitive-indices/🟦️component.ts';
import type { GltfBindPrimitiveMaterialPayload } from './🔗️🔺️bind-primitive-material/🟦️component.ts';
import type { GltfBindSceneRootNodePayload } from './🔗️🎬️bind-scene-root-node/🟦️component.ts';
import type { GltfChangeAssetDescriptiveMetadataPayload } from './✏️📦️change-asset-descriptive-metadata/🟦️component.ts';
import type { GltfChangeAssetExtensionDataPayload } from './✏️📦️change-asset-extension-data/🟦️component.ts';
import type { GltfChangeAssetExtraDataPayload } from './✏️📦️change-asset-extra-data/🟦️component.ts';
import type { GltfChangeAssetVersionPayload } from './✏️📦️change-asset-version/🟦️component.ts';
import type { GltfChangeDocumentExtensionDataPayload } from './✏️📄️change-document-extension-data/🟦️component.ts';
import type { GltfChangeDocumentExtraDataPayload } from './✏️📄️change-document-extra-data/🟦️component.ts';
import type { GltfChangeMaterialAlphaModePayload } from './✏️💎️change-material-alpha-mode/🟦️component.ts';
import type { GltfChangeMaterialDoubleSidedPayload } from './✏️💎️change-material-double-sided/🟦️component.ts';
import type { GltfChangeMeshExtensionDataPayload } from './✏️🕸️change-mesh-extension-data/🟦️component.ts';
import type { GltfChangeMeshExtraDataPayload } from './✏️🕸️change-mesh-extra-data/🟦️component.ts';
import type { GltfChangeMeshMorphWeightsPayload } from './✏️🕸️change-mesh-morph-weights/🟦️component.ts';
import type { GltfChangeMeshNamePayload } from './✏️🕸️change-mesh-name/🟦️component.ts';
import type { GltfChangeNodeExtensionDataPayload } from './✏️🔘️change-node-extension-data/🟦️component.ts';
import type { GltfChangeNodeExtraDataPayload } from './✏️🔘️change-node-extra-data/🟦️component.ts';
import type { GltfChangeNodeMorphWeightsPayload } from './✏️🔘️change-node-morph-weights/🟦️component.ts';
import type { GltfChangeNodeNameMutation } from './✏️🔘️change-node-name/🟦️.ts';
import type { GltfChangePrimitiveExtensionDataPayload } from './✏️🔺️change-primitive-extension-data/🟦️component.ts';
import type { GltfChangePrimitiveExtraDataPayload } from './✏️🔺️change-primitive-extra-data/🟦️component.ts';
import type { GltfChangePrimitiveTopologyModePayload } from './✏️🔺️change-primitive-topology-mode/🟦️component.ts';
import type { GltfChangeSceneExtensionDataPayload } from './✏️🎬️change-scene-extension-data/🟦️component.ts';
import type { GltfChangeSceneExtraDataPayload } from './✏️🎬️change-scene-extra-data/🟦️component.ts';
import type { GltfChangeSceneNamePayload } from './✏️🎬️change-scene-name/🟦️component.ts';
import type { GltfCreateAccessorPayload } from './🌱️📐️create-accessor/🟦️component.ts';
import type { GltfCreateAnimationPayload } from './🌱️🎞️create-animation/🟦️component.ts';
import type { GltfCreateBufferPayload } from './🌱️💾️create-buffer/🟦️component.ts';
import type { GltfCreateBufferViewPayload } from './🌱️👁️create-buffer-view/🟦️component.ts';
import type { GltfCreateCameraPayload } from './🌱️🎥️create-camera/🟦️component.ts';
import type { GltfCreateImagePayload } from './🌱️🖼️create-image/🟦️component.ts';
import type { GltfCreateMaterialPayload } from './🌱️💎️create-material/🟦️component.ts';
import type { GltfCreateMeshPayload } from './🌱️🕸️create-mesh/🟦️component.ts';
import type { GltfCreateMorphTargetPayload } from './🌱️🧬️create-morph-target/🟦️component.ts';
import type { GltfCreateNodePayload } from './🌱️🔘️create-node/🟦️component.ts';
import type { GltfCreatePrimitivePayload } from './🌱️🔺️create-primitive/🟦️component.ts';
import type { GltfCreateSamplerPayload } from './🌱️🎛️create-sampler/🟦️component.ts';
import type { GltfCreateScenePayload } from './🌱️🎬️create-scene/🟦️component.ts';
import type { GltfCreateSkinPayload } from './🌱️🧥️create-skin/🟦️component.ts';
import type { GltfCreateTexturePayload } from './🌱️🎨️create-texture/🟦️component.ts';
import type { GltfDeclareUsedExtensionPayload } from './📣️🧩️add-used-extension/🟦️component.ts';
import type { GltfDeleteAccessorPayload } from './🗑️📐️delete-accessor/🟦️component.ts';
import type { GltfDeleteAnimationPayload } from './🗑️🎞️delete-animation/🟦️component.ts';
import type { GltfDeleteBufferPayload } from './🗑️💾️delete-buffer/🟦️component.ts';
import type { GltfDeleteBufferViewPayload } from './🗑️👁️delete-buffer-view/🟦️component.ts';
import type { GltfDeleteCameraPayload } from './🗑️🎥️delete-camera/🟦️component.ts';
import type { GltfDeleteImagePayload } from './🗑️🖼️delete-image/🟦️component.ts';
import type { GltfDeleteMaterialPayload } from './🗑️💎️delete-material/🟦️component.ts';
import type { GltfDeleteMeshPayload } from './🗑️🕸️delete-mesh/🟦️component.ts';
import type { GltfDeleteMorphTargetPayload } from './🗑️🧬️delete-morph-target/🟦️component.ts';
import type { GltfDeleteNodePayload } from './🗑️🔘️delete-node/🟦️component.ts';
import type { GltfDeletePrimitivePayload } from './🗑️🔺️delete-primitive/🟦️component.ts';
import type { GltfDeleteSamplerPayload } from './🗑️🎛️delete-sampler/🟦️component.ts';
import type { GltfDeleteScenePayload } from './🗑️🎬️delete-scene/🟦️component.ts';
import type { GltfDeleteSkinPayload } from './🗑️🧥️delete-skin/🟦️component.ts';
import type { GltfDeleteTexturePayload } from './🗑️🎨️delete-texture/🟦️component.ts';
import type { GltfMoveAccessorPayload } from './🚚️📐️move-accessor/🟦️component.ts';
import type { GltfMoveAnimationPayload } from './🚚️🎞️move-animation/🟦️component.ts';
import type { GltfMoveBufferPayload } from './🚚️💾️move-buffer/🟦️component.ts';
import type { GltfMoveBufferViewPayload } from './🚚️👁️move-buffer-view/🟦️component.ts';
import type { GltfMoveCameraPayload } from './🚚️🎥️move-camera/🟦️component.ts';
import type { GltfMoveImagePayload } from './🚚️🖼️move-image/🟦️component.ts';
import type { GltfMoveMaterialPayload } from './🚚️💎️move-material/🟦️component.ts';
import type { GltfMoveMeshPayload } from './🚚️🕸️move-mesh/🟦️component.ts';
import type { GltfMoveMorphTargetPayload } from './🚚️🧬️move-morph-target/🟦️component.ts';
import type { GltfMoveMorphTargetAttributePayload } from './🚚️🧬️move-morph-target-attribute/🟦️component.ts';
import type { GltfMoveNodePayload } from './🚚️🔘️move-node/🟦️component.ts';
import type { GltfMoveNodeChildPayload } from './🚚️🔘️move-node-child/🟦️component.ts';
import type { GltfMovePrimitivePayload } from './🚚️🔺️move-primitive/🟦️component.ts';
import type { GltfMovePrimitiveAttributePayload } from './🚚️🔺️move-primitive-attribute/🟦️component.ts';
import type { GltfMoveRequiredExtensionPayload } from './🚚️🧩️move-required-extension/🟦️component.ts';
import type { GltfMoveSamplerPayload } from './🚚️🎛️move-sampler/🟦️component.ts';
import type { GltfMoveScenePayload } from './🚚️🎬️move-scene/🟦️component.ts';
import type { GltfMoveSceneRootNodePayload } from './🚚️🎬️move-scene-root-node/🟦️component.ts';
import type { GltfMoveSkinPayload } from './🚚️🧥️move-skin/🟦️component.ts';
import type { GltfMoveTexturePayload } from './🚚️🎨️move-texture/🟦️component.ts';
import type { GltfMoveUsedExtensionPayload } from './🚚️🧩️move-used-extension/🟦️component.ts';
import type { GltfReorderAccessorsPayload } from './🔀️📐️reorder-accessors/🟦️component.ts';
import type { GltfReorderAnimationsPayload } from './🔀️🎞️reorder-animations/🟦️component.ts';
import type { GltfReorderBufferViewsPayload } from './🔀️👁️reorder-buffer-views/🟦️component.ts';
import type { GltfReorderBuffersPayload } from './🔀️💾️reorder-buffers/🟦️component.ts';
import type { GltfReorderCamerasPayload } from './🔀️🎥️reorder-cameras/🟦️component.ts';
import type { GltfReorderImagesPayload } from './🔀️🖼️reorder-images/🟦️component.ts';
import type { GltfReorderMaterialsPayload } from './🔀️💎️reorder-materials/🟦️component.ts';
import type { GltfReorderMeshsPayload } from './🔀️🕸️reorder-meshs/🟦️component.ts';
import type { GltfReorderMorphTargetAttributesPayload } from './🔀️🧬️reorder-morph-target-attributes/🟦️component.ts';
import type { GltfReorderMorphTargetsPayload } from './🔀️🧬️reorder-morph-targets/🟦️component.ts';
import type { GltfReorderNodeChildrenPayload } from './🔀️🔘️reorder-node-children/🟦️component.ts';
import type { GltfReorderNodesPayload } from './🔀️🔘️reorder-nodes/🟦️component.ts';
import type { GltfReorderPrimitiveAttributesPayload } from './🔀️🔺️reorder-primitive-attributes/🟦️component.ts';
import type { GltfReorderPrimitivesPayload } from './🔀️🔺️reorder-primitives/🟦️component.ts';
import type { GltfReorderRequiredExtensionsPayload } from './🔀️🧩️reorder-required-extensions/🟦️component.ts';
import type { GltfReorderSamplersPayload } from './🔀️🎛️reorder-samplers/🟦️component.ts';
import type { GltfReorderSceneRootNodesPayload } from './🔀️🎬️reorder-scene-root-nodes/🟦️component.ts';
import type { GltfReorderScenesPayload } from './🔀️🎬️reorder-scenes/🟦️component.ts';
import type { GltfReorderSkinsPayload } from './🔀️🧥️reorder-skins/🟦️component.ts';
import type { GltfReorderTexturesPayload } from './🔀️🎨️reorder-textures/🟦️component.ts';
import type { GltfReorderUsedExtensionsPayload } from './🔀️🧩️reorder-used-extensions/🟦️component.ts';
import type { GltfReparentNodePayload } from './👪️🔘️move-node-parent/🟦️component.ts';
import type { GltfRequireExtensionPayload } from './✅️🧩️add-required-extension/🟦️component.ts';
import type { GltfTransformNodePayload } from './🔄️🔘️change-node-transform/🟦️component.ts';
import type { GltfUnbindDefaultScenePayload } from './✂️🎬️unbind-default-scene/🟦️component.ts';
import type { GltfUnbindMorphTargetAttributePayload } from './✂️🧬️unbind-morph-target-attribute/🟦️component.ts';
import type { GltfUnbindNodeCameraPayload } from './✂️🔘️unbind-node-camera/🟦️component.ts';
import type { GltfUnbindNodeChildPayload } from './✂️🔘️unbind-node-child/🟦️component.ts';
import type { GltfUnbindNodeMeshPayload } from './✂️🔘️unbind-node-mesh/🟦️component.ts';
import type { GltfUnbindNodeSkinPayload } from './✂️🔘️unbind-node-skin/🟦️component.ts';
import type { GltfUnbindPrimitiveAttributePayload } from './✂️🔺️unbind-primitive-attribute/🟦️component.ts';
import type { GltfUnbindPrimitiveIndicesPayload } from './✂️🔺️unbind-primitive-indices/🟦️component.ts';
import type { GltfUnbindPrimitiveMaterialPayload } from './✂️🔺️unbind-primitive-material/🟦️component.ts';
import type { GltfUnbindSceneRootNodePayload } from './✂️🎬️unbind-scene-root-node/🟦️component.ts';
import type { GltfUnrequireExtensionPayload } from './🚫️🧩️remove-required-extension/🟦️component.ts';
import type { GltfWithdrawUsedExtensionPayload } from './🔙️🧩️remove-used-extension/🟦️component.ts';

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
