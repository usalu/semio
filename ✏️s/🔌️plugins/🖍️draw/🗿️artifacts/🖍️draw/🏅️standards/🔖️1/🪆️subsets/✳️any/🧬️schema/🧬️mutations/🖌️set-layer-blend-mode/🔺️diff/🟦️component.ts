/** 🔺️ Sparse diff builder for `SetLayerBlendMode` — one `blendMode` field patch. */
import type { SetLayerBlendMode } from "../🦠️mutation/🟦️component.ts";

export function diff(payload: SetLayerBlendMode): { layers: { patched: Array<{ id: string; patch: { blendMode: string } }> } } {
  return { layers: { patched: [{ id: payload.layerId, patch: { blendMode: payload.blendMode } }] } };
}
