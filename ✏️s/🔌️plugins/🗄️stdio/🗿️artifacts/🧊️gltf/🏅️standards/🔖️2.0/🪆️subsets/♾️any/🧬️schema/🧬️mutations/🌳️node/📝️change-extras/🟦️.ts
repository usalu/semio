/** 📝️ `change-node-extra-data` wire twin: the flat `Apply` payload `GltfChangeNodeExtraDataPayload` and the phase wire `ChangeNodeExtraDataMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { type GltfJson, gltfWireIndex, gltfWireLiteral, gltfWireObject, gltfWireRequired, gltfWireTagged, parseGltfJson } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export type GltfDataPresence =
  | { state: "absent" }
  | { state: "present"; value: GltfJson };

export interface GltfChangeNodeExtraDataPayload {
  node: bigint;
  data: GltfDataPresence;
}

export type ChangeNodeExtraDataMutation = GltfPhase<GltfChangeNodeExtraDataPayload, GltfDiff>;

export const parseGltfDataPresence = gltfWireTagged<GltfDataPresence, "state">("state", {
  absent: gltfWireObject<Extract<GltfDataPresence, { state: "absent" }>>({ state: gltfWireRequired(gltfWireLiteral("absent")) }),
  present: gltfWireObject<Extract<GltfDataPresence, { state: "present" }>>({ state: gltfWireRequired(gltfWireLiteral("present")), value: gltfWireRequired(parseGltfJson) }),
});
export const parseGltfChangeNodeExtraDataPayload = gltfWireObject<GltfChangeNodeExtraDataPayload>({ node: gltfWireRequired(gltfWireIndex), data: gltfWireRequired(parseGltfDataPresence) });
export const parseChangeNodeExtraDataMutation = gltfWirePhase(parseGltfChangeNodeExtraDataPayload, parseGltfDiff);
