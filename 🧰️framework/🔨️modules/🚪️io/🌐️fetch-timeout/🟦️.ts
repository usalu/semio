//#region ⏱️FetchWithTimeout
/** @emoji 📨️ Structural view of a fetch response — declared locally so this module's public API
 * never requires the ambient `Response` type from outside this codebase. */
export interface FetchTimeoutResponse {
  readonly ok: boolean;
  readonly status: number;
  readonly statusText: string;
  readonly headers: { get(name: string): string | null };
  json(): Promise<unknown>;
  text(): Promise<string>;
}

/** @emoji ⏱️ Options for {@link fetchWithTimeout}. */
export interface FetchTimeoutOptions {
  readonly timeoutMs: number;
  readonly signal?: AbortSignal;
}

/**
 * @emoji ⏱️ `fetch` composed with a timeout: the request aborts if it hasn't settled within
 * `timeoutMs`, and separately aborts if the caller-supplied `signal` aborts — either can cancel it,
 * neither leaks its timer/listener past this call (both are cleaned up on every exit path: success,
 * timeout, external abort, and thrown `fetch` error alike).
 * `fetch` mit Timeout: sowohl das Zeitlimit als auch das übergebene `signal` können den Aufruf
 * abbrechen; Timer und Listener werden in jedem Fall aufgeräumt.
 */
export async function fetchWithTimeout(url: string, init: RequestInit | undefined, options: FetchTimeoutOptions): Promise<FetchTimeoutResponse> {
  const { timeoutMs, signal: externalSignal } = options;
  if (externalSignal?.aborted) throw externalSignal.reason ?? new Error("fetchWithTimeout: aborted");

  const controller = new AbortController();
  const timer = setTimeout(() => controller.abort(new Error(`fetchWithTimeout: timed out after ${timeoutMs}ms`)), timeoutMs);
  function onExternalAbort(): void {
    controller.abort(externalSignal!.reason);
  }
  externalSignal?.addEventListener("abort", onExternalAbort, { once: true });

  try {
    return await fetch(url, { ...init, signal: controller.signal });
  } finally {
    clearTimeout(timer);
    externalSignal?.removeEventListener("abort", onExternalAbort);
  }
}
//#endregion ⏱️FetchWithTimeout
