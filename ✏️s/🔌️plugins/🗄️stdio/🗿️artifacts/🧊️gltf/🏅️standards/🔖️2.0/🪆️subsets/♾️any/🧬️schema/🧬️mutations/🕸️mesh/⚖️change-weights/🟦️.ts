import type {Binary64} from "@semio-tech/framework/value/ieee754";
/** ⚖️ `change-mesh-morph-weights` wire twin: the flat `Apply` payload `GltfChangeMeshMorphWeightsPayload` and the phase wire `ChangeMeshMorphWeightsMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireNumber, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfChangeMeshMorphWeightsPayload {
  mesh: bigint;
  weights: Binary64[];
}

export type ChangeMeshMorphWeightsMutation = GltfApplyPhase<GltfChangeMeshMorphWeightsPayload>;

export const parseGltfChangeMeshMorphWeightsPayload = gltfWireObject<GltfChangeMeshMorphWeightsPayload>({ mesh: gltfWireRequired(gltfWireIndex), weights: gltfWireRequired(gltfWireArray(gltfWireNumber)) });
export const parseChangeMeshMorphWeightsMutation = gltfWireApplyPhase(parseGltfChangeMeshMorphWeightsPayload);
