/** 🔺️ Sparse diff builder for `SetLayerVisible` — one `visible` field patch. */
import type { SetLayerVisible } from "../🦠️mutation/🟦️.ts";

export function diff(payload: SetLayerVisible): { layers: { modified: Array<{ id: string; patch: { visible: boolean } }> } } {
  return { layers: { modified: [{ id: payload.layerId, patch: { visible: payload.visible } }] } };
}
