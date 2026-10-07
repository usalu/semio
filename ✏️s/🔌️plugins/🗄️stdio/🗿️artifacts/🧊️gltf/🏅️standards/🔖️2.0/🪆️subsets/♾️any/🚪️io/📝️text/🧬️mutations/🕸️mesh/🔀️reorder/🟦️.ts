/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfReorderMeshsPayload,ReorderMeshsMutation} from "../../../../../🧬️schema/🧬️mutations/🕸️mesh/🔀️reorder/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🕸️mesh/🔀️reorder/🟦️.ts";
/** 🔀️ `reorder-meshs` wire twin: the flat `Apply` payload `GltfReorderMeshsPayload` and the phase wire `ReorderMeshsMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfReorderMeshsPayload = gltfWireObject<GltfReorderMeshsPayload>({ order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderMeshsMutation = gltfWirePhase(parseGltfReorderMeshsPayload, parseGltfDiff);
