/** ⏱️ The shared time source of the web renderer and its React hook; the framework-neutral class lives in the core so the custom element needs no React.
 *
 * @see ./🧱️core/🟦️.ts — {@link StaffageClock}
 * @see ../🧍️figure/🟦️.tsx — reads a clock frame by frame
 */
import { useCallback, useRef, useSyncExternalStore } from "react";
import type { ClockState, StaffageClock } from "./🧱️core/🟦️.ts";

export { StaffageClock } from "./🧱️core/🟦️.ts";
export type { ClockOptions, ClockScheduler, ClockState } from "./🧱️core/🟦️.ts";

/** 🪝 Subscribes a component to a clock; a `null` clock yields `null`. Playback notifies when the frame or the playing flag changes; `seek`, `step` and the transport controls notify at once. */
export function useClock(clock: StaffageClock | null | undefined): ClockState | null {
  return useSyncExternalStore(clock?.subscribe ?? noSubscribe, clock?.getSnapshot ?? nullSnapshot, clock?.getSnapshot ?? nullSnapshot);
}

/** 🧊️ The frame time of a clock in seconds, held while `active` is false: an idle figure (offscreen, or showing a pose that ignores time) neither subscribes nor re-renders, and catches up to the clock the moment it becomes active. A `null` clock yields `null`. */
export function useHeldTime(clock: StaffageClock | null | undefined, active: boolean): number | null {
  const last = useRef<number | null>(null);
  const live = clock != null && active;
  const subscribe = useCallback((listener: () => void) => (live ? clock.subscribe(listener) : noUnsubscribe), [clock, live]);
  const snapshot = useCallback((): number | null => {
    if (clock == null) return null;
    if (live || last.current === null) last.current = clock.state.frame / clock.state.fps;
    return last.current;
  }, [clock, live]);
  return useSyncExternalStore(subscribe, snapshot, snapshot);
}

const noUnsubscribe = (): void => undefined;
const noSubscribe = (): (() => void) => () => undefined;
const nullSnapshot = (): null => null;
