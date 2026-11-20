/** 💤️ Work that waits for idle time: the code of the screens a learner opens next is fetched and compiled while the home screen rests, so opening a quiz is not the moment a large chunk is parsed.
 *
 * @see ../📚️app/🟦️.tsx — where the run and results screens are wished
 * @see https://developer.mozilla.org/en-US/docs/Web/API/Window/requestIdleCallback
 */

import { useEffect, useRef } from "react";

/** ⏲️ How long a browser without idle callbacks waits before the work runs, in milliseconds. */
export const IDLE_FALLBACK_MS = 400;

/** 🛋️ Runs `work` once when the browser is idle (after {@link IDLE_FALLBACK_MS} where it has no idle callbacks) and returns the function that calls it off. */
export function whenIdle(work: () => void): () => void {
  if (typeof requestIdleCallback === "function") {
    const handle = requestIdleCallback(work);
    return () => cancelIdleCallback(handle);
  }
  const timer = setTimeout(work, IDLE_FALLBACK_MS);
  return () => clearTimeout(timer);
}

/** 📦️ Starts every function of `loads` when the browser is idle while `active`, once per activation; a screen that goes first calls them off. */
export function usePreload(active: boolean, loads: readonly (() => unknown)[]): void {
  const latest = useRef(loads);
  latest.current = loads;
  useEffect(() => {
    if (!active) return undefined;
    return whenIdle(() => {
      for (const load of latest.current) load();
    });
  }, [active]);
}
