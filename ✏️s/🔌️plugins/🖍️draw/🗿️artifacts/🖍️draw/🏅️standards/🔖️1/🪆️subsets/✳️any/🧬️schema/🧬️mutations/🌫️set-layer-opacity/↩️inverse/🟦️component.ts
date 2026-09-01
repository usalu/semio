/** ↩️ Inverse for `SetLayerOpacity` — reconstructed from BASE state, never post-state. Missing
 * target ⇒ `[]`. */
import type { SetLayerOpacity } from "../🦠️mutation/🟦️component.ts";

export function inverse(payload: SetLayerOpacity, baseOpacity: number | undefined): SetLayerOpacity[] {
  return baseOpacity === undefined ? [] : [{ layerId: payload.layerId, opacity: baseOpacity }];
}
