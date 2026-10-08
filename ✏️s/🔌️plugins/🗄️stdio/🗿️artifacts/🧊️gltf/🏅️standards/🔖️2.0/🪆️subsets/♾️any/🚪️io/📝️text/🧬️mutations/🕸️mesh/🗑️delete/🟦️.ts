/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfDeleteMeshPayload,DeleteMeshMutation} from "../../../../../🧬️schema/🧬️mutations/🕸️mesh/🗑️delete/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🕸️mesh/🗑️delete/🟦️.ts";
/** 🗑️ `delete-mesh` wire twin: the flat `Apply` payload `GltfDeleteMeshPayload` and the phase wire `DeleteMeshMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfDeleteMeshPayload = gltfWireObject<GltfDeleteMeshPayload>({ index: gltfWireRequired(gltfWireIndex) });
export const parseDeleteMeshMutation = gltfWireApplyPhase(parseGltfDeleteMeshPayload);
