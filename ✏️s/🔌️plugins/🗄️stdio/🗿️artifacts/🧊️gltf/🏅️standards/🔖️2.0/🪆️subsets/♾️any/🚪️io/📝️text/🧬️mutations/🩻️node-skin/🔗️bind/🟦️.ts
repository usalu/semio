/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfBindNodeSkinPayload,BindNodeSkinMutation} from "../../../../../🧬️schema/🧬️mutations/🩻️node-skin/🔗️bind/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🩻️node-skin/🔗️bind/🟦️.ts";
/** 🔗️ `bind-node-skin` wire twin: the flat `Apply` payload `GltfBindNodeSkinPayload` and the phase wire `BindNodeSkinMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfBindNodeSkinPayload = gltfWireObject<GltfBindNodeSkinPayload>({ node: gltfWireRequired(gltfWireIndex), skin: gltfWireRequired(gltfWireIndex) });
export const parseBindNodeSkinMutation = gltfWireApplyPhase(parseGltfBindNodeSkinPayload);
