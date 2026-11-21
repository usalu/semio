import type { JackSnapshot } from "../../📸️snapshot/🟦️.ts";

/** 🎯️ Sets or clears the Jack document's root node. */
export interface SetRoot {
  value: string | null;
}

/** 🌱️ Updates the root field of the parent snapshot. */
export function applySetRoot(snapshot: JackSnapshot, mutation: SetRoot): JackSnapshot {
  if ((snapshot.rootNodeId ?? null) === mutation.value) return snapshot;
  const { rootNodeId: _, ...parent } = snapshot;
  return mutation.value === null ? parent : { ...parent, rootNodeId: mutation.value };
}

/** ↩️ Restores the root before the edit. */
export function inverseSetRoot(snapshot: JackSnapshot): SetRoot {
  return { value: snapshot.rootNodeId ?? null };
}
