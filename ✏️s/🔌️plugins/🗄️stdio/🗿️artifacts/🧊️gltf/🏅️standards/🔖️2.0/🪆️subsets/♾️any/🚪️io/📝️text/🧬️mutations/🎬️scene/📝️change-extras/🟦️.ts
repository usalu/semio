/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfDataPresence,GltfChangeSceneExtraDataPayload,ChangeSceneExtraDataMutation} from "../../../../../🧬️schema/🧬️mutations/🎬️scene/📝️change-extras/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🎬️scene/📝️change-extras/🟦️.ts";
/** 📝️ `change-scene-extra-data` wire twin: the flat `Apply` payload `GltfChangeSceneExtraDataPayload` and the phase wire `ChangeSceneExtraDataMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { type GltfJson, gltfWireIndex, gltfWireLiteral, gltfWireObject, gltfWireRequired, gltfWireTagged, parseGltfJson } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfDataPresence = gltfWireTagged<GltfDataPresence, "state">("state", {
  absent: gltfWireObject<Extract<GltfDataPresence, { state: "absent" }>>({ state: gltfWireRequired(gltfWireLiteral("absent")) }),
  present: gltfWireObject<Extract<GltfDataPresence, { state: "present" }>>({ state: gltfWireRequired(gltfWireLiteral("present")), value: gltfWireRequired(parseGltfJson) }),
});
export const parseGltfChangeSceneExtraDataPayload = gltfWireObject<GltfChangeSceneExtraDataPayload>({ scene: gltfWireRequired(gltfWireIndex), data: gltfWireRequired(parseGltfDataPresence) });
export const parseChangeSceneExtraDataMutation = gltfWireApplyPhase(parseGltfChangeSceneExtraDataPayload);
