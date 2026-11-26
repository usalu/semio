/** 📋️ Delivers clipboard effects to their active workspace at the actor crossing. */
import type { WireTurnResult } from "../🪝️wire-codec/🟦️.ts";
import { wireEffectToFriendly } from "../🪟️retained-ui-patch/🟦️.ts";

type ClipboardWriter = (fragment: unknown) => boolean;
const writersByActor = new Map<string, Set<ClipboardWriter>>();

/** ✍️ Receives this actor's workspace write while its copy or cut owns the clipboard. */
export function subscribeWorkspaceClipboardWrites(actorId: string, write: ClipboardWriter): () => void {
  const writers = writersByActor.get(actorId) ?? new Set<ClipboardWriter>();
  writers.add(write);
  writersByActor.set(actorId, writers);
  return () => {
    writers.delete(write);
    if (!writers.size && writersByActor.get(actorId) === writers) writersByActor.delete(actorId);
  };
}

/** 📨️ Installs a live workspace write before the turn's frames can settle a queued paste. */
export function deliverWorkspaceClipboardTurn(actorId: string, turn: WireTurnResult): WireTurnResult {
  const writers = writersByActor.get(actorId);
  if (!writers || !turn.effects.some(effect => effect.tag === "clipboard-write")) return turn;
  const remaining = turn.effects.filter(effect => {
    if (effect.tag !== "clipboard-write") return true;
    const decoded = wireEffectToFriendly(effect);
    if (!decoded || typeof decoded !== "object" || !("clipboardWrite" in decoded)) return true;
    for (const write of writers) if (write(decoded.clipboardWrite.fragment)) return false;
    return true;
  });
  return remaining.length === turn.effects.length ? turn : { ...turn, effects: remaining };
}
