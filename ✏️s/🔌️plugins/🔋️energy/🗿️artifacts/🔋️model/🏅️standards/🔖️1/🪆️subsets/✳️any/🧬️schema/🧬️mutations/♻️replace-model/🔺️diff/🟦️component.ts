/** 🔺️ energy-model replace-model/🔺️diff — mirror of the model-replace sparse diff builder; child
 * minting for `structure`/`zones` is a Rust-side effectful detail not representable here. */
import type { ReplaceModel } from "../🟦️component.ts";

export function diff(payload: ReplaceModel): { model: unknown } {
  return { model: JSON.parse(payload.newModelJson) };
}
