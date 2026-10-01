/** 🧩️ `change-primitive-extension-data` wire twin: the flat `Apply` payload `GltfChangePrimitiveExtensionDataPayload` and the phase wire `ChangePrimitiveExtensionDataMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { type GltfJson, gltfWireIndex, gltfWireLiteral, gltfWireObject, gltfWireRequired, gltfWireTagged, parseGltfJson } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export type GltfDataPresence =
  | { state: "absent" }
  | { state: "present"; value: GltfJson };

export interface GltfChangePrimitiveExtensionDataPayload {
  mesh: bigint;
  primitive: bigint;
  data: GltfDataPresence;
}

export type ChangePrimitiveExtensionDataMutation = GltfPhase<GltfChangePrimitiveExtensionDataPayload, GltfDiff>;

export const parseGltfDataPresence = gltfWireTagged<GltfDataPresence, "state">("state", {
  absent: gltfWireObject<Extract<GltfDataPresence, { state: "absent" }>>({ state: gltfWireRequired(gltfWireLiteral("absent")) }),
  present: gltfWireObject<Extract<GltfDataPresence, { state: "present" }>>({ state: gltfWireRequired(gltfWireLiteral("present")), value: gltfWireRequired(parseGltfJson) }),
});
export const parseGltfChangePrimitiveExtensionDataPayload = gltfWireObject<GltfChangePrimitiveExtensionDataPayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), data: gltfWireRequired(parseGltfDataPresence) });
export const parseChangePrimitiveExtensionDataMutation = gltfWirePhase(parseGltfChangePrimitiveExtensionDataPayload, parseGltfDiff);
