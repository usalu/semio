/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfDataPresence,GltfChangeSceneExtensionDataPayload,ChangeSceneExtensionDataMutation} from "../../../../../🧬️schema/🧬️mutations/🎬️scene/🧩️change-extensions/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🎬️scene/🧩️change-extensions/🟦️.ts";
/** 🧩️ `change-scene-extension-data` wire twin: the flat `Apply` payload `GltfChangeSceneExtensionDataPayload` and the phase wire `ChangeSceneExtensionDataMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { type GltfJson, gltfWireIndex, gltfWireLiteral, gltfWireObject, gltfWireRequired, gltfWireTagged, parseGltfJson } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfDataPresence = gltfWireTagged<GltfDataPresence, "state">("state", {
  absent: gltfWireObject<Extract<GltfDataPresence, { state: "absent" }>>({ state: gltfWireRequired(gltfWireLiteral("absent")) }),
  present: gltfWireObject<Extract<GltfDataPresence, { state: "present" }>>({ state: gltfWireRequired(gltfWireLiteral("present")), value: gltfWireRequired(parseGltfJson) }),
});
export const parseGltfChangeSceneExtensionDataPayload = gltfWireObject<GltfChangeSceneExtensionDataPayload>({ scene: gltfWireRequired(gltfWireIndex), data: gltfWireRequired(parseGltfDataPresence) });
export const parseChangeSceneExtensionDataMutation = gltfWireApplyPhase(parseGltfChangeSceneExtensionDataPayload);
