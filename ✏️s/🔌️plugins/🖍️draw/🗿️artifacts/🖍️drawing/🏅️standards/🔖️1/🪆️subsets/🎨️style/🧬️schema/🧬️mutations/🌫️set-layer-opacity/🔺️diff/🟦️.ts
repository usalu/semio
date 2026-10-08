/** 🔺️ Sparse diff builder for `SetLayerOpacity` — one `opacity` field patch. */
import type { SetLayerOpacity } from "../🦠️mutation/🟦️.ts";

export function diff(payload: SetLayerOpacity): { layers: { modified: Array<{ id: string; patch: { opacity: number } }> } } {
  return { layers: { modified: [{ id: payload.layerId, patch: { opacity: payload.opacity } }] } };
}
