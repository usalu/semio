/** 🦴️ `GltfSkinMutation` twin: the skin slice of the glTF 2.0 mutation vocabulary, a view over the any subset's `GltfMutation`
 * that selects branches and never restates a payload.
 * @see ./🔣️.json */
import { gltfWireRefuse, type GltfWireReader } from "../../../♾️any/🧬️schema/📸️snapshot/🟦️.ts";
import { parseGltfMutation, type GltfMutation } from "../../../♾️any/🧬️schema/🧬️mutations/🟦️.ts";

export type GltfSkinMutation = Extract<GltfMutation, { readonly mutation: "createSkin" | "deleteSkin" | "moveSkin" | "reorderSkins" }>;

const members: readonly GltfMutation["mutation"][] = ["createSkin", "deleteSkin", "moveSkin", "reorderSkins"];

export const parseGltfSkinMutation: GltfWireReader<GltfSkinMutation> = (value, at = "$") => {
  const mutation = parseGltfMutation(value, at);
  return members.includes(mutation.mutation) ? (mutation as GltfSkinMutation) : gltfWireRefuse(`${at}.mutation`, `value is not one of ${members.join(", ")}`);
};
