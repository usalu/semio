/** ✂️ `unbind-node-skin` wire twin: the flat `Apply` payload `GltfUnbindNodeSkinPayload` and the phase wire `UnbindNodeSkinMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfUnbindNodeSkinPayload {
  node: bigint;
}

export type UnbindNodeSkinMutation = GltfPhase<GltfUnbindNodeSkinPayload, GltfDiff>;

export const parseGltfUnbindNodeSkinPayload = gltfWireObject<GltfUnbindNodeSkinPayload>({ node: gltfWireRequired(gltfWireIndex) });
export const parseUnbindNodeSkinMutation = gltfWirePhase(parseGltfUnbindNodeSkinPayload, parseGltfDiff);
