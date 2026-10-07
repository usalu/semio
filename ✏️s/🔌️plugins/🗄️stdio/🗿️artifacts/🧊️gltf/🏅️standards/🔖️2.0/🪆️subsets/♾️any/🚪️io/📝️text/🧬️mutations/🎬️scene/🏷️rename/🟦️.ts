/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfChangeSceneNamePayload,ChangeSceneNameMutation} from "../../../../../🧬️schema/🧬️mutations/🎬️scene/🏷️rename/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🎬️scene/🏷️rename/🟦️.ts";
/** 🏷️ `change-scene-name` wire twin: the flat `Apply` payload `GltfChangeSceneNamePayload` and the phase wire `ChangeSceneNameMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireNullable, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfChangeSceneNamePayload = gltfWireObject<GltfChangeSceneNamePayload>({ scene: gltfWireRequired(gltfWireIndex), value: gltfWireRequired(gltfWireNullable(gltfWireString)) });
export const parseChangeSceneNameMutation = gltfWirePhase(parseGltfChangeSceneNamePayload, parseGltfDiff);
