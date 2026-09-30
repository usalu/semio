/** ✂️ `unbind-primitive-attribute` wire twin: the flat `Apply` payload `GltfUnbindPrimitiveAttributePayload` and the phase wire `UnbindPrimitiveAttributeMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfUnbindPrimitiveAttributePayload {
  mesh: number;
  primitive: number;
  semantic: string;
}

export type UnbindPrimitiveAttributeMutation = GltfPhase<GltfUnbindPrimitiveAttributePayload, GltfDiff>;

export const parseGltfUnbindPrimitiveAttributePayload = gltfWireObject<GltfUnbindPrimitiveAttributePayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), semantic: gltfWireRequired(gltfWireString) });
export const parseUnbindPrimitiveAttributeMutation = gltfWirePhase(parseGltfUnbindPrimitiveAttributePayload, parseGltfDiff);
