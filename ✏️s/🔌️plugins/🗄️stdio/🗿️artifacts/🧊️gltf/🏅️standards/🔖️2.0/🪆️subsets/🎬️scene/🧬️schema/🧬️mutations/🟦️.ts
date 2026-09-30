/** 🎬️ `GltfSceneMutation` twin: the scene slice of the glTF 2.0 mutation vocabulary, a view over the any subset's `GltfMutation`
 * that selects branches and never restates a payload.
 * @see ./🔣️.json */
import { gltfWireRefuse, type GltfWireReader } from "../../../♾️any/🧬️schema/📸️snapshot/🟦️.ts";
import { parseGltfMutation, type GltfMutation } from "../../../♾️any/🧬️schema/🧬️mutations/🟦️.ts";

export type GltfSceneMutation = Extract<GltfMutation, { readonly mutation: "bindDefaultScene" | "bindNodeCamera" | "bindNodeChild" | "bindNodeMesh" | "bindNodeSkin" | "bindSceneRootNode" | "changeNodeExtensionData" | "changeNodeExtraData" | "changeNodeMorphWeights" | "changeNodeName" | "changeSceneExtensionData" | "changeSceneExtraData" | "changeSceneName" | "createNode" | "createScene" | "deleteNode" | "deleteScene" | "moveNode" | "moveNodeChild" | "moveScene" | "moveSceneRootNode" | "reorderNodeChildren" | "reorderNodes" | "reorderSceneRootNodes" | "reorderScenes" | "moveNodeParent" | "changeNodeTransform" | "unbindDefaultScene" | "unbindNodeCamera" | "unbindNodeChild" | "unbindNodeMesh" | "unbindNodeSkin" | "unbindSceneRootNode" }>;

const members: readonly GltfMutation["mutation"][] = ["bindDefaultScene", "bindNodeCamera", "bindNodeChild", "bindNodeMesh", "bindNodeSkin", "bindSceneRootNode", "changeNodeExtensionData", "changeNodeExtraData", "changeNodeMorphWeights", "changeNodeName", "changeSceneExtensionData", "changeSceneExtraData", "changeSceneName", "createNode", "createScene", "deleteNode", "deleteScene", "moveNode", "moveNodeChild", "moveScene", "moveSceneRootNode", "reorderNodeChildren", "reorderNodes", "reorderSceneRootNodes", "reorderScenes", "moveNodeParent", "changeNodeTransform", "unbindDefaultScene", "unbindNodeCamera", "unbindNodeChild", "unbindNodeMesh", "unbindNodeSkin", "unbindSceneRootNode"];

export const parseGltfSceneMutation: GltfWireReader<GltfSceneMutation> = (value, at = "$") => {
  const mutation = parseGltfMutation(value, at);
  return members.includes(mutation.mutation) ? (mutation as GltfSceneMutation) : gltfWireRefuse(`${at}.mutation`, `value is not one of ${members.join(", ")}`);
};
