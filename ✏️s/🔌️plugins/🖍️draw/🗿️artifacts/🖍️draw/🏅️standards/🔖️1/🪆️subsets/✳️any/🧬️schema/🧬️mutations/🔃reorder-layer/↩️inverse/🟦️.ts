/** ↩️ Inverse for `ReorderLayer` — the OLD `(parentId, index)` address, captured from BASE by the
 * caller. Missing target ⇒ `[]`. */
import type { ReorderLayer } from "../🦠️mutation/🟦️.ts";

export function inverse(payload: ReorderLayer, baseLocation: { parentId?: string; index: number } | undefined): ReorderLayer[] {
  return baseLocation === undefined ? [] : [{ layerId: payload.layerId, ...(baseLocation.parentId !== undefined ? { parentId: baseLocation.parentId } : {}), index: baseLocation.index }];
}
