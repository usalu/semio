export type BrowserAssetFailureDisposition = "cancelled" | "missing" | "fault";

/** 🧯️ Separates one unavailable asset from cancellation and renderer-credit faults. */
export function browserAssetFailureDisposition(error: unknown, referenceImage: boolean): BrowserAssetFailureDisposition {
  if (error instanceof DOMException && error.name === "AbortError") return "cancelled";
  if (referenceImage || error instanceof BrowserAssetTransportFailure || error instanceof TypeError) return "missing";
  const detail = error instanceof Error ? error.message : String(error);
  if (detail.startsWith("asset-fetch-status:") || detail.startsWith("asset-response-short-read:")) return "missing";
  return "fault";
}

export class BrowserAssetTransportFailure extends Error {
  constructor(detail: string, readonly retryable: boolean) {
    super(detail);
  }
}

/** 🌐️ Classifies failures at the transport boundary before renderer or decoder work. */
export async function browserAssetNetworkStep<T>(operation: () => Promise<T>): Promise<T> {
  try {
    return await operation();
  } catch (error) {
    if (error instanceof TypeError) throw new BrowserAssetTransportFailure(error.message, true);
    throw error;
  }
}

export function browserAssetHttpStatusRetryable(status: number): boolean {
  return status === 408 || status === 429 || status === 500 || status === 502 || status === 503 || status === 504;
}

export async function retryBrowserAssetResponse<T>(read: () => Promise<T>, prepare?: (delayMs: number) => Promise<void>): Promise<T> {
  if (!prepare) return read();
  for (const delayMs of [200, 800]) {
    try {
      return await read();
    } catch (error) {
      if (!(error instanceof BrowserAssetTransportFailure) || !error.retryable) throw error;
      await prepare(delayMs);
    }
  }
  return read();
}

export async function waitForBrowserAssetRetry(delayMs: number, signal: AbortSignal): Promise<void> {
  signal.throwIfAborted();
  let timer: ReturnType<typeof setTimeout> | undefined;
  let abort: () => void = () => {};
  try {
    await new Promise<void>((resolve, reject) => {
      abort = () => reject(signal.reason);
      signal.addEventListener("abort", abort, { once: true });
      timer = setTimeout(resolve, delayMs);
    });
  } finally {
    signal.removeEventListener("abort", abort);
    clearTimeout(timer);
  }
}
