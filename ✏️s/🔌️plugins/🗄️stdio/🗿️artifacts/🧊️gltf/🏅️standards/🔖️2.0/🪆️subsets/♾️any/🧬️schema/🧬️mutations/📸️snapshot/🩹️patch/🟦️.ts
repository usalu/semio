/** 🩹️ `patch-snapshot` wire twin: the flat payload `PatchSnapshot`, exactly as `./🦀️.rs` writes it.
 * @see ./🧬️schema/🔣️.json */
import { parseSnapshotPatch, type SnapshotPatch } from "../../../../../../../../../../📇️registry/🧬️contract/✏️editing/🩹️patch/🟦️.ts";
import { gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";

export interface PatchSnapshot {
  patch: SnapshotPatch;
}

export const parsePatchSnapshot = gltfWireObject<PatchSnapshot>({ patch: gltfWireRequired((value) => parseSnapshotPatch(value)) });
