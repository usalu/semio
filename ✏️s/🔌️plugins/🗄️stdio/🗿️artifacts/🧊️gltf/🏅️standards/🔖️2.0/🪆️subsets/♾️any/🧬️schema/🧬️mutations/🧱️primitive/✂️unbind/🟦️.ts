/** ✂️ `unbind-primitive-material` wire twin: the flat `Apply` payload `GltfUnbindPrimitiveMaterialPayload` and the phase wire `UnbindPrimitiveMaterialMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfUnbindPrimitiveMaterialPayload {
  mesh: bigint;
  primitive: bigint;
}

export type UnbindPrimitiveMaterialMutation = GltfPhase<GltfUnbindPrimitiveMaterialPayload, GltfDiff>;

export const parseGltfUnbindPrimitiveMaterialPayload = gltfWireObject<GltfUnbindPrimitiveMaterialPayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex) });
export const parseUnbindPrimitiveMaterialMutation = gltfWirePhase(parseGltfUnbindPrimitiveMaterialPayload, parseGltfDiff);
