/** ✂️ `unbind-node-mesh` wire twin: the flat `Apply` payload `GltfUnbindNodeMeshPayload` and the phase wire `UnbindNodeMeshMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfUnbindNodeMeshPayload {
  node: bigint;
}

export type UnbindNodeMeshMutation = GltfApplyPhase<GltfUnbindNodeMeshPayload>;

export const parseGltfUnbindNodeMeshPayload = gltfWireObject<GltfUnbindNodeMeshPayload>({ node: gltfWireRequired(gltfWireIndex) });
export const parseUnbindNodeMeshMutation = gltfWireApplyPhase(parseGltfUnbindNodeMeshPayload);
