/** 🌱️ `create-buffer` wire twin: the flat `Apply` payload `GltfCreateBufferPayload` and the phase wire `CreateBufferMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireByte, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfCreateBufferPayload {
  position: bigint;
  bytes: number[];
}

export type CreateBufferMutation = GltfPhase<GltfCreateBufferPayload, GltfDiff>;

export const parseGltfCreateBufferPayload = gltfWireObject<GltfCreateBufferPayload>({ position: gltfWireRequired(gltfWireIndex), bytes: gltfWireRequired(gltfWireArray(gltfWireByte)) });
export const parseCreateBufferMutation = gltfWirePhase(parseGltfCreateBufferPayload, parseGltfDiff);
