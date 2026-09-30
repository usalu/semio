/** 🌱️ `create-buffer-view` wire twin: the flat `Apply` payload `GltfCreateBufferViewPayload` and the phase wire `CreateBufferViewMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfCreateBufferViewPayload {
  position: number;
  buffer: number;
  byteOffset: number;
  byteLength: number;
}

export type CreateBufferViewMutation = GltfPhase<GltfCreateBufferViewPayload, GltfDiff>;

export const parseGltfCreateBufferViewPayload = gltfWireObject<GltfCreateBufferViewPayload>({ position: gltfWireRequired(gltfWireIndex), buffer: gltfWireRequired(gltfWireIndex), byteOffset: gltfWireRequired(gltfWireIndex), byteLength: gltfWireRequired(gltfWireIndex) });
export const parseCreateBufferViewMutation = gltfWirePhase(parseGltfCreateBufferViewPayload, parseGltfDiff);
