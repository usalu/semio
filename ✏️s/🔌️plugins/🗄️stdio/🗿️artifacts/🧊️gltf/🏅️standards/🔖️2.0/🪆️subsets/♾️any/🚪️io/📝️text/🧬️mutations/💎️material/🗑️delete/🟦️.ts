/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfDeleteMaterialPayload,DeleteMaterialMutation} from "../../../../../🧬️schema/🧬️mutations/💎️material/🗑️delete/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/💎️material/🗑️delete/🟦️.ts";
/** 🗑️ `delete-material` wire twin: the flat `Apply` payload `GltfDeleteMaterialPayload` and the phase wire `DeleteMaterialMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfDeleteMaterialPayload = gltfWireObject<GltfDeleteMaterialPayload>({ index: gltfWireRequired(gltfWireIndex) });
export const parseDeleteMaterialMutation = gltfWirePhase(parseGltfDeleteMaterialPayload, parseGltfDiff);
