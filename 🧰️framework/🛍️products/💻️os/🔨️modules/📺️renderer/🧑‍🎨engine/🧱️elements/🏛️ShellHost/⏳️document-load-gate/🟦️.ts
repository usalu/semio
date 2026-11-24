/** ⏳️ Holds the inputs a loading document cannot take, so the guest never has to refuse them.
 *
 * 🧭️ The guest answers `document.loading` to every verb that would read or edit the document being replaced
 * (`🔌️plugin/🧵️instance/📄️docs/📥️staging/🧷️command-routes/🦀️.rs`); that refusal fails the whole actor turn and its held
 * refusal then fails the load itself. The host therefore decides first, per input: `pass` it, `drop` an ephemeral chrome
 * note, or `defer` it until the load has settled, in arrival order. */
import { sessionInstanceKeyV1 } from "../🔀️surface-switch/🟦️.ts";

export const DOCUMENT_LOADING_CODE_V1 = "document.loading";
export const DOCUMENT_LOAD_DEFER_CAPACITY_V1 = 64;

export type DocumentLoadWaitPolicyV1 = "pass" | "drop" | "defer";

/** 🚦️ What a load does to one input: the ephemeral shell-chrome journal note is dropped (the replaced document owns no
 * row for it), every document-touching kind waits, view, history and interaction verbs pass (the guest admits them, and
 * `historyEditCancelReplay` must reach it to cancel the load). */
export function documentLoadWaitPolicyV1(actionId: string, kind: string | undefined, ephemeralNoteActionId: string): DocumentLoadWaitPolicyV1 {
  if (actionId === ephemeralNoteActionId) return "drop";
  return kind === "mutation" || kind === "clipboard" ? "defer" : "pass";
}

export type DocumentLoadGateV1 = {
  readonly begin: (pluginId: string, instanceId: number) => () => void;
  readonly loading: (pluginId: string, instanceId: number) => boolean;
  readonly deferred: (pluginId: string, instanceId: number) => number;
  readonly settled: (pluginId: string, instanceId: number, signal?: AbortSignal) => Promise<"settled" | "queue-full">;
};

type Waiter = { readonly resolve: () => void; readonly reject: (reason: unknown) => void };
type Slot = { loads: number; readonly waiters: Waiter[] };

/** 🚪️ Counts the loads running per `(pluginId, instanceId)` and parks the deferred inputs behind them. `begin` answers an
 * idempotent release; the last release wakes every parked input in the order it arrived, a cancelled one leaves the queue. */
export function createDocumentLoadGateV1(capacity: number = DOCUMENT_LOAD_DEFER_CAPACITY_V1): DocumentLoadGateV1 {
  const slots = new Map<string, Slot>();
  const key = sessionInstanceKeyV1;
  return {
    begin: (pluginId, instanceId) => {
      const id = key(pluginId, instanceId);
      const slot = slots.get(id) ?? { loads: 0, waiters: [] };
      slots.set(id, slot);
      slot.loads += 1;
      let released = false;
      return () => {
        if (released) return;
        released = true;
        slot.loads -= 1;
        if (slot.loads > 0) return;
        slots.delete(id);
        for (const waiter of slot.waiters.splice(0)) waiter.resolve();
      };
    },
    loading: (pluginId, instanceId) => (slots.get(key(pluginId, instanceId))?.loads ?? 0) > 0,
    deferred: (pluginId, instanceId) => slots.get(key(pluginId, instanceId))?.waiters.length ?? 0,
    settled: (pluginId, instanceId, signal) => {
      const slot = slots.get(key(pluginId, instanceId));
      if (slot === undefined) return Promise.resolve("settled");
      if (slot.waiters.length >= capacity) return Promise.resolve("queue-full");
      if (signal?.aborted) return Promise.reject(signal.reason);
      return new Promise<"settled" | "queue-full">((resolve, reject) => {
        const waiter: Waiter = { resolve: () => { signal?.removeEventListener("abort", abort); resolve("settled"); }, reject };
        const abort = () => {
          const at = slot.waiters.indexOf(waiter);
          if (at >= 0) slot.waiters.splice(at, 1);
          reject(signal!.reason);
        };
        signal?.addEventListener("abort", abort, { once: true });
        slot.waiters.push(waiter);
      });
    },
  };
}
