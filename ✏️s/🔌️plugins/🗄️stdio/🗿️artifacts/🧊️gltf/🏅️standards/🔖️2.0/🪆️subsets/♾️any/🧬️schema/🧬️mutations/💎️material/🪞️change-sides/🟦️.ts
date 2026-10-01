/** 🪞️ `change-material-double-sided` wire twin: the flat `Apply` payload `GltfChangeMaterialDoubleSidedPayload` and the phase wire `ChangeMaterialDoubleSidedMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireBoolean, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfChangeMaterialDoubleSidedPayload {
  material: bigint;
  doubleSided: boolean;
}

export type ChangeMaterialDoubleSidedMutation = GltfPhase<GltfChangeMaterialDoubleSidedPayload, GltfDiff>;

export const parseGltfChangeMaterialDoubleSidedPayload = gltfWireObject<GltfChangeMaterialDoubleSidedPayload>({ material: gltfWireRequired(gltfWireIndex), doubleSided: gltfWireRequired(gltfWireBoolean) });
export const parseChangeMaterialDoubleSidedMutation = gltfWirePhase(parseGltfChangeMaterialDoubleSidedPayload, parseGltfDiff);
