/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfCreateScenePayload,CreateSceneMutation} from "../../../../../🧬️schema/🧬️mutations/🎬️scene/🌱️create/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🎬️scene/🌱️create/🟦️.ts";
/** 🌱️ `create-scene` wire twin: the flat `Apply` payload `GltfCreateScenePayload` and the phase wire `CreateSceneMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireInteger, gltfWireObject, gltfWireRequired, gltfWireOptional, parseGltfScene } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfCreateScenePayload = gltfWireObject<GltfCreateScenePayload>({ position: gltfWireRequired(gltfWireInteger(4294967295)), scene: gltfWireOptional(parseGltfScene) });
export const parseCreateSceneMutation = gltfWireApplyPhase(parseGltfCreateScenePayload);
