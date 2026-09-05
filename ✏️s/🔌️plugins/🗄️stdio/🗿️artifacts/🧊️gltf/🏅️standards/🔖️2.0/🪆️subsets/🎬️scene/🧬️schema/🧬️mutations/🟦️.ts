/** 🧬 Transparent TypeScript aggregate for the scene slice of the glTF 2.0 mutation vocabulary. */
import type { GltfBindDefaultScenePayload } from '../../../♾️any/🧬️schema/🧬️mutations/🏠️default-scene/🔗️bind/🟦️.ts';
import type { GltfBindNodeCameraPayload } from '../../../♾️any/🧬️schema/🧬️mutations/📷️node-camera/🔗️bind/🟦️.ts';
import type { GltfBindNodeChildPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🌿️node-child/🔗️bind/🟦️.ts';
import type { GltfBindNodeMeshPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🏗️node-mesh/🔗️bind/🟦️.ts';
import type { GltfBindNodeSkinPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🩻️node-skin/🔗️bind/🟦️.ts';
import type { GltfBindSceneRootNodePayload } from '../../../♾️any/🧬️schema/🧬️mutations/🌲️scene-root/🔗️bind/🟦️.ts';
import type { GltfChangeNodeExtensionDataPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🌳️node/🧩️change-extensions/🟦️.ts';
import type { GltfChangeNodeExtraDataPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🌳️node/📝️change-extras/🟦️.ts';
import type { GltfChangeNodeMorphWeightsPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🌳️node/⚖️change-weights/🟦️.ts';
import type { GltfChangeNodeNameMutation } from '../../../♾️any/🧬️schema/🧬️mutations/🌳️node/🏷️rename/🟦️.ts';
import type { GltfChangeSceneExtensionDataPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🎬️scene/🧩️change-extensions/🟦️.ts';
import type { GltfChangeSceneExtraDataPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🎬️scene/📝️change-extras/🟦️.ts';
import type { GltfChangeSceneNamePayload } from '../../../♾️any/🧬️schema/🧬️mutations/🎬️scene/🏷️rename/🟦️.ts';
import type { GltfCreateNodePayload } from '../../../♾️any/🧬️schema/🧬️mutations/🌳️node/🌱️create/🟦️.ts';
import type { GltfCreateScenePayload } from '../../../♾️any/🧬️schema/🧬️mutations/🎬️scene/🌱️create/🟦️.ts';
import type { GltfDeleteNodePayload } from '../../../♾️any/🧬️schema/🧬️mutations/🌳️node/🗑️delete/🟦️.ts';
import type { GltfDeleteScenePayload } from '../../../♾️any/🧬️schema/🧬️mutations/🎬️scene/🗑️delete/🟦️.ts';
import type { GltfMoveNodeChildPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🌿️node-child/🚚️move/🟦️.ts';
import type { GltfMoveNodePayload } from '../../../♾️any/🧬️schema/🧬️mutations/🌳️node/🚚️move/🟦️.ts';
import type { GltfMoveScenePayload } from '../../../♾️any/🧬️schema/🧬️mutations/🎬️scene/🚚️move/🟦️.ts';
import type { GltfMoveSceneRootNodePayload } from '../../../♾️any/🧬️schema/🧬️mutations/🌲️scene-root/🚚️move/🟦️.ts';
import type { GltfReorderNodeChildrenPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🌿️node-child/🔀️reorder/🟦️.ts';
import type { GltfReorderNodesPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🌳️node/🔀️reorder/🟦️.ts';
import type { GltfReorderSceneRootNodesPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🌲️scene-root/🔀️reorder/🟦️.ts';
import type { GltfReorderScenesPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🎬️scene/🔀️reorder/🟦️.ts';
import type { GltfReparentNodePayload } from '../../../♾️any/🧬️schema/🧬️mutations/🌳️node/🌿️reparent/🟦️.ts';
import type { GltfTransformNodePayload } from '../../../♾️any/🧬️schema/🧬️mutations/🌳️node/📐️transform/🟦️.ts';
import type { GltfUnbindDefaultScenePayload } from '../../../♾️any/🧬️schema/🧬️mutations/🏠️default-scene/✂️unbind/🟦️.ts';
import type { GltfUnbindNodeCameraPayload } from '../../../♾️any/🧬️schema/🧬️mutations/📷️node-camera/✂️unbind/🟦️.ts';
import type { GltfUnbindNodeChildPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🌿️node-child/✂️unbind/🟦️.ts';
import type { GltfUnbindNodeMeshPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🏗️node-mesh/✂️unbind/🟦️.ts';
import type { GltfUnbindNodeSkinPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🩻️node-skin/✂️unbind/🟦️.ts';
import type { GltfUnbindSceneRootNodePayload } from '../../../♾️any/🧬️schema/🧬️mutations/🌲️scene-root/✂️unbind/🟦️.ts';

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
