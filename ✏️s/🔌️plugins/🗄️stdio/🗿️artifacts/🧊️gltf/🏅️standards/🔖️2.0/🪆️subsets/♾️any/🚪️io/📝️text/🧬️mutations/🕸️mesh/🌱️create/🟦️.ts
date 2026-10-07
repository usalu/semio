/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfCreateMeshPayload,CreateMeshMutation} from "../../../../../🧬️schema/🧬️mutations/🕸️mesh/🌱️create/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🕸️mesh/🌱️create/🟦️.ts";
/** 🌱️ `create-mesh` wire twin: the flat `Apply` payload `GltfCreateMeshPayload` and the phase wire `CreateMeshMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfCreateMeshPayload = gltfWireObject<GltfCreateMeshPayload>({ position: gltfWireRequired(gltfWireIndex) });
export const parseCreateMeshMutation = gltfWirePhase(parseGltfCreateMeshPayload, parseGltfDiff);
