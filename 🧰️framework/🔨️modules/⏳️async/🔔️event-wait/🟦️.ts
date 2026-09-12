//#region 🔔️WaitForEvent
/** @emoji 🔔️ Subscribes `handler` to fire on the next occurrence and returns an unsubscribe. */
export type EventSubscribe<T> = (handler: (value: T) => void) => () => void;

/** @emoji 🔔️ Options for {@link waitForEvent}. */
export interface WaitForEventOptions {
  readonly signal?: AbortSignal;
}

/**
 * @emoji 🔔️ One-shot event-driven gate: resolves with the first value `subscribe` delivers, or
 * rejects if `signal` aborts first (including if it is already aborted). The subscription is torn
 * down on both exit paths — no listener survives past this call, unlike a fixed `setTimeout` wait
 * that either fires early/late or leaks if nothing ever arrives.
 * Einmaliges ereignisgesteuertes Warten: löst beim ersten Ereignis auf oder lehnt bei Abbruch ab; das
 * Abonnement wird in beiden Fällen entfernt.
 */
export function waitForEvent<T>(subscribe: EventSubscribe<T>, options?: WaitForEventOptions): Promise<T> {
  const signal = options?.signal;
  return new Promise<T>((resolve, reject) => {
    if (signal?.aborted) {
      reject(signal.reason ?? new Error("waitForEvent: aborted"));
      return;
    }
    let unsubscribe: (() => void) | null = null;
    function cleanup(): void {
      unsubscribe?.();
      unsubscribe = null;
      signal?.removeEventListener("abort", onAbort);
    }
    function onAbort(): void {
      cleanup();
      reject(signal!.reason ?? new Error("waitForEvent: aborted"));
    }
    unsubscribe = subscribe((value) => {
      cleanup();
      resolve(value);
    });
    signal?.addEventListener("abort", onAbort, { once: true });
  });
}
//#endregion 🔔️WaitForEvent
