/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfMoveMaterialPayload,MoveMaterialMutation} from "../../../../../🧬️schema/🧬️mutations/💎️material/🚚️move/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/💎️material/🚚️move/🟦️.ts";
/** 🚚️ `move-material` wire twin: the flat `Apply` payload `GltfMoveMaterialPayload` and the phase wire `MoveMaterialMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfMoveMaterialPayload = gltfWireObject<GltfMoveMaterialPayload>({ index: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveMaterialMutation = gltfWirePhase(parseGltfMoveMaterialPayload, parseGltfDiff);
