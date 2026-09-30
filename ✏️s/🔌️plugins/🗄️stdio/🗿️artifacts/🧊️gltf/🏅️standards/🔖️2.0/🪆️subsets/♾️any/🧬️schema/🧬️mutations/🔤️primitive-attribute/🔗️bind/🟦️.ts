/** 🔗️ `bind-primitive-attribute` wire twin: the flat `Apply` payload `GltfBindPrimitiveAttributePayload` and the phase wire `BindPrimitiveAttributeMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfBindPrimitiveAttributePayload {
  mesh: number;
  primitive: number;
  semantic: string;
  accessor: number;
}

export type BindPrimitiveAttributeMutation = GltfPhase<GltfBindPrimitiveAttributePayload, GltfDiff>;

export const parseGltfBindPrimitiveAttributePayload = gltfWireObject<GltfBindPrimitiveAttributePayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), semantic: gltfWireRequired(gltfWireString), accessor: gltfWireRequired(gltfWireIndex) });
export const parseBindPrimitiveAttributeMutation = gltfWirePhase(parseGltfBindPrimitiveAttributePayload, parseGltfDiff);
