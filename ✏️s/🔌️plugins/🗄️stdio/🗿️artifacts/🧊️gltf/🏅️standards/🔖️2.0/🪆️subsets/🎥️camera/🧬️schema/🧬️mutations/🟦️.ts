/** 🎥️ `GltfCameraMutation` twin: the camera slice of the glTF 2.0 mutation vocabulary, a view over the any subset's `GltfMutation`
 * that selects branches and never restates a payload.
 * @see ./🔣️.json */
import { gltfWireRefuse, type GltfWireReader } from "../../../♾️any/🧬️schema/📸️snapshot/🟦️.ts";
import { parseGltfMutation, type GltfMutation } from "../../../♾️any/🧬️schema/🧬️mutations/🟦️.ts";

export type GltfCameraMutation = Extract<GltfMutation, { readonly mutation: "createCamera" | "deleteCamera" | "moveCamera" | "reorderCameras" }>;

const members: readonly GltfMutation["mutation"][] = ["createCamera", "deleteCamera", "moveCamera", "reorderCameras"];

export const parseGltfCameraMutation: GltfWireReader<GltfCameraMutation> = (value, at = "$") => {
  const mutation = parseGltfMutation(value, at);
  return members.includes(mutation.mutation) ? (mutation as GltfCameraMutation) : gltfWireRefuse(`${at}.mutation`, `value is not one of ${members.join(", ")}`);
};
