/** ⚖️ `change-mesh-morph-weights` wire twin: the flat `Apply` payload `GltfChangeMeshMorphWeightsPayload` and the phase wire `ChangeMeshMorphWeightsMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireNumber, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfChangeMeshMorphWeightsPayload {
  mesh: number;
  weights: number[];
}

export type ChangeMeshMorphWeightsMutation = GltfPhase<GltfChangeMeshMorphWeightsPayload, GltfDiff>;

export const parseGltfChangeMeshMorphWeightsPayload = gltfWireObject<GltfChangeMeshMorphWeightsPayload>({ mesh: gltfWireRequired(gltfWireIndex), weights: gltfWireRequired(gltfWireArray(gltfWireNumber)) });
export const parseChangeMeshMorphWeightsMutation = gltfWirePhase(parseGltfChangeMeshMorphWeightsPayload, parseGltfDiff);
