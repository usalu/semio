/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfMoveMeshPayload,MoveMeshMutation} from "../../../../../🧬️schema/🧬️mutations/🕸️mesh/🚚️move/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🕸️mesh/🚚️move/🟦️.ts";
/** 🚚️ `move-mesh` wire twin: the flat `Apply` payload `GltfMoveMeshPayload` and the phase wire `MoveMeshMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfMoveMeshPayload = gltfWireObject<GltfMoveMeshPayload>({ index: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveMeshMutation = gltfWireApplyPhase(parseGltfMoveMeshPayload);
