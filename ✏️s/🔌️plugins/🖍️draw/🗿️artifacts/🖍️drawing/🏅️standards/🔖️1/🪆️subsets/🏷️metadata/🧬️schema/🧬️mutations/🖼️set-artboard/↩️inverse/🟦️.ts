/** ↩️ Inverse for `SetArtboard` — reconstructed from BASE state, never post-state. */
import type { SetArtboard } from "../🦠️mutation/🟦️.ts";

export function inverse(_payload: SetArtboard, baseArtboard: SetArtboard["artboard"]): SetArtboard[] {
  return [{ artboard: baseArtboard === null ? null : { width: baseArtboard.width, height: baseArtboard.height } }];
}
