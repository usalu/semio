/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfChangePrimitiveTopologyModePayload,ChangePrimitiveTopologyModeMutation} from "../../../../../🧬️schema/🧬️mutations/🔺️primitive/📐️change-topology/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🔺️primitive/📐️change-topology/🟦️.ts";
/** 📐️ `change-primitive-topology-mode` wire twin: the flat `Apply` payload `GltfChangePrimitiveTopologyModePayload` and the phase wire `ChangePrimitiveTopologyModeMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireNullable, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfChangePrimitiveTopologyModePayload = gltfWireObject<GltfChangePrimitiveTopologyModePayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), mode: gltfWireRequired(gltfWireNullable(gltfWireIndex)) });
export const parseChangePrimitiveTopologyModeMutation = gltfWireApplyPhase(parseGltfChangePrimitiveTopologyModePayload);
