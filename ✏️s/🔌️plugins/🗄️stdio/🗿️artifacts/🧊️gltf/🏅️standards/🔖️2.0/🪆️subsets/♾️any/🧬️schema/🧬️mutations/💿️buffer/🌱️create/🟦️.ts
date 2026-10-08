/** 🌱️ `create-buffer` wire twin: the flat `Apply` payload `GltfCreateBufferPayload` and the phase wire `CreateBufferMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireByte, gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireOptional, type GltfBuffer, parseGltfBuffer } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfCreateBufferPayload {
  position: bigint;
  bytes: number[];
  buffer?: GltfBuffer;
}

export type CreateBufferMutation = GltfApplyPhase<GltfCreateBufferPayload>;

export const parseGltfCreateBufferPayload = gltfWireObject<GltfCreateBufferPayload>({ position: gltfWireRequired(gltfWireIndex), bytes: gltfWireRequired(gltfWireArray(gltfWireByte)), buffer: gltfWireOptional(parseGltfBuffer) });
export const parseCreateBufferMutation = gltfWireApplyPhase(parseGltfCreateBufferPayload);
