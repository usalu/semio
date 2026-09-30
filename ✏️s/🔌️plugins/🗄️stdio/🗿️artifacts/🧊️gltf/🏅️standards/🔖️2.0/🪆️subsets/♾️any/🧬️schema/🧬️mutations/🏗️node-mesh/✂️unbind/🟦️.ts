/** ✂️ `unbind-node-mesh` wire twin: the flat `Apply` payload `GltfUnbindNodeMeshPayload` and the phase wire `UnbindNodeMeshMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfUnbindNodeMeshPayload {
  node: number;
}

export type UnbindNodeMeshMutation = GltfPhase<GltfUnbindNodeMeshPayload, GltfDiff>;

export const parseGltfUnbindNodeMeshPayload = gltfWireObject<GltfUnbindNodeMeshPayload>({ node: gltfWireRequired(gltfWireIndex) });
export const parseUnbindNodeMeshMutation = gltfWirePhase(parseGltfUnbindNodeMeshPayload, parseGltfDiff);
