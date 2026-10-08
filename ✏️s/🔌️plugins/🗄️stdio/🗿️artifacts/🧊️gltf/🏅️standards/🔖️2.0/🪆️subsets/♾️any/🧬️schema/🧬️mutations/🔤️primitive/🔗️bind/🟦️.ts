/** 🔗️ `bind-primitive-attribute` wire twin: the flat `Apply` payload `GltfBindPrimitiveAttributePayload` and the phase wire `BindPrimitiveAttributeMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfBindPrimitiveAttributePayload {
  mesh: bigint;
  primitive: bigint;
  semantic: string;
  accessor: bigint;
}

export type BindPrimitiveAttributeMutation = GltfApplyPhase<GltfBindPrimitiveAttributePayload>;

export const parseGltfBindPrimitiveAttributePayload = gltfWireObject<GltfBindPrimitiveAttributePayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), semantic: gltfWireRequired(gltfWireString), accessor: gltfWireRequired(gltfWireIndex) });
export const parseBindPrimitiveAttributeMutation = gltfWireApplyPhase(parseGltfBindPrimitiveAttributePayload);
