/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfBindNodeChildPayload,BindNodeChildMutation} from "../../../../../🧬️schema/🧬️mutations/🌿️node-child/🔗️bind/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🌿️node-child/🔗️bind/🟦️.ts";
/** 🔗️ `bind-node-child` wire twin: the flat `Apply` payload `GltfBindNodeChildPayload` and the phase wire `BindNodeChildMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfBindNodeChildPayload = gltfWireObject<GltfBindNodeChildPayload>({ parent: gltfWireRequired(gltfWireIndex), child: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseBindNodeChildMutation = gltfWirePhase(parseGltfBindNodeChildPayload, parseGltfDiff);
