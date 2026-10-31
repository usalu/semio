/** ⏱️ The shared time source of the web renderer and its React hook; the framework-neutral class lives in the core so the custom element needs no React.
 *
 * @see ./🧱️core/🟦️.ts — {@link StaffageClock}
 * @see ../🧍️figure/🟦️.tsx — reads a clock frame by frame
 */
import { useSyncExternalStore } from "react";
import type { ClockState, StaffageClock } from "./🧱️core/🟦️.ts";

export { StaffageClock } from "./🧱️core/🟦️.ts";
export type { ClockOptions, ClockScheduler, ClockState } from "./🧱️core/🟦️.ts";

/** 🪝 Subscribes a component to a clock; a `null` clock yields `null`. Playback notifies when the frame or the playing flag changes; `seek`, `step` and the transport controls notify at once. */
export function useClock(clock: StaffageClock | null | undefined): ClockState | null {
  return useSyncExternalStore(clock?.subscribe ?? noSubscribe, clock?.getSnapshot ?? nullSnapshot, clock?.getSnapshot ?? nullSnapshot);
}

const noSubscribe = (): (() => void) => () => undefined;
const nullSnapshot = (): null => null;
