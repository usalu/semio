//#region ⏱️FetchWithTimeout
/** 📨️ Structural view of a fetch response — declared locally so this module's public API
 * never requires the ambient `Response` type from outside this codebase. */
export interface FetchTimeoutResponse {
  readonly ok: boolean;
  readonly status: number;
  readonly statusText: string;
  readonly headers: { get(name: string): string | null };
  json(): Promise<unknown>;
  text(): Promise<string>;
}

/** ⏱️ Options for {@link fetchWithTimeout}. */
export interface FetchTimeoutOptions {
  readonly timeoutMs: number;
  readonly signal?: AbortSignal;
}

const BODY_READERS = ["arrayBuffer", "blob", "bytes", "formData", "json", "text"] as const;

/** 📖️ Makes every body reader of `response` end with the abort reason of `signal` once it aborts, and report to `finish` once a read has settled. */
function boundedBody<R extends object>(response: R, signal: AbortSignal, finish: () => void): R {
  for (const name of BODY_READERS) {
    const reader = (response as Record<string, unknown>)[name];
    if (typeof reader !== "function") continue;
    const bounded = (...args: unknown[]): Promise<unknown> =>
      new Promise((resolve, reject) => {
        if (signal.aborted) return reject(signal.reason);
        const abort = (): void => reject(signal.reason);
        const settle = (): void => {
          signal.removeEventListener("abort", abort);
          finish();
        };
        signal.addEventListener("abort", abort, { once: true });
        (reader as (...rest: unknown[]) => Promise<unknown>).apply(response, args).then(
          (value) => (settle(), resolve(value)),
          (error) => (settle(), reject(error)),
        );
      });
    Object.defineProperty(response, name, { value: bounded, configurable: true, writable: true });
  }
  return response;
}

/**
 * ⌛️ `fetch` composed with a timeout: the request aborts if it hasn't settled within
 * `timeoutMs` — the deadline covers the whole exchange, so a body that stalls after the headers arrived ends with the
 * timeout error as well — and separately aborts if the caller-supplied `signal` aborts; either can cancel it. Timer and
 * listener are cleaned up on every exit path: a thrown `fetch` error at once, a response once one of its body readers
 * (`text`, `json`, `arrayBuffer`, …) has settled; a response whose body is never read keeps the deadline, which then
 * releases its connection. The very response object is returned, its body readers bounded.
 * `fetch` mit Timeout: das Zeitlimit gilt für den ganzen Austausch samt Antwortkörper; sowohl es als auch das übergebene
 * `signal` können den Aufruf abbrechen; Timer und Listener werden aufgeräumt, sobald ein Lesen des Körpers endet.
 */
export async function fetchWithTimeout(url: string, init: RequestInit | undefined, options: FetchTimeoutOptions): Promise<FetchTimeoutResponse> {
  const { timeoutMs, signal: externalSignal } = options;
  if (externalSignal?.aborted) throw externalSignal.reason ?? new Error("fetchWithTimeout: aborted");

  const controller = new AbortController();
  const timer = setTimeout(() => controller.abort(new Error(`fetchWithTimeout: timed out after ${timeoutMs}ms`)), timeoutMs);
  function onExternalAbort(): void {
    controller.abort(externalSignal!.reason);
  }
  function finish(): void {
    clearTimeout(timer);
    externalSignal?.removeEventListener("abort", onExternalAbort);
  }
  externalSignal?.addEventListener("abort", onExternalAbort, { once: true });

  try {
    return boundedBody(await fetch(url, { ...init, signal: controller.signal }), controller.signal, finish);
  } catch (error) {
    finish();
    throw error;
  }
}
//#endregion ⏱️FetchWithTimeout
