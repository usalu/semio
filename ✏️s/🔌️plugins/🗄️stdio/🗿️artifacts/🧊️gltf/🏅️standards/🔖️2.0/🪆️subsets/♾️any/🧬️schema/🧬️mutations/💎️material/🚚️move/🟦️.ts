/** 🚚️ `move-material` wire twin: the flat `Apply` payload `GltfMoveMaterialPayload` and the phase wire `MoveMaterialMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfMoveMaterialPayload {
  index: bigint;
  position: bigint;
}

export type MoveMaterialMutation = GltfPhase<GltfMoveMaterialPayload, GltfDiff>;

export const parseGltfMoveMaterialPayload = gltfWireObject<GltfMoveMaterialPayload>({ index: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveMaterialMutation = gltfWirePhase(parseGltfMoveMaterialPayload, parseGltfDiff);
