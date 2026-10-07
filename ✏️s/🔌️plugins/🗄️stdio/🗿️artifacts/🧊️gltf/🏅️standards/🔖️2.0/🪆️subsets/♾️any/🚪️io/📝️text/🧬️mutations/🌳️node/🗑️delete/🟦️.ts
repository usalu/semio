/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfDeleteNodePayload,DeleteNodeMutation} from "../../../../../🧬️schema/🧬️mutations/🌳️node/🗑️delete/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🌳️node/🗑️delete/🟦️.ts";
/** 🗑️ `delete-node` wire twin: the flat `Apply` payload `GltfDeleteNodePayload` and the phase wire `DeleteNodeMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfDeleteNodePayload = gltfWireObject<GltfDeleteNodePayload>({ index: gltfWireRequired(gltfWireIndex) });
export const parseDeleteNodeMutation = gltfWirePhase(parseGltfDeleteNodePayload, parseGltfDiff);
