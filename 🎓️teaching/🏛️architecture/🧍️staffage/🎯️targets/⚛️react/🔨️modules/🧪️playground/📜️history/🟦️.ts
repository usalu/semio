/** 📜️ In-memory undo and redo of playground edits: a bounded stack of whole playground states, never persisted.
 *
 * @see ../🟦️.tsx — undo and redo replay a state through `playground-replaced`
 */
import { useCallback, useRef, useState } from "react";
import type { Playground } from "../../💾️persistence/🟦️.ts";

/** 📏️ How many edits can be undone. */
export const HISTORY_LIMIT = 50;

/** 📜️ Undo and redo over a source of truth that is read and replaced by the caller. */
export function useHistory(current: () => Playground, replace: (playground: Playground) => void): { readonly record: () => void; readonly undo: () => void; readonly redo: () => void; readonly canUndo: boolean; readonly canRedo: boolean } {
  const past = useRef<Playground[]>([]);
  const future = useRef<Playground[]>([]);
  const [, bump] = useState(0);
  const record = useCallback(() => {
    past.current = [...past.current.slice(-(HISTORY_LIMIT - 1)), current()];
    future.current = [];
    bump((count) => count + 1);
  }, [current]);
  const undo = useCallback(() => {
    const previous = past.current.at(-1);
    if (previous === undefined) return;
    future.current = [...future.current, current()];
    past.current = past.current.slice(0, -1);
    replace(previous);
    bump((count) => count + 1);
  }, [current, replace]);
  const redo = useCallback(() => {
    const next = future.current.at(-1);
    if (next === undefined) return;
    past.current = [...past.current, current()];
    future.current = future.current.slice(0, -1);
    replace(next);
    bump((count) => count + 1);
  }, [current, replace]);
  return { record, undo, redo, canUndo: past.current.length > 0, canRedo: future.current.length > 0 };
}
