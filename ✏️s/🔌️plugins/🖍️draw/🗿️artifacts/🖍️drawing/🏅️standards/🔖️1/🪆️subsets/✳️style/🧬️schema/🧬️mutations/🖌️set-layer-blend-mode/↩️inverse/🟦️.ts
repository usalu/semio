/** ↩️ Inverse for `SetLayerBlendMode` — reconstructed from BASE state, never post-state. Missing
 * target ⇒ `[]`. */
import type { SetLayerBlendMode } from "../🦠️mutation/🟦️.ts";

export function inverse(payload: SetLayerBlendMode, baseBlendMode: string | undefined): SetLayerBlendMode[] {
  return baseBlendMode === undefined ? [] : [{ layerId: payload.layerId, blendMode: baseBlendMode }];
}
