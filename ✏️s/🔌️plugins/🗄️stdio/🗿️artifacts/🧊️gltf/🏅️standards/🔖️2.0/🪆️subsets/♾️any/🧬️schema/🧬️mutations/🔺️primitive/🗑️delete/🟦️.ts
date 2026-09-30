/** 🗑️ `delete-primitive` wire twin: the flat `Apply` payload `GltfDeletePrimitivePayload` and the phase wire `DeletePrimitiveMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfDeletePrimitivePayload {
  mesh: number;
  primitive: number;
}

export type DeletePrimitiveMutation = GltfPhase<GltfDeletePrimitivePayload, GltfDiff>;

export const parseGltfDeletePrimitivePayload = gltfWireObject<GltfDeletePrimitivePayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex) });
export const parseDeletePrimitiveMutation = gltfWirePhase(parseGltfDeletePrimitivePayload, parseGltfDiff);
