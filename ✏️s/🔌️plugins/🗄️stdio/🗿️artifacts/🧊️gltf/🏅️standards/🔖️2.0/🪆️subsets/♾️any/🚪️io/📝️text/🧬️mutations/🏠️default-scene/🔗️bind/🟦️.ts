/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfBindDefaultScenePayload,BindDefaultSceneMutation} from "../../../../../🧬️schema/🧬️mutations/🏠️default-scene/🔗️bind/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🏠️default-scene/🔗️bind/🟦️.ts";
/** 🔗️ `bind-default-scene` wire twin: the flat `Apply` payload `GltfBindDefaultScenePayload` and the phase wire `BindDefaultSceneMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfBindDefaultScenePayload = gltfWireObject<GltfBindDefaultScenePayload>({ scene: gltfWireRequired(gltfWireIndex) });
export const parseBindDefaultSceneMutation = gltfWirePhase(parseGltfBindDefaultScenePayload, parseGltfDiff);
