/** 🎞️ `GltfAnimationMutation` twin: the animation slice of the glTF 2.0 mutation vocabulary, a view over the any subset's `GltfMutation`
 * that selects branches and never restates a payload.
 * @see ./🔣️.json */
import { gltfWireRefuse, type GltfWireReader } from "../../../♾️any/🧬️schema/📸️snapshot/🟦️.ts";
import { parseGltfMutation, type GltfMutation } from "../../../♾️any/🧬️schema/🧬️mutations/🟦️.ts";

export type GltfAnimationMutation = Extract<GltfMutation, { readonly mutation: "createAnimation" | "deleteAnimation" | "moveAnimation" | "reorderAnimations" }>;

const members: readonly GltfMutation["mutation"][] = ["createAnimation", "deleteAnimation", "moveAnimation", "reorderAnimations"];

export const parseGltfAnimationMutation: GltfWireReader<GltfAnimationMutation> = (value, at = "$") => {
  const mutation = parseGltfMutation(value, at);
  return members.includes(mutation.mutation) ? (mutation as GltfAnimationMutation) : gltfWireRefuse(`${at}.mutation`, `value is not one of ${members.join(", ")}`);
};
