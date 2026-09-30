/** 🌱️ `create-skin` wire twin: the flat `Apply` payload `GltfCreateSkinPayload` and the phase wire `CreateSkinMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfCreateSkinPayload {
  position: number;
}

export type CreateSkinMutation = GltfPhase<GltfCreateSkinPayload, GltfDiff>;

export const parseGltfCreateSkinPayload = gltfWireObject<GltfCreateSkinPayload>({ position: gltfWireRequired(gltfWireIndex) });
export const parseCreateSkinMutation = gltfWirePhase(parseGltfCreateSkinPayload, parseGltfDiff);
