/** 📥️ Loads of the client's own code and texts — a screen chunk, a language, the site's material — that give up after a few
 * failures instead of waiting for ever: a chunk a redeploy replaced is gone for good, so only a reload of the page can
 * help, and the client says so.
 *
 * @see ../../../../../../../🔨️modules/⏳️async/🔁️jittered-backoff/🟦️.ts
 */

import { useCallback, useEffect, useRef, useState } from "react";
import { retryWithJitteredBackoff } from "@semio-tech/framework/async";
import { RETRY_TIMING, type RetryTiming } from "../../🛂️proctor/🟦️.ts";

/** 🔢️ How often a load is tried, with jittered backoff between the tries, before it counts as failed. */
export const LOAD_ATTEMPTS = 4;

/** 🩹️ Retries `load` with jittered backoff for {@link LOAD_ATTEMPTS} tries; rejects with the last failure, or with the reason of `signal` when that aborts first. */
export function loadBounded<T>(load: () => Promise<T>, signal?: AbortSignal, timing: RetryTiming = RETRY_TIMING): Promise<T> {
  return retryWithJitteredBackoff(load, { ...timing, maxAttempts: LOAD_ATTEMPTS, signal });
}

/** 🚦️ Whether a load given up on has failed, and the way to try it afresh. */
export interface BoundedLoad {
  readonly failed: boolean;
  readonly retry: () => void;
}

/** 🪝️ Runs `load` while `wanted` — retried like {@link loadBounded}, cancelled when `wanted` or `key` change or the component unmounts —
 * and hands its value to `onLoaded`; once it gave up for `key`, `failed` says so until `retry` starts it afresh. */
export function useBoundedLoad<T>(wanted: boolean, key: string, load: () => Promise<T>, onLoaded: (value: T) => void, timing?: RetryTiming): BoundedLoad {
  const latest = useRef({ load, onLoaded, timing });
  latest.current = { load, onLoaded, timing };
  const [round, setRound] = useState(0);
  const [failedKey, setFailedKey] = useState<string | undefined>(undefined);
  useEffect(() => {
    setFailedKey(undefined);
    if (!wanted) return;
    const abort = new AbortController();
    loadBounded(() => latest.current.load(), abort.signal, latest.current.timing).then(
      (value) => {
        if (!abort.signal.aborted) latest.current.onLoaded(value);
      },
      () => {
        if (!abort.signal.aborted) setFailedKey(key);
      },
    );
    return () => abort.abort();
  }, [wanted, key, round]);
  const retry = useCallback(() => {
    setFailedKey(undefined);
    setRound((current) => current + 1);
  }, []);
  return { failed: wanted && failedKey === key, retry };
}
