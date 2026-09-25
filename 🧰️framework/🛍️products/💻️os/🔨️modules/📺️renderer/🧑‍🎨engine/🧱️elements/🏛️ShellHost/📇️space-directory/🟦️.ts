/**
 * 📇️ The directory history one mounted space index folds. The space index is the shell's projection of a space's
 * directory, not a hub document: its guest folds the FULL event history of its space on every `foldDirectoryEvents`
 * (`🎮️commands/📇fold-directory-events`, `fold_all` from the empty read model), so a lone live event or a command
 * receipt folded on its own names no space and changes nothing. This history collects every event of its space from
 * every lane that carries one — the worker's space lane (sealed history, then live), the global stream, command
 * receipts — once each by `seq`, and answers them in ascending order.
 * @see ../../../../../🏪️store/👷️worker/🟦️.ts
 * @see ../../../../../../../../../✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📇fold-directory-events/🦀️.rs
 */
import type { DirectoryEvent } from "../../../../../📇️directory/🟦️.ts";

/** 📇️ One space's directory history, deduplicated by `seq`. */
export class SpaceDirectoryHistoryV1 {
  private readonly bySeq = new Map<number, DirectoryEvent>();
  private ordered: readonly DirectoryEvent[] = [];

  constructor(readonly spaceId: string) {}

  /** Adds every event of this space; answers whether the history changed. */
  add(events: readonly DirectoryEvent[]): boolean {
    let changed = false;
    for (const event of events) {
      if (event.spaceId !== this.spaceId || this.bySeq.has(event.seq)) continue;
      this.bySeq.set(event.seq, event);
      changed = true;
    }
    if (changed) this.ordered = [...this.bySeq.values()].sort((left, right) => left.seq - right.seq);
    return changed;
  }

  /** Every event of the space, in ascending `seq`: what one fold receives. */
  events(): readonly DirectoryEvent[] {
    return this.ordered;
  }
}
