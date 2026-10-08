/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfChangeMeshNamePayload,ChangeMeshNameMutation} from "../../../../../🧬️schema/🧬️mutations/🕸️mesh/🏷️rename/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🕸️mesh/🏷️rename/🟦️.ts";
/** 🏷️ `change-mesh-name` wire twin: the flat `Apply` payload `GltfChangeMeshNamePayload` and the phase wire `ChangeMeshNameMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireNullable, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfChangeMeshNamePayload = gltfWireObject<GltfChangeMeshNamePayload>({ mesh: gltfWireRequired(gltfWireIndex), value: gltfWireRequired(gltfWireNullable(gltfWireString)) });
export const parseChangeMeshNameMutation = gltfWireApplyPhase(parseGltfChangeMeshNamePayload);
