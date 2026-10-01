/** 🌱️ `create-morph-target` wire twin: the flat `Apply` payload `GltfCreateMorphTargetPayload` and the phase wire `CreateMorphTargetMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfCreateMorphTargetPayload {
  mesh: bigint;
  primitive: bigint;
  position: bigint;
}

export type CreateMorphTargetMutation = GltfPhase<GltfCreateMorphTargetPayload, GltfDiff>;

export const parseGltfCreateMorphTargetPayload = gltfWireObject<GltfCreateMorphTargetPayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseCreateMorphTargetMutation = gltfWirePhase(parseGltfCreateMorphTargetPayload, parseGltfDiff);
