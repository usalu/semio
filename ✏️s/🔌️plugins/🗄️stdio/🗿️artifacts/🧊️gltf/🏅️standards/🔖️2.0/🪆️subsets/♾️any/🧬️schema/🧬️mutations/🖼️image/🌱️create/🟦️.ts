/** 🌱️ `create-image` wire twin: the flat `Apply` payload `GltfCreateImagePayload` and the phase wire `CreateImageMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfCreateImagePayload {
  position: bigint;
}

export type CreateImageMutation = GltfPhase<GltfCreateImagePayload, GltfDiff>;

export const parseGltfCreateImagePayload = gltfWireObject<GltfCreateImagePayload>({ position: gltfWireRequired(gltfWireIndex) });
export const parseCreateImageMutation = gltfWirePhase(parseGltfCreateImagePayload, parseGltfDiff);
