/** 💎️ `GltfMaterialMutation` twin: the material slice of the glTF 2.0 mutation vocabulary, a view over the any subset's `GltfMutation`
 * that selects branches and never restates a payload.
 * @see ./🔣️.json */
import { gltfWireRefuse, type GltfWireReader } from "../../../♾️any/🧬️schema/📸️snapshot/🟦️.ts";
import { parseGltfMutation, type GltfMutation } from "../../../♾️any/🧬️schema/🧬️mutations/🟦️.ts";

export type GltfMaterialMutation = Extract<GltfMutation, { readonly mutation: "changeMaterialAlphaMode" | "changeMaterialDoubleSided" | "createImage" | "createMaterial" | "createSampler" | "createTexture" | "deleteImage" | "deleteMaterial" | "deleteSampler" | "deleteTexture" | "moveImage" | "moveMaterial" | "moveSampler" | "moveTexture" | "reorderImages" | "reorderMaterials" | "reorderSamplers" | "reorderTextures" }>;

const members: readonly GltfMutation["mutation"][] = ["changeMaterialAlphaMode", "changeMaterialDoubleSided", "createImage", "createMaterial", "createSampler", "createTexture", "deleteImage", "deleteMaterial", "deleteSampler", "deleteTexture", "moveImage", "moveMaterial", "moveSampler", "moveTexture", "reorderImages", "reorderMaterials", "reorderSamplers", "reorderTextures"];

export const parseGltfMaterialMutation: GltfWireReader<GltfMaterialMutation> = (value, at = "$") => {
  const mutation = parseGltfMutation(value, at);
  return members.includes(mutation.mutation) ? (mutation as GltfMaterialMutation) : gltfWireRefuse(`${at}.mutation`, `value is not one of ${members.join(", ")}`);
};
