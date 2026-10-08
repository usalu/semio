/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfDataPresence,GltfChangePrimitiveExtraDataPayload,ChangePrimitiveExtraDataMutation} from "../../../../../🧬️schema/🧬️mutations/🔺️primitive/📝️change-extras/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🔺️primitive/📝️change-extras/🟦️.ts";
/** 📝️ `change-primitive-extra-data` wire twin: the flat `Apply` payload `GltfChangePrimitiveExtraDataPayload` and the phase wire `ChangePrimitiveExtraDataMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { type GltfJson, gltfWireIndex, gltfWireLiteral, gltfWireObject, gltfWireRequired, gltfWireTagged, parseGltfJson } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfDataPresence = gltfWireTagged<GltfDataPresence, "state">("state", {
  absent: gltfWireObject<Extract<GltfDataPresence, { state: "absent" }>>({ state: gltfWireRequired(gltfWireLiteral("absent")) }),
  present: gltfWireObject<Extract<GltfDataPresence, { state: "present" }>>({ state: gltfWireRequired(gltfWireLiteral("present")), value: gltfWireRequired(parseGltfJson) }),
});
export const parseGltfChangePrimitiveExtraDataPayload = gltfWireObject<GltfChangePrimitiveExtraDataPayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), data: gltfWireRequired(parseGltfDataPresence) });
export const parseChangePrimitiveExtraDataMutation = gltfWireApplyPhase(parseGltfChangePrimitiveExtraDataPayload);
