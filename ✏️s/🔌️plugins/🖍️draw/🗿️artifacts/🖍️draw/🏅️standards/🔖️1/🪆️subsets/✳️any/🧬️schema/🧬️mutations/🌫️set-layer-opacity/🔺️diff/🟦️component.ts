/** 🔺️ Sparse diff builder for `SetLayerOpacity` — one `opacity` field patch. */
import type { SetLayerOpacity } from "../🦠️mutation/🟦️component.ts";

export function diff(payload: SetLayerOpacity): { layers: { patched: Array<{ id: string; patch: { opacity: number } }> } } {
  return { layers: { patched: [{ id: payload.layerId, patch: { opacity: payload.opacity } }] } };
}
