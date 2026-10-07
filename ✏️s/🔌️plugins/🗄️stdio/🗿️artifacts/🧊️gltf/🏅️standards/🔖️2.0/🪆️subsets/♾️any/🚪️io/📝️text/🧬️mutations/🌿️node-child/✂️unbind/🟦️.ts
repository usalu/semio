/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfUnbindNodeChildPayload,UnbindNodeChildMutation} from "../../../../../🧬️schema/🧬️mutations/🌿️node-child/✂️unbind/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🌿️node-child/✂️unbind/🟦️.ts";
/** ✂️ `unbind-node-child` wire twin: the flat `Apply` payload `GltfUnbindNodeChildPayload` and the phase wire `UnbindNodeChildMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfUnbindNodeChildPayload = gltfWireObject<GltfUnbindNodeChildPayload>({ parent: gltfWireRequired(gltfWireIndex), child: gltfWireRequired(gltfWireIndex) });
export const parseUnbindNodeChildMutation = gltfWirePhase(parseGltfUnbindNodeChildPayload, parseGltfDiff);
