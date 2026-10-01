/** 📝️ `change-scene-extra-data` wire twin: the flat `Apply` payload `GltfChangeSceneExtraDataPayload` and the phase wire `ChangeSceneExtraDataMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { type GltfJson, gltfWireIndex, gltfWireLiteral, gltfWireObject, gltfWireRequired, gltfWireTagged, parseGltfJson } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export type GltfDataPresence =
  | { state: "absent" }
  | { state: "present"; value: GltfJson };

export interface GltfChangeSceneExtraDataPayload {
  scene: bigint;
  data: GltfDataPresence;
}

export type ChangeSceneExtraDataMutation = GltfPhase<GltfChangeSceneExtraDataPayload, GltfDiff>;

export const parseGltfDataPresence = gltfWireTagged<GltfDataPresence, "state">("state", {
  absent: gltfWireObject<Extract<GltfDataPresence, { state: "absent" }>>({ state: gltfWireRequired(gltfWireLiteral("absent")) }),
  present: gltfWireObject<Extract<GltfDataPresence, { state: "present" }>>({ state: gltfWireRequired(gltfWireLiteral("present")), value: gltfWireRequired(parseGltfJson) }),
});
export const parseGltfChangeSceneExtraDataPayload = gltfWireObject<GltfChangeSceneExtraDataPayload>({ scene: gltfWireRequired(gltfWireIndex), data: gltfWireRequired(parseGltfDataPresence) });
export const parseChangeSceneExtraDataMutation = gltfWirePhase(parseGltfChangeSceneExtraDataPayload, parseGltfDiff);
