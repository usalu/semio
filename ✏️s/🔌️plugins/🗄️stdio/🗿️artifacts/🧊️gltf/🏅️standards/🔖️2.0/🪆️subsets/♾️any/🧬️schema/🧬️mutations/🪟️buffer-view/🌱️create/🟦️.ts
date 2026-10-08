/** 🌱️ `create-buffer-view` wire twin: the flat `Apply` payload `GltfCreateBufferViewPayload` and the phase wire `CreateBufferViewMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireOptional, type GltfBufferView, parseGltfBufferView } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfCreateBufferViewPayload {
  position: bigint;
  buffer: bigint;
  byteOffset: bigint;
  byteLength: bigint;
  bufferView?: GltfBufferView;
}

export type CreateBufferViewMutation = GltfApplyPhase<GltfCreateBufferViewPayload>;

export const parseGltfCreateBufferViewPayload = gltfWireObject<GltfCreateBufferViewPayload>({ position: gltfWireRequired(gltfWireIndex), buffer: gltfWireRequired(gltfWireIndex), byteOffset: gltfWireRequired(gltfWireIndex), byteLength: gltfWireRequired(gltfWireIndex), bufferView: gltfWireOptional(parseGltfBufferView) });
export const parseCreateBufferViewMutation = gltfWireApplyPhase(parseGltfCreateBufferViewPayload);
