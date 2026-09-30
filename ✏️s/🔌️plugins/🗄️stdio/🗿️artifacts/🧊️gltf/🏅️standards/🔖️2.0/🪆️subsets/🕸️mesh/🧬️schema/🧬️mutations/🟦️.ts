/** 🕸️ `GltfMeshMutation` twin: the mesh slice of the glTF 2.0 mutation vocabulary, a view over the any subset's `GltfMutation`
 * that selects branches and never restates a payload.
 * @see ./🔣️.json */
import { gltfWireRefuse, type GltfWireReader } from "../../../♾️any/🧬️schema/📸️snapshot/🟦️.ts";
import { parseGltfMutation, type GltfMutation } from "../../../♾️any/🧬️schema/🧬️mutations/🟦️.ts";

export type GltfMeshMutation = Extract<GltfMutation, { readonly mutation: "bindMorphTargetAttribute" | "bindPrimitiveAttribute" | "bindPrimitiveIndices" | "bindPrimitiveMaterial" | "changeMeshExtensionData" | "changeMeshExtraData" | "changeMeshMorphWeights" | "changeMeshName" | "changePrimitiveExtensionData" | "changePrimitiveExtraData" | "changePrimitiveTopologyMode" | "createAccessor" | "createMesh" | "createMorphTarget" | "createPrimitive" | "deleteAccessor" | "deleteMesh" | "deleteMorphTarget" | "deletePrimitive" | "moveAccessor" | "moveMesh" | "moveMorphTarget" | "moveMorphTargetAttribute" | "movePrimitive" | "movePrimitiveAttribute" | "reorderAccessors" | "reorderMeshs" | "reorderMorphTargetAttributes" | "reorderMorphTargets" | "reorderPrimitiveAttributes" | "reorderPrimitives" | "unbindMorphTargetAttribute" | "unbindPrimitiveAttribute" | "unbindPrimitiveIndices" | "unbindPrimitiveMaterial" }>;

const members: readonly GltfMutation["mutation"][] = ["bindMorphTargetAttribute", "bindPrimitiveAttribute", "bindPrimitiveIndices", "bindPrimitiveMaterial", "changeMeshExtensionData", "changeMeshExtraData", "changeMeshMorphWeights", "changeMeshName", "changePrimitiveExtensionData", "changePrimitiveExtraData", "changePrimitiveTopologyMode", "createAccessor", "createMesh", "createMorphTarget", "createPrimitive", "deleteAccessor", "deleteMesh", "deleteMorphTarget", "deletePrimitive", "moveAccessor", "moveMesh", "moveMorphTarget", "moveMorphTargetAttribute", "movePrimitive", "movePrimitiveAttribute", "reorderAccessors", "reorderMeshs", "reorderMorphTargetAttributes", "reorderMorphTargets", "reorderPrimitiveAttributes", "reorderPrimitives", "unbindMorphTargetAttribute", "unbindPrimitiveAttribute", "unbindPrimitiveIndices", "unbindPrimitiveMaterial"];

export const parseGltfMeshMutation: GltfWireReader<GltfMeshMutation> = (value, at = "$") => {
  const mutation = parseGltfMutation(value, at);
  return members.includes(mutation.mutation) ? (mutation as GltfMeshMutation) : gltfWireRefuse(`${at}.mutation`, `value is not one of ${members.join(", ")}`);
};
