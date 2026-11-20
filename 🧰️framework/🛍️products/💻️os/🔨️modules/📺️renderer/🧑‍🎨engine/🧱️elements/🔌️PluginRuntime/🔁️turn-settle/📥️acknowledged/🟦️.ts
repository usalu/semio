import type { WireTurnResult } from "../../🪝️wire-codec/🟦️.ts";

/** 📥️ Keeps pending settlement outputs and the final actor status once ingress and patch receipts have been consumed. */
export function compactAcknowledgedTurns(turns: WireTurnResult[], previouslyFolded: number): void {
  if (previouslyFolded === turns.length) return;
  let kept = Math.max(0, previouslyFolded - 1);
  for (let index = kept; index < turns.length; index++) {
    const turn = turns[index]!;
    if (index === turns.length - 1 || turn.uiPatches.length || turn.effects.length || turn.presence?.length || turn.nextWake !== null) {
      turns[kept++] = turn;
    }
  }
  turns.length = kept;
}
