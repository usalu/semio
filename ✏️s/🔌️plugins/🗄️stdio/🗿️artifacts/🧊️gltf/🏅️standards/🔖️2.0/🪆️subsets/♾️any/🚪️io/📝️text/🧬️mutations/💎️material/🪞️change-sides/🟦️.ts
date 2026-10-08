/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfChangeMaterialDoubleSidedPayload,ChangeMaterialDoubleSidedMutation} from "../../../../../🧬️schema/🧬️mutations/💎️material/🪞️change-sides/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/💎️material/🪞️change-sides/🟦️.ts";
/** 🪞️ `change-material-double-sided` wire twin: the flat `Apply` payload `GltfChangeMaterialDoubleSidedPayload` and the phase wire `ChangeMaterialDoubleSidedMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireBoolean, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfChangeMaterialDoubleSidedPayload = gltfWireObject<GltfChangeMaterialDoubleSidedPayload>({ material: gltfWireRequired(gltfWireIndex), doubleSided: gltfWireRequired(gltfWireBoolean) });
export const parseChangeMaterialDoubleSidedMutation = gltfWireApplyPhase(parseGltfChangeMaterialDoubleSidedPayload);
