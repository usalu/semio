/** 🌱️ `create-node` wire twin: the flat `Apply` payload `GltfCreateNodePayload` and the phase wire `CreateNodeMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfCreateNodePayload {
  position: bigint;
}

export type CreateNodeMutation = GltfPhase<GltfCreateNodePayload, GltfDiff>;

export const parseGltfCreateNodePayload = gltfWireObject<GltfCreateNodePayload>({ position: gltfWireRequired(gltfWireIndex) });
export const parseCreateNodeMutation = gltfWirePhase(parseGltfCreateNodePayload, parseGltfDiff);
