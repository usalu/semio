/** 🌱️ `create-accessor` wire twin: the flat `Apply` payload `GltfCreateAccessorPayload` and the phase wire `CreateAccessorMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { type GltfAccessorType, type GltfComponentType, gltfWireIndex, gltfWireObject, gltfWireRequired, parseGltfAccessorType, parseGltfComponentType, gltfWireOptional, type GltfAccessor, parseGltfAccessor } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfCreateAccessorPayload {
  position: bigint;
  componentType: GltfComponentType;
  count: bigint;
  kind: GltfAccessorType;
  accessor?: GltfAccessor;
}

export type CreateAccessorMutation = GltfApplyPhase<GltfCreateAccessorPayload>;

export const parseGltfCreateAccessorPayload = gltfWireObject<GltfCreateAccessorPayload>({ position: gltfWireRequired(gltfWireIndex), componentType: gltfWireRequired(parseGltfComponentType), count: gltfWireRequired(gltfWireIndex), kind: gltfWireRequired(parseGltfAccessorType), accessor: gltfWireOptional(parseGltfAccessor) });
export const parseCreateAccessorMutation = gltfWireApplyPhase(parseGltfCreateAccessorPayload);
