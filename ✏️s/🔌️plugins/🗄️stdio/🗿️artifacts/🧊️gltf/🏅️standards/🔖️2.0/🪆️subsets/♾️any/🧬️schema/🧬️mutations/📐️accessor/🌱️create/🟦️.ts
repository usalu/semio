/** 🌱️ `create-accessor` wire twin: the flat `Apply` payload `GltfCreateAccessorPayload` and the phase wire `CreateAccessorMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { type GltfAccessorType, type GltfComponentType, gltfWireIndex, gltfWireObject, gltfWireRequired, parseGltfAccessorType, parseGltfComponentType } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfCreateAccessorPayload {
  position: bigint;
  componentType: GltfComponentType;
  count: bigint;
  kind: GltfAccessorType;
}

export type CreateAccessorMutation = GltfPhase<GltfCreateAccessorPayload, GltfDiff>;

export const parseGltfCreateAccessorPayload = gltfWireObject<GltfCreateAccessorPayload>({ position: gltfWireRequired(gltfWireIndex), componentType: gltfWireRequired(parseGltfComponentType), count: gltfWireRequired(gltfWireIndex), kind: gltfWireRequired(parseGltfAccessorType) });
export const parseCreateAccessorMutation = gltfWirePhase(parseGltfCreateAccessorPayload, parseGltfDiff);
