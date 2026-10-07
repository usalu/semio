/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfDataPresence,GltfChangeMeshExtensionDataPayload,ChangeMeshExtensionDataMutation} from "../../../../../🧬️schema/🧬️mutations/🕸️mesh/🧩️change-extensions/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🕸️mesh/🧩️change-extensions/🟦️.ts";
/** 🧩️ `change-mesh-extension-data` wire twin: the flat `Apply` payload `GltfChangeMeshExtensionDataPayload` and the phase wire `ChangeMeshExtensionDataMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { type GltfJson, gltfWireIndex, gltfWireLiteral, gltfWireObject, gltfWireRequired, gltfWireTagged, parseGltfJson } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfDataPresence = gltfWireTagged<GltfDataPresence, "state">("state", {
  absent: gltfWireObject<Extract<GltfDataPresence, { state: "absent" }>>({ state: gltfWireRequired(gltfWireLiteral("absent")) }),
  present: gltfWireObject<Extract<GltfDataPresence, { state: "present" }>>({ state: gltfWireRequired(gltfWireLiteral("present")), value: gltfWireRequired(parseGltfJson) }),
});
export const parseGltfChangeMeshExtensionDataPayload = gltfWireObject<GltfChangeMeshExtensionDataPayload>({ mesh: gltfWireRequired(gltfWireIndex), data: gltfWireRequired(parseGltfDataPresence) });
export const parseChangeMeshExtensionDataMutation = gltfWirePhase(parseGltfChangeMeshExtensionDataPayload, parseGltfDiff);
