/** 🚚️ `move-mesh` wire twin: the flat `Apply` payload `GltfMoveMeshPayload` and the phase wire `MoveMeshMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfMoveMeshPayload {
  index: bigint;
  position: bigint;
}

export type MoveMeshMutation = GltfApplyPhase<GltfMoveMeshPayload>;

export const parseGltfMoveMeshPayload = gltfWireObject<GltfMoveMeshPayload>({ index: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveMeshMutation = gltfWireApplyPhase(parseGltfMoveMeshPayload);
