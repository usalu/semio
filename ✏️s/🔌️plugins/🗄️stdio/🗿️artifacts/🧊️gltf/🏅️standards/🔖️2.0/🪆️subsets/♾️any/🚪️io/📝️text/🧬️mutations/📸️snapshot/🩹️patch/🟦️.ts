/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {PatchSnapshot} from "../../../../../🧬️schema/🧬️mutations/📸️snapshot/🩹️patch/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/📸️snapshot/🩹️patch/🟦️.ts";
/** 🩹️ `patch-snapshot` wire twin: the flat payload `PatchSnapshot`, exactly as `./🦀️.rs` writes it.
 * @see ./🧬️schema/🔣️.json */
import { parseSnapshotPatch, type SnapshotPatch } from "../../../../../../../../../../../📇️registry/🧬️contract/✏️editing/🩹️patch/🟦️.ts";
import { gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";

export const parsePatchSnapshot = gltfWireObject<PatchSnapshot>({ patch: gltfWireRequired((value) => parseSnapshotPatch(value)) });
