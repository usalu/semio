/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfDataPresence,GltfChangeMeshExtraDataPayload,ChangeMeshExtraDataMutation} from "../../../../../🧬️schema/🧬️mutations/🕸️mesh/📝️change-extras/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🕸️mesh/📝️change-extras/🟦️.ts";
/** 📝️ `change-mesh-extra-data` wire twin: the flat `Apply` payload `GltfChangeMeshExtraDataPayload` and the phase wire `ChangeMeshExtraDataMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { type GltfJson, gltfWireIndex, gltfWireLiteral, gltfWireObject, gltfWireRequired, gltfWireTagged, parseGltfJson } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfDataPresence = gltfWireTagged<GltfDataPresence, "state">("state", {
  absent: gltfWireObject<Extract<GltfDataPresence, { state: "absent" }>>({ state: gltfWireRequired(gltfWireLiteral("absent")) }),
  present: gltfWireObject<Extract<GltfDataPresence, { state: "present" }>>({ state: gltfWireRequired(gltfWireLiteral("present")), value: gltfWireRequired(parseGltfJson) }),
});
export const parseGltfChangeMeshExtraDataPayload = gltfWireObject<GltfChangeMeshExtraDataPayload>({ mesh: gltfWireRequired(gltfWireIndex), data: gltfWireRequired(parseGltfDataPresence) });
export const parseChangeMeshExtraDataMutation = gltfWireApplyPhase(parseGltfChangeMeshExtraDataPayload);
