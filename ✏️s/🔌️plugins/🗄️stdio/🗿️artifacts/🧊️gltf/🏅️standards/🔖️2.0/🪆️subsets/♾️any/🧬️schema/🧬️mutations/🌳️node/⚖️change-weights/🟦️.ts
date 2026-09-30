/** ⚖️ `change-node-morph-weights` wire twin: the flat `Apply` payload `GltfChangeNodeMorphWeightsPayload` and the phase wire `ChangeNodeMorphWeightsMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireNumber, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfChangeNodeMorphWeightsPayload {
  node: number;
  weights: number[];
}

export type ChangeNodeMorphWeightsMutation = GltfPhase<GltfChangeNodeMorphWeightsPayload, GltfDiff>;

export const parseGltfChangeNodeMorphWeightsPayload = gltfWireObject<GltfChangeNodeMorphWeightsPayload>({ node: gltfWireRequired(gltfWireIndex), weights: gltfWireRequired(gltfWireArray(gltfWireNumber)) });
export const parseChangeNodeMorphWeightsMutation = gltfWirePhase(parseGltfChangeNodeMorphWeightsPayload, parseGltfDiff);
