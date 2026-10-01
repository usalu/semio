/** 🔗️ `bind-primitive-indices` wire twin: the flat `Apply` payload `GltfBindPrimitiveIndicesPayload` and the phase wire `BindPrimitiveIndicesMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfBindPrimitiveIndicesPayload {
  mesh: bigint;
  primitive: bigint;
  accessor: bigint;
}

export type BindPrimitiveIndicesMutation = GltfPhase<GltfBindPrimitiveIndicesPayload, GltfDiff>;

export const parseGltfBindPrimitiveIndicesPayload = gltfWireObject<GltfBindPrimitiveIndicesPayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), accessor: gltfWireRequired(gltfWireIndex) });
export const parseBindPrimitiveIndicesMutation = gltfWirePhase(parseGltfBindPrimitiveIndicesPayload, parseGltfDiff);
