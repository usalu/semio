/** 🧱️ Items that appear a few at a time: the first one with the task, the rest in chunks that React renders between frames, so that opening a sheet is never one long task however many items its tasks hold.
 *
 * @see ../▶️run/🟦️.tsx — where the task views are opened
 * @see https://react.dev/reference/react/startTransition
 */

import { startTransition, useEffect, useState } from "react";

/** 🥇️ How many items a task renders in its first commit. */
export const ITEMS_FIRST = 1;

/** 🍱️ How many more items each following commit renders. */
export const ITEM_CHUNK = 4;

/** 🪜️ How many of `total` items to render now: {@link ITEMS_FIRST} at first and {@link ITEM_CHUNK} more with every following commit, never more than `total`. `list` names the list: another list starts again, the same list never takes back what it showed. */
export function useGradualCount(total: number, list: string): number {
  const [state, setState] = useState({ list, shown: ITEMS_FIRST });
  const shown = state.list === list ? state.shown : ITEMS_FIRST;
  if (state.list !== list) setState({ list, shown });
  const count = Math.min(total, shown);
  useEffect(() => {
    if (count >= total) return;
    startTransition(() => setState((known) => (known.list === list ? { list, shown: known.shown + ITEM_CHUNK } : known)));
  }, [count, total, list]);
  return count;
}
