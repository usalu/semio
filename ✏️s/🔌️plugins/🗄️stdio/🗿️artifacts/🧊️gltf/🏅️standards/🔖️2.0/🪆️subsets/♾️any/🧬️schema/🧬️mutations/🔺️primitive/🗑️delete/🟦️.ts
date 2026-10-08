/** 🗑️ `delete-primitive` wire twin: the flat `Apply` payload `GltfDeletePrimitivePayload` and the phase wire `DeletePrimitiveMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfDeletePrimitivePayload {
  mesh: bigint;
  primitive: bigint;
}

export type DeletePrimitiveMutation = GltfApplyPhase<GltfDeletePrimitivePayload>;

export const parseGltfDeletePrimitivePayload = gltfWireObject<GltfDeletePrimitivePayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex) });
export const parseDeletePrimitiveMutation = gltfWireApplyPhase(parseGltfDeletePrimitivePayload);
