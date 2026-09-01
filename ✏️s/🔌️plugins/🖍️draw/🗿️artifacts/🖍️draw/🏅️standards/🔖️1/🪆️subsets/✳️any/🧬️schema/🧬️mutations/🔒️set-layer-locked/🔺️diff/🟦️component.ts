/** 🔺️ Sparse diff builder for `SetLayerLocked` — one `locked` field patch. */
import type { SetLayerLocked } from "../🦠️mutation/🟦️component.ts";

export function diff(payload: SetLayerLocked): { layers: { patched: Array<{ id: string; patch: { locked: boolean } }> } } {
  return { layers: { patched: [{ id: payload.layerId, patch: { locked: payload.locked } }] } };
}
