/** ↩️ Inverse for `SetLayerVisible` — reconstructed from BASE state, never post-state. Missing
 * target ⇒ `[]`. */
import type { SetLayerVisible } from "../🦠️mutation/🟦️.ts";

export function inverse(payload: SetLayerVisible, baseVisible: boolean | undefined): SetLayerVisible[] {
  return baseVisible === undefined ? [] : [{ layerId: payload.layerId, visible: baseVisible }];
}
