/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfChangeNodeNamePayload,ChangeNodeNameMutation} from "../../../../../🧬️schema/🧬️mutations/🌳️node/🏷️rename/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🌳️node/🏷️rename/🟦️.ts";
/** 🏷️ `change-node-name` wire twin: the flat `Apply` payload `GltfChangeNodeNamePayload` and the phase wire `ChangeNodeNameMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireInteger, gltfWireNullable, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfChangeNodeNamePayload = gltfWireObject<GltfChangeNodeNamePayload>({ node: gltfWireRequired(gltfWireInteger(4294967295)), value: gltfWireRequired(gltfWireNullable(gltfWireString)) });
export const parseChangeNodeNameMutation = gltfWireApplyPhase(parseGltfChangeNodeNamePayload);
