/** 🧬 Transparent TypeScript aggregate for the scene slice of the glTF 2.0 mutation vocabulary. */
import type { GltfBindDefaultScenePayload } from './🔗️🎬️bind-default-scene/🟦️.ts';
import type { GltfBindNodeCameraPayload } from './🔗️🔘️bind-node-camera/🟦️.ts';
import type { GltfBindNodeChildPayload } from './🔗️🔘️bind-node-child/🟦️.ts';
import type { GltfBindNodeMeshPayload } from './🔗️🔘️bind-node-mesh/🟦️.ts';
import type { GltfBindNodeSkinPayload } from './🔗️🔘️bind-node-skin/🟦️.ts';
import type { GltfBindSceneRootNodePayload } from './🔗️🎬️bind-scene-root-node/🟦️.ts';
import type { GltfChangeNodeExtensionDataPayload } from './✏️🔘️change-node-extension-data/🟦️.ts';
import type { GltfChangeNodeExtraDataPayload } from './✏️🔘️change-node-extra-data/🟦️.ts';
import type { GltfChangeNodeMorphWeightsPayload } from './✏️🔘️change-node-morph-weights/🟦️.ts';
import type { GltfChangeNodeNameMutation } from './✏️🔘️change-node-name/🟦️.ts';
import type { GltfChangeSceneExtensionDataPayload } from './✏️🎬️change-scene-extension-data/🟦️.ts';
import type { GltfChangeSceneExtraDataPayload } from './✏️🎬️change-scene-extra-data/🟦️.ts';
import type { GltfChangeSceneNamePayload } from './✏️🎬️change-scene-name/🟦️.ts';
import type { GltfCreateNodePayload } from './🌱️🔘️create-node/🟦️.ts';
import type { GltfCreateScenePayload } from './🌱️🎬️create-scene/🟦️.ts';
import type { GltfDeleteNodePayload } from './🗑️🔘️delete-node/🟦️.ts';
import type { GltfDeleteScenePayload } from './🗑️🎬️delete-scene/🟦️.ts';
import type { GltfMoveNodeChildPayload } from './🚚️🔘️move-node-child/🟦️.ts';
import type { GltfMoveNodePayload } from './🚚️🔘️move-node/🟦️.ts';
import type { GltfMoveScenePayload } from './🚚️🎬️move-scene/🟦️.ts';
import type { GltfMoveSceneRootNodePayload } from './🚚️🎬️move-scene-root-node/🟦️.ts';
import type { GltfReorderNodeChildrenPayload } from './🔀️🔘️reorder-node-children/🟦️.ts';
import type { GltfReorderNodesPayload } from './🔀️🔘️reorder-nodes/🟦️.ts';
import type { GltfReorderSceneRootNodesPayload } from './🔀️🎬️reorder-scene-root-nodes/🟦️.ts';
import type { GltfReorderScenesPayload } from './🔀️🎬️reorder-scenes/🟦️.ts';
import type { GltfReparentNodePayload } from './👪️🔘️move-node-parent/🟦️.ts';
import type { GltfTransformNodePayload } from './🔄️🔘️change-node-transform/🟦️.ts';
import type { GltfUnbindDefaultScenePayload } from './✂️🎬️unbind-default-scene/🟦️.ts';
import type { GltfUnbindNodeCameraPayload } from './✂️🔘️unbind-node-camera/🟦️.ts';
import type { GltfUnbindNodeChildPayload } from './✂️🔘️unbind-node-child/🟦️.ts';
import type { GltfUnbindNodeMeshPayload } from './✂️🔘️unbind-node-mesh/🟦️.ts';
import type { GltfUnbindNodeSkinPayload } from './✂️🔘️unbind-node-skin/🟦️.ts';
import type { GltfUnbindSceneRootNodePayload } from './✂️🎬️unbind-scene-root-node/🟦️.ts';

export type GltfSceneMutation =
  | { readonly mutation: 'unbindDefaultScene'; readonly payload: GltfUnbindDefaultScenePayload }
  | { readonly mutation: 'unbindSceneRootNode'; readonly payload: GltfUnbindSceneRootNodePayload }
  | { readonly mutation: 'unbindNodeCamera'; readonly payload: GltfUnbindNodeCameraPayload }
  | { readonly mutation: 'unbindNodeChild'; readonly payload: GltfUnbindNodeChildPayload }
  | { readonly mutation: 'unbindNodeMesh'; readonly payload: GltfUnbindNodeMeshPayload }
  | { readonly mutation: 'unbindNodeSkin'; readonly payload: GltfUnbindNodeSkinPayload }
  | { readonly mutation: 'changeSceneExtensionData'; readonly payload: GltfChangeSceneExtensionDataPayload }
  | { readonly mutation: 'changeSceneExtraData'; readonly payload: GltfChangeSceneExtraDataPayload }
  | { readonly mutation: 'changeSceneName'; readonly payload: GltfChangeSceneNamePayload }
  | { readonly mutation: 'changeNodeExtensionData'; readonly payload: GltfChangeNodeExtensionDataPayload }
  | { readonly mutation: 'changeNodeExtraData'; readonly payload: GltfChangeNodeExtraDataPayload }
  | { readonly mutation: 'changeNodeMorphWeights'; readonly payload: GltfChangeNodeMorphWeightsPayload }
  | { readonly mutation: 'changeNodeName'; readonly payload: GltfChangeNodeNameMutation }
  | { readonly mutation: 'createScene'; readonly payload: GltfCreateScenePayload }
  | { readonly mutation: 'createNode'; readonly payload: GltfCreateNodePayload }
  | { readonly mutation: 'moveNodeParent'; readonly payload: GltfReparentNodePayload }
  | { readonly mutation: 'reorderSceneRootNodes'; readonly payload: GltfReorderSceneRootNodesPayload }
  | { readonly mutation: 'reorderScenes'; readonly payload: GltfReorderScenesPayload }
  | { readonly mutation: 'reorderNodeChildren'; readonly payload: GltfReorderNodeChildrenPayload }
  | { readonly mutation: 'reorderNodes'; readonly payload: GltfReorderNodesPayload }
  | { readonly mutation: 'changeNodeTransform'; readonly payload: GltfTransformNodePayload }
  | { readonly mutation: 'bindDefaultScene'; readonly payload: GltfBindDefaultScenePayload }
  | { readonly mutation: 'bindSceneRootNode'; readonly payload: GltfBindSceneRootNodePayload }
  | { readonly mutation: 'bindNodeCamera'; readonly payload: GltfBindNodeCameraPayload }
  | { readonly mutation: 'bindNodeChild'; readonly payload: GltfBindNodeChildPayload }
  | { readonly mutation: 'bindNodeMesh'; readonly payload: GltfBindNodeMeshPayload }
  | { readonly mutation: 'bindNodeSkin'; readonly payload: GltfBindNodeSkinPayload }
  | { readonly mutation: 'deleteScene'; readonly payload: GltfDeleteScenePayload }
  | { readonly mutation: 'deleteNode'; readonly payload: GltfDeleteNodePayload }
  | { readonly mutation: 'moveScene'; readonly payload: GltfMoveScenePayload }
  | { readonly mutation: 'moveSceneRootNode'; readonly payload: GltfMoveSceneRootNodePayload }
  | { readonly mutation: 'moveNode'; readonly payload: GltfMoveNodePayload }
  | { readonly mutation: 'moveNodeChild'; readonly payload: GltfMoveNodeChildPayload };
