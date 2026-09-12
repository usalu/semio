//#region 🔁️RetryWithJitteredBackoff
/** @emoji 🔁️ Options for {@link retryWithJitteredBackoff}. */
export interface JitteredBackoffOptions {
  readonly minMs: number;
  readonly maxMs: number;
  readonly signal?: AbortSignal;
}

function abortReason(signal: AbortSignal): unknown {
  return signal.reason ?? new Error("retryWithJitteredBackoff: aborted");
}

function abortableDelay(ms: number, signal: AbortSignal | undefined): Promise<void> {
  return new Promise<void>((resolve, reject) => {
    if (signal?.aborted) {
      reject(abortReason(signal));
      return;
    }
    const timer = setTimeout(() => {
      cleanup();
      resolve();
    }, ms);
    function cleanup(): void {
      clearTimeout(timer);
      signal?.removeEventListener("abort", onAbort);
    }
    function onAbort(): void {
      cleanup();
      reject(abortReason(signal!));
    }
    signal?.addEventListener("abort", onAbort, { once: true });
  });
}

/**
 * @emoji 🔁️ Retries `fn` with full-jitter exponential backoff (delay is a random value drawn from
 * `[minMs, min(maxMs, minMs * 2^attempt)]`, not a fixed exponential curve — this is what stops many
 * reconnecting clients from ever synchronizing into a hammering herd) until it resolves, `signal`
 * aborts, or `signal` is already aborted. Never returns while looping silently: an abort always
 * settles the returned promise, surfacing the abort's reason rather than hanging forever. For short
 * connection shortages only — the app must not freeze while this retries, and must not hammer the
 * remote end.
 * Wiederholt `fn` mit „full jitter“-Backoff, bis es erfüllt wird oder `signal` abbricht; ein Abbruch
 * löst die zurückgegebene Promise immer auf, statt endlos zu warten.
 */
export async function retryWithJitteredBackoff<T>(fn: () => Promise<T>, options: JitteredBackoffOptions): Promise<T> {
  const { minMs, maxMs, signal } = options;
  let attempt = 0;
  for (;;) {
    if (signal?.aborted) throw abortReason(signal);
    try {
      return await fn();
    } catch (error) {
      if (signal?.aborted) throw abortReason(signal);
      attempt += 1;
      const cap = Math.min(maxMs, minMs * 2 ** attempt);
      const waitMs = cap <= minMs ? minMs : minMs + Math.random() * (cap - minMs);
      try {
        await abortableDelay(waitMs, signal);
      } catch {
        throw signal?.aborted ? abortReason(signal) : error;
      }
    }
  }
}
//#endregion 🔁️RetryWithJitteredBackoff
