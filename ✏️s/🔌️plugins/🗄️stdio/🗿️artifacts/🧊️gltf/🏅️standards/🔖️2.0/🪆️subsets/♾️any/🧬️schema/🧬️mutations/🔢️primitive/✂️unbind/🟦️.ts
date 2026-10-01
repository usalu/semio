/** ✂️ `unbind-primitive-indices` wire twin: the flat `Apply` payload `GltfUnbindPrimitiveIndicesPayload` and the phase wire `UnbindPrimitiveIndicesMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfUnbindPrimitiveIndicesPayload {
  mesh: bigint;
  primitive: bigint;
}

export type UnbindPrimitiveIndicesMutation = GltfPhase<GltfUnbindPrimitiveIndicesPayload, GltfDiff>;

export const parseGltfUnbindPrimitiveIndicesPayload = gltfWireObject<GltfUnbindPrimitiveIndicesPayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex) });
export const parseUnbindPrimitiveIndicesMutation = gltfWirePhase(parseGltfUnbindPrimitiveIndicesPayload, parseGltfDiff);
