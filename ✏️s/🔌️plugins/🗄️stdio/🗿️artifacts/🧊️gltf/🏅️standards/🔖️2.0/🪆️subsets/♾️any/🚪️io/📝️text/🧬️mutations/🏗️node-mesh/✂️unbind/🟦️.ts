/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfUnbindNodeMeshPayload,UnbindNodeMeshMutation} from "../../../../../🧬️schema/🧬️mutations/🏗️node-mesh/✂️unbind/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🏗️node-mesh/✂️unbind/🟦️.ts";
/** ✂️ `unbind-node-mesh` wire twin: the flat `Apply` payload `GltfUnbindNodeMeshPayload` and the phase wire `UnbindNodeMeshMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfUnbindNodeMeshPayload = gltfWireObject<GltfUnbindNodeMeshPayload>({ node: gltfWireRequired(gltfWireIndex) });
export const parseUnbindNodeMeshMutation = gltfWireApplyPhase(parseGltfUnbindNodeMeshPayload);
