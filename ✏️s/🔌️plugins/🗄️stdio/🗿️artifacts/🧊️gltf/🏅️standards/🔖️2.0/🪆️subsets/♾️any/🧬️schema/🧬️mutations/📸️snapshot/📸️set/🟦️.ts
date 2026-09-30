/** 📸️ `set-snapshot` wire twin: the flat `Apply` payload `SetSnapshot`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { type GltfSnapshot, gltfWireObject, gltfWireRequired, parseGltfSnapshot } from "../../../📸️snapshot/🟦️.ts";

export interface SetSnapshot {
  snapshot: GltfSnapshot;
}

export const parseSetSnapshot = gltfWireObject<SetSnapshot>({ snapshot: gltfWireRequired(parseGltfSnapshot) });
