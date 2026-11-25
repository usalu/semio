/** 🪝 The React reader of the text epoch: a component that sizes a window around a drawing re-renders when a face finishes loading, because the texts are then wider or narrower than measured.
 *
 * @see ../🟦️.ts — the measure and its epoch
 */
import { useSyncExternalStore } from "react";
import { subscribeTextEpoch, textEpoch } from "../🟦️.ts";

/** 🕰️ The text epoch as a dependency of every memo that reads `sceneExtent` or `sceneWindow`. */
export function useTextEpoch(): number {
  return useSyncExternalStore(subscribeTextEpoch, textEpoch, () => 0);
}
