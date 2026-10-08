/** ✂️ `unbind-primitive-attribute` wire twin: the flat `Apply` payload `GltfUnbindPrimitiveAttributePayload` and the phase wire `UnbindPrimitiveAttributeMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfUnbindPrimitiveAttributePayload {
  mesh: bigint;
  primitive: bigint;
  semantic: string;
}

export type UnbindPrimitiveAttributeMutation = GltfApplyPhase<GltfUnbindPrimitiveAttributePayload>;

export const parseGltfUnbindPrimitiveAttributePayload = gltfWireObject<GltfUnbindPrimitiveAttributePayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), semantic: gltfWireRequired(gltfWireString) });
export const parseUnbindPrimitiveAttributeMutation = gltfWireApplyPhase(parseGltfUnbindPrimitiveAttributePayload);
