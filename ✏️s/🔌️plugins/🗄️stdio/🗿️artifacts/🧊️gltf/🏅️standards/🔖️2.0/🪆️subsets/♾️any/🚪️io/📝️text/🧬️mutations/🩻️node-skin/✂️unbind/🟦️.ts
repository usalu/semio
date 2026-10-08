/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfUnbindNodeSkinPayload,UnbindNodeSkinMutation} from "../../../../../🧬️schema/🧬️mutations/🩻️node-skin/✂️unbind/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🩻️node-skin/✂️unbind/🟦️.ts";
/** ✂️ `unbind-node-skin` wire twin: the flat `Apply` payload `GltfUnbindNodeSkinPayload` and the phase wire `UnbindNodeSkinMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfUnbindNodeSkinPayload = gltfWireObject<GltfUnbindNodeSkinPayload>({ node: gltfWireRequired(gltfWireIndex) });
export const parseUnbindNodeSkinMutation = gltfWireApplyPhase(parseGltfUnbindNodeSkinPayload);
