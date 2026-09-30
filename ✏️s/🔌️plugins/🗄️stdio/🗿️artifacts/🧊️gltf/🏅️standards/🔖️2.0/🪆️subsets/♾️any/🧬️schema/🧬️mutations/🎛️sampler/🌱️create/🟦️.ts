/** 🌱️ `create-sampler` wire twin: the flat `Apply` payload `GltfCreateSamplerPayload` and the phase wire `CreateSamplerMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfCreateSamplerPayload {
  position: number;
}

export type CreateSamplerMutation = GltfPhase<GltfCreateSamplerPayload, GltfDiff>;

export const parseGltfCreateSamplerPayload = gltfWireObject<GltfCreateSamplerPayload>({ position: gltfWireRequired(gltfWireIndex) });
export const parseCreateSamplerMutation = gltfWirePhase(parseGltfCreateSamplerPayload, parseGltfDiff);
