/** ↩️ Inverse for `SetLayerLocked` — reconstructed from BASE state, never post-state. Missing
 * target ⇒ `[]`. */
import type { SetLayerLocked } from "../🦠️mutation/🟦️component.ts";

export function inverse(payload: SetLayerLocked, baseLocked: boolean | undefined): SetLayerLocked[] {
  return baseLocked === undefined ? [] : [{ layerId: payload.layerId, locked: baseLocked }];
}
