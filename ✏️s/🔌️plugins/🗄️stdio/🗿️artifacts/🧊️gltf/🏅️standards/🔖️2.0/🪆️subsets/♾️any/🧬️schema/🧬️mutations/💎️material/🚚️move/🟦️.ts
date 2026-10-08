/** 🚚️ `move-material` wire twin: the flat `Apply` payload `GltfMoveMaterialPayload` and the phase wire `MoveMaterialMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfMoveMaterialPayload {
  index: bigint;
  position: bigint;
}

export type MoveMaterialMutation = GltfApplyPhase<GltfMoveMaterialPayload>;

export const parseGltfMoveMaterialPayload = gltfWireObject<GltfMoveMaterialPayload>({ index: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveMaterialMutation = gltfWireApplyPhase(parseGltfMoveMaterialPayload);
