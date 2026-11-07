/** 🔺️ Sparse diff builder for `SetArtboard` — one `artboard` field patch. */
import type { SetArtboard } from "../🦠️mutation/🟦️.ts";

export function diff(payload: SetArtboard): { artboard: SetArtboard["artboard"] } {
  return { artboard: payload.artboard === null ? null : { width: payload.artboard.width, height: payload.artboard.height } };
}
