/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfDeleteSkinPayload,DeleteSkinMutation} from "../../../../../🧬️schema/🧬️mutations/🦴️skin/🗑️delete/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🦴️skin/🗑️delete/🟦️.ts";
/** 🗑️ `delete-skin` wire twin: the flat `Apply` payload `GltfDeleteSkinPayload` and the phase wire `DeleteSkinMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfDeleteSkinPayload = gltfWireObject<GltfDeleteSkinPayload>({ index: gltfWireRequired(gltfWireIndex) });
export const parseDeleteSkinMutation = gltfWireApplyPhase(parseGltfDeleteSkinPayload);
