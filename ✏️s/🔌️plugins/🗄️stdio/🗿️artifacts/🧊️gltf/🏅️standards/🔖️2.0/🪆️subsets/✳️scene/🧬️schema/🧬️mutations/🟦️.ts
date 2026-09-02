/** 🧬 Transparent TypeScript aggregate for the scene slice of the glTF 2.0 mutation vocabulary. */
import type { GltfBindDefaultScenePayload } from '../../../✳️any/🧬️schema/🧬️mutations/🔗️🎬️bind-default-scene/🟦️.ts';
import type { GltfBindNodeCameraPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🔗️🔘️bind-node-camera/🟦️.ts';
import type { GltfBindNodeChildPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🔗️🔘️bind-node-child/🟦️.ts';
import type { GltfBindNodeMeshPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🔗️🔘️bind-node-mesh/🟦️.ts';
import type { GltfBindNodeSkinPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🔗️🔘️bind-node-skin/🟦️.ts';
import type { GltfBindSceneRootNodePayload } from '../../../✳️any/🧬️schema/🧬️mutations/🔗️🎬️bind-scene-root-node/🟦️.ts';
import type { GltfChangeNodeExtensionDataPayload } from '../../../✳️any/🧬️schema/🧬️mutations/✏️🔘️change-node-extension-data/🟦️.ts';
import type { GltfChangeNodeExtraDataPayload } from '../../../✳️any/🧬️schema/🧬️mutations/✏️🔘️change-node-extra-data/🟦️.ts';
import type { GltfChangeNodeMorphWeightsPayload } from '../../../✳️any/🧬️schema/🧬️mutations/✏️🔘️change-node-morph-weights/🟦️.ts';
import type { GltfChangeNodeNameMutation } from '../../../✳️any/🧬️schema/🧬️mutations/✏️🔘️change-node-name/🟦️.ts';
import type { GltfChangeSceneExtensionDataPayload } from '../../../✳️any/🧬️schema/🧬️mutations/✏️🎬️change-scene-extension-data/🟦️.ts';
import type { GltfChangeSceneExtraDataPayload } from '../../../✳️any/🧬️schema/🧬️mutations/✏️🎬️change-scene-extra-data/🟦️.ts';
import type { GltfChangeSceneNamePayload } from '../../../✳️any/🧬️schema/🧬️mutations/✏️🎬️change-scene-name/🟦️.ts';
import type { GltfCreateNodePayload } from '../../../✳️any/🧬️schema/🧬️mutations/🌱️🔘️create-node/🟦️.ts';
import type { GltfCreateScenePayload } from '../../../✳️any/🧬️schema/🧬️mutations/🌱️🎬️create-scene/🟦️.ts';
import type { GltfDeleteNodePayload } from '../../../✳️any/🧬️schema/🧬️mutations/🗑️🔘️delete-node/🟦️.ts';
import type { GltfDeleteScenePayload } from '../../../✳️any/🧬️schema/🧬️mutations/🗑️🎬️delete-scene/🟦️.ts';
import type { GltfMoveNodeChildPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🚚️🔘️move-node-child/🟦️.ts';
import type { GltfMoveNodePayload } from '../../../✳️any/🧬️schema/🧬️mutations/🚚️🔘️move-node/🟦️.ts';
import type { GltfMoveScenePayload } from '../../../✳️any/🧬️schema/🧬️mutations/🚚️🎬️move-scene/🟦️.ts';
import type { GltfMoveSceneRootNodePayload } from '../../../✳️any/🧬️schema/🧬️mutations/🚚️🎬️move-scene-root-node/🟦️.ts';
import type { GltfReorderNodeChildrenPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🔀️🔘️reorder-node-children/🟦️.ts';
import type { GltfReorderNodesPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🔀️🔘️reorder-nodes/🟦️.ts';
import type { GltfReorderSceneRootNodesPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🔀️🎬️reorder-scene-root-nodes/🟦️.ts';
import type { GltfReorderScenesPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🔀️🎬️reorder-scenes/🟦️.ts';
import type { GltfReparentNodePayload } from '../../../✳️any/🧬️schema/🧬️mutations/👪️🔘️move-node-parent/🟦️.ts';
import type { GltfTransformNodePayload } from '../../../✳️any/🧬️schema/🧬️mutations/🔄️🔘️change-node-transform/🟦️.ts';
import type { GltfUnbindDefaultScenePayload } from '../../../✳️any/🧬️schema/🧬️mutations/✂️🎬️unbind-default-scene/🟦️.ts';
import type { GltfUnbindNodeCameraPayload } from '../../../✳️any/🧬️schema/🧬️mutations/✂️🔘️unbind-node-camera/🟦️.ts';
import type { GltfUnbindNodeChildPayload } from '../../../✳️any/🧬️schema/🧬️mutations/✂️🔘️unbind-node-child/🟦️.ts';
import type { GltfUnbindNodeMeshPayload } from '../../../✳️any/🧬️schema/🧬️mutations/✂️🔘️unbind-node-mesh/🟦️.ts';
import type { GltfUnbindNodeSkinPayload } from '../../../✳️any/🧬️schema/🧬️mutations/✂️🔘️unbind-node-skin/🟦️.ts';
import type { GltfUnbindSceneRootNodePayload } from '../../../✳️any/🧬️schema/🧬️mutations/✂️🎬️unbind-scene-root-node/🟦️.ts';

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
