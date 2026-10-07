/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfDataPresence,GltfChangePrimitiveExtensionDataPayload,ChangePrimitiveExtensionDataMutation} from "../../../../../🧬️schema/🧬️mutations/🔺️primitive/🧩️change-extensions/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🔺️primitive/🧩️change-extensions/🟦️.ts";
/** 🧩️ `change-primitive-extension-data` wire twin: the flat `Apply` payload `GltfChangePrimitiveExtensionDataPayload` and the phase wire `ChangePrimitiveExtensionDataMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { type GltfJson, gltfWireIndex, gltfWireLiteral, gltfWireObject, gltfWireRequired, gltfWireTagged, parseGltfJson } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfDataPresence = gltfWireTagged<GltfDataPresence, "state">("state", {
  absent: gltfWireObject<Extract<GltfDataPresence, { state: "absent" }>>({ state: gltfWireRequired(gltfWireLiteral("absent")) }),
  present: gltfWireObject<Extract<GltfDataPresence, { state: "present" }>>({ state: gltfWireRequired(gltfWireLiteral("present")), value: gltfWireRequired(parseGltfJson) }),
});
export const parseGltfChangePrimitiveExtensionDataPayload = gltfWireObject<GltfChangePrimitiveExtensionDataPayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), data: gltfWireRequired(parseGltfDataPresence) });
export const parseChangePrimitiveExtensionDataMutation = gltfWirePhase(parseGltfChangePrimitiveExtensionDataPayload, parseGltfDiff);
