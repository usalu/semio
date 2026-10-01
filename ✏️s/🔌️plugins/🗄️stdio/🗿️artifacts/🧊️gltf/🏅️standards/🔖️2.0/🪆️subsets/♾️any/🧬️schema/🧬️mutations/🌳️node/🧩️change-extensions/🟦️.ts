/** 🧩️ `change-node-extension-data` wire twin: the flat `Apply` payload `GltfChangeNodeExtensionDataPayload` and the phase wire `ChangeNodeExtensionDataMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { type GltfJson, gltfWireIndex, gltfWireLiteral, gltfWireObject, gltfWireRequired, gltfWireTagged, parseGltfJson } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export type GltfDataPresence =
  | { state: "absent" }
  | { state: "present"; value: GltfJson };

export interface GltfChangeNodeExtensionDataPayload {
  node: bigint;
  data: GltfDataPresence;
}

export type ChangeNodeExtensionDataMutation = GltfPhase<GltfChangeNodeExtensionDataPayload, GltfDiff>;

export const parseGltfDataPresence = gltfWireTagged<GltfDataPresence, "state">("state", {
  absent: gltfWireObject<Extract<GltfDataPresence, { state: "absent" }>>({ state: gltfWireRequired(gltfWireLiteral("absent")) }),
  present: gltfWireObject<Extract<GltfDataPresence, { state: "present" }>>({ state: gltfWireRequired(gltfWireLiteral("present")), value: gltfWireRequired(parseGltfJson) }),
});
export const parseGltfChangeNodeExtensionDataPayload = gltfWireObject<GltfChangeNodeExtensionDataPayload>({ node: gltfWireRequired(gltfWireIndex), data: gltfWireRequired(parseGltfDataPresence) });
export const parseChangeNodeExtensionDataMutation = gltfWirePhase(parseGltfChangeNodeExtensionDataPayload, parseGltfDiff);
