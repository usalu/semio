/** 🔗️ `bind-primitive-material` wire twin: the flat `Apply` payload `GltfBindPrimitiveMaterialPayload` and the phase wire `BindPrimitiveMaterialMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfBindPrimitiveMaterialPayload {
  mesh: number;
  primitive: number;
  material: number;
}

export type BindPrimitiveMaterialMutation = GltfPhase<GltfBindPrimitiveMaterialPayload, GltfDiff>;

export const parseGltfBindPrimitiveMaterialPayload = gltfWireObject<GltfBindPrimitiveMaterialPayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), material: gltfWireRequired(gltfWireIndex) });
export const parseBindPrimitiveMaterialMutation = gltfWirePhase(parseGltfBindPrimitiveMaterialPayload, parseGltfDiff);
