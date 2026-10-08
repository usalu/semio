/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfDataPresence,GltfChangeNodeExtensionDataPayload,ChangeNodeExtensionDataMutation} from "../../../../../🧬️schema/🧬️mutations/🌳️node/🧩️change-extensions/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🌳️node/🧩️change-extensions/🟦️.ts";
/** 🧩️ `change-node-extension-data` wire twin: the flat `Apply` payload `GltfChangeNodeExtensionDataPayload` and the phase wire `ChangeNodeExtensionDataMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { type GltfJson, gltfWireIndex, gltfWireLiteral, gltfWireObject, gltfWireRequired, gltfWireTagged, parseGltfJson } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfDataPresence = gltfWireTagged<GltfDataPresence, "state">("state", {
  absent: gltfWireObject<Extract<GltfDataPresence, { state: "absent" }>>({ state: gltfWireRequired(gltfWireLiteral("absent")) }),
  present: gltfWireObject<Extract<GltfDataPresence, { state: "present" }>>({ state: gltfWireRequired(gltfWireLiteral("present")), value: gltfWireRequired(parseGltfJson) }),
});
export const parseGltfChangeNodeExtensionDataPayload = gltfWireObject<GltfChangeNodeExtensionDataPayload>({ node: gltfWireRequired(gltfWireIndex), data: gltfWireRequired(parseGltfDataPresence) });
export const parseChangeNodeExtensionDataMutation = gltfWireApplyPhase(parseGltfChangeNodeExtensionDataPayload);
