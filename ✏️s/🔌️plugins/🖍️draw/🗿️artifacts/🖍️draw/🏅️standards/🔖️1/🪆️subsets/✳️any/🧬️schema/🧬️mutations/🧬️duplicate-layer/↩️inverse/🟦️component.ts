/** ↩️ Inverse for `DuplicateLayer` — `delete-layer` of the deterministic duplicate id, recomputed
 * by the caller via the same content-addressed hash `diff` used to create it (hashing is not
 * mirrored here). Missing source ⇒ `[]`. */
import type { DuplicateLayer } from "../🦠️mutation/🟦️component.ts";

export function inverse(payload: DuplicateLayer, duplicateLayerId: string | undefined): Array<{ layerId: string }> {
  return duplicateLayerId === undefined ? [] : [{ layerId: duplicateLayerId }];
}
