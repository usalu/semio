/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfChangePrimitiveTopologyModePayload,ChangePrimitiveTopologyModeMutation} from "../../../../../🧬️schema/🧬️mutations/🔺️primitive/📐️change-topology/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🔺️primitive/📐️change-topology/🟦️.ts";
/** 📐️ `change-primitive-topology-mode` wire twin: the flat `Apply` payload `GltfChangePrimitiveTopologyModePayload` and the phase wire `ChangePrimitiveTopologyModeMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfChangePrimitiveTopologyModePayload = gltfWireObject<GltfChangePrimitiveTopologyModePayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), mode: gltfWireRequired(gltfWireIndex) });
export const parseChangePrimitiveTopologyModeMutation = gltfWirePhase(parseGltfChangePrimitiveTopologyModePayload, parseGltfDiff);
