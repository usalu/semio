/** 🌱️ `create-material` wire twin: the flat `Apply` payload `GltfCreateMaterialPayload` and the phase wire `CreateMaterialMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfCreateMaterialPayload {
  position: bigint;
}

export type CreateMaterialMutation = GltfPhase<GltfCreateMaterialPayload, GltfDiff>;

export const parseGltfCreateMaterialPayload = gltfWireObject<GltfCreateMaterialPayload>({ position: gltfWireRequired(gltfWireIndex) });
export const parseCreateMaterialMutation = gltfWirePhase(parseGltfCreateMaterialPayload, parseGltfDiff);
