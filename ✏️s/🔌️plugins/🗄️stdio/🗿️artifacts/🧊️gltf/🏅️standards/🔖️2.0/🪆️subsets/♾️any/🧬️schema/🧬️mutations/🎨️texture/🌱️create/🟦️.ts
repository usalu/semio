/** 🌱️ `create-texture` wire twin: the flat `Apply` payload `GltfCreateTexturePayload` and the phase wire `CreateTextureMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfCreateTexturePayload {
  position: bigint;
}

export type CreateTextureMutation = GltfPhase<GltfCreateTexturePayload, GltfDiff>;

export const parseGltfCreateTexturePayload = gltfWireObject<GltfCreateTexturePayload>({ position: gltfWireRequired(gltfWireIndex) });
export const parseCreateTextureMutation = gltfWirePhase(parseGltfCreateTexturePayload, parseGltfDiff);
