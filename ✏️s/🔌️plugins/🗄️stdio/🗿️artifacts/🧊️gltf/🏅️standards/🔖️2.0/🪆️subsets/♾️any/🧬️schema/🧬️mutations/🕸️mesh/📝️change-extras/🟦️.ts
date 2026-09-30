/** 📝️ `change-mesh-extra-data` wire twin: the flat `Apply` payload `GltfChangeMeshExtraDataPayload` and the phase wire `ChangeMeshExtraDataMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { type GltfJson, gltfWireIndex, gltfWireLiteral, gltfWireObject, gltfWireRequired, gltfWireTagged, parseGltfJson } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export type GltfDataPresence =
  | { state: "absent" }
  | { state: "present"; value: GltfJson };

export interface GltfChangeMeshExtraDataPayload {
  mesh: number;
  data: GltfDataPresence;
}

export type ChangeMeshExtraDataMutation = GltfPhase<GltfChangeMeshExtraDataPayload, GltfDiff>;

export const parseGltfDataPresence = gltfWireTagged<GltfDataPresence, "state">("state", {
  absent: gltfWireObject<Extract<GltfDataPresence, { state: "absent" }>>({ state: gltfWireRequired(gltfWireLiteral("absent")) }),
  present: gltfWireObject<Extract<GltfDataPresence, { state: "present" }>>({ state: gltfWireRequired(gltfWireLiteral("present")), value: gltfWireRequired(parseGltfJson) }),
});
export const parseGltfChangeMeshExtraDataPayload = gltfWireObject<GltfChangeMeshExtraDataPayload>({ mesh: gltfWireRequired(gltfWireIndex), data: gltfWireRequired(parseGltfDataPresence) });
export const parseChangeMeshExtraDataMutation = gltfWirePhase(parseGltfChangeMeshExtraDataPayload, parseGltfDiff);
