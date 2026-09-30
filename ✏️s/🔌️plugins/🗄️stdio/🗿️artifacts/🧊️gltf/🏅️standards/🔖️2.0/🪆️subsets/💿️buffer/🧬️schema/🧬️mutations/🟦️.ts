/** 💿️ `GltfBufferMutation` twin: the buffer slice of the glTF 2.0 mutation vocabulary, a view over the any subset's `GltfMutation`
 * that selects branches and never restates a payload.
 * @see ./🔣️.json */
import { gltfWireRefuse, type GltfWireReader } from "../../../♾️any/🧬️schema/📸️snapshot/🟦️.ts";
import { parseGltfMutation, type GltfMutation } from "../../../♾️any/🧬️schema/🧬️mutations/🟦️.ts";

export type GltfBufferMutation = Extract<GltfMutation, { readonly mutation: "createBuffer" | "createBufferView" | "deleteBuffer" | "deleteBufferView" | "moveBuffer" | "moveBufferView" | "reorderBufferViews" | "reorderBuffers" }>;

const members: readonly GltfMutation["mutation"][] = ["createBuffer", "createBufferView", "deleteBuffer", "deleteBufferView", "moveBuffer", "moveBufferView", "reorderBufferViews", "reorderBuffers"];

export const parseGltfBufferMutation: GltfWireReader<GltfBufferMutation> = (value, at = "$") => {
  const mutation = parseGltfMutation(value, at);
  return members.includes(mutation.mutation) ? (mutation as GltfBufferMutation) : gltfWireRefuse(`${at}.mutation`, `value is not one of ${members.join(", ")}`);
};
