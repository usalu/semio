/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfDeleteScenePayload,DeleteSceneMutation} from "../../../../../🧬️schema/🧬️mutations/🎬️scene/🗑️delete/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🎬️scene/🗑️delete/🟦️.ts";
/** 🗑️ `delete-scene` wire twin: the flat `Apply` payload `GltfDeleteScenePayload` and the phase wire `DeleteSceneMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfDeleteScenePayload = gltfWireObject<GltfDeleteScenePayload>({ index: gltfWireRequired(gltfWireIndex) });
export const parseDeleteSceneMutation = gltfWireApplyPhase(parseGltfDeleteScenePayload);
