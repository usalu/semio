/** ↩️ energy-model replace-model/↩️inverse — mirror of the current-model round-trip inverse
 * (`replace` is its own inverse partner). */
import type { ReplaceModel } from "../🟦️component.ts";

export function inverse(currentModel: unknown): [ReplaceModel] {
  return [{ mutation: "replace-model", newModelJson: JSON.stringify(currentModel) }];
}
