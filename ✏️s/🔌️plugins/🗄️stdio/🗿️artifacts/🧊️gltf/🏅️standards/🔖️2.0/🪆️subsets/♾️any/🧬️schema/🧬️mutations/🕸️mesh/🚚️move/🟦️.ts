/** 🚚️ `move-mesh` wire twin: the flat `Apply` payload `GltfMoveMeshPayload` and the phase wire `MoveMeshMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfMoveMeshPayload {
  index: bigint;
  position: bigint;
}

export type MoveMeshMutation = GltfPhase<GltfMoveMeshPayload, GltfDiff>;

export const parseGltfMoveMeshPayload = gltfWireObject<GltfMoveMeshPayload>({ index: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveMeshMutation = gltfWirePhase(parseGltfMoveMeshPayload, parseGltfDiff);
