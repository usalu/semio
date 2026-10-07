/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {SetSnapshot} from "../../../../../🧬️schema/🧬️mutations/📸️snapshot/📸️set/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/📸️snapshot/📸️set/🟦️.ts";
/** 📸️ `set-snapshot` wire twin: the flat `Apply` payload `SetSnapshot`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { type GltfSnapshot, gltfWireObject, gltfWireRequired, parseGltfSnapshot } from "../../../📸️snapshot/🔣️json/🟦️.ts";

export const parseSetSnapshot = gltfWireObject<SetSnapshot>({ snapshot: gltfWireRequired(parseGltfSnapshot) });
