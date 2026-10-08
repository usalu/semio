import {parseBlendMode,type BlendMode} from "../../../../../✳️any/🧬️schema/🟦️.ts";
/** 🔺️ Sparse diff builder for `SetLayerBlendMode` — one `blendMode` field patch. */
import type { SetLayerBlendMode } from "../🦠️mutation/🟦️.ts";

export function diff(payload: SetLayerBlendMode): { layers: { modified: Array<{ id: string; patch: { blendMode: BlendMode } }> } } {
  return { layers: { modified: [{ id: payload.layerId, patch: { blendMode: parseBlendMode(payload.blendMode) } }] } };
}
