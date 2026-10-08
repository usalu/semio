/** 🌱️ `create-sampler` wire twin: the flat `Apply` payload `GltfCreateSamplerPayload` and the phase wire `CreateSamplerMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireOptional, type GltfSampler, parseGltfSampler } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfCreateSamplerPayload {
  position: bigint;
  sampler?: GltfSampler;
}

export type CreateSamplerMutation = GltfApplyPhase<GltfCreateSamplerPayload>;

export const parseGltfCreateSamplerPayload = gltfWireObject<GltfCreateSamplerPayload>({ position: gltfWireRequired(gltfWireIndex), sampler: gltfWireOptional(parseGltfSampler) });
export const parseCreateSamplerMutation = gltfWireApplyPhase(parseGltfCreateSamplerPayload);
