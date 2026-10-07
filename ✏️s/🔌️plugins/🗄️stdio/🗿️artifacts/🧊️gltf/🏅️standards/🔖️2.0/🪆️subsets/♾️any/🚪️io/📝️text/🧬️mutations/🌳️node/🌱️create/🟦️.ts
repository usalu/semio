/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfCreateNodePayload,CreateNodeMutation} from "../../../../../🧬️schema/🧬️mutations/🌳️node/🌱️create/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🌳️node/🌱️create/🟦️.ts";
/** 🌱️ `create-node` wire twin: the flat `Apply` payload `GltfCreateNodePayload` and the phase wire `CreateNodeMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfCreateNodePayload = gltfWireObject<GltfCreateNodePayload>({ position: gltfWireRequired(gltfWireIndex) });
export const parseCreateNodeMutation = gltfWirePhase(parseGltfCreateNodePayload, parseGltfDiff);
