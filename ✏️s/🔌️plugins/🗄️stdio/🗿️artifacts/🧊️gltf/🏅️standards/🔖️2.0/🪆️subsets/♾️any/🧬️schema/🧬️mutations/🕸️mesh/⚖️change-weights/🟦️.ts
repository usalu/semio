import type {Binary64} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
/** ⚖️ `change-mesh-morph-weights` wire twin: the flat `Apply` payload `GltfChangeMeshMorphWeightsPayload` and the phase wire `ChangeMeshMorphWeightsMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireNumber, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfChangeMeshMorphWeightsPayload {
  mesh: bigint;
  weights: Binary64[];
}

export type ChangeMeshMorphWeightsMutation = GltfPhase<GltfChangeMeshMorphWeightsPayload, GltfDiff>;

export const parseGltfChangeMeshMorphWeightsPayload = gltfWireObject<GltfChangeMeshMorphWeightsPayload>({ mesh: gltfWireRequired(gltfWireIndex), weights: gltfWireRequired(gltfWireArray(gltfWireNumber)) });
export const parseChangeMeshMorphWeightsMutation = gltfWirePhase(parseGltfChangeMeshMorphWeightsPayload, parseGltfDiff);
