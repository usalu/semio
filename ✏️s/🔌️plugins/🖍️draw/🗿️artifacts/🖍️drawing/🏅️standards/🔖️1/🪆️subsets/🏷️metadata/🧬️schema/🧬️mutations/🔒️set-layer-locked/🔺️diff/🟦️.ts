/** 🔺️ Sparse diff builder for `SetLayerLocked` — one `locked` field patch. */
import type { SetLayerLocked } from "../🦠️mutation/🟦️.ts";

export function diff(payload: SetLayerLocked): { layers: { modified: Array<{ id: string; patch: { locked: boolean } }> } } {
  return { layers: { modified: [{ id: payload.layerId, patch: { locked: payload.locked } }] } };
}
