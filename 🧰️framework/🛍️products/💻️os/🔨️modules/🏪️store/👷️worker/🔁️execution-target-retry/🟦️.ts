/** 🔁️ Execution-target retry policy (`🔣️.json`): one document asset the hub declared temporarily unavailable is asked for again
 * with a bounded, announced, cancellable backoff instead of failing the whole opening (ticket 26/09/23 S15: catalog B2 wfc2d,
 * gis and writer documents opened as "created, but it could not be opened" on one `execution-target/component` 503). */

import contract from "./🔣️.json" with { type: "json" };

export const EXECUTION_TARGET_RETRY_V1 = contract;

/** ⏳️ The wait before retry `retry` (1-based): `backoffInitialMs · backoffFactor^(retry-1)`, capped at `backoffMaxMs`. */
export function executionTargetRetryDelayV1(retry: number): number {
  return Math.min(contract.backoffMaxMs, contract.backoffInitialMs * contract.backoffFactor ** Math.min(Math.max(retry - 1, 0), 30));
}

/** 🚦️ Whether `status` is one the policy declares transient. */
export function executionTargetTransientStatusV1(status: number): boolean {
  return contract.transientStatuses.includes(status);
}

export type ExecutionTargetRetryAnswerV1 = { readonly status: number; readonly body?: ReadableStream<Uint8Array> | null };

/** 🔁️ Runs `request` until it answers anything but a declared transient status, the attempts are spent, or `signal` aborts;
 * before each retry the transient answer's body is released, `onRetry(attempt, of)` announces the next attempt, and `sleep`
 * waits the policy's backoff (it rejects when `signal` aborts). The last answer is returned as it is. */
export async function requestExecutionTargetAssetV1<T extends ExecutionTargetRetryAnswerV1>(
  request: () => Promise<T>,
  options: { readonly signal: AbortSignal; readonly onRetry: (attempt: number, of: number) => void; readonly sleep: (ms: number, signal: AbortSignal) => Promise<void> },
): Promise<T> {
  for (let attempt = 1; ; attempt += 1) {
    const answer = await request();
    if (!executionTargetTransientStatusV1(answer.status) || attempt >= contract.maxAttempts || options.signal.aborted) return answer;
    void answer.body?.cancel().catch(() => undefined);
    options.onRetry(attempt + 1, contract.maxAttempts);
    await options.sleep(executionTargetRetryDelayV1(attempt), options.signal);
  }
}

/** 💤️ A backoff wait that rejects as soon as `signal` aborts. */
export function executionTargetRetrySleepV1(ms: number, signal: AbortSignal): Promise<void> {
  return new Promise((resolve, reject) => {
    if (signal.aborted) {
      reject(signal.reason ?? new Error("document execution target: cancelled"));
      return;
    }
    const timer = setTimeout(() => {
      signal.removeEventListener("abort", abort);
      resolve();
    }, ms);
    const abort = (): void => {
      clearTimeout(timer);
      reject(signal.reason ?? new Error("document execution target: cancelled"));
    };
    signal.addEventListener("abort", abort, { once: true });
  });
}
